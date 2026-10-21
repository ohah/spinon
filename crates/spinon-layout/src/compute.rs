use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;
use spinon_text_layout::TextLayoutEngine;
use taffy::prelude::{AvailableSpace, Size};

use super::{
    GridPlacementMode, LayoutEngine, LayoutError, LayoutFrame, LayoutInput, LayoutIntrinsicSize,
    LayoutIntrinsicSizeSnapshot, LayoutOutput, LayoutUsedBoxEdges, TaffyLayoutEngine, TextFlowItem,
    TextParagraphInput,
    calc_tree::CalcLayoutTree,
    flex_auto_margin::negative_cross_axis_auto_margin_offset_correction,
    flex_baseline,
    positioning::{collect_fixed_owners, collect_positioned_owners, needs_flow_pass},
};

mod positioned_flex;

impl TaffyLayoutEngine {
    /// Decoder 측정 snapshot을 별도 계산 입력으로 사용합니다.
    pub fn compute_with_intrinsic_size_snapshot(
        &self,
        input: &LayoutInput,
        snapshot: &LayoutIntrinsicSizeSnapshot,
    ) -> Result<LayoutOutput, LayoutError> {
        self.compute_with_measurements(input, Some(snapshot), None)
    }

    /// Shaped paragraph 입력을 Taffy measure와 final layout에 연결합니다.
    pub fn compute_with_text_layout_engine(
        &self,
        input: &LayoutInput,
        text_layout_engine: &TextLayoutEngine,
    ) -> Result<LayoutOutput, LayoutError> {
        self.compute_with_measurements(input, None, Some(text_layout_engine))
    }

    /// C11.4 fixture에서 text paragraph와 replaced-resource snapshot을 한 계산에 연결합니다.
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub fn compute_with_intrinsic_size_snapshot_and_text_layout_engine(
        &self,
        input: &LayoutInput,
        snapshot: &LayoutIntrinsicSizeSnapshot,
        text_layout_engine: &TextLayoutEngine,
    ) -> Result<LayoutOutput, LayoutError> {
        self.compute_with_measurements(input, Some(snapshot), Some(text_layout_engine))
    }

    fn compute_with_measurements(
        &self,
        input: &LayoutInput,
        snapshot: Option<&LayoutIntrinsicSizeSnapshot>,
        text_layout_engine: Option<&TextLayoutEngine>,
    ) -> Result<LayoutOutput, LayoutError> {
        let intrinsic_size_nodes = snapshot
            .map(|snapshot| snapshot.sizes().keys().copied().collect::<BTreeSet<_>>())
            .unwrap_or_default();
        let index = super::validate(input, &intrinsic_size_nodes)?;
        validate_intrinsic_sizes(input, &index, snapshot)?;
        validate_text_paragraphs(input, &index, text_layout_engine.is_some())?;
        let postorder = postorder(input, &index)?;
        let intrinsic_sizes = snapshot.map(LayoutIntrinsicSizeSnapshot::sizes);
        let flow_frames = if needs_flow_pass(input) {
            let flow_tree = compute_tree(
                input,
                &index,
                &postorder,
                intrinsic_sizes,
                true,
                text_layout_engine,
            )?;
            Some(collect_frames(input, &index, &flow_tree, &BTreeMap::new())?.frames)
        } else {
            None
        };
        let mut visual_tree = compute_tree(
            input,
            &index,
            &postorder,
            intrinsic_sizes,
            false,
            text_layout_engine,
        )?;
        let raw_output = collect_frames(input, &index, &visual_tree, &BTreeMap::new())?;
        let adjustments = flow_frames
            .as_ref()
            .map(|flow| {
                positioned_flex::absolute_static_position_adjustments(
                    input,
                    &index,
                    &raw_output.frames,
                    flow,
                )
            })
            .transpose()?
            .unwrap_or_default();
        let mut output = if adjustments.is_empty() {
            raw_output
        } else {
            collect_frames(input, &index, &visual_tree, &adjustments)?
        };
        let (text_layouts, text_flow_layouts) = visual_tree.final_text_layouts(input)?;
        output.text_layouts = text_layouts;
        output.text_flow_layouts = text_flow_layouts;
        output.positioned_owners = collect_positioned_owners(input, &index);
        output.fixed_owners = collect_fixed_owners(input, &index);
        if let Some(flow_frames) = flow_frames {
            output.flow_frames = flow_frames;
        } else {
            output.flow_frames.clone_from(&output.frames);
        }
        if let Some(snapshot) = snapshot {
            output.revision = output
                .revision
                .with_intrinsic_size_revision(snapshot.revision());
        }
        output.intrinsic_measurements = visual_tree.take_intrinsic_measurements(output.revision);
        output.grid_item_intrinsic_inputs =
            visual_tree.collect_grid_item_intrinsic_inputs(output.revision)?;
        output.grid_track_sizing_traces = visual_tree.take_grid_track_sizing_traces()?;
        Ok(output)
    }
}

