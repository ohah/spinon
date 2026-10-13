use super::super::super::{
    RUNTIME_CSS_CUSTOM_PROPERTIES_FIXTURE_SOURCE, RUNTIME_CSS_INCREMENTAL_RESTYLE_FIXTURE_SOURCE,
    RUNTIME_CSS_REGISTERED_PROPERTIES_FIXTURE_SOURCE, RUNTIME_CSS_RESULT_CACHE_FIXTURE_SOURCE,
    RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host,
};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C05.2 `@property` 검증 fixture 전용 CSS profile로 V8 GPU host를 생성합니다.
/// 일반 runtime host와 등록 속성 fixture profile은 서로 분리됩니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_registered_properties_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_registered_properties_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[unsafe(no_mangle)]
/// 현재 C04.10 fixture의 살아 있는 DOM 요소에 C05.1 사용자 지정 속성을 적용합니다.
///
/// V8 세션·HostDocument를 새로 만들지 않고 기존 `style` setter를 통해 layout·paint를
/// 갱신합니다. 제한 시간은 평가 후 CSS 결과 대기에만 적용합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_custom_properties_fixture(
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
        RUNTIME_CSS_CUSTOM_PROPERTIES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C05.2 등록 사용자 지정 속성 fixture를 실제 V8·HostDocument 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_registered_properties_fixture(
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
        RUNTIME_CSS_REGISTERED_PROPERTIES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C05.3 detached-node fixture를 실제 V8·HostDocument 경로에서 한 단계 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_runtime_result_cache_fixture(
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
        RUNTIME_CSS_RESULT_CACHE_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 실제 V8 HostDocument에서 inline style 하위 트리 재계산 fixture를 한 단계 실행합니다.
///
/// 첫 호출은 단일 root와 좌우 branch를 만들고, 이후 호출은 왼쪽 branch의 사용자 지정 속성만
/// 번갈아 바꿉니다. 완료 보고에는 cascade 재계산·재사용·상속 문맥 계산 수가 포함됩니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_incremental_restyle_fixture(
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
        RUNTIME_CSS_INCREMENTAL_RESTYLE_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}
