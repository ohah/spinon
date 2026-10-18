use super::spinon_runtime_gpu_host_new_with_root_html;
use std::ffi::CStr;

fn rejected_root_html(source: *const u8, source_length: usize) -> String {
    let mut output = [0_i8; 512];
    let host = unsafe {
        spinon_runtime_gpu_host_new_with_root_html(
            source,
            source_length,
            output.as_mut_ptr(),
            output.len(),
        )
    };
    assert!(host.is_null());
    unsafe { CStr::from_ptr(output.as_ptr()) }
        .to_str()
        .expect("FFI error report should be UTF-8")
        .to_owned()
}

#[test]
fn root_html_constructor_rejects_null_and_empty_sources() {
    assert!(rejected_root_html(std::ptr::null(), 1).contains("비어 있거나 null"));

    let empty = [];
    assert!(rejected_root_html(empty.as_ptr(), 0).contains("비어 있거나 null"));
}

#[test]
fn root_html_constructor_rejects_oversize_and_malformed_utf8_sources() {
    let oversized = vec![b' '; spinon_runtime::ROOT_HTML_SOURCE_MAX_BYTES + 1];
    assert!(rejected_root_html(oversized.as_ptr(), oversized.len(),).contains("바이트까지 허용"));

    let malformed = [0xff];
    assert!(rejected_root_html(malformed.as_ptr(), malformed.len()).contains("UTF-8이 잘못"));
}
