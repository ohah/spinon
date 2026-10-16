struct GroupOpacity {
    values: vec4<f32>,
};

@group(0) @binding(0) var group_layer: texture_2d<f32>;
@group(0) @binding(1) var<uniform> group_opacity: GroupOpacity;

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
    return textureLoad(group_layer, pixel, 0) * group_opacity.values.x;
}
