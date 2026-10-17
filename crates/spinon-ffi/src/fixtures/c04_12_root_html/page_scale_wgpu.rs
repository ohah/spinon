use super::SpinonRuntimeRootHtmlEnvironment;

#[cfg(not(test))]
pub(super) fn validate_page_scale_wgpu_case(
    name: &str,
    environment: SpinonRuntimeRootHtmlEnvironment,
) -> Result<(), String> {
    use spinon_core::{
        EnvironmentRevision, HostDocument, IntrinsicSizeRevision, NodeId, StyleRevision,
    };
    use spinon_render::{
        CssRect, CssSize, OpaqueCssSrgb, RuntimeGlyphBitmap, RuntimeImageResourceKey, RuntimePaint,
        RuntimePaintCommand, RuntimeRenderBox, RuntimeRenderImage, RuntimeRenderKey,
        RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
    };
    use spinon_render_wgpu::readback_runtime_scene_samples;
    use std::sync::Arc;

    const SURFACE_WIDTH: u32 = 1170;
    const SURFACE_HEIGHT: u32 = 2532;
    let document = HostDocument::new().map_err(|error| format!("{name} document: {error}"))?;
    let document = document.snapshot();
    let intrinsic_size_revision = IntrinsicSizeRevision::INITIAL;
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .with_intrinsic_size_revision(intrinsic_size_revision);
    let visual_width = environment.visual_viewport_width_css_px;
    let visual_height = environment.visual_viewport_height_css_px;
    let add_box = |id, frame, paint, order| {
        RuntimeRenderBox::new(
            NodeId::new(id).expect("fixed fixture node ID"),
            frame,
            paint,
            order,
        )
    };
    let mut scene = RuntimeRenderSnapshot::new(
        key,
        CssSize::new(
            environment.viewport_width_css_px,
            environment.viewport_height_css_px,
        )
        .map_err(|error| format!("{name} layout viewport: {error}"))?,
        vec![
            add_box(
                1,
                CssRect::new(
                    0.0,
                    0.0,
                    environment.viewport_width_css_px,
                    environment.viewport_height_css_px,
                )
                .map_err(|error| format!("{name} root frame: {error}"))?,
                RuntimePaint::None,
                0,
            ),
            add_box(
                2,
                CssRect::new(100.0, 50.0, 20.0, 10.0)
                    .map_err(|error| format!("{name} probe frame: {error}"))?,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 0, 0)),
                1,
            ),
            add_box(
                3,
                CssRect::new(visual_width - 5.0, 100.0, 10.0, 10.0)
                    .map_err(|error| format!("{name} right edge frame: {error}"))?,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 0, 255)),
                2,
            ),
            add_box(
                4,
                CssRect::new(visual_width + 1.0, 180.0, 5.0, 5.0)
                    .map_err(|error| format!("{name} outside-right control frame: {error}"))?,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 255, 0)),
                3,
            ),
            add_box(
                5,
                CssRect::new(10.0, visual_height - 5.0, 10.0, 10.0)
                    .map_err(|error| format!("{name} bottom edge frame: {error}"))?,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(0, 255, 0)),
                4,
            ),
            add_box(
                6,
                CssRect::new(30.0, visual_height + 1.0, 5.0, 5.0)
                    .map_err(|error| format!("{name} outside-bottom control frame: {error}"))?,
                RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 255, 0)),
                5,
            ),
            add_box(
                7,
                CssRect::new(40.0, 200.0, 2.0, 2.0)
                    .map_err(|error| format!("{name} image frame: {error}"))?,
                RuntimePaint::None,
                6,
            ),
            add_box(
                8,
                CssRect::new(60.0, 300.0, 20.0, 10.0)
                    .map_err(|error| format!("{name} glyph frame: {error}"))?,
                RuntimePaint::None,
                7,
            ),
        ],
    )
    .map_err(|error| format!("{name} render snapshot: {error}"))?
    .with_page_scale_transform(environment.device_scale_factor, environment.page_scale)
    .map_err(|error| format!("{name} page-scale mapping: {error}"))?;

    let image_node = NodeId::new(7).expect("fixed image node ID");
    let image_source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        intrinsic_size_revision,
    );
    scene = scene
        .with_images(vec![
            RuntimeRenderImage::new(
                image_node,
                image_source,
                2,
                2,
                Arc::from([255_u8, 0, 255, 255].repeat(4)),
            )
            .map_err(|error| format!("{name} image snapshot: {error}"))?,
        ])
        .map_err(|error| format!("{name} image attachment: {error}"))?;

    let glyph_node = NodeId::new(8).expect("fixed glyph node ID");
    let text = RuntimeRenderText::new(
        glyph_node,
        key,
        60.0,
        300.0,
        environment.device_scale_factor,
        RuntimeSrgbColor::new(1.0, 1.0, 1.0, 1.0)
            .map_err(|error| format!("{name} glyph color: {error}"))?,
        spinon_render::CssClip::unconstrained(),
        vec![
            RuntimeGlyphBitmap::new(60, 30, 0, 0, 0, 0, Arc::from(vec![255_u8; 60 * 30]))
                .map_err(|error| format!("{name} glyph bitmap: {error}"))?,
        ],
    )
    .map_err(|error| format!("{name} glyph snapshot: {error}"))?;
    let mut commands = (0..scene.boxes().len())
        .map(|index| RuntimePaintCommand::DrawBox { index })
        .collect::<Vec<_>>();
    commands.push(RuntimePaintCommand::DrawText { index: 0 });
    scene = scene
        .with_texts_and_paint_commands(vec![text], commands)
        .map_err(|error| format!("{name} glyph attachment: {error}"))?;

    let scale = f64::from(environment.device_scale_factor * environment.page_scale);
    let visual_width_px = f64::from(visual_width) * scale;
    let visual_height_px = f64::from(visual_height) * scale;
    let right_edge_x = (visual_width_px - 2.0 * scale).floor() as u32;
    let before_right_x = (visual_width_px - 10.0 * scale).floor() as u32;
    let image_x = (40.0 * scale).floor() as u32 + 1;
    let image_y = (200.0 * scale).floor() as u32 + 1;
    let glyph_x = (60.0 * scale).floor() as u32 + 1;
    let glyph_y = (300.0 * scale).floor() as u32 + 1;
    let edge_y = (100.0 * scale).floor() as u32 + 1;
    let outside_right_y = (180.0 * scale).floor() as u32 + 1;
    let probe_x = (100.0 * scale).floor() as u32;
    let probe_y = (50.0 * scale).floor() as u32;
    let bottom_edge_x = (15.0 * scale).floor() as u32 + 2;
    let before_bottom_y = (visual_height_px - 10.0 * scale).floor() as u32;
    let outside_bottom_x = (30.0 * scale).floor() as u32 + 1;
    let samples = readback_runtime_scene_samples(
        &scene,
        SURFACE_WIDTH,
        SURFACE_HEIGHT,
        &[
            (probe_x, probe_y),
            ((120.0 * scale).ceil() as u32, probe_y),
            (before_right_x, edge_y),
            (right_edge_x.min(SURFACE_WIDTH - 1), edge_y),
            (SURFACE_WIDTH - 1, outside_right_y),
            (bottom_edge_x, before_bottom_y),
            (bottom_edge_x, SURFACE_HEIGHT - 1),
            (outside_bottom_x, SURFACE_HEIGHT - 1),
            (image_x, image_y),
            (glyph_x, glyph_y),
        ],
    )
    .map_err(|error| format!("{name} WGPU page-scale readback: {error}"))?;
    let expected = [
        [255, 0, 0, 255],
        [0, 0, 0, 255],
        [0, 0, 0, 255],
        [0, 0, 255, 255],
        [0, 0, 0, 255],
        [0, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 0, 255],
        [255, 0, 255, 255],
        [255, 255, 255, 255],
    ];
    if samples != expected {
        return Err(format!(
            "{name} WGPU page-scale sample mismatch: actual={samples:?} expected={expected:?}"
        ));
    }
    Ok(())
}
