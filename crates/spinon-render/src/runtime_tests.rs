use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use std::sync::Arc;

use super::{
    CssAxisClip, CssClip, CssRect, CssSize, OpaqueCssSrgb, RuntimePaint, RuntimeRenderBox,
    RuntimeRenderKey,
};

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
fn runtime_render_key_distinguishes_intrinsic_size_revisions() {
    let initial = key();
    let next_revision = spinon_core::IntrinsicSizeRevision::INITIAL
        .checked_next()
        .unwrap();
    let next = initial.with_intrinsic_size_revision(next_revision);
    assert_ne!(initial, next);
    assert_eq!(next.intrinsic_size_revision(), next_revision);
}

#[test]
fn runtime_render_key_distinguishes_scroll_state_revision_independently() {
    let initial = key();
    let next_revision = spinon_core::ScrollStateRevision::INITIAL
        .checked_next()
        .unwrap();
    let next = initial.with_scroll_state_revision(next_revision);
    assert_ne!(initial, next);
    assert_eq!(next.document_revision(), initial.document_revision());
    assert_eq!(next.style_revision(), initial.style_revision());
    assert_eq!(next.environment_revision(), initial.environment_revision());
    assert_eq!(next.scroll_state_revision(), next_revision);
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
    assert_eq!(snapshot.boxes()[1].hit_test_order(), 1);
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

    let second = NodeId::new(2).unwrap();
    assert!(matches!(
        super::RuntimeRenderSnapshot::new(
            key(),
            viewport,
            vec![
                RuntimeRenderBox::new(node, render_box.frame_css_px(), RuntimePaint::None, 0,),
                RuntimeRenderBox::new(second, render_box.frame_css_px(), RuntimePaint::None, 1,)
                    .with_hit_test_order(0),
            ]
        ),
        Err(super::RuntimeRenderError::InvalidHitTestOrder)
    ));
    assert!(matches!(
        super::RuntimeRenderSnapshot::new(
            key(),
            viewport,
            vec![
                RuntimeRenderBox::new(node, render_box.frame_css_px(), RuntimePaint::None, 0,),
                RuntimeRenderBox::new(second, render_box.frame_css_px(), RuntimePaint::None, 1,)
                    .with_hit_test_order(2),
            ]
        ),
        Err(super::RuntimeRenderError::InvalidHitTestOrder)
    ));
}

#[test]
fn runtime_scroll_container_validates_positive_range_and_keeps_offsets_in_range() {
    let owner = NodeId::new(62).unwrap();
    let scrollport = CssRect::new(12.0, 18.0, 114.0, 76.0).unwrap();
    let state =
        super::RuntimeScrollContainer::new(owner, scrollport, 178.0, 76.0, 64.0, 0.0).unwrap();
    assert_eq!(state.owner(), owner);
    assert_eq!(state.scrollport_css_px(), scrollport);
    assert_eq!(state.max_offset_x_css_px(), 64.0);
    assert_eq!(state.max_offset_y_css_px(), 0.0);
    assert_eq!(state.offset_x_css_px(), 64.0);

    for (scrollable_width, offset_x, offset_y) in [
        (f32::NAN, 0.0, 0.0),
        (113.0, 0.0, 0.0),
        (178.0, -0.01, 0.0),
        (178.0, 64.01, 0.0),
        (178.0, 0.0, f32::INFINITY),
    ] {
        assert!(matches!(
            super::RuntimeScrollContainer::new(
                owner,
                scrollport,
                scrollable_width,
                76.0,
                offset_x,
                offset_y
            ),
            Err(super::RuntimeRenderError::InvalidScrollState)
        ));
    }
}

#[test]
fn runtime_snapshot_rejects_duplicate_scroll_owners() {
    let owner = NodeId::new(62).unwrap();
    let container = super::RuntimeScrollContainer::new(
        owner,
        CssRect::new(0.0, 0.0, 10.0, 10.0).unwrap(),
        10.0,
        10.0,
        0.0,
        0.0,
    )
    .unwrap();
    assert!(matches!(
        super::RuntimeRenderSnapshot::new_with_scroll_containers(
            key(),
            CssSize::new(20.0, 20.0).unwrap(),
            Vec::new(),
            vec![container, container]
        ),
        Err(super::RuntimeRenderError::DuplicateScrollOwner(id)) if id == owner
    ));
}

