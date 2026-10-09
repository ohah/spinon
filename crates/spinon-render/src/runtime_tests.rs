use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};

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
