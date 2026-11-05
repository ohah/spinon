use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
};

use super::root_html_column_math::supported_root_html_auto_track_calc;
use crate::StyleLayoutError;

struct AxisConstraints {
    size_property: &'static str,
    min_property: &'static str,
    max_property: &'static str,
    size: ComputedCssDimension,
    min_size: ComputedCssDimension,
    max_size: ComputedCssMaxSize,
}

pub(super) fn validate_fraction_axis_inputs(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    let has_auto_minimum_columns = tracks_have_auto_minimum_fraction(&grid.columns);
    let has_auto_minimum_rows = tracks_have_auto_minimum_fraction(&grid.rows);
    let has_fraction_auto_columns = grid.auto_flow == ComputedCssGridAutoFlow::Column
        && matches!(
            &grid.auto_columns,
            ComputedCssGridAutoColumns::TrackList(tracks) if has_fraction_track(tracks)
        );
    let has_fraction_columns =
        has_explicit_fraction_track(&grid.columns) || has_fraction_auto_columns;
    if has_fraction_columns {
        validate_axis(
            element,
            AxisConstraints {
                size_property: "width",
                min_property: "min-width",
                max_property: "max-width",
                size: element.layout_dimensions.width,
                min_size: element.layout_dimensions.min_width,
                max_size: element.layout_dimensions.max_width,
            },
            "fr",
        )?;
        if has_auto_minimum_columns {
            validate_positive_axis_basis(element, "width", element.layout_dimensions.width)?;
        }
    }
    if has_explicit_fraction_track(&grid.rows) {
        validate_axis(
            element,
            AxisConstraints {
                size_property: "height",
                min_property: "min-height",
                max_property: "max-height",
                size: element.layout_dimensions.height,
                min_size: element.layout_dimensions.min_height,
                max_size: element.layout_dimensions.max_height,
            },
            "fr",
        )?;
        if has_auto_minimum_rows {
            validate_positive_axis_basis(element, "height", element.layout_dimensions.height)?;
        }
    }
    if (has_fraction_columns || has_explicit_fraction_track(&grid.rows))
        && (element.layout_aspect_ratio.is_some()
            || element
                .properties
                .get("aspect-ratio")
                .is_some_and(|value| value != "auto"))
    {
        return Err(unsupported(
            element,
            "aspect-ratio",
            "Root HTML Grid fr track에는 aspect-ratio를 함께 지정할 수 없습니다",
        ));
    }
    Ok(())
}

pub(super) fn has_fraction_maximum_minmax_track(grid: &ComputedCssGridLayout) -> bool {
    grid.columns.iter().chain(&grid.rows).any(|track| {
        matches!(
            track,
            ComputedCssGridTrackSize::MinMax {
                max: ComputedCssGridTrackBreadth::Fraction(value),
                ..
            } if value.is_finite() && *value >= 0.0
        )
    })
}

fn tracks_have_auto_minimum_fraction(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks.iter().any(|track| {
        matches!(
            track,
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::Fraction(value),
            } if value.is_finite() && *value >= 0.0
        )
    })
}

fn validate_positive_axis_basis(
    element: &ComputedElementStyle,
    property: &'static str,
    size: ComputedCssDimension,
) -> Result<(), StyleLayoutError> {
    if matches!(size, ComputedCssDimension::LengthPx(value) if value.is_finite() && value > 0.0) {
        return Ok(());
    }
    Err(unsupported(
        element,
        property,
        "Root HTML Grid auto-minimum flex track requires a definite positive CSS-pixel axis size",
    ))
}

