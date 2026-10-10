use super::super::{RUNTIME_CSS_BLOCK_PAINT_FIXTURE_SOURCE, SpinonRuntimeGpuHost, raw_host};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// C08 Block·기본 페인트 fixture를 실제 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C08 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub(super) unsafe extern "C" fn spinon_runtime_gpu_host_eval_block_paint_fixture(
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
        RUNTIME_CSS_BLOCK_PAINT_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
