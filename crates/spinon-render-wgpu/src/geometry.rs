use bytemuck::{Pod, Zeroable};
use spinon_render::{CssClip, RuntimePaint, RuntimeRenderSnapshot};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ColorEncoding {
    Linear,
    Srgb,
}

pub(crate) fn build_vertices(
    scene: &RuntimeRenderSnapshot,
    width: u32,
    height: u32,
    color_encoding: ColorEncoding,
) -> Result<Vec<Vertex>, String> {
    if width == 0 || height == 0 {
        return Err("WGPU 표면 크기는 0보다 커야 합니다".to_owned());
    }
    let viewport = scene.viewport_css_px();
    let quad_capacity = scene.boxes().iter().fold(0_usize, |count, render_box| {
        let border = render_box.border();
        let sides = [border.top(), border.right(), border.bottom(), border.left()];
        count
            .saturating_add(usize::from(matches!(
                render_box.paint(),
                RuntimePaint::Opaque(_)
            )))
            .saturating_add(
                sides
                    .iter()
                    .filter(|side| {
                        side.width_css_px() > 0.0 && matches!(side.paint(), RuntimePaint::Opaque(_))
                    })
                    .count(),
            )
    });
    let mut vertices = Vec::new();
    vertices
        .try_reserve(quad_capacity.saturating_mul(6))
        .map_err(|_| "WGPU geometry 버퍼를 할당하지 못했습니다".to_owned())?;

    for render_box in scene.boxes() {
        let rect = render_box.frame_css_px();
        let clip = render_box.clip_css_px();
        let x = rect.x();
        let y = rect.y();
        let right = x + rect.width();
        let bottom = y + rect.height();
        append_colored_rect(
            &mut vertices,
            [x, y, right, bottom],
            render_box.paint(),
            clip,
            viewport.width(),
            viewport.height(),
            color_encoding,
        );

        let border = render_box.border();
        let top_width = border.top().width_css_px().min(rect.height());
        let bottom_width = border
            .bottom()
            .width_css_px()
            .min(rect.height() - top_width);
        let left_width = border.left().width_css_px().min(rect.width());
        let right_width = border.right().width_css_px().min(rect.width() - left_width);
        append_colored_rect(
            &mut vertices,
            [x, y, right, y + top_width],
            border.top().paint(),
            clip,
            viewport.width(),
            viewport.height(),
            color_encoding,
        );
        append_colored_rect(
            &mut vertices,
            [
                right - right_width,
                y + top_width,
                right,
                bottom - bottom_width,
            ],
            border.right().paint(),
            clip,
            viewport.width(),
            viewport.height(),
            color_encoding,
        );
        append_colored_rect(
            &mut vertices,
            [x, bottom - bottom_width, right, bottom],
            border.bottom().paint(),
            clip,
            viewport.width(),
            viewport.height(),
            color_encoding,
        );
        append_colored_rect(
            &mut vertices,
            [x, y + top_width, x + left_width, bottom - bottom_width],
            border.left().paint(),
            clip,
            viewport.width(),
            viewport.height(),
            color_encoding,
        );
    }
    Ok(vertices)
}

fn append_colored_rect(
    vertices: &mut Vec<Vertex>,
    rect: [f32; 4],
    paint: RuntimePaint,
    clip: CssClip,
    viewport_width: f32,
    viewport_height: f32,
    color_encoding: ColorEncoding,
) {
    let RuntimePaint::Opaque(color) = paint else {
        return;
    };
    let [x, y, right, bottom] = rect;
    let x0_css = clip.x().map_or(x, |axis| x.max(axis.min()));
    let x1_css = clip.x().map_or(right, |axis| right.min(axis.max()));
    let y0_css = clip.y().map_or(y, |axis| y.max(axis.min()));
    let y1_css = clip.y().map_or(bottom, |axis| bottom.min(axis.max()));
    if x1_css <= x0_css || y1_css <= y0_css {
        return;
    }
    let x0 = -1.0 + 2.0 * x0_css / viewport_width;
    let x1 = -1.0 + 2.0 * x1_css / viewport_width;
    let y0 = 1.0 - 2.0 * y0_css / viewport_height;
    let y1 = 1.0 - 2.0 * y1_css / viewport_height;
    let encode = match color_encoding {
        ColorEncoding::Linear => srgb_to_linear,
        ColorEncoding::Srgb => srgb_encoded,
    };
    let color = [
        encode(color.red()),
        encode(color.green()),
        encode(color.blue()),
        1.0,
    ];
    let top_left = Vertex {
        position: [x0, y0],
        color,
    };
    let top_right = Vertex {
        position: [x1, y0],
        color,
    };
    let bottom_left = Vertex {
        position: [x0, y1],
        color,
    };
    let bottom_right = Vertex {
        position: [x1, y1],
        color,
    };
    vertices.extend_from_slice(&[
        top_left,
        bottom_left,
        top_right,
        top_right,
        bottom_left,
        bottom_right,
    ]);
}

fn srgb_to_linear(channel: u8) -> f32 {
    let encoded = f32::from(channel) / 255.0;
    if encoded <= 0.04045 {
        encoded / 12.92
    } else {
        ((encoded + 0.055) / 1.055).powf(2.4)
    }
}

fn srgb_encoded(channel: u8) -> f32 {
    f32::from(channel) / 255.0
}

