use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::grid::GRID_MAX_TRACKS;
use crate::grid_auto_placement::{
    DenseGridPlacement,
    axis::{AxisPlacement, resolve_axis},
    occupancy::Occupancy,
    reservation::{GridReservationSite, reserve_exact},
};
use crate::{
    GridAutoFlow, GridContainerInput, GridPlacementInput, GridTrackSize, LayoutError, LayoutInput,
};

struct Item {
    node: NodeId,
    row: AxisPlacement,
    column: AxisPlacement,
}

pub(crate) fn normalize_dense_grid_container(
    input: &LayoutInput,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    let node = container_node.id;
    validate_count(node, "grid-template-columns", container.columns.len())?;
    validate_count(node, "grid-template-rows", container.rows.len())?;

    if container.placement_mode == crate::GridPlacementMode::TrackSizing
        && (container.columns_auto_repeat.is_some() || container.rows_auto_repeat.is_some())
    {
        return normalize_auto_repeat_grid_items(input, hidden, container_node, container);
    }

    if container.placement_mode == crate::GridPlacementMode::TrackSizing {
        return super::track_sizing::normalize_track_sizing_grid_container(
            input,
            hidden,
            container_node,
            container,
        );
    }

    let row_flow = matches!(
        container.auto_flow,
        GridAutoFlow::Row | GridAutoFlow::RowDense
    );
    let dense = matches!(
        container.auto_flow,
        GridAutoFlow::RowDense | GridAutoFlow::ColumnDense
    );
    let children = container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    reserve_exact(
        &mut items,
        children.len(),
        node,
        "grid-placement-occupancy",
        GridReservationSite::VisibleItemList,
    )?;
    let mut auto_placed_items = BTreeSet::new();
    let allow_positive_implicit = matches!(
        container.placement_mode,
        crate::GridPlacementMode::TrackSizing
            | crate::GridPlacementMode::TemplateGridImplicitTracks
    );
    for child in children {
        let placement = input.grid_items.get(&child).copied().unwrap_or_default();
        let row = resolve_axis(
            child,
            "grid-row-start",
            "grid-row-end",
            placement.row_start,
            placement.row_end,
            container.rows.len(),
            allow_positive_implicit,
        )?;
        let column = resolve_axis(
            child,
            "grid-column-start",
            "grid-column-end",
            placement.column_start,
            placement.column_end,
            container.columns.len(),
            allow_positive_implicit,
        )?;
        if container.placement_mode == crate::GridPlacementMode::TemplateGridImplicitTracks
            && (row.start.is_none() || column.start.is_none())
        {
            return Err(LayoutError::GridImplicitTrackRequired(node));
        }
        if row.start.is_none() || column.start.is_none() {
            auto_placed_items.insert(child);
        }
        items.push(Item {
            node: child,
            row,
            column,
        });
    }

    let mut row_count = container.rows.len();
    let mut column_count = container.columns.len();
    if matches!(
        container.placement_mode,
        crate::GridPlacementMode::TrackSizing
            | crate::GridPlacementMode::TemplateGridImplicitTracks
    ) {
        for item in &items {
            if let Some(row_start) = item.row.start {
                row_count = row_count.max(track_end(
                    node,
                    "grid-template-rows",
                    row_start,
                    item.row.span,
                )?);
            }
            if let Some(column_start) = item.column.start {
                column_count = column_count.max(track_end(
                    node,
                    "grid-template-columns",
                    column_start,
                    item.column.span,
                )?);
            }
        }
        validate_count(node, "grid-template-rows", row_count)?;
        validate_count(node, "grid-template-columns", column_count)?;
    }
    if row_flow {
        column_count = items
            .iter()
            .filter(|item| item.column.start.is_none())
            .map(|item| item.column.span)
            .max()
            .unwrap_or(0)
            .max(column_count);
        validate_count(node, "grid-template-columns", column_count)?;
    } else {
        row_count = items
            .iter()
            .filter(|item| item.row.start.is_none())
            .map(|item| item.row.span)
            .max()
            .unwrap_or(0)
            .max(row_count);
        validate_count(node, "grid-template-rows", row_count)?;
    }

    let mut output = DenseGridPlacement {
        item_placements: BTreeMap::new(),
        expanded_rows: None,
        expanded_columns: None,
        auto_placed_items,
    };
    let mut occupancy = Occupancy::default();
    let mut placed = BTreeSet::new();

    // CSS Grid §8.5 step 1: definite row and column positions are occupied first.
    for item in &items {
        let (Some(row_start), Some(column_start)) = (item.row.start, item.column.start) else {
            continue;
        };
        let row_end = track_end(node, "grid-template-rows", row_start, item.row.span)?;
        let column_end = track_end(
            node,
            "grid-template-columns",
            column_start,
            item.column.span,
        )?;
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        if matches!(
            container.placement_mode,
            crate::GridPlacementMode::TrackSizing
                | crate::GridPlacementMode::TemplateGridImplicitTracks
        ) {
            row_count = row_count.max(row_end);
            column_count = column_count.max(column_end);
        }
        output.item_placements.insert(
            item.node,
            super::definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
        );
        placed.insert(item.node);
    }

    // Step 2: items locked to a row, or to a column for column flow.
    let locked_count = if row_flow { row_count } else { column_count };
    let mut locked_cursor = Vec::new();
    reserve_exact(
        &mut locked_cursor,
        locked_count,
        node,
        "grid-placement-occupancy",
        GridReservationSite::RowLockedCursor,
    )?;
    locked_cursor.resize(locked_count, 0usize);
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
        let (fixed_start, fixed_span, moving_span) = if row_flow {
            (locked_line, item.row.span, item.column.span)
        } else {
            (locked_line, item.column.span, item.row.span)
        };
        let fixed_end = track_end(
            node,
            if row_flow {
                "grid-template-rows"
            } else {
                "grid-template-columns"
            },
            fixed_start,
            fixed_span,
        )?;
        let mut moving_start = if dense { 0 } else { locked_cursor[locked_line] };
        let (row_start, column_start, row_end, column_end) = loop {
            let moving_end = track_end(
                node,
                if row_flow {
                    "grid-template-columns"
                } else {
                    "grid-template-rows"
                },
                moving_start,
                moving_span,
            )?;
            if row_flow {
                if !occupancy.overlaps(locked_line, fixed_end, moving_start, moving_end) {
                    break (locked_line, moving_start, fixed_end, moving_end);
                }
            } else if !occupancy.overlaps(moving_start, moving_end, locked_line, fixed_end) {
                break (moving_start, locked_line, moving_end, fixed_end);
            }
            moving_start = moving_start
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        };
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        if row_flow {
            column_count = column_count.max(column_end);
            if !dense {
                locked_cursor[locked_line] = column_end;
            }
        } else {
            row_count = row_count.max(row_end);
            if !dense {
                locked_cursor[locked_line] = row_end;
            }
        }
        output.item_placements.insert(
            item.node,
            super::definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
        );
        placed.insert(item.node);
    }

    // Step 3: the flow-secondary axis is fixed before remaining items are scanned.
    if row_flow {
        column_count = items
            .iter()
            .filter(|item| item.column.start.is_none())
            .map(|item| item.column.span)
            .max()
            .unwrap_or(0)
            .max(column_count);
        validate_count(node, "grid-template-columns", column_count)?;
    } else {
        row_count = items
            .iter()
            .filter(|item| item.row.start.is_none())
            .map(|item| item.row.span)
            .max()
            .unwrap_or(0)
            .max(row_count);
        validate_count(node, "grid-template-rows", row_count)?;
    }

    // Step 4: place the remaining items with a persistent sparse cursor or a dense reset.
    let mut cursor_primary = 0usize;
    let mut cursor_secondary = 0usize;
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
                cursor_primary = 0;
            } else if cursor_initialized && fixed_secondary < cursor_secondary {
                cursor_primary = cursor_primary
                    .checked_add(1)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
            }
            loop {
                let (row_start, column_start) = if row_flow {
                    (cursor_primary, fixed_secondary)
                } else {
                    (fixed_secondary, cursor_primary)
                };
                let row_end = track_end(node, "grid-template-rows", row_start, item.row.span)?;
                let column_end = track_end(
                    node,
                    "grid-template-columns",
                    column_start,
                    item.column.span,
                )?;
                if !occupancy.overlaps(row_start, row_end, column_start, column_end) {
                    break (row_start, column_start);
                }
                cursor_primary = cursor_primary
                    .checked_add(1)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
            }
        } else {
            if dense {
                cursor_primary = 0;
                cursor_secondary = 0;
            } else if cursor_initialized {
                cursor_secondary = cursor_secondary
                    .checked_add(1)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
            }
            loop {
                let secondary_span = if row_flow {
                    item.column.span
                } else {
                    item.row.span
                };
                let secondary_count = if row_flow { column_count } else { row_count };
                let secondary_end = cursor_secondary
                    .checked_add(secondary_span)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
                if secondary_end > secondary_count {
                    cursor_primary = cursor_primary
                        .checked_add(1)
                        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
                    cursor_secondary = 0;
                    continue;
                }
                let (row_start, column_start) = if row_flow {
                    (cursor_primary, cursor_secondary)
                } else {
                    (cursor_secondary, cursor_primary)
                };
                let row_end = track_end(node, "grid-template-rows", row_start, item.row.span)?;
                let column_end = track_end(
                    node,
                    "grid-template-columns",
                    column_start,
                    item.column.span,
                )?;
                if !occupancy.overlaps(row_start, row_end, column_start, column_end) {
                    break (row_start, column_start);
                }
                cursor_secondary = cursor_secondary
                    .checked_add(1)
                    .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
            }
        };
        let row_end = track_end(node, "grid-template-rows", row_start, item.row.span)?;
        let column_end = track_end(
            node,
            "grid-template-columns",
            column_start,
            item.column.span,
        )?;
        occupancy.occupy(item.node, row_start, row_end, column_start, column_end)?;
        row_count = row_count.max(row_end);
        column_count = column_count.max(column_end);
        validate_count(node, "grid-template-rows", row_count)?;
        validate_count(node, "grid-template-columns", column_count)?;
        output.item_placements.insert(
            item.node,
            super::definite_item_lines(item.node, row_start, row_end, column_start, column_end)?,
        );
        placed.insert(item.node);
        cursor_primary = if row_flow { row_start } else { column_start };
        cursor_secondary = if row_flow { column_start } else { row_start };
        cursor_initialized = true;
    }

    output.expanded_rows = expand_tracks(
        node,
        "grid-template-rows",
        &container.rows,
        row_count,
        &container.auto_rows,
        true,
        container.placement_mode == crate::GridPlacementMode::TrackSizing,
    )?;
    output.expanded_columns = expand_tracks(
        node,
        "grid-template-columns",
        &container.columns,
        column_count,
        &container.auto_columns,
        false,
        container.placement_mode == crate::GridPlacementMode::TrackSizing,
    )?;
    Ok(output)
}

