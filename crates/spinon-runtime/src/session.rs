#[cfg(test)]
use crate::v8::{NodeCallback, TextCallback};
use crate::v8::{
    SpinonDocumentCollectionStats, SpinonV8Runtime, spinon_v8_current_thread_id,
    spinon_v8_runtime_notify_memory_pressure, spinon_v8_runtime_terminate,
};
pub use spinon_core::TaskPriority;
use std::ffi::CString;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Instant;

mod actor;
mod priority_probe;
mod report;
mod scheduler;
mod shutdown;
mod trace;

#[cfg(test)]
use actor::CallbackState;
use actor::actor_loop;
pub use priority_probe::run_priority_probe;
#[cfg(test)]
use report::{OperationReport, operation_report};
use scheduler::{EnqueueError, TaskScheduler};
pub use shutdown::run_shutdown_probe;
use trace::{TraceSection, finish_reply_handoff};

const QUEUE_CAPACITY: usize = 64;
static NEXT_REPLY_TRACE_COOKIE: AtomicU32 = AtomicU32::new(1);
const OK: i32 = 0;
const ERR_ARGUMENT: i32 = -1;
const ERR_JAVASCRIPT: i32 = -4;
const ERR_QUEUE_FULL: i32 = -5;
const ERR_CLOSED: i32 = -6;
const ERR_WORKER: i32 = -7;
const ERR_CANCELLED: i32 = -8;

struct RuntimeControl {
    runtime: Option<usize>,
    active: bool,
    cancel_generation: u64,
    closing: bool,
}

impl RuntimeControl {
    fn new() -> Self {
        Self {
            runtime: None,
            active: false,
            cancel_generation: 0,
            closing: false,
        }
    }
}

pub struct RuntimeSession {
    scheduler: Arc<TaskScheduler>,
    worker: Mutex<Option<JoinHandle<()>>>,
    control: Arc<Mutex<RuntimeControl>>,
    submission: Mutex<()>,
    shutdown_gate: Mutex<()>,
    next_sequence: AtomicU64,
}

enum Command {
    Eval {
        sequence: u64,
        trace_cookie: u32,
        source: CString,
        caller_thread_id: u64,
        reply: SyncSender<OperationResponse>,
    },
    Dispatch {
        sequence: u64,
        trace_cookie: u32,
        node_id: i32,
        caller_thread_id: u64,
        reply: SyncSender<OperationResponse>,
    },
}

pub struct OperationResponse {
    pub status: i32,
    pub report: String,
}

/// 호스트가 명시적으로 V8에 전달하는 메모리 압박 단계입니다.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryPressureLevel {
    /// 압박이 없다는 상태를 전달합니다. 즉시 GC를 요청하지 않습니다.
    None = 0,
    /// V8의 점진적 회수 휴리스틱을 앞당기도록 알립니다.
    Moderate = 1,
    /// V8에 가능한 빠른 메모리 회수가 필요하다고 알립니다.
    Critical = 2,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn current_thread_id() -> u64 {
    unsafe { spinon_v8_current_thread_id() }
}

fn submit(
    session: &RuntimeSession,
    priority: TaskPriority,
    command: impl FnOnce(u64, u64, u32, SyncSender<OperationResponse>) -> Command,
) -> OperationResponse {
    let _submit_trace = TraceSection::new(c"SpinonR05:runtime-submit");
    let submit_started_at = Instant::now();
    let enqueue_result = {
        let _enqueue_trace = TraceSection::new(c"SpinonR05:runtime-enqueue");
        enqueue(session, priority, command)
    };
    let (receiver, trace_cookie) = match enqueue_result {
        Ok(response) => response,
        Err(response) => return response,
    };
    let response_started_at = Instant::now();
    let enqueue_total_us = response_started_at
        .duration_since(submit_started_at)
        .as_micros();
    let mut response = {
        let _receive_trace = TraceSection::new(c"SpinonR05:reply-receive");
        let received = receiver.recv();
        if received.is_ok() {
            finish_reply_handoff(trace_cookie);
        }
        received.unwrap_or_else(|_| OperationResponse {
            status: ERR_WORKER,
            report: "V8 실행기가 응답하기 전에 종료되었습니다".to_owned(),
        })
    };
    let response_wait_us = response_started_at.elapsed().as_micros();
    let submit_total_us = submit_started_at.elapsed().as_micros();
    let response_report_started_at = Instant::now();
    response.report.push_str(&format!(
        " enqueue_total_us={enqueue_total_us} response_wait_us={response_wait_us} submit_total_us={submit_total_us}"
    ));
    let response_report_append_us = response_report_started_at.elapsed().as_micros();
    response.report.push_str(&format!(
        " response_report_append_us={response_report_append_us}"
    ));
    response
}

