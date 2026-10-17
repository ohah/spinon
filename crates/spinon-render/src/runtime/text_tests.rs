use super::{
    CssAxisClip, CssClip, CssRect, CssSize, RuntimeGlyphBitmap, RuntimePaint, RuntimePaintCommand,
    RuntimeRenderBox, RuntimeRenderError, RuntimeRenderKey, RuntimeRenderSnapshot,
    RuntimeRenderText, RuntimeSrgbColor,
};
use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
use std::sync::Arc;

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

fn bitmap() -> RuntimeGlyphBitmap {
    RuntimeGlyphBitmap::new(2, 2, 0, 0, 0, 0, Arc::from([0_u8, 64, 128, 255])).unwrap()
}

fn text(node: NodeId, source: RuntimeRenderKey) -> RuntimeRenderText {
    text_with_clip(node, source, CssClip::unconstrained())
}

fn text_with_clip(
    node: NodeId,
    source: RuntimeRenderKey,
    clip_css_px: CssClip,
) -> RuntimeRenderText {
    RuntimeRenderText::new(
        node,
        source,
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        clip_css_px,
        vec![bitmap()],
    )
    .unwrap()
}

fn scene(node: NodeId) -> RuntimeRenderSnapshot {
    RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(8.0, 8.0).unwrap(),
        vec![RuntimeRenderBox::new(
            node,
            CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap()
}

fn commands_for_one_text() -> Vec<RuntimePaintCommand> {
    vec![
        RuntimePaintCommand::DrawBox { index: 0 },
        RuntimePaintCommand::DrawText { index: 0 },
    ]
}

#[test]
fn text_snapshot_checks_bitmap_geometry_source_owner_and_unique_node() {
    assert_eq!(
        RuntimeGlyphBitmap::new(2, 2, 0, 0, 0, 0, Arc::from([1_u8, 2, 3])),
        Err(RuntimeRenderError::InvalidTextBitmap)
    );
    assert_eq!(
        RuntimeSrgbColor::new(0.0, f32::NAN, 0.0, 1.0),
        Err(RuntimeRenderError::InvalidPaintColor)
    );
    assert_eq!(
        RuntimeRenderText::new(
            NodeId::new(1).unwrap(),
            key(),
            f32::INFINITY,
            0.0,
            1.0,
            RuntimeSrgbColor::new(0.0, 0.0, 0.0, 1.0).unwrap(),
            super::CssClip::unconstrained(),
            vec![bitmap()],
        ),
        Err(RuntimeRenderError::InvalidTextGeometry)
    );

    let node = NodeId::new(1).unwrap();
    let scene = scene(node);
    let stale_source = key().with_intrinsic_size_revision(
        spinon_core::IntrinsicSizeRevision::INITIAL
            .checked_next()
            .unwrap(),
    );
    assert_eq!(
        scene.with_texts_and_paint_commands(
            vec![text(node, stale_source)],
            commands_for_one_text(),
        ),
        Err(RuntimeRenderError::InvalidTextSource(node))
    );
    assert_eq!(
        scene.with_texts_and_paint_commands(
            vec![text(NodeId::new(2).unwrap(), scene.key())],
            commands_for_one_text(),
        ),
        Err(RuntimeRenderError::TextOwnerMissing(
            NodeId::new(2).unwrap()
        ))
    );
    assert_eq!(
        scene.with_texts_and_paint_commands(
            vec![text(node, scene.key()), text(node, scene.key())],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
                RuntimePaintCommand::DrawText { index: 1 },
            ],
        ),
        Err(RuntimeRenderError::DuplicateText(node))
    );
}

#[test]
fn text_paint_command_indices_must_be_complete_and_in_order() {
    let node = NodeId::new(1).unwrap();
    let scene = scene(node);
    assert_eq!(
        scene.with_texts_and_paint_commands(
            vec![text(node, scene.key())],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 1 },
            ],
        ),
        Err(RuntimeRenderError::InvalidPaintCommands)
    );
    assert_eq!(
        scene.with_texts_and_paint_commands(
            vec![text(node, scene.key())],
            vec![RuntimePaintCommand::DrawBox { index: 0 }],
        ),
        Err(RuntimeRenderError::InvalidPaintCommands)
    );
}

