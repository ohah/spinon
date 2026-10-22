use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::grid::GRID_MAX_TRACKS;
use crate::{
    GridAutoRows, GridContainerInput, GridItemInput, GridPlacementInput, GridTrackSize,
    LayoutError, LayoutInput, grid_placement::invalid_placement,
};

#[path = "grid_auto_placement/axis.rs"]
mod axis;
#[path = "grid_auto_placement/dense_column.rs"]
mod dense_column;
#[path = "grid_auto_placement/occupancy.rs"]
mod occupancy;
#[path = "grid_auto_placement/reservation.rs"]
mod reservation;
#[path = "grid_auto_placement/track_sizing.rs"]
mod track_sizing;

use axis::{AxisPlacement, resolve_axis};
pub(crate) use dense_column::normalize_dense_grid_container;
use occupancy::Occupancy;
use reservation::{GridReservationSite, reserve_exact};

pub(crate) struct DenseGridPlacement {
    pub(crate) item_placements: BTreeMap<NodeId, GridItemInput>,
    pub(crate) expanded_rows: Option<Vec<GridTrackSize>>,
    pub(crate) expanded_columns: Option<Vec<GridTrackSize>>,
    pub(crate) auto_placed_items: BTreeSet<NodeId>,
}

#[cfg(test)]
pub(crate) use reservation::{GridReservationSite as TestGridReservationSite, inject_failure_once};

pub(super) struct SparseGridPlacement {
    pub(super) item_placements: BTreeMap<NodeId, GridItemInput>,
    pub(super) expanded_rows: Option<Vec<GridTrackSize>>,
    pub(super) auto_placed_items: BTreeSet<NodeId>,
}

struct ItemPlacement {
    node: NodeId,
    row: AxisPlacement,
    column: AxisPlacement,
}

