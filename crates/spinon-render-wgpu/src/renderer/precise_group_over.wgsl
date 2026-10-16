struct GroupOpacity {
    values: vec4<f32>,
};

@group(0) @binding(0) var source_layer: texture_2d<f32>;
@group(0) @binding(1) var previous_parent: texture_2d<f32>;
@group(0) @binding(2) var<uniform> group_opacity: GroupOpacity;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(position.xy);
    let source = textureLoad(source_layer, pixel, 0);
    let destination = textureLoad(previous_parent, pixel, 0);
    let source_alpha = source.a * group_opacity.values.x;
    var source_rgb = source.rgb * group_opacity.values.x;
    if source.a >= 1.0 && destination.a >= 1.0 {
        // Keep transparent intermediate groups in float; quantize opaque root composition.
        source_rgb = floor(source_rgb * 255.0) / 255.0;
    }
    return vec4<f32>(
        source_rgb + destination.rgb * (1.0 - source_alpha),
        source_alpha + destination.a * (1.0 - source_alpha),
    );
}
