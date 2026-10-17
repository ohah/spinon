use bytemuck::cast_slice;
use spinon_render::{RuntimePaintCommand, RuntimeRenderSnapshot};
use wgpu::util::DeviceExt;

use crate::geometry::ColoredGeometry;

mod layer;
mod limits;
mod pipeline;
#[cfg(test)]
mod tests;

use layer::{AccumulationLayer, clear_layer, copy_to_next};
use limits::{
    MAX_LAYER_BYTES, maximum_group_depth, needs_precise_composition, required_layer_bytes,
    supports_surface_format, validate_group_depth, validate_layer_extent,
};
use pipeline::{
    PipelineConfig, create_format_probe, create_pipeline, group_bind_group_layout,
    resolve_bind_group_layout, texture_bind_group_layout, vertex_buffer_layout,
};

const COLOR_SHADER: &str = include_str!("precise_color_over.wgsl");
const GROUP_SHADER: &str = include_str!("precise_group_over.wgsl");
const RESOLVE_SHADER: &str = include_str!("precise_resolve.wgsl");
const FLOAT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Float;
const GROUP_UNIFORM_SIZE: u64 = 16;
const RESOLVE_UNIFORM_SIZE: u64 = 16;

pub(crate) struct PreciseSceneDraw<'a> {
    pub(crate) scene: &'a RuntimeRenderSnapshot,
    pub(crate) target: &'a wgpu::TextureView,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) vertex_buffer: &'a wgpu::Buffer,
    pub(crate) geometry: &'a ColoredGeometry,
}

pub(crate) struct PreciseCompositor {
    surface_format: wgpu::TextureFormat,
    max_texture_dimension_2d: u32,
    color_pipeline: wgpu::RenderPipeline,
    color_bind_group_layout: wgpu::BindGroupLayout,
    group_pipeline: wgpu::RenderPipeline,
    group_bind_group_layout: wgpu::BindGroupLayout,
    resolve_pipeline: wgpu::RenderPipeline,
    resolve_bind_group_layout: wgpu::BindGroupLayout,
    root: Option<AccumulationLayer>,
    groups: Vec<AccumulationLayer>,
}

