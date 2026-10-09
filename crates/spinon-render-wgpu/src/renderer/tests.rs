#[cfg(feature = "test-hooks")]
use super::DrawFailureInjector;
use super::{
    RuntimeRendererError, android_auto_backend_order, backend_from_abi, choose_sdr_format,
    color_encoding,
};

#[cfg(feature = "test-hooks")]
#[test]
fn draw_failure_hook_is_consumed_once() {
    let hook = DrawFailureInjector::default();
    hook.arm();
    assert!(hook.take());
    assert!(!hook.take());
}

#[test]
fn android_auto_backend_tries_vulkan_then_gles_sequentially() {
    assert_eq!(android_auto_backend_order(), [1, 2]);
}

#[test]
fn backend_override_rejects_unknown_values() {
    assert!(matches!(
        backend_from_abi(99),
        Err(RuntimeRendererError::InvalidArgument(_))
    ));
}

fn capabilities(
    formats: &[(wgpu::TextureFormat, wgpu::SurfaceColorSpaces)],
) -> wgpu::SurfaceCapabilities {
    wgpu::SurfaceCapabilities {
        formats: formats.iter().map(|(format, _)| *format).collect(),
        format_capabilities: formats
            .iter()
            .map(|(format, color_spaces)| wgpu::SurfaceFormatCapabilities {
                format: *format,
                color_spaces: *color_spaces,
            })
            .collect(),
        present_modes: vec![wgpu::PresentMode::Fifo],
        alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
        usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
    }
}

#[test]
fn surface_prefers_rgba_srgb_when_both_supported() {
    let capabilities = capabilities(&[
        (
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::SurfaceColorSpaces::SRGB,
        ),
        (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::SurfaceColorSpaces::SRGB,
        ),
    ]);
    assert_eq!(
        choose_sdr_format(&capabilities).unwrap(),
        wgpu::TextureFormat::Rgba8UnormSrgb
    );
}

#[test]
fn surface_uses_bgra_srgb_as_fallback() {
    let capabilities = capabilities(&[(
        wgpu::TextureFormat::Bgra8UnormSrgb,
        wgpu::SurfaceColorSpaces::SRGB,
    )]);
    assert_eq!(
        choose_sdr_format(&capabilities).unwrap(),
        wgpu::TextureFormat::Bgra8UnormSrgb
    );
}

#[test]
fn surface_falls_back_to_unorm_when_it_supports_srgb_color_space() {
    let capabilities = capabilities(&[
        (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::SurfaceColorSpaces::EXTENDED_SRGB_LINEAR,
        ),
        (
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::SurfaceColorSpaces::SRGB,
        ),
    ]);
    assert_eq!(
        choose_sdr_format(&capabilities).unwrap(),
        wgpu::TextureFormat::Bgra8Unorm
    );
}

#[test]
fn surface_uses_rgba_unorm_as_android_style_fallback() {
    let capabilities = capabilities(&[(
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::SurfaceColorSpaces::SRGB,
    )]);
    assert_eq!(
        choose_sdr_format(&capabilities).unwrap(),
        wgpu::TextureFormat::Rgba8Unorm
    );
}

#[test]
fn surface_rejects_unorm_format_without_srgb_color_space() {
    let capabilities = capabilities(&[(
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::SurfaceColorSpaces::EXTENDED_SRGB_LINEAR,
    )]);
    assert!(matches!(
        choose_sdr_format(&capabilities),
        Err(RuntimeRendererError::Initialization(_))
    ));
}

#[test]
fn surface_prefers_srgb_texture_format_before_unorm_fallback() {
    let capabilities = capabilities(&[
        (
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::SurfaceColorSpaces::SRGB,
        ),
        (
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::SurfaceColorSpaces::SRGB,
        ),
    ]);
    assert_eq!(
        choose_sdr_format(&capabilities).unwrap(),
        wgpu::TextureFormat::Bgra8UnormSrgb
    );
}

#[test]
fn color_transfer_matches_surface_texture_format() {
    assert_eq!(
        color_encoding(wgpu::TextureFormat::Rgba8UnormSrgb),
        crate::geometry::ColorEncoding::Linear
    );
    assert_eq!(
        color_encoding(wgpu::TextureFormat::Rgba8Unorm),
        crate::geometry::ColorEncoding::Srgb
    );
}
