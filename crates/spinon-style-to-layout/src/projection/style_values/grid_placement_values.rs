use spinon_core::NodeId;
use spinon_layout::{GridContainerInput, GridItemInput};
use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridTemplateAreas};

use crate::StyleLayoutError;

#[path = "grid_placement_values/named_line.rs"]
mod named_line;

pub(super) fn project_grid_item_placement(
    node: NodeId,
    placement: &ComputedCssGridItemPlacement,
    container: &GridContainerInput,
    template_areas: Option<&ComputedCssGridTemplateAreas>,
    allow_implicit_named_lines: bool,
) -> Result<(Option<GridItemInput>, bool), StyleLayoutError> {
    let (row_start, row_end, row_used_implicit_named_line) =
        named_line::project_axis_with_areas_and_implicit_named_lines(
            node,
            "grid-row-start",
            "grid-row-end",
            &placement.row_start,
            &placement.row_end,
            named_line::LineMetadata {
                names: &container.rows_line_names,
                opposite_names: &container.columns_line_names,
                areas: template_areas,
                axis: named_line::GridAxis::Rows,
                track_count: container.rows.len(),
            },
            allow_implicit_named_lines,
        )?;
    let (column_start, column_end, column_used_implicit_named_line) =
        named_line::project_axis_with_areas_and_implicit_named_lines(
            node,
            "grid-column-start",
            "grid-column-end",
            &placement.column_start,
            &placement.column_end,
            named_line::LineMetadata {
                names: &container.columns_line_names,
                opposite_names: &container.rows_line_names,
                areas: template_areas,
                axis: named_line::GridAxis::Columns,
                track_count: container.columns.len(),
            },
            allow_implicit_named_lines,
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
        Ok((
            None,
            row_used_implicit_named_line || column_used_implicit_named_line,
        ))
    } else {
        Ok((
            Some(result),
            row_used_implicit_named_line || column_used_implicit_named_line,
        ))
    }
}
