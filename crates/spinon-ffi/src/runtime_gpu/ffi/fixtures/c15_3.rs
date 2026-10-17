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

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture,
        spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture,
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
    }
}
