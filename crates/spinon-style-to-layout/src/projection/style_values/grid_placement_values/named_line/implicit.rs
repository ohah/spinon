use super::*;

pub(super) fn resolve_named_placements<'a>(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    mut start: Placement<'a>,
    mut end: Placement<'a>,
    line_metadata: &LineMetadata<'_>,
    allow_implicit_named_lines: bool,
) -> Result<(Placement<'a>, Placement<'a>, ImplicitNamedLineUsage), StyleLayoutError> {
    let span_resolver =
        super::ImplicitNamedLineResolver::new(node, line_metadata, allow_implicit_named_lines);
    let start_is_named_line = matches!(start, Placement::NamedLine { .. });
    let end_is_named_line = matches!(end, Placement::NamedLine { .. });
    let mut start_uses_implicit_grid_line = false;
    let mut end_uses_implicit_grid_line = false;
    let mut usage = ImplicitNamedLineUsage::default();
    if matches!(start, Placement::NamedLine { .. }) {
        (start, start_uses_implicit_grid_line) = super::resolve_named_endpoint(
            node,
            start_property,
            start,
            true,
            line_metadata,
            allow_implicit_named_lines,
        )?;
        usage.include(usage_for_line(
            start,
            start_uses_implicit_grid_line,
            line_metadata.track_count,
        ));
    }
    if matches!(end, Placement::NamedLine { .. }) {
        (end, end_uses_implicit_grid_line) = super::resolve_named_endpoint(
            node,
            end_property,
            end,
            false,
            line_metadata,
            allow_implicit_named_lines,
        )?;
        usage.include(usage_for_line(
            end,
            end_uses_implicit_grid_line,
            line_metadata.track_count,
        ));
    }

    match (start, end) {
        (
            Placement::SpanNamed { name: _, .. },
            Placement::Span(_) | Placement::SpanNamed { .. },
        ) => {
            // CSS Grid 충돌 규칙은 끝쪽 span을 버립니다. 남은 named span의 반대쪽에
            // 확정된 line이 없으면 기본 1 track span으로 처리합니다.
            start = Placement::Span(1);
            start_uses_implicit_grid_line = false;
            end = Placement::Auto;
            end_uses_implicit_grid_line = false;
        }
        (Placement::Span(_), Placement::SpanNamed { .. }) => {
            // 끝쪽 span은 충돌로 버리므로 named line 검색도 하지 않습니다.
            end = Placement::Auto;
            end_uses_implicit_grid_line = false;
        }
        _ => {}
    }

    if let Placement::SpanNamed { name, count } = start {
        start = match end {
            Placement::Line(line) => {
                let anchor_uses_implicit =
                    line_requires_implicit_extent(line, line_metadata.track_count);
                if allow_implicit_named_lines && anchor_uses_implicit {
                    end_uses_implicit_grid_line = true;
                    usage.include(usage_for_line(
                        Placement::Line(line),
                        true,
                        line_metadata.track_count,
                    ));
                }
                let (line, is_implicit) = span_resolver.resolve_span_line(
                    start_property,
                    name,
                    count,
                    line,
                    super::SearchDirection::Backward,
                )?;
                start_uses_implicit_grid_line = is_implicit;
                usage.include(usage_for_line(
                    Placement::Line(line),
                    is_implicit,
                    line_metadata.track_count,
                ));
                Placement::Line(line)
            }
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
            Placement::Line(line) => {
                let anchor_uses_implicit =
                    line_requires_implicit_extent(line, line_metadata.track_count);
                if allow_implicit_named_lines && anchor_uses_implicit {
                    start_uses_implicit_grid_line = true;
                    usage.include(usage_for_line(
                        Placement::Line(line),
                        true,
                        line_metadata.track_count,
                    ));
                }
                let (line, is_implicit) = span_resolver.resolve_span_line(
                    end_property,
                    name,
                    count,
                    line,
                    super::SearchDirection::Forward,
                )?;
                end_uses_implicit_grid_line = is_implicit;
                usage.include(usage_for_line(
                    Placement::Line(line),
                    is_implicit,
                    line_metadata.track_count,
                ));
                Placement::Line(line)
            }
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

    if allow_implicit_named_lines {
        validate_unmarked_implicit_line(
            node,
            start_property,
            start,
            start_uses_implicit_grid_line,
            line_metadata.track_count,
        )?;
        validate_unmarked_implicit_line(
            node,
            end_property,
            end,
            end_uses_implicit_grid_line,
            line_metadata.track_count,
        )?;
        if !start_uses_implicit_grid_line && !end_uses_implicit_grid_line {
            validate_unmarked_implicit_span_extent(
                node,
                start_property,
                end_property,
                start,
                end,
                line_metadata.track_count,
            )?;
        }
    }

    usage.allows_positive_named_line_pair_auto_track_sizing = start_is_named_line
        && end_is_named_line
        && start_uses_implicit_grid_line
        && end_uses_implicit_grid_line
        && is_positive_trailing_implicit_line(start, line_metadata.track_count)
        && is_positive_trailing_implicit_line(end, line_metadata.track_count);

    Ok((start, end, usage))
}

fn is_positive_trailing_implicit_line(placement: Placement<'_>, track_count: usize) -> bool {
    matches!(placement, Placement::Line(line)
        if line > 0 && line_requires_implicit_extent(line, track_count))
}

fn usage_for_line(
    placement: Placement<'_>,
    uses_implicit_grid_line: bool,
    track_count: usize,
) -> ImplicitNamedLineUsage {
    if !uses_implicit_grid_line {
        return ImplicitNamedLineUsage::default();
    }
    let requires_signed_extent = matches!(placement, Placement::Line(line)
        if line < 0 && line_requires_implicit_extent(line, track_count));
    ImplicitNamedLineUsage {
        uses_implicit_tracks: true,
        requires_signed_extent,
        allows_positive_auto_track_sizing: false,
        allows_positive_named_line_pair_auto_track_sizing: false,
    }
}

fn line_requires_implicit_extent(line: i32, track_count: usize) -> bool {
    super::normalize_explicit_line(line, track_count).is_err()
}

fn validate_unmarked_implicit_line(
    node: NodeId,
    property: &'static str,
    placement: Placement<'_>,
    is_implicit_named_line: bool,
    track_count: usize,
) -> Result<(), StyleLayoutError> {
    if let Placement::Line(line) = placement
        && !is_implicit_named_line
        && super::normalize_explicit_line(line, track_count).is_err()
    {
        return Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::InvalidGridPlacement {
                node,
                property,
                reason: "line placement requires an implicit line outside the named-line subset",
            },
        ));
    }
    Ok(())
}

