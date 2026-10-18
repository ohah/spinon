use super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use std::ffi::{CStr, c_char};
use std::ptr;
use std::slice;
use std::str;

#[cfg(test)]
#[path = "ffi/tests.rs"]
mod tests;

#[cfg(feature = "c04-runtime-gpu-fixture-test")]
mod fixtures;
mod surface;

const ERR_ARGUMENT: i32 = -1;
const ERR_OUTPUT: i32 = -3;

#[unsafe(no_mangle)]
/// 전용 V8 세션을 생성하고 C04.10 GPU paint profile을 사용합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new() {
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
/// UTF-8 root HTML을 파싱해 기본 GPU runtime 렌더 host를 생성합니다.
///
/// # Safety
/// `source`가 null이 아니면 `source_length` 바이트 동안 읽을 수 있어야 합니다. `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다. 호출이 끝날 때까지 두 버퍼는 유효해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_with_root_html(
    source: *const u8,
    source_length: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    if source.is_null() || source_length == 0 {
        crate::write_report(
            output,
            output_capacity,
            "root HTML bytes가 비어 있거나 null입니다",
        );
        return ptr::null_mut();
    }
    if source_length > spinon_runtime::ROOT_HTML_SOURCE_MAX_BYTES {
        crate::write_report(
            output,
            output_capacity,
            &format!(
                "root HTML은 UTF-8 기준 {}바이트까지 허용합니다",
                spinon_runtime::ROOT_HTML_SOURCE_MAX_BYTES
            ),
        );
        return ptr::null_mut();
    }
    let source_bytes = unsafe { slice::from_raw_parts(source, source_length) };
    let source = match str::from_utf8(source_bytes) {
        Ok(source) => source,
        Err(error) => {
            crate::write_report(
                output,
                output_capacity,
                &format!("root HTML UTF-8이 잘못되었습니다: {error}"),
            );
            return ptr::null_mut();
        }
    };
    let (host, report) = match RuntimeGpuHost::new_with_root_html(source) {
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
/// 화면 환경·surface 변경을 큐에 넣기 전에 현재 장면을 비동기로 무효화합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. 같은 host의 해제와 경합시키면 안 됩니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_begin_presentation_update(
    host: *mut SpinonRuntimeGpuHost,
) -> u64 {
    raw_host(host)
        .and_then(|host| host.begin_presentation_update().ok())
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
/// CSS viewport와 색상 체계를 설정하고 해당 장면 계산이 끝날 때까지 기다립니다.
/// 제한 시간은 CSS 레이아웃 결과 대기에만 적용하며, V8 JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. 이 함수를 기다림 허용 background executor에서
/// 호출하고, 같은 host 해제와 경합시키면 안 됩니다. `output`은 쓰기 가능해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_set_environment(
    host: *mut SpinonRuntimeGpuHost,
    width_css_px: f32,
    height_css_px: f32,
    device_scale_factor: f32,
    dark: i32,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || !matches!(dark, 0 | 1) {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let result = host.set_environment(
        width_css_px,
        height_css_px,
        device_scale_factor,
        dark == 1,
        layout_timeout_millis,
    );
    write_host_coded_result(result, output, output_capacity)
}

#[unsafe(no_mangle)]
/// JavaScript를 동기 평가하고 같은 revision tuple의 layout 장면을 발행합니다.
/// 제한 시간은 평가 후 CSS 레이아웃 결과 대기에만 적용하며, JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. `source`는 NUL 종료 UTF-8이어야 합니다. 이 함수를
/// background executor에서 호출하고, 같은 host 해제와 경합시키면 안 됩니다.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval(
    host: *mut SpinonRuntimeGpuHost,
    source: *const c_char,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if source.is_null() || output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let source = match unsafe { CStr::from_ptr(source) }.to_str() {
        Ok(source) => source,
        Err(_) => return ERR_ARGUMENT,
    };
    match host.eval(source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

fn write_host_coded_result(
    result: Result<String, (i32, String)>,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    match result {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

fn renderer_result(
    result: Result<String, super::RuntimeRendererError>,
) -> Result<String, (i32, String)> {
    result.map_err(|error| {
        let status = if matches!(error, super::RuntimeRendererError::InvalidArgument(_)) {
            ERR_ARGUMENT
        } else {
            -11
        };
        (status, error.to_string())
    })
}

fn write_host_status(
    status: i32,
    report: String,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if !crate::write_report(output, output_capacity, &report) {
        return ERR_OUTPUT;
    }
    status
}
