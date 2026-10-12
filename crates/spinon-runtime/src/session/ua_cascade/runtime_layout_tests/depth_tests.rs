use std::sync::Arc;

use super::super::{environment, wait_for_terminal_state};
use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::MAX_LAYOUT_TREE_DEPTH;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const STACKING_STYLE: &str =
    "position:relative;z-index:0;width:1px;height:1px;background-color:#336699";

#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "1024-depth runtime-worker stack validation runs in the optimized profile"
)]
fn configured_runtime_worker_lays_out_the_maximum_supported_runtime_mount_depth() {
    let (snapshot, handles) = deep_snapshot(MAX_LAYOUT_TREE_DEPTH);
    let coordinator = RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_stacking().unwrap();
    let handle = coordinator.handle();
    handle.register_document_snapshot(snapshot).unwrap();
    environment(&handle, CssMediaEnvironment::MOBILE);

    let cascade = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Ready);
    let layout = wait_for_layout_state(&handle, RuntimeLayoutState::Ready);
    assert_eq!(cascade.completed.as_ref().unwrap().roots.len(), 1);
    assert_eq!(
        cascade.completed.as_ref().unwrap().roots[0]
            .styles
            .elements
            .len(),
        MAX_LAYOUT_TREE_DEPTH
    );
    let completed = layout.completed.as_ref().unwrap();
    assert_eq!(completed.frames.len(), MAX_LAYOUT_TREE_DEPTH);
    let deepest = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == handles[MAX_LAYOUT_TREE_DEPTH - 1].id().get())
        .unwrap();
    assert_eq!(
        (deepest.x, deepest.y, deepest.width, deepest.height),
        (0.0, 0.0, 1.0, 1.0)
    );
    let render = completed.render_snapshot.as_ref().unwrap();
    assert_eq!(render.boxes().len(), MAX_LAYOUT_TREE_DEPTH);
    assert_eq!(
        render.boxes().last().unwrap().node_id(),
        handles[MAX_LAYOUT_TREE_DEPTH - 1].id()
    );

    coordinator.shutdown().unwrap();
}

#[test]
fn over_limit_runtime_tree_fails_before_cascade_and_worker_recovers_on_shallow_revision() {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(93).unwrap();
    let shallow_handles = append_chain(&mut document, owner, HostParent::Root, 2);
    let coordinator = RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_stacking().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    environment(&handle, CssMediaEnvironment::MOBILE);
    wait_for_terminal_state(&handle, RuntimeUaCascadeState::Ready);
    wait_for_layout_state(&handle, RuntimeLayoutState::Ready);

    let overflow_handles = append_chain(
        &mut document,
        owner,
        HostParent::Node(shallow_handles[1]),
        MAX_LAYOUT_TREE_DEPTH - 1,
    );
    let mut handles = shallow_handles.clone();
    handles.extend(overflow_handles.iter().copied());
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let failed_cascade = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Failed);
    let failed_layout = wait_for_layout_state(&handle, RuntimeLayoutState::Failed);
    assert!(failed_cascade.completed.is_none());
    assert!(
        failed_cascade
            .error
            .as_deref()
            .is_some_and(|error| { error.contains("1025") && error.contains("1024") })
    );
    assert_eq!(failed_layout.completed, None);
    assert_eq!(
        failed_layout.error,
        Some(RuntimeLayoutFailure {
            code: "tree_depth_exceeded",
            node_id: Some(handles[MAX_LAYOUT_TREE_DEPTH].id().get()),
            property: None,
        })
    );

    let mut recovery = DocumentChangeBatch::new(owner, document.document_revision());
    recovery.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(shallow_handles[1]),
        node: overflow_handles[0],
    });
    document.commit(recovery).unwrap();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let recovered_cascade = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Ready);
    let recovered_layout = wait_for_layout_state(&handle, RuntimeLayoutState::Ready);
    assert_eq!(recovered_cascade.completed.as_ref().unwrap().roots.len(), 1);
    assert_eq!(
        recovered_cascade
            .completed
            .unwrap()
            .cascade_recomputed_style_elements,
        2
    );
    assert_eq!(recovered_layout.completed.as_ref().unwrap().frames.len(), 2);

    coordinator.shutdown().unwrap();
}

fn deep_snapshot(depth: usize) -> (Arc<spinon_core::HostDocumentSnapshot>, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let handles = append_chain(
        &mut document,
        OwnerId::new(93).unwrap(),
        HostParent::Root,
        depth,
    );
    (Arc::new(document.snapshot()), handles)
}

fn append_chain(
    document: &mut HostDocument,
    owner: OwnerId,
    parent: HostParent,
    count: usize,
) -> Vec<HostNodeHandle> {
    let handles = (0..count)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());

    for (index, handle) in handles.iter().copied().enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: STACKING_STYLE.to_owned().into(),
        });
        let current_parent = index.checked_sub(1).map_or(parent, |parent_index| {
            HostParent::Node(handles[parent_index])
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: current_parent,
            node: handle,
            before: None,
        });
    }

    document.commit(batch).unwrap();
    handles
}
