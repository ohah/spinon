use super::report::{OperationReport, operation_report};
use super::trace::TraceSection;
use super::{
    Command, ERR_CANCELLED, ERR_CLOSED, ERR_JAVASCRIPT, ERR_WORKER, OK, OperationResponse,
    RuntimeControl, SpinonDocumentCollectionStats, SpinonV8Runtime, TaskScheduler,
    current_thread_id, lock,
};
use crate::host::{HostDocumentBridge, collect_callback, commit_callback, query_callback};
use crate::v8::{
    spinon_v8_current_thread_id, spinon_v8_runtime_cancel_termination, spinon_v8_runtime_dispatch,
    spinon_v8_runtime_document_collection_stats, spinon_v8_runtime_eval, spinon_v8_runtime_free,
    spinon_v8_runtime_last_collection_error, spinon_v8_runtime_last_error, spinon_v8_runtime_new,
    spinon_v8_runtime_was_terminated,
};
use std::ffi::{CStr, c_char, c_void};
use std::ptr;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Instant;

pub(super) struct CallbackState {
    pub(super) callback_count: u64,
    pub(super) created_nodes: u64,
    pub(super) last_node_id: i32,
    pub(super) callback_thread_id: u64,
    pub(super) document: HostDocumentBridge,
}

impl CallbackState {
    fn new() -> Result<Self, String> {
        Ok(Self {
            callback_count: 0,
            created_nodes: 0,
            last_node_id: 0,
            callback_thread_id: 0,
            document: HostDocumentBridge::new()?,
        })
    }

    fn reset_operation(&mut self) {
        self.callback_count = 0;
        self.created_nodes = 0;
        self.last_node_id = 0;
        self.callback_thread_id = 0;
    }
}

extern "C" fn on_node(user_data: *mut c_void, node_id: i32, tag: *const c_char) {
    if user_data.is_null() || tag.is_null() {
        return;
    }
    let state = unsafe { &mut *user_data.cast::<CallbackState>() };
    let _tag = unsafe { CStr::from_ptr(tag) }.to_string_lossy();
    state.callback_count = state.callback_count.saturating_add(1);
    state.created_nodes = state.created_nodes.saturating_add(1);
    state.last_node_id = node_id;
    state.callback_thread_id = unsafe { spinon_v8_current_thread_id() };
}

extern "C" fn on_text(user_data: *mut c_void, text: *const c_char) {
    if user_data.is_null() || text.is_null() {
        return;
    }
    let state = unsafe { &mut *user_data.cast::<CallbackState>() };
    let _text = unsafe { CStr::from_ptr(text) }.to_string_lossy();
    state.callback_count = state.callback_count.saturating_add(1);
    state.callback_thread_id = unsafe { spinon_v8_current_thread_id() };
}

fn v8_error(runtime: *mut SpinonV8Runtime) -> String {
    let error = unsafe { spinon_v8_runtime_last_error(runtime) };
    if error.is_null() {
        return "V8 오류 메시지가 비어 있습니다".to_owned();
    }
    unsafe { CStr::from_ptr(error) }
        .to_string_lossy()
        .replace(['\n', '\r'], " ")
}

fn v8_collection_error(runtime: *mut SpinonV8Runtime) -> String {
    let error = unsafe { spinon_v8_runtime_last_collection_error(runtime) };
    if error.is_null() {
        return "diagnostic_unavailable".to_owned();
    }
    unsafe { CStr::from_ptr(error) }
        .to_string_lossy()
        .replace(['\n', '\r', ' '], "_")
}

fn v8_collection_stats(runtime: *mut SpinonV8Runtime) -> SpinonDocumentCollectionStats {
    let mut stats = SpinonDocumentCollectionStats::default();
    unsafe { spinon_v8_runtime_document_collection_stats(runtime, &mut stats) };
    stats
}

