use super::super::super::{
    RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE, RUNTIME_GPU_FIXTURE_SOURCE, SpinonRuntimeGpuHost,
    raw_host,
};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// 저장소의 단일 C04.10 JavaScript fixture를 실제 V8 세션에서 실행합니다.
/// 제한 시간은 평가 후 CSS 레이아웃 결과 대기에만 적용하며, JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_fixture(
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
    match host.eval(RUNTIME_GPU_FIXTURE_SOURCE, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C04.11 HTML style 요소 fixture를 실제 V8 세션에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_author_stylesheets_fixture(
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
        RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
