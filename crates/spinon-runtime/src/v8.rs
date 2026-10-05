use crate::host::{DocumentCollectCallback, DocumentCommitCallback, DocumentQueryCallback};
use std::ffi::{c_char, c_void};

#[repr(C)]
pub(crate) struct SpinonV8Runtime {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct SpinonDocumentCollectionStats {
    pub(crate) scan_count: u64,
    pub(crate) deferred_count: u64,
    pub(crate) scanned_handle_count: u64,
    pub(crate) live_handle_count: u64,
    pub(crate) empty_handle_count: u64,
    pub(crate) last_scan_start_ns: u64,
    pub(crate) last_scan_duration_us: u64,
    pub(crate) wrapper_root_buffer_bytes: u64,
    pub(crate) reclaimed_node_buffer_bytes: u64,
    pub(crate) runtime_poisoned: u32,
}

pub(crate) type NodeCallback = extern "C" fn(*mut c_void, i32, *const c_char);
pub(crate) type TextCallback = extern "C" fn(*mut c_void, *const c_char);

unsafe extern "C" {
    pub(crate) fn spinon_v8_runtime_new(
        node_callback: NodeCallback,
        text_callback: TextCallback,
        document_commit_callback: DocumentCommitCallback,
        document_query_callback: DocumentQueryCallback,
        document_collect_callback: DocumentCollectCallback,
        user_data: *mut c_void,
        document_user_data: *mut c_void,
    ) -> *mut SpinonV8Runtime;
    pub(crate) fn spinon_v8_runtime_eval(
        runtime: *mut SpinonV8Runtime,
        source: *const c_char,
    ) -> i32;
    pub(crate) fn spinon_v8_runtime_dispatch(runtime: *mut SpinonV8Runtime, node_id: i32) -> i32;
    pub(crate) fn spinon_v8_runtime_notify_memory_pressure(
        runtime: *mut SpinonV8Runtime,
        level: i32,
    ) -> i32;
    pub(crate) fn spinon_v8_runtime_last_error(runtime: *mut SpinonV8Runtime) -> *const c_char;
    pub(crate) fn spinon_v8_runtime_last_collection_error(
        runtime: *mut SpinonV8Runtime,
    ) -> *const c_char;
    pub(crate) fn spinon_v8_runtime_document_collection_stats(
        runtime: *mut SpinonV8Runtime,
        stats: *mut SpinonDocumentCollectionStats,
    );
    pub(crate) fn spinon_v8_runtime_was_terminated(runtime: *mut SpinonV8Runtime) -> i32;
    pub(crate) fn spinon_v8_runtime_free(runtime: *mut SpinonV8Runtime);
    pub(crate) fn spinon_v8_runtime_terminate(runtime: *mut SpinonV8Runtime);
    pub(crate) fn spinon_v8_runtime_cancel_termination(runtime: *mut SpinonV8Runtime);
    pub(crate) fn spinon_v8_current_thread_id() -> u64;
}
