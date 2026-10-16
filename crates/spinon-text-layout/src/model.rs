use spinon_core::NodeId;

/// C15.1에서 고정하는 공백 처리 profile입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhiteSpaceMode {
    Normal,
    NoWrap,
    PreLine,
}

/// C15.1에서 고정하는 단어 경계 profile입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordBreakMode {
    Normal,
    BreakAll,
}

/// 계산된 cascade를 대신하지 않는 내부 fixture 글꼴 입력입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct ParagraphStyle {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub white_space: WhiteSpaceMode,
    pub word_break: WordBreakMode,
}

impl ParagraphStyle {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.font_family.trim().is_empty() {
            return Err("font-family");
        }
        if !self.font_size.is_finite() || self.font_size <= 0.0 {
            return Err("font-size");
        }
        if !self.line_height.is_finite() || self.line_height <= 0.0 {
            return Err("line-height");
        }
        Ok(())
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
}
