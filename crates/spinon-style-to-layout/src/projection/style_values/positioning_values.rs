//! Runtime CSS `position`과 물리 inset 값을 typed layout 입력으로 변환합니다.

use spinon_layout::{LayoutEdges, LayoutPosition, LayoutPositioning};
use spinon_style::{ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle};

use super::super::typed_math::CssMathProjector;
use super::{
    primitive_values::{required, unsupported_value},
    spacing_values::parse_inset,
};
use crate::StyleLayoutError;

pub(super) fn parse_positioning(
    element: &ComputedElementStyle,
    projector: &mut CssMathProjector,
    supports_absolute: bool,
    supports_fixed: bool,
    supports_sticky: bool,
) -> Result<LayoutPositioning, StyleLayoutError> {
    match element.layout_position {
        ComputedCssPosition::Static => Ok(LayoutPositioning::default()),
        ComputedCssPosition::Relative => Ok(LayoutPositioning {
            position: LayoutPosition::Relative,
            inset: LayoutEdges {
                top: parse_inset(
                    element.node_id,
                    "top",
                    required(element, "top")?,
                    element.layout_insets.top,
                    element.layout_math_values.get("top"),
                    projector,
                )?,
                right: parse_inset(
                    element.node_id,
                    "right",
                    required(element, "right")?,
                    element.layout_insets.right,
                    element.layout_math_values.get("right"),
                    projector,
                )?,
                bottom: parse_inset(
                    element.node_id,
                    "bottom",
                    required(element, "bottom")?,
                    element.layout_insets.bottom,
                    element.layout_math_values.get("bottom"),
                    projector,
                )?,
                left: parse_inset(
                    element.node_id,
                    "left",
                    required(element, "left")?,
                    element.layout_insets.left,
                    element.layout_math_values.get("left"),
                    projector,
                )?,
            },
        }),
        ComputedCssPosition::Absolute if supports_absolute => Ok(LayoutPositioning {
            position: LayoutPosition::Absolute,
            inset: LayoutEdges {
                top: parse_inset(
                    element.node_id,
                    "top",
                    required(element, "top")?,
                    element.layout_insets.top,
                    element.layout_math_values.get("top"),
                    projector,
                )?,
                right: parse_inset(
                    element.node_id,
                    "right",
                    required(element, "right")?,
                    element.layout_insets.right,
                    element.layout_math_values.get("right"),
                    projector,
                )?,
                bottom: parse_inset(
                    element.node_id,
                    "bottom",
                    required(element, "bottom")?,
                    element.layout_insets.bottom,
                    element.layout_math_values.get("bottom"),
                    projector,
                )?,
                left: parse_inset(
                    element.node_id,
                    "left",
                    required(element, "left")?,
                    element.layout_insets.left,
                    element.layout_math_values.get("left"),
                    projector,
                )?,
            },
        }),
        ComputedCssPosition::Fixed if supports_fixed => Ok(LayoutPositioning {
            position: LayoutPosition::Fixed,
            inset: LayoutEdges {
                top: parse_inset(
                    element.node_id,
                    "top",
                    required(element, "top")?,
                    element.layout_insets.top,
                    element.layout_math_values.get("top"),
                    projector,
                )?,
                right: parse_inset(
                    element.node_id,
                    "right",
                    required(element, "right")?,
                    element.layout_insets.right,
                    element.layout_math_values.get("right"),
                    projector,
                )?,
                bottom: parse_inset(
                    element.node_id,
                    "bottom",
                    required(element, "bottom")?,
                    element.layout_insets.bottom,
                    element.layout_math_values.get("bottom"),
                    projector,
                )?,
                left: parse_inset(
                    element.node_id,
                    "left",
                    required(element, "left")?,
                    element.layout_insets.left,
                    element.layout_math_values.get("left"),
                    projector,
                )?,
            },
        }),
        ComputedCssPosition::Sticky if supports_sticky => {
            let top = element.layout_insets.top;
            if element.layout_math_values.contains_key("top")
                || !matches!(top, ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0)
            {
                return Err(unsupported_value(
                    element.node_id,
                    "top",
                    required(element, "top")?,
                ));
            }
            for (property, value) in [
                ("right", element.layout_insets.right),
                ("bottom", element.layout_insets.bottom),
                ("left", element.layout_insets.left),
            ] {
                if value != ComputedCssSpacingValue::Auto
                    || element.layout_math_values.contains_key(property)
                {
                    return Err(unsupported_value(
                        element.node_id,
                        property,
                        required(element, property)?,
                    ));
                }
            }
            // Sticky는 Taffy에 normal-flow block으로 전달하고, 위치 제약은 render snapshot에서 적용합니다.
            Ok(LayoutPositioning::default())
        }
        position @ (ComputedCssPosition::Absolute
        | ComputedCssPosition::Fixed
        | ComputedCssPosition::Sticky) => Err(unsupported_value(
            element.node_id,
            "position",
            match position {
                ComputedCssPosition::Absolute => "absolute",
                ComputedCssPosition::Fixed => "fixed",
                ComputedCssPosition::Sticky => "sticky",
                ComputedCssPosition::Static | ComputedCssPosition::Relative => unreachable!(),
            },
        )),
    }
}
