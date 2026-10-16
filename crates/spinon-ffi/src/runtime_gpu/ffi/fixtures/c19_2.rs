use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c19_2::RUNTIME_CSS_C19_2_BOX_PAINT_OPACITY_FIXTURE_SOURCE;
use std::{collections::BTreeSet, ffi::c_char, ptr};

const EXPECTED_SAMPLE_POINTS: [(u32, u32); 16] = [
    (340, 232),
    (26, 26),
    (64, 26),
    (80, 26),
    (128, 24),
    (168, 24),
    (208, 24),
    (16, 72),
    (84, 90),
    (68, 68),
    (124, 68),
    (202, 76),
    (212, 76),
    (220, 76),
    (234, 58),
    (256, 80),
];

fn recognized_sample_scale(points: &[(u32, u32)]) -> Option<f64> {
    if points.len() != EXPECTED_SAMPLE_POINTS.len() {
        return None;
    }
    let actual = points.iter().copied().collect::<BTreeSet<_>>();
    if actual.len() != points.len() {
        return None;
    }
    let mut scale_min = 1.0_f64;
    let mut scale_max = 8.0_f64;
    for ((css_x, css_y), (actual_x, actual_y)) in EXPECTED_SAMPLE_POINTS.iter().zip(points) {
        let x_center = f64::from(*css_x) + 0.5;
        let y_center = f64::from(*css_y) + 0.5;
        scale_min = scale_min
            .max(f64::from(*actual_x) / x_center)
            .max(f64::from(*actual_y) / y_center);
        scale_max = scale_max
            .min((f64::from(*actual_x) + 1.0) / x_center)
            .min((f64::from(*actual_y) + 1.0) / y_center);
    }
    if scale_min >= scale_max {
        if scale_min != 8.0 || scale_max != 8.0 {
            return None;
        }
    }
    let scale = if scale_min == scale_max {
        scale_min
    } else {
        scale_min + (scale_max - scale_min) / 2.0
    };
    let expected = EXPECTED_SAMPLE_POINTS
        .iter()
        .map(|(x, y)| {
            (
                ((f64::from(*x) + 0.5) * scale).floor() as u32,
                ((f64::from(*y) + 0.5) * scale).floor() as u32,
            )
        })
        .collect::<BTreeSet<_>>();
    (actual == expected).then_some(scale)
}

#[unsafe(no_mangle)]
/// C19.2 box paint와 opacity 고정 fixture를 실행하는 opt-in runtime GPU host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c19_2_box_paint_opacity_fixture() {
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
/// 고정 C19.2 DOM fixture를 V8·Stylo·Taffy·WGPU scene으로 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C19.2 fixture host여야 합니다. 호출 중 해제하거나 같은 host를
/// 동시에 호출하면 안 되며, 기다림을 허용하는 background executor에서 호출해야 합니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture(
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
        RUNTIME_CSS_C19_2_BOX_PAINT_OPACITY_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 다음 C19.2 draw 뒤 고정 16개 surface 표본을 읽도록 요청합니다.
