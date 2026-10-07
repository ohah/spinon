use super::{
    CallbackState, Command, ERR_CANCELLED, ERR_CLOSED, ERR_QUEUE_FULL, EnqueueError,
    MemoryPressureLevel, OK, OperationReport, QUEUE_CAPACITY, RuntimeSession,
    SpinonDocumentCollectionStats, TaskPriority, TaskScheduler, operation_report,
};
use crate::host::{DocumentCommitCallback, HostDocumentBridge};
use std::ffi::{CStr, c_char, c_void};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct FakeV8Runtime {
    node_callback: super::NodeCallback,
    text_callback: super::TextCallback,
    user_data: usize,
    terminated: AtomicBool,
    was_terminated: AtomicBool,
    operation_order: Mutex<Vec<String>>,
}

unsafe impl Sync for FakeV8Runtime {}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_new(
    node_callback: super::NodeCallback,
    text_callback: super::TextCallback,
    _document_commit_callback: DocumentCommitCallback,
    _document_query_callback: crate::host::DocumentQueryCallback,
    _document_collect_callback: crate::host::DocumentCollectCallback,
    user_data: *mut c_void,
    _document_user_data: *mut c_void,
) -> *mut super::SpinonV8Runtime {
    Box::into_raw(Box::new(FakeV8Runtime {
        node_callback,
        text_callback,
        user_data: user_data as usize,
        terminated: AtomicBool::new(false),
        was_terminated: AtomicBool::new(false),
        operation_order: Mutex::new(Vec::new()),
    }))
    .cast::<super::SpinonV8Runtime>()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_eval(
    runtime: *mut super::SpinonV8Runtime,
    source: *const c_char,
) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    let source = unsafe { CStr::from_ptr(source) }.to_bytes();
    runtime.was_terminated.store(false, Ordering::Release);
    if source == b"hang" {
        while !runtime.terminated.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(1));
        }
        runtime.was_terminated.store(true, Ordering::Release);
        return -1;
    }
    runtime
        .operation_order
        .lock()
        .unwrap()
        .push(format!("eval:{}", String::from_utf8_lossy(source)));
    (runtime.node_callback)(runtime.user_data as *mut c_void, 7, c"view".as_ptr());
    (runtime.text_callback)(runtime.user_data as *mut c_void, c"ready".as_ptr());
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_dispatch(
    runtime: *mut super::SpinonV8Runtime,
    node_id: i32,
) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.was_terminated.store(false, Ordering::Release);
    runtime
        .operation_order
        .lock()
        .unwrap()
        .push(format!("dispatch:{node_id}"));
    (runtime.node_callback)(
        runtime.user_data as *mut c_void,
        node_id + 1,
        c"text".as_ptr(),
    );
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_notify_memory_pressure(
    _runtime: *mut super::SpinonV8Runtime,
    level: i32,
) -> i32 {
    if (0..=2).contains(&level) { 0 } else { -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_was_terminated(runtime: *mut super::SpinonV8Runtime) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    i32::from(runtime.was_terminated.load(Ordering::Acquire))
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_last_error(
    _runtime: *mut super::SpinonV8Runtime,
) -> *const c_char {
    c"fake JavaScript error".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_last_collection_error(
    _runtime: *mut super::SpinonV8Runtime,
) -> *const c_char {
    c"".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_document_collection_stats(
    _runtime: *mut super::SpinonV8Runtime,
    stats: *mut super::SpinonDocumentCollectionStats,
) {
    if !stats.is_null() {
        unsafe { *stats = super::SpinonDocumentCollectionStats::default() };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_terminate(runtime: *mut super::SpinonV8Runtime) {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.terminated.store(true, Ordering::Release);
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_cancel_termination(runtime: *mut super::SpinonV8Runtime) {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.terminated.store(false, Ordering::Release);
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_free(runtime: *mut super::SpinonV8Runtime) {
    drop(unsafe { Box::from_raw(runtime.cast::<FakeV8Runtime>()) });
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_current_thread_id() -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish()
}

fn new_session() -> RuntimeSession {
    RuntimeSession::new()
        .expect("가짜 V8 런타임 세션을 만들어야 합니다")
        .0
}

fn eval(session: &RuntimeSession, source: &str) -> super::OperationResponse {
    session.eval(source, TaskPriority::UserVisible)
}

#[test]
fn report_exposes_caller_owner_callback_threads_and_timings() {
    let callbacks = CallbackState {
        callback_count: 2,
        created_nodes: 1,
        last_node_id: 7,
        callback_thread_id: 42,
        document: HostDocumentBridge::new().unwrap(),
    };
    let report = operation_report(OperationReport {
        sequence: 3,
        operation: "dispatch",
        status: 0,
        caller_thread_id: 10,
        owner_thread_id: 42,
        callback_thread_id: 42,
        submission_lock_wait_us: 1,
        control_lock_wait_us: 2,
        scheduler_lock_wait_us: 3,
        queue_residence_us: 11,
        v8_call_us: 29,
        cancel_requested: false,
        callbacks: &callbacks,
        collection_error: "",
        collection_stats: SpinonDocumentCollectionStats::default(),
        error: "",
    });
    assert!(report.contains("caller_tid=10"));
    assert!(report.contains("owner_tid=42"));
    assert!(report.contains("callback_tid=42"));
    assert!(report.contains("submission_lock_wait_us=1"));
    assert!(report.contains("control_lock_wait_us=2"));
    assert!(report.contains("scheduler_lock_wait_us=3"));
    assert!(report.contains("queue_residence_us=11"));
    assert!(report.contains("v8_call_us=29"));
    assert!(report.contains("cancel_requested=false"));
    assert!(report.contains("document_collection_scans=0"));
    assert!(report.contains("document_collection_deferred=0"));
    assert!(report.contains("document_collection_error=none"));
    assert!(report.contains("document_revision=0"));
    assert!(report.contains("render_tree_revision=0"));
    assert!(report.contains("document_nodes=0"));
    assert!(report.contains("document_string_units=0"));
    assert!(report.contains("document_collection_error=none"));
    assert!(report.contains("callback_count=2"));
}

#[test]
fn bounded_scheduler_selects_priority_then_fifo_and_drains_on_stop() {
    let scheduler = TaskScheduler::new();
    for (priority, node_id) in [
        (TaskPriority::Background, 1),
        (TaskPriority::UserVisible, 2),
        (TaskPriority::Background, 3),
        (TaskPriority::UserBlocking, 4),
        (TaskPriority::UserBlocking, 5),
        (TaskPriority::UserVisible, 6),
    ] {
        let (reply, _response) = std::sync::mpsc::sync_channel(1);
        let command = Command::Dispatch {
            sequence: node_id as u64,
            node_id,
            caller_thread_id: 0,
            reply,
        };
        assert!(scheduler.try_enqueue(priority, command, 0, 0).is_ok());
    }

    let mut order = Vec::new();
    for _ in 0..6 {
        let command = scheduler.receive().expect("대기 작업을 받아야 합니다");
        if let Command::Dispatch { node_id, .. } = command.command {
            order.push(node_id);
        }
    }
    assert_eq!(order, [4, 5, 2, 6, 1, 3]);

    scheduler.stop();
    assert!(scheduler.receive().is_none());
    let (reply, _response) = std::sync::mpsc::sync_channel(1);
    assert!(matches!(
        scheduler.try_enqueue(
            TaskPriority::UserBlocking,
            Command::Dispatch {
                sequence: 7,
                node_id: 7,
                caller_thread_id: 0,
                reply,
            },
            0,
            0,
        ),
        Err(EnqueueError::Stopped)
    ));
}

#[test]
fn session_executes_pending_commands_by_priority_and_fifo() {
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let running = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);

    let background_session = Arc::clone(&session);
    let background =
        thread::spawn(move || background_session.eval("background", TaskPriority::Background));
    wait_for_queue_len(&session, 1);
    let visible_session = Arc::clone(&session);
    let visible = thread::spawn(move || visible_session.dispatch(22, TaskPriority::UserVisible));
    wait_for_queue_len(&session, 2);
    let blocking_first_session = Arc::clone(&session);
    let blocking_first =
        thread::spawn(move || blocking_first_session.dispatch(31, TaskPriority::UserBlocking));
    wait_for_queue_len(&session, 3);
    let blocking_second_session = Arc::clone(&session);
    let blocking_second =
        thread::spawn(move || blocking_second_session.dispatch(32, TaskPriority::UserBlocking));
    wait_for_queue_len(&session, 4);

    assert_eq!(session.cancel(), OK);
    assert_eq!(running.join().unwrap().status, ERR_CANCELLED);
    assert_eq!(background.join().unwrap().status, OK);
    assert_eq!(visible.join().unwrap().status, OK);
    assert_eq!(blocking_first.join().unwrap().status, OK);
    assert_eq!(blocking_second.join().unwrap().status, OK);

    let runtime = super::lock(&session.control)
        .runtime
        .expect("세션 V8 실행기가 남아 있어야 합니다");
    let operation_order = unsafe { &*(runtime as *const FakeV8Runtime) }
        .operation_order
        .lock()
        .unwrap()
        .clone();
    assert_eq!(
        operation_order,
        [
            "dispatch:31",
            "dispatch:32",
            "dispatch:22",
            "eval:background"
        ]
    );
}

fn wait_until_active(session: &RuntimeSession) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while !super::lock(&session.control).active && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    assert!(super::lock(&session.control).active);
}

fn wait_for_queue_len(session: &RuntimeSession, expected: usize) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while super::lock(&session.scheduler.state).queue.len() != expected && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(super::lock(&session.scheduler.state).queue.len(), expected);
}

#[test]
fn persistent_session_keeps_owner_and_callbacks_on_one_worker() {
    let session = new_session();
    let first = eval(&session, "callback");
    assert_eq!(first.status, OK);
    let caller = field(&first.report, "caller_tid");
    let owner = field(&first.report, "owner_tid");
    assert_ne!(caller, owner);
    assert_eq!(owner, field(&first.report, "callback_tid"));

    let second = session.dispatch(7, TaskPriority::UserBlocking);
    assert_eq!(second.status, OK);
    assert_eq!(field(&second.report, "owner_tid"), owner);
    assert_eq!(field(&second.report, "callback_tid"), owner);
}

#[test]
fn cancel_interrupts_active_call_and_session_accepts_next_call() {
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let running = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);
    assert_eq!(session.cancel(), OK);

    let cancelled = running.join().unwrap();
    assert_eq!(cancelled.status, ERR_CANCELLED);
    assert!(cancelled.report.contains("cancel_requested=true"));
    assert!(cancelled.report.contains("owner_tid="));
    let resumed = eval(&session, "again");
    assert_eq!(resumed.status, OK);
    assert!(resumed.report.contains("error=none"));
}

#[test]
fn memory_pressure_levels_reach_the_runtime_and_closed_sessions_reject_them() {
    let session = new_session();
    assert_eq!(MemoryPressureLevel::None as i32, 0);
    assert_eq!(MemoryPressureLevel::Moderate as i32, 1);
    assert_eq!(MemoryPressureLevel::Critical as i32, 2);
    for level in [
        MemoryPressureLevel::None,
        MemoryPressureLevel::Moderate,
        MemoryPressureLevel::Critical,
    ] {
        assert_eq!(session.notify_memory_pressure(level), OK);
    }

    session.shutdown().expect("유휴 세션은 종료되어야 합니다");
    assert_eq!(
        session.notify_memory_pressure(MemoryPressureLevel::None),
        ERR_CLOSED
    );
}

#[test]
fn bounded_queue_rejects_overflow_and_drains_after_cancellation() {
    assert_eq!(QUEUE_CAPACITY, 64);
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let long_eval = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);

    let start = Arc::new(Barrier::new(66));
    let completed = Arc::new(AtomicUsize::new(0));
    let statuses = Arc::new(Mutex::new(Vec::with_capacity(65)));
    let mut callers = Vec::with_capacity(65);
    for node_id in 0..65 {
        let start = Arc::clone(&start);
        let completed = Arc::clone(&completed);
        let statuses = Arc::clone(&statuses);
        let session = Arc::clone(&session);
        callers.push(thread::spawn(move || {
            start.wait();
            let status = session.dispatch(node_id, TaskPriority::UserBlocking).status;
            statuses.lock().unwrap().push(status);
            completed.fetch_add(1, Ordering::Release);
        }));
    }
    start.wait();
    let deadline = Instant::now() + Duration::from_secs(2);
    while completed.load(Ordering::Acquire) == 0 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    assert!(
        completed.load(Ordering::Acquire) > 0,
        "가득 찬 큐가 호출을 거부하지 않았습니다"
    );
    assert_eq!(session.cancel(), OK);

    assert_eq!(long_eval.join().unwrap().status, ERR_CANCELLED);
    for caller in callers {
        caller.join().unwrap();
    }
    let statuses = statuses.lock().unwrap();
    assert!(statuses.contains(&ERR_QUEUE_FULL));
    assert!(
        statuses
            .iter()
            .all(|status| *status == OK || *status == ERR_QUEUE_FULL)
    );
}

fn field(report: &str, key: &str) -> String {
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&format!("{key}=")))
        .unwrap_or_else(|| panic!("보고서에 {key} 필드가 없습니다: {report}"))
        .to_owned()
}
