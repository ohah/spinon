use spinon_core::StyleRevision;
use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport, StylesheetSource,
    StyloDocumentView, compute_cascade,
};

pub(super) const FLEX_MARGIN_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("box-sizing", LonghandId::BoxSizing),
    ("width", LonghandId::Width),
    ("height", LonghandId::Height),
    ("flex-direction", LonghandId::FlexDirection),
    ("flex-grow", LonghandId::FlexGrow),
    ("flex-shrink", LonghandId::FlexShrink),
    ("flex-basis", LonghandId::FlexBasis),
    ("direction", LonghandId::Direction),
    ("row-gap", LonghandId::RowGap),
    ("column-gap", LonghandId::ColumnGap),
    ("margin-top", LonghandId::MarginTop),
    ("margin-right", LonghandId::MarginRight),
    ("margin-bottom", LonghandId::MarginBottom),
    ("margin-left", LonghandId::MarginLeft),
];

pub(super) const FLEX_MARGIN_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "box-sizing",
    "width",
    "height",
    "flex-direction",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "direction",
    "row-gap",
    "column-gap",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "margin-block-start",
    "margin-block-end",
    "margin-inline-start",
    "margin-inline-end",
];

/// C04.6 Flex adapter가 사용하는 margin 포함 computed-style snapshot을 계산합니다.
pub fn compute_flex_margin_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        FLEX_MARGIN_PROPERTIES,
        ComputedStyleProfile::FlexMarginV1,
    )
}

/// C04.7의 scheme·pointer media query를 평가하는 제한 computed-style snapshot입니다.
pub fn compute_flex_media_environment_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        FLEX_MARGIN_PROPERTIES,
        ComputedStyleProfile::FlexMediaEnvironmentV1,
    )
}
