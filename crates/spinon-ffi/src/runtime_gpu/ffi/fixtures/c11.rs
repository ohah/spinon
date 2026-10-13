use super::super::super::RuntimeGpuHost;
use super::super::super::{SpinonRuntimeGpuHost, c11_grid_runtime_scene, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C11.1 고정 Grid scene을 V8·Stylo·Taffy·WGPU runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C11.1 Grid runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c11_1_grid_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    eval_c11_grid_scene(host, 0, layout_timeout_millis, output, output_capacity)
}

#[unsafe(no_mangle)]
/// 고정 C11.1 Grid runtime matrix에서 검증된 scene index 하나를 평가합니다.
/// 임의 JavaScript 문자열이나 fixture 경로는 받지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 C11.1 Grid runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
    host: *mut SpinonRuntimeGpuHost,
    scene_index: u32,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    eval_c11_grid_scene(
        host,
        scene_index,
        layout_timeout_millis,
        output,
        output_capacity,
    )
}

fn eval_c11_grid_scene(
    host: *mut SpinonRuntimeGpuHost,
    scene_index: u32,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let (scene_id, source) = match c11_grid_runtime_scene(scene_index) {
        Ok(scene) => scene,
        Err(error) => {
            return write_host_status(
                ERR_ARGUMENT,
                format!("scene_index={scene_index} error={error}"),
                output,
                output_capacity,
            );
        }
    };
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(&source, layout_timeout_millis) {
        Ok(report) => write_host_status(
            0,
            format!("scene_id={scene_id} {report}"),
            output,
            output_capacity,
        ),
        Err((status, report)) => write_host_status(
            status,
            format!("scene_id={scene_id} {report}"),
            output,
            output_capacity,
        ),
    }
}

#[cfg(feature = "c11-grid-metal-surface-pixels-test")]
#[unsafe(no_mangle)]
/// 고정 C11.1 Grid 장면의 다음 draw surface에서 여러 표본을 요청합니다.
/// `sample_xy`는 `[x0, y0, x1, y1, ...]` 형식입니다.
///
/// # Safety
/// `host`는 살아 있는 C11.1 surface-pixel test host여야 합니다. `sample_xy`는
/// `sample_count * 2`개의 `u32`를 읽을 수 있어야 하고 호출 중 변경되면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다. 같은 host의 호출·해제와
/// 경합시키면 안 되며 renderer 호출과 같은 직렬 실행 경계에서 호출해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test(
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
            "C11 Grid 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if !(1..=64).contains(&sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            "C11 Grid 표본 수는 1–64개여야 합니다".to_owned(),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C11 Grid 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C11 Grid surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 호출자는 sample_count * 2개의 읽기 가능한 u32를 제공해야 하며, 위에서
    // 곱셈 overflow·count 범위·null 포인터를 확인했다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    let mut points = Vec::new();
    if let Err(error) = points.try_reserve_exact(sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C11 Grid 표본 좌표 할당 실패: {error}"),
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
/// C11.1 `display:grid` fixture 전용 V8·Stylo·Taffy·WGPU runtime host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_1_grid_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_1_grid_fixture() {
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
/// C11.2.1 numeric explicit Grid placement fixture 전용 V8·Stylo·Taffy·WGPU host입니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_2_1_grid_placement_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_2_1_grid_placement_fixture() {
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
/// C11.2.2 행 우선 sparse auto-placement fixture 전용 V8·Stylo·Taffy·WGPU host입니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_2_2_grid_auto_placement_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_2_2_grid_auto_placement_fixture() {
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

#[cfg(feature = "c11-grid-metal-surface-pixels-test")]
#[unsafe(no_mangle)]
/// C11.1 Grid surface pixel 검증 전용 test-hook host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_1_grid_surface_pixel_readback_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_1_grid_surface_pixel_readback_fixture() {
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

#[cfg(test)]
mod tests {
    use super::spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture;
    use std::ffi::{CStr, c_char};
    use std::ptr;

    #[test]
    fn scene_index_rejects_out_of_range_before_host_access() {
        for index in [10, u32::MAX] {
            let mut output = [0 as c_char; 256];
            let status = unsafe {
                spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
                    ptr::null_mut(),
                    index,
                    10_000,
                    output.as_mut_ptr(),
                    output.len(),
                )
            };
            assert_eq!(status, -1);
            let report = unsafe { CStr::from_ptr(output.as_ptr()) }.to_string_lossy();
            assert!(report.contains(&format!("scene_index={index}")), "{report}");
            assert!(report.contains("범위 초과"), "{report}");
        }
    }

    #[test]
    fn valid_scene_still_rejects_null_host() {
        let mut output = [0 as c_char; 32];
        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_1_grid_scene_fixture(
                ptr::null_mut(),
                0,
                10_000,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert_eq!(status, -1);
    }
}

#[cfg(all(test, feature = "c11-grid-metal-surface-pixels-test"))]
mod surface_pixel_tests {
    use super::spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test;
    use std::ffi::{CStr, c_char};
    use std::ptr;

    fn report(status: i32, output: &[c_char]) -> String {
        assert_eq!(status, -1);
        unsafe { CStr::from_ptr(output.as_ptr()) }
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn request_rejects_empty_oversized_overflow_and_null_coordinates() {
        for (coordinates, count, expected) in [
            (ptr::null(), 0, "1–64개"),
            (ptr::null(), 65, "1–64개"),
            (ptr::null(), usize::MAX, "범위를 넘었습니다"),
            (ptr::null(), 1, "null입니다"),
        ] {
            let mut output = [0 as c_char; 256];
            let status = unsafe {
                spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinates,
                    count,
                    output.as_mut_ptr(),
                    output.len(),
                )
            };
            assert!(report(status, &output).contains(expected));
        }
    }

    #[test]
    fn request_checks_host_for_one_or_sixty_four_valid_pairs() {
        for count in [1, 64] {
            let coordinates = vec![0_u32; count * 2];
            let mut output = [0 as c_char; 256];
            let status = unsafe {
                spinon_runtime_gpu_host_request_c11_1_grid_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinates.as_ptr(),
                    count,
                    output.as_mut_ptr(),
                    output.len(),
                )
            };
            assert!(report(status, &output).contains("host가 null입니다"));
        }
    }
}
