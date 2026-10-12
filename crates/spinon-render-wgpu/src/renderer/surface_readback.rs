use std::sync::mpsc;
use std::time::Duration;

use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoder, Device, Extent3d, MapMode, Origin3d,
    PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureFormat,
};

const PIXEL_BYTES: usize = 4;
const PADDED_ROW_BYTES: u64 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64;
const MAX_SAMPLES: usize = 64;
const MAP_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct Readback {
    buffer: Buffer,
    points: Vec<(u32, u32)>,
    format: TextureFormat,
}

pub(super) fn validate_points(
    width: u32,
    height: u32,
    points: &[(u32, u32)],
) -> Result<(), String> {
    if points.is_empty() || points.len() > MAX_SAMPLES {
        return Err(format!(
            "surface pixel readback 표본 수는 1–{MAX_SAMPLES}개여야 합니다"
        ));
    }
    if let Some((x, y)) = points.iter().find(|(x, y)| *x >= width || *y >= height) {
        return Err(format!(
            "surface pixel 표본 ({x},{y})이 drawable {width}x{height} 범위를 벗어났습니다"
        ));
    }
    Ok(())
}

pub(super) fn append_copy(
    device: &Device,
    encoder: &mut CommandEncoder,
    texture: &Texture,
    format: TextureFormat,
    width: u32,
    height: u32,
    points: &[(u32, u32)],
) -> Result<Readback, String> {
    validate_points(width, height, points)?;
    if !is_supported_format(format) {
        return Err(format!(
            "surface pixel readback 포맷을 지원하지 않습니다: {format:?}"
        ));
    }

    let sample_count = u64::try_from(points.len())
        .map_err(|_| "surface pixel 표본 수 변환이 범위를 넘었습니다".to_owned())?;
    let buffer_size = PADDED_ROW_BYTES
        .checked_mul(sample_count)
        .ok_or_else(|| "surface pixel readback 버퍼 크기가 범위를 넘었습니다".to_owned())?;
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("spinon-surface-pixel-readback"),
        size: buffer_size,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    for (index, &(x, y)) in points.iter().enumerate() {
        let row_index = u64::try_from(index)
            .map_err(|_| "surface pixel 표본 index가 범위를 넘었습니다".to_owned())?;
        let offset = PADDED_ROW_BYTES
            .checked_mul(row_index)
            .ok_or_else(|| "surface pixel readback offset이 범위를 넘었습니다".to_owned())?;
        encoder.copy_texture_to_buffer(
            TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: Origin3d { x, y, z: 0 },
                aspect: TextureAspect::All,
            },
            TexelCopyBufferInfo {
                buffer: &buffer,
                layout: TexelCopyBufferLayout {
                    offset,
                    bytes_per_row: Some(PADDED_ROW_BYTES as u32),
                    rows_per_image: None,
                },
            },
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
    }
    Ok(Readback {
        buffer,
        points: points.to_vec(),
        format,
    })
}

