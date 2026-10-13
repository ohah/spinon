use super::super::super::{
    RUNTIME_CSS_ABSOLUTE_LENGTHS_FIXTURE_SOURCE, RUNTIME_CSS_FONT_RELATIVE_UNITS_FIXTURE_SOURCE,
    RUNTIME_CSS_PERCENTAGE_DIMENSIONS_FIXTURE_SOURCE,
    RUNTIME_CSS_SPACING_PERCENTAGES_FIXTURE_SOURCE,
};
use super::super::super::{
    RUNTIME_CSS_TYPED_MATH_FIXTURE_SOURCE, RUNTIME_CSS_VIEWPORT_UNITS_FIXTURE_SOURCE,
    SpinonRuntimeGpuHost, raw_host,
};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// C06.5 typed CSS math를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하며 같은 host 접근·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_typed_css_math_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(RUNTIME_CSS_TYPED_MATH_FIXTURE_SOURCE, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.6a viewport 단위를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하며 같은 host 접근·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_viewport_units_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_VIEWPORT_UNITS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.1 percentage width·height·flex-basis를 실제 V8·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_percentage_dimensions_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_PERCENTAGE_DIMENSIONS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.2 percentage margin·padding·gap을 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_spacing_percentages_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_SPACING_PERCENTAGES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.3 절대 CSS 길이 단위를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_absolute_lengths_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_ABSOLUTE_LENGTHS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.4 em/rem fixture를 실제 V8·Stylo·Taffy·WGPU runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_font_relative_units_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_FONT_RELATIVE_UNITS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
