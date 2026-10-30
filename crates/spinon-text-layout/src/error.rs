use std::{error::Error, fmt, ops::Range};

use spinon_core::NodeId;

#[derive(Clone, Debug, PartialEq)]
pub enum TextLayoutError {
    InvalidWidth,
    InvalidStyle(&'static str),
    MissingFontFamily(String),
    InvalidFontData(String),
    FontRevisionOverflow,
    StaleFontRegistryRevision {
        expected: crate::FontRegistryRevision,
        actual: crate::FontRegistryRevision,
    },
    ForeignFontRegistrySnapshot,
    InvalidRasterScale,
    RasterFontSizeOutOfRange {
        paragraph: NodeId,
        font_family: String,
    },
    MissingRasterFont(String),
    InvalidRasterFont(String),
    GlyphRasterizationFailed {
        paragraph: NodeId,
        glyph_id: u32,
    },
    InvalidGlyphBitmap {
        paragraph: NodeId,
        glyph_id: u32,
    },
    RasterSnapshotTooLarge {
        paragraph: NodeId,
        max_bytes: usize,
    },
    RasterAllocationFailed(NodeId),
    InvalidGlyphLayout(NodeId),
    UnsupportedVerticalIntrinsicProfile(NodeId),
    UnsupportedPreWrapTabProfile(NodeId),
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
            Self::StaleFontRegistryRevision { expected, actual } => write!(
                formatter,
                "text snapshot font registry revision이 다릅니다: expected={}, actual={}",
                expected.0, actual.0
            ),
            Self::ForeignFontRegistrySnapshot => {
                formatter.write_str("text snapshot이 다른 font registry에서 만들어졌습니다")
            }
            Self::InvalidRasterScale => {
                formatter.write_str("glyph raster scale은 0보다 크고 64 이하인 유한값이어야 합니다")
            }
            Self::RasterFontSizeOutOfRange {
                paragraph,
                font_family,
            } => write!(
                formatter,
                "paragraph {paragraph}의 '{font_family}' raster pixel size가 지원 범위를 넘었습니다"
            ),
            Self::MissingRasterFont(family) => {
                write!(
                    formatter,
                    "'{family}' font raster source가 등록되지 않았습니다"
                )
            }
            Self::InvalidRasterFont(family) => {
                write!(
                    formatter,
                    "'{family}' font raster source를 읽을 수 없습니다"
                )
            }
            Self::GlyphRasterizationFailed {
                paragraph,
                glyph_id,
            } => write!(
                formatter,
                "paragraph {paragraph}의 glyph {glyph_id}를 rasterize하지 못했습니다"
            ),
            Self::InvalidGlyphBitmap {
                paragraph,
                glyph_id,
            } => write!(
                formatter,
                "paragraph {paragraph}의 glyph {glyph_id} bitmap 크기 또는 pixel 형식이 유효하지 않습니다"
            ),
            Self::RasterSnapshotTooLarge {
                paragraph,
                max_bytes,
            } => write!(
                formatter,
                "paragraph {paragraph}의 glyph raster snapshot이 {max_bytes} byte 제한을 넘었습니다"
            ),
            Self::RasterAllocationFailed(paragraph) => write!(
                formatter,
                "paragraph {paragraph}의 glyph raster 결과 메모리를 예약하지 못했습니다"
            ),
            Self::InvalidGlyphLayout(node) => {
                write!(
                    formatter,
                    "paragraph {node}의 shaping glyph, font metrics 또는 line geometry가 유효하지 않습니다"
                )
            }
            Self::UnsupportedVerticalIntrinsicProfile(node) => write!(
                formatter,
                "paragraph {node}의 vertical intrinsic 입력이 고정 Ahem profile과 일치하지 않습니다"
            ),
            Self::UnsupportedPreWrapTabProfile(node) => write!(
                formatter,
                "paragraph {node}의 pre-wrap tab 입력이 고정 Ahem 10px LTR XX-tab-XX fixture profile과 일치하지 않습니다"
            ),
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
