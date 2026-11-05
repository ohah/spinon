use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRepeat, GridAutoRows, GridContainerInput,
    GridPlacementMode, GridTrackLengthPercentage, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutAlignItems, LayoutCssMathProperty,
};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTemplateAreas, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
    ComputedCssMath, ComputedStyleProfile,
};

use super::super::typed_math::CssMathProjector;
use super::root_html_column_math::supported_root_html_auto_track_calc;
use crate::StyleLayoutError;

#[path = "grid_values/root_html_auto_repeat.rs"]
pub(super) mod root_html_auto_repeat;

pub(super) const MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES: usize = 1024 * 1024;
const MAX_COMBINED_GRID_LINE_AREA_METADATA_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RootHtmlNamedAreaAutoMinimumAxis {
    Columns,
    Rows,
}

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
    project_grid_layout_with_route(node, grid, profile, math, false, false)
}

pub(super) fn project_root_html_one_axis_grid_layout_with_math(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
) -> Result<GridContainerInput, StyleLayoutError> {
    project_grid_layout_with_route(node, grid, profile, math, true, false)
}

pub(super) fn project_root_html_direct_text_aspect_ratio_grid_layout_with_math(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
) -> Result<GridContainerInput, StyleLayoutError> {
    project_grid_layout_with_route(node, grid, profile, math, false, true)
}

