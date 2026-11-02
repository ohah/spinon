use spinon_style::{
    ComputedCssDimension, ComputedCssGridLayout, ComputedCssGridTrackSize, ComputedCssMaxSize,
    ComputedElementStyle,
};

use crate::StyleLayoutError;

pub(super) fn validate_fraction_axis_inputs(
    element: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
) -> Result<(), StyleLayoutError> {
    if has_fraction_track(&grid.columns) {
        validate_axis(
            element,
            "width",
            "min-width",
            "max-width",
            element.layout_dimensions.width,
            element.layout_dimensions.min_width,
            element.layout_dimensions.max_width,
        )?;
    }
    if has_fraction_track(&grid.rows) {
        validate_axis(
            element,
            "height",
            "min-height",
            "max-height",
            element.layout_dimensions.height,
            element.layout_dimensions.min_height,
            element.layout_dimensions.max_height,
        )?;
    }
    if (has_fraction_track(&grid.columns) || has_fraction_track(&grid.rows))
        && element.layout_aspect_ratio.is_some()
    {
        return Err(unsupported(
            element,
            "aspect-ratio",
            "Root HTML Grid fr track에는 aspect-ratio를 함께 지정할 수 없습니다",
        ));
    }
    Ok(())
}

fn has_fraction_track(tracks: &[ComputedCssGridTrackSize]) -> bool {
    tracks
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::Fraction(_)))
}

fn validate_axis(
    element: &ComputedElementStyle,
    size_property: &'static str,
    min_property: &'static str,
    max_property: &'static str,
    size: ComputedCssDimension,
    min_size: ComputedCssDimension,
    max_size: ComputedCssMaxSize,
) -> Result<(), StyleLayoutError> {
    if element.layout_math_values.contains_key(size_property)
        || !matches!(size, ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0)
    {
        return Err(unsupported(
            element,
            size_property,
            "Root HTML Grid fr track requires a definite finite non-negative CSS-pixel size",
        ));
    }
    if min_size != ComputedCssDimension::Auto {
        return Err(unsupported(
            element,
            min_property,
            "Root HTML Grid fr track requires the corresponding computed min-size to be auto",
        ));
    }
    if max_size != ComputedCssMaxSize::None {
        return Err(unsupported(
            element,
            max_property,
            "Root HTML Grid fr track requires the corresponding computed max-size to be none",
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
