use spinon_render::{RuntimePaint, RuntimePaintCommand, RuntimeRenderSnapshot};

pub(super) const MAX_LAYER_BYTES: u64 = 256 * 1024 * 1024;
pub(super) const MAX_OPACITY_GROUP_DEPTH: usize = 64;

pub(super) fn maximum_group_depth(commands: &[RuntimePaintCommand]) -> usize {
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

pub(super) fn validate_group_depth(depth: usize) -> Result<(), String> {
    if depth > MAX_OPACITY_GROUP_DEPTH {
        return Err(format!(
            "정밀 opacity group 깊이가 지원 한도를 넘었습니다: {depth}"
        ));
    }
    Ok(())
}

pub(super) fn required_layer_bytes(width: u32, height: u32, group_depth: usize) -> Option<u64> {
    let layer_count = u64::try_from(group_depth.checked_add(1)?).ok()?;
    u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(16))
        .and_then(|bytes| bytes.checked_mul(2))
        .and_then(|bytes| bytes.checked_mul(layer_count))
}

pub(super) fn validate_layer_extent(
    width: u32,
    height: u32,
    max_dimension: u32,
) -> Result<(), String> {
    if width == 0 || height == 0 || width > max_dimension || height > max_dimension {
        return Err(format!(
            "정밀 opacity 표면 크기가 WGPU 한도를 벗어났습니다: {width}x{height}, 최대 {max_dimension}"
        ));
    }
    Ok(())
}

pub(super) fn needs_precise_composition(scene: &RuntimeRenderSnapshot) -> bool {
    scene.paint_commands().iter().any(|command| {
        matches!(command, RuntimePaintCommand::BeginGroup { opacity } if *opacity < 1.0)
    }) || scene.boxes().iter().any(|render_box| {
        let alpha_is_partial = |paint: RuntimePaint| {
            matches!(paint, RuntimePaint::Color(color) if color.alpha() < 1.0)
        };
        alpha_is_partial(render_box.paint())
            || [
                render_box.border().top(),
                render_box.border().right(),
                render_box.border().bottom(),
                render_box.border().left(),
            ]
            .into_iter()
            .any(|side| alpha_is_partial(side.paint()))
    })
}

pub(super) fn supports_surface_format(format: wgpu::TextureFormat) -> bool {
    matches!(
        format,
        wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm
    )
}