#[test]
fn runtime_scroll_offset_rebuilds_frames_and_keeps_the_owner_clip_stationary() {
    let owner = NodeId::new(62).unwrap();
    let target = NodeId::new(63).unwrap();
    let scrollport = CssRect::new(0.0, 0.0, 100.0, 100.0).unwrap();
    let container =
        super::RuntimeScrollContainer::new(owner, scrollport, 200.0, 100.0, 0.0, 0.0).unwrap();
    let paint_clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 100.0).unwrap());
    let metadata = super::RuntimeScrollRenderMetadata::new(
        CssRect::new(120.0, 10.0, 20.0, 20.0).unwrap(),
        vec![owner],
        vec![super::RuntimeScrollClipLayer::new(
            paint_clip,
            paint_clip,
            Vec::new(),
        )],
    );
    let initial = super::RuntimeRenderSnapshot::new_with_scroll_metadata(
        key(),
        CssSize::new(200.0, 80.0).unwrap(),
        vec![RuntimeRenderBox::new(
            target,
            metadata.base_frame_css_px(),
            RuntimePaint::None,
            0,
        )],
        vec![container],
        vec![metadata],
    )
    .unwrap();
    let moved = initial
        .with_scroll_offsets(initial.key().scroll_state_revision(), &[(owner, 40.0, 0.0)])
        .unwrap();
    let render_box = &moved.boxes()[0];

    assert_eq!(render_box.frame_css_px().x(), 80.0);
    assert_eq!(
        render_box
            .clip_css_px()
            .x()
            .map(|clip| (clip.min(), clip.max())),
        Some((0.0, 100.0))
    );
    assert_eq!(moved.key().scroll_state_revision().get(), 1);
    assert_eq!(
        moved
            .hit_test_css_point(85.0, 15.0)
            .map(|hit| hit.node_id()),
        Some(target)
    );
    assert!(matches!(
        moved.with_scroll_offsets(initial.key().scroll_state_revision(), &[(owner, 0.0, 0.0)]),
        Err(super::RuntimeRenderError::StaleScrollRevision)
    ));
}

#[test]
fn runtime_nested_scroll_offsets_move_inner_clip_with_outer_scroll_only() {
    let outer = NodeId::new(62).unwrap();
    let inner = NodeId::new(63).unwrap();
    let target = NodeId::new(64).unwrap();
    let outer_state = super::RuntimeScrollContainer::new(
        outer,
        CssRect::new(0.0, 0.0, 100.0, 100.0).unwrap(),
        200.0,
        100.0,
        10.0,
        0.0,
    )
    .unwrap();
    let inner_state = super::RuntimeScrollContainer::new(
        inner,
        CssRect::new(20.0, 0.0, 60.0, 60.0).unwrap(),
        100.0,
        60.0,
        20.0,
        0.0,
    )
    .unwrap()
    .with_parent(Some(outer));
    let outer_clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 100.0).unwrap());
    let inner_clip = CssClip::unconstrained().with_x(CssAxisClip::new(20.0, 80.0).unwrap());
    let metadata = super::RuntimeScrollRenderMetadata::new(
        CssRect::new(80.0, 10.0, 20.0, 20.0).unwrap(),
        vec![outer, inner],
        vec![
            super::RuntimeScrollClipLayer::new(outer_clip, outer_clip, Vec::new()),
            super::RuntimeScrollClipLayer::new(inner_clip, inner_clip, vec![outer]),
        ],
    );
    let snapshot = super::RuntimeRenderSnapshot::new_with_scroll_metadata(
        key(),
        CssSize::new(200.0, 80.0).unwrap(),
        vec![RuntimeRenderBox::new(
            target,
            metadata.base_frame_css_px(),
            RuntimePaint::None,
            0,
        )],
        vec![outer_state, inner_state],
        vec![metadata],
    )
    .unwrap();
    let render_box = &snapshot.boxes()[0];

    assert_eq!(render_box.frame_css_px().x(), 50.0);
    assert_eq!(
        render_box
            .clip_css_px()
            .x()
            .map(|clip| (clip.min(), clip.max())),
        Some((10.0, 70.0))
    );
}

