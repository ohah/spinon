use spinon_core::NodeId;
use spinon_layout::GRID_MAX_TRACKS;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
};

use crate::StyleLayoutError;

const AUTO_REPEAT_COUNT_MIN_TRACK_SIZE: f32 = 1.0;

#[derive(Clone, Copy)]
enum Axis {
    Columns,
    Rows,
}

impl Axis {
    fn property(self) -> &'static str {
        match self {
            Self::Columns => "grid-template-columns",
            Self::Rows => "grid-template-rows",
        }
    }

    fn size_property(self) -> &'static str {
        match self {
            Self::Columns => "width",
            Self::Rows => "height",
        }
    }

    fn min_property(self) -> &'static str {
        match self {
            Self::Columns => "min-width",
            Self::Rows => "min-height",
        }
    }

    fn max_property(self) -> &'static str {
        match self {
            Self::Columns => "max-width",
            Self::Rows => "max-height",
        }
    }
}

/// Validates the narrow Root HTML auto-repeat grammar before the generic track projection runs.
/// `true` means that exactly one complete track list is an eligible auto-repeat candidate.
pub(super) fn validate_grid_grammar(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    root_html_one_axis_placement: bool,
    root_html_direct_text_aspect_ratio: bool,
) -> Result<bool, StyleLayoutError> {
    if grid.columns_auto_repeat.is_some() && grid.rows_auto_repeat.is_some() {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "grid-template-columns",
            value: "Root HTML Grid auto-repeat은 한 축에서만 지원합니다".to_owned(),
        });
    }
    let Some(axis) = repeat_axis(grid) else {
        return Ok(false);
    };
    let failure = |value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property: axis.property(),
        value: value.to_owned(),
    };

    if root_html_one_axis_placement || root_html_direct_text_aspect_ratio {
        return Err(failure(
            "Root HTML auto-repeat은 one-axis placement 및 direct-text aspect-ratio route와 결합할 수 없습니다",
        ));
    }
    if grid.columns_fixed_repeat_group_count != 0
        || grid.rows_fixed_repeat_group_count != 0
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto)
        || !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
    {
        return Err(failure(
            "Root HTML auto-repeat은 integer repeat, template areas, column/dense flow 및 implicit tracks와 결합할 수 없습니다",
        ));
    }

    let (tracks, repeat, other_tracks, other_line_names) = match axis {
        Axis::Columns => (
            grid.columns.as_slice(),
            grid.columns_auto_repeat
                .as_ref()
                .expect("repeat axis exists"),
            grid.rows.as_slice(),
            grid.rows_line_names.as_slice(),
        ),
        Axis::Rows => (
            grid.rows.as_slice(),
            grid.rows_auto_repeat.as_ref().expect("repeat axis exists"),
            grid.columns.as_slice(),
            grid.columns_line_names.as_slice(),
        ),
    };
    let pattern_len = repeat.end.checked_sub(repeat.start);
    let pattern = pattern_len.and_then(|_| tracks.get(repeat.start..repeat.end));
    if repeat.start != 0
        || repeat.end != tracks.len()
        || pattern.is_none_or(|pattern| !(1..=2).contains(&pattern.len()))
        || !line_names_are_unnamed(&repeat.line_names)
        || !repeat.fixed_repeat_minmax_track_indices.is_empty()
        || !line_names_are_unnamed(match axis {
            Axis::Columns => grid.columns_line_names.as_slice(),
            Axis::Rows => grid.rows_line_names.as_slice(),
        })
        || other_tracks.len() != 1
        || !matches!(other_tracks.first(), Some(ComputedCssGridTrackSize::LengthPx(value)) if value.is_finite() && *value >= 0.0)
        || !line_names_are_unnamed(other_line_names)
        || !pattern.is_some_and(|pattern| pattern.iter().all(is_supported_repeat_track))
    {
        return Err(failure(
            "Root HTML auto-repeat은 전체 track list의 1~2개 minmax(non-negative px, 1fr) track만 지원하며 다른 축에는 이름 없는 fixed px track 하나만 허용합니다",
        ));
    }

    Ok(true)
}

