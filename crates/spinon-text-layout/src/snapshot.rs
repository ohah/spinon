use std::ops::Range;

use spinon_core::NodeId;

/// 같은 font registry 입력에서 생성한 revision입니다.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FontRegistryRevision(pub u64);

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
}

/// 한 줄 안의 shaped font run입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    pub line_index: usize,
    pub text_range: Range<usize>,
    pub cluster_range: Range<usize>,
    /// ParagraphStyle가 요청한 family이며 실제 fallback 식별은 C15.1 범위가 아닙니다.
    pub font_family: String,
    pub font_size: f32,
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
#[derive(Clone, Debug, PartialEq)]
pub struct TextLayoutSnapshot {
    pub paragraph: NodeId,
    pub font_registry_revision: FontRegistryRevision,
    pub available_width: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub sources: Vec<TextSourceRange>,
    pub lines: Vec<TextLine>,
    pub runs: Vec<TextRun>,
    pub inline_fragments: Vec<InlineFragment>,
}
