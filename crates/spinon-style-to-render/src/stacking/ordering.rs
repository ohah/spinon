use spinon_core::NodeId;
use spinon_style::ComputedCssPosition;
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

fn paint_item_tree_order(item: &OrderedPaintItem) -> u64 {
    match item {
        OrderedPaintItem::Node(node) => node.tree_order,
        OrderedPaintItem::Context(context) => context.tree_order,
    }
}