fn project_grid_layout_with_route(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    root_html_one_axis_placement: bool,
    root_html_direct_text_aspect_ratio: bool,
) -> Result<GridContainerInput, StyleLayoutError> {
    if profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 {
        validate_root_html_grid_tracks(
            node,
            grid,
            root_html_one_axis_placement,
            root_html_direct_text_aspect_ratio,
        )?;
    }
    let has_auto_repeat = grid.columns_auto_repeat.is_some() || grid.rows_auto_repeat.is_some();
    if has_auto_repeat {
        let property = if grid.columns_auto_repeat.is_some() {
            "grid-template-columns"
        } else {
            "grid-template-rows"
        };
        if profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
            && profile != ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "auto-repeat은 각 track list에서 하나씩만 제한 track-sizing profile로 지원됩니다"
                    .to_owned(),
            });
        }
    }
    if let Some(areas) = &grid.template_areas {
        let template_grid_can_expand_areas =
            profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1;
        let row_dimension_matches =
            grid.rows_auto_repeat.is_some() || areas.rows == grid.rows.len();
        let column_dimension_matches =
            grid.columns_auto_repeat.is_some() || areas.columns == grid.columns.len();
        if !template_grid_can_expand_areas && (!row_dimension_matches || !column_dimension_matches)
        {
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
    validate_combined_grid_line_area_metadata(node, grid, profile)?;
    project_grid_layout_with_route_reservations(
        node,
        grid,
        profile,
        math,
        GridProjectionRouteOptions {
            root_html_one_axis_placement,
            root_html_direct_text_aspect_ratio,
        },
        |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
        |_| Ok(()),
    )
}

fn validate_root_html_grid_tracks(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    root_html_one_axis_placement: bool,
    root_html_direct_text_aspect_ratio: bool,
) -> Result<(), StyleLayoutError> {
    let unsupported = |property, value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    };
    let implicit_rows = root_html_has_supported_implicit_rows(&grid.auto_rows);
    let column_flow = grid.auto_flow == ComputedCssGridAutoFlow::Column;
    let dense_flow = matches!(
        grid.auto_flow,
        ComputedCssGridAutoFlow::RowDense | ComputedCssGridAutoFlow::ColumnDense
    );
    let fixed_px_placement_flow = column_flow || dense_flow;
    let has_intrinsic_tracks = root_html_has_intrinsic_explicit_tracks(grid);
    let intrinsic_auto_rows = !root_html_direct_text_aspect_ratio
        && !has_intrinsic_tracks
        && root_html_has_intrinsic_implicit_min_content_rows(grid);
    let direct_text_aspect_ratio_has_only_its_auto_row = root_html_direct_text_aspect_ratio
        && grid.rows.as_slice() == [ComputedCssGridTrackSize::Auto]
        && !grid
            .columns
            .iter()
            .any(computed_track_uses_intrinsic_sizing);
    if root_html_direct_text_aspect_ratio
        && has_intrinsic_tracks
        && !direct_text_aspect_ratio_has_only_its_auto_row
    {
        return Err(unsupported(
            "grid-template-columns",
            "Root HTML direct-text aspect-ratio route does not combine with intrinsic explicit tracks",
        ));
    }
    let root_html_auto_repeat = root_html_auto_repeat::validate_grid_grammar(
        node,
        grid,
        root_html_one_axis_placement,
        root_html_direct_text_aspect_ratio,
    )?;
    if intrinsic_auto_rows {
        let single_positive_px_track = |tracks: &[ComputedCssGridTrackSize]| {
            matches!(
                tracks,
                [ComputedCssGridTrackSize::LengthPx(value)]
                    if value.is_finite() && *value > 0.0
            )
        };
        if !single_positive_px_track(&grid.columns)
            || !single_positive_px_track(&grid.rows)
            || grid.template_areas.is_some()
            || grid.columns_auto_repeat.is_some()
            || grid.rows_auto_repeat.is_some()
            || grid.columns_fixed_repeat_group_count != 0
            || grid.rows_fixed_repeat_group_count != 0
            || grid.auto_flow != ComputedCssGridAutoFlow::Row
            || !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
        {
            return Err(unsupported(
                "grid-auto-rows",
                "Root HTML intrinsic implicit min-content rows require one positive fixed px track per explicit axis, row flow, no areas or repeats, and no implicit columns",
            ));
        }
    }
    let has_intrinsic_explicit_tracks = !root_html_direct_text_aspect_ratio && has_intrinsic_tracks;
    if has_intrinsic_explicit_tracks
        && (root_html_direct_text_aspect_ratio
            || !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto)
            || !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
            || root_html_has_fit_content_tracks(grid)
            || fixed_px_placement_flow)
    {
        return Err(unsupported(
            if !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto) {
                "grid-auto-rows"
            } else if !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto) {
                "grid-auto-columns"
            } else {
                "grid-template-columns"
            },
            "Root HTML intrinsic explicit tracks do not combine with implicit tracks, fit-content(), column/dense flow, or the direct-text aspect-ratio route",
        ));
    }
    for (property, tracks, line_names, auto_repeat) in [
        (
            "grid-template-columns",
            grid.columns.as_slice(),
            grid.columns_line_names.as_slice(),
            grid.columns_auto_repeat.as_ref(),
        ),
        (
            "grid-template-rows",
            grid.rows.as_slice(),
            grid.rows_line_names.as_slice(),
            grid.rows_auto_repeat.as_ref(),
        ),
    ] {
        let single_auto_row = root_html_direct_text_aspect_ratio
            && property == "grid-template-rows"
            && tracks == [ComputedCssGridTrackSize::Auto];
        let has_auto_minimum_track = tracks.iter().any(|track| {
            matches!(
                track,
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::Auto,
                    ..
                }
            )
        });
        if tracks.is_empty()
            || (auto_repeat.is_some() && !root_html_auto_repeat)
            || line_names.iter().any(|group| !group.is_empty())
            || (!single_auto_row
                && tracks
                    .iter()
                    .any(|track| !is_supported_root_html_explicit_track(track)))
            || (implicit_rows && has_auto_minimum_track)
        {
            return Err(unsupported(
                property,
                if implicit_rows && has_auto_minimum_track {
                    "고정 implicit row와 함께 쓰는 Root HTML Grid explicit track은 자동 최소 크기 minmax()를 지원하지 않습니다"
                } else {
                    "Root HTML Grid explicit track은 finite non-negative px/percentage/fr, auto/min-content/max-content, minmax() 및 fit-content(px|%)를 제한 지원하며 line name과 auto-repeat은 지원하지 않습니다"
                },
            ));
        }
        if fixed_px_placement_flow
            && tracks.iter().any(|track| {
                !matches!(
                    track,
                    ComputedCssGridTrackSize::LengthPx(value)
                        if value.is_finite() && *value >= 0.0
                )
            })
        {
            return Err(unsupported(
                property,
                "Root HTML column/dense placement mode requires non-empty finite non-negative direct px tracks",
            ));
        }
    }
    if fixed_px_placement_flow {
        for (property, fixed_repeat_group_count) in [
            (
                "grid-template-columns",
                grid.columns_fixed_repeat_group_count,
            ),
            ("grid-template-rows", grid.rows_fixed_repeat_group_count),
        ] {
            if fixed_repeat_group_count != 0 {
                return Err(unsupported(
                    property,
                    "Root HTML column/dense placement mode excludes repeat() in explicit tracks",
                ));
            }
        }
    }
    if column_flow
        && (grid.template_areas.is_some()
            || !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto))
    {
        return Err(unsupported(
            "grid-auto-flow",
            "Root HTML column-flow mode excludes template areas and non-auto implicit rows",
        ));
    }
    if dense_flow {
        if grid.template_areas.is_some() {
            return Err(unsupported(
                "grid-template-areas",
                "Root HTML dense placement mode excludes template areas",
            ));
        }
        if !root_html_dense_auto_rows_supported(&grid.auto_rows) {
            return Err(unsupported(
                "grid-auto-rows",
                "Root HTML dense placement mode requires auto or one finite non-negative fixed px implicit row size",
            ));
        }
        if !root_html_dense_auto_columns_supported(&grid.auto_columns) {
            return Err(unsupported(
                "grid-auto-columns",
                "Root HTML dense placement mode requires auto or one finite non-negative fixed px implicit column size",
            ));
        }
    }
    if root_html_one_axis_placement
        && (!root_html_dense_auto_rows_supported(&grid.auto_rows)
            || !root_html_dense_auto_columns_supported(&grid.auto_columns))
    {
        return Err(unsupported(
            "grid-auto-rows",
            "Root HTML one-axis mode allows only auto or one finite non-negative px implicit track size per axis",
        ));
    }
    if implicit_rows && grid.template_areas.is_some() {
        return Err(unsupported(
            "grid-auto-rows",
            "Root HTML implicit row sizing은 grid-template-areas와 함께 사용할 수 없습니다",
        ));
    }
    if let Some(areas) = &grid.template_areas {
        validate_root_html_named_area_template(node, grid, areas)?;
    }
    if grid.auto_flow != ComputedCssGridAutoFlow::Row && !column_flow && !dense_flow {
        return Err(unsupported(
            "grid-auto-flow",
            "Root HTML Grid에서는 기본 row auto-placement만 지원합니다",
        ));
    }
    if !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto)
        && !implicit_rows
        && !intrinsic_auto_rows
    {
        return Err(unsupported(
            "grid-auto-rows",
            "Root HTML Grid에서는 finite non-negative direct px/percentage/fr grid-auto-rows 값과 목록만 지원합니다",
        ));
    }
    let invalid_implicit_row = match &grid.auto_rows {
        ComputedCssGridAutoRows::LengthPx(value) => !value.is_finite() || *value < 0.0,
        ComputedCssGridAutoRows::TrackList(tracks) => tracks.iter().any(|track| match track {
            ComputedCssGridTrackSize::LengthPx(value)
            | ComputedCssGridTrackSize::Percentage(value)
            | ComputedCssGridTrackSize::Fraction(value) => !value.is_finite() || *value < 0.0,
            ComputedCssGridTrackSize::Calc(expression) => {
                !supported_root_html_auto_track_calc(expression)
            }
            ComputedCssGridTrackSize::MinContent if intrinsic_auto_rows => false,
            _ => true,
        }),
        _ => false,
    };
    if invalid_implicit_row {
        return Err(unsupported(
            "grid-auto-rows",
            "Root HTML grid-auto-rows는 direct px/percentage/fr와 제한 calc(px + %) 값만 포함해야 합니다",
        ));
    }
    if column_flow && !root_html_auto_columns_supported(&grid.auto_columns) {
        return Err(unsupported(
            "grid-auto-columns",
            "Root HTML column-flow mode requires auto or finite non-negative direct px/percentage/fraction auto columns",
        ));
    }
    if !column_flow
        && !dense_flow
        && !root_html_one_axis_placement
        && !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
    {
        return Err(unsupported(
            "grid-auto-columns",
            "Root HTML row-flow mode excludes implicit column sizing",
        ));
    }
    Ok(())
}

