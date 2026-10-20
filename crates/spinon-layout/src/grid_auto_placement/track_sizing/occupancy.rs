use spinon_core::NodeId;

use crate::{LayoutError, grid::GRID_MAX_TRACKS};

use super::super::reservation::{GridReservationSite, reserve};

#[derive(Default)]
pub(super) struct Occupancy {
    row_origin: i64,
    rows: Vec<Vec<(i64, i64)>>,
}

impl Occupancy {
    pub(super) fn new(row_origin: i64) -> Self {
        Self {
            row_origin,
            rows: Vec::new(),
        }
    }

    pub(super) fn overlaps(
        &self,
        node: NodeId,
        row_start: i64,
        row_end: i64,
        column_start: i64,
        column_end: i64,
    ) -> Result<bool, LayoutError> {
        let start = self.row_index(node, row_start)?;
        let end = self.row_index(node, row_end)?;
        if end < start {
            return Err(LayoutError::GridTrackExpansionOverflow(node));
        }
        Ok(self
            .rows
            .iter()
            .skip(start)
            .take(end - start)
            .any(|intervals| {
                let index =
                    intervals.partition_point(|(_, existing_end)| *existing_end <= column_start);
                intervals
                    .get(index)
                    .is_some_and(|(existing_start, _)| *existing_start < column_end)
            }))
    }

    pub(super) fn occupy(
        &mut self,
        node: NodeId,
        row_start: i64,
        row_end: i64,
        column_start: i64,
        column_end: i64,
    ) -> Result<(), LayoutError> {
        let start = self.row_index(node, row_start)?;
        let end = self.row_index(node, row_end)?;
        if end < start || end > GRID_MAX_TRACKS {
            return Err(LayoutError::GridTrackExpansionOverflow(node));
        }
        if end > self.rows.len() {
            let additional_rows = end - self.rows.len();
            reserve(
                &mut self.rows,
                additional_rows,
                node,
                "grid-placement-occupancy",
                GridReservationSite::OccupancyRows,
            )?;
            self.rows.resize_with(end, Vec::new);
        }
        for intervals in &mut self.rows[start..end] {
            insert_occupied_interval(intervals, node, column_start, column_end)?;
        }
        Ok(())
    }

    fn row_index(&self, node: NodeId, coordinate: i64) -> Result<usize, LayoutError> {
        let offset = coordinate
            .checked_sub(self.row_origin)
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        usize::try_from(offset).map_err(|_| LayoutError::GridTrackExpansionOverflow(node))
    }
}

fn insert_occupied_interval(
    intervals: &mut Vec<(i64, i64)>,
    node: NodeId,
    start: i64,
    end: i64,
) -> Result<(), LayoutError> {
    let index = intervals.partition_point(|(_, existing_end)| *existing_end < start);
    let mut merged_start = start;
    let mut merged_end = end;
    let mut remove_end = index;
    while let Some((existing_start, existing_end)) = intervals.get(remove_end).copied() {
        if existing_start > merged_end {
            break;
        }
        merged_start = merged_start.min(existing_start);
        merged_end = merged_end.max(existing_end);
        remove_end += 1;
    }
    if remove_end == index {
        reserve(
            intervals,
            1,
            node,
            "grid-placement-occupancy",
            GridReservationSite::OccupancyIntervals,
        )?;
        intervals.insert(index, (merged_start, merged_end));
    } else {
        intervals[index] = (merged_start, merged_end);
        intervals.drain(index + 1..remove_end);
    }
    Ok(())
}
