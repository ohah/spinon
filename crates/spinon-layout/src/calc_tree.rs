use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use spinon_core::NodeId as SpinonNodeId;
use taffy::{
    AvailableSpace, BlockContext, Cache, CacheTree, CoreStyle, Display, Layout,
    LayoutBlockContainer, LayoutFlexboxContainer, LayoutInput as TaffyLayoutInput,
    LayoutOutput as TaffyLayoutOutput, LayoutPartialTree, MaybeResolve, NodeId as TaffyNodeId,
    RunMode, Size, Style, TraversePartialTree, compute_block_layout, compute_cached_layout,
    compute_flexbox_layout, compute_hidden_layout, compute_leaf_layout, compute_root_layout,
};

use crate::{
    LayoutCalcId, LayoutCssMathValue, LayoutError, LayoutInput, RootSizingPolicy,
    taffy_style::{to_taffy_style, viewport_block_containing_style},
};

/// Taffy stores calc handles in the low three tag bits, so every owner is eight-byte aligned.
#[repr(align(8))]
struct CalcOwner(LayoutCssMathValue);

struct Node {
    children: Vec<TaffyNodeId>,
    style: Style,
    cache: Cache,
    layout: Layout,
}

impl Node {
    fn new(children: Vec<TaffyNodeId>, style: Style, order: u32) -> Self {
        Self {
            children,
            style,
            cache: Cache::new(),
            layout: Layout::with_order(order),
        }
    }
}

/// Taffy 0.14.0의 공개 고수준 tree resolver는 calc를 0으로 반환합니다.
/// 이 tree는 동일한 Taffy Flex·Block 알고리즘을 사용하면서 owned expression만 해석합니다.
pub(super) struct CalcLayoutTree {
    nodes: Vec<Node>,
    pub(super) engine_ids: BTreeMap<SpinonNodeId, TaffyNodeId>,
    viewport_root: Option<TaffyNodeId>,
    calc_values: HashMap<*const (), Arc<CalcOwner>>,
    calc_error: RefCell<Option<LayoutError>>,
}

impl CalcLayoutTree {
    pub(super) fn new(
        input: &LayoutInput,
        index: &BTreeMap<SpinonNodeId, usize>,
        postorder: &[SpinonNodeId],
    ) -> Result<Self, LayoutError> {
        let mut calc_owners = BTreeMap::<LayoutCalcId, Arc<CalcOwner>>::new();
        for value in &input.css_math {
            calc_owners.insert(value.id, Arc::new(CalcOwner(value.clone())));
        }
        let calc_handles = calc_owners
            .iter()
            .map(|(id, owner)| (*id, Arc::as_ptr(owner).cast::<()>()))
            .collect::<BTreeMap<_, _>>();
        let calc_values = calc_handles
            .iter()
            .map(|(id, pointer)| (*pointer, Arc::clone(&calc_owners[id])))
            .collect::<HashMap<_, _>>();

        let mut engine_ids = BTreeMap::new();
        for (position, external_id) in postorder.iter().copied().enumerate() {
            u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            let engine_id = TaffyNodeId::from(position);
            if usize::from(engine_id) != position {
                return Err(LayoutError::TooManyNodes);
            }
            engine_ids.insert(external_id, engine_id);
        }

        let mut nodes = Vec::with_capacity(postorder.len());
        for (position, external_id) in postorder.iter().copied().enumerate() {
            let source = &input.nodes[index[&external_id]];
            let children = source
                .children
                .iter()
                .map(|child| engine_ids[child])
                .collect();
            let style = to_taffy_style(external_id, source.style, &calc_handles)?;
            let order = u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            nodes.push(Node::new(children, style, order));
        }

        let viewport_root = if input.root_sizing == RootSizingPolicy::BlockFormatting {
            let position = postorder.len();
            u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            let engine_id = TaffyNodeId::from(position);
            if usize::from(engine_id) != position {
                return Err(LayoutError::TooManyNodes);
            }
            let order = u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            nodes.push(Node::new(
                vec![engine_ids[&input.root]],
                viewport_block_containing_style(input.viewport),
                order,
            ));
            Some(engine_id)
        } else {
            None
        };

        Ok(Self {
            nodes,
            engine_ids,
            viewport_root,
            calc_values,
            calc_error: RefCell::new(None),
        })
    }

    pub(super) fn engine_id(&self, id: SpinonNodeId) -> TaffyNodeId {
        self.engine_ids[&id]
    }

    pub(super) fn root_engine_id(&self, id: SpinonNodeId) -> TaffyNodeId {
        self.viewport_root.unwrap_or_else(|| self.engine_id(id))
    }

    pub(super) fn compute_layout(&mut self, root: TaffyNodeId, available: Size<AvailableSpace>) {
        compute_root_layout(self, root, available);
    }

    pub(super) fn take_calc_error(&mut self) -> Option<LayoutError> {
        self.calc_error.get_mut().take()
    }

    pub(super) fn layout(&self, id: TaffyNodeId) -> Option<Layout> {
        self.nodes.get(usize::from(id)).map(|node| node.layout)
    }

    fn node(&self, id: TaffyNodeId) -> &Node {
        &self.nodes[usize::from(id)]
    }

    fn node_mut(&mut self, id: TaffyNodeId) -> &mut Node {
        &mut self.nodes[usize::from(id)]
    }

