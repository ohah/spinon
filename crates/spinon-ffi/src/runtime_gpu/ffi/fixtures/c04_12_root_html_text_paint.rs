use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

const TEXT_FIXTURE_HOST_ERROR: &str = "C04.12 text fixture host가 필요합니다";

fn text_fixture_host_error(is_text_fixture: bool) -> Option<&'static str> {
    (!is_text_fixture).then_some(TEXT_FIXTURE_HOST_ERROR)
}

#[unsafe(no_mangle)]
/// C04.12의 parsed root HTML direct-text paint fixture host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_root_html_text_paint_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_root_html_text_paint_fixture() {
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
/// 로드된 root HTML fixture를 변경하지 않고 text layout·glyph paint scene에 발행합니다.
///
/// # Safety
/// `host`는 살아 있는 C04.12 text fixture host여야 합니다. 다른 host 호출·해제와 경합하면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며 `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_root_html_text_paint_fixture(
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
    if let Some(error) = text_fixture_host_error(host.root_html_text_paint_fixture) {
        return write_host_status(ERR_ARGUMENT, error.to_owned(), output, output_capacity);
    }
    match host.eval("void 0", layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{TEXT_FIXTURE_HOST_ERROR, text_fixture_host_error};

    #[test]
    fn text_eval_requires_its_fixture_host_kind() {
        assert_eq!(
            text_fixture_host_error(false),
            Some(TEXT_FIXTURE_HOST_ERROR)
        );
        assert_eq!(text_fixture_host_error(true), None);
    }
}
