use std::{collections::HashMap, ops::Range, sync::Arc};

use bytemuck::{Pod, Zeroable};
use sha2::{Digest, Sha256};
use spinon_render::{RuntimeRenderSnapshot, RuntimeRenderText};

use crate::geometry::CssToSurfaceMapping;

mod atlas;
use atlas::{GpuGlyphAtlas, build_mask_atlas, create_gpu_atlas};

const GLYPH_SHADER: &str = include_str!("glyphs.wgsl");
pub(super) const MAX_GLYPH_ATLAS_BYTES: usize = 64 * 1024 * 1024;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct GlyphVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

#[derive(Default)]
struct GlyphGeometry {
    vertices: Vec<GlyphVertex>,
    ranges: Vec<Range<u32>>,
    atlases: Vec<Option<Arc<GpuGlyphAtlas>>>,
}

pub(crate) struct PreparedGlyphs {
    ranges: Vec<Range<u32>>,
    atlases: Vec<Option<Arc<GpuGlyphAtlas>>>,
    batch_draw_count: usize,
}

impl PreparedGlyphs {
    pub(crate) fn batch_draw_count(&self) -> usize {
        self.batch_draw_count
    }
}

pub(crate) struct RuntimeGlyphRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: u64,
    vertex_count: u32,
    encode_srgb_color: bool,
    cache: HashMap<[u8; 32], Arc<GpuGlyphAtlas>>,
    cache_bytes: usize,
}

