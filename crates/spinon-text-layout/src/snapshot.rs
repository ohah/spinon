use std::{ops::Range, sync::Arc};

use spinon_core::NodeId;

use crate::{LigatureMode, LineBreakMode, TextDirection};

/// 같은 font registry 입력에서 생성한 revision입니다.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FontRegistryRevision(pub u64);

/// 같은 font registry가 복제된 engine 사이에서 공유하는 identity입니다.
#[derive(Clone, Debug)]
pub(crate) struct FontRegistryToken(Arc<()>);

impl FontRegistryToken {
    pub(crate) fn new() -> Self {
        Self(Arc::new(()))
    }

    pub(crate) fn matches(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// shaped run에서 보존한 글꼴 metrics입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontMetricsSnapshot {
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
}

/// 원본 DOM UTF-16 범위와 공백 변환 후 UTF-8 범위의 연결입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextSourceRange {
    pub node: NodeId,
    pub source_utf16: Range<usize>,
    pub paragraph_utf8: Range<usize>,
    pub inline_ancestors: Vec<NodeId>,
}

/// CSS px 단위의 line content bounds입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 한 줄의 line box와 실제 shaped advance입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLine {
    pub text_range: Range<usize>,
    pub advance: f32,
    pub hanging_advance: f32,
    pub line_top: f32,
    pub line_height: f32,
    pub baseline: f32,
}

/// 한 grapheme/ligature cluster에 귀속한 실제 glyph입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct TextGlyph {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
    /// 이 glyph를 소유하는 grapheme 또는 ligature의 paragraph UTF-8 범위입니다.
    pub cluster_text_range: Range<usize>,
    /// 이 shaped cluster에 기여한 원본 DOM UTF-16 범위입니다.
    pub sources: Vec<TextSourceRange>,
}

/// 한 줄 안의 shaped font run입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    pub line_index: usize,
    pub text_range: Range<usize>,
    pub cluster_range: Range<usize>,
    /// shaper가 이 run에 선택한 Bidi 방향입니다.
    pub is_rtl: bool,
    /// ParagraphStyle 또는 가장 안쪽 inline owner가 요청한 family입니다.
    pub font_family: String,
    pub font_size: f32,
    /// Paragraph top 기준 run baseline과 inline text box line-height입니다.
    pub baseline: f32,
    pub line_height: f32,
    pub font_metrics: FontMetricsSnapshot,
    pub sources: Vec<TextSourceRange>,
    pub glyphs: Vec<TextGlyph>,
}

/// span 또는 text node의 한 줄 조각입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct InlineFragment {
    pub node: NodeId,
    pub line_index: usize,
    pub rect: TextRect,
}

/// Taffy measure와 최종 layout이 같은 width에서 사용해야 하는 immutable 결과입니다.
#[derive(Clone, Debug)]
pub struct TextLayoutSnapshot {
    pub paragraph: NodeId,
    pub font_registry_revision: FontRegistryRevision,
    pub(crate) font_registry_token: FontRegistryToken,
    pub available_width: f32,
    pub direction: TextDirection,
    pub line_break: LineBreakMode,
    pub ligatures: LigatureMode,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub sources: Vec<TextSourceRange>,
    pub lines: Vec<TextLine>,
    pub runs: Vec<TextRun>,
    pub inline_fragments: Vec<InlineFragment>,
}

impl PartialEq for TextLayoutSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph == other.paragraph
            && self.font_registry_revision == other.font_registry_revision
            && self.available_width == other.available_width
            && self.direction == other.direction
            && self.line_break == other.line_break
            && self.ligatures == other.ligatures
            && self.width == other.width
            && self.height == other.height
            && self.text == other.text
            && self.sources == other.sources
            && self.lines == other.lines
            && self.runs == other.runs
            && self.inline_fragments == other.inline_fragments
    }
}
