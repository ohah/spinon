use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    GridAutoFlow, GridAutoTrackSize, GridContainerInput, GridItemInput, GridPlacementInput,
    GridPlacementMode, GridTrackSize, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutError,
    LayoutInput, LayoutJustifyContent, LayoutLengthPercentage, LayoutOverflow, LayoutPosition,
    LayoutStyle, TextDirection,
};

pub(super) fn validate_items(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    let node = container_node.id;
    if container.placement_mode != GridPlacementMode::RootHtmlDenseAutoPlacementV1 {
        return Ok(());
    }
    validate_container(node, container_node.style, container)?;

    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let item = &input.nodes[index[&child]];
        if item.style.order != 0 {
            return Err(LayoutError::UnsupportedGridOrder(child));
        }
        if input
            .positioning
            .get(&child)
            .is_some_and(|position| position.position != LayoutPosition::Static)
        {
            return Err(LayoutError::UnsupportedGridPositionedItem(child));
        }
        let placement = input.grid_items.get(&child).cloned().unwrap_or_default();
        validate_placement(child, &placement)?;
        if !supported_item(input, item) {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }
    }
    Ok(())
}

fn validate_container(
    node: NodeId,
    style: LayoutStyle,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    if !matches!(
        container.auto_flow,
        GridAutoFlow::RowDense | GridAutoFlow::ColumnDense
    ) {
        return Err(LayoutError::UnsupportedGridAutoFlow(node));
    }
    for (property, tracks) in [
        ("grid-template-columns", container.columns.as_slice()),
        ("grid-template-rows", container.rows.as_slice()),
    ] {
        if tracks.is_empty() {
            return Err(LayoutError::InvalidGridTrackList { node, property });
        }
        if tracks.len() > crate::GRID_MAX_TRACKS {
            return Err(LayoutError::GridTrackLimitExceeded {
                node,
                property,
                count: tracks.len(),
                maximum: crate::GRID_MAX_TRACKS,
            });
        }
        if tracks.iter().any(|track| !is_nonnegative_px(track)) {
            return Err(LayoutError::InvalidGridTrackList { node, property });
        }
    }
    if container.columns_auto_repeat.is_some()
        || container.rows_auto_repeat.is_some()
        || container.template_areas.is_some()
        || container
            .columns_line_names
            .iter()
            .chain(&container.rows_line_names)
            .any(|group| !group.is_empty())
    {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }
    validate_auto_track(node, "grid-auto-rows", &container.auto_rows)?;
    validate_auto_track(node, "grid-auto-columns", &container.auto_columns)?;

    let fixed_box = |dimension| matches!(dimension, LayoutDimension::Fixed(value) if value.is_finite() && value >= 0.0);
    let fixed_gap = |gap| matches!(gap, LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0);
    let unsupported_property = if !fixed_box(style.width) {
        Some("width")
    } else if !fixed_box(style.height) {
        Some("height")
    } else if style.box_sizing != LayoutBoxSizing::ContentBox {
        Some("box-sizing")
    } else if style.direction != TextDirection::Ltr {
        Some("direction")
    } else if !fixed_gap(style.gap.row) {
        Some("row-gap")
    } else if !fixed_gap(style.gap.column) {
        Some("column-gap")
    } else if style.overflow_x != LayoutOverflow::Visible {
        Some("overflow-x")
    } else if style.overflow_y != LayoutOverflow::Visible {
        Some("overflow-y")
    } else if !matches!(
        style.align_items,
        crate::LayoutAlignItems::Normal | crate::LayoutAlignItems::Stretch
    ) {
        Some("align-items")
    } else if style
        .align_content
        .is_some_and(|value| value != crate::LayoutAlignContent::Normal)
    {
        Some("align-content")
    } else if !matches!(
        style.justify_content,
        LayoutJustifyContent::Normal | LayoutJustifyContent::FlexStart
    ) {
        Some("justify-content")
    } else if container.justify_items != crate::LayoutAlignItems::Normal {
        Some("justify-items")
    } else if style.min_width != LayoutDimension::Auto {
        Some("min-width")
    } else if style.max_width != LayoutDimension::Auto {
        Some("max-width")
    } else if style.min_height != LayoutDimension::Auto {
        Some("min-height")
    } else if style.max_height != LayoutDimension::Auto {
        Some("max-height")
    } else if style.aspect_ratio.is_some() {
        Some("aspect-ratio")
    } else if style.margin != crate::LayoutEdges::default() {
        Some("margin")
    } else if style.padding != crate::LayoutEdges::default() {
        Some("padding")
    } else if style.border != crate::LayoutBorder::default() {
        Some("border")
    } else {
        None
    };
    if let Some(property) = unsupported_property {
        return Err(LayoutError::UnsupportedGridContainerStyle { node, property });
    }
    Ok(())
}

