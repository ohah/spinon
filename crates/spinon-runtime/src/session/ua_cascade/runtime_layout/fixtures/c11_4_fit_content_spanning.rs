use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::GridContainerInput;
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{
    RuntimeC11_4FitContentSpanningObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{HTML_NAMESPACE, children, style_for, unsupported};

#[path = "c11_4_fit_content_spanning/validation.rs"]
mod validation;

const SCENE_ID: &str = "fit-content-spanning-wpt";
const ROOT_ID: &str = "c114-fit-content-spanning-grid";
const ITEM_ID: &str = "c114-fit-content-spanning-item";
const CHILD_ID: &str = "c114-fit-content-spanning-child";
const WIDTH: f32 = 60.0;
const HEIGHT: f32 = 300.0;
const FIT_LIMIT: f32 = 40.0;
const CHILD_HEIGHT: f32 = 200.0;
const EPSILON: f32 = 1.0e-4;

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<[NodeId; 3]>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if root_id != ROOT_ID {
        return Ok(None);
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), SCENE_ID, "profile or root identity"));
    }

    let root_children = children(snapshot, root)?;
    validation::require_count(root.id(), root_children.len(), 1, "direct grid item count")?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            SCENE_ID,
            "grid item is not a direct child",
        ));
    }
    validation::validate_identity(snapshot, item, ITEM_ID)?;

    let item_children = children(snapshot, item)?;
    validation::require_count(
        item.id(),
        item_children.len(),
        1,
        "nested block child count",
    )?;
    let child = item_children[0];
    if snapshot.parent(child) != Some(HostParent::Node(item)) {
        return Err(unsupported(
            child.id(),
            SCENE_ID,
            "nested child ownership changed",
        ));
    }
    validation::validate_identity(snapshot, child, CHILD_ID)?;

    let node_ids = [root.id(), item.id(), child.id()];
    validation::validate_root_style(style_for(styles, root)?)?;
    validation::validate_item_style(style_for(styles, item)?)?;
    validation::validate_child_style(style_for(styles, child)?)?;
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
            SCENE_ID,
            "computed style node set changed",
        ));
    }
    Ok(Some(node_ids))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_inputs: &std::collections::BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 3],
) -> Result<RuntimeC11_4FitContentSpanningObservation, RuntimeLayoutFailure> {
    validation::validate_root_style(style_for(styles, root)?)?;
    let grid_input = grid_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_spanning_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validation::validate_grid_input(root.id(), grid_input)?;
    if grid_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentSpanningObservation {
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
