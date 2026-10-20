use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::{GridAutoFlow, GridContainerInput, LayoutError, LayoutInput};

use super::{
    DenseGridPlacement,
    reservation::{GridReservationSite, reserve_exact},
};

#[path = "track_sizing/axis.rs"]
mod axis;
#[path = "track_sizing/extent.rs"]
mod extent;
#[path = "track_sizing/lines.rs"]
mod lines;
#[path = "track_sizing/occupancy.rs"]
mod occupancy;
#[path = "track_sizing/tracks.rs"]
mod tracks;

use axis::{AxisPlacement, resolve_axis};
use extent::{Extent, coordinate_index, next_coordinate, raw_track_end, track_end, validate_count};
use lines::{ItemLines, definite_item_lines};
use occupancy::Occupancy;

struct Item {
    node: NodeId,
    row: AxisPlacement,
    column: AxisPlacement,
}

pub(super) fn normalize_track_sizing_grid_container(
    input: &LayoutInput,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    let node = container_node.id;
    validate_count(node, "grid-template-columns", container.columns.len())?;
    validate_count(node, "grid-template-rows", container.rows.len())?;

    let row_flow = matches!(
        container.auto_flow,
        GridAutoFlow::Row | GridAutoFlow::RowDense
    );
    let dense = matches!(
        container.auto_flow,
        GridAutoFlow::RowDense | GridAutoFlow::ColumnDense
    );
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
    let mut auto_placed_items = BTreeSet::new();
    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let placement = input.grid_items.get(&child).copied().unwrap_or_default();
        let row = resolve_axis(
            child,
            "grid-row-start",
            "grid-row-end",
            placement.row_start,
            placement.row_end,
            container.rows.len(),
        )?;
        let column = resolve_axis(
            child,
            "grid-column-start",
            "grid-column-end",
            placement.column_start,
            placement.column_end,
            container.columns.len(),
        )?;
        if row.start.is_none() || column.start.is_none() {
            auto_placed_items.insert(child);
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
    for item in &items {
        if let Some(start) = item.row.start {
            let end = raw_track_end(node, start, item.row.span)?;
            row_extent.include(start, end);
        }
        if let Some(start) = item.column.start {
            let end = raw_track_end(node, start, item.column.span)?;
            column_extent.include(start, end);
        }
    }
    row_extent.count(node, "grid-template-rows")?;
    column_extent.count(node, "grid-template-columns")?;

    let row_origin = row_extent.start;
    let column_origin = column_extent.start;
    let mut occupancy = Occupancy::new(row_origin);
    let mut pending_placements = BTreeMap::new();
    let mut placed = BTreeSet::new();

    // CSS Grid §8.5 step 1: occupy all definite positions before auto placement.
    for item in &items {
        let (Some(row_start), Some(column_start)) = (item.row.start, item.column.start) else {
            continue;
        };
        let row_end = track_end(
            node,
            "grid-template-rows",
            row_start,
            item.row.span,
            row_origin,
        )?;
        let column_end = track_end(
            node,
            "grid-template-columns",
            column_start,
            item.column.span,
            column_origin,
        )?;
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        row_extent.include_end(node, "grid-template-rows", row_end)?;
        column_extent.include_end(node, "grid-template-columns", column_end)?;
        pending_placements.insert(
            item.node,
            ItemLines {
                row_start,
                row_end,
                column_start,
                column_end,
            },
        );
        placed.insert(item.node);
    }

    // Step 2: place items locked to a row or column using signed cursor coordinates.
    let locked_origin = if row_flow { row_origin } else { column_origin };
    let moving_origin = if row_flow { column_origin } else { row_origin };
    let locked_count = if row_flow {
        row_extent.count(node, "grid-template-rows")?
    } else {
        column_extent.count(node, "grid-template-columns")?
    };
    let mut locked_cursor = Vec::new();
    reserve_exact(
        &mut locked_cursor,
        locked_count,
        node,
        "grid-placement-occupancy",
        GridReservationSite::RowLockedCursor,
    )?;
    locked_cursor.resize(locked_count, moving_origin);
    for item in &items {
        if placed.contains(&item.node) {
            continue;
        }
        let locked_line = if row_flow {
            item.row.start
        } else {
            item.column.start
        };
        let automatic_secondary = if row_flow {
            item.column.start.is_none()
        } else {
            item.row.start.is_none()
        };
        let Some(locked_line) = locked_line.filter(|_| automatic_secondary) else {
            continue;
        };
        let locked_index = coordinate_index(node, locked_line, locked_origin)?;
        let cursor = locked_cursor
            .get(locked_index)
            .copied()
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        let (fixed_start, fixed_span, moving_span) = if row_flow {
            (locked_line, item.row.span, item.column.span)
        } else {
            (locked_line, item.column.span, item.row.span)
        };
        let fixed_end = if row_flow {
            track_end(
                node,
                "grid-template-rows",
                fixed_start,
                fixed_span,
                row_origin,
            )?
        } else {
            track_end(
                node,
                "grid-template-columns",
                fixed_start,
                fixed_span,
                column_origin,
            )?
        };
        let mut moving_start = if dense { moving_origin } else { cursor };
        let (row_start, row_end, column_start, column_end) = loop {
            let moving_end = if row_flow {
                track_end(
                    node,
                    "grid-template-columns",
                    moving_start,
                    moving_span,
                    column_origin,
                )?
            } else {
                track_end(
                    node,
                    "grid-template-rows",
                    moving_start,
                    moving_span,
                    row_origin,
                )?
            };
            let overlaps = if row_flow {
                occupancy.overlaps(node, locked_line, fixed_end, moving_start, moving_end)?
            } else {
                occupancy.overlaps(node, moving_start, moving_end, locked_line, fixed_end)?
            };
            if !overlaps {
                break if row_flow {
                    (locked_line, fixed_end, moving_start, moving_end)
                } else {
                    (moving_start, moving_end, locked_line, fixed_end)
                };
            }
            moving_start = next_coordinate(node, moving_start)?;
        };
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        row_extent.include_end(node, "grid-template-rows", row_end)?;
        column_extent.include_end(node, "grid-template-columns", column_end)?;
        if !dense {
            locked_cursor[locked_index] = if row_flow { column_end } else { row_end };
        }
        pending_placements.insert(
            item.node,
            ItemLines {
                row_start,
                row_end,
                column_start,
                column_end,
            },
        );
        placed.insert(item.node);
    }

    // Step 3: ensure an auto-flow secondary axis can fit every item's span.
    if row_flow {
        let minimum = items
            .iter()
            .filter(|item| item.column.start.is_none())
            .map(|item| item.column.span)
            .max()
            .unwrap_or(0)
            .max(column_extent.count(node, "grid-template-columns")?);
        column_extent.ensure_count(node, "grid-template-columns", minimum)?;
    } else {
        let minimum = items
            .iter()
            .filter(|item| item.row.start.is_none())
            .map(|item| item.row.span)
            .max()
            .unwrap_or(0)
            .max(row_extent.count(node, "grid-template-rows")?);
        row_extent.ensure_count(node, "grid-template-rows", minimum)?;
    }

    // Step 4: scan from the start-most implicit line; dense flow resets to the same origin.
    let mut cursor_primary = if row_flow { row_origin } else { column_origin };
    let mut cursor_secondary = if row_flow { column_origin } else { row_origin };
    let mut cursor_initialized = false;
    for item in &items {
        if placed.contains(&item.node) {
            continue;
        }
        let fixed_secondary = if row_flow {
            item.column.start
        } else {
            item.row.start
        };
        let (row_start, column_start) = if let Some(fixed_secondary) = fixed_secondary {
            if dense {
                cursor_primary = if row_flow { row_origin } else { column_origin };
            } else if cursor_initialized && fixed_secondary < cursor_secondary {
                cursor_primary = next_coordinate(node, cursor_primary)?;
            }
            loop {
                let (row_start, column_start) = if row_flow {
                    (cursor_primary, fixed_secondary)
                } else {
                    (fixed_secondary, cursor_primary)
                };
                let row_end = track_end(
                    node,
                    "grid-template-rows",
                    row_start,
                    item.row.span,
                    row_origin,
                )?;
                let column_end = track_end(
                    node,
                    "grid-template-columns",
                    column_start,
                    item.column.span,
                    column_origin,
                )?;
                if !occupancy.overlaps(node, row_start, row_end, column_start, column_end)? {
                    break (row_start, column_start);
                }
                cursor_primary = next_coordinate(node, cursor_primary)?;
            }
        } else {
            if dense {
                cursor_primary = if row_flow { row_origin } else { column_origin };
                cursor_secondary = if row_flow { column_origin } else { row_origin };
            } else if cursor_initialized {
                cursor_secondary = next_coordinate(node, cursor_secondary)?;
            }
            loop {
                let (secondary_origin, secondary_extent, secondary_span) = if row_flow {
                    (column_origin, column_extent, item.column.span)
                } else {
                    (row_origin, row_extent, item.row.span)
                };
                let secondary_limit = secondary_extent.end;
                let secondary_end = raw_track_end(node, cursor_secondary, secondary_span)?;
                if secondary_end > secondary_limit {
                    cursor_primary = next_coordinate(node, cursor_primary)?;
                    cursor_secondary = secondary_origin;
                    continue;
                }
                let (row_start, column_start) = if row_flow {
                    (cursor_primary, cursor_secondary)
                } else {
                    (cursor_secondary, cursor_primary)
                };
                let row_end = track_end(
                    node,
                    "grid-template-rows",
                    row_start,
                    item.row.span,
                    row_origin,
                )?;
                let column_end = track_end(
                    node,
                    "grid-template-columns",
                    column_start,
                    item.column.span,
                    column_origin,
                )?;
                if !occupancy.overlaps(node, row_start, row_end, column_start, column_end)? {
                    break (row_start, column_start);
                }
                cursor_secondary = next_coordinate(node, cursor_secondary)?;
            }
        };
        let row_end = track_end(
            node,
            "grid-template-rows",
            row_start,
            item.row.span,
            row_origin,
        )?;
        let column_end = track_end(
            node,
            "grid-template-columns",
            column_start,
            item.column.span,
            column_origin,
        )?;
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        row_extent.include_end(node, "grid-template-rows", row_end)?;
        column_extent.include_end(node, "grid-template-columns", column_end)?;
        pending_placements.insert(
            item.node,
            ItemLines {
                row_start,
                row_end,
                column_start,
                column_end,
            },
        );
        placed.insert(item.node);
        cursor_primary = if row_flow { row_start } else { column_start };
        cursor_secondary = if row_flow { column_start } else { row_start };
        cursor_initialized = true;
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
        auto_placed_items,
    })
}
