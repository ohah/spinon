//! Runtime CSS `position`과 물리 inset 값을 typed layout 입력으로 변환합니다.

use spinon_layout::{LayoutEdges, LayoutPosition, LayoutPositioning};
use spinon_style::{ComputedCssPosition, ComputedElementStyle};

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
