use std::{error::Error, fmt};

use crate::LayoutCalcId;
use spinon_core::NodeId;
use spinon_text_layout::TextLayoutError;

/// 입력 검증, Taffy 계산 또는 결과 변환 실패입니다.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutError {
    InvalidViewport,
    EmptyTree,
    InvalidHostDocumentRoot(NodeId),
    UnsupportedTextNode(NodeId),
    InvalidTextEncoding(NodeId),
    DuplicateTextParagraph(NodeId),
    UnknownTextParagraph(NodeId),
    UnsupportedTextParagraph {
        node: NodeId,
        reason: &'static str,
    },
    MissingTextLayoutEngine(NodeId),
    IndefiniteTextAvailableWidth(NodeId),
    TextLayoutWidthMismatch(NodeId),
    TextLayoutHeightMismatch(NodeId),
    TextLayout(TextLayoutError),
    IntrinsicSizeSourceMismatch {
        expected: crate::LayoutSourceRevision,
        actual: crate::LayoutSourceRevision,
    },
    DuplicateIntrinsicSizeNode(NodeId),
    UnknownIntrinsicSizeNode(NodeId),
    UnexpectedIntrinsicSizeNode(NodeId),
    MissingIntrinsicSize(NodeId),
    InvalidIntrinsicSize {
        node: NodeId,
        axis: &'static str,
    },
    UnsupportedIntrinsicSizing {
        node: NodeId,
        reason: &'static str,
    },
    MissingRoot(NodeId),
    MissingStyle(NodeId),
    UnknownStyleNode(NodeId),
    DuplicateNode(NodeId),
    MissingChild {
        parent: NodeId,
        child: NodeId,
    },
    DuplicateChild {
        parent: NodeId,
        child: NodeId,
    },
    RootHasParent(NodeId),
    MultipleParents(NodeId),
    DetachedNode(NodeId),
    Cycle(NodeId),
    UnreachableNode(NodeId),
    TreeDepthExceeded {
        node: NodeId,
        depth: usize,
        maximum: usize,
    },
    RootSizeMismatch {
        axis: &'static str,
    },
    InvalidStyle {
        node: NodeId,
        field: &'static str,
    },
    UnknownPositioningNode(NodeId),
    InvalidPositioning {
        node: NodeId,
        field: &'static str,
    },
    UnsupportedPositioning {
        node: NodeId,
        reason: &'static str,
    },
    UnsupportedBaseline {
        node: NodeId,
        reason: &'static str,
    },
    UnsupportedRootPercentageSpacing {
        node: NodeId,
        property: &'static str,
    },
    IndefinitePercentageBasis {
        node: NodeId,
        property: &'static str,
        axis: &'static str,
    },
    DuplicateCssMathId,
    MissingCssMath {
        node: NodeId,
        property: &'static str,
        id: LayoutCalcId,
    },
    CssMathBindingMismatch {
        node: NodeId,
        property: &'static str,
        id: LayoutCalcId,
    },
    InvalidCssMath {
        node: NodeId,
        property: &'static str,
        reason: &'static str,
    },
    UnknownGridContainer(NodeId),
    UnknownGridItem(NodeId),
    UnexpectedGridContainer(NodeId),
    UnexpectedGridItem(NodeId),
    MissingGridContainer(NodeId),
    InvalidGridTrackList {
        node: NodeId,
        property: &'static str,
    },
    InvalidGridTrack {
        node: NodeId,
        property: &'static str,
    },
    GridTrackLimitExceeded {
        node: NodeId,
        property: &'static str,
        count: usize,
        maximum: usize,
    },
    GridTrackExpansionOverflow(NodeId),
    UnsupportedGridAutoFlow(NodeId),
    UnsupportedGridAutoRows {
        node: NodeId,
        reason: &'static str,
    },
    UnsupportedGridAutoColumns {
        node: NodeId,
        reason: &'static str,
    },
    UnsupportedGridPlacement(NodeId),
    InvalidGridPlacement {
        node: NodeId,
        property: &'static str,
        reason: &'static str,
    },
    UnsupportedGridOrder(NodeId),
    UnsupportedGridPositionedItem(NodeId),
    UnsupportedGridItemSize(NodeId),
    UnsupportedGridItemConstraint {
        node: NodeId,
        property: &'static str,
    },
    GridImplicitTrackRequired(NodeId),
    IndefiniteGridBasis {
        node: NodeId,
        axis: &'static str,
    },
    GridAllocationFailure {
        node: NodeId,
        property: &'static str,
    },
    UnknownCssMathHandle,
    TooManyNodes,
    Taffy(String),
    TaffyPanicked,
    MissingComputedLayout(NodeId),
    NonFiniteFrame(NodeId),
}