fn normalize_auto_repeat_grid_items(
    input: &LayoutInput,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    let node = container_node.id;
    let repeat_columns = container.columns_auto_repeat.is_some();
    let mut output = DenseGridPlacement {
        item_placements: BTreeMap::new(),
        expanded_rows: None,
        expanded_columns: None,
        auto_placed_items: BTreeSet::new(),
    };

    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let placement = input.grid_items.get(&child).copied().unwrap_or_default();
        let (
            repeat_start,
            repeat_end,
            fixed_start,
            fixed_end,
            fixed_count,
            repeat_start_property,
            repeat_end_property,
        ) = if repeat_columns {
            (
                placement.column_start,
                placement.column_end,
                placement.row_start,
                placement.row_end,
                container.rows.len(),
                "grid-column-start",
                "grid-column-end",
            )
        } else {
            (
                placement.row_start,
                placement.row_end,
                placement.column_start,
                placement.column_end,
                container.columns.len(),
                "grid-row-start",
                "grid-row-end",
            )
        };
        validate_auto_repeat_placement(
            node,
            repeat_start_property,
            repeat_end_property,
            repeat_start,
            repeat_end,
        )?;
        let fixed_axis_is_definite =
            if container.columns_auto_repeat.is_some() && container.rows_auto_repeat.is_some() {
                // Taffy must resolve both axes against their expanded auto-repeat counts. Here we
                // only classify automatic placement; resolving coordinates against the authored
                // pattern length rejects valid negative and positive lines on the secondary axis.
                has_definite_grid_axis(fixed_start, fixed_end)
            } else {
                resolve_axis(
                    child,
                    if repeat_columns {
                        "grid-row-start"
                    } else {
                        "grid-column-start"
                    },
                    if repeat_columns {
                        "grid-row-end"
                    } else {
                        "grid-column-end"
                    },
                    fixed_start,
                    fixed_end,
                    fixed_count,
                    false,
                )?
                .start
                .is_some()
            };

        if !fixed_axis_is_definite || !has_definite_grid_axis(repeat_start, repeat_end) {
            output.auto_placed_items.insert(child);
        }
        output.item_placements.insert(child, placement);
    }
    Ok(output)
}

