use spinon_render::{OpaqueCssSrgb, StaticRenderSnapshot};

pub(super) const FIXTURE_WIDTH: u32 = 301;
pub(super) const FIXTURE_HEIGHT: u32 = 65;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SurfaceMapping {
    viewport_width: f32,
    viewport_height: f32,
    surface_width: f32,
    surface_height: f32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl SurfaceMapping {
    pub(super) fn new(
        snapshot: &StaticRenderSnapshot,
        surface_width: u32,
        surface_height: u32,
        density: f32,
    ) -> Result<Self, String> {
        if surface_width == 0 || surface_height == 0 || !density.is_finite() || density <= 0.0 {
            return Err("S04 surface 크기·density가 잘못됐습니다".to_owned());
        }
        let viewport = snapshot.viewport_css_px();
        let surface_width = surface_width as f32;
        let surface_height = surface_height as f32;
        let fit = (surface_width / viewport.width()).min(surface_height / viewport.height());
        let scale = density.min(fit);
        if !scale.is_finite() || scale <= 0.0 {
            return Err("S04 CSS px 변환 배율이 잘못됐습니다".to_owned());
        }
        let offset_x = (surface_width - viewport.width() * scale) / 2.0;
        let offset_y = (surface_height - viewport.height() * scale) / 2.0;
        if !offset_x.is_finite() || !offset_y.is_finite() {
            return Err("S04 surface letterbox offset이 잘못됐습니다".to_owned());
        }
        Ok(Self {
            viewport_width: viewport.width(),
            viewport_height: viewport.height(),
            surface_width,
            surface_height,
            scale,
            offset_x,
            offset_y,
        })
    }

    pub(super) fn css_point_from_surface(
        self,
        surface_x: f32,
        surface_y: f32,
    ) -> Result<Option<(f32, f32)>, String> {
        if !surface_x.is_finite() || !surface_y.is_finite() {
            return Err("S04 입력 좌표는 유한한 표면 픽셀이어야 합니다".to_owned());
        }
        let content_right = self.offset_x + self.viewport_width * self.scale;
        let content_bottom = self.offset_y + self.viewport_height * self.scale;
        if surface_x < self.offset_x
            || surface_x >= content_right
            || surface_y < self.offset_y
            || surface_y >= content_bottom
        {
            return Ok(None);
        }
        let css_x = (surface_x - self.offset_x) / self.scale;
        let css_y = (surface_y - self.offset_y) / self.scale;
        if css_x < 0.0
            || css_x >= self.viewport_width
            || css_y < 0.0
            || css_y >= self.viewport_height
        {
            return Ok(None);
        }
        Ok(Some((css_x, css_y)))
    }

    fn rect_ndc(self, x: f32, y: f32, width: f32, height: f32) -> [f32; 4] {
        let left = self.offset_x + x * self.scale;
        let right = self.offset_x + (x + width) * self.scale;
        let top = self.offset_y + y * self.scale;
        let bottom = self.offset_y + (y + height) * self.scale;
        [
            left / self.surface_width * 2.0 - 1.0,
            1.0 - top / self.surface_height * 2.0,
            right / self.surface_width * 2.0 - 1.0,
            1.0 - bottom / self.surface_height * 2.0,
        ]
    }
}

pub(super) fn create_vertex_buffer(device: &wgpu::Device, vertices: &[f32]) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("spinon-s04-static-snapshot-vertices"),
        size: std::mem::size_of_val(vertices) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub(super) fn build_vertices_with_mapping(
    snapshot: &StaticRenderSnapshot,
    mapping: SurfaceMapping,
) -> Result<Vec<f32>, String> {
    let mut vertices = Vec::with_capacity(snapshot.boxes().len() * 36);
    for render_box in snapshot.boxes() {
        let frame = render_box.frame_css_px();
        let [left, top, right, bottom] =
            mapping.rect_ndc(frame.x(), frame.y(), frame.width(), frame.height());
        let color = linear_rgba(render_box.paint());
        for position in [
            [left, top],
            [right, top],
            [left, bottom],
            [left, bottom],
            [right, top],
            [right, bottom],
        ] {
            vertices.extend_from_slice(&[
                position[0],
                position[1],
                color[0],
                color[1],
                color[2],
                color[3],
            ]);
        }
    }
    Ok(vertices)
}

fn linear_rgba(color: OpaqueCssSrgb) -> [f32; 4] {
    [
        srgb_to_linear(f32::from(color.red()) / 255.0),
        srgb_to_linear(f32::from(color.green()) / 255.0),
        srgb_to_linear(f32::from(color.blue()) / 255.0),
        1.0,
    ]
}

pub(super) fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::{build_vertices_with_mapping, SurfaceMapping};
    use crate::s04_snapshot::build_asymmetric_y_snapshot;

    #[test]
    fn surface_mapping_inverts_the_same_scale_and_letterbox_used_for_vertices() {
        let snapshot = build_asymmetric_y_snapshot().expect("S04 snapshot");
        let mapping = SurfaceMapping::new(&snapshot, 1206, 2622, 4.0).expect("surface mapping");
        let vertices = build_vertices_with_mapping(&snapshot, mapping).expect("surface vertices");

        assert_eq!(vertices.len(), snapshot.boxes().len() * 36);
        assert_eq!(
            mapping.css_point_from_surface(603.0, 1207.0).unwrap(),
            Some((150.5, 6.5))
        );
        assert_eq!(mapping.css_point_from_surface(0.99, 1207.0).unwrap(), None);
        assert_eq!(
            mapping.css_point_from_surface(1205.0, 1207.0).unwrap(),
            None
        );
        assert_eq!(
            mapping.css_point_from_surface(603.0, 1180.99).unwrap(),
            None
        );
        assert_eq!(mapping.css_point_from_surface(603.0, 1441.0).unwrap(), None);
        assert!(mapping.css_point_from_surface(f32::NAN, 1207.0).is_err());
    }

    #[test]
    fn surface_mapping_includes_viewport_origin_and_excludes_right_bottom_edges() {
        let snapshot = build_asymmetric_y_snapshot().expect("S04 snapshot");
        let mapping = SurfaceMapping::new(&snapshot, 1206, 2622, 4.0).expect("surface mapping");

        assert_eq!(
            mapping.css_point_from_surface(1.0, 1181.0).unwrap(),
            Some((0.0, 0.0))
        );
        assert_eq!(
            mapping.css_point_from_surface(1205.0, 1181.0).unwrap(),
            None
        );
        assert_eq!(mapping.css_point_from_surface(603.0, 1441.0).unwrap(), None);
        assert!(mapping
            .css_point_from_surface(1204.99, 1440.99)
            .unwrap()
            .is_some());
    }

    #[test]
    fn surface_mapping_rejects_invalid_dimensions_and_density() {
        let snapshot = build_asymmetric_y_snapshot().expect("S04 snapshot");
        assert!(SurfaceMapping::new(&snapshot, 0, 100, 1.0).is_err());
        assert!(SurfaceMapping::new(&snapshot, 100, 100, 0.0).is_err());
        assert!(SurfaceMapping::new(&snapshot, 100, 100, f32::INFINITY).is_err());
    }
}
