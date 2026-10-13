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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GridLineNameReservationSite {
    Groups,
    Names,
    NameText,
}

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
    project_grid_layout_with_reservations(node, grid, profile, &mut reserve, |_| Ok(()))
}

fn project_grid_layout_with_reservations(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
    mut reserve_line_metadata: impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    let columns = project_tracks(&grid.columns, node, "grid-template-columns", &mut reserve)?;
    let rows = project_tracks(&grid.rows, node, "grid-template-rows", &mut reserve)?;
    let (columns_line_names, rows_line_names) =
        if profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 {
            validate_line_name_count(
                node,
                "grid-template-columns",
                &grid.columns,
                &grid.columns_line_names,
            )?;
            validate_line_name_count(
                node,
                "grid-template-rows",
                &grid.rows,
                &grid.rows_line_names,
            )?;
            (
                project_line_names(
                    node,
                    "grid-template-columns",
                    &grid.columns_line_names,
                    &mut reserve_line_metadata,
                )?,
                project_line_names(
                    node,
                    "grid-template-rows",
                    &grid.rows_line_names,
                    &mut reserve_line_metadata,
                )?,
            )
        } else {
            (Vec::new(), Vec::new())
        };
    let sparse_row = profile == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1;
    let dense_auto_placement = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    );
    Ok(GridContainerInput {
        columns,
        rows,
        columns_line_names,
        rows_line_names,
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
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
                GridPlacementMode::DenseAutoPlacement
            }
            _ => GridPlacementMode::Existing,
        },
    })
}

fn validate_line_name_count(
    node: NodeId,
    property: &'static str,
    tracks: &[ComputedCssGridTrackSize],
    names: &[Vec<String>],
) -> Result<(), StyleLayoutError> {
    if tracks.len().checked_add(1) == Some(names.len()) {
        Ok(())
    } else {
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: format!(
                "line-name group 수 {}가 track 수 {}의 N+1 규칙과 다릅니다",
                names.len(),
                tracks.len()
            ),
        })
    }
}

fn project_line_names(
    node: NodeId,
    property: &'static str,
    source: &[Vec<String>],
    reserve: &mut impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<Vec<Vec<String>>, StyleLayoutError> {
    let mut output = Vec::new();
    reserve(GridLineNameReservationSite::Groups).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    output.try_reserve_exact(source.len()).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    for group in source {
        let mut projected = Vec::new();
        reserve(GridLineNameReservationSite::Names).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property,
            })
        })?;
        projected.try_reserve_exact(group.len()).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property,
            })
        })?;
        for name in group {
            let mut owned = String::new();
            reserve(GridLineNameReservationSite::NameText).map_err(|_| {
                StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                    node,
                    property,
                })
            })?;
            owned.try_reserve(name.len()).map_err(|_| {
                StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                    node,
                    property,
                })
            })?;
            owned.push_str(name);
            projected.push(owned);
        }
        output.push(projected);
    }
    Ok(output)
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
    use super::{
        GridLineNameReservationSite, project_grid_layout, project_grid_layout_with_reservation,
        project_grid_layout_with_reservations,
    };
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
            columns_line_names: Vec::new(),
            rows_line_names: Vec::new(),
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

    #[test]
    fn template_profile_projects_n_plus_one_line_names_and_dense_grid_contract() {
        let node = NodeId::new(11_103).unwrap();
        let grid = ComputedCssGridLayout {
            columns: vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(30.0),
            ],
            rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            columns_line_names: vec![
                vec!["start".to_owned()],
                vec!["seam-a".to_owned(), "seam-b".to_owned()],
                vec!["end".to_owned()],
            ],
            rows_line_names: vec![Vec::new(), vec!["row-end".to_owned()]],
            auto_flow: ComputedCssGridAutoFlow::ColumnDense,
            auto_rows: ComputedCssGridAutoRows::LengthPx(12.0),
            auto_columns: spinon_style::ComputedCssGridAutoColumns::LengthPx(16.0),
        };

        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
        )
        .unwrap();

        assert_eq!(projected.columns_line_names, grid.columns_line_names);
        assert_eq!(projected.rows_line_names, grid.rows_line_names);
        assert_eq!(
            projected.auto_flow,
            spinon_layout::GridAutoFlow::ColumnDense
        );
        assert_eq!(
            projected.auto_rows,
            spinon_layout::GridAutoRows::LengthPx(12.0)
        );
        assert_eq!(
            projected.auto_columns,
            spinon_layout::GridAutoColumns::LengthPx(16.0)
        );
        assert_eq!(
            projected.placement_mode,
            spinon_layout::GridPlacementMode::DenseAutoPlacement
        );
    }

    #[test]
    fn template_profile_line_name_allocation_failures_keep_context_and_recover() {
        let node = NodeId::new(11_104).unwrap();
        let grid = ComputedCssGridLayout {
            columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
            rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            columns_line_names: vec![vec!["start".to_owned()], vec!["end".to_owned()]],
            rows_line_names: vec![Vec::new(), Vec::new()],
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
        };

        for site in [
            GridLineNameReservationSite::Groups,
            GridLineNameReservationSite::Names,
            GridLineNameReservationSite::NameText,
        ] {
            let error = project_grid_layout_with_reservations(
                node,
                &grid,
                ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
                |_, _| Ok(()),
                |requested| if requested == site { Err(()) } else { Ok(()) },
            )
            .unwrap_err();
            assert!(
                matches!(
                    error,
                    crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                        node: error_node,
                        property: "grid-template-columns",
                    }) if error_node == node
                ),
                "예약 실패 지점: {site:?}, 실제 오류: {error:?}"
            );
            assert!(
                project_grid_layout(
                    node,
                    &grid,
                    ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
                )
                .is_ok()
            );
        }
    }

    #[test]
    fn template_profile_rejects_line_name_groups_that_do_not_match_track_count() {
        let node = NodeId::new(11_104).unwrap();
        let grid = ComputedCssGridLayout {
            columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
            rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            columns_line_names: vec![vec!["only-one-boundary".to_owned()]],
            rows_line_names: vec![Vec::new(), Vec::new()],
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
        };

        assert!(matches!(
            project_grid_layout(
                node,
                &grid,
                ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-template-columns",
                ..
            }) if error_node == node
        ));
    }
}
