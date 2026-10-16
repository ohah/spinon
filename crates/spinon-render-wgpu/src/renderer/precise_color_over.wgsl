struct ColorVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var previous_layer: texture_2d<f32>;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
) -> ColorVertexOutput {
    var output: ColorVertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: ColorVertexOutput) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(input.position.xy);
    let destination = textureLoad(previous_layer, pixel, 0);
    let source_alpha = clamp(input.color.a, 0.0, 1.0);
    // Preserve the captured RGBA8 path's premultiplied source-channel boundary.
    let source_rgb = floor(input.color.rgb * source_alpha * 255.0) / 255.0;
    return vec4<f32>(
        source_rgb + destination.rgb * (1.0 - source_alpha),
        source_alpha + destination.a * (1.0 - source_alpha),
    );
}
