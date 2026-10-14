use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c13_1::{
    RUNTIME_CSS_C13_1_BORDER_FIXTURE_SOURCE,
    RUNTIME_CSS_C13_1_COMPACT_SURFACE_MATRIX_FIXTURE_SOURCE,
    RUNTIME_CSS_C13_1_OVERFLOW_CLIP_FIXTURE_SOURCE,
};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C13.1 overflow clip V8·Stylo·Taffy·WGPU fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_1_overflow_clip_fixture() {
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
/// C13.1 paint/hit border-strip V8·Stylo·Taffy·WGPU fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_1_overflow_clip_border_fixture() {
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
/// C13.1 compact 13-case surface matrix fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_1_compact_surface_matrix_fixture() {
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
/// iOS Simulator Metal pixel-center readback이 켜진 C13.1 compact fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) =
        match RuntimeGpuHost::new_c13_1_compact_surface_matrix_pixel_readback_fixture() {
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
/// C13.1 Metal 표면에서 한 batch의 RGBA sample readback을 예약합니다. 다음 draw report에 표본이 포함됩니다.
///
/// # Safety
/// `host`는 살아 있는 C13.1 pixel-readback host여야 합니다. `sample_xy`는 `sample_count * 2`
///개의 읽기 가능한 `u32`를 호출 중 유지해야 하며, 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(value_count) = sample_count.checked_mul(2) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.1 surface 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if !(1..=64).contains(&sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.1 surface 표본 수는 1–64개여야 합니다".to_owned(),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.1 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.1 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 곱셈 overflow·표본 수 범위·null 입력을 위에서 확인했고, 포인터의 실제
    // 메모리 유효 범위는 FFI 호출자가 제공한 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    let mut points = Vec::new();
    if let Err(error) = points.try_reserve_exact(sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C13.1 surface 표본 좌표 할당 실패: {error}"),
            output,
            output_capacity,
        );
    }
    points.extend(values.chunks_exact(2).map(|pair| (pair[0], pair[1])));
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C13.1 고정 scene을 실제 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C13.1 overflow clip host여야 하며 호출 중 해제하거나 동시 호출하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
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
        RUNTIME_CSS_C13_1_OVERFLOW_CLIP_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C13.1 paint/hit border-strip scene을 실제 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C13.1 border-strip fixture host여야 하며 호출 중 해제하거나 동시 호출하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture(
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
        RUNTIME_CSS_C13_1_BORDER_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C13.1 compact 13-case source를 runtime에서 평가하고 Chrome hit oracle을 대조합니다.
///
/// # Safety
/// `host`는 살아 있는 C13.1 compact surface matrix host여야 하며 호출 중 해제하거나 동시 호출하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture(
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
        RUNTIME_CSS_C13_1_COMPACT_SURFACE_MATRIX_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture,
        spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture,
        spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture,
        spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture,
        spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture,
        spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture,
        spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture,
        spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test,
    };
    use std::ptr;

    #[test]
    fn c13_1_ffi_rejects_null_host_and_invalid_output_before_runtime_access() {
        let mut output = [0i8; 1];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
                    ptr::null_mut(),
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture(
                    ptr::null_mut(),
                    0,
                    output.as_mut_ptr(),
                    0,
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture(
                    ptr::null_mut(),
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert!(
            unsafe { spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(ptr::null_mut(), 0) }
                .is_null()
        );
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture(ptr::null_mut(), 0)
            }
            .is_null()
        );
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture(ptr::null_mut(), 0)
            }
            .is_null()
        );
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture(
                    ptr::null_mut(),
                    0,
                )
            }
            .is_null()
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture(
                    ptr::null_mut(),
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
    }

    #[test]
    fn c13_1_surface_pixel_request_rejects_invalid_batch_before_host_access() {
        let mut output = [0i8; 128];
        let coordinate = [0u32; 2];
        for (coordinates, count, expected) in [
            (coordinate.as_ptr(), 0usize, "1–64"),
            (coordinate.as_ptr(), 65usize, "1–64"),
            (ptr::null(), 1usize, "null"),
        ] {
            let status = unsafe {
                spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinates,
                    count,
                    output.as_mut_ptr(),
                    output.len(),
                )
            };
            assert_eq!(status, super::super::super::super::ERR_ARGUMENT);
            let report = unsafe { std::ffi::CStr::from_ptr(output.as_ptr()) }
                .to_str()
                .expect("UTF-8 error report");
            assert!(report.contains(expected), "unexpected report: {report}");
        }

        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinate.as_ptr(),
                    1,
                    ptr::null_mut(),
                    0,
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinate.as_ptr(),
                    1,
                    output.as_mut_ptr(),
                    1,
                )
            },
            super::super::super::ERR_OUTPUT
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinate.as_ptr(),
                    usize::MAX,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
    }
}