impl LayoutEngine for TaffyLayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError> {
        self.compute_with_measurements(input, None, None)
    }
}

fn validate_text_paragraphs(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    has_text_engine: bool,
) -> Result<(), LayoutError> {
    if let Some((&node, _)) = input.text_paragraphs.iter().next()
        && !has_text_engine
    {
        return Err(LayoutError::MissingTextLayoutEngine(node));
    }
    let mut hidden = BTreeSet::new();
    let mut pending = vec![(input.root(), false)];
    while let Some((id, ancestor_hidden)) = pending.pop() {
        let node = &input.nodes()[index[&id]];
        let is_hidden = ancestor_hidden || node.style.display == crate::LayoutDisplay::None;
        if is_hidden {
            hidden.insert(id);
        }
        pending.extend(node.children.iter().map(|&child| (child, is_hidden)));
    }
    for (&id, paragraph_input) in &input.text_paragraphs {
        let Some(&position) = index.get(&id) else {
            return Err(LayoutError::UnknownTextParagraph(id));
        };
        let node = &input.nodes()[position];
        if node.style.display == crate::LayoutDisplay::None || hidden.contains(&id) {
            return Err(LayoutError::UnsupportedTextParagraph {
                node: id,
                reason: "숨겨진 subtree에는 paragraph snapshot을 입력할 수 없습니다",
            });
        }
        if node.style.display != crate::LayoutDisplay::Block {
            return Err(LayoutError::UnsupportedTextParagraph {
                node: id,
                reason: "paragraph 또는 flow owner는 display:block이어야 합니다",
            });
        }
        match paragraph_input {
            TextParagraphInput::Paragraph(paragraph) => {
                if paragraph.node != id {
                    return Err(LayoutError::UnsupportedTextParagraph {
                        node: id,
                        reason: "paragraph owner ID와 layout node ID가 다릅니다",
                    });
                }
                if node.children.iter().any(|child| !hidden.contains(child)) {
                    return Err(LayoutError::UnsupportedTextParagraph {
                        node: id,
                        reason: "visible 자식 box가 없는 Block paragraph만 허용됩니다",
                    });
                }
            }
            TextParagraphInput::Flow(flow) => {
                if flow.owner != id || flow.items.is_empty() {
                    return Err(LayoutError::UnsupportedTextParagraph {
                        node: id,
                        reason: "anonymous flow owner 또는 item 목록이 잘못되었습니다",
                    });
                }
                let mut flow_blocks = Vec::new();
                for item in &flow.items {
                    match item {
                        TextFlowItem::Paragraph(paragraph)
                            if paragraph.node == id && !paragraph.sources.is_empty() => {}
                        TextFlowItem::Block(block) => {
                            if flow_blocks.contains(block) {
                                return Err(LayoutError::UnsupportedTextParagraph {
                                    node: id,
                                    reason: "anonymous flow에 같은 block child가 반복됩니다",
                                });
                            }
                            let Some(&child_position) = index.get(block) else {
                                return Err(LayoutError::UnsupportedTextParagraph {
                                    node: id,
                                    reason: "anonymous flow block child가 layout graph에 없습니다",
                                });
                            };
                            if input.nodes()[child_position].style.display
                                != crate::LayoutDisplay::Block
                            {
                                return Err(LayoutError::UnsupportedTextParagraph {
                                    node: id,
                                    reason: "anonymous flow child는 display:block이어야 합니다",
                                });
                            }
                            if input.positioning.get(block).is_some_and(|positioning| {
                                matches!(
                                    positioning.position,
                                    crate::LayoutPosition::Absolute | crate::LayoutPosition::Fixed
                                )
                            }) {
                                return Err(LayoutError::UnsupportedTextParagraph {
                                    node: id,
                                    reason: "anonymous flow에 out-of-flow block child를 넣을 수 없습니다",
                                });
                            }
                            flow_blocks.push(*block);
                        }
                        TextFlowItem::Paragraph(_) => {
                            return Err(LayoutError::UnsupportedTextParagraph {
                                node: id,
                                reason: "anonymous paragraph owner 또는 source가 잘못되었습니다",
                            });
                        }
                    }
                }
                let visible_children = node
                    .children
                    .iter()
                    .copied()
                    .filter(|child| !hidden.contains(child))
                    .collect::<Vec<_>>();
                if flow_blocks.is_empty() || flow_blocks != visible_children {
                    return Err(LayoutError::UnsupportedTextParagraph {
                        node: id,
                        reason: "anonymous flow block children must match visible layout children in source order",
                    });
                }
            }
        }
    }
    Ok(())
}