fn validate_unmarked_implicit_span_extent(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: Placement<'_>,
    end: Placement<'_>,
    track_count: usize,
) -> Result<(), StyleLayoutError> {
    let last_line = track_count
        .checked_add(1)
        .and_then(|line| i64::try_from(line).ok())
        .ok_or_else(|| {
            invalid_implicit_extent(
                node,
                start_property,
                "explicit grid line range exceeds the supported range",
            )
        })?;
    let start_line = match start {
        Placement::Line(line) => Some(
            super::normalize_explicit_line(line, track_count)
                .map_err(|reason| invalid_implicit_extent(node, start_property, reason))?,
        ),
        Placement::Auto | Placement::Span(_) => None,
        Placement::NamedLine { .. } | Placement::SpanNamed { .. } => {
            return Err(super::unsupported_value(
                node,
                start_property,
                "unresolved named placement",
            ));
        }
    };
    let end_line = match end {
        Placement::Line(line) => Some(
            super::normalize_explicit_line(line, track_count)
                .map_err(|reason| invalid_implicit_extent(node, end_property, reason))?,
        ),
        Placement::Auto | Placement::Span(_) => None,
        Placement::NamedLine { .. } | Placement::SpanNamed { .. } => {
            return Err(super::unsupported_value(
                node,
                end_property,
                "unresolved named placement",
            ));
        }
    };

    let (first_line, final_line, property) = match (start, end, start_line, end_line) {
        (Placement::Line(_), Placement::Span(span), Some(first), None) => (
            first,
            first.checked_add(i64::from(span)).ok_or_else(|| {
                invalid_implicit_extent(
                    node,
                    end_property,
                    "line and span exceed the supported range",
                )
            })?,
            end_property,
        ),
        (Placement::Span(span), Placement::Line(_), None, Some(final_line)) => (
            final_line.checked_sub(i64::from(span)).ok_or_else(|| {
                invalid_implicit_extent(
                    node,
                    start_property,
                    "span start exceeds the supported range",
                )
            })?,
            final_line,
            start_property,
        ),
        (Placement::Line(_), Placement::Line(_), Some(first), Some(final_line)) => {
            (first.min(final_line), first.max(final_line), end_property)
        }
        (Placement::Line(_), Placement::Auto, Some(line), None)
        | (Placement::Auto, Placement::Line(_), None, Some(line)) => {
            let first = if matches!(start, Placement::Line(_)) {
                line
            } else {
                line - 1
            };
            (first, first + 1, end_property)
        }
        _ => return Ok(()),
    };

    if first_line < 1 || final_line > last_line {
        return Err(invalid_implicit_extent(
            node,
            property,
            "placement span requires an implicit grid line outside the named-line subset",
        ));
    }
    Ok(())
}

fn invalid_implicit_extent(
    node: NodeId,
    property: &'static str,
    reason: &'static str,
) -> StyleLayoutError {
    StyleLayoutError::Layout(spinon_layout::LayoutError::InvalidGridPlacement {
        node,
        property,
        reason,
    })
}
