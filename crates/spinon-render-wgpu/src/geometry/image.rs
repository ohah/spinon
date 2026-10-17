use bytemuck::{Pod, Zeroable};
use spinon_render::RuntimeRenderSnapshot;
use std::ops::Range;

use super::CssToSurfaceMapping;

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
    let mapping = CssToSurfaceMapping::new(scene, width, height);
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
        let object = image
            .object_rect_css_px(frame)
            .map_err(|error| error.to_string())?;
        let frame_right = frame.x() + frame.width();
        let frame_bottom = frame.y() + frame.height();
        let object_right = object.x() + object.width();
        let object_bottom = object.y() + object.height();
        if ![
            frame.x(),
            frame.y(),
            frame_right,
            frame_bottom,
            object.x(),
            object.y(),
            object_right,
            object_bottom,
        ]
        .into_iter()
        .all(f32::is_finite)
        {
            return Err(format!(
                "runtime image node {}의 frame/object 경계가 유한하지 않습니다",
                image.node_id()
            ));
        }
        let clip = render_box.clip_css_px();
        let clip_x0 = clip.x().map_or(f32::NEG_INFINITY, |axis| axis.min());
        let clip_x1 = clip.x().map_or(f32::INFINITY, |axis| axis.max());
        let clip_y0 = clip.y().map_or(f32::NEG_INFINITY, |axis| axis.min());
        let clip_y1 = clip.y().map_or(f32::INFINITY, |axis| axis.max());
        let x0 = frame.x().max(object.x()).max(clip_x0);
        let x1 = frame_right.min(object_right).min(clip_x1);
        let y0 = frame.y().max(object.y()).max(clip_y0);
        let y1 = frame_bottom.min(object_bottom).min(clip_y1);
        let first = u32::try_from(geometry.vertices.len())
            .map_err(|_| "image 정점 index가 범위를 넘었습니다".to_owned())?;
        if x1 > x0 && y1 > y0 {
            let u0 = ((x0 - object.x()) / object.width()).clamp(0.0, 1.0);
            let v0 = ((y0 - object.y()) / object.height()).clamp(0.0, 1.0);
            let u1 = ((x1 - object.x()) / object.width()).clamp(0.0, 1.0);
            let v1 = ((y1 - object.y()) / object.height()).clamp(0.0, 1.0);
            append_image_quad(
                &mut geometry.vertices,
                [x0, y0, x1, y1],
                [u0, v0, u1, v1],
                mapping,
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
    mapping: CssToSurfaceMapping,
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
    let x0 = mapping.x_to_ndc(x0_css)?;
    let x1 = mapping.x_to_ndc(x1_css)?;
    let y0 = mapping.y_to_ndc(y0_css)?;
    let y1 = mapping.y_to_ndc(y1_css)?;
    if ![x0, x1, y0, y1].into_iter().all(f32::is_finite) {
        return Err("image quad의 표면 좌표가 유한하지 않습니다".to_owned());
    }
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

#[cfg(test)]
mod tests {
    use super::build_image_geometry;
    use spinon_core::{
        EnvironmentRevision, HostDocument, IntrinsicSizeRevision, NodeId, StyleRevision,
    };
    use spinon_render::{
        CssAxisClip, CssClip, CssRect, CssSize, RuntimeImageObjectStyle, RuntimeImageResourceKey,
        RuntimePaint, RuntimeRenderBox, RuntimeRenderImage, RuntimeRenderKey,
        RuntimeRenderSnapshot,
    };
    use std::sync::Arc;

    fn scene_with_image(
        frame: CssRect,
        fit: &str,
        position: &str,
        clip: CssClip,
    ) -> RuntimeRenderSnapshot {
        let document = HostDocument::new().unwrap().snapshot();
        let key = RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
        .with_intrinsic_size_revision(IntrinsicSizeRevision::INITIAL);
        let source = RuntimeImageResourceKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            IntrinsicSizeRevision::INITIAL,
        );
        let node = NodeId::new(7).unwrap();
        let image = RuntimeRenderImage::new_with_object_style(
            node,
            source,
            24,
            12,
            Arc::from([0_u8; 24 * 12 * 4]),
            RuntimeImageObjectStyle::parse(fit, position, "auto").unwrap(),
            StyleRevision::INITIAL,
        )
        .unwrap();
        RuntimeRenderSnapshot::new(
            key,
            CssSize::new(224.0, 72.0).unwrap(),
            vec![RuntimeRenderBox::new(node, frame, RuntimePaint::None, 0).with_clip(clip)],
        )
        .unwrap()
        .with_images(vec![image])
        .unwrap()
    }

    #[test]
    fn cover_geometry_clips_the_content_box_and_maps_cropped_uvs() {
        let scene = scene_with_image(
            CssRect::new(104.0, 8.0, 24.0, 24.0).unwrap(),
            "cover",
            "50% 50%",
            CssClip::unconstrained(),
        );
        let geometry = build_image_geometry(&scene, 224, 72).unwrap();
        let vertices = &geometry.vertices;
        assert_eq!(vertices.len(), 6);
        assert!((vertices[0].uv[0] - 0.25).abs() < 1e-6);
        assert_eq!(vertices[0].uv[1], 0.0);
        assert!((vertices[2].uv[0] - 0.75).abs() < 1e-6);
        assert_eq!(vertices[2].uv[1], 0.0);
        assert!((vertices[0].position[0] - (-1.0 + 2.0 * 104.0 / 224.0)).abs() < 1e-6);
        assert!((vertices[2].position[0] - (-1.0 + 2.0 * 128.0 / 224.0)).abs() < 1e-6);
    }

    #[test]
    fn page_scale_maps_image_geometry_without_changing_cropped_uvs() {
        let scene = scene_with_image(
            CssRect::new(104.0, 8.0, 24.0, 24.0).unwrap(),
            "cover",
            "50% 50%",
            CssClip::unconstrained(),
        )
        .with_page_scale_transform(3.0, 1.0)
        .unwrap();
        let geometry = build_image_geometry(&scene, 672, 216).unwrap();
        let vertices = &geometry.vertices;
        let to_x_px = |ndc: f32| (ndc + 1.0) * 672.0 / 2.0;
        let to_y_px = |ndc: f32| (1.0 - ndc) * 216.0 / 2.0;
        assert!((to_x_px(vertices[0].position[0]) - 312.0).abs() < 0.001);
        assert!((to_x_px(vertices[2].position[0]) - 384.0).abs() < 0.001);
        assert!((to_y_px(vertices[0].position[1]) - 24.0).abs() < 0.001);
        assert_eq!(vertices[0].uv[0], 0.25);
        assert_eq!(vertices[2].uv[0], 0.75);
    }

    #[test]
    fn accumulated_clip_intersects_frame_and_cropped_object() {
        let clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(110.0, 120.0).unwrap())
            .with_y(CssAxisClip::new(10.0, 20.0).unwrap());
        let scene = scene_with_image(
            CssRect::new(104.0, 8.0, 24.0, 24.0).unwrap(),
            "cover",
            "50% 50%",
            clip,
        );
        let geometry = build_image_geometry(&scene, 224, 72).unwrap();
        assert_eq!(geometry.vertices.len(), 6);
        assert!((geometry.vertices[0].uv[0] - 0.375).abs() < 1e-6);
        assert!((geometry.vertices[0].uv[1] - (2.0 / 24.0)).abs() < 1e-6);
        assert!((geometry.vertices[2].uv[0] - (28.0 / 48.0)).abs() < 1e-6);
        assert!((geometry.vertices[1].uv[1] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn content_clip_can_remove_an_object_outside_its_frame() {
        let scene = scene_with_image(
            CssRect::new(8.0, 40.0, 16.0, 16.0).unwrap(),
            "none",
            "-200% 50%",
            CssClip::unconstrained(),
        );
        let geometry = build_image_geometry(&scene, 224, 72).unwrap();
        assert!(geometry.vertices.is_empty());
        assert_eq!(geometry.range_for_image(0), Some(0..0));
    }
}
