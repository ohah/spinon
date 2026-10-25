use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C12.5 top-only sticky runtime fixture용 opt-in V8 GPU host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_5_sticky_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_5_sticky_fixture() {
        Ok(value) => value,
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
/// C12.5 fixture를 V8·Stylo·Taffy·WGPU에서 실행하고 Chrome geometry 기준을 검사합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.5 fixture host여야 하며 호출 중 해제하거나 동시 호출하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_5_sticky_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || layout_timeout_millis == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.5 sticky fixture host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.eval_c12_5_sticky_fixture(layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.5 fixture용 다음 Metal drawable draw에서 고정된 두 색상 표본을 읽습니다.
///
/// # Safety
/// `host`는 살아 있는 C12.5 sticky fixture host여야 합니다. `sample_xy`는 정확히
/// 4개의 읽기 가능한 `u32`를 호출 중 유지해야 하며, 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c12_5_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if sample_count != 2 {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C12.5 Metal 표본 수는 정확히 2개여야 합니다: {sample_count}"),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.5 Metal 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.5 sticky fixture host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 위에서 고정 길이 두 좌표와 null을 검증했으며 실제 포인터 범위는 caller 계약이다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, 4) };
    if values[0] == values[2] && values[1] == values[3] {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.5 Metal 표본 좌표가 중복됐습니다".to_owned(),
            output,
            output_capacity,
        );
    }
    match host
        .request_surface_pixel_readback_for_test(&[(values[0], values[1]), (values[2], values[3])])
    {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c12_5_sticky_fixture,
        spinon_runtime_gpu_host_new_c12_5_sticky_fixture,
        spinon_runtime_gpu_host_request_c12_5_surface_pixels_for_test,
    };
    use std::{ffi::c_char, ptr};

    #[test]
    fn c12_5_ffi_rejects_null_host_and_invalid_output() {
        let mut output = [0 as c_char; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c12_5_sticky_fixture(
                    ptr::null_mut(),
                    1000,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c12_5_surface_pixels_for_test(
                    ptr::null_mut(),
                    ptr::null(),
                    2,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c12_5_sticky_fixture(
                    ptr::null_mut(),
                    0,
                    ptr::null_mut(),
                    0,
                )
            },
            -1
        );
        assert!(
            unsafe { spinon_runtime_gpu_host_new_c12_5_sticky_fixture(ptr::null_mut(), 0) }
                .is_null()
        );
    }
}
