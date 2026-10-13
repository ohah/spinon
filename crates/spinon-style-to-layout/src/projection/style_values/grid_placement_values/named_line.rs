use spinon_core::NodeId;
use spinon_layout::GridPlacementInput;
use spinon_style::ComputedCssGridPlacement;

use crate::StyleLayoutError;

#[derive(Clone, Copy)]
enum Placement<'a> {
    Auto,
    Line(i32),
    NamedLine { name: &'a str, occurrence: i32 },
    Span(u32),
    SpanNamed { name: &'a str, count: u32 },
}

struct LineMetadata<'a> {
    names: &'a [Vec<String>],
    track_count: usize,
}

pub(super) fn project_axis(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &ComputedCssGridPlacement,
    end: &ComputedCssGridPlacement,
    line_names: &[Vec<String>],
    track_count: usize,
) -> Result<(GridPlacementInput, GridPlacementInput), StyleLayoutError> {
    let start = decode(node, start_property, start)?;
    let end = decode(node, end_property, end)?;
    let line_metadata = LineMetadata {
        names: line_names,
        track_count,
    };
    let (start, end) = resolve_named_placements(
        node,
        start_property,
        end_property,
        start,
        end,
        &line_metadata,
    )?;
    Ok((
        to_layout(node, start_property, start)?,
        to_layout(node, end_property, end)?,
    ))
}

fn decode<'a>(
    node: NodeId,
    property: &'static str,
    value: &'a ComputedCssGridPlacement,
) -> Result<Placement<'a>, StyleLayoutError> {
    match value {
        ComputedCssGridPlacement::Auto => Ok(Placement::Auto),
        ComputedCssGridPlacement::Line(line) => Ok(Placement::Line(*line)),
        ComputedCssGridPlacement::NamedLine { name, occurrence } => Ok(Placement::NamedLine {
            name,
            occurrence: *occurrence,
        }),
        ComputedCssGridPlacement::Span(span) => u32::try_from(*span)
            .map(Placement::Span)
            .map_err(|_| unsupported_value(node, property, &span.to_string())),
        ComputedCssGridPlacement::SpanNamed { name, count } => {
            if *count == 0 {
                return Err(unsupported_named_line(
                    node,
                    property,
                    name,
                    "named span count must be positive",
                ));
            }
            Ok(Placement::SpanNamed {
                name,
                count: *count,
            })
        }
        ComputedCssGridPlacement::Unsupported(serialized) => {
            Err(unsupported_value(node, property, serialized))
        }
    }
}

fn resolve_named_placements<'a>(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    mut start: Placement<'a>,
    mut end: Placement<'a>,
    line_metadata: &LineMetadata<'_>,
) -> Result<(Placement<'a>, Placement<'a>), StyleLayoutError> {
    if matches!(start, Placement::NamedLine { .. }) {
        start = resolve_named_endpoint(node, start_property, start, line_metadata)?;
    }
    if matches!(end, Placement::NamedLine { .. }) {
        end = resolve_named_endpoint(node, end_property, end, line_metadata)?;
    }

    match (start, end) {
        (
            Placement::SpanNamed { name: _, .. },
            Placement::Span(_) | Placement::SpanNamed { .. },
        ) => {
            // CSS Grid 충돌 규칙은 끝쪽 span을 버립니다. 남은 named span의 반대쪽에
            // 확정된 line이 없으면 기본 1 track span으로 처리합니다.
            start = Placement::Span(1);
            end = Placement::Auto;
        }
        (Placement::Span(_), Placement::SpanNamed { .. }) => {
            // 끝쪽 span은 충돌로 버리므로 named line 검색도 하지 않습니다.
            end = Placement::Auto;
        }
        _ => {}
    }

    if let Placement::SpanNamed { name, count } = start {
        start = match end {
            Placement::Line(line) => Placement::Line(resolve_named_span_line(
                node,
                start_property,
                name,
                count,
                line,
                SearchDirection::Backward,
                line_metadata,
            )?),
            Placement::Auto | Placement::Span(_) | Placement::SpanNamed { .. } => {
                Placement::Span(1)
            }
            Placement::NamedLine { name, .. } => {
                return Err(unsupported_named_line(
                    node,
                    start_property,
                    name,
                    "named placement was not resolved before span projection",
                ));
            }
        };
    }
    if let Placement::SpanNamed { name, count } = end {
        end = match start {
            Placement::Line(line) => Placement::Line(resolve_named_span_line(
                node,
                end_property,
                name,
                count,
                line,
                SearchDirection::Forward,
                line_metadata,
            )?),
            Placement::Auto | Placement::Span(_) | Placement::SpanNamed { .. } => {
                Placement::Span(1)
            }
            Placement::NamedLine { name, .. } => {
                return Err(unsupported_named_line(
                    node,
                    end_property,
                    name,
                    "named placement was not resolved before span projection",
                ));
            }
        };
    }

    Ok((start, end))
}

fn resolve_named_endpoint<'a>(
    node: NodeId,
    property: &'static str,
    placement: Placement<'a>,
    line_metadata: &LineMetadata<'_>,
) -> Result<Placement<'a>, StyleLayoutError> {
    let Placement::NamedLine { name, occurrence } = placement else {
        return Ok(placement);
    };
    let line = find_named_occurrence(node, property, name, occurrence, line_metadata)?;
    Ok(Placement::Line(line))
}

