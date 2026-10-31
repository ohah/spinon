use spinon_core::{NodeId, TextOrientation, WritingMode};

/// 제한된 inline span의 CSS px 면별 크기입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InlineEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// C15.2a에서만 쓰는 고정 px inline border와 padding입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InlineBoxStyle {
    pub border: InlineEdges,
    pub padding: InlineEdges,
}

impl InlineBoxStyle {
    pub(crate) fn is_empty(self) -> bool {
        self.border.is_zero() && self.padding.is_zero()
    }

    pub(crate) fn is_valid(self) -> bool {
        [
            self.border.top,
            self.border.right,
            self.border.bottom,
            self.border.left,
            self.padding.top,
            self.padding.right,
            self.padding.bottom,
            self.padding.left,
        ]
        .into_iter()
        .all(|edge| edge.is_finite() && edge >= 0.0)
            && self.inline_start().is_finite()
            && self.inline_end().is_finite()
            && self.block_start().is_finite()
            && self.block_end().is_finite()
    }

    pub(crate) fn inline_start(self) -> f32 {
        self.border.left + self.padding.left
    }

    pub(crate) fn inline_end(self) -> f32 {
        self.border.right + self.padding.right
    }

    pub(crate) fn block_start(self) -> f32 {
        self.border.top + self.padding.top
    }

    pub(crate) fn block_end(self) -> f32 {
        self.border.bottom + self.padding.bottom
    }
}

impl InlineEdges {
    fn is_zero(self) -> bool {
        self.top == 0.0 && self.right == 0.0 && self.bottom == 0.0 && self.left == 0.0
    }
}

/// C15.1에서 고정하는 공백 처리 profile입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WhiteSpaceMode {
    Normal,
    NoWrap,
    Pre,
    /// 제한된 `pre` tab profile에서 computed CSS `tab-size`를 전달합니다.
    PreWithTabSize(TabSize),
    PreLine,
    PreWrap,
    BreakSpaces,
}

/// C11.4 익명 Grid `pre` fixture에서 허용하는 computed CSS `tab-size` 값입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabSize {
    Number(f32),
    LengthPx(f32),
}

impl TabSize {
    pub(crate) fn is_valid(self) -> bool {
        match self {
            Self::Number(value) | Self::LengthPx(value) => value.is_finite() && value >= 0.0,
        }
    }

    pub(crate) fn interval_px(self, space_advance: f32) -> f32 {
        match self {
            Self::Number(value) => value * space_advance,
            Self::LengthPx(value) => value,
        }
    }
}

impl Default for TabSize {
    fn default() -> Self {
        Self::Number(8.0)
    }
}

/// C15.1에서 고정하는 단어 경계 profile입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordBreakMode {
    Normal,
    BreakAll,
}

/// C15.3의 고정 paragraph base direction입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
    #[default]
    Auto,
    Ltr,
    Rtl,
}

/// C15.3의 고정 Unicode line-break strictness입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineBreakMode {
    Normal,
    #[default]
    Strict,
}

/// C15.3의 고정 OpenType ligature 선택입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LigatureMode {
    #[default]
    Normal,
    None,
}

/// C04.12 text runtime에서 CSS `font-style`이 계산한 범위입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FontStyleMode {
    #[default]
    Normal,
    Italic,
    Oblique(f32),
}

impl FontStyleMode {
    fn is_valid(self) -> bool {
        match self {
            Self::Normal | Self::Italic => true,
            Self::Oblique(angle) => angle.is_finite() && (-90.0..=90.0).contains(&angle),
        }
    }
}

/// 계산된 cascade를 대신하지 않는 내부 fixture 글꼴 입력입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct ParagraphStyle {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub line_height_is_normal: bool,
    pub font_weight: f32,
    pub font_style: FontStyleMode,
    /// 제한 text profile에서 사용하는 computed CSS `letter-spacing` (CSS px)입니다.
    pub letter_spacing: f32,
    pub white_space: WhiteSpaceMode,
    pub word_break: WordBreakMode,
    pub direction: TextDirection,
    /// Paragraph의 computed writing mode입니다. Glyph rotation/painting은 담당하지 않습니다.
    pub writing_mode: WritingMode,
    /// Paragraph의 computed text orientation입니다. Glyph rotation/painting은 담당하지 않습니다.
    pub text_orientation: TextOrientation,
    pub line_break: LineBreakMode,
    pub ligatures: LigatureMode,
}

impl ParagraphStyle {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.font_family.trim().is_empty() {
            return Err("font-family");
        }
        if !self.font_size.is_finite() || self.font_size <= 0.0 {
            return Err("font-size");
        }
        if !self.line_height_is_normal && (!self.line_height.is_finite() || self.line_height < 0.0)
        {
            return Err("line-height");
        }
        if !self.font_weight.is_finite() || !(1.0..=1000.0).contains(&self.font_weight) {
            return Err("font-weight");
        }
        if !self.font_style.is_valid() {
            return Err("font-style");
        }
        if !self.letter_spacing.is_finite() {
            return Err("letter-spacing");
        }
        if let WhiteSpaceMode::PreWithTabSize(tab_size) = self.white_space
            && !tab_size.is_valid()
        {
            return Err("tab-size");
        }
        Ok(())
    }
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            font_family: String::new(),
            font_size: 0.0,
            line_height: 0.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: FontStyleMode::Normal,
            letter_spacing: 0.0,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: TextDirection::Auto,
            writing_mode: WritingMode::HorizontalTb,
            text_orientation: TextOrientation::Mixed,
            line_break: LineBreakMode::default(),
            ligatures: LigatureMode::default(),
        }
    }
}

/// C15.2b fixture에서 하나의 inline owner가 상속할 글꼴 속성입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct InlineRunStyle {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub line_height_is_normal: bool,
    pub font_weight: f32,
    pub font_style: FontStyleMode,
}

impl InlineRunStyle {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.font_family.trim().is_empty() {
            return Err("font-family");
        }
        if !self.font_size.is_finite() || self.font_size <= 0.0 {
            return Err("font-size");
        }
        if !self.line_height_is_normal && (!self.line_height.is_finite() || self.line_height <= 0.0)
        {
            return Err("line-height");
        }
        if !self.font_weight.is_finite() || !(1.0..=1000.0).contains(&self.font_weight) {
            return Err("font-weight");
        }
        if !self.font_style.is_valid() {
            return Err("font-style");
        }
        Ok(())
    }
}

impl Default for InlineRunStyle {
    fn default() -> Self {
        Self {
            font_family: String::new(),
            font_size: 0.0,
            line_height: 0.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: FontStyleMode::Normal,
        }
    }
}

/// 텍스트 노드와 그 텍스트를 감싸는 상속형 inline span 경로입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextSource {
    pub node: NodeId,
    pub inline_ancestors: Vec<NodeId>,
    pub text: String,
}

/// 하나의 Block 안에서 source order로 연결할 paragraph입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct ParagraphInput {
    pub node: NodeId,
    pub sources: Vec<TextSource>,
    pub style: ParagraphStyle,
    /// 장식된 inline owner의 고정 px border/padding만 저장합니다.
    pub inline_styles: std::collections::BTreeMap<NodeId, InlineBoxStyle>,
    /// C15.2b에서 명시한 inline owner의 family/size/line-height override입니다.
    pub inline_run_styles: std::collections::BTreeMap<NodeId, InlineRunStyle>,
}
