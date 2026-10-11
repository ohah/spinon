use std::sync::mpsc;
use std::time::Duration;

use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoder, Device, Extent3d, MapMode, Origin3d,
    PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureFormat,
};

const PIXEL_BYTES: usize = 4;
const PADDED_ROW_BYTES: u64 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as u64;
const MAP_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct Readback {
    buffer: Buffer,
    sample: (u32, u32),
    format: TextureFormat,
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
    let [(x, y)] = points else {
        return Err("surface pixel readback은 표본 하나만 허용합니다".to_owned());
    };
    if *x >= width || *y >= height {
        return Err(format!(
            "surface pixel 표본 ({x},{y})이 drawable {width}x{height} 범위를 벗어났습니다"
        ));
    }
    if !is_supported_format(format) {
        return Err(format!(
            "surface pixel readback 포맷을 지원하지 않습니다: {format:?}"
        ));
    }

    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some("spinon-c124-surface-pixel-readback"),
        size: PADDED_ROW_BYTES,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d { x: *x, y: *y, z: 0 },
            aspect: TextureAspect::All,
        },
        TexelCopyBufferInfo {
            buffer: &buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
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
    Ok(Readback {
        buffer,
        sample: (*x, *y),
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
    let bytes: [u8; 4] = mapped
        .get(..PIXEL_BYTES)
        .ok_or_else(|| "surface pixel readback buffer가 4바이트보다 작습니다".to_owned())?
        .try_into()
        .map_err(|_| "surface pixel readback sample 크기가 올바르지 않습니다".to_owned())?;
    drop(mapped);
    readback.buffer.unmap();

    let [red, green, blue, alpha] = decode_pixel(bytes, readback.format)?;
    Ok(vec![super::SurfacePixelSample {
        x: readback.sample.0,
        y: readback.sample.1,
        rgba: [red, green, blue, alpha],
    }])
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
    use super::{decode_pixel, is_supported_format};
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
}
