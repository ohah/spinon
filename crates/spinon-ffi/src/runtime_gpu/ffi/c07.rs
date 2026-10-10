use super::super::{
    RUNTIME_CSS_ASPECT_RATIO_FIXTURE_SOURCE, RUNTIME_CSS_BORDER_WIDTH_FIXTURE_SOURCE,
    RUNTIME_CSS_MIN_MAX_SIZING_FIXTURE_SOURCE, SpinonRuntimeGpuHost, raw_host,
};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// C07.1 물리 축 최소·최대 크기 fixture를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하며 같은 host 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_min_max_sizing_fixture(
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
        RUNTIME_CSS_MIN_MAX_SIZING_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C07.3 종횡비 fixture를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하며 같은 host 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_aspect_ratio_fixture(
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
        RUNTIME_CSS_ASPECT_RATIO_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C07.2 테두리 폭 fixture를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하며 같은 host 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_border_width_fixture(
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
        RUNTIME_CSS_BORDER_WIDTH_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
