use spinon_core::{NodeId, StyleRevision};
use style::properties::{ComputedValues, LonghandId, PropertyDeclarationId};

use crate::{
    ComputedBackgroundPaint, ComputedBorderPaint, ComputedCssColor, ComputedSrgbColor, CssOrigin,
    OpaqueCssSrgb, StylesheetSource,
};

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

pub(super) const RUNTIME_ROOT_HTML_BLOCK_PAINT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "box-sizing",
    "width",
    "height",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "background-color",
    "color-scheme",
];

pub(super) const RUNTIME_ROOT_HTML_TEXT_PAINT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
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
    "border-style",
    "border-top-width",
    "border-right-width",
    "border-bottom-width",
    "border-left-width",
    "border-top-style",
    "border-right-style",
    "border-bottom-style",
    "border-left-style",
    "background-color",
    "color",
    "color-scheme",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "line-height",
    "white-space",
    "white-space-collapse",
    "text-wrap-mode",
    "word-break",
    "overflow-wrap",
    "letter-spacing",
    "word-spacing",
    "text-indent",
    "text-transform",
    "text-align",
    "direction",
    "writing-mode",
    "grid-template-columns",
    "grid-template-rows",
    "grid-template-areas",
    "grid-auto-flow",
    "grid-auto-rows",
    "grid-auto-columns",
    "grid-row-start",
    "grid-row-end",
    "grid-column-start",
    "grid-column-end",
    "grid-row",
    "grid-column",
    "grid-area",
    "gap",
    "row-gap",
    "column-gap",
    "overflow",
    "overflow-x",
    "overflow-y",
];

