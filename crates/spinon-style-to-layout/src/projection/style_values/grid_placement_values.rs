use spinon_core::NodeId;
use spinon_layout::{GridItemInput, GridPlacementInput};
use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridPlacement};

use crate::StyleLayoutError;

pub(super) fn project_grid_item_placement(
    node: NodeId,
    placement: &ComputedCssGridItemPlacement,
) -> Result<Option<GridItemInput>, StyleLayoutError> {
    let row_start = project_placement(node, "grid-row-start", &placement.row_start)?;
    let row_end = project_placement(node, "grid-row-end", &placement.row_end)?;
    let column_start = project_placement(node, "grid-column-start", &placement.column_start)?;
    let column_end = project_placement(node, "grid-column-end", &placement.column_end)?;
    let result = GridItemInput {
        row_start,
        row_end,
        column_start,
        column_end,
    };
    if [row_start, row_end, column_start, column_end]
        .into_iter()
        .all(|value| value == GridPlacementInput::Auto)
    {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn project_placement(
    node: NodeId,
    property: &'static str,
    value: &ComputedCssGridPlacement,
) -> Result<GridPlacementInput, StyleLayoutError> {
    match value {
        ComputedCssGridPlacement::Auto => Ok(GridPlacementInput::Auto),
        ComputedCssGridPlacement::Line(line) => Ok(GridPlacementInput::Line(*line)),
        ComputedCssGridPlacement::Span(span) => u32::try_from(*span)
            .map(GridPlacementInput::Span)
            .map_err(|_| unsupported(node, property, &span.to_string())),
        ComputedCssGridPlacement::Unsupported(serialized) => {
            Err(unsupported(node, property, serialized))
        }
    }
}

fn unsupported(node: NodeId, property: &'static str, value: &str) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    }
}
