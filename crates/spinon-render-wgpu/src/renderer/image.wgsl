struct ImageVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct ImageOutputEncoding {
    encode_srgb: u32,
    linear_encoded: u32,
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

fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 {
        return channel / 12.92;
    }
    return pow((channel + 0.055) / 1.055, 2.4);
}

fn round_ties_to_even(channel: f32) -> f32 {
    let lower = floor(channel);
    let fraction = channel - lower;
    if fraction < 0.5 {
        return lower;
    }
    if fraction > 0.5 {
        return lower + 1.0;
    }
    if lower - 2.0 * floor(lower / 2.0) == 0.0 {
        return lower;
    }
    return lower + 1.0;
}

fn sample_linear_encoded(uv: vec2<f32>) -> vec4<f32> {
    let dimensions_u = textureDimensions(image_texture);
    let dimensions = vec2<f32>(dimensions_u);
    let texel_position = uv * dimensions - vec2<f32>(0.5, 0.5);
    let base = vec2<i32>(floor(texel_position));
    let weight = fract(texel_position);
    let maximum = vec2<i32>(dimensions_u) - vec2<i32>(1, 1);
    let p00 = clamp(base, vec2<i32>(0, 0), maximum);
    let p10 = clamp(base + vec2<i32>(1, 0), vec2<i32>(0, 0), maximum);
    let p01 = clamp(base + vec2<i32>(0, 1), vec2<i32>(0, 0), maximum);
    let p11 = clamp(base + vec2<i32>(1, 1), vec2<i32>(0, 0), maximum);
    let c00 = round(textureLoad(image_texture, p00, 0) * 255.0);
    let c10 = round(textureLoad(image_texture, p10, 0) * 255.0);
    let c01 = round(textureLoad(image_texture, p01, 0) * 255.0);
    let c11 = round(textureLoad(image_texture, p11, 0) * 255.0);
    let interpolated = mix(
        mix(c00, c10, weight.x),
        mix(c01, c11, weight.x),
        weight.y,
    );
    return vec4<f32>(
        round_ties_to_even(interpolated.r) / 255.0,
        round_ties_to_even(interpolated.g) / 255.0,
        round_ties_to_even(interpolated.b) / 255.0,
        round_ties_to_even(interpolated.a) / 255.0,
    );
}

@fragment
fn fs_image(input: ImageVertexOutput) -> @location(0) vec4<f32> {
    if image_output_encoding.linear_encoded != 0u {
        let encoded = sample_linear_encoded(input.uv);
        if image_output_encoding.encode_srgb != 0u {
            return encoded;
        }
        return vec4<f32>(
            srgb_to_linear(encoded.r),
            srgb_to_linear(encoded.g),
            srgb_to_linear(encoded.b),
            encoded.a,
        );
    }
    var sampled = textureSample(image_texture, image_sampler, input.uv);
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
