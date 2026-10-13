use spinon_core::NodeId;

use crate::LayoutError;
use crate::grid_auto_placement::reservation::{GridReservationSite, reserve};

#[derive(Default)]
pub(super) struct Occupancy {
    rows: Vec<Vec<(usize, usize)>>,
}

impl Occupancy {
    pub(super) fn overlaps(
        &self,
        row_start: usize,
        row_end: usize,
        column_start: usize,
        column_end: usize,
    ) -> bool {
        self.rows
            .iter()
            .take(row_end)
            .skip(row_start)
            .any(|intervals| {
                let index = intervals.partition_point(|(_, end)| *end <= column_start);
                intervals
                    .get(index)
                    .is_some_and(|(start, _)| *start < column_end)
            })
    }

    pub(super) fn occupy(
        &mut self,
        node: NodeId,
        row_start: usize,
        row_end: usize,
        column_start: usize,
        column_end: usize,
    ) -> Result<(), LayoutError> {
        if row_end > self.rows.len() {
            let additional_rows = row_end - self.rows.len();
            reserve(
                &mut self.rows,
                additional_rows,
                node,
                "grid-placement-occupancy",
                GridReservationSite::OccupancyRows,
            )?;
            self.rows.resize_with(row_end, Vec::new);
        }
        for intervals in &mut self.rows[row_start..row_end] {
            insert_occupied_interval(intervals, node, column_start, column_end)?;
        }
        Ok(())
    }
}

fn insert_occupied_interval(
    intervals: &mut Vec<(usize, usize)>,
    node: NodeId,
    start: usize,
    end: usize,
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
