use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRepeat, GridAutoRows, GridContainerInput,
    GridPlacementMode, GridTrackLengthPercentage, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutAlignItems, LayoutCssMathProperty,
};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedStyleProfile,
};

use super::super::typed_math::CssMathProjector;
use crate::StyleLayoutError;

const MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES: usize = 1024 * 1024;
const MAX_COMBINED_GRID_LINE_AREA_METADATA_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GridLineNameReservationSite {
    Groups,
    Names,
    NameText,
    Areas,
    AreaName,
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
    let has_auto_repeat = grid.columns_auto_repeat.is_some() || grid.rows_auto_repeat.is_some();
    if has_auto_repeat {
        let property = if grid.columns_auto_repeat.is_some() {
            "grid-template-columns"
        } else {
            "grid-template-rows"
        };
        if profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "auto-repeat은 각 track list에서 하나씩만 제한 track-sizing profile로 지원됩니다"
                    .to_owned(),
            });
        }
    }
    if let Some(areas) = &grid.template_areas {
        let row_dimension_matches =
            grid.rows_auto_repeat.is_some() || areas.rows == grid.rows.len();
        let column_dimension_matches =
            grid.columns_auto_repeat.is_some() || areas.columns == grid.columns.len();
        if !row_dimension_matches || !column_dimension_matches {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property: "grid-template-areas",
                value: format!(
                    "area grid {}x{}가 auto-repeat 없는 축의 explicit track {}x{}와 다릅니다",
                    areas.rows,
                    areas.columns,
                    grid.rows.len(),
                    grid.columns.len()
                ),
            });
        }
    }
    if let Some(auto_repeat) = &grid.columns_auto_repeat {
        validate_auto_repeat_neighbor_tracks(
            node,
            "grid-template-columns",
            &grid.columns,
            auto_repeat,
            &auto_repeat.fixed_repeat_minmax_track_indices,
        )?;
    }
    if let Some(auto_repeat) = &grid.rows_auto_repeat {
        validate_auto_repeat_neighbor_tracks(
            node,
            "grid-template-rows",
            &grid.rows,
            auto_repeat,
            &auto_repeat.fixed_repeat_minmax_track_indices,
        )?;
    }
    validate_combined_grid_line_area_metadata(node, grid)?;
    project_grid_layout_with_reservations(
        node,
        grid,
        profile,
        math,
        |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
        |_| Ok(()),
    )
}

fn validate_combined_grid_line_area_metadata(
    node: NodeId,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    let Some(areas) = &grid.template_areas else {
        return Ok(());
    };
    if grid.columns_auto_repeat.is_none() && grid.rows_auto_repeat.is_none() {
        return Ok(());
    }

    fn add_bytes(total: &mut usize, bytes: usize) -> bool {
        let Some(next) = total.checked_add(bytes) else {
            return false;
        };
        if next > MAX_COMBINED_GRID_LINE_AREA_METADATA_BYTES {
            return false;
        }
        *total = next;
        true
    }

    fn add_line_names(total: &mut usize, groups: &[Vec<String>]) -> bool {
        let Some(group_headers) = groups.len().checked_mul(std::mem::size_of::<Vec<String>>())
        else {
            return false;
        };
        if !add_bytes(total, group_headers) {
            return false;
        }
        for group in groups {
            let Some(name_headers) = group.len().checked_mul(std::mem::size_of::<String>()) else {
                return false;
            };
            if !add_bytes(total, name_headers) {
                return false;
            }
            for name in group {
                if !add_bytes(total, name.len()) {
                    return false;
                }
            }
        }
        true
    }

    // Budget the projected Rust headers and UTF-8 payload before any projection allocation.
    // Allocator rounding and allocator-internal bookkeeping are outside this logical byte cap.
    let mut metadata_bytes = 0;
    for _ in 0..2 {
        if !add_bytes(&mut metadata_bytes, std::mem::size_of::<Vec<Vec<String>>>()) {
            return Err(combined_grid_line_area_metadata_error(node));
        }
    }
    for groups in [&grid.columns_line_names, &grid.rows_line_names] {
        if !add_line_names(&mut metadata_bytes, groups) {
            return Err(combined_grid_line_area_metadata_error(node));
        }
    }
    for auto_repeat in [
        grid.columns_auto_repeat.as_ref(),
        grid.rows_auto_repeat.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        if !add_bytes(
            &mut metadata_bytes,
            std::mem::size_of::<Option<spinon_layout::GridAutoRepeat>>(),
        ) {
            return Err(combined_grid_line_area_metadata_error(node));
        }
        if !add_line_names(&mut metadata_bytes, &auto_repeat.line_names) {
            return Err(combined_grid_line_area_metadata_error(node));
        }
    }

    if !add_bytes(
        &mut metadata_bytes,
        std::mem::size_of::<Option<spinon_layout::GridTemplateAreasInput>>(),
    ) {
        return Err(combined_grid_line_area_metadata_error(node));
    }
    let Some(area_headers) = areas
        .areas
        .len()
        .checked_mul(std::mem::size_of::<spinon_layout::GridTemplateAreaInput>())
    else {
        return Err(combined_grid_line_area_metadata_error(node));
    };
    if !add_bytes(&mut metadata_bytes, area_headers) {
        return Err(combined_grid_line_area_metadata_error(node));
    }
    for area in &areas.areas {
        if !add_bytes(&mut metadata_bytes, area.name.len()) {
            return Err(combined_grid_line_area_metadata_error(node));
        }
    }

    Ok(())
}

