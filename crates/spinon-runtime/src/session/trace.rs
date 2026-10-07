use std::ffi::CStr;

#[cfg(target_os = "android")]
#[link(name = "android")]
unsafe extern "C" {
    fn ATrace_beginSection(name: *const std::ffi::c_char);
    fn ATrace_endSection();
}

pub(super) struct TraceSection;

impl TraceSection {
    pub(super) fn new(name: &'static CStr) -> Self {
        #[cfg(target_os = "android")]
        unsafe {
            ATrace_beginSection(name.as_ptr());
        }

        #[cfg(not(target_os = "android"))]
        let _ = name;

        Self
    }
}

impl Drop for TraceSection {
    fn drop(&mut self) {
        #[cfg(target_os = "android")]
        unsafe {
            ATrace_endSection();
        }
    }
}