pub(super) fn validate_percentage_axis_inputs(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    let has_percentage_auto_columns = grid.auto_flow == ComputedCssGridAutoFlow::Column
        && has_percentage_auto_columns(&grid.auto_columns);
    let has_percentage_auto_rows = match &grid.auto_rows {
        ComputedCssGridAutoRows::TrackList(tracks) => tracks.iter().any(|track| match track {
            ComputedCssGridTrackSize::Percentage(_) => true,
            ComputedCssGridTrackSize::Calc(expression) => {
                supported_root_html_auto_track_calc(expression)
            }
            _ => false,
        }),
        ComputedCssGridAutoRows::Auto
        | ComputedCssGridAutoRows::LengthPx(_)
        | ComputedCssGridAutoRows::Unsupported(_) => false,
    };
    if has_explicit_percentage_track(&grid.columns) || has_percentage_auto_columns {
        validate_axis(
            element,
            AxisConstraints {
                size_property: "width",
                min_property: "min-width",
                max_property: "max-width",
                size: element.layout_dimensions.width,
                min_size: element.layout_dimensions.min_width,
                max_size: element.layout_dimensions.max_width,
            },
            "percentage",
        )?;
    }
    if has_explicit_percentage_track(&grid.rows) {
        validate_axis(
            element,
            AxisConstraints {
                size_property: "height",
                min_property: "min-height",
                max_property: "max-height",
                size: element.layout_dimensions.height,
                min_size: element.layout_dimensions.min_height,
                max_size: element.layout_dimensions.max_height,
            },
            "percentage",
        )?;
    }
    if has_percentage_auto_rows {
        validate_axis(
            element,
            AxisConstraints {
                size_property: "height",
                min_property: "min-height",
                max_property: "max-height",
                size: element.layout_dimensions.height,
                min_size: element.layout_dimensions.min_height,
                max_size: element.layout_dimensions.max_height,
            },
            "percentage",
        )?;
    }
    if (has_explicit_percentage_track(&grid.columns)
        || has_percentage_auto_columns
        || has_explicit_percentage_track(&grid.rows)
        || has_percentage_auto_rows)
        && (element.layout_aspect_ratio.is_some()
            || element
                .properties
                .get("aspect-ratio")
                .is_some_and(|value| value != "auto"))
    {
        return Err(unsupported(
            element,
            "aspect-ratio",
            "Root HTML Grid percentage track에는 aspect-ratio를 함께 지정할 수 없습니다",
        ));
    }
    Ok(())
}

pub(super) fn validate_percentage_track_ranges(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    validate_axis_track_range(
        element,
        "grid-template-columns",
        "column-gap",
        &grid.columns,
        element.layout_dimensions.width,
        element.layout_spacing.column_gap,
    )?;
    validate_axis_track_range(
        element,
        "grid-template-rows",
        "row-gap",
        &grid.rows,
        element.layout_dimensions.height,
        element.layout_spacing.row_gap,
    )?;
    validate_auto_row_percentage_ranges(element, &grid.auto_rows)
        .and_then(|()| validate_auto_column_percentage_ranges(element, &grid.auto_columns))
}

fn validate_auto_row_percentage_ranges(
    element: &ComputedElementStyle,
    auto_rows: &ComputedCssGridAutoRows,
) -> Result<(), StyleLayoutError> {
    let ComputedCssGridAutoRows::TrackList(tracks) = auto_rows else {
        return Ok(());
    };
    if !has_percentage_track(tracks) {
        return Ok(());
    }
    let ComputedCssDimension::LengthPx(basis) = element.layout_dimensions.height else {
        return Err(unsupported(
            element,
            "height",
            "Root HTML Grid percentage auto rows require a definite finite non-negative CSS-pixel height",
        ));
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(unsupported(
            element,
            "height",
            "Root HTML Grid percentage auto rows require a definite finite non-negative CSS-pixel height",
        ));
    }
    for track in tracks {
        let ComputedCssGridTrackSize::Percentage(value) = track else {
            continue;
        };
        let resolved = (*value as f64) * (basis as f64);
        if !value.is_finite() || *value < 0.0 || !resolved.is_finite() || resolved > f32::MAX as f64
        {
            return Err(unsupported(
                element,
                "grid-auto-rows",
                "resolved percentage auto row exceeds the finite f32 CSS-pixel range",
            ));
        }
    }
    Ok(())
}

fn validate_auto_column_percentage_ranges(
    element: &ComputedElementStyle,
    auto_columns: &ComputedCssGridAutoColumns,
) -> Result<(), StyleLayoutError> {
    let ComputedCssGridAutoColumns::TrackList(tracks) = auto_columns else {
        return Ok(());
    };
    if !has_percentage_track(tracks) {
        return Ok(());
    }
    let ComputedCssDimension::LengthPx(basis) = element.layout_dimensions.width else {
        return Err(unsupported(
            element,
            "width",
            "Root HTML Grid percentage auto columns require a definite finite non-negative CSS-pixel width",
        ));
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(unsupported(
            element,
            "width",
            "Root HTML Grid percentage auto columns require a definite finite non-negative CSS-pixel width",
        ));
    }
    for track in tracks {
        let ComputedCssGridTrackSize::Percentage(value) = track else {
            continue;
        };
        let resolved = (*value as f64) * (basis as f64);
        if !value.is_finite() || *value < 0.0 || !resolved.is_finite() || resolved > f32::MAX as f64
        {
            return Err(unsupported(
                element,
                "grid-auto-columns",
                "resolved percentage auto column exceeds the finite f32 CSS-pixel range",
            ));
        }
    }
    Ok(())
}

