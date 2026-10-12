use super::super::{SpinonRuntimeGpuHost, c11_2_1_grid_placement_runtime_source, raw_host};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::{CStr, c_char};

#[unsafe(no_mangle)]
/// 고정 C11.2.1 scene ID 하나를 V8 document mutation부터 WGPU scene까지 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C11.2.1 Grid placement host여야 합니다. `scene_id`는 읽을 수 있는
/// NUL 종료 UTF-8 문자열이어야 하며, null이면 안 됩니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity`
/// 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
    host: *mut SpinonRuntimeGpuHost,
    scene_id: *const c_char,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if scene_id.is_null() {
        return ERR_ARGUMENT;
    }
    let scene_id = unsafe { CStr::from_ptr(scene_id) };
    if scene_id.to_bytes().len() > 64 {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.2.1 scene ID가 너무 깁니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Ok(scene_id) = scene_id.to_str() else {
        return write_host_status(
            ERR_ARGUMENT,
            "C11.2.1 scene ID가 UTF-8이 아닙니다".to_owned(),
            output,
            output_capacity,
        );
    };
    let source = match c11_2_1_grid_placement_runtime_source(scene_id) {
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

#[cfg(test)]
mod tests {
    use super::spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture;
    use std::ffi::{CStr, CString, c_char};
    use std::ptr;

    #[test]
    fn fixture_eval_rejects_null_host_and_invalid_output() {
        let mut output = [0 as c_char; 32];
        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
                ptr::null_mut(),
                ptr::null(),
                10_000,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert_eq!(status, -1);

        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
                ptr::null_mut(),
                ptr::null(),
                10_000,
                ptr::null_mut(),
                0,
            )
        };
        assert_eq!(status, -1);
        assert!(
            unsafe { CStr::from_ptr(output.as_ptr()) }
                .to_bytes()
                .is_empty()
        );
    }

    #[test]
    fn fixture_eval_rejects_unknown_scene_without_running_javascript() {
        let scene_id = CString::new("numeric-lines;throw new Error('injected')").expect("CString");
        let mut output = [0 as c_char; 256];
        let status = unsafe {
            spinon_runtime_gpu_host_eval_c11_2_1_grid_placement_fixture(
                ptr::null_mut(),
                scene_id.as_ptr(),
                10_000,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert_eq!(status, -1);
        assert!(
            unsafe { CStr::from_ptr(output.as_ptr()) }
                .to_string_lossy()
                .contains("알 수 없는 C11.2.1 scene ID")
        );
    }
}
