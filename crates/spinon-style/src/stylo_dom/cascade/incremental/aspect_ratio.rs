use crate::stylo_dom::cascade::snapshot::{
    ComputedAspectRatioSizingBox, ComputedLayoutAspectRatio, ComputedStyleProfile,
};
use spinon_core::NodeId;
use style::{properties::ComputedValues, values::generics::position::PreferredRatio};

use crate::stylo_dom::cascade::CssCascadeError;

pub(crate) fn computed_layout_aspect_ratio(
    computed: &ComputedValues,
    node: NodeId,
    profile: ComputedStyleProfile,
) -> Result<Option<ComputedLayoutAspectRatio>, CssCascadeError> {
    let value = computed.clone_aspect_ratio();
    let PreferredRatio::Ratio(ratio) = value.ratio else {
        return Ok(None);
    };

    if ratio.is_degenerate() {
        return Ok(None);
    }
    if value.auto && profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(CssCascadeError::UnsupportedComputedAspectRatio {
            node,
            reason: "auto <ratio>의 content-box 의미는 C11.4 direct-text fixture profile에서만 보존합니다"
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

    Ok(Some(ComputedLayoutAspectRatio {
        ratio: width_over_height,
        sizing_box: if value.auto {
            ComputedAspectRatioSizingBox::ContentBox
        } else {
            ComputedAspectRatioSizingBox::BoxSizing
        },
    }))
}

/// Stylo currently serializes a degenerate computed ratio as `auto`, although
/// CSS Sizing keeps the computed numeric pair and makes only its sizing effect
/// equivalent to `auto`. Preserve the computed pair in our snapshot string.
pub(crate) fn computed_aspect_ratio_serialization(computed: &ComputedValues) -> String {
    let value = computed.clone_aspect_ratio();
    let ratio = match value.ratio {
        PreferredRatio::None => return "auto".to_owned(),
        PreferredRatio::Ratio(ratio) => ratio,
    };
    let numerator = (ratio.0).0;
    let denominator = (ratio.1).0;
    let pair = format!("{numerator} / {denominator}");
    if value.auto {
        format!("auto {pair}")
    } else {
        pair
    }
}