impl fmt::Display for LayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidViewport => {
                write!(formatter, "화면 크기는 0보다 큰 유한한 값이어야 합니다")
            }
            Self::EmptyTree => write!(formatter, "레이아웃할 코어 트리에 루트가 없습니다"),
            Self::InvalidHostDocumentRoot(id) => write!(
                formatter,
                "HostDocument 레이아웃 루트 {id}는 HostRoot 직속 요소가 아닙니다"
            ),
            Self::UnsupportedTextNode(id) => write!(
                formatter,
                "텍스트 노드 {id}는 intrinsic measurement를 지원하지 않습니다"
            ),
            Self::InvalidTextEncoding(id) => write!(
                formatter,
                "텍스트 노드 {id}에 정상적인 UTF-16 surrogate pair가 없습니다"
            ),
            Self::DuplicateTextParagraph(id) => {
                write!(formatter, "paragraph 입력의 노드 {id}가 중복되었습니다")
            }
            Self::UnknownTextParagraph(id) => {
                write!(
                    formatter,
                    "paragraph 입력의 노드 {id}가 Block 트리에 없습니다"
                )
            }
            Self::UnsupportedTextParagraph { node, reason } => write!(
                formatter,
                "노드 {node}의 paragraph 입력을 현재 C15.1 profile에서 지원하지 않습니다: {reason}"
            ),
            Self::MissingTextLayoutEngine(id) => {
                write!(
                    formatter,
                    "노드 {id}의 paragraph를 shaping할 TextLayoutEngine이 없습니다"
                )
            }
            Self::IndefiniteTextAvailableWidth(id) => write!(
                formatter,
                "노드 {id}의 paragraph에는 definite content width가 필요합니다"
            ),
            Self::TextLayoutWidthMismatch(id) => write!(
                formatter,
                "노드 {id}의 최종 content width와 text measure width가 다릅니다"
            ),
            Self::TextLayoutHeightMismatch(id) => write!(
                formatter,
                "노드 {id}의 최종 content height와 text measure height가 다릅니다"
            ),
            Self::TextLayout(error) => error.fmt(formatter),
            Self::IntrinsicSizeSourceMismatch { expected, actual } => write!(
                formatter,
                "intrinsic size source가 현재 layout source와 다릅니다: 기대 {expected:?}, 입력 {actual:?}"
            ),
            Self::DuplicateIntrinsicSizeNode(id) => {
                write!(formatter, "intrinsic size 입력에서 노드 {id}가 중복됩니다")
            }
            Self::UnknownIntrinsicSizeNode(id) => {
                write!(formatter, "intrinsic size 입력의 노드 {id}가 없습니다")
            }
            Self::UnexpectedIntrinsicSizeNode(id) => {
                write!(formatter, "노드 {id}는 HTML img 측정 대상이 아닙니다")
            }
            Self::MissingIntrinsicSize(id) => {
                write!(
                    formatter,
                    "노드 {id}의 auto 크기에 필요한 intrinsic size가 없습니다"
                )
            }
            Self::InvalidIntrinsicSize { node, axis } => write!(
                formatter,
                "노드 {node}의 intrinsic {axis}는 양의 유한 CSS px 값이어야 합니다"
            ),
            Self::UnsupportedIntrinsicSizing { node, reason } => {
                write!(
                    formatter,
                    "노드 {node}의 intrinsic sizing을 지원하지 않습니다: {reason}"
                )
            }
            Self::MissingRoot(id) => write!(formatter, "루트 노드 {id}를 찾을 수 없습니다"),
            Self::MissingStyle(id) => write!(formatter, "노드 {id}의 계산된 스타일이 없습니다"),
            Self::UnknownStyleNode(id) => {
                write!(formatter, "스타일의 노드 {id}가 코어 트리에 없습니다")
            }
            Self::DuplicateNode(id) => write!(formatter, "노드 ID {id}가 중복되었습니다"),
            Self::MissingChild { parent, child } => {
                write!(formatter, "노드 {parent}의 자식 {child}를 찾을 수 없습니다")
            }
            Self::DuplicateChild { parent, child } => {
                write!(formatter, "노드 {parent}에 자식 {child}가 중복되었습니다")
            }
            Self::RootHasParent(id) => {
                write!(formatter, "루트 노드 {id}는 다른 노드의 자식일 수 없습니다")
            }
            Self::MultipleParents(id) => write!(formatter, "노드 {id}에 부모가 둘 이상 있습니다"),
            Self::DetachedNode(id) => write!(formatter, "루트가 아닌 노드 {id}에 부모가 없습니다"),
            Self::Cycle(id) => write!(formatter, "노드 {id}에서 순환 참조를 발견했습니다"),
            Self::UnreachableNode(id) => {
                write!(formatter, "노드 {id}는 루트에서 도달할 수 없습니다")
            }
            Self::TreeDepthExceeded {
                node,
                depth,
                maximum,
            } => write!(
                formatter,
                "레이아웃 트리 깊이 {depth}가 최대 {maximum}을 초과했습니다 (노드 {node})"
            ),
            Self::RootSizeMismatch { axis } => {
                write!(formatter, "루트의 {axis} 크기는 viewport와 같아야 합니다")
            }
            Self::InvalidStyle { node, field } => {
                write!(formatter, "노드 {node}의 {field} 값이 유효하지 않습니다")
            }
            Self::UnknownPositioningNode(id) => {
                write!(
                    formatter,
                    "position 입력의 노드 {id}가 레이아웃 트리에 없습니다"
                )
            }
            Self::InvalidPositioning { node, field } => {
                write!(
                    formatter,
                    "노드 {node}의 position {field} 값이 유효하지 않습니다"
                )
            }
            Self::UnsupportedPositioning { node, reason } => {
                write!(
                    formatter,
                    "노드 {node}의 position 계산을 지원하지 않습니다: {reason}"
                )
            }
            Self::UnsupportedBaseline { node, reason } => {
                write!(
                    formatter,
                    "노드 {node}의 baseline 계산을 지원하지 않습니다: {reason}"
                )
            }
            Self::UnsupportedRootPercentageSpacing { node, property } => write!(
                formatter,
                "레이아웃 root 노드 {node}의 {property} 백분율은 현재 계약에서 지원하지 않습니다"
            ),
            Self::IndefinitePercentageBasis {
                node,
                property,
                axis,
            } => write!(
                formatter,
                "노드 {node}의 {property} 백분율 기준 {axis} 크기를 확정할 수 없습니다"
            ),
            Self::DuplicateCssMathId => write!(formatter, "CSS 계산식 ID가 중복되었습니다"),
            Self::MissingCssMath { node, property, id } => write!(
                formatter,
                "노드 {node}의 {property} CSS 계산식 ID {}를 찾을 수 없습니다",
                id.0
            ),
            Self::CssMathBindingMismatch { node, property, id } => write!(
                formatter,
                "노드 {node}의 {property}가 다른 노드·속성의 CSS 계산식 ID {}를 참조합니다",
                id.0
            ),
            Self::InvalidCssMath {
                node,
                property,
                reason,
            } => write!(
                formatter,
                "노드 {node}의 {property} CSS 계산식을 안전하게 평가할 수 없습니다: {reason}"
            ),
            Self::UnknownGridContainer(id) => {
                write!(formatter, "Grid container 입력의 노드 {id}가 없습니다")
            }
            Self::UnknownGridItem(id) => {
                write!(formatter, "Grid item 입력의 노드 {id}가 없습니다")
            }
            Self::UnexpectedGridContainer(id) => {
                write!(
                    formatter,
                    "Grid container 입력과 display:grid 노드 {id}가 맞지 않습니다"
                )
            }
            Self::UnexpectedGridItem(id) => {
                write!(
                    formatter,
                    "Grid item 입력의 노드 {id}가 Grid container의 직계 자식이 아닙니다"
                )
            }
            Self::MissingGridContainer(id) => {
                write!(
                    formatter,
                    "display:grid 노드 {id}의 computed track 입력이 없습니다"
                )
            }
            Self::InvalidGridTrackList { node, property } => {
                write!(
                    formatter,
                    "노드 {node}의 {property}에 명시 track이 없습니다"
                )
            }
            Self::InvalidGridTrack { node, property } => {
                write!(
                    formatter,
                    "노드 {node}의 {property} track 값이 유효하지 않습니다"
                )
            }
            Self::GridTrackLimitExceeded {
                node,
                property,
                count,
                maximum,
            } => write!(
                formatter,
                "노드 {node}의 {property} track 수 {count}가 Taffy 0.14.0 한계 {maximum}을 초과합니다"
            ),
            Self::GridTrackExpansionOverflow(id) => {
                write!(
                    formatter,
                    "노드 {id}의 Grid cell 수 계산이 범위를 초과합니다"
                )
            }
            Self::UnsupportedGridAutoFlow(id) => {
                write!(
                    formatter,
                    "노드 {id}의 grid-auto-flow를 C11.1에서 지원하지 않습니다"
                )
            }
            Self::UnsupportedGridAutoRows { node, reason } => write!(
                formatter,
                "노드 {node}의 grid-auto-rows를 현재 profile에서 계산할 수 없습니다: {reason}"
            ),
            Self::UnsupportedGridAutoColumns { node, reason } => write!(
                formatter,
                "노드 {node}의 grid-auto-columns를 현재 profile에서 계산할 수 없습니다: {reason}"
            ),
            Self::UnsupportedGridPlacement(id) => {
                write!(
                    formatter,
                    "Grid item {id}의 명시 line placement를 현재 profile에서 지원하지 않습니다"
                )
            }
            Self::InvalidGridPlacement {
                node,
                property,
                reason,
            } => write!(
                formatter,
                "Grid item {node}의 {property} placement를 계산할 수 없습니다: {reason}"
            ),
            Self::UnsupportedGridOrder(id) => {
                write!(
                    formatter,
                    "Grid item {id}의 0이 아닌 order를 C11.1에서 지원하지 않습니다"
                )
            }
            Self::UnsupportedGridPositionedItem(id) => {
                write!(
                    formatter,
                    "위치 지정 Grid item {id}를 C11.1에서 지원하지 않습니다"
                )
            }
            Self::UnsupportedGridItemSize(id) => {
                write!(
                    formatter,
                    "Grid item {id}는 C11.1에서 고정 width와 height가 필요합니다"
                )
            }
            Self::UnsupportedGridItemConstraint { node, property } => write!(
                formatter,
                "Grid item {node}의 {property} 제약은 현재 Grid profile에서 지원하지 않습니다"
            ),
            Self::GridImplicitTrackRequired(id) => {
                write!(
                    formatter,
                    "노드 {id}의 auto-placement가 implicit track을 요구합니다"
                )
            }
            Self::IndefiniteGridBasis { node, axis } => {
                write!(
                    formatter,
                    "노드 {node}의 Grid {axis} basis가 definite하지 않습니다"
                )
            }
            Self::GridAllocationFailure { node, property } => {
                write!(
                    formatter,
                    "노드 {node}의 {property} track 공간을 예약하지 못했습니다"
                )
            }
            Self::UnknownCssMathHandle => {
                write!(
                    formatter,
                    "Taffy가 현재 layout owner가 만들지 않은 CSS 계산식 handle을 요청했습니다"
                )
            }
            Self::TooManyNodes => write!(formatter, "Taffy 노드 ID 범위를 초과했습니다"),
            Self::Taffy(message) => {
                write!(formatter, "Taffy 레이아웃 계산에 실패했습니다: {message}")
            }
            Self::TaffyPanicked => {
                write!(formatter, "Taffy 레이아웃 계산이 panic으로 중단되었습니다")
            }
            Self::MissingComputedLayout(id) => {
                write!(formatter, "노드 {id}의 계산된 레이아웃을 찾을 수 없습니다")
            }
            Self::NonFiniteFrame(id) => {
                write!(formatter, "노드 {id}의 계산 결과가 유한하지 않습니다")
            }
        }
    }
}

impl Error for LayoutError {}
