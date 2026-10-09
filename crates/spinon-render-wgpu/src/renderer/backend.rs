use super::RuntimeRendererError;
use crate::geometry::ColorEncoding;

pub(super) fn backend_from_abi(backend: u32) -> Result<wgpu::Backends, RuntimeRendererError> {
    match backend {
        1 => Ok(wgpu::Backends::VULKAN),
        2 => Ok(wgpu::Backends::GL),
        3 => Ok(wgpu::Backends::METAL),
        _ => Err(RuntimeRendererError::InvalidArgument(
            "GPU backend 값이 올바르지 않습니다",
        )),
    }
}

pub(super) fn android_auto_backend_order() -> [u32; 2] {
    [1, 2]
}

pub(super) fn choose_sdr_format(
    capabilities: &wgpu::SurfaceCapabilities,
) -> Result<wgpu::TextureFormat, RuntimeRendererError> {
    [
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureFormat::Bgra8UnormSrgb,
    ]
    .into_iter()
    .chain([
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Bgra8Unorm,
    ])
    .find(|candidate| {
        capabilities.format_capabilities.iter().any(|item| {
            item.format == *candidate && item.color_spaces.contains(wgpu::SurfaceColorSpaces::SRGB)
        })
    })
    .ok_or_else(|| {
        RuntimeRendererError::Initialization(
            "surface가 8-bit sRGB color space와 호환되는 형식을 제공하지 않습니다".to_owned(),
        )
    })
}

pub(super) fn color_encoding(format: wgpu::TextureFormat) -> ColorEncoding {
    if format.is_srgb() {
        ColorEncoding::Linear
    } else {
        ColorEncoding::Srgb
    }
}
