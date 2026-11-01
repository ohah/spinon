#[path = "named_line/area.rs"]
mod area;
#[path = "named_line/implicit.rs"]
mod implicit;
#[path = "named_line/implicit_resolver.rs"]
mod implicit_resolver;
use implicit_resolver::{
    ImplicitNamedLineResolver, SearchDirection, find_named_occurrence,
    resolve_named_line_occurrence,
};

use spinon_core::NodeId;
use spinon_layout::GridPlacementInput;
use spinon_style::{ComputedCssGridPlacement, ComputedCssGridTemplateAreas};

use crate::StyleLayoutError;

#[derive(Clone, Copy)]
enum Placement<'a> {
    Auto,
    Line(i32),
    NamedLine {
        name: &'a str,
        occurrence: Option<i32>,
    },
    Span(u32),
    SpanNamed {
        name: &'a str,
        count: u32,
    },
}

pub(super) struct LineMetadata<'a> {
    pub(super) names: &'a [Vec<String>],
    pub(super) opposite_names: &'a [Vec<String>],
    pub(super) areas: Option<&'a ComputedCssGridTemplateAreas>,
    pub(super) axis: GridAxis,
    pub(super) track_count: usize,
    pub(super) allow_area_dimension_mismatch: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::projection::style_values) struct ImplicitNamedLineUsage {
    pub(in crate::projection::style_values) uses_implicit_tracks: bool,
    pub(in crate::projection::style_values) requires_signed_extent: bool,
    pub(in crate::projection::style_values) allows_positive_auto_track_sizing: bool,
    pub(in crate::projection::style_values) allows_positive_named_line_pair_auto_track_sizing: bool,
    pub(in crate::projection::style_values) allows_negative_named_line_pair_auto_track_sizing: bool,
}

