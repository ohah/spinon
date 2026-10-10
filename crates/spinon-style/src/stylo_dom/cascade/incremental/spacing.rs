use style::values::computed::length::NonNegativeLengthPercentageOrNormal;
use style::{
    properties::ComputedValues,
    values::computed::{LengthPercentage, Margin, NonNegativeLengthPercentage},
};

use crate::stylo_dom::cascade::{ComputedCssEdges, ComputedCssSpacingValue, ComputedLayoutSpacing};

pub(super) fn computed_layout_spacing(computed: &ComputedValues) -> ComputedLayoutSpacing {
    ComputedLayoutSpacing {
        margin: ComputedCssEdges {
            top: computed_margin(computed.clone_margin_top()),
            right: computed_margin(computed.clone_margin_right()),
            bottom: computed_margin(computed.clone_margin_bottom()),
            left: computed_margin(computed.clone_margin_left()),
        },
        padding: ComputedCssEdges {
            top: computed_nonnegative_length_percentage(computed.clone_padding_top()),
            right: computed_nonnegative_length_percentage(computed.clone_padding_right()),
            bottom: computed_nonnegative_length_percentage(computed.clone_padding_bottom()),
            left: computed_nonnegative_length_percentage(computed.clone_padding_left()),
        },
        row_gap: computed_gap(computed.clone_row_gap()),
        column_gap: computed_gap(computed.clone_column_gap()),
    }
}

fn computed_margin(value: Margin) -> ComputedCssSpacingValue {
    match value {
        Margin::Auto => ComputedCssSpacingValue::Auto,
        Margin::LengthPercentage(value) => computed_length_percentage(value),
        Margin::AnchorSizeFunction(_) | Margin::AnchorContainingCalcFunction(_) => {
            ComputedCssSpacingValue::Unsupported
        }
    }
}

fn computed_nonnegative_length_percentage(
    value: NonNegativeLengthPercentage,
) -> ComputedCssSpacingValue {
    computed_length_percentage(value.0)
}

fn computed_length_percentage(value: LengthPercentage) -> ComputedCssSpacingValue {
    match value.unpack() {
        style::values::computed::length_percentage::Unpacked::Length(length) => {
            finite(length.px(), ComputedCssSpacingValue::LengthPx)
        }
        style::values::computed::length_percentage::Unpacked::Percentage(percentage) => {
            finite(percentage.0, ComputedCssSpacingValue::Percentage)
        }
        style::values::computed::length_percentage::Unpacked::Calc(_) => {
            ComputedCssSpacingValue::Unsupported
        }
    }
}

fn computed_gap(value: NonNegativeLengthPercentageOrNormal) -> ComputedCssSpacingValue {
    match value {
        NonNegativeLengthPercentageOrNormal::LengthPercentage(value) => {
            computed_nonnegative_length_percentage(value)
        }
        NonNegativeLengthPercentageOrNormal::Normal => ComputedCssSpacingValue::Normal,
    }
}

fn finite(value: f32, constructor: fn(f32) -> ComputedCssSpacingValue) -> ComputedCssSpacingValue {
    if value.is_finite() {
        constructor(value)
    } else {
        ComputedCssSpacingValue::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use style::values::computed::{Length, Percentage};

    #[test]
    fn keeps_signed_margin_percentages_without_clamping() {
        assert_eq!(
            computed_margin(Margin::LengthPercentage(LengthPercentage::new_percent(
                Percentage(-0.05)
            ))),
            ComputedCssSpacingValue::Percentage(-0.05)
        );
        assert_eq!(computed_margin(Margin::Auto), ComputedCssSpacingValue::Auto);
    }

    #[test]
    fn keeps_padding_and_gap_typed_values_and_normal_keyword() {
        assert_eq!(
            computed_nonnegative_length_percentage(style::values::generics::NonNegative(
                LengthPercentage::new_percent(Percentage(1.25))
            )),
            ComputedCssSpacingValue::Percentage(1.25)
        );
        assert_eq!(
            computed_nonnegative_length_percentage(style::values::generics::NonNegative(
                LengthPercentage::new_length(Length::new(12.5))
            )),
            ComputedCssSpacingValue::LengthPx(12.5)
        );
        assert_eq!(
            computed_gap(NonNegativeLengthPercentageOrNormal::Normal),
            ComputedCssSpacingValue::Normal
        );
    }
}