pub(super) fn normalize_sparse_grid_container(
    input: &LayoutInput,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<SparseGridPlacement, LayoutError> {
    let node = container_node.id;
    validate_track_count(node, "grid-template-columns", container.columns.len())?;
    validate_track_count(node, "grid-template-rows", container.rows.len())?;

    let visible_children = container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    reserve_exact(
        &mut items,
        visible_children.len(),
        node,
        "grid-placement-occupancy",
        GridReservationSite::VisibleItemList,
    )?;
    let mut auto_placed_items = BTreeSet::new();

    for child in visible_children {
        let default_placement = crate::GridItemInput::default();
        let placement = input.grid_items.get(&child).unwrap_or(&default_placement);
        let row = resolve_axis(
            child,
            "grid-row-start",
            "grid-row-end",
            &placement.row_start,
            &placement.row_end,
            container.rows.len(),
            false,
        )?;
        let column = resolve_axis(
            child,
            "grid-column-start",
            "grid-column-end",
            &placement.column_start,
            &placement.column_end,
            container.columns.len(),
            false,
        )?;
        if column.span > container.columns.len() {
            return Err(LayoutError::GridImplicitTrackRequired(node));
        }
        if !(row.is_definite() && column.is_definite()) {
            auto_placed_items.insert(child);
        }
        items.push(ItemPlacement {
            node: child,
            row,
            column,
        });
    }

    let mut output = SparseGridPlacement {
        item_placements: BTreeMap::new(),
        expanded_rows: None,
        auto_placed_items,
    };
    let mut occupancy = Occupancy::default();
    let mut max_rows = container.rows.len();

    for item in &items {
        if let (Some(row_start), Some(column_start)) = (item.row.start, item.column.start) {
            let row_end = checked_track_end(node, row_start, item.row.span)?;
            let column_end = checked_track_end(node, column_start, item.column.span)?;
            occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
            max_rows = max_rows.max(row_end);
            output.item_placements.insert(
                item.node,
                definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
            );
        }
    }

    let mut row_locked_cursor = Vec::new();
    reserve_exact(
        &mut row_locked_cursor,
        container.rows.len(),
        node,
        "grid-placement-occupancy",
        GridReservationSite::RowLockedCursor,
    )?;
    row_locked_cursor.resize(container.rows.len(), 0usize);

    for item in &items {
        let (Some(row_start), None) = (item.row.start, item.column.start) else {
            continue;
        };
        let row_end = checked_track_end(node, row_start, item.row.span)?;
        let last_column_start = container
            .columns
            .len()
            .checked_sub(item.column.span)
            .ok_or(LayoutError::GridImplicitTrackRequired(node))?;
        let first_column_start = row_locked_cursor[row_start];
        let mut placed = None;
        if first_column_start <= last_column_start {
            for column_start in first_column_start..=last_column_start {
                let column_end = checked_track_end(node, column_start, item.column.span)?;
                if !occupancy.overlaps(row_start, row_end, column_start, column_end) {
                    placed = Some((column_start, column_end));
                    break;
                }
            }
        }
        let Some((column_start, column_end)) = placed else {
            return Err(LayoutError::GridImplicitTrackRequired(node));
        };
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        row_locked_cursor[row_start] = column_end;
        output.item_placements.insert(
            item.node,
            definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
        );
    }

    let mut cursor_row = 0usize;
    let mut cursor_column = None;
    for item in &items {
        if item.row.is_definite() {
            continue;
        }
        let (row_start, column_start) = if let Some(column_start) = item.column.start {
            if cursor_column.is_some_and(|previous| column_start < previous) {
                cursor_row = cursor_row
                    .checked_add(1)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
            }
            cursor_column = Some(column_start);
            find_row_for_definite_column(node, item, &mut cursor_row, column_start, &mut occupancy)?
        } else {
            find_sparse_cell(
                node,
                item,
                &mut cursor_row,
                &mut cursor_column,
                container.columns.len(),
                &mut occupancy,
            )?
        };
        let row_end = checked_track_end(node, row_start, item.row.span)?;
        let column_end = checked_track_end(node, column_start, item.column.span)?;
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        max_rows = max_rows.max(row_end);
        output.item_placements.insert(
            item.node,
            definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
        );
    }

    if max_rows > container.rows.len() {
        let implicit_rows = max_rows - container.rows.len();
        let GridAutoRows::LengthPx(auto_row_size) = &container.auto_rows else {
            return Err(LayoutError::UnsupportedGridAutoRows {
                node,
                reason: "implicit row가 필요하지만 grid-auto-rows:auto는 content sizing이 필요합니다",
            });
        };
        if !auto_row_size.is_finite() || *auto_row_size < 0.0 {
            return Err(LayoutError::InvalidGridTrack {
                node,
                property: "grid-auto-rows",
            });
        }
        let mut expanded_rows = Vec::new();
        reserve_exact(
            &mut expanded_rows,
            max_rows,
            node,
            "grid-template-rows",
            GridReservationSite::ExpandedRows,
        )?;
        expanded_rows.extend_from_slice(&container.rows);
        expanded_rows.extend(std::iter::repeat_n(
            GridTrackSize::LengthPx(*auto_row_size),
            implicit_rows,
        ));
        output.expanded_rows = Some(expanded_rows);
    }

    Ok(output)
}

fn find_row_for_definite_column(
    container: NodeId,
    item: &ItemPlacement,
    cursor_row: &mut usize,
    column_start: usize,
    occupancy: &mut Occupancy,
) -> Result<(usize, usize), LayoutError> {
    loop {
        let row_end = checked_track_end(container, *cursor_row, item.row.span)?;
        let column_end = checked_track_end(container, column_start, item.column.span)?;
        if !occupancy.overlaps(*cursor_row, row_end, column_start, column_end) {
            return Ok((*cursor_row, column_start));
        }
        *cursor_row = cursor_row
            .checked_add(1)
            .ok_or(LayoutError::GridTrackExpansionOverflow(container))?;
    }
}

fn find_sparse_cell(
    container: NodeId,
    item: &ItemPlacement,
    cursor_row: &mut usize,
    cursor_column: &mut Option<usize>,
    columns: usize,
    occupancy: &mut Occupancy,
) -> Result<(usize, usize), LayoutError> {
    let mut column = cursor_column
        .and_then(|previous| previous.checked_add(1))
        .unwrap_or(0);
    loop {
        let column_end = column
            .checked_add(item.column.span)
            .ok_or(LayoutError::GridTrackExpansionOverflow(container))?;
        if column_end > columns {
            *cursor_row = cursor_row
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(container))?;
            column = 0;
            continue;
        }
        let row_end = checked_track_end(container, *cursor_row, item.row.span)?;
        if !occupancy.overlaps(*cursor_row, row_end, column, column_end) {
            *cursor_column = Some(column);
            return Ok((*cursor_row, column));
        }
        column = column
            .checked_add(1)
            .ok_or(LayoutError::GridTrackExpansionOverflow(container))?;
    }
}

fn definite_item_lines(
    node: NodeId,
    row_start: usize,
    row_end: usize,
    column_start: usize,
    column_end: usize,
) -> Result<GridItemInput, LayoutError> {
    let line = |track_start: usize, property: &'static str| {
        i32::try_from(
            track_start
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(node))?,
        )
        .map(GridPlacementInput::Line)
        .map_err(|_| invalid_placement(node, property, "정규화한 line이 정수 범위를 넘었습니다"))
    };
    let line_end = |track_end: usize, property: &'static str| {
        i32::try_from(
            track_end
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(node))?,
        )
        .map(GridPlacementInput::Line)
        .map_err(|_| invalid_placement(node, property, "정규화한 line이 정수 범위를 넘었습니다"))
    };
    Ok(GridItemInput {
        row_start: line(row_start, "grid-row-start")?,
        row_end: line_end(row_end, "grid-row-end")?,
        column_start: line(column_start, "grid-column-start")?,
        column_end: line_end(column_end, "grid-column-end")?,
    })
}

fn checked_track_end(node: NodeId, start: usize, span: usize) -> Result<usize, LayoutError> {
    let end = start
        .checked_add(span)
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    if end > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-rows",
            count: end,
            maximum: GRID_MAX_TRACKS,
        });
    }
    Ok(end)
}

fn validate_track_count(
    node: NodeId,
    property: &'static str,
    count: usize,
) -> Result<(), LayoutError> {
    if count > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count,
            maximum: GRID_MAX_TRACKS,
        });
    }
    Ok(())
}
