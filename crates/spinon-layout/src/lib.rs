mod calc_tree;
mod compute;
mod css_math;
mod error;
mod flex_auto_margin;
mod flex_baseline;
mod host_document;
mod percentage_basis;
mod positioning;
mod revision;
mod style;
mod taffy_style;
mod tree_input;

use std::collections::{BTreeMap, BTreeSet};

use percentage_basis::validate_spacing_percentage_bases;
use positioning::validate_positioning;
use spinon_core::NodeId;

pub use css_math::{LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue};
pub use error::LayoutError;
pub use revision::{LayoutInputRevision, LayoutSourceRevision};
pub use style::{
    AlignmentSafety, ContentAlignmentPosition, FlexDirection, FlexWrap, ItemAlignmentPosition,
    JustifyContentPosition, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf, LayoutBorder,
    LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutGap, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutPosition, LayoutPositioning, LayoutStyle, TextDirection,
    Viewport,
};

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
    /// 상대 inset을 모두 중립화한 normal-flow 기준 프레임입니다.
    pub flow_frames: BTreeMap<NodeId, LayoutFrame>,
    /// 각 box가 사용할 수 있는 가장 가까운 relative containing block입니다.
    pub positioned_owners: BTreeMap<NodeId, PositionedContainingBlockOwner>,
}

/// 계산 트리 안에서 nearest positioned containing block이 되는 owner입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionedContainingBlockOwner {
    Viewport,
    Node(NodeId),
    NoBox,
}

/// 입력 트리를 검증하고 논리 단위 프레임을 반환하는 내부 엔진 경계입니다.
pub trait LayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError>;
}

/// Taffy Flexbox 기반의 현재 내부 엔진입니다.
#[derive(Clone, Copy, Debug, Default)]
pub struct TaffyLayoutEngine;

fn validate(input: &LayoutInput) -> Result<BTreeMap<NodeId, usize>, LayoutError> {
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
    let mut pending = vec![input.root];
    while let Some(id) = pending.pop() {
        if reachable.insert(id) {
            pending.extend(input.nodes[index[&id]].children.iter().copied());
        }
    }
    if let Some(id) = index.keys().find(|id| !reachable.contains(id)) {
        return Err(LayoutError::UnreachableNode(*id));
    }

    validate_positioning(input, &index)?;
    validate_css_math_bindings(input)?;
    validate_spacing_percentage_bases(input, &index)?;

    Ok(index)
}

fn validate_style(node: &LayoutNode) -> Result<(), LayoutError> {
    let valid_dimension = |dimension| match dimension {
        LayoutDimension::Auto | LayoutDimension::Calc(_) => true,
        LayoutDimension::Fixed(value) => value.is_finite() && value >= 0.0,
        LayoutDimension::Percent(value) => value.is_finite() && value >= 0.0,
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
mod positioning_tests;
#[cfg(test)]
mod tests;