impl PreciseCompositor {
    pub(crate) fn try_new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Result<Self, String> {
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let color_bind_group_layout =
            texture_bind_group_layout(device, "spinon-runtime-precise-color-layout", 1);
        let color_pipeline = create_pipeline(
            device,
            PipelineConfig {
                label: "spinon-runtime-precise-color-pipeline",
                source: COLOR_SHADER,
                format: FLOAT_FORMAT,
                bind_group_layout: &color_bind_group_layout,
                vertex_buffer_layout: Some(vertex_buffer_layout()),
                vertex_entry: "vs_main",
                fragment_entry: "fs_main",
            },
        );
        let group_bind_group_layout = group_bind_group_layout(device);
        let group_pipeline = create_pipeline(
            device,
            PipelineConfig {
                label: "spinon-runtime-precise-group-pipeline",
                source: GROUP_SHADER,
                format: FLOAT_FORMAT,
                bind_group_layout: &group_bind_group_layout,
                vertex_buffer_layout: None,
                vertex_entry: "vs_main",
                fragment_entry: "fs_main",
            },
        );

        let resolve_bind_group_layout = resolve_bind_group_layout(device);
        let resolve_pipeline = create_pipeline(
            device,
            PipelineConfig {
                label: "spinon-runtime-precise-resolve-pipeline",
                source: RESOLVE_SHADER,
                format: surface_format,
                bind_group_layout: &resolve_bind_group_layout,
                vertex_buffer_layout: None,
                vertex_entry: "vs_main",
                fragment_entry: "fs_main",
            },
        );
        create_format_probe(device);
        if let Some(error) = pollster::block_on(validation_scope.pop()) {
            let _ = pollster::block_on(out_of_memory_scope.pop());
            return Err(format!("정밀 WGPU accumulator 검증 실패: {error:?}"));
        }
        if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
            return Err(format!("정밀 WGPU accumulator 할당 실패: {error:?}"));
        }
        Ok(Self {
            surface_format,
            max_texture_dimension_2d: device.limits().max_texture_dimension_2d,
            color_pipeline,
            color_bind_group_layout,
            group_pipeline,
            group_bind_group_layout,
            resolve_pipeline,
            resolve_bind_group_layout,
            root: None,
            groups: Vec::new(),
        })
    }

    pub(crate) fn draw_scene(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        draw: PreciseSceneDraw<'_>,
    ) -> Result<usize, String> {
        let PreciseSceneDraw {
            scene,
            target,
            width,
            height,
            vertex_buffer,
            geometry,
        } = draw;
        if !scene.images().is_empty() {
            return Err(
                "정밀 opacity compositor는 runtime image scene을 아직 지원하지 않습니다".to_owned(),
            );
        }
        let max_group_depth = maximum_group_depth(scene.paint_commands());
        validate_group_depth(max_group_depth)?;
        validate_layer_extent(width, height, self.max_texture_dimension_2d)?;
        let layer_bytes = required_layer_bytes(width, height, max_group_depth)
            .ok_or_else(|| "정밀 WGPU layer 메모리 크기가 범위를 넘었습니다".to_owned())?;
        if layer_bytes > MAX_LAYER_BYTES {
            return Err(format!(
                "정밀 WGPU layer 메모리가 한도를 넘었습니다: {layer_bytes} bytes"
            ));
        }
        self.ensure_layers(device, max_group_depth, width, height)?;
        let root = self
            .root
            .as_mut()
            .expect("root accumulator was initialized");
        root.current = 0;
        clear_layer(encoder, root.current_view(), wgpu::Color::BLACK);

        let mut active_groups = Vec::with_capacity(max_group_depth);
        for command in scene.paint_commands() {
            match *command {
                RuntimePaintCommand::BeginGroup { opacity } => {
                    let depth = active_groups.len();
                    let layer = &mut self.groups[depth];
                    layer.current = 0;
                    clear_layer(
                        encoder,
                        layer.current_view(),
                        wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        },
                    );
                    active_groups.push(opacity);
                }
                RuntimePaintCommand::DrawBox { index } => {
                    self.draw_box(
                        device,
                        encoder,
                        index,
                        vertex_buffer,
                        geometry,
                        active_groups.len(),
                    );
                }
                RuntimePaintCommand::DrawText { .. } => {
                    unreachable!("text scenes are excluded from the precise compositor fast path")
                }
                RuntimePaintCommand::EndGroup => {
                    let Some(opacity) = active_groups.pop() else {
                        unreachable!("validated opacity-group commands must balance")
                    };
                    let depth = active_groups.len();
                    let (source, destination): (&AccumulationLayer, &mut AccumulationLayer) =
                        if depth == 0 {
                            (
                                &self.groups[depth],
                                self.root
                                    .as_mut()
                                    .expect("root accumulator was initialized"),
                            )
                        } else {
                            let (parents, child) = self.groups.split_at_mut(depth);
                            (&child[0], &mut parents[depth - 1])
                        };
                    composite_group(
                        device,
                        encoder,
                        &self.group_pipeline,
                        &self.group_bind_group_layout,
                        source.current_view(),
                        destination,
                        opacity,
                    );
                }
            }
        }
        debug_assert!(active_groups.is_empty());
        self.resolve(device, encoder, target);
        Ok(0)
    }

    fn draw_box(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        index: usize,
        vertex_buffer: &wgpu::Buffer,
        geometry: &ColoredGeometry,
        group_depth: usize,
    ) {
        let Some(range) = geometry
            .range_for_box(index)
            .filter(|range| !range.is_empty())
        else {
            return;
        };
        let layer = if group_depth == 0 {
            self.root
                .as_mut()
                .expect("root accumulator was initialized")
        } else {
            &mut self.groups[group_depth - 1]
        };
        copy_to_next(encoder, layer);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("spinon-runtime-precise-color-bind-group"),
            layout: &self.color_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(layer.current_view()),
            }],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("spinon-runtime-precise-color-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: layer.next_view(),
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
        pass.set_pipeline(&self.color_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.draw(range, 0..1);
        drop(pass);
        layer.advance();
    }

    fn resolve(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        let root = self
            .root
            .as_ref()
            .expect("root accumulator was initialized");
        let parameters = [u32::from(self.surface_format.is_srgb()), 0, 0, 0];
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("spinon-runtime-precise-resolve-parameters"),
            contents: cast_slice(&parameters),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("spinon-runtime-precise-resolve-bind-group"),
            layout: &self.resolve_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(root.current_view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: uniform.as_entire_binding(),
                },
            ],
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("spinon-runtime-precise-resolve-pass"),
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
        pass.set_pipeline(&self.resolve_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);
    }

    fn ensure_layers(
        &mut self,
        device: &wgpu::Device,
        group_depth: usize,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let dimensions_changed = self
            .root
            .as_ref()
            .is_some_and(|root| root.size != (width, height))
            || self
                .groups
                .iter()
                .any(|layer| layer.size != (width, height));
        if dimensions_changed {
            self.root = None;
            self.groups.clear();
        }

        let create_root = self.root.is_none();
        let missing_groups = group_depth.saturating_sub(self.groups.len());
        if !create_root && missing_groups == 0 {
            self.groups.truncate(group_depth);
            return Ok(());
        }

        let mut new_groups = Vec::new();
        new_groups
            .try_reserve(missing_groups)
            .map_err(|_| "정밀 WGPU opacity layer 목록을 할당하지 못했습니다".to_owned())?;
        let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let new_root = create_root.then(|| AccumulationLayer::new(device, width, height));
        for _ in 0..missing_groups {
            new_groups.push(AccumulationLayer::new(device, width, height));
        }
        let validation_error = pollster::block_on(validation_scope.pop());
        let out_of_memory_error = pollster::block_on(out_of_memory_scope.pop());
        if let Some(error) = validation_error {
            return Err(format!("정밀 WGPU opacity layer 검증 실패: {error:?}"));
        }
        if let Some(error) = out_of_memory_error {
            return Err(format!("정밀 WGPU opacity layer 할당 실패: {error:?}"));
        }
        if let Some(root) = new_root {
            self.root = Some(root);
        }
        self.groups.extend(new_groups);
        self.groups.truncate(group_depth);
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn composite_group(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    pipeline: &wgpu::RenderPipeline,
    bind_group_layout: &wgpu::BindGroupLayout,
    source: &wgpu::TextureView,
    destination: &mut AccumulationLayer,
    opacity: f32,
) {
    copy_to_next(encoder, destination);
    let uniform_values = [opacity, 0.0, 0.0, 0.0];
    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("spinon-runtime-precise-group-opacity"),
        contents: cast_slice(&uniform_values),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("spinon-runtime-precise-group-bind-group"),
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(source),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(destination.current_view()),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: uniform.as_entire_binding(),
            },
        ],
    });
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("spinon-runtime-precise-group-pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: destination.next_view(),
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
    drop(pass);
    destination.advance();
}

impl PreciseCompositor {
    pub(crate) fn handles(&self, draw: &PreciseSceneDraw<'_>) -> bool {
        if !supports_surface_format(self.surface_format)
            || !draw.scene.images().is_empty()
            || !draw.scene.texts().is_empty()
            || !needs_precise_composition(draw.scene)
        {
            return false;
        }
        let group_depth = maximum_group_depth(draw.scene.paint_commands());
        validate_group_depth(group_depth).is_ok()
            && validate_layer_extent(draw.width, draw.height, self.max_texture_dimension_2d).is_ok()
            && required_layer_bytes(draw.width, draw.height, group_depth)
                .is_some_and(|bytes| bytes <= MAX_LAYER_BYTES)
    }
}
