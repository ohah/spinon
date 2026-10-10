//! 기본 display·box·크기 값을 typed layout 입력으로 변환합니다.

use spinon_core::NodeId;
use spinon_layout::{LayoutBoxSizing, LayoutDimension, LayoutDisplay};
use spinon_style::{ComputedCssDimension, ComputedCssMath, ComputedCssMaxSize};

use super::super::typed_math::CssMathProjector;
use super::spacing_values::layout_math_property;
use crate::StyleLayoutError;

pub(super) fn required<'a>(
    element: &'a spinon_style::ComputedElementStyle,
    property: &'static str,
) -> Result<&'a str, StyleLayoutError> {
    element.properties.get(property).map(String::as_str).ok_or(
        StyleLayoutError::MissingComputedProperty {
            node: element.node_id,
            property,
        },
    )
}

pub(super) fn parse_display(
    node: NodeId,
    value: &str,
    allow_flow_root: bool,
) -> Result<LayoutDisplay, StyleLayoutError> {
    match value {
        "flex" => Ok(LayoutDisplay::Flex),
        "block" => Ok(LayoutDisplay::Block),
        "flow-root" if allow_flow_root => Ok(LayoutDisplay::FlowRoot),
        "none" => Ok(LayoutDisplay::None),
        value => unsupported(node, "display", value),
    }
}

pub(super) fn parse_box_sizing(
    node: NodeId,
    value: &str,
) -> Result<LayoutBoxSizing, StyleLayoutError> {
    match value {
        "border-box" => Ok(LayoutBoxSizing::BorderBox),
        "content-box" => Ok(LayoutBoxSizing::ContentBox),
        value => unsupported(node, "box-sizing", value),
    }
}

pub(super) fn max_size_as_dimension(value: ComputedCssMaxSize) -> ComputedCssDimension {
    match value {
        ComputedCssMaxSize::None => ComputedCssDimension::Auto,
        ComputedCssMaxSize::LengthPx(value) => ComputedCssDimension::LengthPx(value),
        ComputedCssMaxSize::Percentage(value) => ComputedCssDimension::Percentage(value),
        ComputedCssMaxSize::Unsupported => ComputedCssDimension::Unsupported,
    }
}

pub(super) fn parse_dimension(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssDimension,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
) -> Result<LayoutDimension, StyleLayoutError> {
    if let Some(math) = math {
        let id = projector.project(node, layout_math_property(node, property)?, math)?;
        return Ok(LayoutDimension::Calc(id));
    }
    match value {
        ComputedCssDimension::Auto => Ok(LayoutDimension::Auto),
        ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0 => {
            Ok(LayoutDimension::Fixed(value))
        }
        ComputedCssDimension::Percentage(value) if value.is_finite() && value >= 0.0 => {
            Ok(LayoutDimension::Percent(value))
        }
        ComputedCssDimension::LengthPx(_)
        | ComputedCssDimension::Percentage(_)
        | ComputedCssDimension::Unsupported => unsupported(node, property, serialized_value),
    }
}

pub(super) fn unsupported<T>(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<T, StyleLayoutError> {
    Err(unsupported_value(node, property, value))
}

pub(super) fn unsupported_value(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    }
}
