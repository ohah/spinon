use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C13.2 Metal 표면 pixel readback을 활성화한 opt-in fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_2_scroll_surface_pixel_readback_fixture() {
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
/// C13.2 Metal drawable에서 고정된 다섯 표본의 다음 draw readback을 예약합니다.
///
/// # Safety
/// `host`는 살아 있는 C13.2 surface-pixel host여야 합니다. `sample_xy`는 정확히
/// 10개의 읽기 가능한 `u32`를 호출 중 유지해야 하며, 같은 host의 호출·해제와 경합시키면
/// 안 됩니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if sample_count != 5 {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C13.2 surface 표본 수는 정확히 5개여야 합니다: {sample_count}"),
            output,
            output_capacity,
        );
    }
    let Some(value_count) = sample_count.checked_mul(2) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.2 surface 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.2 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    // SAFETY: 표본 수·개수 곱셈·null 입력은 위에서 확인했고, 포인터의 실제 메모리
    // 유효 범위는 FFI 호출자가 제공한 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    for pair_index in 0..sample_count {
        let pair = &values[pair_index * 2..pair_index * 2 + 2];
        if values[..pair_index * 2]
            .chunks_exact(2)
            .any(|previous| previous == pair)
        {
            return write_host_status(
                ERR_ARGUMENT,
                "C13.2 surface 표본 좌표가 중복됐습니다".to_owned(),
                output,
                output_capacity,
            );
        }
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.2 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    let mut points = [(0, 0); 5];
    for (point, pair) in points.iter_mut().zip(values.chunks_exact(2)) {
        *point = (pair[0], pair[1]);
    }
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture,
        spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test,
    };
    use std::{ffi::c_char, ptr};

    #[test]
    fn c13_2_pixel_ffi_rejects_null_host_and_invalid_output_before_runtime_access() {
        let mut output = [0 as c_char; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
                    ptr::null_mut(),
                    ptr::null(),
                    5,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
                    ptr::null_mut(),
                    ptr::null(),
                    5,
                    ptr::null_mut(),
                    0,
                )
            },
            -1
        );
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c13_2_scroll_surface_pixel_readback_fixture(
                    ptr::null_mut(),
                    0,
                )
            }
            .is_null()
        );
    }

    #[test]
    fn c13_2_pixel_request_rejects_bad_batch_before_host_access() {
        let mut output = [0 as c_char; 256];
        for count in [0, 4, 6, usize::MAX] {
            assert_eq!(
                unsafe {
                    spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
                        ptr::null_mut(),
                        ptr::null(),
                        count,
                        output.as_mut_ptr(),
                        output.len(),
                    )
                },
                -1,
                "count={count}"
            );
        }
    }

    #[test]
    fn c13_2_pixel_request_rejects_duplicate_coordinates_before_host_access() {
        let points = [10, 10, 20, 20, 30, 30, 40, 40, 10, 10];
        let mut output = [0 as c_char; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_2_surface_pixels_for_test(
                    ptr::null_mut(),
                    points.as_ptr(),
                    5,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        let message = unsafe { std::ffi::CStr::from_ptr(output.as_ptr()) }.to_string_lossy();
        assert!(message.contains("좌표가 중복됐습니다"), "{message}");
    }
}
