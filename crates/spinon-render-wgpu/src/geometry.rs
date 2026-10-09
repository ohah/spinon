use bytemuck::{Pod, Zeroable};
use spinon_render::{RuntimePaint, RuntimeRenderSnapshot};

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
    let mut vertices = Vec::new();
    vertices
        .try_reserve(scene.boxes().len().saturating_mul(6))
        .map_err(|_| "WGPU geometry 버퍼를 할당하지 못했습니다".to_owned())?;

    for render_box in scene.boxes() {
        let RuntimePaint::Opaque(color) = render_box.paint() else {
            continue;
        };
        let rect = render_box.frame_css_px();
        if rect.width() == 0.0 || rect.height() == 0.0 {
            continue;
        }
        let x0 = -1.0 + 2.0 * rect.x() / viewport.width();
        let x1 = -1.0 + 2.0 * (rect.x() + rect.width()) / viewport.width();
        let y0 = 1.0 - 2.0 * rect.y() / viewport.height();
        let y1 = 1.0 - 2.0 * (rect.y() + rect.height()) / viewport.height();
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
    Ok(vertices)
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
    use spinon_render::{CssRect, CssSize, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey};

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
}
