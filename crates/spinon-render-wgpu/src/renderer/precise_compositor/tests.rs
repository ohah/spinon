use super::limits::MAX_OPACITY_GROUP_DEPTH;
use super::{
    MAX_LAYER_BYTES, maximum_group_depth, required_layer_bytes, supports_surface_format,
    validate_group_depth, validate_layer_extent,
};
use spinon_render::RuntimePaintCommand;

fn nested_group_commands(depth: usize) -> Vec<RuntimePaintCommand> {
    let mut commands = Vec::with_capacity(depth.saturating_mul(2));
    commands.extend((0..depth).map(|_| RuntimePaintCommand::BeginGroup { opacity: 0.5 }));
    commands.extend((0..depth).map(|_| RuntimePaintCommand::EndGroup));
    commands
}

#[test]
fn precise_group_depth_respects_layer_limit() {
    assert_eq!(
        maximum_group_depth(&nested_group_commands(MAX_OPACITY_GROUP_DEPTH)),
        64
    );
    assert_eq!(
        maximum_group_depth(&nested_group_commands(MAX_OPACITY_GROUP_DEPTH + 1)),
        65
    );
    assert!(validate_group_depth(MAX_OPACITY_GROUP_DEPTH).is_ok());
    assert!(validate_group_depth(MAX_OPACITY_GROUP_DEPTH + 1).is_err());
}

#[test]
fn precise_layer_extent_rejects_zero_and_oversize_dimensions() {
    assert!(validate_layer_extent(1, 1, 8192).is_ok());
    assert!(validate_layer_extent(0, 1, 8192).is_err());
    assert!(validate_layer_extent(8193, 1, 8192).is_err());
}

#[test]
fn precise_layer_memory_budget_counts_ping_pong_root_and_group_layers() {
    assert_eq!(required_layer_bytes(2048, 2048, 1), Some(MAX_LAYER_BYTES));
    assert!(required_layer_bytes(2048, 2048, 1).is_some_and(|bytes| bytes <= MAX_LAYER_BYTES));
    assert!(required_layer_bytes(2048, 2048, 2).is_some_and(|bytes| bytes > MAX_LAYER_BYTES));
    assert_eq!(required_layer_bytes(u32::MAX, u32::MAX, usize::MAX), None);
}

#[test]
fn precise_path_only_handles_unorm_srgb_compatible_surface_formats() {
    assert!(supports_surface_format(wgpu::TextureFormat::Rgba8Unorm));
    assert!(supports_surface_format(wgpu::TextureFormat::Bgra8Unorm));
    assert!(!supports_surface_format(
        wgpu::TextureFormat::Rgba8UnormSrgb
    ));
    assert!(!supports_surface_format(
        wgpu::TextureFormat::Bgra8UnormSrgb
    ));
    assert!(!supports_surface_format(wgpu::TextureFormat::Rgba16Float));
}
