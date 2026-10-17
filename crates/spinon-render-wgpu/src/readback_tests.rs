use super::readback_runtime_scene_samples;
use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
use spinon_render::{
    CssClip, CssRect, CssSize, RuntimeGlyphBitmap, RuntimePaint, RuntimePaintCommand,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
};
use std::sync::Arc;

#[test]
fn wgpu_readback_draws_coverage_masks_and_respects_css_clip_edge() {
    let document = HostDocument::new().unwrap().snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let node = NodeId::new(1).unwrap();
    let owner_clip =
        CssClip::unconstrained().with_x(spinon_render::CssAxisClip::new(0.0, 2.0).unwrap());
    let base = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(4.0, 4.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                node,
                CssRect::new(0.0, 0.0, 4.0, 4.0).unwrap(),
                RuntimePaint::None,
                0,
            )
            .with_clip(owner_clip),
        ],
    )
    .unwrap();
    let bitmap =
        RuntimeGlyphBitmap::new(3, 2, 0, 0, 0, 0, Arc::from([0_u8, 64, 255, 128, 192, 255]))
            .unwrap();
    let text = RuntimeRenderText::new(
        node,
        base.key(),
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(1.0, 0.25, 0.0, 1.0).unwrap(),
        CssClip::unconstrained(),
        vec![bitmap],
    )
    .unwrap();
    let scene = base
        .with_texts_and_paint_commands(
            vec![text],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
            ],
        )
        .unwrap();

    let pixels = readback_runtime_scene_samples(
        &scene,
        4,
        4,
        &[(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (1, 3)],
    )
    .expect("macOS headless WGPU adapter must render partial glyph-mask coverage");
    assert_eq!(pixels[0], [0, 0, 0, 255]);
    assert_eq!(pixels[1], [64, 16, 0, 255]);
    assert_eq!(pixels[2], [0, 0, 0, 255]);
    assert_eq!(pixels[3], [128, 32, 0, 255]);
    assert_eq!(pixels[4], [192, 48, 0, 255]);
    assert_eq!(pixels[5], [0, 0, 0, 255]);
    assert_eq!(pixels[6], [0, 0, 0, 255]);
}

#[test]
fn wgpu_readback_keeps_glyph_ink_outside_an_unclipped_owner() {
    let document = HostDocument::new().unwrap().snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let node = NodeId::new(1).unwrap();
    let base = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(4.0, 4.0).unwrap(),
        vec![RuntimeRenderBox::new(
            node,
            CssRect::new(0.0, 0.0, 1.0, 1.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap();
    let bitmap =
        RuntimeGlyphBitmap::new(3, 2, 0, 0, 0, 0, Arc::from([0_u8, 64, 255, 128, 192, 255]))
            .unwrap();
    let text = RuntimeRenderText::new(
        node,
        base.key(),
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(1.0, 0.25, 0.0, 1.0).unwrap(),
        CssClip::unconstrained(),
        vec![bitmap],
    )
    .unwrap();
    let scene = base
        .with_texts_and_paint_commands(
            vec![text],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
            ],
        )
        .unwrap();

    let pixels = readback_runtime_scene_samples(&scene, 4, 4, &[(2, 0), (2, 1)])
        .expect("unclipped owner must retain glyph ink outside its own frame");
    assert_eq!(pixels, [[255, 64, 0, 255], [255, 64, 0, 255]]);
}

#[test]
fn wgpu_readback_composites_glyph_masks_inside_an_opacity_group() {
    let document = HostDocument::new().unwrap().snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let node = NodeId::new(1).unwrap();
    let base = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(4.0, 4.0).unwrap(),
        vec![RuntimeRenderBox::new(
            node,
            CssRect::new(0.0, 0.0, 4.0, 4.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap();
    let bitmap =
        RuntimeGlyphBitmap::new(2, 2, 0, 0, 0, 0, Arc::from([0_u8, 64, 128, 255])).unwrap();
    let text = RuntimeRenderText::new(
        node,
        base.key(),
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(1.0, 0.25, 0.0, 0.5).unwrap(),
        CssClip::unconstrained().with_x(spinon_render::CssAxisClip::new(0.0, 2.0).unwrap()),
        vec![bitmap],
    )
    .unwrap();
    let scene = base
        .with_texts_and_paint_commands(
            vec![text],
            vec![
                RuntimePaintCommand::BeginGroup { opacity: 0.5 },
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
                RuntimePaintCommand::EndGroup,
            ],
        )
        .unwrap();

    let pixels =
        readback_runtime_scene_samples(&scene, 4, 4, &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)])
            .expect("glyph mask inside an opacity group must render to the WGPU target");
    assert_eq!(pixels[0], [0, 0, 0, 255]);
    assert_eq!(pixels[1], [16, 4, 0, 255]);
    assert_eq!(pixels[2], [32, 8, 0, 255]);
    assert_eq!(pixels[3], [64, 16, 0, 255]);
    assert_eq!(pixels[4], [0, 0, 0, 255]);
}
