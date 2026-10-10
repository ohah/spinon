//! Stylo computed CSS에서 Flex 관련 값과 순서 정수를 해석합니다.

use spinon_core::NodeId;
use spinon_layout::{
    FlexDirection, FlexWrap, LayoutAlignItems, LayoutJustifyContent, TextDirection,
};

use crate::StyleLayoutError;

use super::{unsupported, unsupported_value};

pub(super) fn parse_align_items(
    node: NodeId,
    value: &str,
) -> Result<LayoutAlignItems, StyleLayoutError> {
    match value {
        "normal" | "stretch" => Ok(LayoutAlignItems::Stretch),
        "flex-start" => Ok(LayoutAlignItems::FlexStart),
        "flex-end" => Ok(LayoutAlignItems::FlexEnd),
        "center" => Ok(LayoutAlignItems::Center),
        value => unsupported(node, "align-items", value),
    }
}

pub(super) fn parse_justify_content(
    node: NodeId,
    value: &str,
) -> Result<LayoutJustifyContent, StyleLayoutError> {
    match value {
        "normal" | "flex-start" => Ok(LayoutJustifyContent::FlexStart),
        "flex-end" => Ok(LayoutJustifyContent::FlexEnd),
        "center" => Ok(LayoutJustifyContent::Center),
        "space-between" => Ok(LayoutJustifyContent::SpaceBetween),
        "space-around" => Ok(LayoutJustifyContent::SpaceAround),
        "space-evenly" => Ok(LayoutJustifyContent::SpaceEvenly),
        value => unsupported(node, "justify-content", value),
    }
}

pub(super) fn parse_flex_direction(
    node: NodeId,
    value: &str,
    supports_reverse: bool,
) -> Result<FlexDirection, StyleLayoutError> {
    match value {
        "row" => Ok(FlexDirection::Row),
        "column" => Ok(FlexDirection::Column),
        "row-reverse" if supports_reverse => Ok(FlexDirection::RowReverse),
        "column-reverse" if supports_reverse => Ok(FlexDirection::ColumnReverse),
        value => unsupported(node, "flex-direction", value),
    }
}

pub(super) fn parse_flex_wrap(node: NodeId, value: &str) -> Result<FlexWrap, StyleLayoutError> {
    match value {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        value => unsupported(node, "flex-wrap", value),
    }
}

pub(super) fn parse_direction(
    node: NodeId,
    value: &str,
) -> Result<TextDirection, StyleLayoutError> {
    match value {
        "ltr" => Ok(TextDirection::Ltr),
        "rtl" => Ok(TextDirection::Rtl),
        value => unsupported(node, "direction", value),
    }
}

pub(super) fn parse_number(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<f32, StyleLayoutError> {
    let parsed = value
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite() && *number >= 0.0);
    parsed.ok_or_else(|| unsupported_value(node, property, value))
}

pub(super) fn parse_order(node: NodeId, value: &str) -> Result<i32, StyleLayoutError> {
    value
        .parse::<i32>()
        .map_err(|_| unsupported_value(node, "order", value))
}