impl ImplicitNamedLineUsage {
    fn include(&mut self, other: Self) {
        self.uses_implicit_tracks |= other.uses_implicit_tracks;
        self.requires_signed_extent |= other.requires_signed_extent;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GridAxis {
    Rows,
    Columns,
}

#[cfg(test)]
pub(super) fn project_axis(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &ComputedCssGridPlacement,
    end: &ComputedCssGridPlacement,
    line_names: &[Vec<String>],
    track_count: usize,
) -> Result<(GridPlacementInput, GridPlacementInput), StyleLayoutError> {
    project_axis_with_areas(
        node,
        start_property,
        end_property,
        start,
        end,
        LineMetadata {
            names: line_names,
            opposite_names: &[],
            areas: None,
            axis: GridAxis::Rows,
            track_count,
            allow_area_dimension_mismatch: false,
        },
    )
}

#[cfg(test)]
pub(super) fn project_axis_with_areas(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &ComputedCssGridPlacement,
    end: &ComputedCssGridPlacement,
    line_metadata: LineMetadata<'_>,
) -> Result<(GridPlacementInput, GridPlacementInput), StyleLayoutError> {
    let (start, end, _) = project_axis_with_areas_and_implicit_named_lines(
        node,
        start_property,
        end_property,
        start,
        end,
        line_metadata,
        false,
    )?;
    Ok((start, end))
}

pub(super) fn project_axis_with_areas_and_implicit_named_lines(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &ComputedCssGridPlacement,
    end: &ComputedCssGridPlacement,
    line_metadata: LineMetadata<'_>,
    allow_implicit_named_lines: bool,
) -> Result<
    (
        GridPlacementInput,
        GridPlacementInput,
        ImplicitNamedLineUsage,
    ),
    StyleLayoutError,
> {
    let start = decode(node, start_property, start)?;
    let end = decode(node, end_property, end)?;
    let (start, end, used_implicit_named_line) = implicit::resolve_named_placements(
        node,
        start_property,
        end_property,
        start,
        end,
        &line_metadata,
        allow_implicit_named_lines,
    )?;
    Ok((
        to_layout(node, start_property, start)?,
        to_layout(node, end_property, end)?,
        used_implicit_named_line,
    ))
}

/// Auto-repeat 축의 line index는 반복 수에 따라 달라지므로 이름 있는 배치를 Taffy까지 보존합니다.
pub(super) fn project_axis_for_auto_repeat(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &ComputedCssGridPlacement,
    end: &ComputedCssGridPlacement,
) -> Result<(GridPlacementInput, GridPlacementInput, bool), StyleLayoutError> {
    let start = decode(node, start_property, start)?;
    let end = decode(node, end_property, end)?;
    Ok((
        to_auto_repeat_layout(node, start_property, start)?,
        to_auto_repeat_layout(node, end_property, end)?,
        false,
    ))
}

fn to_auto_repeat_layout(
    node: NodeId,
    property: &'static str,
    placement: Placement<'_>,
) -> Result<GridPlacementInput, StyleLayoutError> {
    match placement {
        Placement::Auto => Ok(GridPlacementInput::Auto),
        Placement::Line(line) => Ok(GridPlacementInput::Line(line)),
        Placement::Span(count) => Ok(GridPlacementInput::Span(count)),
        Placement::NamedLine { name, occurrence } => {
            let occurrence_was_explicit = occurrence.is_some();
            let occurrence = occurrence.unwrap_or(0);
            if occurrence == 0 && occurrence_was_explicit {
                return Err(unsupported_named_line(
                    node,
                    property,
                    name,
                    "explicit named-line occurrence는 0일 수 없습니다",
                ));
            }
            i16::try_from(occurrence).map_err(|_| {
                unsupported_named_line(
                    node,
                    property,
                    name,
                    "named-line occurrence가 Taffy의 정수 범위를 넘었습니다",
                )
            })?;
            Ok(GridPlacementInput::NamedLine {
                name: copy_layout_name(node, property, name)?,
                occurrence,
            })
        }
        Placement::SpanNamed { name, count } => {
            u16::try_from(count).map_err(|_| {
                unsupported_named_line(
                    node,
                    property,
                    name,
                    "named span이 Taffy의 정수 범위를 넘었습니다",
                )
            })?;
            Ok(GridPlacementInput::SpanNamed {
                name: copy_layout_name(node, property, name)?,
                count,
            })
        }
    }
}

fn copy_layout_name(
    node: NodeId,
    property: &'static str,
    name: &str,
) -> Result<String, StyleLayoutError> {
    let mut owned = String::new();
    owned.try_reserve(name.len()).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    owned.push_str(name);
    Ok(owned)
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

fn resolve_named_endpoint<'a>(
    node: NodeId,
    property: &'static str,
    placement: Placement<'a>,
    is_start: bool,
    line_metadata: &LineMetadata<'_>,
    allow_implicit_named_lines: bool,
) -> Result<(Placement<'a>, bool), StyleLayoutError> {
    let Placement::NamedLine { name, occurrence } = placement else {
        return Ok((placement, false));
    };
    let (line, is_implicit_named_line) = if let Some(occurrence) = occurrence {
        resolve_named_line_occurrence(
            node,
            property,
            name,
            occurrence,
            line_metadata,
            allow_implicit_named_lines,
        )?
    } else {
        area::resolve_bare_custom_ident(
            node,
            property,
            name,
            is_start,
            line_metadata,
            allow_implicit_named_lines,
        )?
    };
    Ok((Placement::Line(line), is_implicit_named_line))
}

fn find_first_named_occurrence_if_present(
    node: NodeId,
    property: &'static str,
    name: &str,
    line_metadata: &LineMetadata<'_>,
) -> Result<Option<i32>, StyleLayoutError> {
    validate_line_metadata(node, property, name, line_metadata)?;
    let Some((index, _)) = line_metadata
        .names
        .iter()
        .enumerate()
        .find(|(index, _)| area::line_has_name(line_metadata, *index, name))
    else {
        return Ok(None);
    };
    i32::try_from(index + 1).map(Some).map_err(|_| {
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
