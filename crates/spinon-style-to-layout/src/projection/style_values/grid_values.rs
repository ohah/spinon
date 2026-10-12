use spinon_core::NodeId;
use spinon_layout::{GridAutoFlow, GridContainerInput, GridTrackSize};
use spinon_style::{ComputedCssGridAutoFlow, ComputedCssGridLayout, ComputedCssGridTrackSize};

pub(super) fn project_grid_layout(
    node: NodeId,
    grid: &ComputedCssGridLayout,
) -> Result<GridContainerInput, spinon_layout::LayoutError> {
    project_grid_layout_with_reservation(node, grid, |tracks, count| {
        tracks.try_reserve_exact(count).map_err(|_| ())
    })
}

fn project_grid_layout_with_reservation(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridContainerInput, spinon_layout::LayoutError> {
    let columns = project_tracks(&grid.columns, node, "grid-template-columns", &mut reserve)?;
    let rows = project_tracks(&grid.rows, node, "grid-template-rows", &mut reserve)?;
    Ok(GridContainerInput {
        columns,
        rows,
        auto_flow: match grid.auto_flow {
            ComputedCssGridAutoFlow::Row => GridAutoFlow::Row,
            ComputedCssGridAutoFlow::Column => GridAutoFlow::Column,
            ComputedCssGridAutoFlow::RowDense => GridAutoFlow::RowDense,
            ComputedCssGridAutoFlow::ColumnDense => GridAutoFlow::ColumnDense,
        },
    })
}

fn project_tracks(
    source: &[ComputedCssGridTrackSize],
    node: NodeId,
    property: &'static str,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<Vec<GridTrackSize>, spinon_layout::LayoutError> {
    let track = |value: &ComputedCssGridTrackSize| match value {
        ComputedCssGridTrackSize::LengthPx(value) => GridTrackSize::LengthPx(*value),
        ComputedCssGridTrackSize::Percentage(value) => GridTrackSize::Percentage(*value),
        ComputedCssGridTrackSize::Fraction(value) => GridTrackSize::Fraction(*value),
    };
    let mut tracks = Vec::new();
    reserve(&mut tracks, source.len())
        .map_err(|_| spinon_layout::LayoutError::GridAllocationFailure { node, property })?;
    tracks.extend(source.iter().map(track));
    Ok(tracks)
}

#[cfg(test)]
mod tests {
    use super::project_grid_layout_with_reservation;
    use spinon_core::NodeId;
    use spinon_layout::LayoutError;
    use spinon_style::{ComputedCssGridAutoFlow, ComputedCssGridLayout, ComputedCssGridTrackSize};

    #[test]
    fn projection_reservation_failure_keeps_node_and_axis_context() {
        let node = NodeId::new(11_101).unwrap();
        let grid = ComputedCssGridLayout {
            columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
            rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            auto_flow: ComputedCssGridAutoFlow::Row,
        };
        let mut calls = 0;
        let error = project_grid_layout_with_reservation(node, &grid, |_, _| {
            calls += 1;
            if calls == 2 { Err(()) } else { Ok(()) }
        })
        .unwrap_err();

        assert_eq!(
            error,
            LayoutError::GridAllocationFailure {
                node,
                property: "grid-template-rows"
            }
        );
    }
}
