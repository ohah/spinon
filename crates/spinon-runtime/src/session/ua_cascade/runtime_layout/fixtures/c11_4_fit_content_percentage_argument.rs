use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::{GridContainerInput, LayoutCssMathValue, LayoutGridTrackSizingAxis};
use spinon_style::{ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{
    RuntimeC11_4FitContentPercentageArgumentObservation, RuntimeLayoutCompleted,
    RuntimeLayoutFailure, RuntimeUaCascadeKey, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{HTML_NAMESPACE, children, style_for, unsupported};
use style_validation::{
    validate_child_style, validate_grid_input, validate_item_style, validate_layout_math,
    validate_root_style,
};

const PERCENTAGE_SCENE: FitContentArgumentScene = FitContentArgumentScene {
    scene_id: "fit-content-percentage-row-wpt",
    root_id: "c114-fit-content-percentage-grid",
    item_id: "c114-fit-content-percentage-item",
    child_id: "c114-fit-content-percentage-child",
    auto_rows_css: "fit-content(50%)",
    row_size: 150.0,
};
const CALC_SCENE: FitContentArgumentScene = FitContentArgumentScene {
    scene_id: "fit-content-calc-percentage-row-wpt",
    root_id: "c114-fit-content-calc-percentage-grid",
    item_id: "c114-fit-content-calc-percentage-item",
    child_id: "c114-fit-content-calc-percentage-child",
    auto_rows_css: "fit-content(calc(10% + 5px))",
    row_size: 35.0,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct FitContentArgumentScene {
    scene_id: &'static str,
    root_id: &'static str,
    item_id: &'static str,
    child_id: &'static str,
    auto_rows_css: &'static str,
    row_size: f32,
}

fn scene_for_root_id(root_id: &str) -> Option<&'static FitContentArgumentScene> {
    [&PERCENTAGE_SCENE, &CALC_SCENE]
        .into_iter()
        .find(|scene| scene.root_id == root_id)
}

fn scene_for_id(scene_id: &str) -> Option<&'static FitContentArgumentScene> {
    [&PERCENTAGE_SCENE, &CALC_SCENE]
        .into_iter()
        .find(|scene| scene.scene_id == scene_id)
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<([NodeId; 3], &'static str)>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = scene_for_root_id(&root_id) else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id(),
            scene.scene_id,
            "profile or root identity",
        ));
    }

    let root_children = children(snapshot, root)?;
    require_child_count(
        root.id(),
        root_children.len(),
        1,
        "direct grid item count",
        scene,
    )?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            scene.scene_id,
            "grid item is not a direct child",
        ));
    }
    validate_element_identity(snapshot, item, scene.item_id, scene)?;
    let item_children = children(snapshot, item)?;
    require_child_count(
        item.id(),
        item_children.len(),
        1,
        "nested block child count",
        scene,
    )?;
    let child = item_children[0];
    if snapshot.parent(child) != Some(HostParent::Node(item)) {
        return Err(unsupported(
            child.id(),
            scene.scene_id,
            "nested child ownership changed",
        ));
    }
    validate_element_identity(snapshot, child, scene.child_id, scene)?;

    let node_ids = [root.id(), item.id(), child.id()];
    validate_root_style(style_for(styles, root)?, scene)?;
    validate_item_style(style_for(styles, item)?, scene)?;
    validate_child_style(style_for(styles, child)?, scene)?;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<BTreeSet<_>>();
    if style_ids.len() != node_ids.len()
        || node_ids.iter().any(|node| !style_ids.contains(node))
        || styles.elements.len() != style_ids.len()
    {
        return Err(unsupported(
            root.id(),
            scene.scene_id,
            "computed style node set changed",
        ));
    }
    Ok(Some((node_ids, scene.scene_id)))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_container_inputs: &BTreeMap<NodeId, GridContainerInput>,
    css_math: &[LayoutCssMathValue],
    node_ids: [NodeId; 3],
    scene_id: &'static str,
) -> Result<RuntimeC11_4FitContentPercentageArgumentObservation, RuntimeLayoutFailure> {
    let scene = scene_for_id(scene_id).ok_or_else(|| failure("unknown fit-content scene"))?;
    validate_root_style(style_for(styles, root)?, scene)?;
    let grid_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_percentage_argument_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_input, scene)?;
    validate_layout_math(root.id(), grid_input, css_math, scene)?;
    if grid_container_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            scene.scene_id,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentPercentageArgumentObservation {
        scene_id,
        css_math: css_math.to_vec().into(),
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_input.clone(),
        node_ids,
    })
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_percentage_argument
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
    if !revisions_match(
        observation.style_snapshot.style_revision,
        observation.style_snapshot.generation.get(),
        observation.style_snapshot.document_revision.get(),
        observation.style_snapshot.render_tree_revision.get(),
        diagnostics.revision,
        completed.key,
    ) {
        return Err(failure(
            "style snapshot revision does not match layout output",
        ));
    }
    if observation.style_snapshot.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || observation.node_ids[0] != observation.grid_root
        || observation
            .node_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != observation.node_ids.len()
    {
        return Err(failure(
            "fixture identity or computed style profile changed",
        ));
    }
    let scene = scene_for_id(observation.scene_id)
        .ok_or_else(|| failure("unknown fit-content scene in output"))?;
    validate_root_style(
        style_for_node(&observation.style_snapshot, observation.grid_root)?,
        scene,
    )?;
    validate_item_style(
        style_for_node(&observation.style_snapshot, observation.node_ids[1])?,
        scene,
    )?;
    validate_child_style(
        style_for_node(&observation.style_snapshot, observation.node_ids[2])?,
        scene,
    )?;
    validate_grid_input(
        observation.grid_root,
        &observation.grid_container_input,
        scene,
    )?;
    validate_layout_math(
        observation.grid_root,
        &observation.grid_container_input,
        &observation.css_math,
        scene,
    )?;

    let frames = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<BTreeMap<_, _>>();
    if completed.frames.len() != observation.node_ids.len()
        || frames.len() != observation.node_ids.len()
    {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, 60.0, 300.0),
        (0.0, 0.0, 60.0, scene.row_size),
        (0.0, 0.0, 30.0, 400.0),
    ]) {
        if frames.get(&node.get()) != Some(&expected) {
            return Err(failure(
                "layout frame differs from pinned CSS-pixel geometry",
            ));
        }
    }

    if diagnostics.traces.len() != 1
        || diagnostics.traces[0].node != observation.grid_root
        || diagnostics.traces[0].passes.len() != 2
    {
        return Err(failure(
            "expected one root trace with Columns and Rows passes",
        ));
    }
    let columns = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .count();
    let mut rows = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    let row_pass = rows
        .next()
        .ok_or_else(|| failure("row sizing pass missing"))?;
    if columns != 1 || rows.next().is_some() {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    trace_validation::validate_rows_pass(row_pass, observation.node_ids[1], scene.row_size)
}