fn validate_auto_repeat_placement(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: GridPlacementInput,
    end: GridPlacementInput,
) -> Result<(), LayoutError> {
    if matches!(
        (start, end),
        (GridPlacementInput::Span(_), GridPlacementInput::Span(_))
    ) {
        return Err(LayoutError::InvalidGridPlacement {
            node,
            property: start_property,
            reason: "auto-repeat 축에서 양 끝점을 모두 span으로 지정할 수 없습니다",
        });
    }
    for (placement, property) in [(start, start_property), (end, end_property)] {
        match placement {
            GridPlacementInput::Auto => {}
            GridPlacementInput::Line(line) => {
                if line == 0 || i16::try_from(line).is_err() {
                    return Err(LayoutError::InvalidGridPlacement {
                        node,
                        property,
                        reason: "auto-repeat line은 0이 아니고 Taffy의 정수 범위 안이어야 합니다",
                    });
                }
            }
            GridPlacementInput::Span(span) => {
                if span == 0 || u16::try_from(span).is_err() {
                    return Err(LayoutError::InvalidGridPlacement {
                        node,
                        property,
                        reason: "auto-repeat span은 양수이고 Taffy의 정수 범위 안이어야 합니다",
                    });
                }
            }
        }
    }
    Ok(())
}

fn has_definite_grid_axis(start: GridPlacementInput, end: GridPlacementInput) -> bool {
    matches!(start, GridPlacementInput::Line(_)) || matches!(end, GridPlacementInput::Line(_))
}