pub(super) fn root_html_has_intrinsic_explicit_tracks(grid: &ComputedCssGridLayout) -> bool {
    grid.columns
        .iter()
        .chain(&grid.rows)
        .any(computed_track_uses_intrinsic_sizing)
}

pub(super) fn root_html_has_intrinsic_implicit_min_content_rows(
    grid: &ComputedCssGridLayout,
) -> bool {
    matches!(
        &grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(tracks)
            if tracks.as_slice() == [ComputedCssGridTrackSize::MinContent]
    )
}

fn computed_track_uses_intrinsic_sizing(track: &ComputedCssGridTrackSize) -> bool {
    match track {
        ComputedCssGridTrackSize::Auto
        | ComputedCssGridTrackSize::MinContent
        | ComputedCssGridTrackSize::MaxContent => true,
        ComputedCssGridTrackSize::MinMax { min, max } => {
            let existing_auto_minimum_fraction = matches!(min, ComputedCssGridTrackBreadth::Auto)
                && matches!(max, ComputedCssGridTrackBreadth::Fraction(value) if value.is_finite() && *value >= 0.0);
            let percentage_minimum_fraction_maximum = matches!(
                (min, max),
                (
                    ComputedCssGridTrackBreadth::Percentage(_),
                    ComputedCssGridTrackBreadth::Fraction(_)
                )
            );
            !existing_auto_minimum_fraction
                && (percentage_minimum_fraction_maximum
                    || matches!(
                        min,
                        ComputedCssGridTrackBreadth::Auto
                            | ComputedCssGridTrackBreadth::MinContent
                            | ComputedCssGridTrackBreadth::MaxContent
                    )
                    || matches!(
                        max,
                        ComputedCssGridTrackBreadth::Auto
                            | ComputedCssGridTrackBreadth::MinContent
                            | ComputedCssGridTrackBreadth::MaxContent
                    ))
        }
        ComputedCssGridTrackSize::LengthPx(_)
        | ComputedCssGridTrackSize::Percentage(_)
        | ComputedCssGridTrackSize::Fraction(_)
        | ComputedCssGridTrackSize::FitContent(_)
        | ComputedCssGridTrackSize::Calc(_) => false,
    }
}

