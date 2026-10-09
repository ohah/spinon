use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use std::sync::Arc;

use super::{CssRect, CssSize, OpaqueCssSrgb, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey};

fn key() -> RuntimeRenderKey {
    let snapshot = HostDocument::new().unwrap().snapshot();
    RuntimeRenderKey::new(
        snapshot.generation(),
        snapshot.document_revision(),
        snapshot.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
}

#[test]
fn runtime_scene_keeps_complete_key_dom_order_and_transparent_nodes() {
    let first = NodeId::new(1).unwrap();
    let second = NodeId::new(2).unwrap();
    let expected_key = key();
    let snapshot = super::RuntimeRenderSnapshot::new(
        expected_key,
        CssSize::new(301.0, 100.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                first,
                CssRect::new(0.0, 0.0, 51.0, 31.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(51, 102, 255)),
                0,
            ),
            RuntimeRenderBox::new(
                second,
                CssRect::new(62.0, 0.0, 41.0, 31.0).unwrap(),
                RuntimePaint::None,
                1,
            ),
        ],
    )
    .unwrap();

    assert_eq!(snapshot.boxes()[0].node_id(), first);
    assert_eq!(snapshot.boxes()[1].node_id(), second);
    assert_eq!(snapshot.boxes()[1].paint(), RuntimePaint::None);
    assert_eq!(snapshot.boxes()[1].paint_order(), 1);
    assert_eq!(snapshot.viewport_css_px().width(), 301.0);
    assert_eq!(snapshot.key(), expected_key);
}

#[test]
fn runtime_scene_accepts_empty_and_rejects_duplicate_or_reordered_nodes() {
    let viewport = CssSize::new(10.0, 10.0).unwrap();
    assert!(super::RuntimeRenderSnapshot::new(key(), viewport, Vec::new()).is_ok());
    let node = NodeId::new(1).unwrap();
    let render_box = RuntimeRenderBox::new(
        node,
        CssRect::new(0.0, 0.0, 1.0, 1.0).unwrap(),
        RuntimePaint::None,
        0,
    );
    assert!(matches!(
        super::RuntimeRenderSnapshot::new(key(), viewport, vec![render_box, render_box]),
        Err(super::RuntimeRenderError::DuplicateNode(id)) if id == node
    ));
    assert!(matches!(
        super::RuntimeRenderSnapshot::new(
            key(),
            viewport,
            vec![RuntimeRenderBox::new(
                node,
                render_box.frame_css_px(),
                RuntimePaint::None,
                1
            )]
        ),
        Err(super::RuntimeRenderError::InvalidPaintOrder)
    ));
}

#[test]
fn document_revision_rebinding_preserves_shared_box_storage() {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: "http://www.w3.org/1999/xhtml".to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();
    let original_snapshot = document.snapshot();

    let detached = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: detached,
        namespace: "http://www.w3.org/1999/xhtml".to_owned(),
        local_name: "button".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: detached,
        name: AttributeName::new(None, "id").unwrap(),
        value: "detached".into(),
    });
    document.commit(batch).unwrap();
    let new_document_snapshot = document.snapshot();
    let original = super::RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            original_snapshot.generation(),
            original_snapshot.document_revision(),
            original_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(320.0, 640.0).unwrap(),
        vec![RuntimeRenderBox::new(
            root.id(),
            CssRect::new(0.0, 0.0, 320.0, 640.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap();

    let rebound = original
        .with_document_snapshot_revision(&new_document_snapshot)
        .unwrap();

    assert_eq!(
        rebound.key().document_revision(),
        new_document_snapshot.document_revision()
    );
    assert_eq!(rebound.key().generation(), original.key().generation());
    assert_eq!(
        rebound.key().render_tree_revision(),
        original.key().render_tree_revision()
    );
    assert!(Arc::ptr_eq(&original.boxes, &rebound.boxes));
}

#[test]
fn document_revision_rebinding_rejects_other_generations_and_connected_trees() {
    let mut document = HostDocument::new().unwrap();
    let original_snapshot = document.snapshot();
    let original = super::RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            original_snapshot.generation(),
            original_snapshot.document_revision(),
            original_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(10.0, 10.0).unwrap(),
        Vec::new(),
    )
    .unwrap();

    assert!(
        original
            .with_document_snapshot_revision(&HostDocument::new().unwrap().snapshot())
            .is_none()
    );

    let node = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(2).unwrap(), document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: "http://www.w3.org/1999/xhtml".to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: spinon_core::HostParent::Root,
        node,
        before: None,
    });
    document.commit(batch).unwrap();
    assert!(
        original
            .with_document_snapshot_revision(&document.snapshot())
            .is_none()
    );
}
