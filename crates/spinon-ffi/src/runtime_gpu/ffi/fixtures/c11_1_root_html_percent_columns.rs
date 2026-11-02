use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C11.1 parsed Root HTML percentage implicit-column fixture 전용 host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c11_1_root_html_percent_columns_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c11_1_root_html_percent_columns_fixture() {
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