fn root_html_dense_auto_rows_supported(auto_rows: &ComputedCssGridAutoRows) -> bool {
    match auto_rows {
        ComputedCssGridAutoRows::Auto => true,
        ComputedCssGridAutoRows::LengthPx(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridAutoRows::TrackList(_) | ComputedCssGridAutoRows::Unsupported(_) => false,
    }
}

fn root_html_dense_auto_columns_supported(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => true,
        ComputedCssGridAutoColumns::LengthPx(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridAutoColumns::TrackList(_) | ComputedCssGridAutoColumns::Unsupported(_) => {
            false
        }
    }
}

fn is_supported_root_html_explicit_track(track: &ComputedCssGridTrackSize) -> bool {
    let finite_nonnegative = |value: &f32| value.is_finite() && *value >= 0.0;
    match track {
        ComputedCssGridTrackSize::LengthPx(value)
        | ComputedCssGridTrackSize::Percentage(value)
        | ComputedCssGridTrackSize::Fraction(value) => finite_nonnegative(value),
        ComputedCssGridTrackSize::Auto
        | ComputedCssGridTrackSize::MinContent
        | ComputedCssGridTrackSize::MaxContent => true,
        ComputedCssGridTrackSize::MinMax { min, max } => {
            let supported_min = match min {
                spinon_style::ComputedCssGridTrackBreadth::LengthPx(value)
                | spinon_style::ComputedCssGridTrackBreadth::Percentage(value) => {
                    finite_nonnegative(value)
                }
                spinon_style::ComputedCssGridTrackBreadth::Auto
                | spinon_style::ComputedCssGridTrackBreadth::MinContent
                | spinon_style::ComputedCssGridTrackBreadth::MaxContent => true,
                spinon_style::ComputedCssGridTrackBreadth::Fraction(_)
                | spinon_style::ComputedCssGridTrackBreadth::Calc(_) => false,
            };
            let supported_max = match max {
                spinon_style::ComputedCssGridTrackBreadth::LengthPx(value)
                | spinon_style::ComputedCssGridTrackBreadth::Percentage(value)
                | spinon_style::ComputedCssGridTrackBreadth::Fraction(value) => {
                    finite_nonnegative(value)
                }
                spinon_style::ComputedCssGridTrackBreadth::Auto
                | spinon_style::ComputedCssGridTrackBreadth::MinContent
                | spinon_style::ComputedCssGridTrackBreadth::MaxContent => true,
                spinon_style::ComputedCssGridTrackBreadth::Calc(_) => false,
            };
            supported_min && supported_max
        }
        ComputedCssGridTrackSize::FitContent(value) => match value {
            spinon_style::ComputedCssGridTrackLengthPercentage::LengthPx(value)
            | spinon_style::ComputedCssGridTrackLengthPercentage::Percentage(value) => {
                finite_nonnegative(value)
            }
            spinon_style::ComputedCssGridTrackLengthPercentage::Calc(_) => false,
        },
        _ => false,
    }
}

fn root_html_auto_columns_supported(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    let valid_track = |track: &ComputedCssGridTrackSize| match track {
        ComputedCssGridTrackSize::LengthPx(value)
        | ComputedCssGridTrackSize::Percentage(value)
        | ComputedCssGridTrackSize::Fraction(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridTrackSize::Calc(expression) => {
            supported_root_html_auto_track_calc(expression)
        }
        _ => false,
    };
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => true,
        ComputedCssGridAutoColumns::LengthPx(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridAutoColumns::TrackList(tracks) => {
            !tracks.is_empty()
                && tracks.len() <= spinon_layout::GRID_MAX_TRACKS
                && tracks.iter().all(valid_track)
        }
        ComputedCssGridAutoColumns::Unsupported(_) => false,
    }
}

fn root_html_direct_auto_row_track_list(tracks: &[ComputedCssGridTrackSize]) -> bool {
    !tracks.is_empty()
        && tracks.len() <= spinon_layout::GRID_MAX_TRACKS
        && tracks.iter().all(|track| match track {
            ComputedCssGridTrackSize::LengthPx(_)
            | ComputedCssGridTrackSize::Percentage(_)
            | ComputedCssGridTrackSize::Fraction(_) => true,
            ComputedCssGridTrackSize::Calc(expression) => {
                supported_root_html_auto_track_calc(expression)
            }
            _ => false,
        })
}

pub(super) fn root_html_has_supported_implicit_rows(auto_rows: &ComputedCssGridAutoRows) -> bool {
    match auto_rows {
        ComputedCssGridAutoRows::LengthPx(_) => true,
        ComputedCssGridAutoRows::TrackList(tracks) => root_html_direct_auto_row_track_list(tracks),
        ComputedCssGridAutoRows::Auto | ComputedCssGridAutoRows::Unsupported(_) => false,
    }
}

fn validate_root_html_named_area_template(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    areas: &ComputedCssGridTemplateAreas,
) -> Result<(), StyleLayoutError> {
    if root_html_has_intrinsic_explicit_tracks(grid) {
        return super::root_html_intrinsic_named_area::validate_root_html_intrinsic_named_area_template(
            node, grid, areas,
        );
    }
    let unsupported = |property, value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    };
    let auto_minimum_axis = root_html_named_area_auto_minimum_axis(grid);
    if auto_minimum_axis.is_none() {
        for (property, tracks) in [
            ("grid-template-columns", grid.columns.as_slice()),
            ("grid-template-rows", grid.rows.as_slice()),
        ] {
            if tracks
                .iter()
                .any(|track| matches!(track, ComputedCssGridTrackSize::MinMax { .. }))
            {
                return Err(unsupported(
                    property,
                    "Root HTML named-area 제한 profile은 지정된 자동 최소 크기 행렬 밖의 minmax()를 허용하지 않습니다",
                ));
            }
        }
    }
    let columns_match = matches!(
        grid.columns.as_slice(),
        [ComputedCssGridTrackSize::LengthPx(first), ComputedCssGridTrackSize::LengthPx(second)]
            if *first == 40.0 && *second == 80.0
    );
    let rows_match = matches!(
        grid.rows.as_slice(),
        [ComputedCssGridTrackSize::LengthPx(first), ComputedCssGridTrackSize::LengthPx(second)]
            if *first == 20.0 && *second == 50.0
    );
    let fractional_columns_match = matches!(
        grid.columns.as_slice(),
        [ComputedCssGridTrackSize::Fraction(first), ComputedCssGridTrackSize::Fraction(second)]
            if *first == 1.0 && *second == 2.0
    );
    let fractional_rows_match = matches!(
        grid.rows.as_slice(),
        [ComputedCssGridTrackSize::Fraction(first), ComputedCssGridTrackSize::Fraction(second)]
            if *first == 1.0 && *second == 2.0
    );
    let bounded_fraction_tracks_match = fractional_columns_match && fractional_rows_match;
    let fixed_tracks_match = columns_match && rows_match;
    let percentage_tracks_match =
        contains_percentage_track(&grid.columns) || contains_percentage_track(&grid.rows);
    let percentage_area_tracks_match = percentage_tracks_match
        && is_two_direct_px_percentage_tracks(&grid.columns)
        && is_two_direct_px_percentage_tracks(&grid.rows);
    let fit_content_area_tracks_match =
        root_html_dimension_matched_fit_content_area_tracks(grid, areas);
    if !(fixed_tracks_match
        || bounded_fraction_tracks_match
        || percentage_area_tracks_match
        || fit_content_area_tracks_match
        || auto_minimum_axis.is_some())
    {
        return Err(unsupported(
            "grid-template-areas",
            "Root HTML named-area 제한 경로는 기존 header/side/content 고정 px·1fr/2fr·직접 px/percentage 패턴 또는 dimension-matched px/fit-content(px) 트랙을 지원합니다",
        ));
    }
    let expected = [
        ("header", 0, 1, 0, 2),
        ("side", 1, 2, 0, 1),
        ("content", 1, 2, 1, 2),
    ];
    let existing_named_areas_match = areas.rows == 2
        && areas.columns == 2
        && areas.areas.len() == expected.len()
        && expected
            .iter()
            .all(|(name, row_start, row_end, column_start, column_end)| {
                areas.areas.iter().any(|area| {
                    area.name == *name
                        && area.row_start == *row_start
                        && area.row_end == *row_end
                        && area.column_start == *column_start
                        && area.column_end == *column_end
                })
            });
    if !fit_content_area_tracks_match && !existing_named_areas_match {
        return Err(unsupported(
            "grid-template-areas",
            "Root HTML 기존 named-area profiles는 header/side/content의 직사각형 2x2 영역을 요구합니다",
        ));
    }
    Ok(())
}

/// Root HTML runtime이 허용하는 named-area 자동 최소 크기 track 행렬을 식별합니다.
/// 비교 대조군으로 0 최소 크기 track도 포함하며, 그 밖의 `minmax()` 조합은
/// `validate_root_html_named_area_template`에서 계속 거부합니다.
pub(super) fn root_html_named_area_auto_minimum_axis(
    grid: &ComputedCssGridLayout,
) -> Option<RootHtmlNamedAreaAutoMinimumAxis> {
    let is_zero_minimum_fraction = |track: &ComputedCssGridTrackSize| {
        matches!(
            track,
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(min),
                max: ComputedCssGridTrackBreadth::Fraction(factor),
            } if *min == 0.0 && factor.is_finite() && *factor >= 0.0
        )
    };
    let is_auto_minimum_fraction = |track: &ComputedCssGridTrackSize| {
        matches!(
            track,
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::Fraction(factor),
            } if factor.is_finite() && *factor >= 0.0
        )
    };
    let is_auto_or_zero_minimum_fraction = |track: &ComputedCssGridTrackSize| {
        is_auto_minimum_fraction(track) || is_zero_minimum_fraction(track)
    };

    let columns_axis = matches!(
        grid.columns.as_slice(),
        [first, ComputedCssGridTrackSize::LengthPx(neighbor)]
            if is_auto_or_zero_minimum_fraction(first) && *neighbor == 40.0
    ) && matches!(
        grid.rows.as_slice(),
        [ComputedCssGridTrackSize::LengthPx(first), ComputedCssGridTrackSize::LengthPx(second)]
            if *first == 20.0 && *second == 20.0
    );
    if columns_axis {
        return Some(RootHtmlNamedAreaAutoMinimumAxis::Columns);
    }

    let rows_axis = matches!(
        grid.columns.as_slice(),
        [ComputedCssGridTrackSize::LengthPx(first), ComputedCssGridTrackSize::LengthPx(second)]
            if *first == 40.0 && *second == 40.0
    ) && matches!(
        grid.rows.as_slice(),
        [first, ComputedCssGridTrackSize::LengthPx(neighbor)]
            if is_auto_or_zero_minimum_fraction(first) && *neighbor == 40.0
    );
    rows_axis.then_some(RootHtmlNamedAreaAutoMinimumAxis::Rows)
}

