use super::*;

pub(super) fn validate_root_html_auto_repeat_contract(
    node: NodeId,
    style: LayoutStyle,
    container: &crate::GridContainerInput,
) -> Result<(), LayoutError> {
    let columns_repeat = container.columns_auto_repeat.as_ref();
    let rows_repeat = container.rows_auto_repeat.as_ref();
    if columns_repeat.is_some() == rows_repeat.is_some() {
        return Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        });
    }
    if container.template_areas.is_some() {
        return Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-areas",
        });
    }
    if container.auto_flow != GridAutoFlow::Row {
        return Err(LayoutError::UnsupportedGridAutoFlow(node));
    }
    if container.auto_rows != GridAutoRows::Auto {
        return Err(LayoutError::UnsupportedGridAutoRows {
            node,
            reason: "Root HTML auto-repeat requires grid-auto-rows:auto",
        });
    }
    if container.auto_columns != GridAutoColumns::Auto {
        return Err(LayoutError::UnsupportedGridAutoColumns {
            node,
            reason: "Root HTML auto-repeat requires grid-auto-columns:auto",
        });
    }

    let (
        property,
        repeated_tracks,
        repeated_names,
        repeat,
        other_tracks,
        other_names,
        size,
        min_size,
        max_size,
    ) = if let Some(repeat) = columns_repeat {
        (
            "grid-template-columns",
            container.columns.as_slice(),
            container.columns_line_names.as_slice(),
            repeat,
            container.rows.as_slice(),
            container.rows_line_names.as_slice(),
            style.width,
            style.min_width,
            style.max_width,
        )
    } else {
        let repeat = rows_repeat.expect("exactly one auto-repeat axis was checked");
        (
            "grid-template-rows",
            container.rows.as_slice(),
            container.rows_line_names.as_slice(),
            repeat,
            container.columns.as_slice(),
            container.columns_line_names.as_slice(),
            style.height,
            style.min_height,
            style.max_height,
        )
    };
    let repeat_pattern = repeated_tracks.get(repeat.start..repeat.end);
    let supported_pattern_track = |track: &GridTrackSize| {
        matches!(
            track,
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(minimum),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            } if minimum.is_finite() && *minimum > 0.0
        )
    };
    let line_names_are_unnamed = |groups: &[Vec<String>]| {
        groups
            .iter()
            .all(|group| group.iter().all(String::is_empty))
    };
    if repeated_tracks.len() > GRID_MAX_TRACKS
        || repeat.start != 0
        || repeat.end != repeated_tracks.len()
        || repeat_pattern.is_none_or(|pattern| {
            !(1..=2).contains(&pattern.len()) || !pattern.iter().all(supported_pattern_track)
        })
        || !line_names_are_unnamed(repeated_names)
        || !line_names_are_unnamed(&repeat.line_names)
        || other_tracks.len() != 1
        || !matches!(
            other_tracks.first(),
            Some(GridTrackSize::LengthPx(value)) if value.is_finite() && *value >= 0.0
        )
        || !line_names_are_unnamed(other_names)
    {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    if !matches!(size, LayoutDimension::Fixed(value) if value.is_finite() && value > 0.0)
        || min_size != LayoutDimension::Auto
        || max_size != LayoutDimension::Auto
    {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: match (size, min_size, max_size) {
                (LayoutDimension::Fixed(_), value, _) if value != LayoutDimension::Auto => {
                    if property == "grid-template-columns" {
                        "min-width"
                    } else {
                        "min-height"
                    }
                }
                (_, _, value) if value != LayoutDimension::Auto => {
                    if property == "grid-template-columns" {
                        "max-width"
                    } else {
                        "max-height"
                    }
                }
                _ => {
                    if property == "grid-template-columns" {
                        "width"
                    } else {
                        "height"
                    }
                }
            },
        });
    }
    let fixed_gap = |gap| matches!(gap, LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0);
    let unsupported_property = if style.aspect_ratio.is_some() {
        Some("aspect-ratio")
    } else if style.margin != crate::LayoutEdges::default() {
        Some("margin")
    } else if style.padding != crate::LayoutEdges::default() {
        Some("padding")
    } else if style.border != crate::LayoutBorder::default() {
        Some("border")
    } else if !fixed_gap(style.gap.row) {
        Some("row-gap")
    } else if !fixed_gap(style.gap.column) {
        Some("column-gap")
    } else if style.direction != crate::TextDirection::Ltr {
        Some("direction")
    } else if style.overflow_x != crate::LayoutOverflow::Visible {
        Some("overflow-x")
    } else if style.overflow_y != crate::LayoutOverflow::Visible {
        Some("overflow-y")
    } else if style.align_items != crate::LayoutAlignItems::Normal {
        Some("align-items")
    } else if style.align_content.is_some_and(|value| {
        !matches!(
            value,
            crate::LayoutAlignContent::Normal | crate::LayoutAlignContent::FlexStart
        ) && !matches!(
            value,
            crate::LayoutAlignContent::Position {
                position: crate::ContentAlignmentPosition::Start,
                safety: None,
            }
        )
    }) {
        Some("align-content")
    } else if !matches!(
        style.justify_content,
        crate::LayoutJustifyContent::Normal | crate::LayoutJustifyContent::FlexStart
    ) && !matches!(
        style.justify_content,
        crate::LayoutJustifyContent::Position {
            position: crate::JustifyContentPosition::Start,
            safety: None,
        }
    ) {
        Some("justify-content")
    } else if container.justify_items != crate::LayoutAlignItems::Normal {
        Some("justify-items")
    } else {
        None
    };
    if let Some(property) = unsupported_property {
        return Err(LayoutError::UnsupportedGridContainerStyle { node, property });
    }
    Ok(())
}

