use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use spinon_core::NodeId as SpinonNodeId;
use spinon_text_layout::{ParagraphInput, TextLayoutEngine, TextLayoutSnapshot};
use taffy::{
    AvailableSpace, BlockContext, Cache, CacheTree, Display, Layout, LayoutBlockContainer,
    LayoutFlexboxContainer, LayoutInput as TaffyLayoutInput, LayoutOutput as TaffyLayoutOutput,
    LayoutPartialTree, MaybeResolve, NodeId as TaffyNodeId, RunMode, Size, SizingMode,
    TraversePartialTree, compute_block_layout, compute_cached_layout, compute_flexbox_layout,
    compute_grid_layout, compute_hidden_layout, compute_leaf_layout, compute_root_layout,
};

use self::replaced_sizing::{ReplacedSizingInput, resolve_used_size};
use crate::{
    LayoutCalcId, LayoutCssMathValue, LayoutError, LayoutInput, LayoutIntrinsicSize,
    LayoutPosition, LayoutTextSnapshot, PositionedContainingBlockOwner, RootSizingPolicy,
    compact_taffy_style::CompactTaffyStyle,
    grid_placement::normalize_grid_layout,
    positioning::collect_positioned_owners,
    taffy_style::{to_taffy_style, viewport_block_containing_style},
};

#[path = "calc_tree/grid.rs"]
mod grid;
#[path = "calc_tree/replaced_sizing.rs"]
mod replaced_sizing;

/// Taffy stores calc handles in the low three tag bits, so every owner is eight-byte aligned.
#[repr(align(8))]
struct CalcOwner(LayoutCssMathValue);

struct Node {
    children: Vec<TaffyNodeId>,
    style: CompactTaffyStyle,
    replaced_sizing: Option<ReplacedSizingInput>,
    cache: Cache,
    layout: Layout,
}