pub(super) fn root_html_dimension_matched_fit_content_area_tracks(
    grid: &ComputedCssGridLayout,
    areas: &ComputedCssGridTemplateAreas,
) -> bool {
    if areas.rows == 0
        || areas.columns == 0
        || areas.rows != grid.rows.len()
        || areas.columns != grid.columns.len()
        || areas.areas.is_empty()
    {
        return false;
    }

    let mut has_fit_content = false;
    for track in grid.columns.iter().chain(&grid.rows) {
        let valid = match track {
            ComputedCssGridTrackSize::LengthPx(value) => value.is_finite() && *value >= 0.0,
            ComputedCssGridTrackSize::FitContent(
                spinon_style::ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) => {
                has_fit_content = true;
                value.is_finite() && *value >= 0.0
            }
            _ => false,
        };
        if !valid {
            return false;
        }
    }
    has_fit_content
}

fn contains_percentage_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::Percentage(_)))
}

fn is_two_direct_px_percentage_tracks(tracks: &[ComputedCssGridTrackSize]) -> bool {
    matches!(
        tracks,
        [
            ComputedCssGridTrackSize::LengthPx(_) | ComputedCssGridTrackSize::Percentage(_),
            ComputedCssGridTrackSize::LengthPx(_) | ComputedCssGridTrackSize::Percentage(_)
        ]
    )
}