impl RuntimeGlyphRenderer {
    pub(crate) fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("spinon-runtime-glyph-bind-group-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("spinon-runtime-glyph-pipeline-layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("spinon-runtime-glyph-shader"),
            source: wgpu::ShaderSource::Wgsl(GLYPH_SHADER.into()),
        });
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("spinon-runtime-glyph-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_glyph"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GlyphVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRIBUTES,
                })],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_glyph"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("spinon-runtime-glyph-nearest-clamp-sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });
        let vertex_capacity = std::mem::size_of::<GlyphVertex>() as u64;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("spinon-runtime-glyph-vertices"),
            size: vertex_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if let Some(error) = pollster::block_on(validation_scope.pop()) {
            let _ = pollster::block_on(out_of_memory_scope.pop());
            return Err(format!("WGPU glyph pipeline 검증 실패: {error:?}"));
        }
        if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
            return Err(format!("WGPU glyph pipeline 할당 실패: {error:?}"));
        }
        Ok(Self {
            pipeline,
            bind_group_layout,
            sampler,
            vertex_buffer,
            vertex_capacity,
            vertex_count: 0,
            encode_srgb_color: surface_format.is_srgb(),
            cache: HashMap::new(),
            cache_bytes: 0,
        })
    }

    pub(crate) fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: Option<&RuntimeRenderSnapshot>,
        surface_width: u32,
        surface_height: u32,
    ) -> Result<PreparedGlyphs, String> {
        if surface_width == 0 || surface_height == 0 {
            return Err("WGPU glyph 표면 크기는 0보다 커야 합니다".to_owned());
        }
        let Some(scene) = scene else {
            self.vertex_count = 0;
            return Ok(PreparedGlyphs {
                ranges: Vec::new(),
                atlases: Vec::new(),
                batch_draw_count: 0,
            });
        };
        let mapping = CssToSurfaceMapping::new(scene, surface_width, surface_height);
        let mut geometry = GlyphGeometry::default();
        geometry
            .vertices
            .try_reserve(scene.texts().len().saturating_mul(6))
            .map_err(|_| "WGPU glyph 정점 목록을 할당하지 못했습니다".to_owned())?;
        geometry
            .ranges
            .try_reserve_exact(scene.texts().len())
            .map_err(|_| "WGPU glyph draw 범위를 할당하지 못했습니다".to_owned())?;
        geometry
            .atlases
            .try_reserve_exact(scene.texts().len())
            .map_err(|_| "WGPU glyph atlas 목록을 할당하지 못했습니다".to_owned())?;

        let max_dimension = device.limits().max_texture_dimension_2d;
        for text in scene.texts() {
            let first_vertex = u32::try_from(geometry.vertices.len())
                .map_err(|_| "WGPU glyph vertex index가 범위를 넘었습니다".to_owned())?;
            let atlas_data = build_mask_atlas(text, max_dimension)?;
            let Some(atlas_data) = atlas_data else {
                geometry.atlases.push(None);
                geometry.ranges.push(first_vertex..first_vertex);
                continue;
            };
            let mut hasher = Sha256::new();
            hasher.update(atlas_data.width.to_le_bytes());
            hasher.update(atlas_data.height.to_le_bytes());
            hasher.update(&atlas_data.pixels);
            let atlas_key: [u8; 32] = hasher.finalize().into();
            let atlas = if let Some(atlas) = self.cache.get(&atlas_key) {
                Arc::clone(atlas)
            } else {
                let atlas = Arc::new(create_gpu_atlas(
                    device,
                    queue,
                    &self.bind_group_layout,
                    &self.sampler,
                    &atlas_data,
                )?);
                if self
                    .cache_bytes
                    .checked_add(atlas.byte_len)
                    .is_none_or(|bytes| bytes > MAX_GLYPH_ATLAS_BYTES)
                {
                    self.cache.clear();
                    self.cache_bytes = 0;
                }
                self.cache_bytes = self
                    .cache_bytes
                    .checked_add(atlas.byte_len)
                    .ok_or_else(|| "WGPU glyph atlas cache 크기가 넘쳤습니다".to_owned())?;
                self.cache.insert(atlas_key, Arc::clone(&atlas));
                atlas
            };
            append_text_vertices(
                &mut geometry.vertices,
                text,
                &atlas_data.placements,
                atlas_data.width,
                atlas_data.height,
                mapping,
                self.encode_srgb_color,
            )?;
            let last_vertex = u32::try_from(geometry.vertices.len())
                .map_err(|_| "WGPU glyph vertex index가 범위를 넘었습니다".to_owned())?;
            geometry.ranges.push(first_vertex..last_vertex);
            geometry.atlases.push(Some(atlas));
        }
        self.upload_vertices(device, queue, &geometry.vertices)?;
        let batch_draw_count = geometry
            .ranges
            .iter()
            .filter(|range| !range.is_empty())
            .count();
        Ok(PreparedGlyphs {
            ranges: geometry.ranges,
            atlases: geometry.atlases,
            batch_draw_count,
        })
    }

    fn upload_vertices(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[GlyphVertex],
    ) -> Result<(), String> {
        self.vertex_count = u32::try_from(vertices.len())
            .map_err(|_| "WGPU glyph 정점 수가 범위를 넘었습니다".to_owned())?;
        if vertices.is_empty() {
            return Ok(());
        }
        let bytes = bytemuck::cast_slice(vertices);
        let required = u64::try_from(bytes.len())
            .map_err(|_| "WGPU glyph 정점 byte 수가 범위를 넘었습니다".to_owned())?;
        if required > self.vertex_capacity {
            let capacity = required
                .checked_next_power_of_two()
                .ok_or_else(|| "WGPU glyph 정점 buffer 용량이 넘쳤습니다".to_owned())?;
            let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
            let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
            let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("spinon-runtime-glyph-vertices"),
                size: capacity,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            if let Some(error) = pollster::block_on(validation_scope.pop()) {
                let _ = pollster::block_on(out_of_memory_scope.pop());
                return Err(format!("WGPU glyph vertex buffer 검증 실패: {error}"));
            }
            if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
                return Err(format!("WGPU glyph vertex buffer 할당 실패: {error}"));
            }
            self.vertex_buffer = vertex_buffer;
            self.vertex_capacity = capacity;
        }
        queue.write_buffer(&self.vertex_buffer, 0, bytes);
        Ok(())
    }

    pub(crate) fn draw_text<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        prepared: &'pass PreparedGlyphs,
        index: usize,
    ) {
        let Some(range) = prepared.ranges.get(index).filter(|range| !range.is_empty()) else {
            return;
        };
        let Some(Some(atlas)) = prepared.atlases.get(index) else {
            return;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &atlas.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(range.clone(), 0..1);
    }
}

