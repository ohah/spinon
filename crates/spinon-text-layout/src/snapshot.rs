use std::{ops::Range, sync::Arc};

use spinon_core::{NodeId, TextOrientation, WritingMode};

use crate::{FontStyleMode, LigatureMode, LineBreakMode, TextDirection};

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
#[derive(Clone, Debug)]
pub struct TextRun {
    pub line_index: usize,
    pub text_range: Range<usize>,
    pub cluster_range: Range<usize>,
    /// shaper가 이 run에 선택한 Bidi 방향입니다.
    pub is_rtl: bool,
    /// ParagraphStyle 또는 가장 안쪽 inline owner가 요청한 family입니다.
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: f32,
    pub font_style: FontStyleMode,
    /// Paragraph top 기준 run baseline과 inline text box line-height입니다.
    pub baseline: f32,
    pub line_height: f32,
    pub font_metrics: FontMetricsSnapshot,
    /// Parley가 이 face에 선택한 variation coordinates입니다.
    pub(crate) normalized_coords: Vec<i16>,
    /// Parley가 선택한 face가 요청 굵기보다 가벼울 때의 faux bold입니다.
    pub(crate) synthesis_embolden: bool,
    /// Parley가 선택한 face에 적용할 faux oblique 각도입니다.
    pub(crate) synthesis_skew_degrees: Option<f32>,
    pub sources: Vec<TextSourceRange>,
    pub glyphs: Vec<TextGlyph>,
    pub(crate) font_data: parley::FontData,
}

impl PartialEq for TextRun {
    fn eq(&self, other: &Self) -> bool {
        self.line_index == other.line_index
            && self.text_range == other.text_range
            && self.cluster_range == other.cluster_range
            && self.is_rtl == other.is_rtl
            && self.font_family == other.font_family
            && self.font_size == other.font_size
            && self.font_weight == other.font_weight
            && self.font_style == other.font_style
            && self.baseline == other.baseline
            && self.line_height == other.line_height
            && self.font_metrics == other.font_metrics
            && self.normalized_coords == other.normalized_coords
            && self.synthesis_embolden == other.synthesis_embolden
            && self.synthesis_skew_degrees == other.synthesis_skew_degrees
            && self.sources == other.sources
            && self.glyphs == other.glyphs
            && self.font_data.index == other.font_data.index
            && self.font_data.data.data() == other.font_data.data.data()
    }
}

/// span 또는 text node의 한 줄 조각입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct InlineFragment {
    pub node: NodeId,
    pub line_index: usize,
    pub rect: TextRect,
}

/// 같은 paragraph와 font registry에서 계산한 CSS intrinsic inline sizes입니다.
#[derive(Clone, Debug)]
pub struct TextIntrinsicInlineSizes {
    paragraph: NodeId,
    font_registry_revision: FontRegistryRevision,
    font_registry_token: FontRegistryToken,
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
}

/// Writing mode의 inline/block 축에서 측정한 CSS px intrinsic 크기입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogicalIntrinsicSize {
    pub inline_px: f32,
    pub block_px: f32,
}

/// 같은 paragraph와 font registry에서 계산한 논리축 min/max-content 크기입니다.
#[derive(Clone, Debug)]
pub struct TextIntrinsicLogicalSizes {
    paragraph: NodeId,
    font_registry_revision: FontRegistryRevision,
    font_registry_token: FontRegistryToken,
    writing_mode: WritingMode,
    text_orientation: TextOrientation,
    min_content: LogicalIntrinsicSize,
    max_content: LogicalIntrinsicSize,
}

impl TextIntrinsicLogicalSizes {
    pub(crate) fn new(
        paragraph: NodeId,
        font_registry_revision: FontRegistryRevision,
        font_registry_token: FontRegistryToken,
        writing_mode: WritingMode,
        text_orientation: TextOrientation,
        min_content: LogicalIntrinsicSize,
        max_content: LogicalIntrinsicSize,
    ) -> Self {
        Self {
            paragraph,
            font_registry_revision,
            font_registry_token,
            writing_mode,
            text_orientation,
            min_content,
            max_content,
        }
    }

    pub const fn paragraph(&self) -> NodeId {
        self.paragraph
    }

    pub const fn font_registry_revision(&self) -> FontRegistryRevision {
        self.font_registry_revision
    }

    pub const fn writing_mode(&self) -> WritingMode {
        self.writing_mode
    }

    pub const fn text_orientation(&self) -> TextOrientation {
        self.text_orientation
    }

    /// 두 결과가 같은 font registry의 같은 revision에서 계산됐는지 비교합니다.
    pub fn uses_same_font_registry_revision(&self, other: &Self) -> bool {
        self.font_registry_revision == other.font_registry_revision
            && self.font_registry_token.matches(&other.font_registry_token)
    }

    pub const fn min_content(&self) -> LogicalIntrinsicSize {
        self.min_content
    }

    pub const fn max_content(&self) -> LogicalIntrinsicSize {
        self.max_content
    }
}

impl PartialEq for TextIntrinsicLogicalSizes {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph == other.paragraph
            && self.uses_same_font_registry_revision(other)
            && self.writing_mode == other.writing_mode
            && self.text_orientation == other.text_orientation
            && self.min_content == other.min_content
            && self.max_content == other.max_content
    }
}

impl TextIntrinsicInlineSizes {
    pub(crate) fn new(
        paragraph: NodeId,
        font_registry_revision: FontRegistryRevision,
        font_registry_token: FontRegistryToken,
        min_content: f32,
        max_content: f32,
        min_content_height: f32,
        max_content_height: f32,
    ) -> Self {
        Self {
            paragraph,
            font_registry_revision,
            font_registry_token,
            min_content,
            max_content,
            min_content_height,
            max_content_height,
        }
    }

    pub const fn paragraph(&self) -> NodeId {
        self.paragraph
    }

    pub const fn font_registry_revision(&self) -> FontRegistryRevision {
        self.font_registry_revision
    }

    /// 두 결과가 같은 font registry의 같은 revision에서 계산됐는지 비교합니다.
    pub fn uses_same_font_registry_revision(&self, other: &Self) -> bool {
        self.font_registry_revision == other.font_registry_revision
            && self.font_registry_token.matches(&other.font_registry_token)
    }

    pub const fn min_content(&self) -> f32 {
        self.min_content
    }

    pub const fn max_content(&self) -> f32 {
        self.max_content
    }

    pub const fn min_content_height(&self) -> f32 {
        self.min_content_height
    }

    pub const fn max_content_height(&self) -> f32 {
        self.max_content_height
    }

    pub(crate) fn to_logical_sizes(
        &self,
        writing_mode: WritingMode,
        text_orientation: TextOrientation,
    ) -> TextIntrinsicLogicalSizes {
        TextIntrinsicLogicalSizes::new(
            self.paragraph,
            self.font_registry_revision,
            self.font_registry_token.clone(),
            writing_mode,
            text_orientation,
            LogicalIntrinsicSize {
                inline_px: self.min_content,
                block_px: self.min_content_height,
            },
            LogicalIntrinsicSize {
                inline_px: self.max_content,
                block_px: self.max_content_height,
            },
        )
    }
}

impl PartialEq for TextIntrinsicInlineSizes {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph == other.paragraph
            && self.uses_same_font_registry_revision(other)
            && self.min_content == other.min_content
            && self.max_content == other.max_content
            && self.min_content_height == other.min_content_height
            && self.max_content_height == other.max_content_height
    }
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