fn validate_combined_grid_line_area_metadata(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
) -> Result<(), StyleLayoutError> {
    let Some(areas) = &grid.template_areas else {
        return Ok(());
    };
    let carries_area_metadata = grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        || (profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            && grid.template_areas.is_some());
    if !carries_area_metadata {
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
    if profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        && grid.columns_auto_repeat.is_none()
        && grid.rows_auto_repeat.is_none()
    {
        for (groups, authored_track_count, area_track_count) in [
            (&grid.columns_line_names, grid.columns.len(), areas.columns),
            (&grid.rows_line_names, grid.rows.len(), areas.rows),
        ] {
            let target_groups = authored_track_count
                .max(area_track_count)
                .checked_add(1)
                .ok_or_else(|| combined_grid_line_area_metadata_error(node))?;
            let missing_groups = target_groups.saturating_sub(groups.len());
            let Some(group_headers) =
                missing_groups.checked_mul(std::mem::size_of::<Vec<String>>())
            else {
                return Err(combined_grid_line_area_metadata_error(node));
            };
            if !add_bytes(&mut metadata_bytes, group_headers) {
                return Err(combined_grid_line_area_metadata_error(node));
            }
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

#[cfg(test)]
fn project_grid_layout_with_reservations(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
    mut reserve_line_metadata: impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    project_grid_layout_with_route_reservations(
        node,
        grid,
        profile,
        math,
        GridProjectionRouteOptions {
            root_html_one_axis_placement: false,
            root_html_direct_text_aspect_ratio: false,
        },
        &mut reserve,
        &mut reserve_line_metadata,
    )
}

#[derive(Clone, Copy)]
struct GridProjectionRouteOptions {
    root_html_one_axis_placement: bool,
    root_html_direct_text_aspect_ratio: bool,
}

fn project_grid_layout_with_route_reservations(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    profile: ComputedStyleProfile,
    math: &mut CssMathProjector,
    route_options: GridProjectionRouteOptions,
    mut reserve: impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
    mut reserve_line_metadata: impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<GridContainerInput, StyleLayoutError> {
    let GridProjectionRouteOptions {
        root_html_one_axis_placement,
        root_html_direct_text_aspect_ratio,
    } = route_options;
    let mut columns = project_tracks(
        &grid.columns,
        node,
        "grid-template-columns",
        LayoutCssMathProperty::GridTemplateColumns,
        math,
        &mut reserve,
    )?;
    let mut rows = project_tracks(
        &grid.rows,
        node,
        "grid-template-rows",
        LayoutCssMathProperty::GridTemplateRows,
        math,
        &mut reserve,
    )?;
    let root_html_named_areas = profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        && grid.template_areas.is_some();
    let (mut columns_line_names, mut rows_line_names) = if root_html_named_areas {
        (
            empty_grid_line_names(
                node,
                "grid-template-columns",
                columns.len(),
                &mut reserve_line_metadata,
            )?,
            empty_grid_line_names(
                node,
                "grid-template-rows",
                rows.len(),
                &mut reserve_line_metadata,
            )?,
        )
    } else if matches!(
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
    let root_html_dense_flow = profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        && matches!(
            grid.auto_flow,
            ComputedCssGridAutoFlow::RowDense | ComputedCssGridAutoFlow::ColumnDense
        );
    let dense_auto_placement = root_html_dense_flow
        || matches!(
            profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        );
    let allow_template_area_track_sizing = profile
        == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        && grid.template_areas.is_some();
    let root_html_implicit_rows = profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        && !root_html_has_intrinsic_explicit_tracks(grid)
        && (root_html_has_supported_implicit_rows(&grid.auto_rows)
            || grid
                .columns
                .iter()
                .any(|track| matches!(track, ComputedCssGridTrackSize::MinMax { .. }))
            || grid
                .rows
                .iter()
                .any(|track| matches!(track, ComputedCssGridTrackSize::MinMax { .. })));
    let root_html_intrinsic_auto_rows = profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        && !root_html_direct_text_aspect_ratio
        && !root_html_has_intrinsic_explicit_tracks(grid)
        && root_html_has_intrinsic_implicit_min_content_rows(grid);
    let root_html_column_flow = profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        && grid.auto_flow == ComputedCssGridAutoFlow::Column;
    let root_html_one_axis_placement =
        profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 && root_html_one_axis_placement;
    let mut auto_rows = if sparse_row
        || dense_auto_placement
        || root_html_implicit_rows
        || root_html_intrinsic_auto_rows
        || root_html_one_axis_placement
    {
        project_auto_rows(
            node,
            &grid.auto_rows,
            profile,
            allow_template_area_track_sizing,
            root_html_intrinsic_auto_rows,
            math,
            &mut reserve,
        )?
    } else {
        GridAutoRows::Auto
    };
    let mut auto_columns =
        if dense_auto_placement || root_html_column_flow || root_html_one_axis_placement {
            project_auto_columns(
                node,
                &grid.auto_columns,
                profile,
                allow_template_area_track_sizing,
                math,
                &mut reserve,
            )?
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
    let template_areas = if columns_auto_repeat.is_some()
        || rows_auto_repeat.is_some()
        || profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        || (profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            && grid.template_areas.is_some())
    {
        grid.template_areas
            .as_ref()
            .map(|areas| project_template_areas(node, areas, &mut reserve_line_metadata))
            .transpose()?
    } else {
        None
    };
    if profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        && let Some(areas) = &template_areas
    {
        let missing_columns = extend_template_area_tracks(
            TemplateAreaTrackExpansion {
                node,
                template_property: "grid-template-columns",
                auto_property: "grid-auto-columns",
                area_track_count: usize::from(areas.columns),
            },
            &auto_columns,
            &mut columns,
            &mut columns_line_names,
            &mut reserve,
            &mut reserve_line_metadata,
        )?;
        advance_template_area_auto_track_phase(&mut auto_columns, missing_columns);
        let missing_rows = extend_template_area_tracks(
            TemplateAreaTrackExpansion {
                node,
                template_property: "grid-template-rows",
                auto_property: "grid-auto-rows",
                area_track_count: usize::from(areas.rows),
            },
            &auto_rows,
            &mut rows,
            &mut rows_line_names,
            &mut reserve,
            &mut reserve_line_metadata,
        )?;
        advance_template_area_auto_track_phase(&mut auto_rows, missing_rows);
    }
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
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                if matches!(
                    grid.auto_flow,
                    ComputedCssGridAutoFlow::RowDense | ComputedCssGridAutoFlow::ColumnDense
                ) =>
            {
                GridPlacementMode::RootHtmlDenseAutoPlacementV1
            }
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                if !root_html_direct_text_aspect_ratio
                    && (root_html_has_intrinsic_explicit_tracks(grid)
                        || root_html_intrinsic_auto_rows) =>
            {
                GridPlacementMode::RootHtmlIntrinsicTrackSizingV1
            }
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                if grid.auto_flow == ComputedCssGridAutoFlow::Column =>
            {
                GridPlacementMode::RootHtmlColumnFlowV1
            }
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 if grid.template_areas.is_some() => {
                GridPlacementMode::TemplateGrid
            }
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                if root_html_has_fit_content_tracks(grid) =>
            {
                GridPlacementMode::RootHtmlFitContentV1
            }
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 if root_html_implicit_rows => {
                GridPlacementMode::RootHtmlImplicitRowsV1
            }
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
                GridPlacementMode::DenseAutoPlacement
            }
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
                GridPlacementMode::TrackSizing
            }
            _ => GridPlacementMode::Existing,
        },
        track_sizing_trace_enabled: root_html_intrinsic_auto_rows,
    })
}

fn root_html_has_fit_content_tracks(grid: &ComputedCssGridLayout) -> bool {
    grid.columns
        .iter()
        .chain(&grid.rows)
        .any(|track| matches!(track, ComputedCssGridTrackSize::FitContent(_)))
}

struct TemplateAreaTrackExpansion {
    node: NodeId,
    template_property: &'static str,
    auto_property: &'static str,
    area_track_count: usize,
}

fn extend_template_area_tracks(
    expansion: TemplateAreaTrackExpansion,
    auto_track: &GridAutoRows,
    tracks: &mut Vec<GridTrackSize>,
    line_names: &mut Vec<Vec<String>>,
    reserve_tracks: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
    mut reserve_line_metadata: impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<usize, StyleLayoutError> {
    let TemplateAreaTrackExpansion {
        node,
        template_property,
        auto_property,
        area_track_count,
    } = expansion;
    let missing_tracks = area_track_count.saturating_sub(tracks.len());
    if missing_tracks > 0 {
        match auto_track {
            GridAutoRows::Auto => {
                reserve_tracks(tracks, missing_tracks).map_err(|_| {
                    StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                        node,
                        property: auto_property,
                    })
                })?;
                tracks.extend(std::iter::repeat_n(GridTrackSize::Auto, missing_tracks));
            }
            GridAutoRows::LengthPx(value) => {
                reserve_tracks(tracks, missing_tracks).map_err(|_| {
                    StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                        node,
                        property: auto_property,
                    })
                })?;
                tracks.extend(std::iter::repeat_n(
                    GridTrackSize::LengthPx(*value),
                    missing_tracks,
                ));
            }
            GridAutoRows::TrackList(pattern) if !pattern.is_empty() => {
                reserve_tracks(tracks, missing_tracks).map_err(|_| {
                    StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                        node,
                        property: auto_property,
                    })
                })?;
                tracks.extend(pattern.iter().cycle().take(missing_tracks).cloned());
            }
            GridAutoRows::TrackList(_) => {
                return Err(StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: auto_property,
                    value: "area 확장에 빈 grid-auto-* 목록을 사용할 수 없습니다".to_owned(),
                });
            }
        }
    }

    let required_groups =
        tracks
            .len()
            .checked_add(1)
            .ok_or_else(|| StyleLayoutError::UnsupportedComputedValue {
                node,
                property: template_property,
                value: "grid-template-areas line-name metadata 크기가 범위를 넘었습니다".to_owned(),
            })?;
    let missing_groups = required_groups.saturating_sub(line_names.len());
    if missing_groups > 0 {
        reserve_line_metadata(GridLineNameReservationSite::Groups).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property: template_property,
            })
        })?;
        line_names.try_reserve_exact(missing_groups).map_err(|_| {
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                node,
                property: template_property,
            })
        })?;
        line_names.extend(std::iter::repeat_with(Vec::new).take(missing_groups));
    }
    Ok(missing_tracks)
}

