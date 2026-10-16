use std::{error::Error, fmt};

use spinon_core::NodeId;

#[derive(Clone, Debug, PartialEq)]
pub enum TextLayoutError {
    InvalidWidth,
    InvalidStyle(&'static str),
    MissingFontFamily(String),
    InvalidFontData(String),
    FontRevisionOverflow,
    InvalidGlyphLayout(NodeId),
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
            Self::UnexpectedInlineBox(node) => {
                write!(
                    formatter,
                    "paragraph {node}에 C15.1에서 지원하지 않는 inline box가 있습니다"
                )
            }
        }
    }
}

impl Error for TextLayoutError {}