/// Validates the computed container inputs used by both repeat-count calculation and Taffy's sizing.
pub(crate) fn validate_container(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    let Some(axis) = repeat_axis(grid) else {
        return Ok(());
    };
    let node = element.node_id;
    let failure = |property, value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    };
    let (size, min_size, max_size) = match axis {
        Axis::Columns => (
            element.layout_dimensions.width,
            element.layout_dimensions.min_width,
            element.layout_dimensions.max_width,
        ),
        Axis::Rows => (
            element.layout_dimensions.height,
            element.layout_dimensions.min_height,
            element.layout_dimensions.max_height,
        ),
    };
    if !matches!(size, ComputedCssDimension::LengthPx(value) if value.is_finite() && value > 0.0)
        || element
            .layout_math_values
            .contains_key(axis.size_property())
    {
        return Err(failure(
            axis.size_property(),
            "Root HTML auto-repeat requires a definite positive finite CSS-pixel axis size",
        ));
    }
    if min_size != ComputedCssDimension::Auto {
        return Err(failure(
            axis.min_property(),
            "Root HTML auto-repeat requires min-size:auto on its repeated axis",
        ));
    }
    if max_size != ComputedCssMaxSize::None {
        return Err(failure(
            axis.max_property(),
            "Root HTML auto-repeat requires max-size:none on its repeated axis",
        ));
    }
    if element.layout_aspect_ratio.is_some()
        || element
            .properties
            .get("aspect-ratio")
            .is_some_and(|value| value != "auto")
    {
        return Err(failure(
            "aspect-ratio",
            "Root HTML auto-repeat requires aspect-ratio:auto",
        ));
    }

    for (property, value) in [
        ("margin-top", element.layout_spacing.margin.top),
        ("margin-right", element.layout_spacing.margin.right),
        ("margin-bottom", element.layout_spacing.margin.bottom),
        ("margin-left", element.layout_spacing.margin.left),
        ("padding-top", element.layout_spacing.padding.top),
        ("padding-right", element.layout_spacing.padding.right),
        ("padding-bottom", element.layout_spacing.padding.bottom),
        ("padding-left", element.layout_spacing.padding.left),
    ] {
        if !is_zero_px(value) || element.layout_math_values.contains_key(property) {
            return Err(failure(
                property,
                "Root HTML auto-repeat requires zero margins and padding",
            ));
        }
    }
    for (property, value) in [
        ("border-top-width", element.layout_border.top),
        ("border-right-width", element.layout_border.right),
        ("border-bottom-width", element.layout_border.bottom),
        ("border-left-width", element.layout_border.left),
    ] {
        if value != 0.0 {
            return Err(failure(
                property,
                "Root HTML auto-repeat requires zero border widths",
            ));
        }
    }
    for (property, value) in [
        ("column-gap", element.layout_spacing.column_gap),
        ("row-gap", element.layout_spacing.row_gap),
    ] {
        if !is_supported_gap(value) || element.layout_math_values.contains_key(property) {
            return Err(failure(
                property,
                "Root HTML auto-repeat supports only normal or finite non-negative px gaps",
            ));
        }
    }
    Ok(())
}

