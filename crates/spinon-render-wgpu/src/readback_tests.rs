use super::readback_runtime_scene_samples;
use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
use spinon_render::{
    CssClip, CssRect, CssSize, OpaqueCssSrgb, RuntimeBorder, RuntimeBorderSide, RuntimeGlyphBitmap,
    RuntimePaint, RuntimePaintCommand, RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot,
    RuntimeRenderText, RuntimeSrgbColor,
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
fn page_scale_readback_matches_chrome_scale_two_and_clips_the_physical_edges() {
    let document = HostDocument::new().unwrap().snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let red = RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 0, 0));
    let blue = RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 0, 255));
    let green = RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 255, 0));
    let yellow = RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 255, 0));
    let scene = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(390.0, 844.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(0.0, 0.0, 390.0, 844.0).unwrap(),
                RuntimePaint::None,
                0,
            ),
            RuntimeRenderBox::new(
                NodeId::new(2).unwrap(),
                CssRect::new(100.0, 50.0, 20.0, 10.0).unwrap(),
                red,
                1,
            ),
            RuntimeRenderBox::new(
                NodeId::new(3).unwrap(),
                CssRect::new(190.0, 100.0, 10.0, 10.0).unwrap(),
                blue,
                2,
            ),
            RuntimeRenderBox::new(
                NodeId::new(4).unwrap(),
                CssRect::new(196.0, 180.0, 5.0, 5.0).unwrap(),
                yellow,
                3,
            ),
            RuntimeRenderBox::new(
                NodeId::new(5).unwrap(),
                CssRect::new(10.0, 417.0, 10.0, 10.0).unwrap(),
                green,
                4,
            ),
        ],
    )
    .unwrap()
    .with_page_scale_transform(3.0, 2.0)
    .unwrap();

    let samples = readback_runtime_scene_samples(
        &scene,
        1170,
        2532,
        &[
            (600, 300),
            (719, 359),
            (720, 359),
            (1_139, 601),
            (1_169, 601),
            (1_169, 1_081),
            (61, 2_501),
            (61, 2_531),
        ],
    )
    .expect("fixed Chrome scale-two scene should render with WGPU");
    assert_eq!(
        samples,
        [
            [255, 0, 0, 255],
            [255, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 0, 255, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 255, 0, 255],
        ]
    );
}

#[test]
fn page_scale_readback_preserves_css_clip_border_order_and_opacity_groups() {
    let document = HostDocument::new().unwrap().snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let red = RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 0, 0));
    let blue = RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 0, 255));
    let green = RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 255, 0));
    let no_border = RuntimeBorderSide::new(0.0, RuntimePaint::None).unwrap();
    let top_border = RuntimeBorderSide::new(2.0, blue).unwrap();
    let scene = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(60.0, 60.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(10.0, 10.0, 10.0, 10.0).unwrap(),
                red,
                0,
            )
            .with_border(RuntimeBorder::new(
                top_border, no_border, no_border, no_border,
            ))
            .with_clip(
                CssClip::unconstrained()
                    .with_x(spinon_render::CssAxisClip::new(11.0, 19.0).unwrap())
                    .with_y(spinon_render::CssAxisClip::new(10.0, 20.0).unwrap()),
            ),
            RuntimeRenderBox::new(
                NodeId::new(2).unwrap(),
                CssRect::new(25.0, 10.0, 10.0, 10.0).unwrap(),
                green,
                1,
            ),
        ],
    )
    .unwrap()
    .with_page_scale_transform(3.0, 1.0)
    .unwrap()
    .with_texts_and_paint_commands(
        Vec::new(),
        vec![
            RuntimePaintCommand::DrawBox { index: 0 },
            RuntimePaintCommand::BeginGroup { opacity: 0.5 },
            RuntimePaintCommand::DrawBox { index: 1 },
            RuntimePaintCommand::EndGroup,
        ],
    )
    .unwrap();

    let samples = readback_runtime_scene_samples(
        &scene,
        180,
        180,
        &[(34, 31), (40, 37), (31, 37), (58, 37), (90, 45)],
    )
    .expect("page-scale CSS clip, border and opacity scene should render with WGPU");
    assert_eq!(
        samples,
        [
            [0, 0, 255, 255],
            [255, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 0, 0, 255],
            [0, 127, 0, 255],
        ]
    );
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
