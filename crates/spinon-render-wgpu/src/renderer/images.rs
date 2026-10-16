use std::{collections::HashMap, num::NonZeroU64, ops::Range};

use sha2::{Digest, Sha256};
use spinon_render::{RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot};

use crate::geometry::{ImageGeometry, ImageVertex, build_image_geometry};

const IMAGE_SHADER: &str = include_str!("image.wgsl");
const RGBA8_BYTES_PER_PIXEL: u32 = 4;
const ALPHA_OFFSET: usize = 3;
const IMAGE_OUTPUT_ENCODING_BYTES: u64 = 16;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ImageCacheKey {
    source: RuntimeImageResourceKey,
    node_id: u64,
    width: u32,
    height: u32,
    rgba_sha256: [u8; 32],
}

impl ImageCacheKey {
    fn new(image: &RuntimeRenderImage) -> Self {
        Self {
            source: image.source(),
            node_id: image.node_id().get(),
            width: image.width(),
            height: image.height(),
            rgba_sha256: Sha256::digest(image.rgba8()).into(),
        }
    }
}

struct GpuImage {
    _texture: wgpu::Texture,
    _view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

pub(crate) struct PreparedImages {
    geometry: ImageGeometry,
    keys: Vec<ImageCacheKey>,
}

impl PreparedImages {
    pub(crate) fn range(&self, index: usize) -> Option<Range<u32>> {
        self.geometry.range_for_image(index)
    }
}

pub(crate) struct RuntimeImageRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    output_encoding: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    vertex_capacity: u64,
    vertex_count: u32,
    cache: HashMap<ImageCacheKey, GpuImage>,
}

