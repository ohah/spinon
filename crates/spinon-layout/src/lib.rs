mod calc_tree;
mod css_math;
mod error;
mod host_document;
mod percentage_basis;
mod revision;
mod style;
mod taffy_style;
mod tree_input;

use std::collections::{BTreeMap, BTreeSet};

use calc_tree::CalcLayoutTree;
use percentage_basis::validate_spacing_percentage_bases;
use spinon_core::NodeId;
use taffy::prelude::{AvailableSpace, Size};

pub use css_math::{LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue};
pub use error::LayoutError;
pub use revision::{LayoutInputRevision, LayoutSourceRevision};
pub use style::{
    FlexDirection, LayoutAlignItems, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges,
    LayoutGap, LayoutJustifyContent, LayoutLengthPercentage, LayoutStyle, TextDirection, Viewport,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RootSizingPolicy {
    MatchViewport,
    ResolveWithinViewport,
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
    pub frames: BTreeMap<NodeId, LayoutFrame>,
}

/// 입력 트리를 검증하고 논리 단위 프레임을 반환하는 내부 엔진 경계입니다.
pub trait LayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError>;
}

/// Taffy Flexbox 기반의 현재 내부 엔진입니다.
#[derive(Clone, Copy, Debug, Default)]
pub struct TaffyLayoutEngine;

impl LayoutEngine for TaffyLayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError> {
        let index = validate(input)?;
        let postorder = postorder(input, &index)?;
        let mut tree = CalcLayoutTree::new(input, &index, &postorder)?;
        let engine_root = tree.engine_id(input.root);
        let compute_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tree.compute_layout(
                engine_root,
                Size {
                    width: AvailableSpace::Definite(input.viewport.width),
                    height: AvailableSpace::Definite(input.viewport.height),
                },
            );
        }));
        if compute_result.is_err() {
            return Err(LayoutError::TaffyPanicked);
        }
        if let Some(error) = tree.take_calc_error() {
            return Err(error);
        }

        collect_frames(input, &index, &tree.engine_ids, &tree)
    }
}

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

    if input.root_sizing == RootSizingPolicy::MatchViewport {
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
    for (field, value) in [
        ("margin.top", node.style.margin.top),
        ("margin.right", node.style.margin.right),
        ("margin.bottom", node.style.margin.bottom),
        ("margin.left", node.style.margin.left),
    ] {
        if !value.is_calc() && !value.value().is_finite() {
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

fn postorder(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<Vec<NodeId>, LayoutError> {
    let mut output = Vec::with_capacity(input.nodes.len());
    let mut visited = BTreeSet::new();
    let mut pending = vec![(input.root, false)];
    while let Some((id, exiting)) = pending.pop() {
        if exiting {
            output.push(id);
            continue;
        }
        if !visited.insert(id) {
            continue;
        }
        pending.push((id, true));
        for &child in input.nodes[index[&id]].children.iter().rev() {
            pending.push((child, false));
        }
    }
    if output.len() != input.nodes.len() {
        return Err(LayoutError::UnreachableNode(
            index
                .keys()
                .find(|id| !visited.contains(id))
                .copied()
                .unwrap_or(input.root),
        ));
    }
    Ok(output)
}

fn collect_frames(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    engine_ids: &BTreeMap<NodeId, taffy::prelude::NodeId>,
    tree: &CalcLayoutTree,
) -> Result<LayoutOutput, LayoutError> {
    let mut frames = BTreeMap::new();
    let mut pending = vec![(input.root, 0.0_f32, 0.0_f32)];
    while let Some((external_id, parent_x, parent_y)) = pending.pop() {
        let engine_id = engine_ids[&external_id];
        let layout = tree
            .layout(engine_id)
            .ok_or(LayoutError::MissingComputedLayout(external_id))?;
        let frame = LayoutFrame {
            x: parent_x + layout.location.x,
            y: parent_y + layout.location.y,
            width: layout.size.width,
            height: layout.size.height,
        };
        if [frame.x, frame.y, frame.width, frame.height]
            .into_iter()
            .any(|value| !value.is_finite())
        {
            return Err(LayoutError::NonFiniteFrame(external_id));
        }
        frames.insert(external_id, frame);
        let node = &input.nodes[index[&external_id]];
        for &child in node.children.iter().rev() {
            pending.push((child, frame.x, frame.y));
        }
    }
    Ok(LayoutOutput {
        revision: input.revision,
        frames,
    })
}

#[cfg(test)]
mod tests;