#[test]
fn text_clip_intersects_the_owner_boxes_effective_paint_clip() {
    let owner = NodeId::new(1).unwrap();
    let owner_clip = CssClip::unconstrained()
        .with_x(CssAxisClip::new(3.0, 8.0).unwrap())
        .with_y(CssAxisClip::new(2.0, 9.0).unwrap());
    let text_clip = CssClip::unconstrained()
        .with_x(CssAxisClip::new(5.0, 10.0).unwrap())
        .with_y(CssAxisClip::new(1.0, 6.0).unwrap());
    let base = RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(16.0, 16.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                owner,
                CssRect::new(0.0, 0.0, 16.0, 16.0).unwrap(),
                RuntimePaint::None,
                0,
            )
            .with_clip(owner_clip),
        ],
    )
    .unwrap();
    let scene = base
        .with_texts_and_paint_commands(
            vec![text_with_clip(owner, base.key(), text_clip)],
            commands_for_one_text(),
        )
        .unwrap();

    let effective_clip = scene.texts()[0].clip_css_px();
    assert_eq!(
        effective_clip.x(),
        Some(CssAxisClip::new(5.0, 8.0).unwrap())
    );
    assert_eq!(
        effective_clip.y(),
        Some(CssAxisClip::new(2.0, 6.0).unwrap())
    );
}

#[test]
fn text_can_overflow_a_visible_owner_when_neither_clip_is_constrained() {
    let owner = NodeId::new(1).unwrap();
    let base = scene(owner);
    let scene = base
        .with_texts_and_paint_commands(vec![text(owner, base.key())], commands_for_one_text())
        .unwrap();

    assert_eq!(scene.texts()[0].clip_css_px(), CssClip::unconstrained());
}

#[test]
fn text_owner_that_is_itself_a_scroll_container_is_rejected() {
    let owner = NodeId::new(1).unwrap();
    let scene = scene(owner);
    let scroll = super::RuntimeScrollContainer::new(
        owner,
        CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap(),
        8.0,
        16.0,
        0.0,
        0.0,
    )
    .unwrap();
    let metadata = super::RuntimeScrollRenderMetadata::new(
        CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap(),
        Vec::new(),
        vec![super::RuntimeScrollClipLayer::new(
            super::CssClip::unconstrained(),
            super::CssClip::unconstrained(),
            Vec::new(),
        )],
    );
    let owner_snapshot =
        super::RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
            scene.key(),
            scene.viewport_css_px(),
            scene.boxes().to_vec(),
            vec![scroll],
            vec![metadata],
            vec![text(owner, scene.key())],
            commands_for_one_text(),
        );
    assert_eq!(
        owner_snapshot,
        Err(RuntimeRenderError::UnsupportedTextScroll(owner))
    );
}

#[test]
fn text_inside_an_unscrolled_overflow_owner_keeps_its_paint_clip() {
    let parent = NodeId::new(1).unwrap();
    let text_owner = NodeId::new(2).unwrap();
    let key = key();
    let parent_frame = CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 16.0, 8.0).unwrap();
    let parent_clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 8.0).unwrap());
    let boxes = vec![
        RuntimeRenderBox::new(parent, parent_frame, RuntimePaint::None, 0).with_clip(parent_clip),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 1)
            .with_clip(parent_clip),
    ];
    let scroll =
        super::RuntimeScrollContainer::new(parent, parent_frame, 16.0, 8.0, 0.0, 0.0).unwrap();
    let clip_layer = || super::RuntimeScrollClipLayer::new(parent_clip, parent_clip, Vec::new());
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(parent_frame, Vec::new(), vec![clip_layer()]),
        super::RuntimeScrollRenderMetadata::new(owner_frame, vec![parent], vec![clip_layer()]),
    ];
    let scene = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(16.0, 8.0).unwrap(),
        boxes,
        vec![scroll],
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    )
    .unwrap();

    assert_eq!(scene.texts()[0].clip_css_px().x(), parent_clip.x());
}

#[test]
fn text_rejects_scroll_ancestor_metadata_without_a_matching_container() {
    let parent = NodeId::new(1).unwrap();
    let text_owner = NodeId::new(2).unwrap();
    let key = key();
    let parent_frame = CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 16.0, 8.0).unwrap();
    let clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 8.0).unwrap());
    let boxes = vec![
        RuntimeRenderBox::new(parent, parent_frame, RuntimePaint::None, 0).with_clip(clip),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 1).with_clip(clip),
    ];
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(parent_frame, Vec::new(), Vec::new()),
        super::RuntimeScrollRenderMetadata::new(owner_frame, vec![parent], Vec::new()),
    ];
    let result = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(16.0, 8.0).unwrap(),
        boxes,
        Vec::new(),
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    );

    assert!(matches!(
        result,
        Err(RuntimeRenderError::InvalidScrollState)
    ));
}

