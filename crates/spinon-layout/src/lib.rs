mod calc_tree;
mod compact_taffy_style;
mod compute;
mod css_math;
mod error;
mod flex_auto_margin;
mod flex_baseline;
mod grid;
mod grid_auto_placement;
mod grid_placement;
mod host_document;
mod intrinsic_size;
mod percentage_basis;
mod positioning;
mod revision;
mod style;
mod taffy_style;
#[cfg(test)]
mod test_allocator;
mod text_flow;
mod tree_input;

#[cfg(test)]
#[path = "c15_text_layout_tests.rs"]
mod c15_text_layout_tests;

use std::collections::{BTreeMap, BTreeSet};

use percentage_basis::validate_spacing_percentage_bases;
use positioning::validate_positioning;
use spinon_core::NodeId;

/// 현재 재귀형 Taffy 경로에서 허용하는 최대 입력 트리 깊이입니다. 루트 깊이는 1입니다.
pub const MAX_LAYOUT_TREE_DEPTH: usize = 1_024;

pub use css_math::{LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue};
pub use error::LayoutError;
pub use grid::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridAutoTrackSize, GridContainerInput,
    GridPlacementMode, GridTrackSize,
};
pub use grid_placement::{GridItemInput, GridPlacementInput};
pub use intrinsic_size::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot};
pub use revision::{LayoutInputRevision, LayoutSourceRevision};
pub use spinon_text_layout::{
    FontRegistryRevision, FontStyleMode, GlyphBitmap, GlyphBitmapContent, InlineFragment,
    InlineRunStyle, LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle,
    TextDirection as TextParagraphDirection, TextGlyph, TextGlyphRasterSnapshot, TextLayoutEngine,
    TextLayoutSnapshot, TextLine, TextRect, TextRun, TextSource, TextSourceRange, WhiteSpaceMode,
    WordBreakMode,
};
pub use style::{
    AlignmentSafety, ContentAlignmentPosition, FlexDirection, FlexWrap, ItemAlignmentPosition,
    JustifyContentPosition, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf, LayoutBorder,
    LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutGap, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutOverflow, LayoutPosition, LayoutPositioning, LayoutStyle,
    TextDirection, Viewport,
};
pub(crate) use text_flow::{TextFlowInput, TextFlowItem, TextParagraphInput};

/// 부모와 자식 ID 순서 및 레이아웃 스타일을 묶은 입력 노드입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutNode {
    pub id: NodeId,
    pub children: Vec<NodeId>,
    pub style: LayoutStyle,
}

/// 한 번의 전체 레이아웃 계산 입력입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutInput {
    root: NodeId,
    revision: LayoutInputRevision,
    viewport: Viewport,
    root_sizing: RootSizingPolicy,
    nodes: Vec<LayoutNode>,
    css_math: Vec<LayoutCssMathValue>,
    positioning: BTreeMap<NodeId, LayoutPositioning>,
    grid_containers: BTreeMap<NodeId, GridContainerInput>,
    grid_items: BTreeMap<NodeId, GridItemInput>,
    text_paragraphs: BTreeMap<NodeId, TextParagraphInput>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RootSizingPolicy {
    Match,
    ResolveWithin,
    BlockFormatting,
}

impl LayoutInput {
    pub const fn root(&self) -> NodeId {
        self.root
    }

    pub const fn source_revision(&self) -> LayoutSourceRevision {
        self.revision.source()
    }

    pub const fn revision(&self) -> LayoutInputRevision {
        self.revision
    }

    pub const fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn nodes(&self) -> &[LayoutNode] {
        &self.nodes
    }

    /// Stylo에서 보존한 계산식을 노드·속성 식별자와 함께 이 레이아웃 입력에 연결합니다.
    pub fn with_css_math(mut self, values: Vec<LayoutCssMathValue>) -> Self {
        self.css_math = values;
        self
    }

    /// Typed position과 inset 값을 일반 크기·Flex 스타일과 별도로 연결합니다.
    pub fn with_positioning(mut self, values: BTreeMap<NodeId, LayoutPositioning>) -> Self {
        self.positioning = values;
        self
    }