const ROOT_HTML_BLOCK_PAINT_UA_STYLESHEET: &str = r#"
@namespace "http://www.w3.org/1999/xhtml";
html { display: block; }
body { display: block; margin: 8px; }
main, div { display: block; }
p {
    display: block;
    margin-block-start: 1em;
    margin-block-end: 1em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h1 {
    display: block;
    font-size: 2em;
    font-weight: bold;
    margin-block-start: 0.67em;
    margin-block-end: 0.67em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h2 {
    display: block;
    font-size: 1.5em;
    font-weight: bold;
    margin-block-start: 0.83em;
    margin-block-end: 0.83em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h3 {
    display: block;
    font-size: 1.17em;
    font-weight: bold;
    margin-block-start: 1em;
    margin-block-end: 1em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h4 {
    display: block;
    font-weight: bold;
    margin-block-start: 1.33em;
    margin-block-end: 1.33em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h5 {
    display: block;
    font-size: 0.83em;
    font-weight: bold;
    margin-block-start: 1.67em;
    margin-block-end: 1.67em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
h6 {
    display: block;
    font-size: 0.67em;
    font-weight: bold;
    margin-block-start: 2.33em;
    margin-block-end: 2.33em;
    margin-inline-start: 0;
    margin-inline-end: 0;
}
head, base, meta, title, link, script, style { display: none; }
strong, b { font-weight: bolder; }
em, i { font-style: italic; }
"#;

pub(super) const RUNTIME_BLOCK_IMAGE_AUTHOR_PROPERTIES: &[&str] = &[
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
    "object-fit",
    "object-position",
    "image-rendering",
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

pub(super) fn c19_computed_color_properties() -> Vec<(&'static str, LonghandId)> {
    vec![
        ("background-color", LonghandId::BackgroundColor),
        ("color", LonghandId::Color),
    ]
}

pub(super) fn runtime_root_html_block_paint_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_paint_properties();
    properties.push(("color-scheme", LonghandId::ColorScheme));
    properties
}

pub(super) fn runtime_root_html_text_paint_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_root_html_block_paint_properties();
    properties.extend([
        ("min-width", LonghandId::MinWidth),
        ("max-width", LonghandId::MaxWidth),
        ("min-height", LonghandId::MinHeight),
        ("max-height", LonghandId::MaxHeight),
        ("padding-top", LonghandId::PaddingTop),
        ("padding-right", LonghandId::PaddingRight),
        ("padding-bottom", LonghandId::PaddingBottom),
        ("padding-left", LonghandId::PaddingLeft),
        ("border-top-width", LonghandId::BorderTopWidth),
        ("border-right-width", LonghandId::BorderRightWidth),
        ("border-bottom-width", LonghandId::BorderBottomWidth),
        ("border-left-width", LonghandId::BorderLeftWidth),
        ("border-top-style", LonghandId::BorderTopStyle),
        ("border-right-style", LonghandId::BorderRightStyle),
        ("border-bottom-style", LonghandId::BorderBottomStyle),
        ("border-left-style", LonghandId::BorderLeftStyle),
        ("font-size", LonghandId::FontSize),
        ("font-weight", LonghandId::FontWeight),
        ("font-style", LonghandId::FontStyle),
        ("line-height", LonghandId::LineHeight),
        ("white-space-collapse", LonghandId::WhiteSpaceCollapse),
        ("text-wrap-mode", LonghandId::TextWrapMode),
        ("word-break", LonghandId::WordBreak),
        ("overflow-wrap", LonghandId::OverflowWrap),
        ("letter-spacing", LonghandId::LetterSpacing),
        ("word-spacing", LonghandId::WordSpacing),
        ("text-indent", LonghandId::TextIndent),
        ("text-transform", LonghandId::TextTransform),
        ("text-align", LonghandId::TextAlign),
        ("line-break", LonghandId::LineBreak),
        ("direction", LonghandId::Direction),
        ("writing-mode", LonghandId::WritingMode),
        ("overflow-x", LonghandId::OverflowX),
        ("overflow-y", LonghandId::OverflowY),
        ("grid-template-columns", LonghandId::GridTemplateColumns),
        ("grid-template-rows", LonghandId::GridTemplateRows),
        ("grid-template-areas", LonghandId::GridTemplateAreas),
        ("row-gap", LonghandId::RowGap),
        ("column-gap", LonghandId::ColumnGap),
        ("grid-auto-flow", LonghandId::GridAutoFlow),
        ("grid-auto-rows", LonghandId::GridAutoRows),
        ("grid-auto-columns", LonghandId::GridAutoColumns),
        ("grid-row-start", LonghandId::GridRowStart),
        ("grid-row-end", LonghandId::GridRowEnd),
        ("grid-column-start", LonghandId::GridColumnStart),
        ("grid-column-end", LonghandId::GridColumnEnd),
    ]);
    properties.sort_by_key(|(name, _)| *name);
    properties.dedup_by_key(|(name, _)| *name);
    properties
}

pub(super) fn runtime_block_image_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_paint_properties();
    properties.extend([
        ("object-fit", LonghandId::ObjectFit),
        ("object-position", LonghandId::ObjectPosition),
        ("image-rendering", LonghandId::ImageRendering),
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
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
        | ComputedStyleProfile::RuntimeBlockPaintV1
        | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
        | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        | ComputedStyleProfile::RuntimeBlockImageV1
        | ComputedStyleProfile::RuntimeBlockFormattingV1
        | ComputedStyleProfile::RuntimeBlockPositioningV1
        | ComputedStyleProfile::RuntimeBlockFlexStackingV1
        | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        | ComputedStyleProfile::C19BoxPaintOpacityV1
        | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
        | ComputedStyleProfile::RuntimeBlockFlexGridV1
        | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
        | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
            let paint = match profile {
                ComputedStyleProfile::C19BoxPaintOpacityV1 => {
                    computed_c19_runtime_background_paint(computed, node)?
                }
                ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1 => {
                    computed_root_html_background_paint(computed, node)?
                }
                ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1 => {
                    computed_root_html_block_background_paint(computed, node)?
                }
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 => {
                    computed_root_html_block_background_paint(computed, node)?
                }
                _ => computed_runtime_background_paint(computed, node)?,
            };
            let color = match paint {
                ComputedBackgroundPaint::Transparent => None,
                ComputedBackgroundPaint::Opaque(color) => Some(color),
                ComputedBackgroundPaint::Translucent(_) => None,
            };
            Ok((color, Some(paint)))
        }
        ComputedStyleProfile::C19ComputedColorV1 => Ok((None, None)),
        _ => Ok((None, None)),
    }
}

pub(super) fn computed_color_for_profile(
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node: NodeId,
) -> Result<(Option<ComputedCssColor>, Option<ComputedCssColor>), CssCascadeError> {
    if !matches!(
        profile,
        ComputedStyleProfile::C19ComputedColorV1 | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
    ) {
        return Ok((None, None));
    }

    let computed_color =
        ComputedCssColor::from_absolute_color(computed.clone_color()).map_err(|reason| {
            CssCascadeError::UnsupportedComputedCssColor {
                node,
                property: "color",
                reason: reason.to_owned(),
            }
        })?;
    if profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 {
        return Ok((Some(computed_color), None));
    }

    let background = match computed.clone_background_color() {
        style::values::computed::Color::Absolute(color) => color,
        style::values::computed::Color::CurrentColor => computed.clone_color(),
        _ => {
            return Err(CssCascadeError::UnsupportedComputedCssColor {
                node,
                property: "background-color",
                reason: "Stylo computed color가 절대 색상 또는 currentColor가 아닙니다".to_owned(),
            });
        }
    };
    let computed_background_color =
        ComputedCssColor::from_absolute_color(background).map_err(|reason| {
            CssCascadeError::UnsupportedComputedCssColor {
                node,
                property: "background-color",
                reason: reason.to_owned(),
            }
        })?;
    Ok((Some(computed_color), Some(computed_background_color)))
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

/// C04.12의 parsed root HTML document를 제한 Block paint profile로 계산합니다.
pub fn compute_runtime_root_html_block_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_root_html_block_paint_properties();
    let user_agent_stylesheets = [StylesheetSource {
        id: "ua:root-html-block-paint-v0".to_owned(),
        base_url: "https://spinon.invalid/ua/root-html-block-paint-v0.css".to_owned(),
        origin: CssOrigin::UserAgent,
        css: ROOT_HTML_BLOCK_PAINT_UA_STYLESHEET.to_owned(),
    }];
    super::compute_cascade_with_additional_user_agent_stylesheets(
        view,
        &user_agent_stylesheets,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1,
    )
}

/// C04.12의 parsed root HTML 고정 direct-text paint computed values를 계산합니다.
pub fn compute_runtime_root_html_text_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_root_html_text_paint_properties();
    let user_agent_stylesheets = [StylesheetSource {
        id: "ua:root-html-text-paint-v0".to_owned(),
        base_url: "https://spinon.invalid/ua/root-html-text-paint-v0.css".to_owned(),
        origin: CssOrigin::UserAgent,
        css: ROOT_HTML_BLOCK_PAINT_UA_STYLESHEET.to_owned(),
    }];
    super::compute_cascade_with_additional_user_agent_stylesheets(
        view,
        &user_agent_stylesheets,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
}

/// C19.1의 `color`와 `background-color`를 Stylo typed carrier로 계산합니다.
pub fn compute_c19_computed_color_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = c19_computed_color_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::C19ComputedColorV1,
    )
}

/// C18.3의 image object properties와 Block paint를 같은 cascade에서 계산합니다.
pub fn compute_runtime_block_image_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let properties = runtime_block_image_properties();
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockImageV1,
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

fn computed_root_html_background_paint(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<ComputedBackgroundPaint, CssCascadeError> {
    let color = match computed.clone_background_color() {
        style::values::computed::Color::Absolute(color) => color,
        _ => {
            return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
                node,
                reason: "Stylo 계산값이 절대 색상이 아닙니다".to_owned(),
            });
        }
    };
    let converted = ComputedCssColor::from_absolute_color(color).map_err(|reason| {
        CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: reason.to_owned(),
        }
    })?;
    if converted.color_space != crate::ComputedCssColorSpace::Srgb {
        return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "root HTML Canvas profile은 sRGB 색상만 지원합니다".to_owned(),
        });
    }
    let [red, green, blue, alpha] = converted.converted_srgb;
    if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
        return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "root HTML Canvas alpha가 유한한 0..1 값이 아닙니다".to_owned(),
        });
    }
    if alpha == 0.0 {
        return Ok(ComputedBackgroundPaint::Transparent);
    }
    if [red, green, blue]
        .iter()
        .any(|channel| !channel.is_finite() || !(0.0..=1.0).contains(channel))
    {
        return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "root HTML Canvas sRGB channel이 0..1 범위를 벗어났습니다".to_owned(),
        });
    }
    if alpha == 1.0 {
        let channel_byte = |value: f32| {
            let scaled = value * 255.0;
            let rounded = scaled.round();
            ((scaled - rounded).abs() <= 0.0001).then_some(rounded as u8)
        };
        let Some(red) = channel_byte(red) else {
            return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
                node,
                reason: "root HTML Canvas red channel은 8-bit 경계에 맞지 않습니다".to_owned(),
            });
        };
        let Some(green) = channel_byte(green) else {
            return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
                node,
                reason: "root HTML Canvas green channel은 8-bit 경계에 맞지 않습니다".to_owned(),
            });
        };
        let Some(blue) = channel_byte(blue) else {
            return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
                node,
                reason: "root HTML Canvas blue channel은 8-bit 경계에 맞지 않습니다".to_owned(),
            });
        };
        return Ok(ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
            red,
            green,
            blue,
        }));
    }
    Ok(ComputedBackgroundPaint::Translucent(ComputedSrgbColor {
        red,
        green,
        blue,
        alpha,
    }))
}

