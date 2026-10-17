use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c15_3::RUNTIME_CSS_SOURCE;
use std::{ffi::c_char, ptr};

#[unsafe(no_mangle)]
/// C15.3 fixed-font WGPU glyph paint fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c15_3_glyph_paint_fixture() {
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
/// 고정 C19.2 box scene과 C15 glyph raster를 실제 WGPU scene으로 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C15.3 fixture host여야 합니다. 호출 중 해제하거나 같은 host를
/// 동시에 호출하면 안 되며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture(
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
    match host.eval(RUNTIME_CSS_SOURCE, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 다음 C15.3 draw에서 실제 drawable의 고정 overflow/scroll 표본 6개를 읽도록 요청합니다.
///
/// # Safety
/// `host`는 살아 있는 C15.3 readback fixture host여야 합니다. `sample_xy`는 정확히 12개의
/// 읽기 가능한 `u32`를 호출 중 유지해야 합니다. 호출 중 host 해제나 동시 호출은 허용하지
/// 않으며 `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c15_3_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if sample_count != 6 {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C15.3 surface 표본 수는 정확히 6개여야 합니다: {sample_count}"),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C15.3 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(value_count) = sample_count.checked_mul(2) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C15.3 surface 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 표본 수를 6개로 제한했고 포인터 범위는 호출자 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    let points = values
        .chunks_exact(2)
        .map(|point| (point[0], point[1]))
        .collect::<Vec<_>>();
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C15.3 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture,
        spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture,
        spinon_runtime_gpu_host_request_c15_3_surface_pixels_for_test,
    };
    use std::ptr;

    #[test]
    fn c15_3_fixture_ffi_rejects_null_outputs_and_hosts() {
        let mut output = [0_i8; 128];
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture(ptr::null_mut(), output.len())
            }
            .is_null()
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture(
                    ptr::null_mut(),
                    100,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c15_3_surface_pixels_for_test(
                    ptr::null_mut(),
                    ptr::null(),
                    3,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
    }

    #[test]
    fn c15_3_surface_readback_accepts_only_the_six_bounded_fixture_samples() {
        let mut output = [0_i8; 128];
        let points = [0_u32; 14];
        for sample_count in [4, 7] {
            assert_eq!(
                unsafe {
                    spinon_runtime_gpu_host_request_c15_3_surface_pixels_for_test(
                        ptr::null_mut(),
                        points.as_ptr(),
                        sample_count,
                        output.as_mut_ptr(),
                        output.len(),
                    )
                },
                -1
            );
        }
    }
}
