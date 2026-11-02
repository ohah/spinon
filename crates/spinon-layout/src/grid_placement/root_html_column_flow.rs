use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode, LayoutError,
    LayoutInput, grid_auto_placement::DenseGridPlacement,
};

pub(super) fn validate_items(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    let node = container_node.id;
    if container.placement_mode != GridPlacementMode::RootHtmlColumnFlowV1 {
        return Ok(());
    }
    let capacity = container
        .columns
        .len()
        .checked_mul(container.rows.len())
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    let visible_count = container_node
        .children
        .iter()
        .filter(|child| !hidden[child])
        .count();
    if visible_count > capacity {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }
    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let child_node = &input.nodes[index[&child]];
        if child_node.style.order != 0 {
            return Err(LayoutError::UnsupportedGridOrder(child));
        }
        if input
            .grid_items
            .get(&child)
            .is_some_and(|placement| !placement.is_auto())
        {
            return Err(super::invalid_placement(
                child,
                "grid-row-start",
                "Root HTML column-flow mode requires all four placement longhands to be auto",
            ));
        }
        if input.grid_containers.contains_key(&child)
            || !crate::grid::root_html_fixed_empty_item_is_supported(input, child_node)
        {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }
    }
    Ok(())
}

pub(super) fn normalize(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    validate_items(input, index, hidden, container_node, container)?;
    let placement = crate::grid_auto_placement::normalize_dense_grid_container(
        input,
        hidden,
        container_node,
        container,
    )?;
    let node = container_node.id;
    let visible_count = container_node
        .children
        .iter()
        .filter(|child| !hidden[child])
        .count();
    let positions_are_explicit = |item: &GridItemInput| {
        fn axis_fits(start: &GridPlacementInput, end: &GridPlacementInput, tracks: usize) -> bool {
            matches!(
                (start, end),
                (GridPlacementInput::Line(start), GridPlacementInput::Line(end))
                    if *start >= 1
                        && *end > *start
                        && usize::try_from(*end - 1).is_ok_and(|last_line| last_line <= tracks)
            )
        }
        axis_fits(&item.row_start, &item.row_end, container.rows.len())
            && axis_fits(
                &item.column_start,
                &item.column_end,
                container.columns.len(),
            )
    };
    if placement.expanded_rows.is_some()
        || placement.expanded_columns.is_some()
        || placement.item_placements.len() != visible_count
        || placement.auto_placed_items.len() != visible_count
        || placement
            .item_placements
            .values()
            .any(|item| !positions_are_explicit(item))
    {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }
    Ok(placement)
}
