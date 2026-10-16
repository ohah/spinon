use spinon_core::NodeId;
use taffy::AvailableSpace;

use crate::{LayoutDimension, LayoutError, LayoutIntrinsicSize};

use super::{
    Edges, EffectiveRatio, dimension_to_content, unconstrained_size, unsupported,
    validate_used_size,
};

pub(super) struct PreferredDimensionInput {
    pub(super) node: NodeId,
    pub(super) property: &'static str,
    pub(super) dimension: LayoutDimension,
    pub(super) edge: f32,
    pub(super) containing_basis: Option<f32>,
    pub(super) available_space: AvailableSpace,
    pub(super) natural: LayoutIntrinsicSize,
    pub(super) ratio: Option<EffectiveRatio>,
    pub(super) edges: Edges,
}

pub(super) fn preferred_dimension_to_content(
    input: PreferredDimensionInput,
) -> Result<Option<f32>, LayoutError> {
    let PreferredDimensionInput {
        node,
        property,
        dimension,
        edge,
        containing_basis,
        available_space,
        natural,
        ratio,
        edges,
    } = input;
    let (value, apply_box_sizing) = match dimension {
        LayoutDimension::Auto => return Ok(None),
        LayoutDimension::Fixed(_) | LayoutDimension::Calc(_) => {
            return dimension_to_content(node, property, dimension, edge);
        }
        LayoutDimension::Percent(fraction) => {
            if available_space == AvailableSpace::MinContent {
                (0.0, true)
            } else if let Some(basis) = containing_basis {
                (basis * fraction, true)
            } else {
                return Err(unsupported(
                    node,
                    "indefinite percentage size는 cyclic min-content 문맥 외에는 지원하지 않습니다",
                ));
            }
        }
        LayoutDimension::CyclicMinContentPercent(fraction) => {
            if available_space == AvailableSpace::MinContent || containing_basis.is_none() {
                (0.0, true)
            } else {
                (containing_basis.map_or(0.0, |basis| basis * fraction), true)
            }
        }
        LayoutDimension::MinContent | LayoutDimension::MaxContent | LayoutDimension::FitContent => {
            let intrinsic = unconstrained_size(node, natural.width, natural.height, ratio, edges)?;
            let intrinsic = if property == "width" {
                intrinsic.content_width
            } else {
                intrinsic.content_height
            };
            let value = match dimension {
                LayoutDimension::FitContent => match available_space {
                    AvailableSpace::Definite(stretch_fit) => {
                        intrinsic.max(stretch_fit.min(intrinsic))
                    }
                    AvailableSpace::MinContent => intrinsic,
                    AvailableSpace::MaxContent => intrinsic,
                },
                LayoutDimension::MinContent | LayoutDimension::MaxContent => intrinsic,
                _ => unreachable!("intrinsic sizing keyword match is exhaustive"),
            };
            // Intrinsic contributions are content-box sizes. `box-sizing:border-box`
            // constrains explicit sizes, but must not subtract padding and border from
            // the image's natural content contribution.
            (value, false)
        }
    };
    validate_used_size(node, value, 0.0)?;
    Ok(Some(if apply_box_sizing {
        (value - edge).max(0.0)
    } else {
        value
    }))
}
