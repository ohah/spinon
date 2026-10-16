use spinon_core::{HostNodeHandle, NodeId};

#[derive(Clone, Copy)]
pub(super) struct OrderedNode {
    pub(super) node: NodeId,
    pub(super) tree_order: u64,
}

#[derive(Clone, Copy)]
pub(super) struct OrderedContext {
    pub(super) context: usize,
    pub(super) stack_level: i32,
    pub(super) tree_order: u64,
}

#[derive(Clone, Copy)]
pub(super) enum OrderedPaintItem {
    Node(OrderedNode),
    Context(OrderedContext),
}

#[derive(Default)]
pub(super) struct StackingContext {
    pub(super) parent: Option<usize>,
    pub(super) root: Option<NodeId>,
    pub(super) opacity: Option<f32>,
    pub(super) negative: Vec<OrderedContext>,
    pub(super) blocks: Vec<OrderedNode>,
    pub(super) flex_items: Vec<OrderedNode>,
    pub(super) zero: Vec<OrderedPaintItem>,
    pub(super) positive: Vec<OrderedContext>,
}

pub(super) struct PendingNode {
    pub(super) handle: HostNodeHandle,
    pub(super) parent_context: usize,
    pub(super) parent_is_flex: bool,
    pub(super) ancestor_display_none: bool,
}

pub(super) enum PaintAction {
    EnterContext(usize),
    EmitNode(NodeId),
}
