use std::{error::Error, fmt, ops::Range};

use spinon_core::NodeId;

#[derive(Clone, Debug, PartialEq)]
pub enum TextLayoutError {
    InvalidWidth,
    InvalidStyle(&'static str),
    MissingFontFamily(String),
    InvalidFontData(String),
    FontRevisionOverflow,
    InvalidGlyphLayout(NodeId),
    MissingGlyphCoverage {
        paragraph: NodeId,
        text_range: Range<usize>,
    },
    InvalidInlineBoxStyle(NodeId),
    InlineBoxWithoutText(NodeId),
    InvalidInlineHierarchy(NodeId),
    UnsupportedInlineDepth(NodeId),
    InlineBoxIdOverflow,
    UnexpectedInlineBox(NodeId),
}

impl fmt::Display for TextLayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWidth => write!(
                formatter,
                "text available width는 0 이상의 유한값이어야 합니다"
            ),
            Self::InvalidStyle(property) => {
                write!(
                    formatter,
                    "text paragraph의 {property} 값이 유효하지 않습니다"
                )
            }
            Self::MissingFontFamily(family) => {
                write!(formatter, "고정 font registry에 '{family}' 글꼴이 없습니다")
            }
            Self::InvalidFontData(family) => {
                write!(
                    formatter,
                    "font registry가 '{family}' 글꼴 데이터를 등록하지 못했습니다"
                )
            }
            Self::FontRevisionOverflow => write!(formatter, "font registry revision이 넘쳤습니다"),
            Self::InvalidGlyphLayout(node) => {
                write!(
                    formatter,
                    "paragraph {node}의 shaping glyph, font metrics 또는 line geometry가 유효하지 않습니다"
                )
            }
            Self::MissingGlyphCoverage {
                paragraph,
                text_range,
            } => write!(
                formatter,
                "paragraph {paragraph}의 UTF-8 범위 {text_range:?}에 고정 글꼴 glyph가 없습니다"
            ),
            Self::InvalidInlineBoxStyle(node) => write!(
                formatter,
                "paragraph {node}의 inline border/padding 값이나 owner가 유효하지 않습니다"
            ),
            Self::InlineBoxWithoutText(node) => write!(
                formatter,
                "inline node {node}의 border/padding을 연결할 visible text 범위가 없습니다"
            ),
            Self::InvalidInlineHierarchy(node) => write!(
                formatter,
                "inline node {node}가 일관되지 않은 ancestor 경로에 나타납니다"
            ),
            Self::UnsupportedInlineDepth(node) => write!(
                formatter,
                "inline node {node}가 C15.2a의 두 단계 nesting 한도를 넘었습니다"
            ),
            Self::InlineBoxIdOverflow => {
                write!(formatter, "inline box ID 범위가 넘쳤습니다")
            }
            Self::UnexpectedInlineBox(node) => {
                write!(
                    formatter,
                    "paragraph {node}에 입력으로 등록되지 않은 inline box가 있습니다"
                )
            }
        }
    }
}

impl Error for TextLayoutError {}
