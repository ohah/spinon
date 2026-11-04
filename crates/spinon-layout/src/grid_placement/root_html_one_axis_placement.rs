use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridItemInput, GridPlacementInput,
    GridPlacementMode, GridTrackSize, LayoutAlignContent, LayoutAlignItems, LayoutDimension,
    LayoutDisplay, LayoutError, LayoutInput, LayoutJustifyContent, LayoutOverflow, LayoutPosition,
    LayoutStyle, TextDirection,
};

pub(super) fn validate_items(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    if container.placement_mode != GridPlacementMode::RootHtmlOneAxisAutoPlacementV1 {
        return Ok(());
    }
    validate_container(container_node.id, container_node.style, container)?;

    let mut grid_locked_axis = None;
    let mut item_count = 0usize;
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
        if !supported_item(input, item) {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }

        let placement = input.grid_items.get(&child).cloned().unwrap_or_default();
        let axis = locked_axis(child, &placement)?;
        if grid_locked_axis.is_some_and(|previous| previous != axis) {
            return Err(super::invalid_placement(
                child,
                "grid-row-start",
                "Root HTML one-axis mode requires every item to use the same definite axis",
            ));
        }
        grid_locked_axis = Some(axis);
        item_count = item_count
            .checked_add(1)
            .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
    }
    if item_count == 0 {
        return Err(LayoutError::GridImplicitTrackRequired(container_node.id));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DefiniteAxis {
    Row,
    Column,
}

fn locked_axis(node: NodeId, placement: &GridItemInput) -> Result<DefiniteAxis, LayoutError> {
    validate_endpoints(node, placement)?;
    let row_has_line = has_line(&placement.row_start, &placement.row_end);
    let column_has_line = has_line(&placement.column_start, &placement.column_end);
    let row_is_auto = is_auto_axis(&placement.row_start, &placement.row_end);
    let column_is_auto = is_auto_axis(&placement.column_start, &placement.column_end);

    match (row_has_line, row_is_auto, column_has_line, column_is_auto) {
        (true, false, false, true) => Ok(DefiniteAxis::Row),
        (false, true, true, false) => Ok(DefiniteAxis::Column),
        _ => Err(super::invalid_placement(
            node,
            "grid-row-start",
            "Root HTML one-axis mode requires one numeric definite axis and auto on the other axis",
        )),
    }
}

fn validate_endpoints(node: NodeId, placement: &GridItemInput) -> Result<(), LayoutError> {
    for (property, value) in [
        ("grid-row-start", &placement.row_start),
        ("grid-row-end", &placement.row_end),
        ("grid-column-start", &placement.column_start),
        ("grid-column-end", &placement.column_end),
    ] {
        match value {
            GridPlacementInput::Auto | GridPlacementInput::Line(_) => {}
            GridPlacementInput::Span(span) if *span > 0 => {}
            GridPlacementInput::Span(_) => {
                return Err(super::invalid_placement(
                    node,
                    property,
                    "Root HTML one-axis mode requires positive numeric spans",
                ));
            }
            GridPlacementInput::NamedLine { .. } | GridPlacementInput::SpanNamed { .. } => {
                return Err(super::invalid_placement(
                    node,
                    property,
                    "Root HTML one-axis mode does not support named placement",
                ));
            }
        }
    }
    Ok(())
}

fn has_line(start: &GridPlacementInput, end: &GridPlacementInput) -> bool {
    matches!(start, GridPlacementInput::Line(_)) || matches!(end, GridPlacementInput::Line(_))
}

fn is_auto_axis(start: &GridPlacementInput, end: &GridPlacementInput) -> bool {
    matches!(start, GridPlacementInput::Auto) && matches!(end, GridPlacementInput::Auto)
}

fn validate_container(
    node: NodeId,
    style: LayoutStyle,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    if container.auto_flow != GridAutoFlow::Row {
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

    let fixed_dimension = |dimension| matches!(dimension, LayoutDimension::Fixed(value) if value.is_finite() && value >= 0.0);
    let fixed_zero_gap =
        |gap| matches!(gap, crate::LayoutLengthPercentage::LengthPx(value) if value == 0.0);
    let unsupported_property = if !fixed_dimension(style.width) {
        Some("width")
    } else if !fixed_dimension(style.height) {
        Some("height")
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
    } else if style.direction != TextDirection::Ltr {
        Some("direction")
    } else if style.overflow_x != LayoutOverflow::Visible {
        Some("overflow-x")
    } else if style.overflow_y != LayoutOverflow::Visible {
        Some("overflow-y")
    } else if !fixed_zero_gap(style.gap.row) {
        Some("row-gap")
    } else if !fixed_zero_gap(style.gap.column) {
        Some("column-gap")
    } else if style.margin != crate::LayoutEdges::default() {
        Some("margin")
    } else if style.padding != crate::LayoutEdges::default() {
        Some("padding")
    } else if style.border != crate::LayoutBorder::default() {
        Some("border")
    } else if style.align_items != LayoutAlignItems::Normal {
        Some("align-items")
    } else if style
        .align_content
        .is_some_and(|value| value != LayoutAlignContent::Normal)
    {
        Some("align-content")
    } else if !matches!(
        style.justify_content,
        LayoutJustifyContent::Normal | LayoutJustifyContent::FlexStart
    ) {
        Some("justify-content")
    } else if container.justify_items != LayoutAlignItems::Normal {
        Some("justify-items")
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
    size: &GridAutoRows,
) -> Result<(), LayoutError> {
    match size {
        GridAutoRows::Auto => Ok(()),
        GridAutoRows::LengthPx(value) if value.is_finite() && *value >= 0.0 => Ok(()),
        GridAutoRows::LengthPx(_) => Err(LayoutError::InvalidGridTrack { node, property }),
        GridAutoRows::TrackList(_) if property == "grid-auto-rows" => {
            Err(LayoutError::UnsupportedGridAutoRows {
                node,
                reason: "Root HTML one-axis mode requires no implicit tracks",
            })
        }
        GridAutoRows::TrackList(_) => Err(LayoutError::UnsupportedGridAutoColumns {
            node,
            reason: "Root HTML one-axis mode requires no implicit tracks",
        }),
    }
}

fn supported_item(input: &LayoutInput, item: &crate::LayoutNode) -> bool {
    let auto_or_fixed = |dimension| match dimension {
        LayoutDimension::Auto => true,
        LayoutDimension::Fixed(value) => value.is_finite() && value >= 0.0,
        _ => false,
    };
    item.style.display == LayoutDisplay::Block
        && item.children.is_empty()
        && !input.grid_containers.contains_key(&item.id)
        && !input
            .text_paragraphs
            .get(&item.id)
            .is_some_and(crate::TextParagraphInput::has_content)
        && auto_or_fixed(item.style.width)
        && auto_or_fixed(item.style.height)
        && item.style.min_width == LayoutDimension::Auto
        && item.style.max_width == LayoutDimension::Auto
        && item.style.min_height == LayoutDimension::Auto
        && item.style.max_height == LayoutDimension::Auto
        && item.style.aspect_ratio.is_none()
        && item.style.order == 0
        && item.style.align_self == crate::LayoutAlignSelf::Auto
        && item.style.justify_self == crate::LayoutAlignSelf::Auto
        && item.style.margin == crate::LayoutEdges::default()
        && item.style.padding == crate::LayoutEdges::default()
        && item.style.border == crate::LayoutBorder::default()
        && item.style.overflow_x == LayoutOverflow::Visible
        && item.style.overflow_y == LayoutOverflow::Visible
}

fn is_nonnegative_px(track: &GridTrackSize) -> bool {
    matches!(track, GridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}