#[cfg(test)]
mod tests {
    use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
    use spinon_render::{
        CssAxisClip, CssClip, CssRect, CssSize, RuntimeBorder, RuntimeBorderSide, RuntimePaint,
        RuntimeRenderBox, RuntimeRenderKey,
    };

    use super::{ColorEncoding, build_vertices};

    fn scene() -> spinon_render::RuntimeRenderSnapshot {
        let document = HostDocument::new().unwrap();
        let snapshot = document.snapshot();
        spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                snapshot.generation(),
                snapshot.document_revision(),
                snapshot.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(100.0, 50.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(0.0, 0.0, 50.0, 25.0).unwrap(),
                    RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(51, 102, 255)),
                    0,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(2).unwrap(),
                    CssRect::new(50.0, 0.0, 25.0, 25.0).unwrap(),
                    RuntimePaint::None,
                    1,
                ),
            ],
        )
        .unwrap()
    }

    #[test]
    fn clip_intersects_the_quad_before_gpu_vertex_generation() {
        let document = HostDocument::new().unwrap();
        let source = document.snapshot();
        let scene = spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                source.generation(),
                source.document_revision(),
                source.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(100.0, 100.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(9).unwrap(),
                    CssRect::new(0.0, 0.0, 100.0, 100.0).unwrap(),
                    RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(255, 0, 0)),
                    0,
                )
                .with_clip(
                    CssClip::unconstrained()
                        .with_x(CssAxisClip::new(20.0, 60.0).unwrap())
                        .with_y(CssAxisClip::new(10.0, 40.0).unwrap()),
                ),
            ],
        )
        .unwrap();

        let vertices = build_vertices(&scene, 100, 100, ColorEncoding::Srgb).unwrap();
        assert_eq!(vertices.len(), 6);
        for (actual, expected) in vertices[0].position.into_iter().zip([-0.6, 0.8]) {
            assert!((actual - expected).abs() < 1.0e-6);
        }
        for (actual, expected) in vertices[5].position.into_iter().zip([0.2, 0.2]) {
            assert!((actual - expected).abs() < 1.0e-6);
        }
    }

    #[test]
    fn geometry_preserves_css_edges_and_skips_transparent_paint() {
        let vertices = build_vertices(&scene(), 200, 100, ColorEncoding::Linear).unwrap();
        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0].position, [-1.0, 1.0]);
        assert_eq!(vertices[1].position, [-1.0, 0.0]);
        assert_eq!(vertices[2].position, [0.0, 1.0]);
        assert!((vertices[0].color[0] - 0.0331).abs() < 0.0002);
        assert!((vertices[0].color[1] - 0.1329).abs() < 0.0002);
        assert!((vertices[0].color[2] - 1.0).abs() < 0.0001);
    }

    #[test]
    fn geometry_rejects_zero_sized_surface() {
        assert!(build_vertices(&scene(), 0, 100, ColorEncoding::Linear).is_err());
        assert!(build_vertices(&scene(), 100, 0, ColorEncoding::Linear).is_err());
    }

    #[test]
    fn geometry_keeps_srgb_encoded_values_for_unorm_surface_formats() {
        let vertices = build_vertices(&scene(), 200, 100, ColorEncoding::Srgb).unwrap();
        assert!((vertices[0].color[0] - 51.0 / 255.0).abs() < f32::EPSILON);
        assert!((vertices[0].color[1] - 102.0 / 255.0).abs() < f32::EPSILON);
        assert_eq!(vertices[0].color[2], 1.0);
    }

    #[test]
    fn border_paint_uses_css_side_widths_and_the_ancestor_clip() {
        let document = HostDocument::new().unwrap();
        let source = document.snapshot();
        let border_color = RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(240, 198, 116));
        let side = RuntimeBorderSide::new(4.0, border_color).unwrap();
        let scene = spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                source.generation(),
                source.document_revision(),
                source.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(400.0, 160.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(10.0, 10.0, 120.0, 96.0).unwrap(),
                    RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(52, 66, 86)),
                    0,
                )
                .with_border(RuntimeBorder::new(side, side, side, side))
                .with_clip(CssClip::unconstrained().with_x(CssAxisClip::new(14.0, 130.0).unwrap())),
            ],
        )
        .unwrap();

        let vertices = build_vertices(&scene, 400, 160, ColorEncoding::Srgb).unwrap();
        assert_eq!(
            vertices.len(),
            24,
            "ancestor clipping removes the left border quad"
        );
        let right_border_first = vertices[12];
        assert!((right_border_first.position[0] - (-0.37)).abs() < 1.0e-6);
        assert!((right_border_first.position[1] - 0.825).abs() < 1.0e-6);
        assert!((right_border_first.color[0] - 240.0 / 255.0).abs() < f32::EPSILON);
        assert!((right_border_first.color[1] - 198.0 / 255.0).abs() < f32::EPSILON);
        assert!((right_border_first.color[2] - 116.0 / 255.0).abs() < f32::EPSILON);
        let right_border_inner_edge = vertices[14];
        assert!((right_border_inner_edge.position[0] - (-0.35)).abs() < 1.0e-6);
    }

    #[test]
    fn runtime_border_side_rejects_non_finite_or_negative_widths() {
        let paint = RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(240, 198, 116));
        assert!(RuntimeBorderSide::new(f32::NAN, paint).is_err());
        assert!(RuntimeBorderSide::new(f32::INFINITY, paint).is_err());
        assert!(RuntimeBorderSide::new(-1.0, paint).is_err());
    }
}
