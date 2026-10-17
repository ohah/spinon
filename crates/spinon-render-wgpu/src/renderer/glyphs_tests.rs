use super::append_text_vertices;
use super::atlas::{MaskAtlasData, build_mask_atlas};
use bytemuck::cast_slice;
use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
use spinon_render::{
    CssAxisClip, CssClip, CssRect, CssSize, RuntimeGlyphBitmap, RuntimePaint, RuntimePaintCommand,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
};
use std::sync::Arc;

use crate::geometry::CssToSurfaceMapping;

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

fn text(glyphs: Vec<RuntimeGlyphBitmap>, clip: CssClip) -> RuntimeRenderText {
    text_for(glyphs, clip, key())
}

fn text_for(
    glyphs: Vec<RuntimeGlyphBitmap>,
    clip: CssClip,
    source: RuntimeRenderKey,
) -> RuntimeRenderText {
    RuntimeRenderText::new(
        NodeId::new(1).unwrap(),
        source,
        0.0,
        0.0,
        1.0,
        RuntimeSrgbColor::new(1.0, 0.25, 0.0, 1.0).unwrap(),
        clip,
        glyphs,
    )
    .unwrap()
}

fn bitmap(width: u32, height: u32, pixels: &[u8]) -> RuntimeGlyphBitmap {
    RuntimeGlyphBitmap::new(width, height, 0, 0, 1, 1, Arc::from(pixels)).unwrap()
}

#[test]
fn atlas_rows_keep_each_glyph_coverage_and_cache_dimensions_are_part_of_key() {
    let first = text(
        vec![bitmap(2, 2, &[0, 64, 128, 255]), bitmap(1, 1, &[192])],
        CssClip::unconstrained(),
    );
    let atlas = build_mask_atlas(&first, 16).unwrap().unwrap();
    assert_eq!((atlas.width, atlas.height), (3, 2));
    assert_eq!(atlas.placements, [(0, 0), (2, 0)]);
    assert_eq!(atlas.pixels, [0, 64, 192, 128, 255, 0]);

    let same_bytes_other_shape = MaskAtlasData {
        width: 2,
        height: 3,
        placements: Vec::new(),
        pixels: atlas.pixels.clone(),
    };
    let same_bytes_same_shape = MaskAtlasData {
        width: atlas.width,
        height: atlas.height,
        placements: Vec::new(),
        pixels: atlas.pixels.clone(),
    };
    let key = |atlas: &MaskAtlasData| {
        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(atlas.width.to_le_bytes());
        hasher.update(atlas.height.to_le_bytes());
        hasher.update(&atlas.pixels);
        <[u8; 32]>::from(hasher.finalize())
    };
    assert_ne!(key(&same_bytes_other_shape), key(&same_bytes_same_shape));
}

