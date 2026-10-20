use spinon_core::NodeId;

use crate::{GridItemInput, GridPlacementInput, LayoutError};

#[derive(Clone, Copy)]
pub(super) struct ItemLines {
    pub(super) row_start: i64,
    pub(super) row_end: i64,
    pub(super) column_start: i64,
    pub(super) column_end: i64,
}

pub(super) fn definite_item_lines(
    item: NodeId,
    lines: ItemLines,
    row_shift: i64,
    column_shift: i64,
) -> Result<GridItemInput, LayoutError> {
    let line = |coordinate: i64, shift: i64, property: &'static str| {
        let line = coordinate
            .checked_add(shift)
            .and_then(|coordinate| coordinate.checked_add(1))
            .and_then(|line| i32::try_from(line).ok())
            .ok_or_else(|| {
                crate::grid_placement::invalid_placement(
                    item,
                    property,
                    "origin shift 후 normalized line이 정수 범위를 넘었습니다",
                )
            })?;
        Ok::<_, LayoutError>(GridPlacementInput::Line(line))
    };
    Ok(GridItemInput {
        row_start: line(lines.row_start, row_shift, "grid-row-start")?,
        row_end: line(lines.row_end, row_shift, "grid-row-end")?,
        column_start: line(lines.column_start, column_shift, "grid-column-start")?,
        column_end: line(lines.column_end, column_shift, "grid-column-end")?,
    })
}
