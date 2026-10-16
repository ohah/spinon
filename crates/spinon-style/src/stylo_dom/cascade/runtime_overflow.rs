use spinon_core::NodeId;
use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    runtime_layout::{RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, runtime_flex_wrap_properties},
    runtime_paint::{
        RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES, runtime_block_positioning_properties,
    },
    runtime_stacking::runtime_block_flex_stacking_author_properties,
};
use crate::StylesheetSource;

pub(super) fn runtime_block_flex_overflow_clip_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_flex_wrap_properties();
    for property in runtime_block_positioning_properties() {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    for property in [
        ("z-index", LonghandId::ZIndex),
        ("visibility", LonghandId::Visibility),
        ("opacity", LonghandId::Opacity),
        ("overflow-x", LonghandId::OverflowX),
        ("overflow-y", LonghandId::OverflowY),
        ("overflow-clip-margin", LonghandId::OverflowClipMargin),
    ] {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    properties
}

/// C13.1 overflow clip author declaration allowlist입니다.
pub(super) fn runtime_block_flex_overflow_clip_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_stacking_author_properties();
    for property in [
        "overflow",
        "overflow-x",
        "overflow-y",
        "overflow-clip-margin",
    ] {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

/// C13.1용 혼합 Block/Flex/positioning/stacking cascade에 overflow 값을 추가합니다.
pub fn compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: spinon_core::StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_flex_overflow_clip_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1,
    )
}

/// C19.2의 alpha paint와 C13.1 입력을 같은 Stylo cascade에서 계산합니다.
pub fn compute_c19_box_paint_opacity_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: spinon_core::StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_flex_overflow_clip_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::C19BoxPaintOpacityV1,
    )
}

/// C13.1에서 runtime layout·paint·clip 입력 밖의 inline declaration을 찾습니다.
pub fn first_unsupported_runtime_block_flex_overflow_clip_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_overflow_clip_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        property.starts_with("--")
            || RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES.contains(&property)
            || RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES.contains(&property)
            || ["z-index", "visibility"].contains(&property)
            || allowed.contains(&property)
    })
}