#[test]
fn glyph_vertices_clip_both_geometry_and_mask_coordinates_at_half_open_edge() {
    let text = text(
        vec![bitmap(2, 2, &[255; 4])],
        CssClip::unconstrained().with_x(CssAxisClip::new(2.0, 3.0).unwrap()),
    );
    let mut vertices = Vec::new();
    let scene = RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(4.0, 4.0).unwrap(),
        vec![RuntimeRenderBox::new(
            text.node_id(),
            CssRect::new(0.0, 0.0, 4.0, 4.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap();
    let mapping = CssToSurfaceMapping::new(&scene, 4, 4);
    append_text_vertices(&mut vertices, &text, &[(0, 0)], 2, 2, mapping, false).unwrap();
    assert_eq!(vertices.len(), 6);
    assert_eq!(vertices[0].position, [0.0, 0.5]);
    assert_eq!(vertices[2].position, [0.5, 0.5]);
    assert_eq!(vertices[0].uv, [0.5, 0.0]);
    assert_eq!(vertices[2].uv, [1.0, 0.0]);

    let bytes = cast_slice::<_, u8>(&vertices);
    assert_eq!(bytes.len(), std::mem::size_of_val(vertices.as_slice()));
}

#[test]
fn page_scale_maps_glyph_raster_and_clips_it_at_visual_viewport() {
    let document = HostDocument::new().unwrap().snapshot();
    let node = NodeId::new(1).unwrap();
    let base = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(500.0, 1082.0513).unwrap(),
        vec![RuntimeRenderBox::new(
            node,
            CssRect::new(100.0, 50.0, 20.0, 10.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap()
    .with_page_scale_transform(3.0, 1.0)
    .unwrap();
    let glyph =
        RuntimeGlyphBitmap::new(60, 30, 0, 0, 0, 0, Arc::from(vec![255_u8; 60 * 30])).unwrap();
    let visible_text = RuntimeRenderText::new(
        node,
        base.key(),
        100.0,
        50.0,
        3.0,
        RuntimeSrgbColor::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        CssClip::unconstrained(),
        vec![glyph],
    )
    .unwrap();
    let scene = base
        .with_texts_and_paint_commands(
            vec![visible_text],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
            ],
        )
        .unwrap();
    let atlas = build_mask_atlas(&scene.texts()[0], 1024).unwrap().unwrap();
    let mapping = CssToSurfaceMapping::new(&scene, 1170, 2532);
    let mut vertices = Vec::new();
    append_text_vertices(
        &mut vertices,
        &scene.texts()[0],
        &atlas.placements,
        atlas.width,
        atlas.height,
        mapping,
        false,
    )
    .unwrap();
    let to_x_px = |ndc: f32| (ndc + 1.0) * 1170.0 / 2.0;
    let to_y_px = |ndc: f32| (1.0 - ndc) * 2532.0 / 2.0;
    assert!((to_x_px(vertices[0].position[0]) - 300.0).abs() < 0.001);
    assert!((to_x_px(vertices[2].position[0]) - 360.0).abs() < 0.001);
    assert!((to_y_px(vertices[0].position[1]) - 150.0).abs() < 0.001);
    assert!((to_y_px(vertices[1].position[1]) - 180.0).abs() < 0.001);

    let outside_text = RuntimeRenderText::new(
        node,
        scene.key(),
        400.0,
        50.0,
        3.0,
        RuntimeSrgbColor::new(0.0, 0.0, 0.0, 1.0).unwrap(),
        CssClip::unconstrained(),
        vec![
            RuntimeGlyphBitmap::new(30, 30, 0, 0, 0, 0, Arc::from(vec![255_u8; 30 * 30])).unwrap(),
        ],
    )
    .unwrap();
    let mut clipped_vertices = Vec::new();
    let outside_atlas = build_mask_atlas(&outside_text, 1024).unwrap().unwrap();
    append_text_vertices(
        &mut clipped_vertices,
        &outside_text,
        &outside_atlas.placements,
        outside_atlas.width,
        outside_atlas.height,
        mapping,
        false,
    )
    .unwrap();
    assert!(clipped_vertices.is_empty());
}

#[test]
fn empty_mask_list_does_not_create_an_atlas() {
    let text = text(Vec::new(), CssClip::unconstrained());
    assert!(build_mask_atlas(&text, 16).unwrap().is_none());
}

#[test]
fn atlas_rejects_a_glyph_wider_than_the_device_texture_limit() {
    let glyph = bitmap(17, 1, &[255; 17]);
    let text = text(vec![glyph], CssClip::unconstrained());
    let error = match build_mask_atlas(&text, 16) {
        Err(error) => error,
        Ok(_) => panic!("texture 폭 초과 glyph atlas가 허용되었습니다"),
    };
    assert!(error.contains("texture 제한을 넘었습니다"));
}

#[test]
fn atlas_rejects_packed_rows_that_exceed_device_texture_height() {
    let glyphs = (0..3).map(|_| bitmap(4, 8, &[255; 32])).collect();
    let text = text(glyphs, CssClip::unconstrained());
    let error = match build_mask_atlas(&text, 15) {
        Err(error) => error,
        Ok(_) => panic!("texture 높이 초과 glyph atlas가 허용되었습니다"),
    };
    assert!(error.contains("높이 제한을 넘었습니다"));
}

#[test]
fn scene_paint_order_can_interleave_box_and_glyph_commands() {
    let node = NodeId::new(1).unwrap();
    let base_scene = RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(4.0, 4.0).unwrap(),
        vec![RuntimeRenderBox::new(
            node,
            CssRect::new(0.0, 0.0, 4.0, 4.0).unwrap(),
            RuntimePaint::None,
            0,
        )],
    )
    .unwrap();
    let scene = base_scene
        .with_texts_and_paint_commands(
            vec![text_for(
                vec![bitmap(2, 2, &[255; 4])],
                CssClip::unconstrained(),
                base_scene.key(),
            )],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawText { index: 0 },
            ],
        )
        .unwrap();
    assert!(matches!(
        scene.paint_commands()[1],
        RuntimePaintCommand::DrawText { index: 0 }
    ));
}
