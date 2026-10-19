use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRepeat, GridAutoRows, GridContainerInput,
    GridPlacementMode, GridTrackLengthPercentage, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutCssMathProperty,
};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackSize, ComputedStyleProfile,
};

use super::super::typed_math::CssMathProjector;
use crate::StyleLayoutError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GridLineNameReservationSite {
    Groups,
    Names,
    NameText,
}

#[cfg(test)]
pub(super) fn project_grid_layout(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
) -> Result<GridContainerInput, StyleLayoutError> {
    let mut math = CssMathProjector::default();
    project_grid_layout_with_math(node, grid, profile, &mut math)
}

pub(super) fn project_grid_layout_with_math(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
) -> Result<GridContainerInput, StyleLayoutError> {
    if grid.columns_auto_repeat.is_some() || grid.rows_auto_repeat.is_some() {
        let property = if grid.columns_auto_repeat.is_some() {
            "grid-template-columns"
        } else {
            "grid-template-rows"
        };
        if profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
            || (grid.columns_auto_repeat.is_some() && grid.rows_auto_repeat.is_some())
            || grid.template_areas.is_some()
        {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "auto-repeat은 단일 축의 제한 track-sizing profile에서만 지원됩니다"
                    .to_owned(),
            });
        }
    }
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
    project_grid_layout_with_reservations(
        node,
        grid,
        profile,
        math,
        |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
        |_| Ok(()),
    )
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

#[cfg(test)]
fn project_grid_layout_with_reservation(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    let mut math = CssMathProjector::default();
    project_grid_layout_with_reservations(node, grid, profile, &mut math, &mut reserve, |_| Ok(()))
}

fn project_grid_layout_with_reservations(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
    mut reserve_line_metadata: impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    let columns = project_tracks(
        &grid.columns,
        node,
        "grid-template-columns",
        LayoutCssMathProperty::GridTemplateColumns,
        math,
        &mut reserve,
    )?;
    let rows = project_tracks(
        &grid.rows,
        node,
        "grid-template-rows",
        LayoutCssMathProperty::GridTemplateRows,
        math,
        &mut reserve,
    )?;
    let (columns_line_names, rows_line_names) = if matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    ) {
        validate_axis_line_names(
            node,
            "grid-template-columns",
            &grid.columns,
            &grid.columns_line_names,
            grid.columns_auto_repeat.is_some(),
        )?;
        validate_axis_line_names(
            node,
            "grid-template-rows",
            &grid.rows,
            &grid.rows_line_names,
            grid.rows_auto_repeat.is_some(),
        )?;
        (
            if grid.columns_auto_repeat.is_some() {
                Vec::new()
            } else {
                project_line_names(
                    node,
                    "grid-template-columns",
                    &grid.columns_line_names,
                    &mut reserve_line_metadata,
                )?
            },
            if grid.rows_auto_repeat.is_some() {
                Vec::new()
            } else {
                project_line_names(
                    node,
                    "grid-template-rows",
                    &grid.rows_line_names,
                    &mut reserve_line_metadata,
                )?
            },
        )
    } else {
        (Vec::new(), Vec::new())
    };
    let sparse_row = profile == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1;
    let dense_auto_placement = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    Ok(GridContainerInput {
        columns,
        rows,
        columns_auto_repeat: grid.columns_auto_repeat.map(project_auto_repeat),
        rows_auto_repeat: grid.rows_auto_repeat.map(project_auto_repeat),
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
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
                GridPlacementMode::TrackSizing
            }
            _ => GridPlacementMode::Existing,
        },
    })
}

fn project_auto_repeat_mode(
    mode: ComputedCssGridAutoRepeatMode,
) -> spinon_layout::GridAutoRepeatMode {
    match mode {
        ComputedCssGridAutoRepeatMode::AutoFill => spinon_layout::GridAutoRepeatMode::AutoFill,
        ComputedCssGridAutoRepeatMode::AutoFit => spinon_layout::GridAutoRepeatMode::AutoFit,
    }
}

