use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_style::{
    ComputedCssOverflow, ComputedCssPosition, ComputedCssZIndex, ComputedElementStyle,
};
use spinon_style_to_layout::StyleLayoutOutput;

use super::reserve::FallibleReserve;
use crate::StyleRenderError;

use super::{
    tree::validate_runtime_frame,
    types::{OrderedContext, OrderedNode, OrderedPaintItem, PaintAction, StackingContext},
};

pub(super) fn add_context(
    parent: &mut StackingContext,
    context: usize,
    stack_level: i32,
    tree_order: u64,
    reserve: &mut impl FallibleReserve,
) -> Result<(), StyleRenderError> {
    let ordered = OrderedContext {
        context,
        stack_level,
        tree_order,
    };
    if stack_level < 0 {
        reserve.reserve(&mut parent.negative, 1, "negative contexts")?;
        parent.negative.push(ordered);
    } else if stack_level == 0 {
        reserve.reserve(&mut parent.zero, 1, "zero contexts")?;
        parent.zero.push(OrderedPaintItem::Context(ordered));
    } else {
        reserve.reserve(&mut parent.positive, 1, "positive contexts")?;
        parent.positive.push(ordered);
    }
    Ok(())
}

pub(super) fn add_node(
    parent: &mut StackingContext,
    node: NodeId,
    tree_order: u64,
    position: ComputedCssPosition,
    is_flex_item: bool,
    reserve: &mut impl FallibleReserve,
) -> Result<(), StyleRenderError> {
    let ordered = OrderedNode { node, tree_order };
    if position != ComputedCssPosition::Static {
        reserve.reserve(&mut parent.zero, 1, "positioned auto boxes")?;
        parent.zero.push(OrderedPaintItem::Node(ordered));
    } else if is_flex_item {
        reserve.reserve(&mut parent.flex_items, 1, "flex item boxes")?;
        parent.flex_items.push(ordered);
    } else {
        reserve.reserve(&mut parent.blocks, 1, "block flow boxes")?;
        parent.blocks.push(ordered);
    }
    Ok(())
}

pub(super) fn flatten_contexts(
    document_node_count: usize,
    visible_node_count: usize,
    contexts: &mut [StackingContext],
    output: &StyleLayoutOutput,
    reserve: &mut impl FallibleReserve,
) -> Result<Vec<NodeId>, StyleRenderError> {
    for context in contexts.iter_mut() {
        context
            .negative
            .sort_unstable_by_key(|item| (item.stack_level, item.tree_order));
        context.blocks.sort_unstable_by_key(|item| item.tree_order);
        context
            .flex_items
            .sort_unstable_by_key(|item| item.tree_order);
        context.zero.sort_unstable_by_key(paint_item_tree_order);
        context
            .positive
            .sort_unstable_by_key(|item| (item.stack_level, item.tree_order));
    }
    let mut actions = Vec::new();
    reserve.reserve(&mut actions, document_node_count, "paint traversal")?;
    actions.push(PaintAction::EnterContext(0));
    let mut nodes = Vec::new();
    reserve.reserve(&mut nodes, visible_node_count, "paint nodes")?;
    while let Some(action) = actions.pop() {
        match action {
            PaintAction::EmitNode(node) => nodes.push(node),
            PaintAction::EnterContext(index) => {
                let context = contexts.get(index).ok_or(StyleRenderError::InvalidRoot)?;
                for item in context.positive.iter().rev() {
                    actions.push(PaintAction::EnterContext(item.context));
                }
                for item in context.zero.iter().rev() {
                    actions.push(match item {
                        OrderedPaintItem::Node(node) => PaintAction::EmitNode(node.node),
                        OrderedPaintItem::Context(context) => {
                            PaintAction::EnterContext(context.context)
                        }
                    });
                }
                for item in context.flex_items.iter().rev() {
                    actions.push(PaintAction::EmitNode(item.node));
                }
                for item in context.blocks.iter().rev() {
                    actions.push(PaintAction::EmitNode(item.node));
                }
                for item in context.negative.iter().rev() {
                    actions.push(PaintAction::EnterContext(item.context));
                }
                if let Some(root) = context.root {
                    let frame = output
                        .layout
                        .frames
                        .get(&root)
                        .copied()
                        .ok_or(StyleRenderError::MissingLayoutFrame(root))?;
                    validate_runtime_frame(root, frame)?;
                    if frame.width != 0.0 && frame.height != 0.0 {
                        actions.push(PaintAction::EmitNode(root));
                    }
                }
            }
        }
    }
    Ok(nodes)
}

