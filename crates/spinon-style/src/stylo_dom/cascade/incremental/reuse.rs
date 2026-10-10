use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use super::super::{ComputedElementStyle, ComputedStyleSnapshot, StyloDocumentView};

pub(super) struct CascadeReusePlan {
    pub(super) old_styles: BTreeMap<NodeId, ComputedElementStyle>,
    pub(super) dirty_nodes: BTreeSet<NodeId>,
    pub(super) context_nodes: BTreeSet<NodeId>,
}

pub(super) fn make_reuse_plan(
    view: &StyloDocumentView,
    previous: &ComputedStyleSnapshot,
    dirty_root_ids: &[NodeId],
) -> Option<CascadeReusePlan> {
    let mut current_handles = BTreeMap::new();
    let mut pending = vec![view.root_handle()];
    while let Some(handle) = pending.pop() {
        current_handles.insert(handle.id(), handle);
        if let Some(children) = view.snapshot().children(handle) {
            pending.extend(children);
        }
    }

    let mut old_styles = BTreeMap::new();
    for old in previous.elements.iter() {
        if old_styles.insert(old.node_id, old.clone()).is_some() {
            return None;
        }
    }
    let current_element_ids = current_handles
        .iter()
        .filter_map(|(id, handle)| view.element(*handle).map(|_| *id))
        .collect::<BTreeSet<_>>();
    if current_element_ids != old_styles.keys().copied().collect::<BTreeSet<_>>()
        || previous.elements.first().map(|style| style.node_id) != Some(view.root_handle().id())
    {
        return None;
    }

    let mut dirty_nodes = BTreeSet::new();
    for dirty_root_id in dirty_root_ids {
        let root = *current_handles.get(dirty_root_id)?;
        view.element(root)?;
        let mut subtree = vec![root];
        while let Some(handle) = subtree.pop() {
            if view.element(handle).is_some() {
                dirty_nodes.insert(handle.id());
            }
            if let Some(children) = view.snapshot().children(handle) {
                subtree.extend(children);
            }
        }
    }

    let mut context_nodes = BTreeSet::new();
    for dirty_root_id in dirty_root_ids {
        let mut handle = *current_handles.get(dirty_root_id)?;
        while handle != view.root_handle() {
            let parent = match view.snapshot().parent(handle)? {
                spinon_core::HostParent::Node(parent) => parent,
                spinon_core::HostParent::Root => break,
            };
            if view.element(parent).is_some() && !dirty_nodes.contains(&parent.id()) {
                context_nodes.insert(parent.id());
            }
            handle = parent;
        }
    }

    Some(CascadeReusePlan {
        old_styles,
        dirty_nodes,
        context_nodes,
    })
}
