use spinon_core::NodeId;
use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    runtime_layout::{
        RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, first_unsupported_inline_property,
        runtime_flex_wrap_properties,
    },
    runtime_paint::{
        RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES, runtime_block_positioning_properties,
    },
};
use crate::StylesheetSource;

/// C12.4 Block/Flex 혼합 runtime cascade를 위해 합친 author declaration allowlist입니다.
pub(super) fn runtime_block_flex_stacking_author_properties() -> Vec<&'static str> {
    let mut properties = Vec::new();
    for property in RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES
        .iter()
        .chain(RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES)
        .copied()
        .chain(["z-index", "visibility"])
    {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

/// C12.4의 Block/Flex layout·position·paint 입력을 단일 cascade로 계산합니다.
pub fn compute_runtime_block_flex_stacking_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: spinon_core::StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = runtime_flex_wrap_properties();
    for property in runtime_block_positioning_properties() {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    properties.extend([
        ("z-index", LonghandId::ZIndex),
        ("visibility", LonghandId::Visibility),
    ]);
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1,
    )
}

/// C12.4에서 runtime layout/paint 입력 밖의 inline declaration을 찾습니다.
pub fn first_unsupported_runtime_block_flex_stacking_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_stacking_author_properties();
    first_unsupported_inline_property(view, |property| {
        property.starts_with("--") || allowed.contains(&property)
    })
}