fn validate_axis_track_range(
    element: &ComputedElementStyle,
    property: &'static str,
    gap_property: &'static str,
    tracks: &[ComputedCssGridTrackSize],
    basis: ComputedCssDimension,
    gap: ComputedCssSpacingValue,
) -> Result<(), StyleLayoutError> {
    let has_percentage_gap = matches!(gap, ComputedCssSpacingValue::Percentage(_));
    let has_percentage_track = has_explicit_percentage_track(tracks);
    let has_minmax_track = tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::MinMax { .. }));
    if !has_percentage_track && !has_percentage_gap && !has_minmax_track {
        return Ok(());
    }

    let basis = if has_percentage_track || has_percentage_gap {
        let ComputedCssDimension::LengthPx(basis) = basis else {
            return Err(unsupported(
                element,
                if property == "grid-template-columns" {
                    "width"
                } else {
                    "height"
                },
                "Root HTML Grid percentage track requires a definite finite non-negative CSS-pixel size",
            ));
        };
        if !basis.is_finite() || basis < 0.0 {
            return Err(unsupported(
                element,
                if property == "grid-template-columns" {
                    "width"
                } else {
                    "height"
                },
                "Root HTML Grid percentage track requires a definite finite non-negative CSS-pixel size",
            ));
        }
        basis
    } else {
        match basis {
            ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
            _ => 0.0,
        }
    };

    let gap = match gap {
        ComputedCssSpacingValue::Normal => 0.0,
        ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
        ComputedCssSpacingValue::Percentage(value) if value.is_finite() && value >= 0.0 => {
            let resolved = f64::from(value) * f64::from(basis);
            if !resolved.is_finite() || resolved > f64::from(f32::MAX) {
                return Err(unsupported(
                    element,
                    gap_property,
                    "resolved percentage gap exceeds the finite f32 CSS-pixel range",
                ));
            }
            resolved as f32
        }
        _ => {
            return Err(unsupported(
                element,
                gap_property,
                "Root HTML Grid percentage tracks require a finite non-negative px or percentage gap",
            ));
        }
    };

    let max_f32 = f32::MAX as f64;
    let mut fixed_total = (gap as f64) * tracks.len().saturating_sub(1) as f64;
    let mut fraction_factor_total = 0.0_f64;
    if !fixed_total.is_finite() || fixed_total > max_f32 {
        return Err(unsupported(
            element,
            property,
            "track gutters exceed the finite f32 CSS-pixel range",
        ));
    }
    for track in tracks {
        match track {
            ComputedCssGridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0 => {
                fixed_total += *value as f64;
            }
            ComputedCssGridTrackSize::Percentage(value) if value.is_finite() && *value >= 0.0 => {
                let resolved = (*value as f64) * (basis as f64);
                if !resolved.is_finite() || resolved > max_f32 {
                    return Err(unsupported(
                        element,
                        property,
                        "resolved percentage track exceeds the finite f32 CSS-pixel range",
                    ));
                }
                fixed_total += resolved;
            }
            ComputedCssGridTrackSize::Fraction(value) if value.is_finite() && *value >= 0.0 => {
                fraction_factor_total += f64::from(*value);
            }
            ComputedCssGridTrackSize::MinMax { min, max } => {
                let minimum = match min {
                    ComputedCssGridTrackBreadth::Auto
                    | ComputedCssGridTrackBreadth::MinContent
                    | ComputedCssGridTrackBreadth::MaxContent => None,
                    _ => match resolve_fixed_breadth(min, basis) {
                        Some(minimum) => {
                            if !minimum.is_finite() || minimum < 0.0 || minimum > max_f32 {
                                return Err(unsupported(
                                    element,
                                    property,
                                    "resolved minmax minimum exceeds the finite f32 CSS-pixel range",
                                ));
                            }
                            Some(minimum)
                        }
                        // Intrinsic contributions are data-dependent; keep scanning known
                        // fixed/percentage tracks so a later aggregate overflow is not hidden.
                        None => None,
                    },
                };
                let resolved = match max {
                    ComputedCssGridTrackBreadth::Fraction(value)
                        if value.is_finite() && *value >= 0.0 =>
                    {
                        fraction_factor_total += f64::from(*value);
                        minimum.unwrap_or(0.0)
                    }
                    _ => {
                        match resolve_fixed_breadth(max, basis) {
                            Some(maximum) => {
                                if !maximum.is_finite() || maximum < 0.0 || maximum > max_f32 {
                                    return Err(unsupported(
                                        element,
                                        property,
                                        "resolved minmax maximum exceeds the finite f32 CSS-pixel range",
                                    ));
                                }
                                minimum.unwrap_or(0.0).max(maximum)
                            }
                            None => {
                                // Keep a known fixed minimum in the aggregate, then validate the
                                // remaining tracks. Intrinsic maxima are resolved by layout.
                                minimum.unwrap_or(0.0)
                            }
                        }
                    }
                };
                fixed_total += resolved;
            }
            ComputedCssGridTrackSize::Percentage(_) => {
                return Err(unsupported(
                    element,
                    property,
                    "Root HTML Grid percentage tracks must be finite and non-negative",
                ));
            }
            ComputedCssGridTrackSize::LengthPx(_) | ComputedCssGridTrackSize::Fraction(_) => {
                // The explicit-track validator reports the original px/fr input diagnostic.
                continue;
            }
            _ => {
                // Intrinsic and unsupported forms contribute no known fixed amount here. The
                // explicit-track validator still reports unsupported forms before layout.
                continue;
            }
        }
        if !fixed_total.is_finite() || fixed_total > max_f32 {
            return Err(unsupported(
                element,
                property,
                "percentage tracks, fixed tracks, and gaps exceed the finite f32 CSS-pixel range",
            ));
        }
        if !fraction_factor_total.is_finite() || fraction_factor_total > max_f32 {
            return Err(unsupported(
                element,
                property,
                "grid fraction factors exceed the finite f32 range",
            ));
        }
    }
    Ok(())
}