fn computed_root_html_block_background_paint(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<ComputedBackgroundPaint, CssCascadeError> {
    let paint = computed_root_html_background_paint(computed, node)?;
    if matches!(paint, ComputedBackgroundPaint::Translucent(_)) {
        return Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "root HTML Block paint는 부분 alpha를 지원하지 않습니다".to_owned(),
        });
    }
    Ok(paint)
}

pub(super) fn computed_c19_runtime_background_paint(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<ComputedBackgroundPaint, CssCascadeError> {
    match computed.clone_background_color() {
        style::values::computed::Color::Absolute(color) => {
            OpaqueCssSrgb::c19_runtime_paint_from_absolute_color(color).map_err(|reason| {
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

pub(super) fn computed_runtime_border_paint(
    computed: &ComputedValues,
    node: NodeId,
    border: super::ComputedLayoutBorder,
) -> Result<ComputedBorderPaint, CssCascadeError> {
    computed_border_paint(
        computed,
        node,
        border,
        OpaqueCssSrgb::runtime_paint_from_absolute_color,
    )
}

pub(super) fn computed_c19_runtime_border_paint(
    computed: &ComputedValues,
    node: NodeId,
    border: super::ComputedLayoutBorder,
) -> Result<ComputedBorderPaint, CssCascadeError> {
    computed_border_paint(
        computed,
        node,
        border,
        OpaqueCssSrgb::c19_runtime_paint_from_absolute_color,
    )
}

fn computed_border_paint(
    computed: &ComputedValues,
    node: NodeId,
    border: super::ComputedLayoutBorder,
    convert_color: fn(style::color::AbsoluteColor) -> Result<ComputedBackgroundPaint, &'static str>,
) -> Result<ComputedBorderPaint, CssCascadeError> {
    let side = |width: f32,
                style_property: LonghandId,
                color_property: &'static str,
                style_name: &'static str,
                color: style::values::computed::Color|
     -> Result<ComputedBackgroundPaint, CssCascadeError> {
        if width <= 0.0 {
            return Ok(ComputedBackgroundPaint::Transparent);
        }
        let style =
            computed.computed_value_to_string(PropertyDeclarationId::Longhand(style_property));
        if style != "solid" {
            return Err(CssCascadeError::UnsupportedComputedBorderStyle {
                node,
                property: style_name,
                value: style,
            });
        }
        let color = match color {
            style::values::computed::Color::Absolute(color) => color,
            style::values::computed::Color::CurrentColor => computed.clone_color(),
            _ => {
                return Err(CssCascadeError::UnsupportedComputedBorderColor {
                    node,
                    property: color_property,
                    reason: "Stylo 계산값이 절대 색상이 아닙니다".to_owned(),
                });
            }
        };
        convert_color(color).map_err(|reason| CssCascadeError::UnsupportedComputedBorderColor {
            node,
            property: color_property,
            reason: reason.to_owned(),
        })
    };

    Ok(ComputedBorderPaint {
        top: side(
            border.top,
            LonghandId::BorderTopStyle,
            "border-top-color",
            "border-top-style",
            computed.clone_border_top_color(),
        )?,
        right: side(
            border.right,
            LonghandId::BorderRightStyle,
            "border-right-color",
            "border-right-style",
            computed.clone_border_right_color(),
        )?,
        bottom: side(
            border.bottom,
            LonghandId::BorderBottomStyle,
            "border-bottom-color",
            "border-bottom-style",
            computed.clone_border_bottom_color(),
        )?,
        left: side(
            border.left,
            LonghandId::BorderLeftStyle,
            "border-left-color",
            "border-left-style",
            computed.clone_border_left_color(),
        )?,
    })
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

pub fn first_unsupported_runtime_root_html_block_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_ROOT_HTML_BLOCK_PAINT_AUTHOR_PROPERTIES.contains(&property)
    })
}

pub fn first_unsupported_runtime_root_html_text_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_ROOT_HTML_TEXT_PAINT_AUTHOR_PROPERTIES.contains(&property)
    })
}

pub fn first_unsupported_runtime_block_image_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        RUNTIME_BLOCK_IMAGE_AUTHOR_PROPERTIES.contains(&property)
    })
    .or_else(|| first_unsupported_runtime_block_image_object_position_inline_value(view))
}