///
/// # Safety
/// `host`는 살아 있는 C19.2 pixel-readback host여야 합니다. `sample_xy`는 정확히 32개의
/// 읽기 가능한 `u32`를 호출 중 유지해야 합니다. 좌표는 고정된 CSS 표본을 균일 배율 1~8로
/// drawable pixel center에 변환한 값이어야 합니다. 호출 중 host 해제나 동시 호출은 허용하지
/// 않습니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if sample_count != EXPECTED_SAMPLE_POINTS.len() {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C19.2 surface 표본 수는 정확히 16개여야 합니다: {sample_count}"),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C19.2 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(value_count) = sample_count.checked_mul(2) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C19.2 surface 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 표본 수를 16으로 제한해 길이를 확인했고 실제 포인터 범위는 호출자 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    let mut points = Vec::new();
    if let Err(error) = points.try_reserve_exact(sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C19.2 surface 표본 좌표 할당 실패: {error}"),
            output,
            output_capacity,
        );
    }
    points.extend(values.chunks_exact(2).map(|pair| (pair[0], pair[1])));
    if points.iter().copied().collect::<BTreeSet<_>>().len() != points.len() {
        return write_host_status(
            ERR_ARGUMENT,
            "C19.2 surface 표본 좌표에 중복이 있습니다".to_owned(),
            output,
            output_capacity,
        );
    }
    if recognized_sample_scale(&points).is_none() {
        return write_host_status(
            ERR_ARGUMENT,
            "C19.2 surface 표본 좌표가 1~8 균일 배율의 고정 fixture와 다릅니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C19.2 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C19.2 scene을 offscreen WGPU texture에 다시 그려 fixed CSS pixel samples를 읽습니다.
///
/// # Safety
/// `host`는 살아 있는 C19.2 fixture host여야 합니다. `output`은 `output_capacity` 바이트를
/// 쓸 수 있어야 하며, 호출 중 host 해제나 같은 host의 동시 호출은 허용하지 않습니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_readback_c19_2_offscreen_for_test(
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
    let points = EXPECTED_SAMPLE_POINTS;
    let samples = match host.readback_c19_2_offscreen_samples(360, 240, &points) {
        Ok(samples) if samples.len() == points.len() => samples,
        Ok(samples) => {
            return write_host_status(
                ERR_ARGUMENT,
                format!("C19.2 offscreen 표본 수가 다릅니다: {}", samples.len()),
                output,
                output_capacity,
            );
        }
        Err(error) => {
            return write_host_status(ERR_ARGUMENT, error, output, output_capacity);
        }
    };
    let values = points
        .iter()
        .zip(samples)
        .map(|((x, y), rgba)| {
            serde_json::json!({
                "x": x,
                "y": y,
                "rgba": rgba,
            })
        })
        .collect::<Vec<_>>();
    match serde_json::to_string(&values) {
        Ok(json) => write_host_status(
            0,
            format!("offscreen_viewport_css_px=360x240 surface_pixel_samples_json={json}"),
            output,
            output_capacity,
        ),
        Err(error) => write_host_status(
            ERR_ARGUMENT,
            format!("C19.2 offscreen 표본 JSON 변환 실패: {error}"),
            output,
            output_capacity,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EXPECTED_SAMPLE_POINTS, recognized_sample_scale,
        spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture,
        spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture,
        spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test,
    };
    use std::ptr;

    #[test]
    fn c19_2_fixed_css_samples_accept_drawable_pixel_center_scales() {
        assert!(recognized_sample_scale(&EXPECTED_SAMPLE_POINTS).is_some());
        for scale in 1..=8 {
            let scaled = EXPECTED_SAMPLE_POINTS
                .iter()
                .map(|(x, y)| {
                    (
                        ((f64::from(*x) + 0.5) * f64::from(scale)).floor() as u32,
                        ((f64::from(*y) + 0.5) * f64::from(scale)).floor() as u32,
                    )
                })
                .collect::<Vec<_>>();
            assert!(recognized_sample_scale(&scaled).is_some());
        }
        let scaled = EXPECTED_SAMPLE_POINTS
            .iter()
            .map(|(x, y)| {
                (
                    ((f64::from(*x) + 0.5) * 2.625).floor() as u32,
                    ((f64::from(*y) + 0.5) * 2.625).floor() as u32,
                )
            })
            .collect::<Vec<_>>();
        assert!(recognized_sample_scale(&scaled).is_some());
        let mut mutated = EXPECTED_SAMPLE_POINTS;
        mutated[1].0 += 1;
        assert_eq!(recognized_sample_scale(&mutated), None);
    }

    #[test]
    fn c19_2_ffi_rejects_null_hosts_bad_outputs_and_mutated_sample_lists() {
        let mut output = [0i8; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c19_2_box_paint_opacity_fixture(
                    ptr::null_mut(),
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert!(
            unsafe {
                spinon_runtime_gpu_host_new_c19_2_box_paint_opacity_fixture(ptr::null_mut(), 0)
            }
            .is_null()
        );

        let mut coordinates = [0u32; 32];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinates.as_ptr(),
                    15,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert!(
            std::ffi::CStr::from_bytes_until_nul(unsafe {
                std::slice::from_raw_parts(output.as_ptr().cast::<u8>(), output.len())
            })
            .expect("status output is NUL terminated")
            .to_string_lossy()
            .contains("정확히 16개")
        );

        coordinates.copy_from_slice(&[
            340, 232, 26, 26, 64, 26, 80, 26, 128, 24, 168, 24, 208, 24, 16, 72, 84, 90, 68, 68,
            124, 68, 202, 76, 212, 76, 220, 76, 234, 58, 234, 58,
        ]);
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c19_2_surface_pixels_for_test(
                    ptr::null_mut(),
                    coordinates.as_ptr(),
                    16,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            super::super::super::super::ERR_ARGUMENT
        );
        assert!(
            std::ffi::CStr::from_bytes_until_nul(unsafe {
                std::slice::from_raw_parts(output.as_ptr().cast::<u8>(), output.len())
            })
            .expect("status output is NUL terminated")
            .to_string_lossy()
            .contains("중복")
        );
    }
}
