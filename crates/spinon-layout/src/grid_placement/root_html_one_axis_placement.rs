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
    let mixed_explicit =
        container.placement_mode == GridPlacementMode::RootHtmlMixedExplicitOneAxisAutoPlacementV1;
    let mixed_cross_axis_column =
        container.placement_mode == GridPlacementMode::RootHtmlMixedCrossAxisColumnAutoPlacementV1;
    let mixed_cross_axis = matches!(
        container.placement_mode,
        GridPlacementMode::RootHtmlMixedCrossAxisAutoPlacementV1
            | GridPlacementMode::RootHtmlMixedCrossAxisColumnAutoPlacementV1
    );
    let intrinsic_track_sizing =
        container.placement_mode == GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    if container.placement_mode != GridPlacementMode::RootHtmlOneAxisAutoPlacementV1
        && !mixed_explicit
        && !mixed_cross_axis
        && !intrinsic_track_sizing
    {
        return Ok(());
    }
    validate_container(
        container_node.id,
        container_node.style,
        container,
        mixed_cross_axis_column,
        intrinsic_track_sizing,
    )?;

    let mut grid_locked_axis = None;
    let mut item_count = 0usize;
    let mut first_item = None;
    let mut fully_definite_count = 0usize;
    let mut one_axis_count = 0usize;
    let mut fully_auto_count = 0usize;
    let mut row_locked_count = 0usize;
    let mut column_locked_count = 0usize;
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
        if !supported_item(input, item, container_node.id, intrinsic_track_sizing) {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }

        let placement = input.grid_items.get(&child).cloned().unwrap_or_default();
        validate_endpoints(child, &placement, mixed_cross_axis)?;
        if first_item.is_none() {
            first_item = Some(child);
        }
        if intrinsic_track_sizing && is_fully_auto(&placement) {
            fully_auto_count = fully_auto_count
                .checked_add(1)
                .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
        } else if (mixed_explicit || mixed_cross_axis) && is_fully_definite(&placement) {
            fully_definite_count = fully_definite_count
                .checked_add(1)
                .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
        } else if mixed_cross_axis && is_fully_auto(&placement) {
            // Fully auto items take part after both explicit and row-locked placement phases.
        } else {
            let axis = locked_axis(child, &placement, mixed_cross_axis)?;
            if mixed_cross_axis {
                match axis {
                    DefiniteAxis::Row => {
                        row_locked_count = row_locked_count
                            .checked_add(1)
                            .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
                    }
                    DefiniteAxis::Column => {
                        column_locked_count = column_locked_count
                            .checked_add(1)
                            .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
                    }
                }
            } else {
                if grid_locked_axis.is_some_and(|previous| previous != axis) {
                    return Err(super::invalid_placement(
                        child,
                        "grid-row-start",
                        "Root HTML one-axis mode requires every partial item to use the same definite axis",
                    ));
                }
                grid_locked_axis = Some(axis);
            }
            one_axis_count = one_axis_count
                .checked_add(1)
                .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
        }
        item_count = item_count
            .checked_add(1)
            .ok_or(LayoutError::GridImplicitTrackRequired(container_node.id))?;
    }
    if item_count == 0 {
        return Err(LayoutError::GridImplicitTrackRequired(container_node.id));
    }
    if mixed_explicit && (fully_definite_count == 0 || one_axis_count == 0) {
        return Err(super::invalid_placement(
            first_item.unwrap_or(container_node.id),
            "grid-row-start",
            "Root HTML mixed placement requires fully definite and same-axis partial items",
        ));
    }
    if mixed_cross_axis && (row_locked_count == 0 || column_locked_count == 0) {
        return Err(super::invalid_placement(
            first_item.unwrap_or(container_node.id),
            "grid-row-start",
            "Root HTML mixed cross-axis mode requires both row-locked and column-locked items",
        ));
    }
    if intrinsic_track_sizing && one_axis_count != 0 && fully_auto_count != 0 {
        return Err(super::invalid_placement(
            first_item.unwrap_or(container_node.id),
            "grid-row-start",
            "Root HTML intrinsic track sizing does not mix auto and one-axis placed items",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DefiniteAxis {
    Row,
    Column,
}

fn locked_axis(
    node: NodeId,
    placement: &GridItemInput,
    mixed_cross_axis: bool,
) -> Result<DefiniteAxis, LayoutError> {
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
            if mixed_cross_axis {
                "Root HTML mixed cross-axis mode requires one numeric definite axis and auto on the other axis"
            } else {
                "Root HTML one-axis mode requires one numeric definite axis and auto on the other axis"
            },
        )),
    }
}