pub(super) fn actor_loop(
    scheduler: Arc<TaskScheduler>,
    control: Arc<Mutex<RuntimeControl>>,
    ready: mpsc::Sender<Result<u64, String>>,
) {
    let mut callbacks = match CallbackState::new() {
        Ok(callbacks) => callbacks,
        Err(error) => {
            let _ = ready.send(Err(format!("HostDocument를 만들지 못했습니다: {error}")));
            return;
        }
    };
    let callback_data = ptr::addr_of_mut!(callbacks).cast::<c_void>();
    let document_data = ptr::addr_of_mut!(callbacks.document).cast::<c_void>();
    let runtime = unsafe {
        spinon_v8_runtime_new(
            on_node,
            on_text,
            commit_callback,
            query_callback,
            collect_callback,
            callback_data,
            document_data,
        )
    };
    if runtime.is_null() {
        let _ = ready.send(Err("V8 Isolate를 만들지 못했습니다".to_owned()));
        return;
    }

    let owner_thread_id = current_thread_id();
    {
        let mut state = lock(&control);
        state.runtime = Some(runtime as usize);
    }
    if ready.send(Ok(owner_thread_id)).is_err() {
        let mut state = lock(&control);
        state.runtime = None;
        drop(state);
        unsafe { spinon_v8_runtime_free(runtime) };
        return;
    }

    while let Some(queued) = scheduler.receive() {
        let queue_residence_us = queued.enqueued_at.elapsed().as_micros();
        let submission_lock_wait_us = queued.submission_lock_wait_us;
        let control_lock_wait_us = queued.control_lock_wait_us;
        let scheduler_lock_wait_us = queued.scheduler_lock_wait_us;
        match queued.command {
            Command::Eval {
                sequence,
                source,
                caller_thread_id,
                reply,
            } => {
                let generation = match begin_execution(&control, runtime) {
                    Ok(generation) => generation,
                    Err(response) => {
                        let _ = reply.send(response);
                        continue;
                    }
                };
                callbacks.reset_operation();
                let started_at = Instant::now();
                let v8_result = unsafe { spinon_v8_runtime_eval(runtime, source.as_ptr()) };
                let v8_call_us = started_at.elapsed().as_micros();
                let collection_error = v8_collection_error(runtime);
                let collection_stats = v8_collection_stats(runtime);
                let v8_was_terminated = unsafe { spinon_v8_runtime_was_terminated(runtime) != 0 };
                let error = if v8_result == 0 {
                    String::new()
                } else {
                    v8_error(runtime)
                };
                let cancel_requested = finish_execution(&control, runtime, generation);
                let status = if v8_result == 0 {
                    OK
                } else if cancel_requested && v8_was_terminated {
                    ERR_CANCELLED
                } else {
                    ERR_JAVASCRIPT
                };
                let report = operation_report(OperationReport {
                    sequence,
                    operation: "eval",
                    status,
                    caller_thread_id,
                    owner_thread_id,
                    callback_thread_id: callbacks.callback_thread_id,
                    submission_lock_wait_us,
                    control_lock_wait_us,
                    scheduler_lock_wait_us,
                    queue_residence_us,
                    v8_call_us,
                    cancel_requested,
                    callbacks: &callbacks,
                    collection_error: &collection_error,
                    collection_stats,
                    error: &error,
                });
                let reply_trace = TraceSection::new(c"SpinonR05:reply-send");
                let _ = reply.send(OperationResponse { status, report });
                drop(reply_trace);
            }
            Command::Dispatch {
                sequence,
                node_id,
                caller_thread_id,
                reply,
            } => {
                let actor_started_at = Instant::now();
                let generation = match begin_execution(&control, runtime) {
                    Ok(generation) => generation,
                    Err(response) => {
                        let _ = reply.send(response);
                        continue;
                    }
                };
                callbacks.reset_operation();
                let started_at = Instant::now();
                let v8_result = unsafe { spinon_v8_runtime_dispatch(runtime, node_id) };
                let v8_call_us = started_at.elapsed().as_micros();
                let post_v8_started_at = Instant::now();
                let collection_error = v8_collection_error(runtime);
                let collection_stats = v8_collection_stats(runtime);
                let v8_was_terminated = unsafe { spinon_v8_runtime_was_terminated(runtime) != 0 };
                let error = if v8_result == 0 {
                    String::new()
                } else {
                    v8_error(runtime)
                };
                let cancel_requested = finish_execution(&control, runtime, generation);
                let status = if v8_result == 0 {
                    OK
                } else if cancel_requested && v8_was_terminated {
                    ERR_CANCELLED
                } else {
                    ERR_JAVASCRIPT
                };
                let post_v8_us = post_v8_started_at.elapsed().as_micros();
                let report_started_at = Instant::now();
                let report = operation_report(OperationReport {
                    sequence,
                    operation: "dispatch",
                    status,
                    caller_thread_id,
                    owner_thread_id,
                    callback_thread_id: callbacks.callback_thread_id,
                    submission_lock_wait_us,
                    control_lock_wait_us,
                    scheduler_lock_wait_us,
                    queue_residence_us,
                    v8_call_us,
                    cancel_requested,
                    callbacks: &callbacks,
                    collection_error: &collection_error,
                    collection_stats,
                    error: &error,
                });
                let report_build_us = report_started_at.elapsed().as_micros();
                let report_finalize_started_at = Instant::now();
                let mut report =
                    format!("{report} post_v8_us={post_v8_us} report_build_us={report_build_us}");
                let report_finalize_us = report_finalize_started_at.elapsed().as_micros();
                let actor_before_reply_us = actor_started_at.elapsed().as_micros();
                report.push_str(&format!(
                    " report_finalize_us={report_finalize_us} actor_before_reply_us={actor_before_reply_us}"
                ));
                let reply_trace = TraceSection::new(c"SpinonR05:reply-send");
                let _ = reply.send(OperationResponse { status, report });
                drop(reply_trace);
            }
        }
    }

    {
        let mut state = lock(&control);
        state.active = false;
        state.runtime = None;
    }
    unsafe { spinon_v8_runtime_free(runtime) };
}

fn begin_execution(
    control: &Mutex<RuntimeControl>,
    runtime: *mut SpinonV8Runtime,
) -> Result<u64, OperationResponse> {
    let mut state = lock(control);
    if state.closing {
        return Err(OperationResponse {
            status: ERR_CLOSED,
            report: "세션 종료 중이라 명령을 실행하지 않았습니다".to_owned(),
        });
    }
    if state.runtime != Some(runtime as usize) || state.active {
        return Err(OperationResponse {
            status: ERR_WORKER,
            report: "실행기 소유권 상태가 올바르지 않습니다".to_owned(),
        });
    }
    state.active = true;
    Ok(state.cancel_generation)
}

fn finish_execution(
    control: &Mutex<RuntimeControl>,
    runtime: *mut SpinonV8Runtime,
    generation: u64,
) -> bool {
    let cancelled = {
        let mut state = lock(control);
        let cancelled = state.cancel_generation != generation;
        state.active = false;
        cancelled
    };
    if cancelled {
        // 취소 플래그 정리는 다음 V8 작업을 시작하기 전에 소유 스레드에서 수행합니다.
        unsafe { spinon_v8_runtime_cancel_termination(runtime) };
    }
    cancelled
}