#[test]
fn snapshot_rejects_duplicate_scroll_ancestor_metadata_instead_of_double_moving() {
    let parent = NodeId::new(1).unwrap();
    let text_owner = NodeId::new(2).unwrap();
    let key = key();
    let parent_frame = CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 16.0, 8.0).unwrap();
    let boxes = vec![
        RuntimeRenderBox::new(parent, parent_frame, RuntimePaint::None, 0),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 1),
    ];
    let scroll =
        super::RuntimeScrollContainer::new(parent, parent_frame, 16.0, 8.0, 3.0, 0.0).unwrap();
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(parent_frame, Vec::new(), Vec::new()),
        super::RuntimeScrollRenderMetadata::new(owner_frame, vec![parent, parent], Vec::new()),
    ];
    let result = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(16.0, 8.0).unwrap(),
        boxes,
        vec![scroll],
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    );

    assert!(matches!(
        result,
        Err(RuntimeRenderError::InvalidScrollState)
    ));
}

#[test]
fn text_inside_a_scrolled_horizontal_ancestor_moves_and_resets_from_base_origin() {
    let parent = NodeId::new(1).unwrap();
    let text_owner = NodeId::new(2).unwrap();
    let key = key();
    let parent_frame = CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 16.0, 8.0).unwrap();
    let clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 8.0).unwrap());
    let boxes = vec![
        RuntimeRenderBox::new(parent, parent_frame, RuntimePaint::None, 0).with_clip(clip),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 1).with_clip(clip),
    ];
    let scroll =
        super::RuntimeScrollContainer::new(parent, parent_frame, 16.0, 8.0, 3.0, 0.0).unwrap();
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(
            parent_frame,
            Vec::new(),
            vec![super::RuntimeScrollClipLayer::new(clip, clip, Vec::new())],
        ),
        super::RuntimeScrollRenderMetadata::new(
            owner_frame,
            vec![parent],
            vec![super::RuntimeScrollClipLayer::new(clip, clip, Vec::new())],
        ),
    ];
    let result = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(16.0, 8.0).unwrap(),
        boxes,
        vec![scroll],
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    );

    let scene = result.unwrap();
    assert_eq!(scene.texts()[0].origin_x_css_px(), -3.0);
    assert_eq!(scene.texts()[0].origin_y_css_px(), 0.0);
    assert_eq!(scene.texts()[0].clip_css_px().x(), clip.x());

    let moved = scene
        .with_scroll_offsets(scene.key().scroll_state_revision(), &[(parent, 6.0, 0.0)])
        .unwrap();
    assert_eq!(moved.texts()[0].origin_x_css_px(), -6.0);
    assert_eq!(moved.texts()[0].clip_css_px().x(), clip.x());

    let reset = moved
        .with_scroll_offsets(moved.key().scroll_state_revision(), &[(parent, 0.0, 0.0)])
        .unwrap();
    assert_eq!(reset.texts()[0].origin_x_css_px(), 0.0);
    assert_eq!(reset.texts()[0].clip_css_px().x(), clip.x());
}

#[test]
fn text_inside_a_scrolled_vertical_ancestor_moves_and_resets_from_base_origin() {
    let parent = NodeId::new(1).unwrap();
    let text_owner = NodeId::new(2).unwrap();
    let key = key();
    let parent_frame = CssRect::new(0.0, 0.0, 8.0, 8.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 16.0, 8.0).unwrap();
    let clip = CssClip::unconstrained().with_y(CssAxisClip::new(0.0, 8.0).unwrap());
    let boxes = vec![
        RuntimeRenderBox::new(parent, parent_frame, RuntimePaint::None, 0).with_clip(clip),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 1).with_clip(clip),
    ];
    let scroll =
        super::RuntimeScrollContainer::new(parent, parent_frame, 16.0, 40.0, 0.0, 18.0).unwrap();
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(
            parent_frame,
            Vec::new(),
            vec![super::RuntimeScrollClipLayer::new(clip, clip, Vec::new())],
        ),
        super::RuntimeScrollRenderMetadata::new(
            owner_frame,
            vec![parent],
            vec![super::RuntimeScrollClipLayer::new(clip, clip, Vec::new())],
        ),
    ];
    let result = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(16.0, 8.0).unwrap(),
        boxes,
        vec![scroll],
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    );

    let scene = result.unwrap();
    assert_eq!(scene.texts()[0].origin_x_css_px(), 0.0);
    assert_eq!(scene.texts()[0].origin_y_css_px(), -18.0);
    assert_eq!(scene.texts()[0].clip_css_px().y(), clip.y());

    let moved = scene
        .with_scroll_offsets(scene.key().scroll_state_revision(), &[(parent, 0.0, 7.0)])
        .unwrap();
    assert_eq!(moved.texts()[0].origin_y_css_px(), -7.0);
    assert_eq!(moved.texts()[0].clip_css_px().y(), clip.y());

    let reset = moved
        .with_scroll_offsets(moved.key().scroll_state_revision(), &[(parent, 0.0, 0.0)])
        .unwrap();
    assert_eq!(reset.texts()[0].origin_y_css_px(), 0.0);
    assert_eq!(reset.texts()[0].clip_css_px().y(), clip.y());
}