    /// Stylo에서 계산한 Grid track과 item placement를 동일한 입력 revision에 연결합니다.
    pub fn with_grid_layout(
        mut self,
        containers: BTreeMap<NodeId, GridContainerInput>,
        items: BTreeMap<NodeId, GridItemInput>,
    ) -> Self {
        self.grid_containers = containers;
        self.grid_items = items;
        self
    }

    /// Definite Block content width에서 shaping할 paragraph snapshot을 입력합니다.
    pub fn with_text_paragraphs(
        mut self,
        paragraphs: impl IntoIterator<Item = ParagraphInput>,
    ) -> Result<Self, LayoutError> {
        let mut values = BTreeMap::new();
        for paragraph in paragraphs {
            let node = paragraph.node;
            if values
                .insert(node, TextParagraphInput::Paragraph(paragraph))
                .is_some()
            {
                return Err(LayoutError::DuplicateTextParagraph(node));
            }
        }
        self.text_paragraphs = values;
        Ok(self)
    }
}

/// 화면 루트 왼쪽 위 기준의 절대 프레임입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutFrame {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 성공한 한 번의 계산에서 반환한 모든 노드의 프레임입니다.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutOutput {
    pub revision: LayoutInputRevision,
    /// 조상·자기 상대 inset을 반영해 renderer에 게시할 최종 프레임입니다.
    pub frames: BTreeMap<NodeId, LayoutFrame>,
    /// 각 box의 resolved padding과 border edge입니다. paint 단계의 clipping에 사용합니다.
    pub used_box_edges: BTreeMap<NodeId, LayoutUsedBoxEdges>,
    /// 상대 inset을 모두 중립화한 normal-flow 기준 프레임입니다.
    pub flow_frames: BTreeMap<NodeId, LayoutFrame>,
    /// 각 box가 사용할 수 있는 가장 가까운 relative containing block입니다.
    pub positioned_owners: BTreeMap<NodeId, PositionedContainingBlockOwner>,
    /// CSS fixed box의 viewport owner를 absolute owner와 별도로 보존합니다.
    pub fixed_owners: BTreeMap<NodeId, FixedContainingBlockOwner>,
    /// Taffy measure가 최종 content width에서 만든 revision-stamped text snapshot입니다.
    pub text_layouts: BTreeMap<NodeId, LayoutTextSnapshot>,
    /// 직접 Block 자식 앞뒤의 anonymous inline paragraph 결과를 원본 순서로 보존합니다.
    pub text_flow_layouts: BTreeMap<NodeId, Vec<LayoutTextFlowSegment>>,
}

/// 하나의 layout 계산 입력 revision에 결속한 shaping 결과입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutTextSnapshot {
    pub revision: LayoutInputRevision,
    pub input: ParagraphInput,
    pub layout: TextLayoutSnapshot,
}

/// Block owner 기준 합성 paragraph의 geometry와 shaping 결과입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutTextFlowSegment {
    pub item_index: usize,
    pub frame: LayoutFrame,
    pub snapshot: LayoutTextSnapshot,
}

/// Taffy가 계산한 사용값 padding과 border 두께입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutUsedBoxEdges {
    pub padding: LayoutBorder,
    pub border: LayoutBorder,
}

/// 계산 트리 안에서 nearest positioned containing block이 되는 owner입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionedContainingBlockOwner {
    Viewport,
    Node(NodeId),
    NoBox,
}

/// 현재 viewport-only fixed profile에서 각 fixed box의 owner입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixedContainingBlockOwner {
    Viewport,
    NoBox,
}

/// 입력 트리를 검증하고 논리 단위 프레임을 반환하는 내부 엔진 경계입니다.
pub trait LayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError>;
}

/// Taffy Block·Flexbox·Grid 계산기를 연결하는 내부 엔진입니다.
#[derive(Clone, Copy, Debug, Default)]
pub struct TaffyLayoutEngine;