fn has_fraction_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::Fraction(_)))
}

fn has_explicit_fraction_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks.iter().any(|track| {
        matches!(
            track,
            ComputedCssGridTrackSize::Fraction(_)
                | ComputedCssGridTrackSize::MinMax {
                    max: ComputedCssGridTrackBreadth::Fraction(_),
                    ..
                }
        )
    })
}

fn has_percentage_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::Percentage(_)))
}

fn has_explicit_percentage_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks.iter().any(|track| match track {
        ComputedCssGridTrackSize::Percentage(_) => true,
        ComputedCssGridTrackSize::MinMax { min, max } => {
            matches!(min, ComputedCssGridTrackBreadth::Percentage(_))
                || matches!(max, ComputedCssGridTrackBreadth::Percentage(_))
        }
        _ => false,
    })
}

fn resolve_fixed_breadth(breadth: &ComputedCssGridTrackBreadth, basis: f32) -> Option<f64> {
    match breadth {
        ComputedCssGridTrackBreadth::LengthPx(value) => Some(f64::from(*value)),
        ComputedCssGridTrackBreadth::Percentage(value) => {
            Some(f64::from(*value) * f64::from(basis))
        }
        _ => None,
    }
}

fn has_percentage_auto_columns(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    matches!(
        auto_columns,
        ComputedCssGridAutoColumns::TrackList(tracks) if has_percentage_track(tracks)
    )
}

fn validate_axis(
    element: &ComputedElementStyle,
    constraints: AxisConstraints,
    track_kind: &str,
) -> Result<(), StyleLayoutError> {
    let AxisConstraints {
        size_property,
        min_property,
        max_property,
        size,
        min_size,
        max_size,
    } = constraints;
    if element
        .properties
        .get(size_property)
        .is_some_and(|value| value == "auto")
        || element.layout_math_values.contains_key(size_property)
        || !matches!(size, ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0)
    {
        return Err(unsupported(
            element,
            size_property,
            &format!(
                "Root HTML Grid {track_kind} track requires a definite finite non-negative CSS-pixel size"
            ),
        ));
    }
    if min_size != ComputedCssDimension::Auto {
        return Err(unsupported(
            element,
            min_property,
            &format!(
                "Root HTML Grid {track_kind} track requires the corresponding computed min-size to be auto"
            ),
        ));
    }
    if max_size != ComputedCssMaxSize::None {
        return Err(unsupported(
            element,
            max_property,
            &format!(
                "Root HTML Grid {track_kind} track requires the corresponding computed max-size to be none"
            ),
        ));
    }
    Ok(())
}

pub(super) fn unsupported(
    element: &ComputedElementStyle,
    property: &'static str,
    explanation: &str,
) -> StyleLayoutError {
    let value = element
        .properties
        .get(property)
        .cloned()
        .unwrap_or_else(|| "<computed value unavailable>".to_owned());
    StyleLayoutError::UnsupportedComputedValue {
        node: element.node_id,
        property,
        value: format!("{explanation}: {value}"),
    }
}
