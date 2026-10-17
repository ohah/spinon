use super::{
    CssRect, CssSize, RuntimeGlyphBitmap, RuntimePaint, RuntimePaintCommand, RuntimeRenderBox,
    RuntimeRenderError, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeRenderText,
    RuntimeSrgbColor,
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
    RuntimeRenderText::new(
        node,
        source,
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        super::CssClip::unconstrained(),
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
fn text_owner_inside_scroll_container_is_rejected_until_scroll_transform_is_supported() {
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
