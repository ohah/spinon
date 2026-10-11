use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;
use taffy::prelude::{AvailableSpace, Size};

use super::{
    LayoutEngine, LayoutError, LayoutFrame, LayoutInput, LayoutOutput, TaffyLayoutEngine,
    calc_tree::CalcLayoutTree,
    flex_auto_margin::negative_cross_axis_auto_margin_offset_correction,
    flex_baseline,
    positioning::{collect_fixed_owners, collect_positioned_owners, needs_flow_pass},
};

mod positioned_flex;

impl LayoutEngine for TaffyLayoutEngine {
    fn compute(&self, input: &LayoutInput) -> Result<LayoutOutput, LayoutError> {
        let index = super::validate(input)?;
        let postorder = postorder(input, &index)?;
        let flow_frames = if needs_flow_pass(input) {
            let flow_tree = compute_tree(input, &index, &postorder, true)?;
            Some(collect_frames(input, &index, &flow_tree, &BTreeMap::new())?.frames)
        } else {
            None
        };
        let visual_tree = compute_tree(input, &index, &postorder, false)?;
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
        output.positioned_owners = collect_positioned_owners(input, &index);
        output.fixed_owners = collect_fixed_owners(input, &index);
        if let Some(flow_frames) = flow_frames {
            output.flow_frames = flow_frames;
        } else {
            output.flow_frames.clone_from(&output.frames);
        }
        Ok(output)
    }
}

fn compute_tree(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    postorder: &[NodeId],
    flow_only: bool,
) -> Result<CalcLayoutTree, LayoutError> {
    let mut tree = if flow_only {
        CalcLayoutTree::new_flow_only(input, index, postorder)?
    } else {
        CalcLayoutTree::new(input, index, postorder)?
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
        for &child in tree.layout_children(external_id).iter().rev() {
            pending.push((child, frame.x, frame.y, Some(external_id)));
        }
    }
    Ok(LayoutOutput {
        revision: input.revision,
        flow_frames: frames.clone(),
        positioned_owners: BTreeMap::new(),
        fixed_owners: BTreeMap::new(),
        frames,
    })
}