pub(super) fn flatten_hit_test_order(
    document_nodes: &[NodeId],
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    render_nodes: &[NodeId],
    reserve: &mut impl FallibleReserve,
) -> Result<BTreeMap<NodeId, u32>, StyleRenderError> {
    let Some(root) = document_nodes.first().copied() else {
        return Err(StyleRenderError::InvalidRoot);
    };

    let mut render_rank = BTreeMap::new();
    for (rank, node) in render_nodes.iter().copied().enumerate() {
        render_rank.insert(node, rank);
    }
    let mut subtree_first_rank = BTreeMap::new();
    for (node, rank) in &render_rank {
        subtree_first_rank.insert(*node, *rank);
    }

    let mut children_by_parent = BTreeMap::<NodeId, Vec<NodeId>>::new();
    for node in document_nodes.iter().copied().skip(1) {
        let parent = parent_by_node
            .get(&node)
            .copied()
            .ok_or(StyleRenderError::InvalidRoot)?;
        let children = children_by_parent.entry(parent).or_default();
        reserve.reserve(children, 1, "hit-test children")?;
        children.push(node);
    }

    for node in document_nodes.iter().rev() {
        let Some(parent) = parent_by_node.get(node).copied() else {
            continue;
        };
        let Some(rank) = subtree_first_rank.get(node).copied() else {
            continue;
        };
        subtree_first_rank
            .entry(parent)
            .and_modify(|parent_rank| *parent_rank = (*parent_rank).min(rank))
            .or_insert(rank);
    }
    for children in children_by_parent.values_mut() {
        children.sort_by_key(|child| subtree_first_rank.get(child).copied().unwrap_or(usize::MAX));
    }

    let mut pending = Vec::new();
    reserve.reserve(&mut pending, 1, "hit-test traversal")?;
    pending.push(root);
    let mut hit_order = BTreeMap::new();
    while let Some(node) = pending.pop() {
        if render_rank.contains_key(&node) {
            let rank =
                u32::try_from(hit_order.len()).map_err(|_| StyleRenderError::PaintOrderOverflow)?;
            hit_order.insert(node, rank);
        }
        if let Some(children) = children_by_parent.get(&node) {
            reserve.reserve(&mut pending, children.len(), "hit-test traversal")?;
            pending.extend(children.iter().rev().copied());
        }
    }
    if hit_order.len() != render_nodes.len() {
        return Err(StyleRenderError::InvalidRoot);
    }
    Ok(hit_order)
}

/// C13.1 overflow clip에서 auto positioned clip 상자의 단순 in-flow subtree를 부모 배경 뒤로 둡니다.
pub(super) fn paint_auto_positioned_clip_subtrees(
    document_nodes: &[NodeId],
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    paint_nodes: &mut Vec<NodeId>,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
    reserve: &mut impl FallibleReserve,
) -> Result<(), StyleRenderError> {
    let mut children_by_parent = BTreeMap::<NodeId, Vec<NodeId>>::new();
    for (child, parent) in parent_by_node {
        let children = children_by_parent.entry(*parent).or_default();
        reserve.reserve(children, 1, "paint child map")?;
        children.push(*child);
    }

    for candidate in document_nodes {
        let style = styles
            .get(candidate)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(*candidate))?;
        if !matches!(
            style.layout_position,
            ComputedCssPosition::Absolute | ComputedCssPosition::Fixed
        ) || style.stacking_z_index != Some(ComputedCssZIndex::Auto)
            || !matches!(style.layout_overflow_x, Some(ComputedCssOverflow::Clip))
                && !matches!(style.layout_overflow_y, Some(ComputedCssOverflow::Clip))
        {
            continue;
        }

        let mut descendants = Vec::new();
        let mut pending = Vec::new();
        if let Some(children) = children_by_parent.get(candidate) {
            reserve.reserve(&mut pending, children.len(), "clip subtree paint traversal")?;
            pending.extend(children.iter().rev().copied());
        }
        let mut simple_in_flow_subtree = true;
        while let Some(node) = pending.pop() {
            let descendant_style = styles
                .get(&node)
                .copied()
                .ok_or(StyleRenderError::MissingComputedStyle(node))?;
            if descendant_style
                .properties
                .get("display")
                .map(String::as_str)
                == Some("none")
            {
                continue;
            }
            if descendant_style.layout_position != ComputedCssPosition::Static
                || descendant_style.stacking_z_index != Some(ComputedCssZIndex::Auto)
            {
                simple_in_flow_subtree = false;
                break;
            }
            reserve.reserve(&mut descendants, 1, "clip subtree descendants")?;
            descendants.push(node);
            if let Some(children) = children_by_parent.get(&node) {
                reserve.reserve(&mut pending, children.len(), "clip subtree paint traversal")?;
                pending.extend(children.iter().rev().copied());
            }
        }
        if !simple_in_flow_subtree || descendants.is_empty() {
            continue;
        }
        descendants.sort_unstable();
        let Some(parent_rank) = paint_nodes.iter().position(|node| node == candidate) else {
            continue;
        };
        if !paint_nodes
            .iter()
            .enumerate()
            .any(|(rank, node)| rank < parent_rank && descendants.binary_search(node).is_ok())
        {
            continue;
        }

        let mut moved = Vec::new();
        reserve.reserve(&mut moved, descendants.len(), "clip subtree paint order")?;
        moved.extend(
            paint_nodes
                .iter()
                .copied()
                .filter(|node| descendants.binary_search(node).is_ok()),
        );
        paint_nodes.retain(|node| descendants.binary_search(node).is_err());
        let parent_rank = paint_nodes
            .iter()
            .position(|node| node == candidate)
            .ok_or(StyleRenderError::InvalidRoot)?;
        paint_nodes.splice(parent_rank + 1..parent_rank + 1, moved);
    }
    Ok(())
}

fn paint_item_tree_order(item: &OrderedPaintItem) -> u64 {
    match item {
        OrderedPaintItem::Node(node) => node.tree_order,
        OrderedPaintItem::Context(context) => context.tree_order,
    }
}