fn expand_tracks(
    node: NodeId,
    property: &'static str,
    explicit: &[GridTrackSize],
    count: usize,
    auto_size: &crate::GridAutoTrackSize,
    rows: bool,
    allow_track_sizing: bool,
) -> Result<Option<Vec<GridTrackSize>>, LayoutError> {
    validate_count(node, property, count)?;
    if count == explicit.len() {
        return Ok(None);
    }
    let pattern = match auto_size {
        crate::GridAutoTrackSize::Auto => {
            if allow_track_sizing {
                TrackPattern::Single(GridTrackSize::Auto)
            } else {
                return if rows {
                    Err(LayoutError::UnsupportedGridAutoRows {
                        node,
                        reason: "implicit row가 필요하지만 grid-auto-rows:auto의 content sizing은 미구현입니다",
                    })
                } else {
                    Err(LayoutError::UnsupportedGridAutoColumns {
                        node,
                        reason: "implicit column이 필요하지만 grid-auto-columns:auto의 content sizing은 미구현입니다",
                    })
                };
            }
        }
        crate::GridAutoTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0 => {
            TrackPattern::Single(GridTrackSize::LengthPx(*value))
        }
        crate::GridAutoTrackSize::LengthPx(_) => {
            return Err(LayoutError::InvalidGridTrack { node, property });
        }
        crate::GridAutoTrackSize::TrackList(tracks) if allow_track_sizing && !tracks.is_empty() => {
            TrackPattern::List(tracks)
        }
        crate::GridAutoTrackSize::TrackList(_) => {
            return if rows {
                Err(LayoutError::UnsupportedGridAutoRows {
                    node,
                    reason: "grid-auto-rows track-size 목록은 C11.4 track-sizing profile에서만 지원됩니다",
                })
            } else {
                Err(LayoutError::UnsupportedGridAutoColumns {
                    node,
                    reason: "grid-auto-columns track-size 목록은 C11.4 track-sizing profile에서만 지원됩니다",
                })
            };
        }
    };
    let implicit_count = count
        .checked_sub(explicit.len())
        .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
    let mut tracks = Vec::new();
    reserve_exact(
        &mut tracks,
        count,
        node,
        property,
        if rows {
            GridReservationSite::ExpandedRows
        } else {
            GridReservationSite::ExpandedColumns
        },
    )?;
    tracks.extend_from_slice(explicit);
    for index in 0..implicit_count {
        let track = match pattern {
            TrackPattern::Single(track) => track,
            TrackPattern::List(pattern) => pattern[index % pattern.len()],
        };
        tracks.push(track);
    }
    Ok(Some(tracks))
}

#[derive(Clone, Copy)]
enum TrackPattern<'a> {
    Single(GridTrackSize),
    List(&'a [GridTrackSize]),
}

fn track_end(
    node: NodeId,
    property: &'static str,
    start: usize,
    span: usize,
) -> Result<usize, LayoutError> {
    let end = start
        .checked_add(span)
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    validate_count(node, property, end)?;
    Ok(end)
}

fn validate_count(node: NodeId, property: &'static str, count: usize) -> Result<(), LayoutError> {
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
