use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};

use crate::{LayoutDisplay, LayoutError, LayoutStyle};

pub(super) fn collect_host_handles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<BTreeMap<NodeId, HostNodeHandle>, LayoutError> {
    let mut handles = BTreeMap::new();
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        if handles.insert(handle.id(), handle).is_some() {
            return Err(LayoutError::DuplicateNode(handle.id()));
        }
        let node = snapshot
            .node(handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        } else if !matches!(node.kind(), HostNodeKind::Text(_)) {
            return Err(LayoutError::InvalidHostDocumentRoot(handle.id()));
        }
    }
    Ok(handles)
}

pub(super) fn collect_hidden_layout_nodes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
    excluded: &mut BTreeSet<NodeId>,
) -> Result<(), LayoutError> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
        if matches!(node.kind(), HostNodeKind::Element(_)) {
            if !styles.contains_key(&handle.id()) {
                return Err(LayoutError::MissingStyle(handle.id()));
            }
            excluded.insert(handle.id());
            pending.extend(
                snapshot
                    .children(handle)
                    .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?
                    .filter(|child| {
                        snapshot
                            .node(*child)
                            .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(_)))
                    }),
            );
        }
    }
    Ok(())
}

pub(super) fn is_hidden_by_ancestor(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
) -> Result<bool, LayoutError> {
    let mut current = handle;
    loop {
        let style = styles
            .get(&current.id())
            .copied()
            .ok_or(LayoutError::MissingStyle(current.id()))?;
        if style.display == LayoutDisplay::None {
            return Ok(true);
        }
        if current == root {
            return Ok(false);
        }
        match snapshot.parent(current) {
            Some(HostParent::Node(parent)) => current = parent,
            Some(HostParent::Root) | None => return Ok(false),
        }
    }
}
