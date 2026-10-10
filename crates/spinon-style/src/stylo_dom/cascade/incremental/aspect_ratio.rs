use spinon_core::NodeId;
use style::{properties::ComputedValues, values::generics::position::PreferredRatio};

use crate::stylo_dom::cascade::CssCascadeError;

pub(crate) fn computed_layout_aspect_ratio(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<Option<f32>, CssCascadeError> {
    let value = computed.clone_aspect_ratio();
    let PreferredRatio::Ratio(ratio) = value.ratio else {
        return Ok(None);
    };

    if ratio.is_degenerate() {
        return Ok(None);
    }
    if value.auto {
        return Err(CssCascadeError::UnsupportedComputedAspectRatio {
            node,
            reason: "auto <ratio>의 content-box 의미는 현재 Taffy 계약에서 보존하지 않습니다"
                .into(),
        });
    }

    let numerator = (ratio.0).0;
    let denominator = (ratio.1).0;
    let width_over_height = numerator / denominator;
    if !numerator.is_finite()
        || !denominator.is_finite()
        || numerator <= 0.0
        || denominator <= 0.0
        || !width_over_height.is_finite()
        || width_over_height <= 0.0
    {
        return Err(CssCascadeError::UnsupportedComputedAspectRatio {
            node,
            reason: format!("ratio가 유한한 양수 f32가 아닙니다: {numerator} / {denominator}"),
        });
    }

    Ok(Some(width_over_height))
}