#[derive(Clone, Copy)]
enum SearchDirection {
    Forward,
    Backward,
}

fn resolve_named_span_line(
    node: NodeId,
    property: &'static str,
    name: &str,
    count: u32,
    opposite_line: i32,
    direction: SearchDirection,
    line_metadata: &LineMetadata<'_>,
) -> Result<i32, StyleLayoutError> {
    validate_line_metadata(node, property, name, line_metadata)?;
    let opposite_line = normalize_explicit_line(opposite_line, line_metadata.track_count)
        .map_err(|reason| unsupported_named_line(node, property, name, reason))?;
    let opposite_index = usize::try_from(opposite_line - 1).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "opposite definite line exceeds the supported range",
        )
    })?;
    let mut remaining = count;
    let mut found = None;
    match direction {
        SearchDirection::Forward => {
            for (index, names) in line_metadata
                .names
                .iter()
                .enumerate()
                .skip(opposite_index + 1)
            {
                if names.iter().any(|candidate| candidate == name) {
                    remaining -= 1;
                    if remaining == 0 {
                        found = Some(index);
                        break;
                    }
                }
            }
        }
        SearchDirection::Backward => {
            for (index, names) in line_metadata
                .names
                .iter()
                .enumerate()
                .take(opposite_index)
                .rev()
            {
                if names.iter().any(|candidate| candidate == name) {
                    remaining -= 1;
                    if remaining == 0 {
                        found = Some(index);
                        break;
                    }
                }
            }
        }
    }
    if let Some(index) = found {
        return i32::try_from(index + 1).map_err(|_| {
            unsupported_named_line(
                node,
                property,
                name,
                "resolved line exceeds the supported range",
            )
        });
    }
    Err(unsupported_named_line(
        node,
        property,
        name,
        "named span requires an implicit grid line outside the explicit grid",
    ))
}

fn find_named_occurrence(
    node: NodeId,
    property: &'static str,
    name: &str,
    occurrence: i32,
    line_metadata: &LineMetadata<'_>,
) -> Result<i32, StyleLayoutError> {
    validate_line_metadata(node, property, name, line_metadata)?;
    if occurrence == 0 {
        return Err(unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence cannot be zero",
        ));
    }
    let mut matches = line_metadata
        .names
        .iter()
        .enumerate()
        .filter(|(_, names)| names.iter().any(|candidate| candidate == name));
    let selected = if occurrence > 0 {
        let ordinal = usize::try_from(occurrence - 1).map_err(|_| {
            unsupported_named_line(
                node,
                property,
                name,
                "named line occurrence exceeds the supported range",
            )
        })?;
        matches.nth(ordinal).map(|(index, _)| index)
    } else {
        let ordinal = usize::try_from(occurrence.unsigned_abs() - 1).map_err(|_| {
            unsupported_named_line(
                node,
                property,
                name,
                "named line occurrence exceeds the supported range",
            )
        })?;
        matches.rev().nth(ordinal).map(|(index, _)| index)
    }
    .ok_or_else(|| {
        unsupported_named_line(
            node,
            property,
            name,
            "named line occurrence requires an implicit grid line outside the explicit grid",
        )
    })?;
    i32::try_from(selected + 1).map_err(|_| {
        unsupported_named_line(
            node,
            property,
            name,
            "resolved line exceeds the supported range",
        )
    })
}

fn validate_line_metadata(
    node: NodeId,
    property: &'static str,
    name: &str,
    line_metadata: &LineMetadata<'_>,
) -> Result<(), StyleLayoutError> {
    if line_metadata.track_count.checked_add(1) != Some(line_metadata.names.len()) {
        return Err(unsupported_named_line(
            node,
            property,
            name,
            "explicit grid line-name metadata is missing or has the wrong length",
        ));
    }
    Ok(())
}

fn normalize_explicit_line(line: i32, track_count: usize) -> Result<i64, &'static str> {
    if line == 0 {
        return Err("opposite definite line cannot be zero");
    }
    let line_count = track_count
        .checked_add(1)
        .and_then(|count| i64::try_from(count).ok())
        .ok_or("track count exceeds range")?;
    let normalized = if line < 0 {
        i64::from(line) + line_count + 1
    } else {
        i64::from(line)
    };
    if normalized < 1 || normalized > line_count {
        return Err("opposite definite line requires an implicit grid line");
    }
    Ok(normalized)
}

fn to_layout(
    node: NodeId,
    property: &'static str,
    placement: Placement<'_>,
) -> Result<GridPlacementInput, StyleLayoutError> {
    match placement {
        Placement::Auto => Ok(GridPlacementInput::Auto),
        Placement::Line(line) => Ok(GridPlacementInput::Line(line)),
        Placement::Span(count) => Ok(GridPlacementInput::Span(count)),
        Placement::NamedLine { name, .. } | Placement::SpanNamed { name, .. } => {
            Err(unsupported_named_line(
                node,
                property,
                name,
                "named placement was not resolved before layout projection",
            ))
        }
    }
}

fn unsupported_value(node: NodeId, property: &'static str, value: &str) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    }
}

fn unsupported_named_line(
    node: NodeId,
    property: &'static str,
    name: &str,
    reason: &'static str,
) -> StyleLayoutError {
    StyleLayoutError::UnsupportedNamedGridLine {
        node,
        property,
        name: name.to_owned(),
        reason,
    }
}

#[cfg(test)]
#[path = "named_line/tests.rs"]
mod tests;