fn validate_intrinsic_sizes(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    snapshot: Option<&LayoutIntrinsicSizeSnapshot>,
) -> Result<(), LayoutError> {
    let Some(snapshot) = snapshot else {
        return Ok(());
    };
    if snapshot.source() != input.source_revision() {
        return Err(LayoutError::IntrinsicSizeSourceMismatch {
            expected: input.source_revision(),
            actual: snapshot.source(),
        });
    }
    let mut hidden = BTreeSet::new();
    let mut pending = vec![(input.root(), false)];
    while let Some((id, ancestor_hidden)) = pending.pop() {
        let node = &input.nodes()[index[&id]];
        let is_hidden = ancestor_hidden || node.style.display == crate::LayoutDisplay::None;
        if is_hidden {
            hidden.insert(id);
        }
        pending.extend(node.children.iter().map(|&child| (child, is_hidden)));
    }
    let mut track_sizing_grid_items = BTreeSet::new();
    for (&container, grid) in &input.grid_containers {
        if grid.placement_mode == GridPlacementMode::TrackSizing
            && let Some(&container_index) = index.get(&container)
        {
            track_sizing_grid_items.extend(input.nodes()[container_index].children.iter().copied());
        }
    }
    for (&id, size) in snapshot.sizes() {
        let Some(&position) = index.get(&id) else {
            return Err(LayoutError::UnknownIntrinsicSizeNode(id));
        };
        let node = &input.nodes()[position];
        if !node.children.is_empty() {
            return Err(LayoutError::UnsupportedIntrinsicSizing {
                node: id,
                reason: "측정 leaf에 layout 자식이 있습니다",
            });
        }
        let zero_size_pending =
            size.width == Some(0.0) && size.height == Some(0.0) && size.aspect_ratio.is_none();
        for (axis, value) in [("width", size.width), ("height", size.height)] {
            if value.is_some_and(|value| !value.is_finite() || (value <= 0.0 && !zero_size_pending))
            {
                return Err(LayoutError::InvalidIntrinsicSize { node: id, axis });
            }
        }
        if size
            .aspect_ratio
            .is_some_and(|ratio| !ratio.is_finite() || ratio <= 0.0)
        {
            return Err(LayoutError::InvalidIntrinsicSize {
                node: id,
                axis: "aspect ratio",
            });
        }
        if let (Some(width), Some(height), Some(ratio)) =
            (size.width, size.height, size.aspect_ratio)
        {
            let dimensions_ratio = width / height;
            if !dimensions_ratio.is_finite()
                || dimensions_ratio <= 0.0
                || (dimensions_ratio - ratio).abs()
                    > 0.0001 * dimensions_ratio.abs().max(ratio.abs()).max(1.0)
            {
                return Err(LayoutError::InvalidIntrinsicSize {
                    node: id,
                    axis: "aspect ratio",
                });
            }
        }
        if !zero_size_pending && let (Some(width), Some(height)) = (size.width, size.height) {
            let ratio = width / height;
            if !ratio.is_finite() || ratio <= 0.0 {
                return Err(LayoutError::InvalidIntrinsicSize {
                    node: id,
                    axis: "aspect ratio",
                });
            }
        }
        if !hidden.contains(&id) {
            let width_auto = node.style.width == crate::LayoutDimension::Auto;
            let height_auto = node.style.height == crate::LayoutDimension::Auto;
            if width_auto
                && height_auto
                && size.width.is_none()
                && size.height.is_none()
                && size.aspect_ratio.is_some()
            {
                return Err(LayoutError::UnsupportedIntrinsicSizing {
                    node: id,
                    reason: "ratio만 있는 auto/auto intrinsic contribution은 지원하지 않습니다",
                });
            }
            let is_track_sizing_grid_item = track_sizing_grid_items.contains(&id);
            if node.style.aspect_ratio.is_some()
                && [
                    node.style.min_width,
                    node.style.max_width,
                    node.style.min_height,
                    node.style.max_height,
                ]
                .into_iter()
                .any(|dimension| dimension != crate::LayoutDimension::Auto)
                && !is_track_sizing_grid_item
            {
                return Err(LayoutError::UnsupportedIntrinsicSizing {
                    node: id,
                    reason: "aspect-ratio와 authored min/max 조합은 지원하지 않습니다",
                });
            }
        }
    }
    Ok(())
}