pub(super) fn map_samples(
    device: &Device,
    submission: &wgpu::SubmissionIndex,
    readback: Readback,
) -> Result<Vec<super::SurfacePixelSample>, String> {
    let (sender, receiver) = mpsc::sync_channel(1);
    readback
        .buffer
        .slice(..)
        .map_async(MapMode::Read, move |result| {
            let _ = sender.send(result.map_err(|error| error.to_string()));
        });

    device
        .poll(PollType::Wait {
            submission_index: Some(submission.clone()),
            timeout: Some(MAP_TIMEOUT),
        })
        .map_err(|error| format!("surface pixel readback GPU 대기 실패: {error}"))?;
    receiver
        .recv_timeout(Duration::from_millis(100))
        .map_err(|error| {
            format!("surface pixel readback mapping 결과가 오지 않았습니다: {error}")
        })??;

    let mapped = readback
        .buffer
        .slice(..)
        .get_mapped_range()
        .map_err(|error| format!("surface pixel readback buffer mapping 실패: {error}"))?;
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(readback.points.len())
        .map_err(|error| format!("surface pixel 표본 결과 할당 실패: {error}"))?;
    for (index, &(x, y)) in readback.points.iter().enumerate() {
        let row_index = u64::try_from(index)
            .map_err(|_| "surface pixel 결과 index가 범위를 넘었습니다".to_owned())?;
        let start = PADDED_ROW_BYTES
            .checked_mul(row_index)
            .and_then(|offset| usize::try_from(offset).ok())
            .ok_or_else(|| "surface pixel 결과 offset이 범위를 넘었습니다".to_owned())?;
        let end = start
            .checked_add(PIXEL_BYTES)
            .ok_or_else(|| "surface pixel 결과 범위가 넘쳤습니다".to_owned())?;
        let bytes: [u8; 4] = mapped
            .get(start..end)
            .ok_or_else(|| "surface pixel readback buffer가 표본보다 작습니다".to_owned())?
            .try_into()
            .map_err(|_| "surface pixel readback sample 크기가 올바르지 않습니다".to_owned())?;
        let [red, green, blue, alpha] = decode_pixel(bytes, readback.format)?;
        samples.push(super::SurfacePixelSample {
            x,
            y,
            rgba: [red, green, blue, alpha],
        });
    }
    drop(mapped);
    readback.buffer.unmap();
    Ok(samples)
}

const fn is_supported_format(format: TextureFormat) -> bool {
    matches!(
        format,
        TextureFormat::Rgba8Unorm
            | TextureFormat::Rgba8UnormSrgb
            | TextureFormat::Bgra8Unorm
            | TextureFormat::Bgra8UnormSrgb
    )
}

fn decode_pixel(bytes: [u8; 4], format: TextureFormat) -> Result<[u8; 4], String> {
    match format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => Ok(bytes),
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            Ok([bytes[2], bytes[1], bytes[0], bytes[3]])
        }
        _ => Err(format!(
            "surface pixel readback 포맷을 지원하지 않습니다: {format:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_pixel, is_supported_format, validate_points};
    use wgpu::TextureFormat;

    #[test]
    fn accepts_only_eight_bit_rgba_and_bgra_formats() {
        assert!(is_supported_format(TextureFormat::Rgba8Unorm));
        assert!(is_supported_format(TextureFormat::Rgba8UnormSrgb));
        assert!(is_supported_format(TextureFormat::Bgra8Unorm));
        assert!(is_supported_format(TextureFormat::Bgra8UnormSrgb));
        assert!(!is_supported_format(TextureFormat::Rgba16Float));
        assert!(!is_supported_format(TextureFormat::Depth24Plus));
    }

    #[test]
    fn decodes_rgba_and_bgra_without_changing_alpha() {
        assert_eq!(
            decode_pixel([0x28, 0x66, 0xf6, 0xff], TextureFormat::Rgba8UnormSrgb)
                .expect("RGBA sample"),
            [0x28, 0x66, 0xf6, 0xff]
        );
        assert_eq!(
            decode_pixel([0xf6, 0x66, 0x28, 0x7f], TextureFormat::Bgra8Unorm).expect("BGRA sample"),
            [0x28, 0x66, 0xf6, 0x7f]
        );
        assert!(decode_pixel([0; 4], TextureFormat::Rgba16Float).is_err());
    }

    #[test]
    fn validates_sample_batch_boundaries_and_drawable_coordinates() {
        assert!(validate_points(320, 240, &[]).is_err());
        assert!(validate_points(320, 240, &[(0, 0)]).is_ok());
        assert!(validate_points(320, 240, &[(1, 1); 7]).is_ok());
        assert!(validate_points(320, 240, &[(2, 2); 64]).is_ok());
        assert!(validate_points(320, 240, &[(2, 2); 65]).is_err());
        assert!(validate_points(320, 240, &[(320, 0)]).is_err());
        assert!(validate_points(320, 240, &[(0, 240)]).is_err());
    }
}
