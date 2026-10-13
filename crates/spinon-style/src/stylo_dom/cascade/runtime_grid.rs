use spinon_core::{NodeId, StyleRevision};
use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    runtime_layout::{RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, runtime_flex_wrap_properties},
    runtime_paint::{
        RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES, runtime_block_positioning_properties,
    },
};
use crate::StylesheetSource;

/// C11.1에서 혼합 Block/Flex/Grid cascade에 허용하는 CSS 선언입니다.
pub(super) fn runtime_block_flex_grid_author_properties() -> Vec<&'static str> {
    let mut properties = Vec::new();
    for property in RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES
        .iter()
        .chain(RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES)
        .copied()
        .chain([
            "grid-template-columns",
            "grid-template-rows",
            "grid-auto-flow",
        ])
    {
        if property == "aspect-ratio" {
            continue;
        }
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_grid_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_flex_wrap_properties();
    for property in runtime_block_positioning_properties() {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    properties.extend([
        ("grid-template-columns", LonghandId::GridTemplateColumns),
        ("grid-template-rows", LonghandId::GridTemplateRows),
        ("grid-auto-flow", LonghandId::GridAutoFlow),
    ]);
    properties
}

pub(super) fn runtime_block_flex_grid_placement_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_author_properties();
    for property in [
        "grid-row-start",
        "grid-row-end",
        "grid-column-start",
        "grid-column-end",
        "grid-row",
        "grid-column",
        "grid-area",
    ] {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_grid_placement_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_properties();
    properties.extend([
        ("grid-row-start", LonghandId::GridRowStart),
        ("grid-row-end", LonghandId::GridRowEnd),
        ("grid-column-start", LonghandId::GridColumnStart),
        ("grid-column-end", LonghandId::GridColumnEnd),
    ]);
    properties
}

pub(super) fn runtime_block_flex_grid_auto_placement_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_placement_author_properties();
    properties.push("grid-auto-rows");
    properties
}

pub(super) fn runtime_block_flex_grid_auto_placement_properties() -> Vec<(&'static str, LonghandId)>
{
    let mut properties = runtime_block_flex_grid_placement_properties();
    properties.push(("grid-auto-rows", LonghandId::GridAutoRows));
    properties
}

pub(super) fn runtime_block_flex_grid_dense_auto_placement_author_properties() -> Vec<&'static str>
{
    let mut properties = runtime_block_flex_grid_auto_placement_author_properties();
    properties.push("grid-auto-columns");
    properties
}

pub(super) fn runtime_block_flex_grid_dense_auto_placement_properties()
-> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_auto_placement_properties();
    properties.push(("grid-auto-columns", LonghandId::GridAutoColumns));
    properties
}

pub(super) fn runtime_block_flex_grid_template_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_dense_auto_placement_author_properties();
    properties.extend(["grid-template", "grid", "grid-template-areas"]);
    properties
}

pub(super) fn runtime_block_flex_grid_template_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_dense_auto_placement_properties();
    properties.push(("grid-template-areas", LonghandId::GridTemplateAreas));
    properties
}

/// C11.1 explicit Grid tracks와 기존 Block/Flex 입력을 같은 cascade에서 계산합니다.
pub fn compute_runtime_block_flex_grid_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridV1,
    )
}

/// C11.2.1 numeric explicit placement을 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1,
    )
}

/// C11.2.2 sparse row auto-placement를 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_auto_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
    )
}

/// C11.2.3 dense·column auto-placement를 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_dense_auto_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1,
    )
}

/// C11.3.1의 정수 `repeat()` 확장과 line-name metadata를 계산하는 누적 Grid profile입니다.
pub fn compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_template_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
}

/// C11.1 inline Grid cascade 밖 선언을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.1 inline cascade에서 미지원 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.2 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_auto_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.3 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_dense_auto_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.3.1 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_template_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_template_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}
