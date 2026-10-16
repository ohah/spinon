struct ImageVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct ImageOutputEncoding {
    encode_srgb: u32,
    padding_0: u32,
    padding_1: u32,
    padding_2: u32,
};

@group(0) @binding(0) var image_texture: texture_2d<f32>;
@group(0) @binding(1) var image_sampler: sampler;
@group(0) @binding(2) var<uniform> image_output_encoding: ImageOutputEncoding;

@vertex
fn vs_image(
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
) -> ImageVertexOutput {
    var output: ImageVertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.uv = uv;
    return output;
}

fn linear_to_srgb(channel: f32) -> f32 {
    if channel <= 0.0031308 {
        return channel * 12.92;
    }
    return 1.055 * pow(max(channel, 0.0), 1.0 / 2.4) - 0.055;
}

@fragment
fn fs_image(input: ImageVertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(image_texture, image_sampler, input.uv);
    if image_output_encoding.encode_srgb != 0u {
        var encoded = vec3<f32>(
            linear_to_srgb(sampled.r),
            linear_to_srgb(sampled.g),
            linear_to_srgb(sampled.b),
        );
        if sampled.a > 0.0 && sampled.a < 1.0 {
            let quantized_premultiplied = round(encoded * sampled.a * 255.0) / 255.0;
            encoded = quantized_premultiplied / sampled.a;
        }
        return vec4<f32>(encoded, sampled.a);
    }
    return sampled;
}