fn validate(
    input: &LayoutInput,
    intrinsic_size_nodes: &BTreeSet<NodeId>,
) -> Result<BTreeMap<NodeId, usize>, LayoutError> {
    if !input.viewport.width.is_finite()
        || !input.viewport.height.is_finite()
        || input.viewport.width <= 0.0
        || input.viewport.height <= 0.0
    {
        return Err(LayoutError::InvalidViewport);
    }

    let mut index = BTreeMap::new();
    for (position, node) in input.nodes.iter().enumerate() {
        if index.insert(node.id, position).is_some() {
            return Err(LayoutError::DuplicateNode(node.id));
        }
    }
    let Some(&root_position) = index.get(&input.root) else {
        return Err(LayoutError::MissingRoot(input.root));
    };

    for node in &input.nodes {
        validate_style(node)?;
    }

    if input.root_sizing == RootSizingPolicy::Match {
        let root = &input.nodes[root_position];
        if root.style.width != LayoutDimension::Fixed(input.viewport.width) {
            return Err(LayoutError::RootSizeMismatch { axis: "width" });
        }
        if root.style.height != LayoutDimension::Fixed(input.viewport.height) {
            return Err(LayoutError::RootSizeMismatch { axis: "height" });
        }
    }

    let mut parent_count = BTreeMap::<NodeId, usize>::new();
    for node in &input.nodes {
        let mut child_set = BTreeSet::new();
        for &child in &node.children {
            if !index.contains_key(&child) {
                return Err(LayoutError::MissingChild {
                    parent: node.id,
                    child,
                });
            }
            if !child_set.insert(child) {
                return Err(LayoutError::DuplicateChild {
                    parent: node.id,
                    child,
                });
            }
            if child == input.root {
                return Err(LayoutError::RootHasParent(input.root));
            }
            *parent_count.entry(child).or_default() += 1;
            if parent_count[&child] > 1 {
                return Err(LayoutError::MultipleParents(child));
            }
        }
    }

    for &id in index.keys() {
        if id == input.root {
            continue;
        }
        if !parent_count.contains_key(&id) {
            return Err(LayoutError::DetachedNode(id));
        }
    }

    detect_cycles(input, &index)?;
    let mut reachable = BTreeSet::new();
    let mut pending = vec![(input.root, 1_usize)];
    while let Some((id, depth)) = pending.pop() {
        if reachable.insert(id) {
            if depth > MAX_LAYOUT_TREE_DEPTH {
                return Err(LayoutError::TreeDepthExceeded {
                    node: id,
                    depth,
                    maximum: MAX_LAYOUT_TREE_DEPTH,
                });
            }
            pending.extend(
                input.nodes[index[&id]]
                    .children
                    .iter()
                    .rev()
                    .copied()
                    .map(|child| (child, depth.saturating_add(1))),
            );
        }
    }
    if let Some(id) = index.keys().find(|id| !reachable.contains(id)) {
        return Err(LayoutError::UnreachableNode(*id));
    }

    validate_positioning(input, &index)?;
    validate_css_math_bindings(input)?;
    let definite_axes = validate_spacing_percentage_bases(input, &index)?;
    grid::validate_grid_layout(input, &index, &definite_axes, intrinsic_size_nodes)?;

    Ok(index)
}

