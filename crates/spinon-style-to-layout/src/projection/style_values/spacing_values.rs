//! CSS gap, margin, padding과 typed math를 layout spacing 값으로 투영합니다.

use spinon_core::NodeId;
use spinon_layout::{LayoutCssMathProperty, LayoutLengthPercentage};
use spinon_style::{ComputedCssMath, ComputedCssSpacingValue};

use crate::StyleLayoutError;

use super::super::typed_math::CssMathProjector;
use super::primitive_values::unsupported_value;

pub(super) fn parse_gap(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssSpacingValue,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
) -> Result<LayoutLengthPercentage, StyleLayoutError> {
    if math.is_none() && value == ComputedCssSpacingValue::Normal {
        return Ok(LayoutLengthPercentage::ZERO);
    }
    parse_nonnegative_spacing(node, property, serialized_value, value, math, projector)
}

pub(super) fn parse_margin(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssSpacingValue,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
    allow_auto: bool,
) -> Result<LayoutLengthPercentage, StyleLayoutError> {
    if allow_auto && math.is_none() && value == ComputedCssSpacingValue::Auto {
        return Ok(LayoutLengthPercentage::Auto);
    }
    parse_spacing(
        node,
        property,
        serialized_value,
        value,
        true,
        math,
        projector,
    )
}

pub(super) fn parse_nonnegative_spacing(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssSpacingValue,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
) -> Result<LayoutLengthPercentage, StyleLayoutError> {
    parse_spacing(
        node,
        property,
        serialized_value,
        value,
        false,
        math,
        projector,
    )
}

fn parse_spacing(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssSpacingValue,
    allow_negative: bool,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
) -> Result<LayoutLengthPercentage, StyleLayoutError> {
    if let Some(math) = math {
        let id = projector.project(node, layout_math_property(node, property)?, math)?;
        return Ok(LayoutLengthPercentage::Calc(id));
    }
    let convert = |value: f32, percentage: bool| {
        (value.is_finite() && (allow_negative || value >= 0.0)).then_some(if percentage {
            LayoutLengthPercentage::percent(value)
        } else {
            LayoutLengthPercentage::length(value)
        })
    };
    match value {
        ComputedCssSpacingValue::LengthPx(value) => convert(value, false),
        ComputedCssSpacingValue::Percentage(value) => convert(value, true),
        ComputedCssSpacingValue::Auto
        | ComputedCssSpacingValue::Normal
        | ComputedCssSpacingValue::Unsupported => None,
    }
    .ok_or_else(|| unsupported_value(node, property, serialized_value))
}

pub(super) fn layout_math_property(
    node: NodeId,
    property: &'static str,
) -> Result<LayoutCssMathProperty, StyleLayoutError> {
    match property {
        "width" => Ok(LayoutCssMathProperty::Width),
        "height" => Ok(LayoutCssMathProperty::Height),
        "min-width" => Ok(LayoutCssMathProperty::MinWidth),
        "max-width" => Ok(LayoutCssMathProperty::MaxWidth),
        "min-height" => Ok(LayoutCssMathProperty::MinHeight),
        "max-height" => Ok(LayoutCssMathProperty::MaxHeight),
        "flex-basis" => Ok(LayoutCssMathProperty::FlexBasis),
        "margin-top" => Ok(LayoutCssMathProperty::MarginTop),
        "margin-right" => Ok(LayoutCssMathProperty::MarginRight),
        "margin-bottom" => Ok(LayoutCssMathProperty::MarginBottom),
        "margin-left" => Ok(LayoutCssMathProperty::MarginLeft),
        "padding-top" => Ok(LayoutCssMathProperty::PaddingTop),
        "padding-right" => Ok(LayoutCssMathProperty::PaddingRight),
        "padding-bottom" => Ok(LayoutCssMathProperty::PaddingBottom),
        "padding-left" => Ok(LayoutCssMathProperty::PaddingLeft),
        "row-gap" => Ok(LayoutCssMathProperty::RowGap),
        "column-gap" => Ok(LayoutCssMathProperty::ColumnGap),
        _ => Err(unsupported_value(node, property, "typed CSS math")),
    }
}
