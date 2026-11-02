use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTrackSize, ComputedCssMaxSize,
    ComputedCssSpacingValue, ComputedElementStyle,
};

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
    let has_fraction_auto_columns = grid.auto_flow == ComputedCssGridAutoFlow::Column
        && matches!(
            &grid.auto_columns,
            ComputedCssGridAutoColumns::TrackList(tracks) if has_fraction_track(tracks)
        );
    let has_fraction_columns = has_fraction_track(&grid.columns) || has_fraction_auto_columns;
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
    }
    if has_fraction_track(&grid.rows) {
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
    }
    if (has_fraction_columns || has_fraction_track(&grid.rows))
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

pub(super) fn validate_percentage_axis_inputs(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    let has_percentage_auto_columns = grid.auto_flow == ComputedCssGridAutoFlow::Column
        && has_percentage_auto_columns(&grid.auto_columns);
    let has_percentage_auto_rows = match &grid.auto_rows {
        ComputedCssGridAutoRows::TrackList(tracks) => has_percentage_track(tracks),
        ComputedCssGridAutoRows::Auto
        | ComputedCssGridAutoRows::LengthPx(_)
        | ComputedCssGridAutoRows::Unsupported(_) => false,
    };
    if has_percentage_track(&grid.columns) || has_percentage_auto_columns {
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
    if has_percentage_track(&grid.rows) {
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
    if (has_percentage_track(&grid.columns)
        || has_percentage_auto_columns
        || has_percentage_track(&grid.rows)
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

pub(super) fn validate_implicit_column_relative_extent(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
    required_columns: usize,
) -> Result<(), StyleLayoutError> {
    if !has_relative_auto_columns(&grid.auto_columns) || required_columns <= grid.columns.len() {
        return Ok(());
    }
    let ComputedCssDimension::LengthPx(basis) = element.layout_dimensions.width else {
        return Err(unsupported(
            element,
            "width",
            "Root HTML Grid percentage/fraction auto columns require a definite finite non-negative CSS-pixel width",
        ));
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(unsupported(
            element,
            "width",
            "Root HTML Grid percentage/fraction auto columns require a definite finite non-negative CSS-pixel width",
        ));
    }
    let tracks = match &grid.auto_columns {
        ComputedCssGridAutoColumns::TrackList(tracks) => tracks.as_slice(),
        _ => return Ok(()),
    };
    let gap = match element.layout_spacing.column_gap {
        ComputedCssSpacingValue::Normal => 0.0,
        ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
        _ => {
            return Err(unsupported(
                element,
                "column-gap",
                "relative implicit columns require the existing finite non-negative fixed-px gap subset",
            ));
        }
    };
    let mut total = (gap as f64) * required_columns.saturating_sub(1) as f64;
    for track in &grid.columns {
        let ComputedCssGridTrackSize::LengthPx(value) = track else {
            return Ok(());
        };
        total += *value as f64;
    }
    if !total.is_finite() || total > f32::MAX as f64 {
        return Err(unsupported(
            element,
            "grid-auto-columns",
            "explicit tracks and gaps exceed the finite f32 CSS-pixel extent",
        ));
    }
    for index in 0..required_columns - grid.columns.len() {
        let value = match tracks[index % tracks.len()] {
            ComputedCssGridTrackSize::LengthPx(value) => value as f64,
            ComputedCssGridTrackSize::Percentage(value) => (value as f64) * (basis as f64),
            ComputedCssGridTrackSize::Fraction(value) if value.is_finite() && value >= 0.0 => {
                continue;
            }
            ComputedCssGridTrackSize::Fraction(_) => {
                return Err(unsupported(
                    element,
                    "grid-auto-columns",
                    "fraction implicit columns must be finite and non-negative",
                ));
            }
            _ => return Ok(()),
        };
        if !value.is_finite() || value < 0.0 {
            return Err(unsupported(
                element,
                "grid-auto-columns",
                "resolved relative implicit-column extent must remain finite and non-negative",
            ));
        }
        total += value;
        if !total.is_finite() || total > f32::MAX as f64 {
            return Err(unsupported(
                element,
                "grid-auto-columns",
                "relative implicit columns exceed the finite f32 CSS-pixel extent",
            ));
        }
    }
    Ok(())
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
    if !has_percentage_track(tracks) {
        return Ok(());
    }

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

    let gap = match gap {
        ComputedCssSpacingValue::Normal => 0.0,
        ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
        _ => {
            return Err(unsupported(
                element,
                gap_property,
                "Root HTML Grid percentage tracks require the existing finite non-negative fixed-px gap subset",
            ));
        }
    };

    let max_f32 = f32::MAX as f64;
    let mut fixed_total = (gap as f64) * tracks.len().saturating_sub(1) as f64;
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
            ComputedCssGridTrackSize::Fraction(value) if value.is_finite() && *value >= 0.0 => {}
            ComputedCssGridTrackSize::Percentage(_) => {
                return Err(unsupported(
                    element,
                    property,
                    "Root HTML Grid percentage tracks must be finite and non-negative",
                ));
            }
            ComputedCssGridTrackSize::LengthPx(_) | ComputedCssGridTrackSize::Fraction(_) => {
                // The explicit-track validator reports the original px/fr input diagnostic.
                return Ok(());
            }
            _ => {
                // The explicit-track validator reports unsupported track forms before layout.
                return Ok(());
            }
        }
        if !fixed_total.is_finite() || fixed_total > max_f32 {
            return Err(unsupported(
                element,
                property,
                "percentage tracks, fixed tracks, and gaps exceed the finite f32 CSS-pixel range",
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

fn has_percentage_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::Percentage(_)))
}

fn has_percentage_auto_columns(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    matches!(
        auto_columns,
        ComputedCssGridAutoColumns::TrackList(tracks) if has_percentage_track(tracks)
    )
}

fn has_relative_auto_columns(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    matches!(
        auto_columns,
        ComputedCssGridAutoColumns::TrackList(tracks)
            if tracks.iter().any(|track| matches!(track, ComputedCssGridTrackSize::Percentage(_) | ComputedCssGridTrackSize::Fraction(_)))
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

fn unsupported(
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
