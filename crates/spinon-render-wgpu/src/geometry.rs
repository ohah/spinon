use bytemuck::{Pod, Zeroable};
use spinon_render::{CssClip, CssSize, RuntimePaint, RuntimeRenderSnapshot};
use std::ops::Range;

mod image;
pub(crate) use image::{ImageGeometry, ImageVertex, build_image_geometry};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub(crate) struct ColoredGeometry {
    pub(crate) vertices: Vec<Vertex>,
    ranges: Vec<Range<u32>>,
}

impl ColoredGeometry {
    pub(crate) fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            ranges: Vec::new(),
        }
    }

    pub(crate) fn range_for_box(&self, index: usize) -> Option<Range<u32>> {
        self.ranges.get(index).cloned()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ColorEncoding {
    Linear,
    Srgb,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CssToSurfaceMapping {
    viewport: CssSize,
    surface_width: u32,
    surface_height: u32,
    physical_scale: Option<f32>,
}

impl CssToSurfaceMapping {
    pub(crate) fn new(
        scene: &RuntimeRenderSnapshot,
        surface_width: u32,
        surface_height: u32,
    ) -> Self {
        Self {
            viewport: scene.viewport_css_px(),
            surface_width,
            surface_height,
            physical_scale: scene
                .page_scale_transform()
                .map(|transform| transform.physical_scale()),
        }
    }

    pub(crate) fn visible_width_css(self) -> f64 {
        self.physical_scale.map_or_else(
            || f64::from(self.viewport.width()),
            |scale| f64::from(self.surface_width) / f64::from(scale),
        )
    }

    pub(crate) fn visible_height_css(self) -> f64 {
        self.physical_scale.map_or_else(
            || f64::from(self.viewport.height()),
            |scale| f64::from(self.surface_height) / f64::from(scale),
        )
    }

    pub(crate) fn x_to_ndc(self, css: f32) -> Result<f32, String> {
        let ndc = if let Some(scale) = self.physical_scale {
            (-1.0 + 2.0 * f64::from(css) * f64::from(scale) / f64::from(self.surface_width)) as f32
        } else {
            -1.0 + 2.0 * css / self.viewport.width()
        };
        if ndc.is_finite() {
            Ok(ndc)
        } else {
            Err("CSS x 좌표를 유한한 WGPU 표면 좌표로 변환하지 못했습니다".to_owned())
        }
    }

    pub(crate) fn y_to_ndc(self, css: f32) -> Result<f32, String> {
        let ndc = if let Some(scale) = self.physical_scale {
            (1.0 - 2.0 * f64::from(css) * f64::from(scale) / f64::from(self.surface_height)) as f32
        } else {
            1.0 - 2.0 * css / self.viewport.height()
        };
        if ndc.is_finite() {
            Ok(ndc)
        } else {
            Err("CSS y 좌표를 유한한 WGPU 표면 좌표로 변환하지 못했습니다".to_owned())
        }
    }

    pub(crate) fn x_to_ndc_f64(self, css: f64) -> Result<f32, String> {
        let ndc = if let Some(scale) = self.physical_scale {
            -1.0 + 2.0 * css * f64::from(scale) / f64::from(self.surface_width)
        } else {
            let surface_px = css * f64::from(self.surface_width) / f64::from(self.viewport.width());
            -1.0 + 2.0 * surface_px / f64::from(self.surface_width)
        };
        let ndc = ndc as f32;
        if ndc.is_finite() {
            Ok(ndc)
        } else {
            Err("glyph x 좌표를 유한한 WGPU 표면 좌표로 변환하지 못했습니다".to_owned())
        }
    }

    pub(crate) fn y_to_ndc_f64(self, css: f64) -> Result<f32, String> {
        let ndc = if let Some(scale) = self.physical_scale {
            1.0 - 2.0 * css * f64::from(scale) / f64::from(self.surface_height)
        } else {
            let surface_px =
                css * f64::from(self.surface_height) / f64::from(self.viewport.height());
            1.0 - 2.0 * surface_px / f64::from(self.surface_height)
        };
        let ndc = ndc as f32;
        if ndc.is_finite() {
            Ok(ndc)
        } else {
            Err("glyph y 좌표를 유한한 WGPU 표면 좌표로 변환하지 못했습니다".to_owned())
        }
    }
}

#[cfg(test)]
pub(crate) fn build_vertices(
    scene: &RuntimeRenderSnapshot,
    width: u32,
    height: u32,
    color_encoding: ColorEncoding,
) -> Result<Vec<Vertex>, String> {
    Ok(build_colored_geometry(scene, width, height, color_encoding)?.vertices)
}

pub(crate) fn build_colored_geometry(
    scene: &RuntimeRenderSnapshot,
    width: u32,
    height: u32,
    color_encoding: ColorEncoding,
) -> Result<ColoredGeometry, String> {
    if width == 0 || height == 0 {
        return Err("WGPU 표면 크기는 0보다 커야 합니다".to_owned());
    }
    let mapping = CssToSurfaceMapping::new(scene, width, height);
    let quad_capacity = scene.boxes().iter().fold(0_usize, |count, render_box| {
        let border = render_box.border();
        let sides = [border.top(), border.right(), border.bottom(), border.left()];
        count
            .saturating_add(usize::from(matches!(
                render_box.paint(),
                RuntimePaint::Opaque(_) | RuntimePaint::Color(_)
            )))
            .saturating_add(
                sides
                    .iter()
                    .filter(|side| {
                        side.width_css_px() > 0.0
                            && matches!(
                                side.paint(),
                                RuntimePaint::Opaque(_) | RuntimePaint::Color(_)
                            )
                    })
                    .count(),
            )
    });
    let mut geometry = ColoredGeometry {
        vertices: Vec::new(),
        ranges: Vec::new(),
    };
    geometry
        .vertices
        .try_reserve(quad_capacity.saturating_mul(6))
        .map_err(|_| "WGPU geometry 버퍼를 할당하지 못했습니다".to_owned())?;
    geometry
        .ranges
        .try_reserve(scene.boxes().len())
        .map_err(|_| "WGPU geometry draw 범위를 할당하지 못했습니다".to_owned())?;

    for render_box in scene.boxes() {
        let first = u32::try_from(geometry.vertices.len())
            .map_err(|_| "WGPU 정점 index가 범위를 넘었습니다".to_owned())?;
        let rect = render_box.frame_css_px();
        let clip = render_box.clip_css_px();
        let x = rect.x();
        let y = rect.y();
        let right = x + rect.width();
        let bottom = y + rect.height();
        append_colored_rect(
            &mut geometry.vertices,
            [x, y, right, bottom],
            render_box.paint(),
            clip,
            mapping,
            color_encoding,
        )?;

        let border = render_box.border();
        let top_width = border.top().width_css_px().min(rect.height());
        let bottom_width = border
            .bottom()
            .width_css_px()
            .min(rect.height() - top_width);
        let left_width = border.left().width_css_px().min(rect.width());
        let right_width = border.right().width_css_px().min(rect.width() - left_width);
        append_colored_rect(
            &mut geometry.vertices,
            [x, y, right, y + top_width],
            border.top().paint(),
            clip,
            mapping,
            color_encoding,
        )?;
        append_colored_rect(
            &mut geometry.vertices,
            [
                right - right_width,
                y + top_width,
                right,
                bottom - bottom_width,
            ],
            border.right().paint(),
            clip,
            mapping,
            color_encoding,
        )?;
        append_colored_rect(
            &mut geometry.vertices,
            [x, bottom - bottom_width, right, bottom],
            border.bottom().paint(),
            clip,
            mapping,
            color_encoding,
        )?;
        append_colored_rect(
            &mut geometry.vertices,
            [x, y + top_width, x + left_width, bottom - bottom_width],
            border.left().paint(),
            clip,
            mapping,
            color_encoding,
        )?;
        let last = u32::try_from(geometry.vertices.len())
            .map_err(|_| "WGPU 정점 index가 범위를 넘었습니다".to_owned())?;
        geometry.ranges.push(first..last);
    }
    Ok(geometry)
}

fn append_colored_rect(
    vertices: &mut Vec<Vertex>,
    rect: [f32; 4],
    paint: RuntimePaint,
    clip: CssClip,
    mapping: CssToSurfaceMapping,
    color_encoding: ColorEncoding,
) -> Result<(), String> {
    let (red, green, blue, alpha) = match paint {
        RuntimePaint::None => return Ok(()),
        RuntimePaint::Opaque(color) => (
            f32::from(color.red()) / 255.0,
            f32::from(color.green()) / 255.0,
            f32::from(color.blue()) / 255.0,
            1.0,
        ),
        RuntimePaint::Color(color) => (color.red(), color.green(), color.blue(), color.alpha()),
    };
    let [x, y, right, bottom] = rect;
    let x0_css = clip.x().map_or(x, |axis| x.max(axis.min()));
    let x1_css = clip.x().map_or(right, |axis| right.min(axis.max()));
    let y0_css = clip.y().map_or(y, |axis| y.max(axis.min()));
    let y1_css = clip.y().map_or(bottom, |axis| bottom.min(axis.max()));
    if x1_css <= x0_css || y1_css <= y0_css {
        return Ok(());
    }
    let x0 = mapping.x_to_ndc(x0_css)?;
    let x1 = mapping.x_to_ndc(x1_css)?;
    let y0 = mapping.y_to_ndc(y0_css)?;
    let y1 = mapping.y_to_ndc(y1_css)?;
    let encode: fn(f32) -> f32 = match color_encoding {
        ColorEncoding::Linear => srgb_to_linear,
        ColorEncoding::Srgb => |channel| channel,
    };
    let color = [encode(red), encode(green), encode(blue), alpha];
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
    Ok(())
}

fn srgb_to_linear(encoded: f32) -> f32 {
    if encoded <= 0.04045 {
        encoded / 12.92
    } else {
        ((encoded + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use spinon_core::{
        EnvironmentRevision, HostDocument, IntrinsicSizeRevision, NodeId, StyleRevision,
    };
    use spinon_render::{
        CssAxisClip, CssClip, CssRect, CssSize, RuntimeBorder, RuntimeBorderSide,
        RuntimeImageResourceKey, RuntimePaint, RuntimeRenderBox, RuntimeRenderImage,
        RuntimeRenderKey, RuntimeSrgbColor,
    };

    use super::{ColorEncoding, build_image_geometry, build_vertices};
    use std::sync::Arc;

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
    fn page_scale_geometry_matches_chrome_fixed_width_probe_pixels() {
        let document = HostDocument::new().unwrap().snapshot();
        let scene = spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                document.generation(),
                document.document_revision(),
                document.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(500.0, 1082.0513).unwrap(),
            vec![RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(100.0, 50.0, 20.0, 10.0).unwrap(),
                RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(255, 0, 0)),
                0,
            )],
        )
        .unwrap()
        .with_page_scale_transform(3.0, 1.0)
        .unwrap();
        let vertices = build_vertices(&scene, 1170, 2532, ColorEncoding::Srgb).unwrap();
        let to_x_px = |ndc: f32| f64::from((ndc + 1.0) * 1170.0 / 2.0);
        let to_y_px = |ndc: f32| f64::from((1.0 - ndc) * 2532.0 / 2.0);
        assert!((to_x_px(vertices[0].position[0]) - 300.0).abs() < 0.001);
        assert!((to_x_px(vertices[2].position[0]) - 360.0).abs() < 0.001);
        assert!((to_y_px(vertices[0].position[1]) - 150.0).abs() < 0.001);
        assert!((to_y_px(vertices[1].position[1]) - 180.0).abs() < 0.001);
        assert_eq!(scene.viewport_css_px().width(), 500.0);
        assert_eq!(
            scene.hit_test_css_point(100.0, 50.0).unwrap().node_id(),
            NodeId::new(1).unwrap()
        );
    }

    #[test]
    fn page_scale_geometry_matches_chrome_half_scale_probe_pixels() {
        let document = HostDocument::new().unwrap().snapshot();
        let scene = spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                document.generation(),
                document.document_revision(),
                document.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(780.0, 1688.0).unwrap(),
            vec![RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(100.0, 50.0, 20.0, 10.0).unwrap(),
                RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(255, 0, 0)),
                0,
            )],
        )
        .unwrap()
        .with_page_scale_transform(3.0, 0.5)
        .unwrap();
        let vertices = build_vertices(&scene, 1170, 2532, ColorEncoding::Srgb).unwrap();
        let to_x_px = |ndc: f32| f64::from((ndc + 1.0) * 1170.0 / 2.0);
        let to_y_px = |ndc: f32| f64::from((1.0 - ndc) * 2532.0 / 2.0);
        assert!((to_x_px(vertices[0].position[0]) - 150.0).abs() < 0.001);
        assert!((to_x_px(vertices[2].position[0]) - 180.0).abs() < 0.001);
        assert!((to_y_px(vertices[0].position[1]) - 75.0).abs() < 0.001);
        assert!((to_y_px(vertices[1].position[1]) - 90.0).abs() < 0.001);
    }

    fn clipped_image_scene(clip: CssClip) -> spinon_render::RuntimeRenderSnapshot {
        let document = HostDocument::new().unwrap();
        let source_snapshot = document.snapshot();
        let intrinsic_revision = IntrinsicSizeRevision::INITIAL.checked_next().unwrap();
        let key = RuntimeRenderKey::new(
            source_snapshot.generation(),
            source_snapshot.document_revision(),
            source_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
        .with_intrinsic_size_revision(intrinsic_revision);
        let image_node = NodeId::new(2).unwrap();
        let image_source = RuntimeImageResourceKey::new(
            source_snapshot.generation(),
            source_snapshot.document_revision(),
            source_snapshot.render_tree_revision(),
            intrinsic_revision,
        );
        spinon_render::RuntimeRenderSnapshot::new(
            key,
            CssSize::new(80.0, 40.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(0.0, 0.0, 80.0, 40.0).unwrap(),
                    RuntimePaint::None,
                    0,
                ),
                RuntimeRenderBox::new(
                    image_node,
                    CssRect::new(45.0, 9.0, 24.0, 16.0).unwrap(),
                    RuntimePaint::None,
                    1,
                )
                .with_clip(clip),
            ],
        )
        .unwrap()
        .with_images(vec![
            RuntimeRenderImage::new(
                image_node,
                image_source,
                24,
                16,
                Arc::from(vec![255_u8; 24 * 16 * 4]),
            )
            .unwrap(),
        ])
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
    fn geometry_preserves_css_paint_alpha_and_rejects_invalid_channels() {
        let document = HostDocument::new().unwrap();
        let source = document.snapshot();
        let color = RuntimeSrgbColor::new(128.0 / 255.0, 64.0 / 255.0, 32.0 / 255.0, 0.5).unwrap();
        let scene = spinon_render::RuntimeRenderSnapshot::new(
            RuntimeRenderKey::new(
                source.generation(),
                source.document_revision(),
                source.render_tree_revision(),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            CssSize::new(20.0, 10.0).unwrap(),
            vec![RuntimeRenderBox::new(
                NodeId::new(3).unwrap(),
                CssRect::new(0.0, 0.0, 10.0, 10.0).unwrap(),
                RuntimePaint::Color(color),
                0,
            )],
        )
        .unwrap();
        let vertices = build_vertices(&scene, 20, 10, ColorEncoding::Srgb).unwrap();
        assert_eq!(
            vertices[0].color,
            [128.0 / 255.0, 64.0 / 255.0, 32.0 / 255.0, 0.5]
        );
        assert!(RuntimeSrgbColor::new(f32::NAN, 0.0, 0.0, 1.0).is_err());
        assert!(RuntimeSrgbColor::new(0.0, 0.0, 1.01, 1.0).is_err());
        assert!(RuntimeSrgbColor::new(0.0, 0.0, 0.0, -0.01).is_err());
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

    #[test]
    fn image_clip_adjusts_uv_with_destination_and_preserves_top_row() {
        let scene = clipped_image_scene(
            CssClip::unconstrained().with_x(CssAxisClip::new(45.0, 61.0).unwrap()),
        );
        let geometry = build_image_geometry(&scene, 80, 40).unwrap();
        assert_eq!(geometry.vertices.len(), 6);
        assert_eq!(geometry.range_for_image(0), Some(0..6));
        assert_eq!(geometry.vertices[0].uv, [0.0, 0.0]);
        assert!((geometry.vertices[2].uv[0] - (16.0 / 24.0)).abs() < 1.0e-6);
        assert_eq!(geometry.vertices[2].uv[1], 0.0);
        assert!((geometry.vertices[5].uv[0] - (16.0 / 24.0)).abs() < 1.0e-6);
        assert_eq!(geometry.vertices[5].uv[1], 1.0);
        assert_eq!(geometry.vertices[0].position[1], 1.0 - 2.0 * 9.0 / 40.0);
    }

    #[test]
    fn fully_clipped_image_has_an_empty_draw_range() {
        let scene = clipped_image_scene(
            CssClip::unconstrained().with_x(CssAxisClip::new(80.0, 90.0).unwrap()),
        );
        let geometry = build_image_geometry(&scene, 80, 40).unwrap();
        assert!(geometry.vertices.is_empty());
        assert_eq!(geometry.range_for_image(0), Some(0..0));
    }
}