pub(super) fn is_fully_definite(placement: &GridItemInput) -> bool {
    has_line(&placement.row_start, &placement.row_end)
        && has_line(&placement.column_start, &placement.column_end)
        && !is_auto_axis(&placement.row_start, &placement.row_end)
        && !is_auto_axis(&placement.column_start, &placement.column_end)
}

fn is_fully_auto(placement: &GridItemInput) -> bool {
    is_auto_axis(&placement.row_start, &placement.row_end)
        && is_auto_axis(&placement.column_start, &placement.column_end)
}

fn validate_endpoints(
    node: NodeId,
    placement: &GridItemInput,
    mixed_cross_axis: bool,
) -> Result<(), LayoutError> {
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
                    if mixed_cross_axis {
                        "Root HTML mixed cross-axis mode requires positive numeric spans"
                    } else {
                        "Root HTML one-axis mode requires positive numeric spans"
                    },
                ));
            }
            GridPlacementInput::NamedLine { .. } | GridPlacementInput::SpanNamed { .. } => {
                return Err(super::invalid_placement(
                    node,
                    property,
                    if mixed_cross_axis {
                        "Root HTML mixed cross-axis mode does not support named placement"
                    } else {
                        "Root HTML one-axis mode does not support named placement"
                    },
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
    column_flow: bool,
    intrinsic_track_sizing: bool,
) -> Result<(), LayoutError> {
    if (column_flow && container.auto_flow != GridAutoFlow::Column)
        || (!column_flow && container.auto_flow != GridAutoFlow::Row)
    {
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
        if tracks.iter().any(|track| {
            if intrinsic_track_sizing {
                !crate::grid::root_html_intrinsic_track_is_supported(track)
            } else {
                !is_nonnegative_px(track)
            }
        }) {
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

    let fixed_dimension = |dimension| {
        matches!(
            dimension,
            LayoutDimension::Fixed(value)
                if value.is_finite()
                    && (value > 0.0 || (!intrinsic_track_sizing && value == 0.0))
        )
    };
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
    } else if style.align_content.is_some_and(|value| {
        if intrinsic_track_sizing {
            !matches!(
                value,
                LayoutAlignContent::Normal | LayoutAlignContent::FlexStart
            ) && !matches!(
                value,
                LayoutAlignContent::Position {
                    position: crate::ContentAlignmentPosition::Start,
                    safety: None,
                }
            )
        } else {
            value != LayoutAlignContent::Normal
        }
    }) {
        Some("align-content")
    } else if !(matches!(
        style.justify_content,
        LayoutJustifyContent::Normal | LayoutJustifyContent::FlexStart
    ) || intrinsic_track_sizing
        && matches!(
            style.justify_content,
            LayoutJustifyContent::Position {
                position: crate::JustifyContentPosition::Start,
                safety: None,
            }
        ))
    {
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
                reason: "Root HTML one-axis placement modes require no implicit tracks",
            })
        }
        GridAutoRows::TrackList(_) => Err(LayoutError::UnsupportedGridAutoColumns {
            node,
            reason: "Root HTML one-axis placement modes require no implicit tracks",
        }),
    }
}

fn supported_item(
    input: &LayoutInput,
    item: &crate::LayoutNode,
    grid_container: NodeId,
    intrinsic_track_sizing: bool,
) -> bool {
    let auto_or_fixed = |dimension| match dimension {
        LayoutDimension::Auto => true,
        LayoutDimension::Fixed(value) => value.is_finite() && value >= 0.0,
        _ => false,
    };
    let has_text_content = input
        .text_paragraphs
        .get(&item.id)
        .is_some_and(crate::TextParagraphInput::has_content);
    let root_html_default_grid_text_item = intrinsic_track_sizing
        && matches!(
            input.text_paragraphs.get(&item.id),
            Some(crate::TextParagraphInput::AnonymousGridText {
                grid_container: owner,
                root_html_default: true,
                ..
            }) if *owner == grid_container
        );
    let min_constraint_is_supported = |dimension| {
        dimension == LayoutDimension::Auto
            || (intrinsic_track_sizing && dimension == LayoutDimension::Fixed(0.0))
    };
    item.style.display == LayoutDisplay::Block
        && item.children.is_empty()
        && !input.grid_containers.contains_key(&item.id)
        && (!has_text_content || intrinsic_track_sizing || root_html_default_grid_text_item)
        && auto_or_fixed(item.style.width)
        && auto_or_fixed(item.style.height)
        && min_constraint_is_supported(item.style.min_width)
        && item.style.max_width == LayoutDimension::Auto
        && min_constraint_is_supported(item.style.min_height)
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
