use std::num::NonZeroU64;

use crate::geometry::Vertex;

use super::{FLOAT_FORMAT, GROUP_UNIFORM_SIZE, RESOLVE_UNIFORM_SIZE};

pub(super) fn texture_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

pub(super) fn texture_bind_group_layout(
    device: &wgpu::Device,
    label: &'static str,
    bindings: u32,
) -> wgpu::BindGroupLayout {
    let entries = (0..bindings).map(texture_layout_entry).collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &entries,
    })
}

pub(super) fn group_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("spinon-runtime-precise-group-layout"),
        entries: &[
            texture_layout_entry(0),
            texture_layout_entry(1),
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(GROUP_UNIFORM_SIZE),
                },
                count: None,
            },
        ],
    })
}

pub(super) fn resolve_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("spinon-runtime-precise-resolve-layout"),
        entries: &[
            texture_layout_entry(0),
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(RESOLVE_UNIFORM_SIZE),
                },
                count: None,
            },
        ],
    })
}

pub(super) fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRIBUTES,
    }
}

pub(super) struct PipelineConfig<'a> {
    pub(super) label: &'static str,
    pub(super) source: &'a str,
    pub(super) format: wgpu::TextureFormat,
    pub(super) bind_group_layout: &'a wgpu::BindGroupLayout,
    pub(super) vertex_buffer_layout: Option<wgpu::VertexBufferLayout<'a>>,
    pub(super) vertex_entry: &'static str,
    pub(super) fragment_entry: &'static str,
}

pub(super) fn create_pipeline(
    device: &wgpu::Device,
    config: PipelineConfig<'_>,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(config.label),
        source: wgpu::ShaderSource::Wgsl(config.source.into()),
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(config.label),
        bind_group_layouts: &[Some(config.bind_group_layout)],
        immediate_size: 0,
    });
    let buffers = config
        .vertex_buffer_layout
        .into_iter()
        .map(Some)
        .collect::<Vec<_>>();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(config.label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(config.vertex_entry),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(config.fragment_entry),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

pub(super) fn create_format_probe(device: &wgpu::Device) {
    let _probe = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spinon-runtime-precise-format-probe"),
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: FLOAT_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
}