#[test]
fn nested_scroll_offsets_move_text_but_keep_each_scrollport_clip_attached_to_its_owner() {
    let outer = NodeId::new(1).unwrap();
    let inner = NodeId::new(2).unwrap();
    let text_owner = NodeId::new(3).unwrap();
    let key = key();
    let outer_frame = CssRect::new(0.0, 0.0, 20.0, 20.0).unwrap();
    let inner_frame = CssRect::new(0.0, 0.0, 16.0, 16.0).unwrap();
    let owner_frame = CssRect::new(0.0, 0.0, 30.0, 8.0).unwrap();
    let outer_clip = CssClip::unconstrained()
        .with_x(CssAxisClip::new(0.0, 20.0).unwrap())
        .with_y(CssAxisClip::new(0.0, 20.0).unwrap());
    let inner_clip = CssClip::unconstrained()
        .with_x(CssAxisClip::new(0.0, 16.0).unwrap())
        .with_y(CssAxisClip::new(0.0, 16.0).unwrap());
    let boxes = vec![
        RuntimeRenderBox::new(outer, outer_frame, RuntimePaint::None, 0),
        RuntimeRenderBox::new(inner, inner_frame, RuntimePaint::None, 1),
        RuntimeRenderBox::new(text_owner, owner_frame, RuntimePaint::None, 2),
    ];
    let outer_layer = || super::RuntimeScrollClipLayer::new(outer_clip, outer_clip, Vec::new());
    let inner_layer = || super::RuntimeScrollClipLayer::new(inner_clip, inner_clip, vec![outer]);
    let metadata = vec![
        super::RuntimeScrollRenderMetadata::new(outer_frame, Vec::new(), vec![outer_layer()]),
        super::RuntimeScrollRenderMetadata::new(
            inner_frame,
            vec![outer],
            vec![outer_layer(), inner_layer()],
        ),
        super::RuntimeScrollRenderMetadata::new(
            owner_frame,
            vec![inner, outer],
            vec![outer_layer(), inner_layer()],
        ),
    ];
    let scroll_containers = vec![
        super::RuntimeScrollContainer::new(outer, outer_frame, 25.0, 20.0, 4.0, 0.0).unwrap(),
        super::RuntimeScrollContainer::new(inner, inner_frame, 21.0, 16.0, 5.0, 0.0)
            .unwrap()
            .with_parent(Some(outer)),
    ];
    let scene = RuntimeRenderSnapshot::new_with_scroll_metadata_texts_and_paint_commands(
        key,
        CssSize::new(32.0, 24.0).unwrap(),
        boxes,
        scroll_containers,
        metadata,
        vec![text(text_owner, key)],
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::DrawBox { index: 2 },
            RuntimePaintCommand::DrawText { index: 0 },
        ],
    )
    .unwrap();

    assert_eq!(scene.texts()[0].origin_x_css_px(), -9.0);
    assert_eq!(
        scene.texts()[0].clip_css_px().x(),
        Some(CssAxisClip::new(0.0, 12.0).unwrap())
    );

    let moved = scene
        .with_scroll_offsets(
            scene.key().scroll_state_revision(),
            &[(outer, 2.0, 0.0), (inner, 3.0, 0.0)],
        )
        .unwrap();
    assert_eq!(moved.texts()[0].origin_x_css_px(), -5.0);
    assert_eq!(
        moved.texts()[0].clip_css_px().x(),
        Some(CssAxisClip::new(0.0, 14.0).unwrap())
    );

    let reset = moved
        .with_scroll_offsets(
            moved.key().scroll_state_revision(),
            &[(outer, 0.0, 0.0), (inner, 0.0, 0.0)],
        )
        .unwrap();
    assert_eq!(reset.texts()[0].origin_x_css_px(), 0.0);
    assert_eq!(
        reset.texts()[0].clip_css_px().x(),
        Some(CssAxisClip::new(0.0, 16.0).unwrap())
    );
}
