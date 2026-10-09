use std::sync::mpsc;
use std::time::Duration;

use spinon_render::RuntimeRenderSnapshot;

use crate::geometry::{ColorEncoding, build_vertices};
use crate::renderer::create_pipeline;

/// 같은 runtime scene·shader·geometry 경로를 headless texture에 그려 표본을 읽습니다.
/// 앱 공개 API가 아니라 고정 Chromium fixture 검증용입니다.
#[doc(hidden)]
pub fn readback_runtime_scene_samples(
    scene: &RuntimeRenderSnapshot,
    width: u32,
    height: u32,
    sample_points: &[(u32, u32)],
) -> Result<Vec<[u8; 4]>, String> {
    if width == 0 || height == 0 {
        return Err("WGPU readback 표면 크기는 0보다 커야 합니다".to_owned());
    }
    if sample_points
        .iter()
        .any(|(x, y)| *x >= width || *y >= height)
    {
        return Err("WGPU readback 표본이 표면 범위를 벗어났습니다".to_owned());
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::METAL | wgpu::Backends::VULKAN | wgpu::Backends::GL,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
        ..Default::default()
    }))
    .map_err(|error| format!("headless WGPU adapter를 얻지 못했습니다: {error}"))?;
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_limits: adapter.limits(),
        ..Default::default()
    }))
    .map_err(|error| format!("headless WGPU device를 만들지 못했습니다: {error}"))?;

    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spinon-runtime-scene-readback-target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let pipeline = create_pipeline(&device, format);
    let vertices = build_vertices(scene, width, height, ColorEncoding::Linear)?;
    let vertex_count = u32::try_from(vertices.len())
        .map_err(|_| "WGPU readback 정점 수가 범위를 넘었습니다".to_owned())?;
    let bytes = bytemuck::cast_slice(&vertices);
    let vertex_buffer_size = u64::try_from(
        bytes
            .len()
            .max(std::mem::size_of::<crate::geometry::Vertex>()),
    )
    .map_err(|_| "WGPU readback 정점 버퍼 크기가 범위를 넘었습니다".to_owned())?;
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("spinon-runtime-scene-readback-vertices"),
        size: vertex_buffer_size,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !bytes.is_empty() {
        queue.write_buffer(&vertex_buffer, 0, bytes);
    }

    let unpadded_bytes_per_row = width
        .checked_mul(4)
        .ok_or_else(|| "WGPU readback 행 크기가 범위를 넘었습니다".to_owned())?;
    let alignment = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let bytes_per_row = unpadded_bytes_per_row
        .checked_add(alignment - 1)
        .map(|value| value / alignment * alignment)
        .ok_or_else(|| "WGPU readback 정렬 행 크기가 범위를 넘었습니다".to_owned())?;
    let buffer_size = u64::from(bytes_per_row)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "WGPU readback 버퍼 크기가 범위를 넘었습니다".to_owned())?;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("spinon-runtime-scene-readback-buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("spinon-runtime-scene-readback-encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("spinon-runtime-scene-readback-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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
        if vertex_count != 0 {
            pass.set_pipeline(&pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.draw(0..vertex_count, 0..1);
        }
    }
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    let submission = queue.submit([encoder.finish()]);
    let (sender, receiver) = mpsc::sync_channel(1);
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result.map_err(|error| error.to_string()));
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: Some(Duration::from_secs(10)),
        })
        .map_err(|error| format!("WGPU readback 제출 대기 실패: {error}"))?;
    receiver
        .recv_timeout(Duration::from_secs(1))
        .map_err(|error| format!("WGPU readback map 대기 실패: {error}"))??;

    let mapped = readback
        .slice(..)
        .get_mapped_range()
        .map_err(|error| format!("WGPU readback bytes 접근 실패: {error}"))?;
    let mut samples = Vec::new();
    samples
        .try_reserve(sample_points.len())
        .map_err(|_| "WGPU readback 표본 목록을 할당하지 못했습니다".to_owned())?;
    for (x, y) in sample_points {
        let offset = usize::try_from(u64::from(*y) * u64::from(bytes_per_row) + u64::from(*x) * 4)
            .map_err(|_| "WGPU readback 표본 offset이 범위를 넘었습니다".to_owned())?;
        let rgba = mapped
            .get(offset..offset + 4)
            .ok_or_else(|| "WGPU readback 표본이 mapped 범위를 벗어났습니다".to_owned())?;
        samples.push([rgba[0], rgba[1], rgba[2], rgba[3]]);
    }
    drop(mapped);
    readback.unmap();
    Ok(samples)
}
