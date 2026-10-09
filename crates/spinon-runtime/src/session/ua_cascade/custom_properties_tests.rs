use super::*;

fn wait_for_document_revision(
    handle: &RuntimeUaCascadeHandle,
    revision: u64,
) -> (RuntimeUaCascadeSnapshot, RuntimeLayoutSnapshot) {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let cascade = handle.snapshot();
        let layout = handle.layout_snapshot();
        let cascade_matches = cascade
            .completed
            .as_ref()
            .is_some_and(|completed| completed.key.document_revision == revision);
        let layout_matches = layout
            .completed
            .as_ref()
            .is_some_and(|completed| completed.key.document_revision == revision);
        if cascade_matches && layout_matches {
            return (cascade, layout);
        }
        assert!(
            Instant::now() < deadline,
            "새 문서 revision 계산 시간 초과: cascade={cascade:?}, layout={layout:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

fn make_document() -> (
    HostDocument,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let source = document.reserve_node_handle().unwrap();
    let destination = document.reserve_node_handle().unwrap();
    let target = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(3052).unwrap(), document.document_revision());
    create_styled_element(&mut batch, root, "display:block;width:300px;height:200px");
    create_styled_element(&mut batch, source, "display:block;--size:41px");
    create_styled_element(&mut batch, destination, "display:block;--size:91px");
    create_styled_element(
        &mut batch,
        target,
        "display:block;width:var(--size);height:8px",
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: source,
        before: None,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(source),
        node: target,
        before: None,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: destination,
        before: None,
    });
    document.commit(batch).unwrap();
    (document, root, source, destination, target)
}

fn create_styled_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    inline_style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: inline_style.into(),
    });
}

fn completed_width(snapshot: &RuntimeUaCascadeSnapshot, target: HostNodeHandle) -> &str {
    snapshot.completed.as_ref().unwrap().roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.node_id == target.id())
        .unwrap()
        .properties["width"]
        .as_str()
}

fn completed_frame_width(snapshot: &RuntimeLayoutSnapshot, target: HostNodeHandle) -> f32 {
    snapshot
        .completed
        .as_ref()
        .unwrap()
        .frames
        .iter()
        .find(|frame| frame.node_id == target.id().get())
        .unwrap()
        .width
}

#[test]
fn runtime_recalculates_custom_property_inheritance_after_update_move_detach_and_reinsert() {
    let (mut document, root, source, destination, target) = make_document();
    let coordinator = RuntimeUaCascadeCoordinator::new_runtime_gpu().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    environment(&handle, CssMediaEnvironment::MOBILE);

    let initial_revision = document.document_revision().get();
    let (initial, initial_layout) = wait_for_document_revision(&handle, initial_revision);
    assert_eq!(completed_width(&initial, target), "41px");
    assert_eq!(completed_frame_width(&initial_layout, target), 41.0);
    assert_eq!(
        initial_layout
            .completed
            .as_ref()
            .unwrap()
            .key
            .document_revision,
        initial_revision
    );

    let mut update =
        DocumentChangeBatch::new(OwnerId::new(3052).unwrap(), document.document_revision());
    update.push(DocumentOperation::SetAttribute {
        node: source,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:block;--size:73px".into(),
    });
    document.commit(update).unwrap();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let updated_revision = document.document_revision().get();
    let (updated, updated_layout) = wait_for_document_revision(&handle, updated_revision);
    assert_eq!(completed_width(&updated, target), "73px");
    assert_eq!(completed_frame_width(&updated_layout, target), 73.0);

    let mut move_target =
        DocumentChangeBatch::new(OwnerId::new(3052).unwrap(), document.document_revision());
    move_target.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(destination),
        node: target,
        before: None,
    });
    document.commit(move_target).unwrap();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let moved_revision = document.document_revision().get();
    let (moved, moved_layout) = wait_for_document_revision(&handle, moved_revision);
    assert_eq!(completed_width(&moved, target), "91px");
    assert_eq!(completed_frame_width(&moved_layout, target), 91.0);

    let mut detach =
        DocumentChangeBatch::new(OwnerId::new(3052).unwrap(), document.document_revision());
    detach.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(destination),
        node: target,
    });
    document.commit(detach).unwrap();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let detached_revision = document.document_revision().get();
    let (detached, detached_layout) = wait_for_document_revision(&handle, detached_revision);
    assert!(
        detached.completed.as_ref().unwrap().roots[0]
            .styles
            .elements
            .iter()
            .all(|style| style.node_id != target.id())
    );
    assert!(
        detached_layout
            .completed
            .as_ref()
            .unwrap()
            .frames
            .iter()
            .all(|frame| frame.node_id != target.id().get())
    );

    let mut reinsert =
        DocumentChangeBatch::new(OwnerId::new(3052).unwrap(), document.document_revision());
    reinsert.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(source),
        node: target,
        before: None,
    });
    document.commit(reinsert).unwrap();
    handle
        .register_document_snapshot(Arc::new(document.snapshot()))
        .unwrap();
    let reinserted_revision = document.document_revision().get();
    let (reinserted, reinserted_layout) = wait_for_document_revision(&handle, reinserted_revision);
    assert_eq!(completed_width(&reinserted, target), "73px");
    assert_eq!(completed_frame_width(&reinserted_layout, target), 73.0);

    assert_eq!(
        reinserted.completed.as_ref().unwrap().roots[0].root_node_id,
        root.id().get()
    );
    coordinator.shutdown().unwrap();
}
