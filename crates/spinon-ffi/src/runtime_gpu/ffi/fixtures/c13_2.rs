use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c13_2_scroll::{
    RUNTIME_CSS_C13_2_ROOT_BODY_HIDDEN_SOURCE, RUNTIME_CSS_C13_2_ROOT_BODY_SOURCE,
    RUNTIME_CSS_C13_2_SCROLL_FIXTURE_SOURCE,
};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C13.2 opt-in scroll state·paint·hit runtime fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_2_scroll_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_2_scroll_fixture() {
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
/// C13.2 fixture를 실제 V8·Stylo·Taffy·render snapshot에서 평가하고 offset을 적용합니다.
///
/// # Safety
/// `host`는 살아 있는 C13.2 scroll fixture host여야 하며 호출 중 해제하거나 동시 호출하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || layout_timeout_millis == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.2 scroll fixture host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.eval(RUNTIME_CSS_C13_2_ROOT_BODY_SOURCE, layout_timeout_millis) {
        Ok(root_report) if root_report.contains("layout=ready") => {
            let root_scroll = match host.apply_c13_2_root_body_propagation_offset() {
                Ok(report) => report,
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            let hidden_report = match host.eval(
                RUNTIME_CSS_C13_2_ROOT_BODY_HIDDEN_SOURCE,
                layout_timeout_millis,
            ) {
                Ok(report) if report.contains("layout=ready") => report,
                Ok(report) => {
                    return write_host_status(
                        -10,
                        format!("C13.2 hidden root/body layout가 준비되지 않았습니다: {report}"),
                        output,
                        output_capacity,
                    );
                }
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            let hidden_scroll = match host.apply_c13_2_root_body_hidden_body_offset() {
                Ok(report) => report,
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            host.reset_c13_2_scroll_state_for_fixture_transition();
            let initial_report = match host.eval(
                RUNTIME_CSS_C13_2_SCROLL_FIXTURE_SOURCE,
                layout_timeout_millis,
            ) {
                Ok(report) if report.contains("layout=ready") => report,
                Ok(report) => {
                    return write_host_status(
                        -10,
                        format!("C13.2 scroll matrix layout가 준비되지 않았습니다: {report}"),
                        output,
                        output_capacity,
                    );
                }
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            let scroll_report = match host.apply_c13_2_scroll_fixture_offset() {
                Ok(report) => report,
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            let lifecycle_report = match host.verify_c13_2_scroll_lifecycle(layout_timeout_millis) {
                Ok(report) => report,
                Err((status, report)) => {
                    return write_host_status(status, report, output, output_capacity);
                }
            };
            write_host_status(
                0,
                format!(
                    "{root_report} {root_scroll} {hidden_report} {hidden_scroll} {initial_report} {scroll_report} {lifecycle_report}"
                ),
                output,
                output_capacity,
            )
        }
        Ok(report) => write_host_status(
            -10,
            format!("C13.2 visible root/body layout가 준비되지 않았습니다: {report}"),
            output,
            output_capacity,
        ),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_eval_c13_2_scroll_fixture,
        spinon_runtime_gpu_host_new_c13_2_scroll_fixture,
    };
    use std::{ffi::c_char, ptr};

    #[test]
    fn c13_2_ffi_rejects_null_host_and_invalid_output_before_runtime_access() {
        let mut output = [0 as c_char; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
                    ptr::null_mut(),
                    1000,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_2_scroll_fixture(
                    ptr::null_mut(),
                    1000,
                    ptr::null_mut(),
                    0,
                )
            },
            -1
        );
        assert!(
            unsafe { spinon_runtime_gpu_host_new_c13_2_scroll_fixture(ptr::null_mut(), 0) }
                .is_null()
        );
    }
}
