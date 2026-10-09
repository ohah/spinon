use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport, StyloDocumentView,
    compute_cascade,
};
use crate::StylesheetSource;
use spinon_core::StyleRevision;

const SUPPORTED_ELEMENTS_UA_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("list-style-type", LonghandId::ListStyleType),
    ("margin-block-start", LonghandId::MarginBlockStart),
    ("margin-block-end", LonghandId::MarginBlockEnd),
    ("margin-inline-start", LonghandId::MarginInlineStart),
    ("margin-inline-end", LonghandId::MarginInlineEnd),
    ("padding-inline-start", LonghandId::PaddingInlineStart),
];

/// 내장 지원 HTML UA 규칙과 author stylesheet를 적용한 computed-style snapshot을 만듭니다.
///
/// 이 profile은 `supported-elements-v0.css`가 다루는 구조 기본값만 계산합니다. 반환된
/// `inline`, `inline-block`, `list-item` 값의 화면 배치나 list marker 생성은 보장하지 않습니다.
pub fn compute_supported_elements_ua_cascade(
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
        SUPPORTED_ELEMENTS_UA_PROPERTIES,
        ComputedStyleProfile::SupportedElementsUaV1,
    )
}