fn enqueue(
    session: &RuntimeSession,
    priority: TaskPriority,
    command: impl FnOnce(u64, u64, u32, SyncSender<OperationResponse>) -> Command,
) -> Result<(Receiver<OperationResponse>, u32), OperationResponse> {
    let (reply, response) = {
        let _channel_trace = TraceSection::new(c"SpinonR05:reply-channel-create");
        mpsc::sync_channel(1)
    };
    let submission_lock_started_at = Instant::now();
    let _submission = lock(&session.submission);
    let submission_lock_wait_us = submission_lock_started_at.elapsed().as_micros();
    let control_lock_started_at = Instant::now();
    let control_lock_wait_us;
    {
        let control = lock(&session.control);
        control_lock_wait_us = control_lock_started_at.elapsed().as_micros();
        if control.closing {
            return Err(OperationResponse {
                status: ERR_CLOSED,
                report: "세션이 종료되었습니다".to_owned(),
            });
        }
    }
    let sequence = session.next_sequence.fetch_add(1, Ordering::Relaxed) + 1;
    let trace_cookie = NEXT_REPLY_TRACE_COOKIE.fetch_add(1, Ordering::Relaxed);
    let caller_thread_id = current_thread_id();
    match session.scheduler.try_enqueue(
        priority,
        command(sequence, caller_thread_id, trace_cookie, reply),
        submission_lock_wait_us,
        control_lock_wait_us,
    ) {
        Ok(()) => {}
        Err(EnqueueError::Full) => {
            return Err(OperationResponse {
                status: ERR_QUEUE_FULL,
                report: format!("명령 큐가 가득 찼습니다 capacity={QUEUE_CAPACITY}"),
            });
        }
        Err(EnqueueError::Stopped) => {
            return Err(OperationResponse {
                status: ERR_CLOSED,
                report: "V8 실행기 큐가 종료되었습니다".to_owned(),
            });
        }
    }
    drop(_submission);
    Ok((response, trace_cookie))
}

fn cancel_control(control: &Mutex<RuntimeControl>) -> i32 {
    let mut state = lock(control);
    if !state.active {
        return 1;
    }
    let Some(runtime) = state.runtime else {
        return ERR_WORKER;
    };
    state.cancel_generation = state.cancel_generation.wrapping_add(1);
    unsafe { spinon_v8_runtime_terminate(runtime as *mut SpinonV8Runtime) };
    OK
}

impl RuntimeSession {
    /// V8 Isolate를 만들고 소유 전용 OS 스레드의 준비를 기다립니다.
    pub fn new() -> Result<(Self, String), String> {
        let control = Arc::new(Mutex::new(RuntimeControl::new()));
        let worker_control = Arc::clone(&control);
        let scheduler = Arc::new(TaskScheduler::new());
        let worker_scheduler = Arc::clone(&scheduler);
        let (ready_sender, ready_receiver) = mpsc::channel();
        let worker = thread::Builder::new()
            .name("spinon-js-runtime".to_owned())
            .spawn(move || actor_loop(worker_scheduler, worker_control, ready_sender))
            .map_err(|error| format!("실행기 스레드 생성 실패: {error}"))?;

        let owner_thread_id = match ready_receiver.recv() {
            Ok(Ok(thread_id)) => thread_id,
            Ok(Err(error)) => {
                let _ = worker.join();
                return Err(error);
            }
            Err(_) => {
                let _ = worker.join();
                return Err("V8 실행기 초기화 응답을 받지 못했습니다".to_owned());
            }
        };
        let report = format!(
            "session=ready owner_tid={owner_thread_id} isolate_per_session=1 queue_capacity={QUEUE_CAPACITY} queue_policy=strict-priority-fifo"
        );
        Ok((
            Self {
                scheduler,
                worker: Mutex::new(Some(worker)),
                control,
                submission: Mutex::new(()),
                shutdown_gate: Mutex::new(()),
                next_sequence: AtomicU64::new(0),
            },
            report,
        ))
    }

    /// JavaScript 평가를 지정한 논리 우선순위로 제출하고 완료 응답을 돌려줍니다.
    pub fn eval(&self, source: &str, priority: TaskPriority) -> OperationResponse {
        let source = match CString::new(source) {
            Ok(source) => source,
            Err(_) => {
                return OperationResponse {
                    status: ERR_ARGUMENT,
                    report: "JavaScript 원본에 NUL 문자가 있습니다".to_owned(),
                };
            }
        };
        submit(
            self,
            priority,
            |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
                sequence,
                trace_cookie,
                source,
                caller_thread_id,
                reply,
            },
        )
    }

    /// 등록된 JavaScript 이벤트 함수를 지정한 우선순위로 호출합니다.
    pub fn dispatch(&self, node_id: i32, priority: TaskPriority) -> OperationResponse {
        submit(
            self,
            priority,
            |sequence, caller_thread_id, trace_cookie, reply| Command::Dispatch {
                sequence,
                trace_cookie,
                node_id,
                caller_thread_id,
                reply,
            },
        )
    }

    /// 호스트가 정한 압박 단계를 V8에 전달합니다. OS 신호나 판단 시점은 정하지 않습니다.
    ///
    /// 이 호출은 JavaScript 취소나 이벤트 dispatch를 요청하지 않습니다. V8이 안전 지점에서
    /// 회수를 수행할 수 있지만 완료 시점·회수량은 보장하지 않습니다. 종료 중이면
    /// `ERR_CLOSED`를 돌려주며, Isolate 해제 전에 통지가 끝나도록 런타임 상태 잠금을 유지합니다.
    pub fn notify_memory_pressure(&self, level: MemoryPressureLevel) -> i32 {
        let control = lock(&self.control);
        if control.closing {
            return ERR_CLOSED;
        }
        let Some(runtime) = control.runtime else {
            return ERR_CLOSED;
        };
        let status = unsafe {
            spinon_v8_runtime_notify_memory_pressure(runtime as *mut SpinonV8Runtime, level as i32)
        };
        if status == 0 { OK } else { ERR_ARGUMENT }
    }

    /// 실행 중인 JavaScript를 취소합니다. 실행 중인 작업이 없으면 1을 돌려줍니다.
    pub fn cancel(&self) -> i32 {
        cancel_control(&self.control)
    }
}

impl Drop for RuntimeSession {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod tests;
