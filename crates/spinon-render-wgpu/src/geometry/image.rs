use bytemuck::{Pod, Zeroable};
use spinon_render::RuntimeRenderSnapshot;
use std::ops::Range;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct ImageVertex {
    pub(super) position: [f32; 2],
    pub(super) uv: [f32; 2],
}

pub(crate) struct ImageGeometry {
    pub(crate) vertices: Vec<ImageVertex>,
    ranges: Vec<Range<u32>>,
}

impl ImageGeometry {
    pub(crate) fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            ranges: Vec::new(),
        }
    }

    pub(crate) fn range_for_image(&self, index: usize) -> Option<Range<u32>> {
        self.ranges.get(index).cloned()
    }
}

pub(crate) fn build_image_geometry(
    scene: &RuntimeRenderSnapshot,
    width: u32,
    height: u32,
) -> Result<ImageGeometry, String> {
    if width == 0 || height == 0 {
        return Err("WGPU 표면 크기는 0보다 커야 합니다".to_owned());
    }
    let viewport = scene.viewport_css_px();
    let mut geometry = ImageGeometry {
        vertices: Vec::new(),
        ranges: Vec::new(),
    };
    geometry
        .vertices
        .try_reserve(scene.images().len().saturating_mul(6))
        .map_err(|_| "image 정점 버퍼를 할당하지 못했습니다".to_owned())?;
    geometry
        .ranges
        .try_reserve(scene.images().len())
        .map_err(|_| "image draw 범위를 할당하지 못했습니다".to_owned())?;

    for image in scene.images() {
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == image.node_id())
            .ok_or_else(|| format!("runtime image node {}가 장면에 없습니다", image.node_id()))?;
        let frame = render_box.frame_css_px();
        if frame.width() != image.width() as f32 || frame.height() != image.height() as f32 {
            return Err(format!(
                "runtime image node {}의 frame이 natural size와 다릅니다",
                image.node_id()
            ));
        }
        let clip = render_box.clip_css_px();
        let x0 = clip.x().map_or(frame.x(), |axis| frame.x().max(axis.min()));
        let x1 = clip.x().map_or(frame.x() + frame.width(), |axis| {
            (frame.x() + frame.width()).min(axis.max())
        });
        let y0 = clip.y().map_or(frame.y(), |axis| frame.y().max(axis.min()));
        let y1 = clip.y().map_or(frame.y() + frame.height(), |axis| {
            (frame.y() + frame.height()).min(axis.max())
        });
        let first = u32::try_from(geometry.vertices.len())
            .map_err(|_| "image 정점 index가 범위를 넘었습니다".to_owned())?;
        if x1 > x0 && y1 > y0 {
            append_image_quad(
                &mut geometry.vertices,
                [x0, y0, x1, y1],
                [
                    (x0 - frame.x()) / frame.width(),
                    (y0 - frame.y()) / frame.height(),
                    (x1 - frame.x()) / frame.width(),
                    (y1 - frame.y()) / frame.height(),
                ],
                viewport.width(),
                viewport.height(),
            )?;
        }
        let last = u32::try_from(geometry.vertices.len())
            .map_err(|_| "image 정점 index가 범위를 넘었습니다".to_owned())?;
        geometry.ranges.push(first..last);
    }
    Ok(geometry)
}

fn append_image_quad(
    vertices: &mut Vec<ImageVertex>,
    rect: [f32; 4],
    uv: [f32; 4],
    viewport_width: f32,
    viewport_height: f32,
) -> Result<(), String> {
    let [x0_css, y0_css, x1_css, y1_css] = rect;
    let [u0, v0, u1, v1] = uv;
    if ![x0_css, y0_css, x1_css, y1_css, u0, v0, u1, v1]
        .into_iter()
        .all(f32::is_finite)
        || x1_css <= x0_css
        || y1_css <= y0_css
        || u0 < 0.0
        || v0 < 0.0
        || u1 > 1.0
        || v1 > 1.0
        || u1 <= u0
        || v1 <= v0
    {
        return Err("image quad 또는 UV가 유효하지 않습니다".to_owned());
    }
    let x0 = -1.0 + 2.0 * x0_css / viewport_width;
    let x1 = -1.0 + 2.0 * x1_css / viewport_width;
    let y0 = 1.0 - 2.0 * y0_css / viewport_height;
    let y1 = 1.0 - 2.0 * y1_css / viewport_height;
    let top_left = ImageVertex {
        position: [x0, y0],
        uv: [u0, v0],
    };
    let top_right = ImageVertex {
        position: [x1, y0],
        uv: [u1, v0],
    };
    let bottom_left = ImageVertex {
        position: [x0, y1],
        uv: [u0, v1],
    };
    let bottom_right = ImageVertex {
        position: [x1, y1],
        uv: [u1, v1],
    };
    vertices.extend_from_slice(&[
        top_left,
        bottom_left,
        top_right,
        top_right,
        bottom_left,
        bottom_right,
    ]);
    Ok(())
}
