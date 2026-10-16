use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c18_3::RUNTIME_CSS_C18_3_OBJECT_FIT_POSITION_FIXTURE_SOURCE;
use std::{ffi::c_char, ptr};

#[unsafe(no_mangle)]
/// C18.3 object-fit/object-position fixture를 실행하는 opt-in V8·WGPU host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c18_3_object_fit_position_fixture() {
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
/// 고정 C18.3 DOM fixture를 V8·Stylo·Taffy·WGPU scene으로 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C18.3 fixture host여야 합니다. 호출 중 해제하거나 같은 host를
/// 동시에 호출하면 안 되며, 기다림을 허용하는 background executor에서 호출해야 합니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c18_3_object_fit_position_fixture(
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
        RUNTIME_CSS_C18_3_OBJECT_FIT_POSITION_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 다음 C18.3 draw 뒤 drawable 표본 19개를 읽도록 요청합니다.
///
/// # Safety
/// `host`는 살아 있는 C18.3 object-fit fixture host여야 합니다. `sample_xy`는 정확히
/// 38개의 읽기 가능한 `u32`를 호출 중 유지해야 합니다. 호출 중 host 해제나 동시 호출은
/// 허용하지 않습니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if sample_count != 19 {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C18.3 surface 표본 수는 정확히 19개여야 합니다: {sample_count}"),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C18.3 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C18.3 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    // SAFETY: 고정 표본 수로 읽을 길이를 확인했고 실제 포인터 범위는 호출자 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, 38) };
    let mut points = [(0, 0); 19];
    for (index, pair) in values.chunks_exact(2).enumerate() {
        if values[..index * 2]
            .chunks_exact(2)
            .any(|previous| previous == pair)
        {
            return write_host_status(
                ERR_ARGUMENT,
                "C18.3 surface 표본 좌표가 중복됐습니다".to_owned(),
                output,
                output_capacity,
            );
        }
        points[index] = (pair[0], pair[1]);
    }
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture,
        spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test,
    };
    use std::{ffi::c_char, ptr};

    #[test]
    fn c18_3_pixel_ffi_rejects_null_and_wrong_sample_count_before_host_access() {
        let mut output = [0 as c_char; 256];
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(ptr::null_mut(), 0)
            }
            .is_null()
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test(
                    ptr::null_mut(),
                    ptr::null(),
                    18,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
    }
}