#[test]
fn runtime_hit_test_applies_axis_clips_and_uses_half_open_edges() {
    let node = NodeId::new(40).unwrap();
    let snapshot = super::RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(100.0, 100.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                node,
                CssRect::new(10.0, 10.0, 50.0, 50.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(20, 40, 60)),
                0,
            )
            .with_clip(
                CssClip::unconstrained()
                    .with_x(CssAxisClip::new(20.0, 40.0).unwrap())
                    .with_y(CssAxisClip::new(25.0, 45.0).unwrap()),
            ),
        ],
    )
    .unwrap();

    assert_eq!(
        snapshot.hit_test_css_point(20.0, 25.0).unwrap().node_id(),
        node
    );
    assert!(snapshot.hit_test_css_point(40.0, 30.0).is_none());
    assert!(snapshot.hit_test_css_point(30.0, 45.0).is_none());
    assert!(snapshot.hit_test_css_point(19.0, 30.0).is_none());
    assert!(snapshot.hit_test_css_point(-1.0, 30.0).is_none());
}

#[test]
fn runtime_hit_test_can_order_targets_independently_from_paint() {
    let target = NodeId::new(41).unwrap();
    let painted_parent = NodeId::new(42).unwrap();
    let frame = CssRect::new(10.0, 10.0, 40.0, 30.0).unwrap();
    let snapshot = super::RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(100.0, 80.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                target,
                frame,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(20, 40, 60)),
                0,
            )
            .with_hit_test_order(1),
            RuntimeRenderBox::new(
                painted_parent,
                frame,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(60, 40, 20)),
                1,
            )
            .with_hit_test_order(0),
        ],
    )
    .unwrap();

    assert_eq!(snapshot.boxes()[0].paint_order(), 0);
    assert_eq!(snapshot.boxes()[1].paint_order(), 1);
    assert_eq!(
        snapshot.hit_test_css_point(20.0, 20.0).unwrap().node_id(),
        target
    );
}

#[test]
fn runtime_hit_test_uses_its_clip_independently_from_the_paint_clip() {
    let node = NodeId::new(41).unwrap();
    let render_box = RuntimeRenderBox::new(
        node,
        CssRect::new(10.0, 10.0, 50.0, 50.0).unwrap(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(20, 40, 60)),
        0,
    )
    .with_clip(CssClip::unconstrained().with_x(CssAxisClip::new(20.0, 40.0).unwrap()))
    .with_hit_test_clip(CssClip::unconstrained().with_x(CssAxisClip::new(10.0, 50.0).unwrap()));
    let snapshot = super::RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(100.0, 100.0).unwrap(),
        vec![render_box],
    )
    .unwrap();

    assert!(!render_box.clip_css_px().contains(15.0, 30.0));
    assert!(render_box.hit_test_clip_css_px().contains(15.0, 30.0));
    assert_eq!(
        snapshot.hit_test_css_point(15.0, 30.0).unwrap().node_id(),
        node
    );
    assert!(snapshot.hit_test_css_point(50.0, 30.0).is_none());
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
    let intrinsic_revision = spinon_core::IntrinsicSizeRevision::INITIAL
        .checked_next()
        .unwrap();
    let original = super::RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            original_snapshot.generation(),
            original_snapshot.document_revision(),
            original_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
        .with_intrinsic_size_revision(intrinsic_revision),
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
    assert_eq!(rebound.key().intrinsic_size_revision(), intrinsic_revision);
    assert_eq!(
        rebound.key().scroll_state_revision(),
        original.key().scroll_state_revision()
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
