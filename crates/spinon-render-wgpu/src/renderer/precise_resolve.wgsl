struct ResolveParameters {
    surface_is_srgb: u32,
    padding_1: u32,
    padding_2: u32,
    padding_3: u32,
};

@group(0) @binding(0) var resolved_layer: texture_2d<f32>;
@group(0) @binding(1) var<uniform> resolve_parameters: ResolveParameters;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

fn quantize_unorm8(channel: f32) -> f32 {
    let scaled = clamp(channel, 0.0, 1.0) * 255.0;
    return floor(scaled + 0.5) / 255.0;
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

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(position.xy);
    let source = textureLoad(resolved_layer, pixel, 0);
    if resolve_parameters.surface_is_srgb != 0u {
        let encoded = vec3<f32>(
            linear_to_srgb(source.r),
            linear_to_srgb(source.g),
            linear_to_srgb(source.b),
        );
        let quantized = vec3<f32>(
            quantize_unorm8(encoded.r),
            quantize_unorm8(encoded.g),
            quantize_unorm8(encoded.b),
        );
        return vec4<f32>(
            srgb_to_linear(quantized.r),
            srgb_to_linear(quantized.g),
            srgb_to_linear(quantized.b),
            quantize_unorm8(source.a),
        );
    }
    return vec4<f32>(
        quantize_unorm8(source.r),
        quantize_unorm8(source.g),
        quantize_unorm8(source.b),
        quantize_unorm8(source.a),
    );
}
