use std::ffi::CStr;
use std::sync::mpsc::SyncSender;

#[cfg(target_os = "android")]
#[link(name = "android")]
unsafe extern "C" {
    fn ATrace_beginSection(name: *const std::ffi::c_char);
    fn ATrace_endSection();
    fn ATrace_beginAsyncSection(name: *const std::ffi::c_char, cookie: i32);
    fn ATrace_endAsyncSection(name: *const std::ffi::c_char, cookie: i32);
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

pub(super) fn finish_reply_handoff(cookie: u32) {
    #[cfg(target_os = "android")]
    unsafe {
        ATrace_endAsyncSection(c"SpinonR05:reply-handoff".as_ptr(), cookie as i32);
    }

    #[cfg(not(target_os = "android"))]
    let _ = cookie;
}

pub(super) fn send_reply<T>(sender: SyncSender<T>, response: T, cookie: u32) {
    let _send_trace = TraceSection::new(c"SpinonR05:reply-send");
    #[cfg(target_os = "android")]
    unsafe {
        ATrace_beginAsyncSection(c"SpinonR05:reply-handoff".as_ptr(), cookie as i32);
    }

    #[cfg(not(target_os = "android"))]
    let _ = cookie;

    if sender.send(response).is_err() {
        finish_reply_handoff(cookie);
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