fn first_unsupported_runtime_block_image_object_position_inline_value(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let guard = view.shared_lock().read();
    let mut pending = vec![view.root_handle()];
    while let Some(handle) = pending.pop() {
        if let Some(element) = view.element(handle)
            && let Some(inline_style) = element.data().inline_style.as_ref()
        {
            let block = inline_style.read_with(&guard);
            for declaration in block.declarations() {
                if declaration.id().name().as_ref() != "object-position" {
                    continue;
                }
                let mut value = String::new();
                if declaration.to_css(&mut value).is_err()
                    || !supports_runtime_image_object_position(&value)
                {
                    return Some((handle.id(), "object-position".to_owned()));
                }
            }
        }
        if let Some(children) = view.snapshot().children(handle) {
            let children = children
                .filter(|child| view.is_member(*child))
                .collect::<Vec<_>>();
            pending.extend(children.into_iter().rev());
        }
    }
    None
}

pub(crate) fn supports_runtime_image_object_position(value: &str) -> bool {
    let mut components = value.split_whitespace();
    let Some(x) = components.next() else {
        return false;
    };
    let Some(y) = components.next() else {
        return false;
    };
    if components.next().is_some() {
        return false;
    }
    [x, y].into_iter().all(|component| {
        component
            .strip_suffix('%')
            .and_then(|number| number.parse::<f32>().ok())
            .is_some_and(f32::is_finite)
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