fn combined_grid_line_area_metadata_error(node: NodeId) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property: "grid-template-areas",
        value: "line-name과 template-area metadata 합계가 1 MiB 한도를 넘었습니다".to_owned(),
    }
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

fn validate_auto_repeat_neighbor_tracks(
    node: NodeId,
    property: &'static str,
    tracks: &[ComputedCssGridTrackSize],
    auto_repeat: &ComputedCssGridAutoRepeat,
    fixed_repeat_minmax_track_indices: &[usize],
) -> Result<(), StyleLayoutError> {
    if auto_repeat.start >= auto_repeat.end || auto_repeat.end > tracks.len() {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "auto-repeat fragment 범위가 track 목록을 벗어났습니다".to_owned(),
        });
    }
    let mut previous_provenance_index = None;
    for &index in fixed_repeat_minmax_track_indices {
        if index >= tracks.len()
            || (auto_repeat.start..auto_repeat.end).contains(&index)
            || previous_provenance_index.is_some_and(|previous| previous >= index)
            || !is_fixed_minimum_fraction_maximum(&tracks[index])
        {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "fixed-repeat MinMax provenance가 track 목록과 일치하지 않습니다".to_owned(),
            });
        }
        previous_provenance_index = Some(index);
    }
    let unsupported = tracks
        .iter()
        .enumerate()
        .filter(|(index, _)| *index < auto_repeat.start || *index >= auto_repeat.end)
        .find(|(index, track)| {
            let single_trailing_column_neighbor = property == "grid-template-columns"
                && auto_repeat.end.checked_add(1) == Some(tracks.len())
                && *index == auto_repeat.end;
            !is_supported_auto_repeat_neighbor(
                track,
                *index,
                single_trailing_column_neighbor,
                single_trailing_column_neighbor,
                fixed_repeat_minmax_track_indices,
            )
        });
    if unsupported.is_some() {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "auto-repeat 주변 track은 유한한 0 이상 length/percentage, fixed minimum이 있는 minmax, 제한된 signed calc(length + percentage), 열 auto-repeat의 단일 trailing minmax(min-content, px) 형태여야 합니다".to_owned(),
        });
    }
    Ok(())
}

