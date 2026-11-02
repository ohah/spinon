use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridLayout,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
};

use crate::StyleLayoutError;

use super::root_html_grid_tracks::unsupported;

pub(super) fn validate_root_html_gap(
    element: &ComputedElementStyle,
    property: &'static str,
    gap: ComputedCssSpacingValue,
    basis: ComputedCssDimension,
) -> Result<(), StyleLayoutError> {
    if resolved_root_html_gap(gap, basis).is_some() {
        if !matches!(gap, ComputedCssSpacingValue::Percentage(_)) {
            return Ok(());
        }
        let (size_property, min_property, max_property, min_size, max_size) =
            if property == "row-gap" {
                (
                    "height",
                    "min-height",
                    "max-height",
                    element.layout_dimensions.min_height,
                    element.layout_dimensions.max_height,
                )
            } else {
                (
                    "width",
                    "min-width",
                    "max-width",
                    element.layout_dimensions.min_width,
                    element.layout_dimensions.max_width,
                )
            };
        if element.layout_math_values.contains_key(size_property)
            || element
                .properties
                .get(size_property)
                .is_some_and(|value| value == "auto")
        {
            return Err(unsupported(
                element,
                size_property,
                "percentage Grid gap requires the corresponding computed axis size to be definite",
            ));
        }
        if min_size != ComputedCssDimension::Auto {
            return Err(unsupported(
                element,
                min_property,
                "percentage Grid gap requires the corresponding min-size to be auto",
            ));
        }
        if max_size != ComputedCssMaxSize::None {
            return Err(unsupported(
                element,
                max_property,
                "percentage Grid gap requires the corresponding max-size to be none",
            ));
        }
        return Ok(());
    }
    Err(unsupported(
        element,
        property,
        "Root HTML Grid gap requires normal, finite non-negative px, or a finite non-negative percentage with a definite finite non-negative CSS-pixel axis size",
    ))
}

pub(super) fn resolved_root_html_gap(
    gap: ComputedCssSpacingValue,
    basis: ComputedCssDimension,
) -> Option<f32> {
    match gap {
        ComputedCssSpacingValue::Normal => Some(0.0),
        ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0 => {
            Some(value)
        }
        ComputedCssSpacingValue::Percentage(value) if value.is_finite() && value >= 0.0 => {
            let ComputedCssDimension::LengthPx(basis) = basis else {
                return None;
            };
            if !basis.is_finite() || basis < 0.0 {
                return None;
            }
            let resolved = f64::from(value) * f64::from(basis);
            (resolved.is_finite() && resolved <= f64::from(f32::MAX)).then_some(resolved as f32)
        }
        ComputedCssSpacingValue::LengthPx(_)
        | ComputedCssSpacingValue::Percentage(_)
        | ComputedCssSpacingValue::Auto
        | ComputedCssSpacingValue::Unsupported => None,
    }
}

pub(super) fn validate_implicit_column_relative_extent(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
    required_columns: usize,
) -> Result<(), StyleLayoutError> {
    let has_relative_auto_columns = matches!(
        &grid.auto_columns,
        ComputedCssGridAutoColumns::TrackList(tracks)
            if tracks.iter().any(|track| matches!(track, ComputedCssGridTrackSize::Percentage(_) | ComputedCssGridTrackSize::Fraction(_)))
    );
    if !has_relative_auto_columns || required_columns <= grid.columns.len() {
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
    let ComputedCssGridAutoColumns::TrackList(tracks) = &grid.auto_columns else {
        return Ok(());
    };
    let gap = resolved_root_html_gap(
        element.layout_spacing.column_gap,
        element.layout_dimensions.width,
    )
    .ok_or_else(|| {
        unsupported(
            element,
            "column-gap",
            "relative implicit columns require a finite non-negative gap resolvable against the definite width",
        )
    })?;
    let mut total = f64::from(gap) * required_columns.saturating_sub(1) as f64;
    for track in &grid.columns {
        let ComputedCssGridTrackSize::LengthPx(value) = track else {
            return Ok(());
        };
        total += f64::from(*value);
    }
    if !total.is_finite() || total > f64::from(f32::MAX) {
        return Err(unsupported(
            element,
            "grid-auto-columns",
            "explicit tracks and gaps exceed the finite f32 CSS-pixel extent",
        ));
    }
    for index in 0..required_columns - grid.columns.len() {
        let value = match tracks[index % tracks.len()] {
            ComputedCssGridTrackSize::LengthPx(value) => f64::from(value),
            ComputedCssGridTrackSize::Percentage(value) => f64::from(value) * f64::from(basis),
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
        if !total.is_finite() || total > f64::from(f32::MAX) {
            return Err(unsupported(
                element,
                "grid-auto-columns",
                "relative implicit columns exceed the finite f32 CSS-pixel extent",
            ));
        }
    }
    Ok(())
}
