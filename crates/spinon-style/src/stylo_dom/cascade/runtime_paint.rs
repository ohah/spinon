use spinon_core::{NodeId, StyleRevision};
use style::properties::{ComputedValues, LonghandId};

use crate::{ComputedBackgroundPaint, OpaqueCssSrgb, StylesheetSource};

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    runtime_layout::{
        RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, RUNTIME_FLEX_LAYOUT_PROPERTIES,
        runtime_flex_wrap_properties,
    },
};

pub(super) const RUNTIME_BLOCK_PAINT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "position",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "box-sizing",
    "width",
    "height",
    "background-color",
    "color",
    "font-size",
    "font-family",
];

pub(super) const RUNTIME_BLOCK_FORMATTING_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "position",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "box-sizing",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "border",
    "border-width",
    "border-top-width",
    "border-right-width",
    "border-bottom-width",
    "border-left-width",
    "border-style",
    "border-top-style",
    "border-right-style",
    "border-bottom-style",
    "border-left-style",
    "border-color",
    "border-top-color",
    "border-right-color",
    "border-bottom-color",
    "border-left-color",
    "background-color",
    "color",
    "font-size",
    "font-family",
];

pub(super) const RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "position",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "direction",
    "box-sizing",
    "width",
    "height",
    "min-width",
    "max-width",
    "min-height",
    "max-height",
    "aspect-ratio",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "border",
    "border-width",
    "border-top-width",
    "border-right-width",
    "border-bottom-width",
    "border-left-width",
    "border-style",
    "border-top-style",
    "border-right-style",
    "border-bottom-style",
    "border-left-style",
    "border-color",
    "border-top-color",
    "border-right-color",
    "border-bottom-color",
    "border-left-color",
    "background-color",
    "color",
    "font-size",
    "font-family",
];

pub(super) fn runtime_block_paint_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_layout_properties();
    properties.extend([
        ("background-color", LonghandId::BackgroundColor),
        ("color", LonghandId::Color),
        ("font-family", LonghandId::FontFamily),
    ]);
    properties
}

pub(super) fn runtime_block_formatting_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_layout_properties();
    properties.extend([
        ("background-color", LonghandId::BackgroundColor),
        ("color", LonghandId::Color),
        ("font-family", LonghandId::FontFamily),
    ]);
    properties
}

pub(super) fn runtime_block_positioning_properties() -> Vec<(&'static str, LonghandId)> {
    runtime_block_formatting_properties()
}

fn runtime_block_layout_properties() -> Vec<(&'static str, LonghandId)> {
    RUNTIME_FLEX_LAYOUT_PROPERTIES
        .iter()
        .copied()
        .filter(|(name, _)| *name != "flex-wrap")
        .collect()
}

pub(super) fn computed_background_for_profile(
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node: NodeId,
) -> Result<(Option<OpaqueCssSrgb>, Option<ComputedBackgroundPaint>), CssCascadeError> {
    match profile {
        ComputedStyleProfile::S04FlexPaintV1 => {
            let color = super::s04::computed_background_color(computed, node)?;
            Ok((Some(color), Some(ComputedBackgroundPaint::Opaque(color))))
        }
        ComputedStyleProfile::RuntimeFlexPaintV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        | ComputedStyleProfile::RuntimeBlockPaintV1
        | ComputedStyleProfile::RuntimeBlockFormattingV1
        | ComputedStyleProfile::RuntimeBlockPositioningV1 => {
            let paint = computed_runtime_background_paint(computed, node)?;
            let color = match paint {
                ComputedBackgroundPaint::Transparent => None,
                ComputedBackgroundPaint::Opaque(color) => Some(color),
            };
            Ok((color, Some(paint)))
        }
        _ => Ok((None, None)),
    }
}

/// C08의 제한 Block 입력과 background paint를 같은 Stylo cascade에서 계산합니다.
pub fn compute_runtime_block_paint_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_runtime_block_paint_cascade_with_stylesheets(view, &[], viewport, style_revision)
}

/// C08의 제한 author stylesheet와 inline 입력을 같은 Stylo cascade에서 계산합니다.
pub fn compute_runtime_block_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_paint_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockPaintV1,
    )
}

/// C09.1의 Block 흐름·상자 속성과 기본 paint를 같은 Stylo cascade에서 계산합니다.
pub fn compute_runtime_block_formatting_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_formatting_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockFormattingV1,
    )
}

/// C12.2의 Block absolute positioning과 기존 Block 입력·기본 paint를 계산합니다.
pub fn compute_runtime_block_positioning_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_positioning_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockPositioningV1,
    )
}

/// C04.10 runtime Flex와 단색 배경 paint를 같은 Stylo cascade에서 계산합니다.
pub fn compute_runtime_flex_paint_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = super::runtime_layout::runtime_flex_wrap_properties();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        &[],
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexPaintV1,
    )
}

/// C05.1 사용자 지정 속성과 `var()`를 포함하는 runtime layout·paint snapshot을 계산합니다.
pub fn compute_runtime_flex_custom_properties_paint_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        view,
        &[],
        viewport,
        style_revision,
    )
}

/// C04.11 author stylesheet와 C05.1 사용자 지정 속성을 runtime layout·paint에서 계산합니다.
pub fn compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = runtime_flex_wrap_properties();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
    )
}

/// C05.2 stylesheet `@property` 등록을 포함하는 runtime layout·paint snapshot을 계산합니다.
pub fn compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = runtime_flex_wrap_properties();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
    )
}

pub(super) fn computed_runtime_background_paint(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<ComputedBackgroundPaint, CssCascadeError> {
    match computed.clone_background_color() {
        style::values::computed::Color::Absolute(color) => {
            OpaqueCssSrgb::runtime_paint_from_absolute_color(color).map_err(|reason| {
                CssCascadeError::UnsupportedComputedBackgroundColor {
                    node,
                    reason: reason.to_owned(),
                }
            })
        }
        _ => Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "Stylo 계산값이 절대 색상이 아닙니다".to_owned(),
        }),
    }
}

pub fn first_unsupported_runtime_flex_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        property == "background-color"
            || property == "flex-wrap"
            || property == "flex-flow"
            || RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES.contains(&property)
    })
}

pub fn first_unsupported_runtime_block_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_BLOCK_PAINT_AUTHOR_PROPERTIES.contains(&property)
    })
}

pub fn first_unsupported_runtime_block_formatting_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_BLOCK_FORMATTING_AUTHOR_PROPERTIES.contains(&property)
    })
}

pub fn first_unsupported_runtime_block_positioning_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES.contains(&property)
    })
}
