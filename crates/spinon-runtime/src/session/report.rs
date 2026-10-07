use super::SpinonDocumentCollectionStats;
use super::actor::CallbackState;

pub(super) struct OperationReport<'a> {
    pub(super) sequence: u64,
    pub(super) operation: &'a str,
    pub(super) status: i32,
    pub(super) caller_thread_id: u64,
    pub(super) owner_thread_id: u64,
    pub(super) callback_thread_id: u64,
    pub(super) submission_lock_wait_us: u128,
    pub(super) control_lock_wait_us: u128,
    pub(super) scheduler_lock_wait_us: u128,
    pub(super) queue_residence_us: u128,
    pub(super) v8_call_us: u128,
    pub(super) cancel_requested: bool,
    pub(super) callbacks: &'a CallbackState,
    pub(super) collection_error: &'a str,
    pub(super) collection_stats: SpinonDocumentCollectionStats,
    pub(super) error: &'a str,
}

pub(super) fn operation_report(report: OperationReport<'_>) -> String {
    let OperationReport {
        sequence,
        operation,
        status,
        caller_thread_id,
        owner_thread_id,
        callback_thread_id,
        submission_lock_wait_us,
        control_lock_wait_us,
        scheduler_lock_wait_us,
        queue_residence_us,
        v8_call_us,
        cancel_requested,
        callbacks,
        collection_error,
        collection_stats,
        error,
    } = report;
    let error = if error.is_empty() { "none" } else { error };
    let collection_error = if collection_error.is_empty() {
        "none"
    } else {
        collection_error
    };
    format!(
        "seq={sequence} op={operation} status={status} caller_tid={caller_thread_id} owner_tid={owner_thread_id} callback_tid={callback_thread_id} submission_lock_wait_us={submission_lock_wait_us} control_lock_wait_us={control_lock_wait_us} scheduler_lock_wait_us={scheduler_lock_wait_us} queue_residence_us={queue_residence_us} v8_call_us={v8_call_us} cancel_requested={cancel_requested} callback_count={} created_nodes={} last_node_id={} document_revision={} render_tree_revision={} document_nodes={} document_string_units={} document_collection_scans={} document_collection_deferred={} document_collection_scanned_handles={} document_collection_live_handles={} document_collection_empty_handles={} document_collection_last_scan_start_ns={} document_collection_last_scan_us={} document_collection_wrapper_root_buffer_bytes={} document_collection_reclaimed_node_buffer_bytes={} document_collection_poisoned={} document_collection_error={collection_error} error={error}",
        callbacks.callback_count,
        callbacks.created_nodes,
        callbacks.last_node_id,
        callbacks.document.document_revision(),
        callbacks.document.render_tree_revision(),
        callbacks.document.node_count(),
        callbacks.document.string_units(),
        collection_stats.scan_count,
        collection_stats.deferred_count,
        collection_stats.scanned_handle_count,
        collection_stats.live_handle_count,
        collection_stats.empty_handle_count,
        collection_stats.last_scan_start_ns,
        collection_stats.last_scan_duration_us,
        collection_stats.wrapper_root_buffer_bytes,
        collection_stats.reclaimed_node_buffer_bytes,
        collection_stats.runtime_poisoned,
    )
}
