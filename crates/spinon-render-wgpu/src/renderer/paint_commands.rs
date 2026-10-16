use bytemuck::cast_slice;
use spinon_render::{RuntimePaintCommand, RuntimeRenderSnapshot};
use wgpu::util::DeviceExt;

use crate::geometry::ColoredGeometry;

use super::images::{PreparedImages, RuntimeImageRenderer};
use super::precise_compositor::{PreciseCompositor, PreciseSceneDraw};

mod pipeline;

use pipeline::create_group_composite_pipeline;

const MAX_OPACITY_GROUP_DEPTH: usize = 64;
const MAX_OPACITY_LAYER_BYTES: u64 = 256 * 1024 * 1024;

struct OpacityLayer {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    size: (u32, u32),
}

struct BoxDrawResources<'a> {
    scene: &'a RuntimeRenderSnapshot,
    color_pipeline: &'a wgpu::RenderPipeline,
    vertex_buffer: &'a wgpu::Buffer,
    geometry: &'a ColoredGeometry,
    image_renderer: &'a RuntimeImageRenderer,
    prepared_images: &'a PreparedImages,
}

pub(crate) struct OpacityGroupRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
    layers: Vec<OpacityLayer>,
    precise_compositor: Option<PreciseCompositor>,
}

impl OpacityGroupRenderer {
    pub(crate) fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let (pipeline, bind_group_layout) = create_group_composite_pipeline(device, format);
        let precise_compositor = PreciseCompositor::try_new(device, format).ok();
        Self {
            pipeline,
            bind_group_layout,
            format,
            layers: Vec::new(),
            precise_compositor,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_scene(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        scene: &RuntimeRenderSnapshot,
        target: &wgpu::TextureView,
        width: u32,
        height: u32,
        color_pipeline: &wgpu::RenderPipeline,
        vertex_buffer: &wgpu::Buffer,
        geometry: &ColoredGeometry,
        image_renderer: &RuntimeImageRenderer,
        prepared_images: &PreparedImages,
    ) -> Result<usize, String> {
        let precise_draw = PreciseSceneDraw {
            scene,
            target,
            width,
            height,
            vertex_buffer,
            geometry,
        };
        if let Some(precise_compositor) = self
            .precise_compositor
            .as_mut()
            .filter(|compositor| compositor.handles(&precise_draw))
            && let Ok(image_count) = precise_compositor.draw_scene(device, encoder, precise_draw)
        {
            return Ok(image_count);
        }
        let draw_resources = BoxDrawResources {
            scene,
            color_pipeline,
            vertex_buffer,
            geometry,
            image_renderer,
            prepared_images,
        };
        let max_group_depth = maximum_group_depth(scene.paint_commands());
        if max_group_depth > MAX_OPACITY_GROUP_DEPTH {
            return Err(format!(
                "opacity group 깊이가 지원 한도를 넘었습니다: {max_group_depth}"
            ));
        }
        let layer_bytes = u64::from(width)
            .checked_mul(u64::from(height))
            .and_then(|pixels| pixels.checked_mul(4))
            .and_then(|bytes| bytes.checked_mul(max_group_depth as u64))
            .ok_or_else(|| "opacity group layer 메모리 크기가 범위를 넘었습니다".to_owned())?;
        if layer_bytes > MAX_OPACITY_LAYER_BYTES {
            return Err(format!(
                "opacity group layer 메모리가 한도를 넘었습니다: {layer_bytes} bytes"
            ));
        }
        if max_group_depth == 0 {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("spinon-runtime-gpu-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            return Ok(draw_commands_in_pass(&mut pass, &draw_resources));
        }

        self.ensure_layers(device, max_group_depth, width, height)?;
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("spinon-runtime-gpu-root-clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        let mut active_groups = Vec::with_capacity(max_group_depth);
        let mut image_draw_count = 0;
        for command in scene.paint_commands() {
            match *command {
                RuntimePaintCommand::BeginGroup { opacity } => {
                    let depth = active_groups.len();
                    clear_layer(encoder, &self.layers[depth].view);
                    active_groups.push(opacity);
                }
                RuntimePaintCommand::DrawBox { index } => {
                    let destination = active_groups
                        .len()
                        .checked_sub(1)
                        .map_or(target, |depth| &self.layers[depth].view);
                    image_draw_count += draw_box(encoder, destination, index, &draw_resources);
                }
                RuntimePaintCommand::EndGroup => {
                    let Some(opacity) = active_groups.pop() else {
                        unreachable!("validated opacity-group commands must balance")
                    };
                    let depth = active_groups.len();
                    let source = &self.layers[depth].view;
                    let destination = if depth == 0 {
                        target
                    } else {
                        &self.layers[depth - 1].view
                    };
                    composite_layer(
                        device,
                        encoder,
                        &self.pipeline,
                        &self.bind_group_layout,
                        source,
                        destination,
                        opacity,
                    );
                }
            }
        }
        debug_assert!(active_groups.is_empty());
        Ok(image_draw_count)
    }

    fn ensure_layers(
        &mut self,
        device: &wgpu::Device,
        depth: usize,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if self
            .layers
            .iter()
            .any(|layer| layer.size != (width, height))
        {
            self.layers.clear();
        }
        let missing = depth.saturating_sub(self.layers.len());
        if missing == 0 {
            self.layers.truncate(depth);
            return Ok(());
        }
        let mut new_layers = Vec::new();
        new_layers
            .try_reserve(missing)
            .map_err(|_| "opacity group layer 목록을 할당하지 못했습니다".to_owned())?;
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        for _ in 0..missing {
            new_layers.push(create_layer(device, self.format, width, height));
        }
        let validation_error = pollster::block_on(validation_scope.pop());
        let out_of_memory_error = pollster::block_on(out_of_memory_scope.pop());
        if let Some(error) = validation_error {
            return Err(format!("opacity group layer 검증 실패: {error:?}"));
        }
        if let Some(error) = out_of_memory_error {
            return Err(format!("opacity group layer 할당 실패: {error:?}"));
        }
        self.layers.extend(new_layers);
        self.layers.truncate(depth);
        Ok(())
    }
}

fn create_layer(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
) -> OpacityLayer {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spinon-runtime-opacity-group-layer"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    OpacityLayer {
        _texture: texture,
        view,
        size: (width, height),
    }
}

fn maximum_group_depth(commands: &[RuntimePaintCommand]) -> usize {
    let mut depth = 0_usize;
    let mut maximum = 0_usize;
    for command in commands {
        match command {
            RuntimePaintCommand::BeginGroup { .. } => {
                depth += 1;
                maximum = maximum.max(depth);
            }
            RuntimePaintCommand::EndGroup => depth -= 1,
            RuntimePaintCommand::DrawBox { .. } => {}
        }
    }
    maximum
}

fn draw_commands_in_pass<'pass>(
    pass: &mut wgpu::RenderPass<'pass>,
    resources: &'pass BoxDrawResources<'pass>,
) -> usize {
    let mut image_draw_count = 0;
    for command in resources.scene.paint_commands() {
        if let RuntimePaintCommand::DrawBox { index } = *command {
            image_draw_count += draw_box_in_pass(pass, resources, index);
        }
    }
    image_draw_count
}

fn draw_box(
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    index: usize,
    resources: &BoxDrawResources<'_>,
) -> usize {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("spinon-runtime-opacity-group-child-pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    draw_box_in_pass(&mut pass, resources, index)
}

fn draw_box_in_pass<'pass>(
    pass: &mut wgpu::RenderPass<'pass>,
    resources: &'pass BoxDrawResources<'pass>,
    index: usize,
) -> usize {
    if let Some(range) = resources.geometry.range_for_box(index)
        && !range.is_empty()
    {
        pass.set_pipeline(resources.color_pipeline);
        pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
        pass.draw(range, 0..1);
    }
    let Some(render_box) = resources.scene.boxes().get(index) else {
        unreachable!("validated paint command box indices must be valid")
    };
    let Some(image_index) = resources
        .scene
        .images()
        .iter()
        .position(|image| image.node_id() == render_box.node_id())
    else {
        return 0;
    };
    usize::from(
        resources
            .image_renderer
            .draw_image(pass, resources.prepared_images, image_index),
    )
}

fn clear_layer(encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("spinon-runtime-opacity-group-clear"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
}

fn composite_layer(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    pipeline: &wgpu::RenderPipeline,
    bind_group_layout: &wgpu::BindGroupLayout,
    source: &wgpu::TextureView,
    target: &wgpu::TextureView,
    opacity: f32,
) {
    let uniform_values = [opacity, 0.0, 0.0, 0.0];
    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("spinon-runtime-opacity-group-uniform"),
        contents: cast_slice(&uniform_values),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("spinon-runtime-opacity-group-bind-group"),
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(source),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: uniform.as_entire_binding(),
            },
        ],
    });
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("spinon-runtime-opacity-group-composite"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bind_group, &[]);
    pass.draw(0..3, 0..1);
}
