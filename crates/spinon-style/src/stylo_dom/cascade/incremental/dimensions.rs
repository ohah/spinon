use std::collections::BTreeMap;

use style::{
    properties::ComputedValues,
    typed_om::{MathValue, NumericValue, ToTyped, TypedValue},
    values::computed::position::Inset,
    values::computed::{FlexBasis, LengthPercentage, MaxSize, NonNegativeLengthPercentage, Size},
};

use crate::stylo_dom::cascade::{
    ComputedCssDimension, ComputedCssMath, ComputedCssMaxSize, ComputedLayoutDimensions,
};

pub(super) fn computed_layout_dimensions(computed: &ComputedValues) -> ComputedLayoutDimensions {
    ComputedLayoutDimensions {
        width: computed_size(computed.clone_width()),
        height: computed_size(computed.clone_height()),
        min_width: computed_size(computed.clone_min_width()),
        max_width: computed_max_size(computed.clone_max_width()),
        min_height: computed_size(computed.clone_min_height()),
        max_height: computed_max_size(computed.clone_max_height()),
        flex_basis: computed_flex_basis(computed.clone_flex_basis()),
    }
}

pub(super) fn computed_layout_math_values(
    computed: &ComputedValues,
    source_math: BTreeMap<String, ComputedCssMath>,
) -> BTreeMap<String, ComputedCssMath> {
    let mut output = BTreeMap::new();
    let width = computed.clone_width();
    let height = computed.clone_height();
    let flex_basis = computed.clone_flex_basis();
    if let Size::LengthPercentage(value) = width {
        insert_math(&mut output, "width", value.0);
    }
    if let Size::LengthPercentage(value) = height {
        insert_math(&mut output, "height", value.0);
    }
    if let FlexBasis::Size(Size::LengthPercentage(value)) = flex_basis {
        insert_math(&mut output, "flex-basis", value.0);
    }
    for (name, value) in [
        ("min-width", computed.clone_min_width()),
        ("min-height", computed.clone_min_height()),
    ] {
        if let Size::LengthPercentage(value) = value {
            insert_math(&mut output, name, value.0);
        }
    }
    for (name, value) in [
        ("max-width", computed.clone_max_width()),
        ("max-height", computed.clone_max_height()),
    ] {
        if let MaxSize::LengthPercentage(value) = value {
            insert_math(&mut output, name, value.0);
        }
    }
    for (name, value) in [
        ("margin-top", computed.clone_margin_top()),
        ("margin-right", computed.clone_margin_right()),
        ("margin-bottom", computed.clone_margin_bottom()),
        ("margin-left", computed.clone_margin_left()),
    ] {
        if let style::values::computed::Margin::LengthPercentage(value) = value {
            insert_math(&mut output, name, value);
        }
    }
    for (name, value) in [
        ("top", computed.clone_top()),
        ("right", computed.clone_right()),
        ("bottom", computed.clone_bottom()),
        ("left", computed.clone_left()),
    ] {
        if let Inset::LengthPercentage(value) = value {
            insert_math(&mut output, name, value);
        }
    }
    for (name, value) in [
        ("padding-top", computed.clone_padding_top()),
        ("padding-right", computed.clone_padding_right()),
        ("padding-bottom", computed.clone_padding_bottom()),
        ("padding-left", computed.clone_padding_left()),
    ] {
        insert_math(&mut output, name, value.0);
    }
    for (name, value) in [
        ("row-gap", computed.clone_row_gap()),
        ("column-gap", computed.clone_column_gap()),
    ] {
        if let style::values::computed::length::NonNegativeLengthPercentageOrNormal::LengthPercentage(
            value,
        ) = value
        {
            insert_math(&mut output, name, value.0);
        }
    }
    for (property, value) in source_math {
        output.entry(property).or_insert(value);
    }
    output
}

fn insert_math(
    output: &mut BTreeMap<String, ComputedCssMath>,
    property: &str,
    value: LengthPercentage,
) {
    if let style::values::computed::length_percentage::Unpacked::Calc(calc) = value.unpack()
        && let Some(TypedValue::Numeric(value)) = calc.to_typed_value()
        && let Some(value) = computed_math(&value, 0, &mut 256)
    {
        output.insert(property.to_owned(), value);
    }
}

