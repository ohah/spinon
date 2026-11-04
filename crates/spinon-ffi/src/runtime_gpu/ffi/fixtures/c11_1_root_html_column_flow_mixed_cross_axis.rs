use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::{CStr, c_char};
use std::ptr;

#[unsafe(no_mangle)]
/// 고정 parsed Root HTML mixed column-flow 장면 하나를 실행하는 fixture 전용 host를 생성합니다.
///
/// # Safety
/// `scene_id`는 읽을 수 있는 NUL 종료 UTF-8 문자열이어야 합니다. `output`은
/// `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_1_root_html_column_flow_mixed_cross_axis_fixture(
    scene_id: *const c_char,
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if scene_id.is_null() || output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let scene_id = unsafe { CStr::from_ptr(scene_id) };
    if scene_id.to_bytes().len() > 64 {
        write_host_status(
            ERR_ARGUMENT,
            "C11.1 Root HTML mixed column-flow scene ID가 너무 깁니다".to_owned(),
            output,
            output_capacity,
        );
        return ptr::null_mut();
    }
    let Ok(scene_id) = scene_id.to_str() else {
        write_host_status(
            ERR_ARGUMENT,
            "C11.1 Root HTML mixed column-flow scene ID가 UTF-8이 아닙니다".to_owned(),
            output,
            output_capacity,
        );
        return ptr::null_mut();
    };
    let (host, report) =
        match RuntimeGpuHost::new_c11_1_root_html_column_flow_mixed_cross_axis_fixture(scene_id) {
            Ok(host) => host,
            Err(error) => {
                write_host_status(ERR_ARGUMENT, error, output, output_capacity);
                return ptr::null_mut();
            }
        };
    if write_host_status(0, report, output, output_capacity) != 0 {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[cfg(test)]
mod tests {
    use super::spinon_runtime_gpu_host_new_c11_1_root_html_column_flow_mixed_cross_axis_fixture;
    use std::ffi::{CStr, CString, c_char};
    use std::ptr;

    #[test]
    fn fixture_host_creation_rejects_missing_arguments_and_unlisted_scenes() {
        let mut output = [0 as c_char; 512];
        let missing = unsafe {
            spinon_runtime_gpu_host_new_c11_1_root_html_column_flow_mixed_cross_axis_fixture(
                ptr::null(),
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert!(missing.is_null());

        let unknown = CString::new("row-sparse-baseline").unwrap();
        let host = unsafe {
            spinon_runtime_gpu_host_new_c11_1_root_html_column_flow_mixed_cross_axis_fixture(
                unknown.as_ptr(),
                output.as_mut_ptr(),
                output.len(),
            )
        };
        assert!(host.is_null());
        let report = unsafe { CStr::from_ptr(output.as_ptr()) }.to_str().unwrap();
        assert!(report.contains("알 수 없는 C11.1 Root HTML mixed column-flow scene ID"));
    }
}
