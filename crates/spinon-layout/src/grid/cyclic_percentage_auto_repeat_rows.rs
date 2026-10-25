use super::*;
use crate::LayoutGap;

pub(super) fn fixture_cyclic_percentage_auto_repeat_generalized_rows_scene(
    input: &LayoutInput,
    node: &crate::LayoutNode,
    container: &GridContainerInput,
    allow_definite_control: bool,
) -> bool {
    if input.root != node.id
        || input.root_sizing != crate::RootSizingPolicy::ResolveWithin
        || input.nodes.len() != 4
        || input.grid_containers.len() != 1
        || input.grid_items.len() != 3
        || node.children.len() != 3
        || container.rows_auto_repeat.is_none()
        || container.columns_auto_repeat.is_some()
    {
        return false;
    }
    let [prefix_id, primary_id, secondary_id] = node.children.as_slice() else {
        return false;
    };
    let Some(prefix) = input
        .nodes
        .iter()
        .find(|candidate| candidate.id == *prefix_id)
    else {
        return false;
    };
    let Some(primary) = input
        .nodes
        .iter()
        .find(|candidate| candidate.id == *primary_id)
    else {
        return false;
    };
    let Some(secondary) = input
        .nodes
        .iter()
        .find(|candidate| candidate.id == *secondary_id)
    else {
        return false;
    };
    let height = match node.style.height {
        LayoutDimension::MaxContent => LayoutDimension::MaxContent,
        LayoutDimension::Fixed(44.0) if allow_definite_control => LayoutDimension::Fixed(44.0),
        _ => return false,
    };
    let expected_root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        box_sizing: LayoutBoxSizing::ContentBox,
        width: LayoutDimension::Fixed(24.0),
        height,
        flex_direction: crate::FlexDirection::Row,
        flex_shrink: 1.0,
        align_content: Some(crate::LayoutAlignContent::Position {
            position: crate::ContentAlignmentPosition::Start,
            safety: None,
        }),
        justify_content: crate::LayoutJustifyContent::Position {
            position: crate::JustifyContentPosition::Start,
            safety: None,
        },
        gap: LayoutGap {
            row: LayoutLengthPercentage::LengthPx(2.0),
            column: LayoutLengthPercentage::ZERO,
        },
        ..LayoutStyle::default()
    };
    let expected_item_style = |width, height| LayoutStyle {
        display: LayoutDisplay::Block,
        box_sizing: LayoutBoxSizing::BorderBox,
        width: LayoutDimension::Fixed(width),
        height: LayoutDimension::Fixed(height),
        min_width: LayoutDimension::Fixed(0.0),
        min_height: LayoutDimension::Fixed(0.0),
        flex_direction: crate::FlexDirection::Row,
        flex_shrink: 1.0,
        align_content: Some(crate::LayoutAlignContent::Normal),
        ..LayoutStyle::default()
    };
    let Some(repeat) = container.rows_auto_repeat.as_ref() else {
        return false;
    };
    let static_auto_insets = crate::LayoutEdges {
        top: LayoutLengthPercentage::Auto,
        right: LayoutLengthPercentage::Auto,
        bottom: LayoutLengthPercentage::Auto,
        left: LayoutLengthPercentage::Auto,
    };
    let placement_matches = input.grid_items.get(prefix_id)
        == Some(&crate::GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Auto,
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Auto,
        })
        && [primary_id, secondary_id].into_iter().all(|item_id| {
            input.grid_items.get(item_id)
                == Some(&crate::GridItemInput {
                    row_start: GridPlacementInput::Line(2),
                    row_end: GridPlacementInput::Auto,
                    column_start: GridPlacementInput::Line(1),
                    column_end: GridPlacementInput::Auto,
                })
        });

    node.style == expected_root_style
        && container.placement_mode == crate::GridPlacementMode::TrackSizing
        && container.rows.as_slice()
            == [
                GridTrackSize::LengthPx(8.0),
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(0.0),
                    max: GridTrackMaxSizingFunction::Percentage(0.25),
                },
                GridTrackSize::LengthPx(8.0),
            ]
        && matches!(
            repeat.mode,
            GridAutoRepeatMode::AutoFill | GridAutoRepeatMode::AutoFit
        )
        && repeat.start == 1
        && repeat.end == 2
        && repeat.line_names.iter().all(Vec::is_empty)
        && container.rows_line_names.iter().all(Vec::is_empty)
        && container.columns.as_slice() == [GridTrackSize::LengthPx(24.0)]
        && container.columns_line_names.iter().all(Vec::is_empty)
        && container.template_areas.is_none()
        && container.auto_flow == crate::GridAutoFlow::Row
        && container.auto_rows == crate::GridAutoRows::Auto
        && container.auto_columns == crate::GridAutoColumns::Auto
        && prefix.children.is_empty()
        && prefix.style == expected_item_style(24.0, 8.0)
        && primary.children.is_empty()
        && primary.style == expected_item_style(24.0, 24.0)
        && secondary.children.is_empty()
        && secondary.style == expected_item_style(16.0, 16.0)
        && placement_matches
        && input.positioning.len() == 4
        && [node.id, *prefix_id, *primary_id, *secondary_id]
            .into_iter()
            .all(|node_id| input.positioning.contains_key(&node_id))
        && input.positioning.values().all(|positioning| {
            positioning.position == LayoutPosition::Static
                && positioning.inset == static_auto_insets
        })
}