    fn compute_child_layout_with_block_context(
        &mut self,
        node_id: TaffyNodeId,
        inputs: TaffyLayoutInput,
        block_context: Option<&mut BlockContext<'_>>,
    ) -> TaffyLayoutOutput {
        if inputs.run_mode == RunMode::PerformHiddenLayout {
            return compute_hidden_layout(self, node_id);
        }

        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let display = tree.node(node_id).style.display;
            let has_children = tree.child_count(node_id) > 0;
            match (display, has_children) {
                (Display::None, _) => compute_hidden_layout(tree, node_id),
                (Display::Block, true) => {
                    compute_block_layout(tree, node_id, inputs, block_context)
                }
                (Display::FlowRoot, true) => compute_block_layout(tree, node_id, inputs, None),
                (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
                (_, false) => {
                    let mut style = tree.node(node_id).style.clone();
                    let style_size = style
                        .size()
                        .maybe_resolve(inputs.parent_size, |value, basis| {
                            tree.resolve_calc_value(value, basis)
                        });
                    if style_size.width.is_some() && style_size.height.is_some() {
                        // Taffy 0.14's leaf path treats the preferred ratio as a minimum for
                        // height even when both CSS sizes are definite. CSS leaves both sizes
                        // untouched in that case, so keep the leaf path but disable that
                        // incompatible post-measure adjustment.
                        style.aspect_ratio = None;
                    }
                    compute_leaf_layout(
                        inputs,
                        &style,
                        |value, basis| tree.resolve_calc_value(value, basis),
                        |_, _| Size::ZERO,
                    )
                }
            }
        })
    }

    fn record_error(&self, error: LayoutError) {
        let mut slot = self.calc_error.borrow_mut();
        if slot.is_none() {
            *slot = Some(error);
        }
    }
}

impl TraversePartialTree for CalcLayoutTree {
    type ChildIter<'a> = std::iter::Copied<std::slice::Iter<'a, TaffyNodeId>>;

    fn child_ids(&self, parent_node_id: TaffyNodeId) -> Self::ChildIter<'_> {
        self.node(parent_node_id).children.iter().copied()
    }

    fn child_count(&self, node_id: TaffyNodeId) -> usize {
        self.node(node_id).children.len()
    }

    fn get_child_id(&self, node_id: TaffyNodeId, child_index: usize) -> TaffyNodeId {
        self.node(node_id).children[child_index]
    }
}

impl LayoutPartialTree for CalcLayoutTree {
    type CoreContainerStyle<'a> = &'a Style;
    type CustomIdent = String;

    fn get_core_container_style(&self, node_id: TaffyNodeId) -> Self::CoreContainerStyle<'_> {
        &self.node(node_id).style
    }

    fn resolve_calc_value(&self, value: *const (), basis: f32) -> f32 {
        let Some(owner) = self.calc_values.get(&value) else {
            self.record_error(LayoutError::UnknownCssMathHandle);
            return 0.0;
        };
        match owner.0.resolve(basis) {
            Ok(value) => value,
            Err(reason) => {
                self.record_error(owner.0.error(reason));
                0.0
            }
        }
    }

    fn set_unrounded_layout(&mut self, node_id: TaffyNodeId, layout: &Layout) {
        self.node_mut(node_id).layout = *layout;
    }

    fn compute_child_layout(
        &mut self,
        node_id: TaffyNodeId,
        inputs: TaffyLayoutInput,
    ) -> TaffyLayoutOutput {
        self.compute_child_layout_with_block_context(node_id, inputs, None)
    }
}

impl CacheTree for CalcLayoutTree {
    fn cache_get(
        &mut self,
        node_id: TaffyNodeId,
        input: &TaffyLayoutInput,
    ) -> Option<TaffyLayoutOutput> {
        self.node_mut(node_id).cache.get(input)
    }

    fn cache_store(
        &mut self,
        node_id: TaffyNodeId,
        input: &TaffyLayoutInput,
        layout_output: TaffyLayoutOutput,
    ) {
        self.node_mut(node_id).cache.store(input, layout_output);
    }

    fn cache_clear(&mut self, node_id: TaffyNodeId) {
        self.node_mut(node_id).cache.clear();
    }
}

impl LayoutFlexboxContainer for CalcLayoutTree {
    type FlexboxContainerStyle<'a> = &'a Style;
    type FlexboxItemStyle<'a> = &'a Style;

    fn get_flexbox_container_style(&self, node_id: TaffyNodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: TaffyNodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node(child_node_id).style
    }
}

impl LayoutBlockContainer for CalcLayoutTree {
    type BlockContainerStyle<'a> = &'a Style;
    type BlockItemStyle<'a> = &'a Style;

    fn get_block_container_style(&self, node_id: TaffyNodeId) -> Self::BlockContainerStyle<'_> {
        &self.node(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: TaffyNodeId) -> Self::BlockItemStyle<'_> {
        &self.node(child_node_id).style
    }

    fn compute_block_child_layout(
        &mut self,
        node_id: TaffyNodeId,
        inputs: TaffyLayoutInput,
        block_context: Option<&mut BlockContext<'_>>,
    ) -> TaffyLayoutOutput {
        self.compute_child_layout_with_block_context(node_id, inputs, block_context)
    }
}
