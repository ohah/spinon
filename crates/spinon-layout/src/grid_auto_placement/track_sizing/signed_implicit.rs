use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::{GridContainerInput, LayoutError, LayoutInput};

use super::super::reservation::{GridReservationSite, reserve_exact};
use super::{
    DenseGridPlacement, Item,
    axis::resolve_axis,
    extent::{Extent, raw_track_end, validate_count},
    lines::{ItemLines, definite_item_lines},
    tracks,
};

/// C11.3.3 signed named-line 경로는 definite placement만 leading/trailing track으로 확장합니다.
pub(in crate::grid_auto_placement) fn normalize_signed_implicit_grid_container(
    input: &LayoutInput,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    let node = container_node.id;
    validate_count(node, "grid-template-columns", container.columns.len())?;
    validate_count(node, "grid-template-rows", container.rows.len())?;

    let visible_count = container_node
        .children
        .iter()
        .filter(|child| !hidden[child])
        .count();
    let mut items = Vec::new();
    reserve_exact(
        &mut items,
        visible_count,
        node,
        "grid-placement-occupancy",
        GridReservationSite::VisibleItemList,
    )?;
    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let default_placement = crate::GridItemInput::default();
        let placement = input.grid_items.get(&child).unwrap_or(&default_placement);
        let row = resolve_axis(
            child,
            "grid-row-start",
            "grid-row-end",
            &placement.row_start,
            &placement.row_end,
            container.rows.len(),
        )?;
        let column = resolve_axis(
            child,
            "grid-column-start",
            "grid-column-end",
            &placement.column_start,
            &placement.column_end,
            container.columns.len(),
        )?;
        if row.start.is_none() || column.start.is_none() {
            return Err(LayoutError::GridImplicitTrackRequired(node));
        }
        items.push(Item {
            node: child,
            row,
            column,
        });
    }

    let mut row_extent = Extent::explicit(node, "grid-template-rows", container.rows.len())?;
    let mut column_extent =
        Extent::explicit(node, "grid-template-columns", container.columns.len())?;
    let mut pending_placements = BTreeMap::new();
    for item in &items {
        let row_start = item
            .row
            .start
            .ok_or(LayoutError::GridImplicitTrackRequired(node))?;
        let column_start = item
            .column
            .start
            .ok_or(LayoutError::GridImplicitTrackRequired(node))?;
        let row_end = raw_track_end(node, row_start, item.row.span)?;
        let column_end = raw_track_end(node, column_start, item.column.span)?;
        row_extent.include(row_start, row_end);
        column_extent.include(column_start, column_end);
        pending_placements.insert(
            item.node,
            ItemLines {
                row_start,
                row_end,
                column_start,
                column_end,
            },
        );
    }

    row_extent.count(node, "grid-template-rows")?;
    column_extent.count(node, "grid-template-columns")?;
    let row_leading = row_extent.leading_count(node)?;
    let column_leading = column_extent.leading_count(node)?;
    let row_trailing = row_extent.trailing_count(node, container.rows.len())?;
    let column_trailing = column_extent.trailing_count(node, container.columns.len())?;
    let row_shift = row_extent.origin_shift(node)?;
    let column_shift = column_extent.origin_shift(node)?;
    let mut item_placements = BTreeMap::new();
    for (item, lines) in pending_placements {
        item_placements.insert(
            item,
            definite_item_lines(item, lines, row_shift, column_shift)?,
        );
    }

    Ok(DenseGridPlacement {
        item_placements,
        expanded_rows: tracks::expand_tracks(
            node,
            "grid-template-rows",
            &container.rows,
            row_leading,
            row_trailing,
            &container.auto_rows,
            true,
        )?,
        expanded_columns: tracks::expand_tracks(
            node,
            "grid-template-columns",
            &container.columns,
            column_leading,
            column_trailing,
            &container.auto_columns,
            false,
        )?,
        auto_placed_items: BTreeSet::new(),
    })
}
