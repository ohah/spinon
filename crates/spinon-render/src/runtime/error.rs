use std::{error::Error, fmt};

use spinon_core::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRenderError {
    DuplicateNode(NodeId),
    InvalidPaintOrder,
    InvalidPaintCommands,
    InvalidHitTestOrder,
    InvalidClip,
    InvalidPageScaleTransform,
    InvalidBorderWidth,
    InvalidPaintColor,
    InvalidScrollState,
    DuplicateScrollOwner(NodeId),
    StaleScrollRevision,
    DuplicateScrollRequest(NodeId),
    InvalidScrollRequest(NodeId),
    InvalidScrollTarget(NodeId),
    InvalidScrollPoint,
    InvalidScrollDelta,
    ScrollRevisionExhausted,
    DuplicateImage(NodeId),
    ImageNodeMissing(NodeId),
    InvalidImageSource(NodeId),
    InvalidImageBytes(NodeId),
    InvalidImageFrame(NodeId),
    InvalidImageGeometry(NodeId),
    InvalidImageStyleRevision(NodeId),
    UnsupportedImageBorder(NodeId),
    InvalidTextGeometry,
    InvalidTextBitmap,
    InvalidTextSource(NodeId),
    TextOwnerMissing(NodeId),
    DuplicateText(NodeId),
    UnsupportedTextScroll(NodeId),
}

impl fmt::Display for RuntimeRenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode(node) => write!(formatter, "runtime 렌더 노드 {node}가 중복됩니다"),
            Self::InvalidPaintOrder => {
                formatter.write_str("runtime paint_order가 장면 순서의 0부터 연속값이 아닙니다")
            }
            Self::InvalidPaintCommands => formatter
                .write_str("runtime paint command의 box·text 순서 또는 group 경계가 맞지 않습니다"),
            Self::InvalidHitTestOrder => {
                formatter.write_str("runtime hit_test_order가 중복되거나 장면 범위를 벗어났습니다")
            }
            Self::InvalidClip => {
                formatter.write_str("runtime clip은 유한하고 순서가 맞는 경계여야 합니다")
            }
            Self::InvalidPageScaleTransform => formatter.write_str(
                "runtime page-scale transform의 DPR, page scale 또는 곱이 유효하지 않습니다",
            ),
            Self::InvalidBorderWidth => {
                formatter.write_str("runtime border 폭은 유한한 0 이상의 CSS px여야 합니다")
            }
            Self::InvalidPaintColor => {
                formatter.write_str("runtime sRGB paint 채널과 alpha는 유한한 0..1 값이어야 합니다")
            }
            Self::InvalidScrollState => {
                formatter.write_str("runtime scroll range 또는 offset이 유효하지 않습니다")
            }
            Self::DuplicateScrollOwner(node) => {
                write!(formatter, "runtime scroll owner {node}가 중복되었습니다")
            }
            Self::StaleScrollRevision => {
                formatter.write_str("runtime scroll 요청의 state revision이 오래되었습니다")
            }
            Self::DuplicateScrollRequest(node) => {
                write!(
                    formatter,
                    "runtime scroll 요청에서 owner {node}가 중복되었습니다"
                )
            }
            Self::InvalidScrollRequest(node) => {
                write!(
                    formatter,
                    "runtime scroll 요청의 owner {node} 또는 offset이 유효하지 않습니다"
                )
            }
            Self::InvalidScrollTarget(node) => {
                write!(
                    formatter,
                    "runtime scroll 입력의 target {node}가 snapshot에 없습니다"
                )
            }
            Self::InvalidScrollPoint => {
                formatter.write_str("runtime scroll 입력 point는 유한한 CSS px여야 합니다")
            }
            Self::InvalidScrollDelta => {
                formatter.write_str("runtime scroll 입력 delta는 유한한 CSS px여야 합니다")
            }
            Self::ScrollRevisionExhausted => {
                formatter.write_str("runtime scroll state revision 번호를 더 증가시킬 수 없습니다")
            }
            Self::DuplicateImage(node) => {
                write!(formatter, "runtime image node {node}가 중복됩니다")
            }
            Self::ImageNodeMissing(node) => {
                write!(
                    formatter,
                    "runtime image node {node}가 render snapshot에 없습니다"
                )
            }
            Self::InvalidImageSource(node) => {
                write!(
                    formatter,
                    "runtime image node {node}의 source revision이 다릅니다"
                )
            }
            Self::InvalidImageBytes(node) => {
                write!(
                    formatter,
                    "runtime image node {node}의 RGBA8 크기가 유효하지 않습니다"
                )
            }
            Self::InvalidImageFrame(node) => {
                write!(
                    formatter,
                    "runtime image node {node} frame이 자연 크기와 다릅니다"
                )
            }
            Self::InvalidImageGeometry(node) => {
                write!(
                    formatter,
                    "runtime image node {node}의 object geometry가 유효하지 않습니다"
                )
            }
            Self::InvalidImageStyleRevision(node) => {
                write!(
                    formatter,
                    "runtime image node {node}의 object style revision이 장면과 다릅니다"
                )
            }
            Self::UnsupportedImageBorder(node) => {
                write!(
                    formatter,
                    "runtime image node {node}의 border는 지원하지 않습니다"
                )
            }
            Self::InvalidTextGeometry => {
                formatter.write_str("runtime text 위치와 device scale이 유효하지 않습니다")
            }
            Self::InvalidTextBitmap => {
                formatter.write_str("runtime glyph bitmap 크기·payload가 유효하지 않습니다")
            }
            Self::InvalidTextSource(node) => {
                write!(
                    formatter,
                    "runtime text node {node}의 source revision이 장면과 다릅니다"
                )
            }
            Self::TextOwnerMissing(node) => {
                write!(
                    formatter,
                    "runtime text owner {node}가 render snapshot에 없습니다"
                )
            }
            Self::DuplicateText(node) => {
                write!(formatter, "runtime text owner {node}가 중복됩니다")
            }
            Self::UnsupportedTextScroll(node) => {
                write!(
                    formatter,
                    "scroll offset을 적용할 수 있는 runtime text owner가 아닙니다: {node}"
                )
            }
        }
    }
}

impl Error for RuntimeRenderError {}
