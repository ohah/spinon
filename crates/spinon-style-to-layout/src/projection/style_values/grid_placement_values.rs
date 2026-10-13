use spinon_core::NodeId;
use spinon_layout::{GridContainerInput, GridItemInput};
use spinon_style::ComputedCssGridItemPlacement;

use crate::StyleLayoutError;

#[path = "grid_placement_values/named_line.rs"]
mod named_line;

pub(super) fn project_grid_item_placement(
    node: NodeId,
    placement: &ComputedCssGridItemPlacement,
    container: &GridContainerInput,
) -> Result<Option<GridItemInput>, StyleLayoutError> {
    let (row_start, row_end) = named_line::project_axis(
        node,
        "grid-row-start",
        "grid-row-end",
        &placement.row_start,
        &placement.row_end,
        &container.rows_line_names,
        container.rows.len(),
    )?;
    let (column_start, column_end) = named_line::project_axis(
        node,
        "grid-column-start",
        "grid-column-end",
        &placement.column_start,
        &placement.column_end,
        &container.columns_line_names,
        container.columns.len(),
    )?;
    let result = GridItemInput {
        row_start,
        row_end,
        column_start,
        column_end,
    };
    if [row_start, row_end, column_start, column_end]
        .into_iter()
        .all(|value| value == spinon_layout::GridPlacementInput::Auto)
    {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}
