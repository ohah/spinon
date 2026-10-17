use spinon_render::RuntimeRenderText;

use super::MAX_GLYPH_ATLAS_BYTES;

pub(super) struct GpuGlyphAtlas {
    _texture: wgpu::Texture,
    pub(super) bind_group: wgpu::BindGroup,
    pub(super) byte_len: usize,
}

pub(super) struct MaskAtlasData {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) placements: Vec<(u32, u32)>,
    pub(super) pixels: Vec<u8>,
}

pub(super) fn build_mask_atlas(
    text: &RuntimeRenderText,
    max_dimension: u32,
) -> Result<Option<MaskAtlasData>, String> {
    let glyphs = text.glyphs();
    if glyphs.is_empty() {
        return Ok(None);
    }
    let mut total_area = 0_u64;
    let mut max_width = 1_u32;
    for glyph in glyphs {
        let pixel_count = u64::from(glyph.width_px())
            .checked_mul(u64::from(glyph.height_px()))
            .ok_or_else(|| "WGPU glyph atlas area 계산이 넘쳤습니다".to_owned())?;
        total_area = total_area
            .checked_add(pixel_count)
            .ok_or_else(|| "WGPU glyph atlas 전체 면적 계산이 넘쳤습니다".to_owned())?;
        max_width = max_width.max(glyph.width_px());
    }
    if max_width > max_dimension {
        return Err(format!(
            "runtime glyph 폭이 device texture 제한을 넘었습니다: {max_width}>{max_dimension}"
        ));
    }
    let desired_width = (total_area as f64).sqrt().ceil() as u64;
    let atlas_width = u64::from(max_width)
        .max(desired_width)
        .min(u64::from(max_dimension)) as u32;
    let mut placements = Vec::new();
    placements
        .try_reserve_exact(glyphs.len())
        .map_err(|_| "WGPU glyph atlas 배치 목록을 할당하지 못했습니다".to_owned())?;
    let mut cursor_x = 0_u32;
    let mut cursor_y = 0_u32;
    let mut row_height = 0_u32;
    for glyph in glyphs {
        if cursor_x
            .checked_add(glyph.width_px())
            .is_none_or(|right| right > atlas_width)
        {
            cursor_y = cursor_y
                .checked_add(row_height)
                .ok_or_else(|| "WGPU glyph atlas 높이 계산이 넘쳤습니다".to_owned())?;
            cursor_x = 0;
            row_height = 0;
        }
        if cursor_y
            .checked_add(glyph.height_px())
            .is_none_or(|bottom| bottom > max_dimension)
        {
            return Err("runtime glyph atlas가 device texture 높이 제한을 넘었습니다".to_owned());
        }
        placements.push((cursor_x, cursor_y));
        cursor_x += glyph.width_px();
        row_height = row_height.max(glyph.height_px());
    }
    let atlas_height = cursor_y
        .checked_add(row_height)
        .filter(|height| *height > 0)
        .ok_or_else(|| "WGPU glyph atlas 높이가 유효하지 않습니다".to_owned())?;
    let byte_len = u64::from(atlas_width)
        .checked_mul(u64::from(atlas_height))
        .and_then(|bytes| usize::try_from(bytes).ok())
        .filter(|bytes| *bytes <= MAX_GLYPH_ATLAS_BYTES)
        .ok_or_else(|| "runtime glyph atlas가 64 MiB 한도를 넘었습니다".to_owned())?;
    let mut pixels = Vec::new();
    pixels
        .try_reserve_exact(byte_len)
        .map_err(|_| "WGPU glyph atlas pixel buffer를 할당하지 못했습니다".to_owned())?;
    pixels.resize(byte_len, 0);
    for (glyph, (x, y)) in glyphs.iter().zip(&placements) {
        let glyph_width = usize::try_from(glyph.width_px())
            .map_err(|_| "runtime glyph 폭이 host 크기로 변환되지 않습니다".to_owned())?;
        let atlas_width = usize::try_from(atlas_width)
            .map_err(|_| "runtime glyph atlas 폭이 host 크기로 변환되지 않습니다".to_owned())?;
        let y = usize::try_from(*y)
            .map_err(|_| "runtime glyph atlas y 좌표가 host 크기로 변환되지 않습니다".to_owned())?;
        let x = usize::try_from(*x)
            .map_err(|_| "runtime glyph atlas x 좌표가 host 크기로 변환되지 않습니다".to_owned())?;
        let glyph_height = usize::try_from(glyph.height_px())
            .map_err(|_| "runtime glyph 높이가 host 크기로 변환되지 않습니다".to_owned())?;
        for row in 0..glyph_height {
            let source_start = row * glyph_width;
            let target_start = (y + row) * atlas_width + x;
            pixels[target_start..target_start + glyph_width]
                .copy_from_slice(&glyph.pixels()[source_start..source_start + glyph_width]);
        }
    }
    Ok(Some(MaskAtlasData {
        width: atlas_width,
        height: atlas_height,
        placements,
        pixels,
    }))
}

pub(super) fn create_gpu_atlas(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    atlas: &MaskAtlasData,
) -> Result<GpuGlyphAtlas, String> {
    let out_of_memory_scope = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
    let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spinon-runtime-glyph-mask-atlas"),
        size: wgpu::Extent3d {
            width: atlas.width,
            height: atlas.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("spinon-runtime-glyph-atlas-bind-group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
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
        &atlas.pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(atlas.width),
            rows_per_image: Some(atlas.height),
        },
        wgpu::Extent3d {
            width: atlas.width,
            height: atlas.height,
            depth_or_array_layers: 1,
        },
    );
    if let Some(error) = pollster::block_on(validation_scope.pop()) {
        let _ = pollster::block_on(out_of_memory_scope.pop());
        return Err(format!("WGPU glyph atlas 검증 실패: {error}"));
    }
    if let Some(error) = pollster::block_on(out_of_memory_scope.pop()) {
        return Err(format!("WGPU glyph atlas 할당 실패: {error}"));
    }
    Ok(GpuGlyphAtlas {
        _texture: texture,
        bind_group,
        byte_len: atlas.pixels.len(),
    })
}
