use super::super::{
    RUNTIME_CSS_BLOCK_FORMATTING_FIXTURE_SOURCE, RUNTIME_CSS_MARGIN_COLLAPSE_FIXTURE_SOURCE,
    SpinonRuntimeGpuHost, raw_host,
};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// C09.1 CSS Block width·containing block·auto margin fixture를 실제 V8 경로에서 평가합니다.
/// 각 HostDocument 노드의 DOM preorder와 CSS frame을 보고에 포함합니다.
///
/// # Safety
/// `host`는 살아 있는 C09.1 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_block_formatting_fixture(
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
        RUNTIME_CSS_BLOCK_FORMATTING_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C09.2 세로 margin collapse fixture를 실제 V8·Stylo·Taffy 경로에서 평가합니다.
/// 각 HostDocument 노드의 DOM preorder와 CSS frame을 보고에 포함합니다.
///
/// # Safety
/// `host`는 살아 있는 C09 Block formatting runtime host여야 합니다. 같은 host의 호출·해제와
/// 경합시키면 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_margin_collapse_fixture(
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
        RUNTIME_CSS_MARGIN_COLLAPSE_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