/// Returns the explicit track counts after applying the same auto-repeat count rule as vendored Taffy.
pub(crate) fn expanded_track_counts(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(usize, usize), StyleLayoutError> {
    if !validate_grid_grammar(element.node_id, grid, false, false)? {
        return Ok((grid.columns.len(), grid.rows.len()));
    }
    validate_container(element, grid)?;
    let axis = repeat_axis(grid).expect("validated auto-repeat has an axis");
    let count = expanded_repeat_count(element, grid, axis)?;
    Ok(match axis {
        Axis::Columns => (count, grid.rows.len()),
        Axis::Rows => (grid.columns.len(), count),
    })
}

fn expanded_repeat_count(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
    axis: Axis,
) -> Result<usize, StyleLayoutError> {
    let node = element.node_id;
    let failure = |value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property: axis.property(),
        value: value.to_owned(),
    };
    let (tracks, repeat, axis_size, gap) = match axis {
        Axis::Columns => (
            grid.columns.as_slice(),
            grid.columns_auto_repeat
                .as_ref()
                .expect("repeat axis exists"),
            element.layout_dimensions.width,
            element.layout_spacing.column_gap,
        ),
        Axis::Rows => (
            grid.rows.as_slice(),
            grid.rows_auto_repeat.as_ref().expect("repeat axis exists"),
            element.layout_dimensions.height,
            element.layout_spacing.row_gap,
        ),
    };
    let ComputedCssDimension::LengthPx(axis_size) = axis_size else {
        return Err(failure(
            "auto-repeat axis size is not a definite CSS-pixel value",
        ));
    };
    let gap = match gap {
        ComputedCssSpacingValue::Normal => 0.0,
        ComputedCssSpacingValue::LengthPx(value) => value,
        _ => return Err(failure("auto-repeat gap is not a fixed CSS-pixel value")),
    };
    let pattern = tracks
        .get(repeat.start..repeat.end)
        .ok_or_else(|| failure("auto-repeat track range is invalid"))?;
    let mut minima = [0.0_f32; 2];
    for (index, track) in pattern.iter().enumerate() {
        let ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(minimum),
            ..
        } = track
        else {
            return Err(failure(
                "auto-repeat minimum is not a fixed CSS-pixel value",
            ));
        };
        minima[index] = *minimum;
    }
    let Some(expanded) = compute_expanded_track_count(axis_size, gap, &minima[..pattern.len()])
    else {
        return Err(failure(
            "auto-repeat count exceeds GRID_MAX_TRACKS or its arithmetic overflowed",
        ));
    };
    Ok(expanded)
}

fn compute_expanded_track_count(axis_size: f32, gap: f32, minima: &[f32]) -> Option<usize> {
    if minima.is_empty()
        || minima.len() > 2
        || !axis_size.is_finite()
        || axis_size <= 0.0
        || !gap.is_finite()
        || gap < 0.0
    {
        return None;
    }
    let mut pattern_space = 0.0_f64;
    let mut taffy_pattern_space = 0.0_f32;
    for minimum in minima {
        if !minimum.is_finite() || *minimum < 0.0 {
            return None;
        }
        let count_size = minimum.max(AUTO_REPEAT_COUNT_MIN_TRACK_SIZE);
        pattern_space += f64::from(count_size);
        taffy_pattern_space += count_size;
        if !pattern_space.is_finite() || !taffy_pattern_space.is_finite() {
            return None;
        }
    }
    let taffy_first_repeat_space = taffy_pattern_space + ((minima.len() - 1) as f32 * gap);
    let taffy_per_repeat_space = taffy_pattern_space + (minima.len() as f32 * gap);
    if !taffy_first_repeat_space.is_finite() || !taffy_per_repeat_space.is_finite() {
        return None;
    }
    let pattern_track_count = minima.len() as f64;
    let axis_size = f64::from(axis_size);
    let gap = f64::from(gap);
    let denominator = pattern_space + (pattern_track_count * gap);
    if !denominator.is_finite() || denominator <= 0.0 {
        return None;
    }

    let count = ((axis_size + gap) / denominator).floor().max(1.0);
    let maximum_repetitions = GRID_MAX_TRACKS / minima.len();
    if !count.is_finite() || count > maximum_repetitions as f64 {
        return None;
    }
    let repetitions = count as usize;
    repetitions
        .checked_mul(minima.len())
        .filter(|count| *count <= GRID_MAX_TRACKS)
}

fn repeat_axis(grid: &ComputedCssGridLayout) -> Option<Axis> {
    match (
        grid.columns_auto_repeat.is_some(),
        grid.rows_auto_repeat.is_some(),
    ) {
        (true, false) => Some(Axis::Columns),
        (false, true) => Some(Axis::Rows),
        _ => None,
    }
}

fn is_supported_repeat_track(track: &ComputedCssGridTrackSize) -> bool {
    matches!(
        track,
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(minimum),
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        } if minimum.is_finite() && *minimum >= 0.0
    )
}

fn line_names_are_unnamed(groups: &[Vec<String>]) -> bool {
    groups.iter().all(Vec::is_empty)
}

fn is_zero_px(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0)
}

fn is_supported_gap(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::Normal)
        || matches!(value, ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0)
}

#[cfg(test)]
#[path = "root_html_auto_repeat_tests.rs"]
mod tests;
