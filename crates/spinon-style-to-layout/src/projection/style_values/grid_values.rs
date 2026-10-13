use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackSize,
};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRows,
    ComputedCssGridLayout, ComputedCssGridTrackSize, ComputedStyleProfile,
};

use crate::StyleLayoutError;

pub(super) fn project_grid_layout(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
) -> Result<GridContainerInput, StyleLayoutError> {
    project_grid_layout_with_reservation(node, grid, profile, |tracks, count| {
        tracks.try_reserve_exact(count).map_err(|_| ())
    })
}

fn project_grid_layout_with_reservation(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    let columns = project_tracks(&grid.columns, node, "grid-template-columns", &mut reserve)?;
    let rows = project_tracks(&grid.rows, node, "grid-template-rows", &mut reserve)?;
    let sparse_row = profile == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1;
    let dense_auto_placement =
        profile == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1;
    Ok(GridContainerInput {
        columns,
        rows,
        auto_flow: match grid.auto_flow {
            ComputedCssGridAutoFlow::Row => GridAutoFlow::Row,
            ComputedCssGridAutoFlow::Column => GridAutoFlow::Column,
            ComputedCssGridAutoFlow::RowDense => GridAutoFlow::RowDense,
            ComputedCssGridAutoFlow::ColumnDense => GridAutoFlow::ColumnDense,
        },
        auto_rows: if sparse_row || dense_auto_placement {
            project_auto_rows(node, &grid.auto_rows)?
        } else {
            GridAutoRows::Auto
        },
        auto_columns: if dense_auto_placement {
            project_auto_columns(node, &grid.auto_columns)?
        } else {
            GridAutoColumns::Auto
        },
        placement_mode: match profile {
            ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1 => {
                GridPlacementMode::ExplicitOnly
            }
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1 => {
                GridPlacementMode::SparseRow
            }
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1 => {
                GridPlacementMode::DenseAutoPlacement
            }
            _ => GridPlacementMode::Existing,
        },
    })
}

fn project_auto_columns(
    node: NodeId,
    auto_columns: &ComputedCssGridAutoColumns,
) -> Result<GridAutoColumns, StyleLayoutError> {
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => Ok(GridAutoColumns::Auto),
        ComputedCssGridAutoColumns::LengthPx(value) => Ok(GridAutoColumns::LengthPx(*value)),
        ComputedCssGridAutoColumns::Unsupported(value) => {
            Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property: "grid-auto-columns",
                value: value.clone(),
            })
        }
    }
}

fn project_auto_rows(
    node: NodeId,
    auto_rows: &ComputedCssGridAutoRows,
) -> Result<GridAutoRows, StyleLayoutError> {
    match auto_rows {
        ComputedCssGridAutoRows::Auto => Ok(GridAutoRows::Auto),
        ComputedCssGridAutoRows::LengthPx(value) => Ok(GridAutoRows::LengthPx(*value)),
        ComputedCssGridAutoRows::Unsupported(value) => {
            Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property: "grid-auto-rows",
                value: value.clone(),
            })
        }
    }
}

fn project_tracks(
    source: &[ComputedCssGridTrackSize],
    node: NodeId,
    property: &'static str,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<Vec<GridTrackSize>, StyleLayoutError> {
    let track = |value: &ComputedCssGridTrackSize| match value {
        ComputedCssGridTrackSize::LengthPx(value) => GridTrackSize::LengthPx(*value),
        ComputedCssGridTrackSize::Percentage(value) => GridTrackSize::Percentage(*value),
        ComputedCssGridTrackSize::Fraction(value) => GridTrackSize::Fraction(*value),
    };
    let mut tracks = Vec::new();
    reserve(&mut tracks, source.len()).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    tracks.extend(source.iter().map(track));
    Ok(tracks)
}

#[cfg(test)]
mod tests {
    use super::project_grid_layout_with_reservation;
    use spinon_core::NodeId;
    use spinon_layout::LayoutError;
    use spinon_style::{
        ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridLayout,
        ComputedCssGridTrackSize, ComputedStyleProfile,
    };

    #[test]
    fn projection_reservation_failure_keeps_node_and_axis_context() {
        let node = NodeId::new(11_101).unwrap();
        let grid = ComputedCssGridLayout {
            columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
            rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
        };
        let mut calls = 0;
        let error = project_grid_layout_with_reservation(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
            |_, _| {
                calls += 1;
                if calls == 2 { Err(()) } else { Ok(()) }
            },
        )
        .unwrap_err();

        assert!(matches!(
            error,
            crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                node: error_node,
                property: "grid-template-rows",
            }) if error_node == node
        ));
    }
}
