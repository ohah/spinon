use super::super::super::{
    RuntimeGpuHost, SpinonRuntimeGpuHost, c11_4_grid_track_sizing_runtime_source, raw_host,
};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::{CStr, c_char};
use std::ptr;

#[unsafe(no_mangle)]
/// C11.4 intrinsic track-sizing fixture 전용 V8·Stylo·Taffy·WGPU host입니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_4_grid_track_sizing_fixture() {
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
/// 고정 C11.4 scene ID 하나를 실제 V8·Stylo·Taffy·WGPU 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C11.4 track-sizing host여야 합니다. `scene_id`는 읽을 수 있는 NUL
/// 종료 UTF-8 문자열이어야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture(
    host: *mut SpinonRuntimeGpuHost,
    scene_id: *const c_char,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || scene_id.is_null() {
        return ERR_ARGUMENT;
    }
    let scene_id = unsafe { CStr::from_ptr(scene_id) };
    if scene_id.to_bytes().len() > 64 {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.4 scene ID가 너무 깁니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Ok(scene_id) = scene_id.to_str() else {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.4 scene ID가 UTF-8이 아닙니다".to_owned(),
            output,
            output_capacity,
        );
    };
    let source = match c11_4_grid_track_sizing_runtime_source(scene_id) {
        Ok(source) => source,
        Err(error) => return write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    };
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(feature = "c11-grid-metal-surface-pixels-test")]
#[unsafe(no_mangle)]
/// C11.4 fixture host의 다음 WGPU draw surface에서 여러 픽셀을 요청합니다.
/// `sample_xy`는 `[x0, y0, x1, y1, ...]` 형식입니다.
///
/// # Safety
/// `host`는 살아 있는 C11.4 track-sizing host여야 합니다. `sample_xy`는
/// `sample_count * 2`개의 `u32`를 읽을 수 있어야 하며 호출 중 변경되면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다. 같은 host의 다른 호출·해제와
/// 경합시키면 안 되고 renderer 호출과 같은 직렬 실행 경계에서 호출해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c11_4_grid_surface_pixels_for_test(
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
            "C11.4 Grid 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if !(1..=64).contains(&sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.4 Grid 표본 수는 1–64개여야 합니다".to_owned(),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.4 Grid 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.4 Grid surface-pixel host가 null입니다".to_owned(),
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
            format!("C11.4 Grid 표본 좌표 할당 실패: {error}"),
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

#[cfg(test)]
mod tests {
    use super::spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture;
    use std::ffi::CString;

    #[test]
    fn c11_4_eval_rejects_untrusted_scene_ids_and_bad_arguments() {
        let mut output = [0_i8; 256];
        let unknown = CString::new("unknown;throw 1").unwrap();
        // SAFETY: output is writable and scene ID is a valid NUL-terminated CString.
        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture(
                std::ptr::null_mut(),
                unknown.as_ptr(),
                100,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert_eq!(status, -1);

        // SAFETY: null pointers are explicitly rejected before they are dereferenced.
        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture(
                std::ptr::null_mut(),
                std::ptr::null(),
                100,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert_eq!(status, -1);
    }
}