fn compute_tree(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    postorder: &[NodeId],
    intrinsic_sizes: Option<&BTreeMap<NodeId, LayoutIntrinsicSize>>,
    flow_only: bool,
    text_layout_engine: Option<&TextLayoutEngine>,
) -> Result<CalcLayoutTree, LayoutError> {
    let mut tree = if flow_only {
        CalcLayoutTree::new_flow_only(input, index, postorder, intrinsic_sizes, text_layout_engine)?
    } else {
        CalcLayoutTree::new(input, index, postorder, intrinsic_sizes, text_layout_engine)?
    };
    let compute_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tree.compute_layout(
            tree.root_engine_id(input.root),
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
    flex_baseline::apply(input, index, postorder, &mut tree)?;
    Ok(tree)
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
    tree: &CalcLayoutTree,
    adjustments: &BTreeMap<NodeId, (f32, f32)>,
) -> Result<LayoutOutput, LayoutError> {
    let mut frames = BTreeMap::new();
    let mut used_box_edges = BTreeMap::new();
    let mut pending = tree
        .root_children(input.root)
        .into_iter()
        .rev()
        .map(|id| (id, 0.0_f32, 0.0_f32, None))
        .collect::<Vec<_>>();
    while let Some((external_id, parent_x, parent_y, parent_id)) = pending.pop() {
        let engine_id = tree.engine_id(external_id);
        let layout = tree
            .layout(engine_id)
            .ok_or(LayoutError::MissingComputedLayout(external_id))?;
        let (correction_x, correction_y) = parent_id
            .map(|parent_id| {
                negative_cross_axis_auto_margin_offset_correction(
                    input.nodes[index[&parent_id]].style,
                    input.nodes[index[&external_id]].style,
                    layout,
                )
            })
            .unwrap_or((0.0, 0.0));
        let frame = LayoutFrame {
            x: parent_x
                + layout.location.x
                + correction_x
                + adjustments.get(&external_id).map_or(0.0, |value| value.0),
            y: parent_y
                + layout.location.y
                + correction_y
                + adjustments.get(&external_id).map_or(0.0, |value| value.1),
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
        used_box_edges.insert(
            external_id,
            LayoutUsedBoxEdges {
                padding: crate::LayoutBorder {
                    top: layout.padding.top,
                    right: layout.padding.right,
                    bottom: layout.padding.bottom,
                    left: layout.padding.left,
                },
                border: crate::LayoutBorder {
                    top: layout.border.top,
                    right: layout.border.right,
                    bottom: layout.border.bottom,
                    left: layout.border.left,
                },
            },
        );
        for &child in tree.layout_children(external_id).iter().rev() {
            pending.push((child, frame.x, frame.y, Some(external_id)));
        }
    }
    Ok(LayoutOutput {
        revision: input.revision,
        flow_frames: frames.clone(),
        used_box_edges,
        positioned_owners: BTreeMap::new(),
        fixed_owners: BTreeMap::new(),
        text_layouts: BTreeMap::new(),
        text_flow_layouts: BTreeMap::new(),
        intrinsic_measurements: Vec::new(),
        grid_item_intrinsic_inputs: Vec::new(),
        grid_track_sizing_traces: Vec::new(),
        frames,
    })
}