fn validate_auto_track(
    node: NodeId,
    property: &'static str,
    track: &GridAutoTrackSize,
) -> Result<(), LayoutError> {
    match track {
        GridAutoTrackSize::Auto => Ok(()),
        GridAutoTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0 => Ok(()),
        GridAutoTrackSize::LengthPx(_) => Err(LayoutError::InvalidGridTrack { node, property }),
        GridAutoTrackSize::TrackList(_) => {
            if property == "grid-auto-rows" {
                Err(LayoutError::UnsupportedGridAutoRows {
                    node,
                    reason: "Root HTML dense mode requires one fixed px implicit row size",
                })
            } else {
                Err(LayoutError::UnsupportedGridAutoColumns {
                    node,
                    reason: "Root HTML dense mode requires one fixed px implicit column size",
                })
            }
        }
    }
}

fn validate_placement(node: NodeId, placement: &GridItemInput) -> Result<(), LayoutError> {
    for (property, value) in [
        ("grid-row-start", &placement.row_start),
        ("grid-row-end", &placement.row_end),
        ("grid-column-start", &placement.column_start),
        ("grid-column-end", &placement.column_end),
    ] {
        let valid = match value {
            GridPlacementInput::Auto => true,
            GridPlacementInput::Line(line) => *line > 0,
            GridPlacementInput::Span(span) => *span > 0,
            GridPlacementInput::NamedLine { .. } | GridPlacementInput::SpanNamed { .. } => false,
        };
        if !valid {
            return Err(super::invalid_placement(
                node,
                property,
                "Root HTML dense mode requires auto, positive lines, or positive spans",
            ));
        }
    }
    Ok(())
}

fn supported_item(input: &LayoutInput, item: &crate::LayoutNode) -> bool {
    let fixed = |dimension| matches!(dimension, LayoutDimension::Fixed(value) if value.is_finite() && value >= 0.0);
    matches!(
        item.style.display,
        LayoutDisplay::Block | LayoutDisplay::Grid
    ) && (item.children.is_empty() || item.style.display == LayoutDisplay::Grid)
        && (!input.grid_containers.contains_key(&item.id)
            || item.style.display == LayoutDisplay::Grid)
        && !input
            .text_paragraphs
            .get(&item.id)
            .is_some_and(crate::TextParagraphInput::has_content)
        && fixed(item.style.width)
        && fixed(item.style.height)
        && item.style.order == 0
        && item.style.min_width == LayoutDimension::Auto
        && item.style.max_width == LayoutDimension::Auto
        && item.style.min_height == LayoutDimension::Auto
        && item.style.max_height == LayoutDimension::Auto
        && item.style.aspect_ratio.is_none()
        && item.style.margin == crate::LayoutEdges::default()
        && item.style.padding == crate::LayoutEdges::default()
        && item.style.border == crate::LayoutBorder::default()
        && item.style.overflow_x == LayoutOverflow::Visible
        && item.style.overflow_y == LayoutOverflow::Visible
        && item.style.align_self == crate::LayoutAlignSelf::Auto
}

fn is_nonnegative_px(track: &GridTrackSize) -> bool {
    matches!(track, GridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}

#[cfg(test)]
mod tests {
    use super::{validate_auto_track, validate_placement};
    use crate::{GridAutoTrackSize, GridItemInput, GridPlacementInput, GridTrackSize, LayoutError};
    use spinon_core::NodeId;

    #[test]
    fn auto_implicit_track_value_is_admitted_for_no_expansion_cases() {
        assert!(
            validate_auto_track(
                NodeId::new(1).unwrap(),
                "grid-auto-rows",
                &GridAutoTrackSize::Auto
            )
            .is_ok()
        );
    }

    #[test]
    fn auto_track_pattern_lists_are_rejected() {
        let node = NodeId::new(1).unwrap();
        let error = validate_auto_track(
            node,
            "grid-auto-columns",
            &GridAutoTrackSize::TrackList(vec![GridTrackSize::LengthPx(10.0)]),
        )
        .unwrap_err();
        assert!(
            matches!(error, LayoutError::UnsupportedGridAutoColumns { node: actual, .. } if actual == node)
        );
    }

    #[test]
    fn placements_reject_zero_negative_and_named_lines_but_keep_positive_spans() {
        let node = NodeId::new(1).unwrap();
        let mut placement = GridItemInput {
            column_end: GridPlacementInput::Span(2),
            ..GridItemInput::default()
        };
        assert!(validate_placement(node, &placement).is_ok());

        placement.column_end = GridPlacementInput::Line(0);
        assert!(matches!(
            validate_placement(node, &placement),
            Err(LayoutError::InvalidGridPlacement {
                property: "grid-column-end",
                ..
            })
        ));
        placement.column_end = GridPlacementInput::Span(0);
        assert!(matches!(
            validate_placement(node, &placement),
            Err(LayoutError::InvalidGridPlacement {
                property: "grid-column-end",
                ..
            })
        ));
        placement.column_end = GridPlacementInput::NamedLine {
            name: "x".to_owned(),
            occurrence: 1,
        };
        assert!(matches!(
            validate_placement(node, &placement),
            Err(LayoutError::InvalidGridPlacement {
                property: "grid-column-end",
                ..
            })
        ));
    }
}