fn advance_template_area_auto_track_phase(auto_track: &mut GridAutoRows, consumed_tracks: usize) {
    let GridAutoRows::TrackList(pattern) = auto_track else {
        return;
    };
    if pattern.is_empty() {
        return;
    }
    let phase = consumed_tracks % pattern.len();
    pattern.rotate_left(phase);
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

fn empty_grid_line_names(
    node: NodeId,
    property: &'static str,
    track_count: usize,
    reserve: &mut impl FnMut(GridLineNameReservationSite) -> Result<(), ()>,
) -> Result<Vec<Vec<String>>, StyleLayoutError> {
    let line_count =
        track_count
            .checked_add(1)
            .ok_or_else(|| StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "explicit Grid line 수가 지원 범위를 넘었습니다".to_owned(),
            })?;
    reserve(GridLineNameReservationSite::Groups).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    let mut lines = Vec::new();
    lines.try_reserve_exact(line_count).map_err(|_| {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property,
        })
    })?;
    lines.extend(std::iter::repeat_with(Vec::new).take(line_count));
    Ok(lines)
}

fn project_auto_columns(
    node: NodeId,
    auto_columns: &ComputedCssGridAutoColumns,
    profile: ComputedStyleProfile,
    allow_template_area_track_sizing: bool,
    math: &mut CssMathProjector,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridAutoColumns, StyleLayoutError> {
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => Ok(GridAutoColumns::Auto),
        ComputedCssGridAutoColumns::LengthPx(value) => Ok(GridAutoColumns::LengthPx(*value)),
        ComputedCssGridAutoColumns::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                && !tracks.is_empty()
                && (allow_template_area_track_sizing
                    || tracks
                        .iter()
                        .all(|track| matches!(track, ComputedCssGridTrackSize::LengthPx(_)))) =>
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
        ComputedCssGridAutoColumns::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                && !tracks.is_empty()
                && tracks.iter().all(|track| {
                    match track {
                        ComputedCssGridTrackSize::LengthPx(value)
                        | ComputedCssGridTrackSize::Percentage(value)
                        | ComputedCssGridTrackSize::Fraction(value) => {
                            value.is_finite() && *value >= 0.0
                        }
                        ComputedCssGridTrackSize::Calc(expression) => {
                            supported_root_html_auto_track_calc(expression)
                        }
                        _ => false,
                    }
                }) =>
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
                value: "track-size 목록은 C11.4 track-sizing 또는 area-backed C11.3.3 template profile에서만 지원됩니다".to_owned(),
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
    allow_template_area_track_sizing: bool,
    allow_root_html_intrinsic_min_content: bool,
    math: &mut CssMathProjector,
    reserve: &mut impl FnMut(&mut Vec<GridTrackSize>, usize) -> Result<(), ()>,
) -> Result<GridAutoRows, StyleLayoutError> {
    match auto_rows {
        ComputedCssGridAutoRows::Auto => Ok(GridAutoRows::Auto),
        ComputedCssGridAutoRows::LengthPx(value) => Ok(GridAutoRows::LengthPx(*value)),
        ComputedCssGridAutoRows::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                && !tracks.is_empty()
                && (allow_template_area_track_sizing
                    || tracks
                        .iter()
                        .all(|track| matches!(track, ComputedCssGridTrackSize::LengthPx(_)))) =>
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
        ComputedCssGridAutoRows::TrackList(tracks)
            if profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
                && (root_html_direct_auto_row_track_list(tracks)
                    || (allow_root_html_intrinsic_min_content
                        && tracks.as_slice() == [ComputedCssGridTrackSize::MinContent])) =>
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
            value: "track-size 목록은 C11.4 track-sizing 또는 area-backed C11.3.3 template profile에서만 지원됩니다".to_owned(),
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
#[path = "grid_values/root_html_grid_track_tests.rs"]
mod root_html_grid_track_tests;

#[cfg(test)]
#[path = "grid_values/auto_repeat_neighbor_tests.rs"]
mod auto_repeat_neighbor_tests;

#[cfg(test)]
#[path = "grid_values/auto_repeat_fixed_maximum_tests.rs"]
mod auto_repeat_fixed_maximum_tests;

#[cfg(test)]
#[path = "grid_values/auto_track_tests.rs"]
mod auto_track_tests;

#[cfg(test)]
#[path = "grid_values/template_area_track_list_tests.rs"]
mod template_area_track_list_tests;

#[cfg(test)]
#[path = "grid_values/combined_metadata_tests.rs"]
mod combined_metadata_tests;