pub(super) fn computed_math(
    value: &NumericValue,
    depth: usize,
    remaining_nodes: &mut usize,
) -> Option<ComputedCssMath> {
    if depth > 32 || *remaining_nodes == 0 {
        return None;
    }
    *remaining_nodes -= 1;
    match value {
        NumericValue::Unit(unit) => match unit.unit_str() {
            "number" => Some(ComputedCssMath::Number(unit.value)),
            "px" => Some(ComputedCssMath::LengthPx(unit.value)),
            "percent" => Some(ComputedCssMath::Percentage(unit.value / 100.0)),
            _ => None,
        },
        NumericValue::Math(math) => {
            let mut children = |values: &[NumericValue]| {
                if values.is_empty() || values.len() > 64 {
                    return None;
                }
                values
                    .iter()
                    .map(|value| computed_math(value, depth + 1, remaining_nodes))
                    .collect::<Option<Vec<_>>>()
            };
            match math {
                MathValue::Sum(value) => children(&value.values).map(ComputedCssMath::Sum),
                MathValue::Product(value) => children(&value.values).map(ComputedCssMath::Product),
                MathValue::Negate(value) => {
                    computed_math(value.value.as_ref(), depth + 1, remaining_nodes)
                        .map(|value| ComputedCssMath::Negate(Box::new(value)))
                }
                MathValue::Invert(value) => {
                    computed_math(value.value.as_ref(), depth + 1, remaining_nodes)
                        .map(|value| ComputedCssMath::Invert(Box::new(value)))
                }
                MathValue::Min(value) => children(&value.values).map(ComputedCssMath::Min),
                MathValue::Max(value) => children(&value.values).map(ComputedCssMath::Max),
                MathValue::Clamp(value) => {
                    let [min, value, max] = &*value.values;
                    Some(ComputedCssMath::Clamp {
                        min: Box::new(computed_math(min, depth + 1, remaining_nodes)?),
                        value: Box::new(computed_math(value, depth + 1, remaining_nodes)?),
                        max: Box::new(computed_math(max, depth + 1, remaining_nodes)?),
                    })
                }
            }
        }
    }
}

fn computed_flex_basis(value: FlexBasis) -> ComputedCssDimension {
    match value {
        FlexBasis::Content => ComputedCssDimension::Unsupported,
        FlexBasis::Size(size) => computed_size(size),
    }
}

fn computed_size(value: Size) -> ComputedCssDimension {
    match value {
        Size::Auto => ComputedCssDimension::Auto,
        Size::LengthPercentage(value) => computed_length_percentage(value),
        Size::MaxContent
        | Size::MinContent
        | Size::FitContent
        | Size::WebkitFillAvailable
        | Size::Stretch
        | Size::FitContentFunction(_)
        | Size::AnchorSizeFunction(_)
        | Size::AnchorContainingCalcFunction(_) => ComputedCssDimension::Unsupported,
    }
}

fn computed_max_size(value: MaxSize) -> ComputedCssMaxSize {
    match value {
        MaxSize::None => ComputedCssMaxSize::None,
        MaxSize::LengthPercentage(value) => match computed_length_percentage(value) {
            ComputedCssDimension::LengthPx(value) => ComputedCssMaxSize::LengthPx(value),
            ComputedCssDimension::Percentage(value) => ComputedCssMaxSize::Percentage(value),
            ComputedCssDimension::Auto | ComputedCssDimension::Unsupported => {
                ComputedCssMaxSize::Unsupported
            }
        },
        MaxSize::MaxContent
        | MaxSize::MinContent
        | MaxSize::FitContent
        | MaxSize::WebkitFillAvailable
        | MaxSize::Stretch
        | MaxSize::FitContentFunction(_)
        | MaxSize::AnchorSizeFunction(_)
        | MaxSize::AnchorContainingCalcFunction(_) => ComputedCssMaxSize::Unsupported,
    }
}

fn computed_length_percentage(value: NonNegativeLengthPercentage) -> ComputedCssDimension {
    match value.0.unpack() {
        style::values::computed::length_percentage::Unpacked::Length(length) => {
            finite_nonnegative(length.px(), ComputedCssDimension::LengthPx)
        }
        style::values::computed::length_percentage::Unpacked::Percentage(percentage) => {
            finite_nonnegative(percentage.0, ComputedCssDimension::Percentage)
        }
        style::values::computed::length_percentage::Unpacked::Calc(_) => {
            ComputedCssDimension::Unsupported
        }
    }
}

fn finite_nonnegative(
    value: f32,
    constructor: fn(f32) -> ComputedCssDimension,
) -> ComputedCssDimension {
    if value.is_finite() && value >= 0.0 {
        constructor(value)
    } else {
        ComputedCssDimension::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use style::values::{computed::LengthPercentage, generics::NonNegative};

    #[test]
    fn preserves_computed_size_percentages_as_fractions() {
        assert_eq!(
            computed_length_percentage(NonNegative(LengthPercentage::new_percent(
                style::values::computed::Percentage(0.5)
            ))),
            ComputedCssDimension::Percentage(0.5)
        );
        assert_eq!(
            computed_length_percentage(NonNegative(LengthPercentage::new_percent(
                style::values::computed::Percentage(1.25)
            ))),
            ComputedCssDimension::Percentage(1.25)
        );
        assert_eq!(
            computed_length_percentage(NonNegative(LengthPercentage::new_percent(
                style::values::computed::Percentage(0.0)
            ))),
            ComputedCssDimension::Percentage(0.0)
        );
    }

    #[test]
    fn preserves_lengths_and_rejects_calc_for_the_c06_1_bridge() {
        assert_eq!(
            computed_length_percentage(NonNegative(LengthPercentage::new_length(
                style::values::computed::Length::new(12.5)
            ))),
            ComputedCssDimension::LengthPx(12.5)
        );
        assert_eq!(computed_size(Size::Auto), ComputedCssDimension::Auto);
        assert_eq!(
            computed_flex_basis(FlexBasis::Content),
            ComputedCssDimension::Unsupported
        );
    }
}
