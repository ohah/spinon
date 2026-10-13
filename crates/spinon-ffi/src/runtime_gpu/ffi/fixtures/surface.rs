use super::super::{ERR_ARGUMENT, SpinonRuntimeGpuHost, raw_host, write_host_coded_result};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// 내부 검증 빌드에서 다음 WGPU draw 한 번의 오류 전파를 시험용으로 설정합니다.
///
/// # Safety
/// `host`는 살아 있어야 하며 호출은 draw·resize·destroy와 같은 render queue에서 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_inject_next_draw_failure_for_test(
    host: *mut SpinonRuntimeGpuHost,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    write_host_coded_result(
        host.inject_next_draw_failure_for_test(),
        output,
        output_capacity,
    )
}