fn append_text_vertices(
    vertices: &mut Vec<GlyphVertex>,
    text: &RuntimeRenderText,
    placements: &[(u32, u32)],
    atlas_width: u32,
    atlas_height: u32,
    mapping: CssToSurfaceMapping,
    encode_srgb_color: bool,
) -> Result<(), String> {
    let scale = f64::from(text.device_scale_factor());
    let clip = text.clip_css_px();
    let encode = |channel| {
        if encode_srgb_color {
            srgb_to_linear(channel)
        } else {
            channel
        }
    };
    vertices
        .try_reserve(text.glyphs().len().saturating_mul(6))
        .map_err(|_| "WGPU glyph 정점 목록을 늘리지 못했습니다".to_owned())?;
    for (glyph_index, (glyph, (atlas_x, atlas_y))) in
        text.glyphs().iter().zip(placements).enumerate()
    {
        let left_px = i64::from(glyph.origin_x_px()) + i64::from(glyph.left_px());
        let top_px = i64::from(glyph.baseline_y_px()) + i64::from(glyph.top_px());
        let left = f64::from(text.origin_x_css_px()) + left_px as f64 / scale;
        let top = f64::from(text.origin_y_css_px()) + top_px as f64 / scale;
        let width = f64::from(glyph.width_px()) / scale;
        let height = f64::from(glyph.height_px()) / scale;
        let right = left + width;
        let bottom = top + height;
        if ![left, top, right, bottom].into_iter().all(f64::is_finite)
            || width <= 0.0
            || height <= 0.0
        {
            return Err(format!(
                "runtime glyph {}의 CSS 좌표가 유효하지 않습니다",
                text.node_id()
            ));
        }
        let visible_left = clip
            .x()
            .map_or(left, |axis| left.max(f64::from(axis.min())))
            .max(0.0);
        let visible_right = clip
            .x()
            .map_or(right, |axis| right.min(f64::from(axis.max())))
            .min(mapping.visible_width_css());
        let visible_top = clip
            .y()
            .map_or(top, |axis| top.max(f64::from(axis.min())))
            .max(0.0);
        let visible_bottom = clip
            .y()
            .map_or(bottom, |axis| bottom.min(f64::from(axis.max())))
            .min(mapping.visible_height_css());
        if visible_right <= visible_left || visible_bottom <= visible_top {
            continue;
        }
        let glyph_color = text
            .glyph_colors()
            .and_then(|colors| colors.get(glyph_index).copied())
            .unwrap_or_else(|| text.color());
        let color = [
            encode(glyph_color.red()),
            encode(glyph_color.green()),
            encode(glyph_color.blue()),
            glyph_color.alpha(),
        ];
        let x0 = mapping.x_to_ndc_f64(visible_left)?;
        let x1 = mapping.x_to_ndc_f64(visible_right)?;
        let y0 = mapping.y_to_ndc_f64(visible_top)?;
        let y1 = mapping.y_to_ndc_f64(visible_bottom)?;
        let u0 = (f64::from(*atlas_x) + (visible_left - left) * scale) / f64::from(atlas_width);
        let u1 = (f64::from(*atlas_x) + (visible_right - left) * scale) / f64::from(atlas_width);
        let v0 = (f64::from(*atlas_y) + (visible_top - top) * scale) / f64::from(atlas_height);
        let v1 = (f64::from(*atlas_y) + (visible_bottom - top) * scale) / f64::from(atlas_height);
        let top_left = GlyphVertex {
            position: [x0, y0],
            uv: [u0 as f32, v0 as f32],
            color,
        };
        let top_right = GlyphVertex {
            position: [x1, y0],
            uv: [u1 as f32, v0 as f32],
            color,
        };
        let bottom_left = GlyphVertex {
            position: [x0, y1],
            uv: [u0 as f32, v1 as f32],
            color,
        };
        let bottom_right = GlyphVertex {
            position: [x1, y1],
            uv: [u1 as f32, v1 as f32],
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
#[path = "glyphs_tests.rs"]
mod tests;
