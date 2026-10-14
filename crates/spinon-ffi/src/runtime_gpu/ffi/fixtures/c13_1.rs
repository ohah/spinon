use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c13_1::RUNTIME_CSS_C13_1_OVERFLOW_CLIP_FIXTURE_SOURCE;
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

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture,
        spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture,
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
        assert!(
            unsafe { spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture(ptr::null_mut(), 0) }
                .is_null()
        );
    }
}
