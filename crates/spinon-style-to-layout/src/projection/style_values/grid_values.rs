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
    if let Some(areas) = &grid.template_areas
        && (areas.rows != grid.rows.len() || areas.columns != grid.columns.len())
    {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "grid-template-areas",
            value: format!(
                "area grid {}x{}가 explicit track {}x{}와 다릅니다",
                areas.rows,
                areas.columns,
                grid.rows.len(),
                grid.columns.len()
            ),
        });
    }
    project_grid_layout_with_reservation(node, grid, profile, |tracks, count| {
        tracks.try_reserve_exact(count).map_err(|_| ())
    })
}

fn has_implicit_template_area_line_aliases(grid: &ComputedCssGridLayout) -> bool {
    let has_area_edge_pair = |line_names: &[Vec<String>], area: &str| {
        let mut has_start = false;
        let mut has_end = false;
        for name in line_names.iter().flatten() {
            has_start |= name.strip_suffix("-start") == Some(area);
            has_end |= name.strip_suffix("-end") == Some(area);
        }
        has_start && has_end
    };

    grid.columns_line_names
        .iter()
        .chain(&grid.rows_line_names)
        .flatten()
        .filter_map(|name| name.strip_suffix("-start"))
        .any(|area| {
            has_area_edge_pair(&grid.columns_line_names, area)
                && has_area_edge_pair(&grid.rows_line_names, area)
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
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                if grid.template_areas.is_some()
                    || has_implicit_template_area_line_aliases(grid) =>
            {
                GridPlacementMode::TemplateGrid
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
#[path = "grid_values/tests.rs"]
mod tests;