pub(super) fn validate_root_html_auto_repeat_items(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    container_node: &crate::LayoutNode,
    hidden: &BTreeMap<NodeId, bool>,
) -> Result<(), LayoutError> {
    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let placement = input.grid_items.get(&child);
        if placement.is_some_and(|placement| !placement.is_auto()) {
            return Err(LayoutError::InvalidGridPlacement {
                node: child,
                property: "grid-row-start",
                reason: "Root HTML auto-repeat requires default auto placement on every visible item",
            });
        }
        let item = &input.nodes[index[&child]];
        if item.style.order != 0 {
            return Err(LayoutError::UnsupportedGridOrder(child));
        }
        if input
            .positioning
            .get(&child)
            .is_some_and(|positioning| positioning.position != LayoutPosition::Static)
        {
            return Err(LayoutError::UnsupportedGridPositionedItem(child));
        }
        if item.style.aspect_ratio.is_some()
            || item.style.justify_self != crate::LayoutAlignSelf::Auto
            || item.style.align_self != crate::LayoutAlignSelf::Auto
        {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }
    }
    Ok(())
}

pub(super) fn root_html_auto_repeat_cell_capacity(
    node: NodeId,
    style: LayoutStyle,
    container: &crate::GridContainerInput,
) -> Result<usize, LayoutError> {
    let invalid = |property| LayoutError::InvalidGridTrackList { node, property };
    let (property, tracks, repeat, size, gap, other_track_count) =
        if let Some(repeat) = container.columns_auto_repeat.as_ref() {
            (
                "grid-template-columns",
                container.columns.as_slice(),
                repeat,
                style.width,
                style.gap.column,
                container.rows.len(),
            )
        } else if let Some(repeat) = container.rows_auto_repeat.as_ref() {
            (
                "grid-template-rows",
                container.rows.as_slice(),
                repeat,
                style.height,
                style.gap.row,
                container.columns.len(),
            )
        } else {
            return Err(invalid("grid-template-columns"));
        };
    let (LayoutDimension::Fixed(axis_size), LayoutLengthPercentage::LengthPx(gap)) = (size, gap)
    else {
        return Err(invalid(property));
    };
    let Some(pattern) = tracks.get(repeat.start..repeat.end) else {
        return Err(invalid(property));
    };
    if !(1..=2).contains(&pattern.len())
        || !axis_size.is_finite()
        || axis_size <= 0.0
        || !gap.is_finite()
        || gap < 0.0
    {
        return Err(invalid(property));
    }
    let mut pattern_space = 0.0_f32;
    for track in pattern {
        let GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(minimum),
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        } = track
        else {
            return Err(invalid(property));
        };
        if !minimum.is_finite() || *minimum <= 0.0 {
            return Err(invalid(property));
        }
        pattern_space += minimum.max(1.0);
    }
    let first_repeat_space = pattern_space + ((pattern.len() - 1) as f32 * gap);
    let per_repeat_space = pattern_space + (pattern.len() as f32 * gap);
    if !pattern_space.is_finite()
        || !first_repeat_space.is_finite()
        || !per_repeat_space.is_finite()
    {
        return Err(invalid(property));
    }
    let max_repetitions = GRID_MAX_TRACKS.div_ceil(pattern.len());
    let repetitions = if first_repeat_space > axis_size || per_repeat_space <= 0.0 {
        1
    } else {
        let fitting_repetitions = (axis_size - first_repeat_space) / per_repeat_space;
        if !fitting_repetitions.is_finite() || fitting_repetitions < 0.0 {
            return Err(invalid(property));
        }
        if fitting_repetitions >= max_repetitions as f32 {
            max_repetitions
        } else {
            (fitting_repetitions.floor() as usize).saturating_add(1)
        }
    };
    let expanded_tracks = repetitions
        .checked_mul(pattern.len())
        .map(|count| count.min(GRID_MAX_TRACKS))
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    expanded_tracks
        .checked_mul(other_track_count)
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))
}
