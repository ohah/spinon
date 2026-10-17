struct GlyphVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@group(0) @binding(0) var glyph_mask: texture_2d<f32>;
@group(0) @binding(1) var glyph_sampler: sampler;

@vertex
fn vs_glyph(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> GlyphVertexOutput {
    var output: GlyphVertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.uv = uv;
    output.color = color;
    return output;
}

@fragment
fn fs_glyph(input: GlyphVertexOutput) -> @location(0) vec4<f32> {
    let coverage = textureSample(glyph_mask, glyph_sampler, input.uv).r;
    return vec4<f32>(input.color.rgb, input.color.a * coverage);
}