fn is_supported_auto_repeat_neighbor(
    track: &ComputedCssGridTrackSize,
    index: usize,
    allow_intrinsic_trailing: bool,
    allow_fixed_minimum_auto_trailing: bool,
    fixed_repeat_minmax_track_indices: &[usize],
) -> bool {
    match track {
        ComputedCssGridTrackSize::LengthPx(value) | ComputedCssGridTrackSize::Percentage(value) => {
            value.is_finite() && *value >= 0.0
        }
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::MinContent,
            max: spinon_style::ComputedCssGridTrackBreadth::LengthPx(maximum),
        } if allow_intrinsic_trailing => maximum.is_finite() && *maximum >= 0.0,
        ComputedCssGridTrackSize::MinMax { min, max }
            if is_fixed_nonnegative_grid_track_breadth(min)
                && is_fixed_nonnegative_grid_track_breadth(max) =>
        {
            true
        }
        ComputedCssGridTrackSize::MinMax {
            min:
                spinon_style::ComputedCssGridTrackBreadth::LengthPx(minimum)
                | spinon_style::ComputedCssGridTrackBreadth::Percentage(minimum),
            max: spinon_style::ComputedCssGridTrackBreadth::Auto,
        } if allow_fixed_minimum_auto_trailing => minimum.is_finite() && *minimum >= 0.0,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(minimum),
            max: spinon_style::ComputedCssGridTrackBreadth::MaxContent,
        } => minimum.is_finite() && *minimum >= 0.0,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(minimum),
            max: spinon_style::ComputedCssGridTrackBreadth::Fraction(factor),
        } => minimum.is_finite() && *minimum >= 0.0 && factor.is_finite() && *factor >= 0.0,
        ComputedCssGridTrackSize::MinMax { .. } => {
            fixed_repeat_minmax_track_indices
                .binary_search(&index)
                .is_ok()
                && is_fixed_minimum_fraction_maximum(track)
        }
        ComputedCssGridTrackSize::Calc(value) => is_supported_auto_repeat_neighbor_calc(value),
        _ => false,
    }
}

fn is_fixed_nonnegative_grid_track_breadth(
    breadth: &spinon_style::ComputedCssGridTrackBreadth,
) -> bool {
    match breadth {
        spinon_style::ComputedCssGridTrackBreadth::LengthPx(value)
        | spinon_style::ComputedCssGridTrackBreadth::Percentage(value) => {
            value.is_finite() && *value >= 0.0
        }
        _ => false,
    }
}

fn is_fixed_minimum_fraction_maximum(track: &ComputedCssGridTrackSize) -> bool {
    matches!(
        track,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(min),
            max: spinon_style::ComputedCssGridTrackBreadth::Fraction(max),
        } if min.is_finite() && *min >= 0.0 && max.is_finite() && *max >= 0.0
    )
}

fn is_supported_auto_repeat_neighbor_calc(value: &ComputedCssMath) -> bool {
    let ComputedCssMath::Sum(terms) = value else {
        return false;
    };
    let [first, second] = terms.as_slice() else {
        return false;
    };
    let (length, percentage) = if let (Some(length), Some(percentage)) = (
        signed_auto_repeat_length(first),
        signed_auto_repeat_percentage(second),
    ) {
        (length, percentage)
    } else if let (Some(length), Some(percentage)) = (
        signed_auto_repeat_length(second),
        signed_auto_repeat_percentage(first),
    ) {
        (length, percentage)
    } else {
        return false;
    };
    length.is_finite() && percentage.is_finite()
}

fn signed_auto_repeat_length(value: &ComputedCssMath) -> Option<f32> {
    match value {
        ComputedCssMath::LengthPx(length) => Some(*length),
        ComputedCssMath::Negate(value) => match value.as_ref() {
            ComputedCssMath::LengthPx(length) => Some(-*length),
            _ => None,
        },
        _ => None,
    }
}