impl Node {
    fn new(
        children: Vec<TaffyNodeId>,
        style: CompactTaffyStyle,
        replaced_sizing: Option<ReplacedSizingInput>,
        order: u32,
    ) -> Self {
        Self {
            children,
            style,
            replaced_sizing,
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
    layout_children: BTreeMap<SpinonNodeId, Vec<SpinonNodeId>>,
    viewport_children: Vec<SpinonNodeId>,
    viewport_root: Option<TaffyNodeId>,
    calc_values: HashMap<*const (), Arc<CalcOwner>>,
    calc_error: RefCell<Option<LayoutError>>,
    text_layout_engine: Option<RefCell<TextLayoutEngine>>,
    text_paragraphs: HashMap<TaffyNodeId, ParagraphInput>,
    text_layouts: RefCell<HashMap<TaffyNodeId, BTreeMap<u32, TextLayoutSnapshot>>>,
}

impl CalcLayoutTree {
    pub(super) fn new(
        input: &LayoutInput,
        index: &BTreeMap<SpinonNodeId, usize>,
        postorder: &[SpinonNodeId],
        intrinsic_sizes: Option<&BTreeMap<SpinonNodeId, LayoutIntrinsicSize>>,
        text_layout_engine: Option<&TextLayoutEngine>,
    ) -> Result<Self, LayoutError> {
        Self::new_with_inset_mode(
            input,
            index,
            postorder,
            intrinsic_sizes,
            false,
            text_layout_engine,
        )
    }

    pub(super) fn new_flow_only(
        input: &LayoutInput,
        index: &BTreeMap<SpinonNodeId, usize>,
        postorder: &[SpinonNodeId],
        intrinsic_sizes: Option<&BTreeMap<SpinonNodeId, LayoutIntrinsicSize>>,
        text_layout_engine: Option<&TextLayoutEngine>,
    ) -> Result<Self, LayoutError> {
        Self::new_with_inset_mode(
            input,
            index,
            postorder,
            intrinsic_sizes,
            true,
            text_layout_engine,
        )
    }

    fn new_with_inset_mode(
        input: &LayoutInput,
        index: &BTreeMap<SpinonNodeId, usize>,
        postorder: &[SpinonNodeId],
        intrinsic_sizes: Option<&BTreeMap<SpinonNodeId, LayoutIntrinsicSize>>,
        flow_only: bool,
        text_layout_engine: Option<&TextLayoutEngine>,
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
        let text_paragraphs = input
            .text_paragraphs
            .iter()
            .map(|(node, paragraph)| (engine_ids[node], paragraph.clone()))
            .collect();

        let mut layout_children = input
            .nodes
            .iter()
            .map(|node| (node.id, node.children.clone()))
            .collect::<BTreeMap<_, _>>();
        let normalized_grid = normalize_grid_layout(input, index)?;
        let mut viewport_children = Vec::new();
        if !flow_only {
            reparent_out_of_flow_children(
                input,
                index,
                postorder,
                &mut layout_children,
                &mut viewport_children,
            )?;
        }

        let mut nodes = Vec::with_capacity(postorder.len() + 1);
        for (position, external_id) in postorder.iter().copied().enumerate() {
            let source = &input.nodes[index[&external_id]];
            let mut children = layout_children[&external_id]
                .iter()
                .map(|child| engine_ids[child])
                .collect::<Vec<_>>();
            if source.style.display == crate::LayoutDisplay::Flex {
                // CSS order 값이 같으면 원본 순서를 유지하도록 안정 정렬합니다.
                children.sort_by_key(|child| {
                    let child_id = postorder[usize::from(*child)];
                    input.nodes[index[&child_id]].style.order
                });
            }
            let positioning = input
                .positioning
                .get(&external_id)
                .copied()
                .unwrap_or_default();
            let replaced_sizing = intrinsic_sizes
                .and_then(|sizes| sizes.get(&external_id).copied())
                .map(|natural| ReplacedSizingInput {
                    node: external_id,
                    style: source.style,
                    natural,
                });
            let style = to_taffy_style(
                external_id,
                source.style,
                replaced_sizing.is_some(),
                positioning,
                flow_only,
                &calc_handles,
                crate::taffy_style::GridTaffyStyleInput {
                    container: input.grid_containers.get(&external_id),
                    expanded_rows: normalized_grid
                        .expanded_rows
                        .get(&external_id)
                        .map(Vec::as_slice),
                    expanded_columns: normalized_grid
                        .expanded_columns
                        .get(&external_id)
                        .map(Vec::as_slice),
                    item: normalized_grid.item_placements.get(&external_id).copied(),
                },
            )?;
            let order = u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            nodes.push(Node::new(children, style, replaced_sizing, order));
        }

        let needs_viewport_containing_block = input.root_sizing
            == RootSizingPolicy::BlockFormatting
            || input
                .positioning
                .get(&input.root)
                .is_some_and(|positioning| positioning.position == LayoutPosition::Relative)
            || !viewport_children.is_empty();
        let viewport_root = if needs_viewport_containing_block {
            let position = postorder.len();
            u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            let engine_id = TaffyNodeId::from(position);
            if usize::from(engine_id) != position {
                return Err(LayoutError::TooManyNodes);
            }
            let order = u32::try_from(position).map_err(|_| LayoutError::TooManyNodes)?;
            let mut root_children = Vec::with_capacity(viewport_children.len() + 1);
            root_children.push(engine_ids[&input.root]);
            root_children.extend(viewport_children.iter().map(|id| engine_ids[id]));
            nodes.push(Node::new(
                root_children,
                viewport_block_containing_style(input.viewport),
                None,
                order,
            ));
            Some(engine_id)
        } else {
            None
        };

        Ok(Self {
            nodes,
            engine_ids,
            layout_children,
            viewport_children,
            viewport_root,
            calc_values,
            calc_error: RefCell::new(None),
            text_layout_engine: text_layout_engine.cloned().map(RefCell::new),
            text_paragraphs,
            text_layouts: RefCell::new(HashMap::new()),
        })
    }

    pub(super) fn engine_id(&self, id: SpinonNodeId) -> TaffyNodeId {
        self.engine_ids[&id]
    }

    pub(super) fn root_engine_id(&self, id: SpinonNodeId) -> TaffyNodeId {
        self.viewport_root.unwrap_or_else(|| self.engine_id(id))
    }

    pub(super) fn layout_children(&self, id: SpinonNodeId) -> &[SpinonNodeId] {
        &self.layout_children[&id]
    }

    pub(super) fn root_children(&self, id: SpinonNodeId) -> Vec<SpinonNodeId> {
        if self.viewport_root.is_some() {
            let mut children = Vec::with_capacity(self.viewport_children.len() + 1);
            children.push(id);
            children.extend(self.viewport_children.iter().copied());
            children
        } else {
            vec![id]
        }
    }

    pub(super) fn compute_layout(&mut self, root: TaffyNodeId, available: Size<AvailableSpace>) {
        compute_root_layout(self, root, available);
    }

    pub(super) fn take_calc_error(&mut self) -> Option<LayoutError> {
        self.calc_error.get_mut().take()
    }

    pub(super) fn final_text_layouts(
        &self,
        input: &LayoutInput,
    ) -> Result<BTreeMap<SpinonNodeId, LayoutTextSnapshot>, LayoutError> {
        let mut result = BTreeMap::new();
        let text_layouts = self.text_layouts.borrow();
        for (&engine_id, paragraph) in &self.text_paragraphs {
            let layout = self
                .layout(engine_id)
                .ok_or(LayoutError::MissingComputedLayout(paragraph.node))?;
            let content_width = layout.size.width
                - layout.border.left
                - layout.border.right
                - layout.padding.left
                - layout.padding.right;
            let normalized_content_width = if content_width == 0.0 {
                0.0
            } else {
                content_width
            };
            let width_key = normalized_content_width.to_bits();
            let snapshot = text_layouts
                .get(&engine_id)
                .and_then(|by_width| by_width.get(&width_key))
                .ok_or(LayoutError::TextLayoutWidthMismatch(paragraph.node))?;
            if snapshot.available_width.to_bits() != width_key {
                return Err(LayoutError::TextLayoutWidthMismatch(paragraph.node));
            }
            let node_style = input.nodes().iter().find(|node| node.id == paragraph.node);
            let content_height = layout.size.height
                - layout.border.top
                - layout.border.bottom
                - layout.padding.top
                - layout.padding.bottom;
            if node_style.is_some_and(|node| node.style.height == crate::LayoutDimension::Auto)
                && (content_height - snapshot.height).abs() > 0.05
            {
                return Err(LayoutError::TextLayoutHeightMismatch(paragraph.node));
            }
            result.insert(
                paragraph.node,
                LayoutTextSnapshot {
                    revision: input.revision(),
                    input: paragraph.clone(),
                    layout: snapshot.clone(),
                },
            );
        }
        Ok(result)
    }

    pub(super) fn layout(&self, id: TaffyNodeId) -> Option<Layout> {
        self.nodes.get(usize::from(id)).map(|node| node.layout)
    }

    pub(crate) fn layout_for(&self, id: SpinonNodeId) -> Option<Layout> {
        self.layout(self.engine_id(id))
    }

    pub(crate) fn shift_y(&mut self, id: SpinonNodeId, delta: f32) -> Result<(), LayoutError> {
        let engine_id = self.engine_id(id);
        let node = self.node_mut(engine_id);
        let y = node.layout.location.y + delta;
        if !y.is_finite() {
            return Err(LayoutError::NonFiniteFrame(id));
        }
        node.layout.location.y = y;
        Ok(())
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
            let display = tree.node(node_id).style.display();
            let has_children = tree.child_count(node_id) > 0;
            match (display, has_children) {
                (Display::None, _) => compute_hidden_layout(tree, node_id),
                (Display::Block, true) => {
                    compute_block_layout(tree, node_id, inputs, block_context)
                }
                (Display::FlowRoot, true) => compute_block_layout(tree, node_id, inputs, None),
                (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
                (Display::Grid, true) => compute_grid_layout(tree, node_id, inputs),
                (_, false) => {
                    let mut inputs = inputs;
                    let mut style = tree.node(node_id).style.clone();
                    let replaced_sizing = tree.node(node_id).replaced_sizing;
                    let inherent_size = inputs.sizing_mode == SizingMode::InherentSize;
                    let parent_size = inputs.parent_size;
                    let used_replaced_size = if inherent_size {
                        replaced_sizing.map(|input| {
                            match resolve_used_size(
                                input.node,
                                input,
                                inputs.known_dimensions,
                                true,
                                parent_size,
                                inputs.available_space,
                            ) {
                                Ok(size) => size,
                                Err(error) => {
                                    tree.record_error(error);
                                    replaced_sizing::UsedReplacedSize {
                                        content_width: 0.0,
                                        content_height: 0.0,
                                        width: 0.0,
                                        height: 0.0,
                                    }
                                }
                            }
                        })
                    } else {
                        None
                    };
                    if let Some(size) = used_replaced_size {
                        style.set_used_replaced_border_box_size(size.width, size.height);
                        inputs.known_dimensions = Size {
                            width: Some(size.width),
                            height: Some(size.height),
                        };
                    }
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
                        style.disable_aspect_ratio();
                    }
                    compute_leaf_layout(
                        inputs,
                        &style,
                        |value, basis| tree.resolve_calc_value(value, basis),
                        |known_dimensions, available_space| {
                            if tree.text_paragraphs.contains_key(&node_id) {
                                match tree.measure_text(node_id, available_space.width) {
                                    Ok(size) => size,
                                    Err(error) => {
                                        tree.record_error(error);
                                        Size::ZERO
                                    }
                                }
                            } else {
                                replaced_sizing.map_or(Size::ZERO, |input| match resolve_used_size(
                                    input.node,
                                    input,
                                    known_dimensions,
                                    inherent_size,
                                    parent_size,
                                    available_space,
                                ) {
                                    Ok(size) => Size {
                                        width: size.content_width,
                                        height: size.content_height,
                                    },
                                    Err(error) => {
                                        tree.record_error(error);
                                        Size::ZERO
                                    }
                                })
                            }
                        },
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

    fn measure_text(
        &self,
        node_id: TaffyNodeId,
        available_width: AvailableSpace,
    ) -> Result<Size<f32>, LayoutError> {
        let Some(paragraph) = self.text_paragraphs.get(&node_id).cloned() else {
            return Ok(Size::ZERO);
        };
        let AvailableSpace::Definite(width) = available_width else {
            return Err(LayoutError::IndefiniteTextAvailableWidth(paragraph.node));
        };
        let width = if width == 0.0 { 0.0 } else { width };
        let engine = self
            .text_layout_engine
            .as_ref()
            .ok_or(LayoutError::MissingTextLayoutEngine(paragraph.node))?;
        let snapshot = engine
            .borrow_mut()
            .layout_paragraph(&paragraph, width)
            .map_err(LayoutError::TextLayout)?;
        self.text_layouts
            .borrow_mut()
            .entry(node_id)
            .or_default()
            .insert(width.to_bits(), snapshot.clone());
        Ok(Size {
            width: snapshot.width,
            height: snapshot.height,
        })
    }
}

fn reparent_out_of_flow_children(
    input: &LayoutInput,
    index: &BTreeMap<SpinonNodeId, usize>,
    postorder: &[SpinonNodeId],
    children: &mut BTreeMap<SpinonNodeId, Vec<SpinonNodeId>>,
    viewport_children: &mut Vec<SpinonNodeId>,
) -> Result<(), LayoutError> {
    let owners = collect_positioned_owners(input, index);
    let mut source_parents = BTreeMap::new();
    for node in &input.nodes {
        for &child in &node.children {
            source_parents.insert(child, node.id);
        }
    }

    for &id in postorder {
        let Some(positioning) = input.positioning.get(&id) else {
            continue;
        };
        let destination = match positioning.position {
            LayoutPosition::Absolute => owners[&id],
            LayoutPosition::Fixed if owners[&id] == PositionedContainingBlockOwner::NoBox => {
                PositionedContainingBlockOwner::NoBox
            }
            LayoutPosition::Fixed => PositionedContainingBlockOwner::Viewport,
            LayoutPosition::Static | LayoutPosition::Relative => continue,
        };
        if destination == PositionedContainingBlockOwner::NoBox {
            continue;
        }
        let source_parent =
            source_parents
                .get(&id)
                .copied()
                .ok_or(LayoutError::UnsupportedPositioning {
                    node: id,
                    reason: "out-of-flow 노드의 원본 부모를 찾을 수 없습니다",
                })?;
        let destination_parent = match destination {
            PositionedContainingBlockOwner::Viewport => None,
            PositionedContainingBlockOwner::Node(owner) => Some(owner),
            PositionedContainingBlockOwner::NoBox => unreachable!(),
        };
        if destination_parent == Some(source_parent) {
            continue;
        }

        let source_children =
            children
                .get_mut(&source_parent)
                .ok_or(LayoutError::UnsupportedPositioning {
                    node: id,
                    reason: "out-of-flow 노드의 원본 layout parent가 없습니다",
                })?;
        let Some(source_position) = source_children.iter().position(|&child| child == id) else {
            return Err(LayoutError::UnsupportedPositioning {
                node: id,
                reason: "out-of-flow 노드를 원본 layout parent에서 찾을 수 없습니다",
            });
        };
        source_children.remove(source_position);
        if let Some(owner) = destination_parent {
            children
                .get_mut(&owner)
                .ok_or(LayoutError::UnsupportedPositioning {
                    node: id,
                    reason: "positioned containing block owner가 layout graph에 없습니다",
                })?
                .push(id);
        } else {
            viewport_children.push(id);
        }
    }
    Ok(())
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
    type CoreContainerStyle<'a> = &'a CompactTaffyStyle;
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
    type FlexboxContainerStyle<'a> = &'a CompactTaffyStyle;
    type FlexboxItemStyle<'a> = &'a CompactTaffyStyle;

    fn get_flexbox_container_style(&self, node_id: TaffyNodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: TaffyNodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node(child_node_id).style
    }
}

impl LayoutBlockContainer for CalcLayoutTree {
    type BlockContainerStyle<'a> = &'a CompactTaffyStyle;
    type BlockItemStyle<'a> = &'a CompactTaffyStyle;

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
