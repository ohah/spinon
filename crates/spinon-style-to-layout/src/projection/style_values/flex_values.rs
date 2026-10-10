//! Stylo computed CSS에서 Flex 관련 값과 순서 정수를 해석합니다.

use spinon_core::NodeId;
use spinon_layout::{
    AlignmentSafety, ContentAlignmentPosition, FlexDirection, FlexWrap, ItemAlignmentPosition,
    JustifyContentPosition, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf,
    LayoutJustifyContent, TextDirection,
};

use crate::StyleLayoutError;

use super::primitive_values::{unsupported, unsupported_value};

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

pub(super) fn parse_runtime_align_items(
    node: NodeId,
    value: &str,
) -> Result<LayoutAlignItems, StyleLayoutError> {
    let parsed = match value {
        "normal" => Some(LayoutAlignItems::Normal),
        "stretch" => Some(LayoutAlignItems::Stretch),
        "flex-start" => Some(LayoutAlignItems::FlexStart),
        "flex-end" => Some(LayoutAlignItems::FlexEnd),
        "center" => Some(LayoutAlignItems::Center),
        _ => parse_item_position(value)
            .map(|(position, safety)| LayoutAlignItems::Position { position, safety }),
    };
    parsed.ok_or_else(|| unsupported_value(node, "align-items", value))
}

pub(super) fn parse_runtime_align_self(
    node: NodeId,
    value: &str,
) -> Result<LayoutAlignSelf, StyleLayoutError> {
    let parsed = match value {
        "auto" => Some(LayoutAlignSelf::Auto),
        "normal" => Some(LayoutAlignSelf::Normal),
        "stretch" => Some(LayoutAlignSelf::Stretch),
        "flex-start" => Some(LayoutAlignSelf::FlexStart),
        "flex-end" => Some(LayoutAlignSelf::FlexEnd),
        "center" => Some(LayoutAlignSelf::Center),
        _ => parse_item_position(value)
            .map(|(position, safety)| LayoutAlignSelf::Position { position, safety }),
    };
    parsed.ok_or_else(|| unsupported_value(node, "align-self", value))
}

pub(super) fn parse_runtime_align_content(
    node: NodeId,
    value: &str,
) -> Result<LayoutAlignContent, StyleLayoutError> {
    let parsed = match value {
        "normal" => Some(LayoutAlignContent::Normal),
        "stretch" => Some(LayoutAlignContent::Stretch),
        "flex-start" => Some(LayoutAlignContent::FlexStart),
        "flex-end" => Some(LayoutAlignContent::FlexEnd),
        "center" => Some(LayoutAlignContent::Center),
        "space-between" => Some(LayoutAlignContent::SpaceBetween),
        "space-around" => Some(LayoutAlignContent::SpaceAround),
        "space-evenly" => Some(LayoutAlignContent::SpaceEvenly),
        _ => parse_content_position(value)
            .map(|(position, safety)| LayoutAlignContent::Position { position, safety }),
    };
    parsed.ok_or_else(|| unsupported_value(node, "align-content", value))
}

pub(super) fn parse_runtime_justify_content(
    node: NodeId,
    value: &str,
) -> Result<LayoutJustifyContent, StyleLayoutError> {
    let parsed = match value {
        "normal" => Some(LayoutJustifyContent::Normal),
        "stretch" => Some(LayoutJustifyContent::Stretch),
        "flex-start" => Some(LayoutJustifyContent::FlexStart),
        "flex-end" => Some(LayoutJustifyContent::FlexEnd),
        "center" => Some(LayoutJustifyContent::Center),
        "space-between" => Some(LayoutJustifyContent::SpaceBetween),
        "space-around" => Some(LayoutJustifyContent::SpaceAround),
        "space-evenly" => Some(LayoutJustifyContent::SpaceEvenly),
        _ => parse_justify_position(value)
            .map(|(position, safety)| LayoutJustifyContent::Position { position, safety }),
    };
    parsed.ok_or_else(|| unsupported_value(node, "justify-content", value))
}

fn split_safety(value: &str) -> Option<(Option<AlignmentSafety>, &str)> {
    let mut parts = value.split_ascii_whitespace();
    let first = parts.next()?;
    let second = parts.next();
    if parts.next().is_some() {
        return None;
    }
    match second {
        None => Some((None, first)),
        Some(position) => {
            let safety = match first {
                "safe" => AlignmentSafety::Safe,
                "unsafe" => AlignmentSafety::Unsafe,
                _ => return None,
            };
            Some((Some(safety), position))
        }
    }
}

fn parse_item_position(value: &str) -> Option<(ItemAlignmentPosition, Option<AlignmentSafety>)> {
    let (safety, position) = split_safety(value)?;
    let position = match position {
        "start" => ItemAlignmentPosition::Start,
        "end" => ItemAlignmentPosition::End,
        "flex-start" => ItemAlignmentPosition::FlexStart,
        "flex-end" => ItemAlignmentPosition::FlexEnd,
        "self-start" => ItemAlignmentPosition::SelfStart,
        "self-end" => ItemAlignmentPosition::SelfEnd,
        "center" => ItemAlignmentPosition::Center,
        _ => return None,
    };
    Some((position, safety))
}

fn parse_content_position(
    value: &str,
) -> Option<(ContentAlignmentPosition, Option<AlignmentSafety>)> {
    let (safety, position) = split_safety(value)?;
    let position = match position {
        "start" => ContentAlignmentPosition::Start,
        "end" => ContentAlignmentPosition::End,
        "flex-start" => ContentAlignmentPosition::FlexStart,
        "flex-end" => ContentAlignmentPosition::FlexEnd,
        "center" => ContentAlignmentPosition::Center,
        _ => return None,
    };
    Some((position, safety))
}

fn parse_justify_position(
    value: &str,
) -> Option<(JustifyContentPosition, Option<AlignmentSafety>)> {
    let (safety, position) = split_safety(value)?;
    let position = match position {
        "start" => JustifyContentPosition::Start,
        "end" => JustifyContentPosition::End,
        "flex-start" => JustifyContentPosition::FlexStart,
        "flex-end" => JustifyContentPosition::FlexEnd,
        "center" => JustifyContentPosition::Center,
        "left" => JustifyContentPosition::Left,
        "right" => JustifyContentPosition::Right,
        _ => return None,
    };
    Some((position, safety))
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