fn signed_auto_repeat_percentage(value: &ComputedCssMath) -> Option<f32> {
    match value {
        ComputedCssMath::Percentage(percentage) => Some(*percentage),
        ComputedCssMath::Negate(value) => match value.as_ref() {
            ComputedCssMath::Percentage(percentage) => Some(-*percentage),
            _ => None,
        },
        _ => None,
    }
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
            grid.columns_auto_repeat.as_ref(),
        )?;
        validate_axis_line_names(
            node,
            "grid-template-rows",
            &grid.rows,
            &grid.rows_line_names,
            grid.rows_auto_repeat.as_ref(),
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
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    let auto_rows = if sparse_row || dense_auto_placement {
        project_auto_rows(node, &grid.auto_rows, profile, math, &mut reserve)?
    } else {
        GridAutoRows::Auto
    };
    let auto_columns = if dense_auto_placement {
        project_auto_columns(node, &grid.auto_columns, profile, math, &mut reserve)?
    } else {
        GridAutoColumns::Auto
    };
    let columns_auto_repeat = grid
        .columns_auto_repeat
        .as_ref()
        .map(|auto_repeat| {
            project_auto_repeat(
                node,
                "grid-template-columns",
                auto_repeat,
                &mut reserve_line_metadata,
            )
        })
        .transpose()?;
    let rows_auto_repeat = grid
        .rows_auto_repeat
        .as_ref()
        .map(|auto_repeat| {
            project_auto_repeat(
                node,
                "grid-template-rows",
                auto_repeat,
                &mut reserve_line_metadata,
            )
        })
        .transpose()?;
    let template_areas = if columns_auto_repeat.is_some() || rows_auto_repeat.is_some() {
        grid.template_areas
            .as_ref()
            .map(|areas| project_template_areas(node, areas, &mut reserve_line_metadata))
            .transpose()?
    } else {
        None
    };
    Ok(GridContainerInput {
        columns,
        rows,
        columns_auto_repeat,
        rows_auto_repeat,
        columns_line_names,
        rows_line_names,
        template_areas,
        justify_items: LayoutAlignItems::Normal,
        auto_flow: match grid.auto_flow {
            ComputedCssGridAutoFlow::Row => GridAutoFlow::Row,
            ComputedCssGridAutoFlow::Column => GridAutoFlow::Column,
            ComputedCssGridAutoFlow::RowDense => GridAutoFlow::RowDense,
            ComputedCssGridAutoFlow::ColumnDense => GridAutoFlow::ColumnDense,
        },
        auto_rows,
        auto_columns,
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

fn project_template_areas(
    node: NodeId,
    source: &spinon_style::ComputedCssGridTemplateAreas,
    reserve: &mut impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<spinon_layout::GridTemplateAreasInput, StyleLayoutError> {
    let property = "grid-template-areas";
    let rows = u16::try_from(source.rows)
        .ok()
        .filter(|rows| *rows != 0 && usize::from(*rows) <= spinon_layout::GRID_MAX_TRACKS);
    let columns = u16::try_from(source.columns)
        .ok()
        .filter(|columns| *columns != 0 && usize::from(*columns) <= spinon_layout::GRID_MAX_TRACKS);
    let (Some(rows), Some(columns)) = (rows, columns) else {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "area dimension이 Taffy의 grid coordinate 한도를 벗어났습니다".to_owned(),
        });
    };
    let mut metadata_bytes = source
        .areas
        .len()
        .checked_mul(std::mem::size_of::<spinon_layout::GridTemplateAreaInput>())
        .filter(|bytes| *bytes <= MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES)
        .ok_or_else(|| StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "area metadata가 1 MiB 한도를 넘었습니다".to_owned(),
        })?;
    for area in &source.areas {
        if area.name.is_empty()
            || area.name.as_bytes().contains(&0)
            || area.row_start >= area.row_end
            || area.row_end > source.rows
            || area.column_start >= area.column_end
            || area.column_end > source.columns
        {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "area 이름 또는 rectangle이 유효하지 않습니다".to_owned(),
            });
        }
        metadata_bytes = metadata_bytes
            .checked_add(area.name.len())
            .filter(|bytes| *bytes <= MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES)
            .ok_or_else(|| StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "area metadata가 1 MiB 한도를 넘었습니다".to_owned(),
            })?;
    }
    reserve(GridLineNameReservationSite::Areas).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    let mut areas = Vec::new();
    areas.try_reserve_exact(source.areas.len()).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    for area in &source.areas {
        let (Some(row_start), Some(row_end), Some(column_start), Some(column_end)) = (
            u16::try_from(area.row_start).ok(),
            u16::try_from(area.row_end).ok(),
            u16::try_from(area.column_start).ok(),
            u16::try_from(area.column_end).ok(),
        ) else {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "area rectangle이 Taffy의 grid coordinate 범위를 벗어났습니다".to_owned(),
            });
        };
        let mut name = String::new();
        reserve(GridLineNameReservationSite::AreaName).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property,
            })
        })?;
        name.try_reserve(area.name.len()).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property,
            })
        })?;
        name.push_str(&area.name);
        areas.push(spinon_layout::GridTemplateAreaInput {
            name,
            row_start,
            row_end,
            column_start,
            column_end,
        });
    }
    Ok(spinon_layout::GridTemplateAreasInput {
        rows,
        columns,
        areas,
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

fn project_auto_repeat(
    node: NodeId,
    property: &'static str,
    auto_repeat: &ComputedCssGridAutoRepeat,
    reserve: &mut impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<GridAutoRepeat, StyleLayoutError> {
    Ok(GridAutoRepeat {
        mode: project_auto_repeat_mode(auto_repeat.mode),
        start: auto_repeat.start,
        end: auto_repeat.end,
        line_names: project_line_names(node, property, &auto_repeat.line_names, reserve)?,
    })
}

fn validate_line_name_count(
    node: NodeId,
    property: &'static str,
    tracks: &[ComputedCssGridTrackSize],
    names: &[Vec<String>],
) -> Result<(), StyleLayoutError> {
    if tracks.is_empty() && names.is_empty() {
        return Ok(());
    }
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
    auto_repeat: Option<&ComputedCssGridAutoRepeat>,
) -> Result<(), StyleLayoutError> {
    let Some(auto_repeat) = auto_repeat else {
        return validate_line_name_count(node, property, tracks, names);
    };
    if auto_repeat.start >= auto_repeat.end || auto_repeat.end > tracks.len() {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "auto-repeat fragment 범위가 track 목록을 벗어났습니다".to_owned(),
        });
    }
    let repeated_track_count = auto_repeat.end - auto_repeat.start;
    let component_count = tracks
        .len()
        .checked_sub(repeated_track_count)
        .and_then(|count| count.checked_add(1));
    let valid_outer_count = component_count.and_then(|count| count.checked_add(1));
    let has_authored_names = names
        .iter()
        .chain(&auto_repeat.line_names)
        .any(|group| !group.is_empty());
    let valid_outer_names = if has_authored_names {
        valid_outer_count == Some(names.len())
    } else {
        names.is_empty() || valid_outer_count == Some(names.len())
    };
    let valid_repeated_names = if auto_repeat.line_names.iter().any(|group| !group.is_empty()) {
        auto_repeat.line_names.len() == repeated_track_count + 1
    } else {
        auto_repeat.line_names.is_empty()
            || auto_repeat.line_names.len() == repeated_track_count + 1
    };
    if !valid_outer_names || !valid_repeated_names {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: "auto-repeat line-name group 수가 component 경계 N+1 규칙과 다릅니다".to_owned(),
        });
    }
    Ok(())
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
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridAutoColumns, StyleLayoutError> {
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => Ok(GridAutoColumns::Auto),
        ComputedCssGridAutoColumns::LengthPx(value) => Ok(GridAutoColumns::LengthPx(*value)),
        ComputedCssGridAutoColumns::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 =>
        {
            project_tracks(
                tracks,
                node,
                "grid-auto-columns",
                LayoutCssMathProperty::GridTemplateColumns,
                math,
                reserve,
            )
            .map(GridAutoColumns::TrackList)
        }
        ComputedCssGridAutoColumns::TrackList(_) => {
            Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property: "grid-auto-columns",
                value: "track-size 목록은 C11.4 track-sizing profile에서만 지원됩니다".to_owned(),
            })
        }
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
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridAutoRows, StyleLayoutError> {
    match auto_rows {
        ComputedCssGridAutoRows::Auto => Ok(GridAutoRows::Auto),
        ComputedCssGridAutoRows::LengthPx(value) => Ok(GridAutoRows::LengthPx(*value)),
        ComputedCssGridAutoRows::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 =>
        {
            project_tracks(
                tracks,
                node,
                "grid-auto-rows",
                LayoutCssMathProperty::GridTemplateRows,
                math,
                reserve,
            )
            .map(GridAutoRows::TrackList)
        }
        ComputedCssGridAutoRows::TrackList(_) => Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "grid-auto-rows",
            value: "track-size 목록은 C11.4 track-sizing profile에서만 지원됩니다".to_owned(),
        }),
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

#[cfg(test)]
#[path = "grid_values/auto_repeat_neighbor_tests.rs"]
mod auto_repeat_neighbor_tests;

#[cfg(test)]
#[path = "grid_values/auto_track_tests.rs"]
mod auto_track_tests;

#[cfg(test)]
#[path = "grid_values/combined_metadata_tests.rs"]
mod combined_metadata_tests;
