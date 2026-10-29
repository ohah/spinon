use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::GridContainerInput;
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{
    RuntimeC11_4FitContentLimitBoundariesObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{HTML_NAMESPACE, children, style_for, unsupported};

#[path = "c11_4_fit_content_limit_boundaries/validation.rs"]
mod validation;

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    root_id: &'static str,
    item_id: &'static str,
    child_id: &'static str,
    limit: f32,
    expected_row: f32,
}

const CASES: [Case; 3] = [
    Case {
        scene_id: "fit-content-limit-40-wpt",
        root_id: "c114-fit-content-limit-40-grid",
        item_id: "c114-fit-content-limit-40-item",
        child_id: "c114-fit-content-limit-40-child",
        limit: 40.0,
        expected_row: 40.0,
    },
    Case {
        scene_id: "fit-content-limit-30-wpt",
        root_id: "c114-fit-content-limit-30-grid",
        item_id: "c114-fit-content-limit-30-item",
        child_id: "c114-fit-content-limit-30-child",
        limit: 30.0,
        expected_row: 30.0,
    },
    Case {
        scene_id: "fit-content-limit-200-wpt",
        root_id: "c114-fit-content-limit-200-grid",
        item_id: "c114-fit-content-limit-200-item",
        child_id: "c114-fit-content-limit-200-child",
        limit: 200.0,
        expected_row: 100.0,
    },
];

fn case_for_scene(scene_id: &str) -> Option<Case> {
    CASES.iter().copied().find(|case| case.scene_id == scene_id)
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<(&'static str, [NodeId; 3])>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(case) = CASES.iter().copied().find(|case| case.root_id == root_id) else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id(),
            case.scene_id,
            "profile or root identity",
        ));
    }

    let root_children = children(snapshot, root)?;
    validation::require_count(
        root.id(),
        root_children.len(),
        1,
        case.scene_id,
        "grid item count",
    )?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "grid item is not a direct child",
        ));
    }
    validation::validate_identity(snapshot, item, case.item_id, case.scene_id)?;

    let item_children = children(snapshot, item)?;
    validation::require_count(
        item.id(),
        item_children.len(),
        1,
        case.scene_id,
        "nested child count",
    )?;
    let child = item_children[0];
    if snapshot.parent(child) != Some(HostParent::Node(item)) {
        return Err(unsupported(
            child.id(),
            case.scene_id,
            "nested child ownership changed",
        ));
    }
    validation::validate_identity(snapshot, child, case.child_id, case.scene_id)?;

    let node_ids = [root.id(), item.id(), child.id()];
    validation::validate_root_style(style_for(styles, root)?, case)?;
    validation::validate_item_style(style_for(styles, item)?, case)?;
    validation::validate_child_style(style_for(styles, child)?, case)?;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<std::collections::BTreeSet<_>>();
    if style_ids.len() != node_ids.len()
        || node_ids.iter().any(|node| !style_ids.contains(node))
        || styles.elements.len() != style_ids.len()
    {
        return Err(unsupported(
            root.id(),
            case.scene_id,
            "computed style node set changed",
        ));
    }
    Ok(Some((case.scene_id, node_ids)))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    scene_id: &'static str,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_inputs: &std::collections::BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 3],
) -> Result<RuntimeC11_4FitContentLimitBoundariesObservation, RuntimeLayoutFailure> {
    let case = case_for_scene(scene_id)
        .ok_or_else(|| unsupported(root.id(), scene_id, "unknown limit boundary scene"))?;
    validation::validate_root_style(style_for(styles, root)?, case)?;
    let grid_input = grid_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_limit_boundaries_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validation::validate_grid_input(root.id(), grid_input, case)?;
    if grid_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            scene_id,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentLimitBoundariesObservation {
        scene_id,
        limit_css_px: case.limit,
        expected_row_css_px: case.expected_row,
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_input.clone(),
        node_ids,
    })
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    validation::validate_output(completed)
}
