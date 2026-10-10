use super::super::{
    RUNTIME_CSS_FLEX_ALIGNMENT_FIXTURE_SOURCE, RUNTIME_CSS_FLEX_ORDER_FIXTURE_SOURCE,
    RUNTIME_CSS_FLEX_REVERSE_FIXTURE_SOURCE, RUNTIME_CSS_FLEX_WRAP_FIXTURE_SOURCE,
    SpinonRuntimeGpuHost, raw_host,
};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

#[unsafe(no_mangle)]
/// C10.1 row Flex 줄바꿈·gap fixture를 V8→Stylo→Taffy→WGPU 경로에서 실행하고
/// DOM preorder별 CSS frame을 보고합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_wrap_fixture(
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
    match host.eval_with_node_frame_report(
        RUNTIME_CSS_FLEX_WRAP_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C10.3.1 reverse Flex 방향·줄 쌓기 fixture를 V8→Stylo→Taffy→WGPU 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_reverse_fixture(
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
    match host.eval_with_node_frame_report(
        RUNTIME_CSS_FLEX_REVERSE_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C10.3.2 Flex `order` layout·paint fixture를 V8→Stylo→Taffy→WGPU 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_order_fixture(
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
    match host.eval_with_node_frame_report(
        RUNTIME_CSS_FLEX_ORDER_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C10.2 min/max freeze·재분배 fixture를 기본 V8→Stylo→Taffy→WGPU 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_distribution_fixture(
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
    match host.eval_with_node_frame_report(
        super::super::RUNTIME_CSS_FLEX_DISTRIBUTION_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C10.3.3 Flex Box Alignment fixture를 V8→Stylo→Taffy→WGPU 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 기다림 허용 background executor에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_alignment_fixture(
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
    match host.eval_with_node_frame_report(
        RUNTIME_CSS_FLEX_ALIGNMENT_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C10.3.4 Flex first/last baseline과 중첩 container 전파 fixture를 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.10 runtime GPU host여야 합니다. 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. 호출은 대기 가능한 백그라운드 실행기에서 해야 하며, `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_flex_baseline_fixture(
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
    match host.eval_with_node_frame_report(
        super::super::RUNTIME_CSS_FLEX_BASELINE_FIXTURE_SOURCE,
        layout_timeout_millis,
        true,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