impl RuntimeImageRenderer {
    pub(crate) fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("spinon-runtime-image-bind-group-layout"),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(IMAGE_OUTPUT_ENCODING_BYTES),
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("spinon-runtime-image-pipeline-layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("spinon-runtime-image-shader"),
            source: wgpu::ShaderSource::Wgsl(IMAGE_SHADER.into()),
        });
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("spinon-runtime-image-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_image"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ImageVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRIBUTES,
                })],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_image"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("spinon-runtime-image-nearest-clamp-sampler"),
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
        let output_encoding = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("spinon-runtime-image-output-encoding"),
            size: IMAGE_OUTPUT_ENCODING_BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let encode_for_unorm_target = u32::from(!surface_format.is_srgb());
        let output_encoding_words = [encode_for_unorm_target, 0, 0, 0];
        queue.write_buffer(
            &output_encoding,
            0,
            bytemuck::bytes_of(&output_encoding_words),
        );
        let vertex_capacity = std::mem::size_of::<ImageVertex>() as u64;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("spinon-runtime-image-vertices"),
            size: vertex_capacity,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if let Some(error) = pollster::block_on(validation_scope.pop()) {
            let _ = pollster::block_on(out_of_memory_scope.pop());
            return Err(format!("WGPU image pipeline 검증 실패: {error:?}"));
        }
        if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
            return Err(format!("WGPU image pipeline 할당 실패: {error:?}"));
        }
        Ok(Self {
            pipeline,
            bind_group_layout,
            sampler,
            output_encoding,
            vertex_buffer,
            vertex_capacity,
            vertex_count: 0,
            cache: HashMap::new(),
        })
    }

    pub(crate) fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: Option<&RuntimeRenderSnapshot>,
        surface_width: u32,
        surface_height: u32,
    ) -> Result<PreparedImages, String> {
        let Some(scene) = scene else {
            self.cache.clear();
            self.vertex_count = 0;
            return Ok(PreparedImages {
                geometry: {
                    if surface_width == 0 || surface_height == 0 {
                        return Err("WGPU 표면 크기는 0보다 커야 합니다".to_owned());
                    }
                    ImageGeometry::empty()
                },
                keys: Vec::new(),
            });
        };
        let geometry = build_image_geometry(scene, surface_width, surface_height)?;
        let max_dimension = device.limits().max_texture_dimension_2d;
        let mut keys = Vec::new();
        keys.try_reserve_exact(scene.images().len())
            .map_err(|_| "WGPU image cache key 목록을 할당하지 못했습니다".to_owned())?;
        for image in scene.images() {
            if !image.source().matches(scene.key()) {
                return Err(format!(
                    "runtime image node {}의 source revision이 장면과 다릅니다",
                    image.node_id()
                ));
            }
            if image.width() > max_dimension || image.height() > max_dimension {
                return Err(format!(
                    "runtime image node {}가 device texture dimension 제한을 넘었습니다",
                    image.node_id()
                ));
            }
            if image
                .rgba8()
                .chunks_exact(4)
                .any(|pixel| pixel[ALPHA_OFFSET] != u8::MAX)
            {
                return Err(format!(
                    "runtime image node {}의 alpha는 이 paint profile에서 지원하지 않습니다",
                    image.node_id()
                ));
            }
            let key = ImageCacheKey::new(image);
            if !self.cache.contains_key(&key) {
                let gpu_image = self.create_image(device, queue, image)?;
                self.cache.insert(key.clone(), gpu_image);
            }
            keys.push(key);
        }
        self.cache.retain(|key, _| keys.contains(key));
        self.upload_vertices(device, queue, &geometry.vertices)?;
        Ok(PreparedImages { geometry, keys })
    }

    pub(crate) fn draw_image<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        prepared: &'pass PreparedImages,
        image_index: usize,
    ) -> bool {
        let Some(key) = prepared.keys.get(image_index) else {
            return false;
        };
        let Some(range) = prepared.range(image_index) else {
            return false;
        };
        if range.is_empty() || self.vertex_count == 0 {
            return false;
        }
        let Some(image) = self.cache.get(key) else {
            return false;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &image.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(range, 0..1);
        true
    }

    fn create_image(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &RuntimeRenderImage,
    ) -> Result<GpuImage, String> {
        let expected_bytes = image
            .width()
            .checked_mul(image.height())
            .and_then(|pixels| pixels.checked_mul(RGBA8_BYTES_PER_PIXEL))
            .and_then(|bytes| usize::try_from(bytes).ok());
        if expected_bytes != Some(image.rgba8().len()) {
            return Err(format!(
                "runtime image node {}의 RGBA8 byte length가 dimensions와 다릅니다",
                image.node_id()
            ));
        }
        let bytes_per_row = image
            .width()
            .checked_mul(RGBA8_BYTES_PER_PIXEL)
            .ok_or_else(|| "WGPU image row byte 수가 범위를 넘었습니다".to_owned())?;
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("spinon-runtime-image-rgba8-srgb"),
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("spinon-runtime-image-bind-group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.output_encoding.as_entire_binding(),
                },
            ],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image.rgba8(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(image.height()),
            },
            wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
        );
        if let Some(error) = pollster::block_on(validation_scope.pop()) {
            let _ = pollster::block_on(out_of_memory_scope.pop());
            return Err(format!("WGPU image texture 검증 실패: {error:?}"));
        }
        if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
            return Err(format!("WGPU image texture 할당 실패: {error:?}"));
        }
        Ok(GpuImage {
            _texture: texture,
            _view: view,
            bind_group,
        })
    }

    fn upload_vertices(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[ImageVertex],
    ) -> Result<(), String> {
        self.vertex_count = u32::try_from(vertices.len())
            .map_err(|_| "WGPU image vertex 수가 범위를 넘었습니다".to_owned())?;
        if vertices.is_empty() {
            return Ok(());
        }
        let bytes = bytemuck::cast_slice(vertices);
        let required = u64::try_from(bytes.len())
            .map_err(|_| "WGPU image vertex byte 수가 범위를 넘었습니다".to_owned())?;
        if required > self.vertex_capacity {
            let capacity = required
                .checked_next_power_of_two()
                .ok_or_else(|| "WGPU image vertex 용량 계산이 넘쳤습니다".to_owned())?;
            let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
            let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
            let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("spinon-runtime-image-vertices"),
                size: capacity,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            if let Some(error) = pollster::block_on(validation_scope.pop()) {
                let _ = pollster::block_on(out_of_memory_scope.pop());
                return Err(format!("WGPU image vertex buffer 검증 실패: {error}"));
            }
            if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
                return Err(format!("WGPU image vertex buffer 할당 실패: {error}"));
            }
            self.vertex_buffer = vertex_buffer;
            self.vertex_capacity = capacity;
        }
        queue.write_buffer(&self.vertex_buffer, 0, bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ImageCacheKey;
    use spinon_core::{HostDocument, IntrinsicSizeRevision, NodeId};
    use spinon_render::{RuntimeImageResourceKey, RuntimeRenderImage};
    use std::sync::Arc;

    #[test]
    fn texture_identity_includes_source_node_content_and_dimensions() {
        let document = HostDocument::new().unwrap().snapshot();
        let source = RuntimeImageResourceKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            IntrinsicSizeRevision::INITIAL,
        );
        let image = RuntimeRenderImage::new(
            NodeId::new(1).unwrap(),
            source,
            1,
            1,
            Arc::from([1_u8, 2, 3, 255]),
        )
        .unwrap();
        let base = ImageCacheKey::new(&image);
        let changed_pixels =
            RuntimeRenderImage::new(image.node_id(), source, 1, 1, Arc::from([3_u8, 2, 1, 255]))
                .unwrap();
        assert_ne!(base, ImageCacheKey::new(&changed_pixels));
        let changed_dimensions = RuntimeRenderImage::new(
            image.node_id(),
            source,
            2,
            1,
            Arc::from([1_u8, 2, 3, 255, 4, 5, 6, 255]),
        )
        .unwrap();
        assert_ne!(base, ImageCacheKey::new(&changed_dimensions));
        let other_source = RuntimeImageResourceKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        );
        let changed_source = RuntimeRenderImage::new(
            image.node_id(),
            other_source,
            1,
            1,
            Arc::from([1_u8, 2, 3, 255]),
        )
        .unwrap();
        assert_ne!(base, ImageCacheKey::new(&changed_source));
        let other_generation = HostDocument::new().unwrap().snapshot().generation();
        let changed_generation = RuntimeRenderImage::new(
            image.node_id(),
            RuntimeImageResourceKey::new(
                other_generation,
                document.document_revision(),
                document.render_tree_revision(),
                IntrinsicSizeRevision::INITIAL,
            ),
            1,
            1,
            Arc::from([1_u8, 2, 3, 255]),
        )
        .unwrap();
        assert_ne!(base, ImageCacheKey::new(&changed_generation));
        assert_eq!(
            base.source.document_revision(),
            document.document_revision()
        );
    }
}