fn validate_element_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected_id: &str,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected_id {
        return Err(unsupported(
            node.id(),
            scene.scene_id,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

fn style_for_node(
    styles: &ComputedStyleSnapshot,
    node: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    let mut matches = styles.elements.iter().filter(|style| style.node_id == node);
    let style = matches
        .next()
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node.get()), None))?;
    if matches.next().is_some() {
        return Err(layout_failure(
            "duplicate_computed_style",
            Some(node.get()),
            None,
        ));
    }
    Ok(style)
}

fn require_child_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    detail: &str,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    if actual != expected {
        return Err(unsupported(node, scene.scene_id, detail));
    }
    Ok(())
}

fn revisions_match(
    style_revision: spinon_core::StyleRevision,
    generation: u64,
    document_revision: u64,
    render_tree_revision: u64,
    layout_revision: spinon_layout::LayoutInputRevision,
    key: RuntimeUaCascadeKey,
) -> bool {
    let source_matches = match layout_revision.source() {
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: layout_generation,
            document,
            render_tree,
        } => {
            layout_generation.get() == generation
                && document.get() == document_revision
                && render_tree.get() == render_tree_revision
                && generation == key.generation
                && document_revision == key.document_revision
                && render_tree_revision == key.render_tree_revision
        }
        spinon_layout::LayoutSourceRevision::Tree(_) => false,
    };
    source_matches
        && style_revision == layout_revision.style()
        && style_revision == key.style_revision
        && layout_revision.environment().get() == key.environment_revision
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_fit_content_percentage_argument_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[path = "c11_4_fit_content_percentage_argument/trace_validation.rs"]
mod trace_validation;

#[path = "c11_4_fit_content_percentage_argument/style_validation.rs"]
mod style_validation;

#[cfg(test)]
#[path = "c11_4_fit_content_percentage_argument/tests.rs"]
mod tests;