fn validate_style(node: &LayoutNode) -> Result<(), LayoutError> {
    if node.style.display == LayoutDisplay::Inline {
        return Err(LayoutError::InvalidStyle {
            node: node.id,
            field: "display:inline",
        });
    }
    let valid_dimension = |dimension| match dimension {
        LayoutDimension::Auto
        | LayoutDimension::Calc(_)
        | LayoutDimension::MinContent
        | LayoutDimension::MaxContent
        | LayoutDimension::FitContent => true,
        LayoutDimension::Fixed(value) => value.is_finite() && value >= 0.0,
        LayoutDimension::Percent(value) => value.is_finite() && value >= 0.0,
        LayoutDimension::CyclicMinContentPercent(value) => value.is_finite() && value >= 0.0,
    };
    if !valid_dimension(node.style.width) {
        return Err(LayoutError::InvalidStyle {
            node: node.id,
            field: "width",
        });
    }
    if !valid_dimension(node.style.height) {
        return Err(LayoutError::InvalidStyle {
            node: node.id,
            field: "height",
        });
    }
    for (field, dimension) in [
        ("min_width", node.style.min_width),
        ("max_width", node.style.max_width),
        ("min_height", node.style.min_height),
        ("max_height", node.style.max_height),
    ] {
        if !valid_dimension(dimension) {
            return Err(LayoutError::InvalidStyle {
                node: node.id,
                field,
            });
        }
    }
    if !valid_dimension(node.style.flex_basis) {
        return Err(LayoutError::InvalidStyle {
            node: node.id,
            field: "flex_basis",
        });
    }
    if node
        .style
        .aspect_ratio
        .is_some_and(|ratio| !ratio.is_finite() || ratio <= 0.0)
    {
        return Err(LayoutError::InvalidStyle {
            node: node.id,
            field: "aspect_ratio",
        });
    }
    for (field, value) in [
        ("margin.top", node.style.margin.top),
        ("margin.right", node.style.margin.right),
        ("margin.bottom", node.style.margin.bottom),
        ("margin.left", node.style.margin.left),
    ] {
        if !matches!(value, LayoutLengthPercentage::Auto)
            && !value.is_calc()
            && !value.value().is_finite()
        {
            return Err(LayoutError::InvalidStyle {
                node: node.id,
                field,
            });
        }
    }
    for (field, value) in [
        ("padding.top", node.style.padding.top),
        ("padding.right", node.style.padding.right),
        ("padding.bottom", node.style.padding.bottom),
        ("padding.left", node.style.padding.left),
        ("gap.row", node.style.gap.row),
        ("gap.column", node.style.gap.column),
    ] {
        if !value.is_calc() && (!value.value().is_finite() || value.value() < 0.0) {
            return Err(LayoutError::InvalidStyle {
                node: node.id,
                field,
            });
        }
    }
    for (field, value) in [
        ("border.top", node.style.border.top),
        ("border.right", node.style.border.right),
        ("border.bottom", node.style.border.bottom),
        ("border.left", node.style.border.left),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(LayoutError::InvalidStyle {
                node: node.id,
                field,
            });
        }
    }
    for (field, value) in [
        ("flex_grow", node.style.flex_grow),
        ("flex_shrink", node.style.flex_shrink),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(LayoutError::InvalidStyle {
                node: node.id,
                field,
            });
        }
    }
    Ok(())
}

fn validate_css_math_bindings(input: &LayoutInput) -> Result<(), LayoutError> {
    let by_id = input
        .css_math
        .iter()
        .map(|value| (value.id, value))
        .collect::<BTreeMap<_, _>>();
    if by_id.len() != input.css_math.len() {
        return Err(LayoutError::DuplicateCssMathId);
    }

    for value in &input.css_math {
        value
            .contains_percentage()
            .map_err(|reason| value.error(reason))?;
    }
    for node in &input.nodes {
        for (property, id) in node.style.calc_values() {
            let Some(value) = by_id.get(&id) else {
                return Err(LayoutError::MissingCssMath {
                    node: node.id,
                    property: property.name(),
                    id,
                });
            };
            if value.node_id != node.id || value.property != property {
                return Err(LayoutError::CssMathBindingMismatch {
                    node: node.id,
                    property: property.name(),
                    id,
                });
            }
        }
    }
    for (&node, positioning) in &input.positioning {
        for (property, id) in positioning.calc_values() {
            let Some(value) = by_id.get(&id) else {
                return Err(LayoutError::MissingCssMath {
                    node,
                    property: property.name(),
                    id,
                });
            };
            if value.node_id != node || value.property != property {
                return Err(LayoutError::CssMathBindingMismatch {
                    node,
                    property: property.name(),
                    id,
                });
            }
        }
    }
    Ok(())
}

fn detect_cycles(input: &LayoutInput, index: &BTreeMap<NodeId, usize>) -> Result<(), LayoutError> {
    let mut visited = BTreeSet::new();
    for &start in index.keys() {
        if visited.contains(&start) {
            continue;
        }
        let mut active = BTreeSet::new();
        let mut pending = vec![(start, false)];
        while let Some((id, exiting)) = pending.pop() {
            if exiting {
                active.remove(&id);
                visited.insert(id);
                continue;
            }
            if visited.contains(&id) {
                continue;
            }
            if !active.insert(id) {
                return Err(LayoutError::Cycle(id));
            }
            pending.push((id, true));
            for &child in input.nodes[index[&id]].children.iter().rev() {
                if active.contains(&child) {
                    return Err(LayoutError::Cycle(child));
                }
                if !visited.contains(&child) {
                    pending.push((child, false));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod absolute_position_tests;
#[cfg(test)]
mod depth_tests;
#[cfg(test)]
mod fixed_position_tests;
#[cfg(test)]
#[path = "intrinsic_size_tests.rs"]
mod intrinsic_size_tests;
#[cfg(test)]
mod positioning_tests;
#[cfg(test)]
mod tests;