fn project_auto_repeat(auto_repeat: ComputedCssGridAutoRepeat) -> GridAutoRepeat {
    GridAutoRepeat {
        mode: project_auto_repeat_mode(auto_repeat.mode),
        start: auto_repeat.start,
        end: auto_repeat.end,
    }
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

fn validate_axis_line_names(
    node: NodeId,
    property: &'static str,
    tracks: &[ComputedCssGridTrackSize],
    names: &[Vec<String>],
    auto_repeat: bool,
) -> Result<(), StyleLayoutError> {
    if auto_repeat {
        if names.is_empty() {
            Ok(())
        } else {
            Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "auto-repeat의 line name은 현재 지원 범위 밖입니다".to_owned(),
            })
        }
    } else {
        validate_line_name_count(node, property, tracks, names)
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
    math_property: LayoutCssMathProperty,
    math: &mut CssMathProjector,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<Vec<GridTrackSize>, StyleLayoutError> {
    let mut tracks = Vec::new();
    reserve(&mut tracks, source.len()).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    for value in source {
        tracks.push(project_track(value, node, property, math_property, math)?);
    }
    Ok(tracks)
}

fn project_track(
    value: &ComputedCssGridTrackSize,
    node: NodeId,
    property: &'static str,
    math_property: LayoutCssMathProperty,
    math: &mut CssMathProjector,
) -> Result<GridTrackSize, StyleLayoutError> {
    let length_percentage = |value: &spinon_style::ComputedCssGridTrackLengthPercentage,
                             math: &mut CssMathProjector|
     -> Result<GridTrackLengthPercentage, StyleLayoutError> {
        Ok(match value {
            spinon_style::ComputedCssGridTrackLengthPercentage::LengthPx(value) => {
                GridTrackLengthPercentage::LengthPx(*value)
            }
            spinon_style::ComputedCssGridTrackLengthPercentage::Percentage(value) => {
                GridTrackLengthPercentage::Percentage(*value)
            }
            spinon_style::ComputedCssGridTrackLengthPercentage::Calc(expression) => {
                GridTrackLengthPercentage::Calc(math.project(node, math_property, expression)?)
            }
        })
    };
    let breadth = |value: &spinon_style::ComputedCssGridTrackBreadth,
                   math: &mut CssMathProjector|
     -> Result<GridTrackMinSizingFunction, StyleLayoutError> {
        match value {
            spinon_style::ComputedCssGridTrackBreadth::LengthPx(value) => {
                Ok(GridTrackMinSizingFunction::LengthPx(*value))
            }
            spinon_style::ComputedCssGridTrackBreadth::Percentage(value) => {
                Ok(GridTrackMinSizingFunction::Percentage(*value))
            }
            spinon_style::ComputedCssGridTrackBreadth::Calc(expression) => Ok(
                GridTrackMinSizingFunction::Calc(math.project(node, math_property, expression)?),
            ),
            spinon_style::ComputedCssGridTrackBreadth::Fraction(_) => {
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property,
                    value: "minmax() 최소 함수에 flex factor가 들어왔습니다".to_owned(),
                })
            }
            spinon_style::ComputedCssGridTrackBreadth::Auto => Ok(GridTrackMinSizingFunction::Auto),
            spinon_style::ComputedCssGridTrackBreadth::MinContent => {
                Ok(GridTrackMinSizingFunction::MinContent)
            }
            spinon_style::ComputedCssGridTrackBreadth::MaxContent => {
                Ok(GridTrackMinSizingFunction::MaxContent)
            }
        }
    };
    let max_breadth = |value: &spinon_style::ComputedCssGridTrackBreadth,
                       math: &mut CssMathProjector|
     -> Result<GridTrackMaxSizingFunction, StyleLayoutError> {
        Ok(match value {
            spinon_style::ComputedCssGridTrackBreadth::LengthPx(value) => {
                GridTrackMaxSizingFunction::LengthPx(*value)
            }
            spinon_style::ComputedCssGridTrackBreadth::Percentage(value) => {
                GridTrackMaxSizingFunction::Percentage(*value)
            }
            spinon_style::ComputedCssGridTrackBreadth::Calc(expression) => {
                GridTrackMaxSizingFunction::Calc(math.project(node, math_property, expression)?)
            }
            spinon_style::ComputedCssGridTrackBreadth::Fraction(value) => {
                GridTrackMaxSizingFunction::Fraction(*value)
            }
            spinon_style::ComputedCssGridTrackBreadth::Auto => GridTrackMaxSizingFunction::Auto,
            spinon_style::ComputedCssGridTrackBreadth::MinContent => {
                GridTrackMaxSizingFunction::MinContent
            }
            spinon_style::ComputedCssGridTrackBreadth::MaxContent => {
                GridTrackMaxSizingFunction::MaxContent
            }
        })
    };
    Ok(match value {
        ComputedCssGridTrackSize::LengthPx(value) => GridTrackSize::LengthPx(*value),
        ComputedCssGridTrackSize::Percentage(value) => GridTrackSize::Percentage(*value),
        ComputedCssGridTrackSize::Fraction(value) => GridTrackSize::Fraction(*value),
        ComputedCssGridTrackSize::Calc(expression) => {
            GridTrackSize::Calc(math.project(node, math_property, expression)?)
        }
        ComputedCssGridTrackSize::Auto => GridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent => GridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent => GridTrackSize::MaxContent,
        ComputedCssGridTrackSize::MinMax { min, max } => GridTrackSize::MinMax {
            min: breadth(min, math)?,
            max: max_breadth(max, math)?,
        },
        ComputedCssGridTrackSize::FitContent(value) => {
            GridTrackSize::FitContent(length_percentage(value, math)?)
        }
    })
}

#[cfg(test)]
#[path = "grid_values/tests.rs"]
mod tests;
