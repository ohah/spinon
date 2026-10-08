use super::super::trace::finish_reply_handoff;
use super::super::{
    Command, ERR_CANCELLED, ERR_QUEUE_FULL, OK, OperationResponse, QUEUE_CAPACITY, RuntimeSession,
    TaskPriority, enqueue, lock,
};
use super::{
    PendingPriorityProbe, PriorityProbeResult, abort_priority_probe, parse_priority_probe_result,
};
use std::ffi::CString;
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const BLOCKER_TIMEOUT: Duration = Duration::from_secs(3);
const OVERFLOW_TIMEOUT: Duration = Duration::from_secs(2);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(20);
const CLEANUP_TIMEOUT: Duration = Duration::from_secs(5);
const THREAD_JOIN_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) fn run_priority_saturation_probe(
    session: &Arc<RuntimeSession>,
) -> Result<String, String> {
    wait_until_idle(session)?;

    let blocker_session = Arc::clone(session);
    let blocker = thread::Builder::new()
        .name("spinon-priority-saturation-blocker".to_owned())
        .spawn(move || blocker_session.eval("while (true) {}", TaskPriority::Background))
        .map_err(|error| format!("큐 포화 차단 작업 생성 실패: {error}"))?;

    if !wait_until_active(session, BLOCKER_TIMEOUT) {
        let cleanup = fail_and_cleanup(session, blocker, Vec::new(), None);
        return Err(format!(
            "큐 포화 차단 JavaScript가 시간 안에 실행을 시작하지 않았습니다: {cleanup}"
        ));
    }

    let mut accepted = Vec::with_capacity(QUEUE_CAPACITY);
    for index in 0..QUEUE_CAPACITY {
        let marker = index as i32 + 1;
        match enqueue_accepted_marker(session, marker) {
            Ok(task) => accepted.push(task),
            Err(error) => {
                let cleanup = fail_and_cleanup(session, blocker, accepted, None);
                return Err(format!(
                    "큐 포화 전에 accepted 작업이 거부됐습니다: marker={marker} status={} report={} cleanup={cleanup}",
                    error.status, error.report,
                ));
            }
        }
    }

    let queued = lock(&session.scheduler.state).queue.len();
    if queued != QUEUE_CAPACITY {
        let cleanup = fail_and_cleanup(session, blocker, accepted, None);
        return Err(format!(
            "포화 입력의 queue 깊이가 다릅니다: expected={QUEUE_CAPACITY} actual={queued} cleanup={cleanup}"
        ));
    }

    let overflow_session = Arc::clone(session);
    let (overflow_sender, overflow_receiver) = mpsc::channel();
    let overflow_thread = match thread::Builder::new()
        .name("spinon-priority-saturation-overflow".to_owned())
        .spawn(move || {
            let result = enqueue_overflow_marker(&overflow_session);
            let _ = overflow_sender.send(result);
        }) {
        Ok(thread) => thread,
        Err(error) => {
            let cleanup = fail_and_cleanup(session, blocker, accepted, None);
            return Err(format!("65번째 제출 thread 생성 실패: {error}; {cleanup}"));
        }
    };

    let overflow_admission = match overflow_receiver.recv_timeout(OVERFLOW_TIMEOUT) {
        Ok(admission) => admission,
        Err(error) => {
            abort_priority_probe(session);
            let blocker_join = join_with_timeout(blocker, "차단 작업");
            let overflow_join = join_with_timeout(overflow_thread, "overflow 제출");
            let late_admission = overflow_receiver.try_recv().ok();
            let overflow_task = late_admission.and_then(overflow_task_if_admitted);
            let drain_result = drain_after_abort(accepted, overflow_task);
            return Err(format!(
                "65번째 제출이 blocker 취소 전에 반환되지 않았습니다: {error} blocker_cleanup={} overflow_cleanup={} response_cleanup={}",
                cleanup_result(&blocker_join),
                cleanup_result(&overflow_join),
                cleanup_result(&drain_result)
            ));
        }
    };

    if let Err(error) = join_with_timeout(overflow_thread, "65번째 제출") {
        let overflow_task = overflow_task_if_admitted(overflow_admission);
        let cleanup = fail_and_cleanup(session, blocker, accepted, overflow_task);
        return Err(format!("65번째 제출 thread 회수 실패: {error}; {cleanup}"));
    }

    let overflow_error = match &overflow_admission {
        Err(response) if response.status == ERR_QUEUE_FULL => None,
        Err(response) => Some(format!(
            "65번째 제출 오류가 -5가 아닙니다: status={} report={}",
            response.status, response.report
        )),
        Ok(_) => Some("가득 찬 queue가 65번째 제출을 수락했습니다".to_owned()),
    };
    let overflow_task = overflow_task_if_admitted(overflow_admission);
    let queue_after_overflow = lock(&session.scheduler.state).queue.len();

    let cancel_status = session.cancel();
    let blocker_response = match join_with_timeout(blocker, "차단 작업") {
        Ok(response) => response,
        Err(error) => {
            abort_priority_probe(session);
            let drain = drain_after_abort(accepted, overflow_task);
            return Err(format!("큐 포화 blocker 종료 실패: {error}; {drain:?}"));
        }
    };

    let accepted_results = collect_accepted_results(accepted);
    let overflow_result = overflow_task.map(collect_overflow_result);

    if let Some(error) = overflow_error {
        return Err(error);
    }
    if cancel_status != OK || blocker_response.status != ERR_CANCELLED {
        return Err(format!(
            "큐 포화 blocker 취소 결과가 다릅니다: cancel_status={cancel_status} blocker_status={}",
            blocker_response.status
        ));
    }

    let accepted_results = accepted_results?;
    if queue_after_overflow != QUEUE_CAPACITY {
        return Err(format!(
            "overflow 거부 뒤 queue 깊이가 보존되지 않았습니다: expected={QUEUE_CAPACITY} actual={queue_after_overflow}"
        ));
    }
    if let Some(Err(error)) = overflow_result {
        return Err(error);
    }
    if accepted_results.len() != QUEUE_CAPACITY {
        return Err(format!(
            "accepted 응답 개수가 다릅니다: expected={QUEUE_CAPACITY} actual={}",
            accepted_results.len()
        ));
    }

    let owner_thread_id = accepted_results[0].owner_thread_id;
    if owner_thread_id == 0 {
        return Err("accepted 응답의 owner thread ID가 0입니다".to_owned());
    }
    let mut previous_sequence = 0;
    for (index, result) in accepted_results.iter().enumerate() {
        let expected_marker = index as i32 + 1;
        if result.marker != expected_marker
            || result.execution_order != index as u64 + 1
            || result.sequence <= previous_sequence
            || result.owner_thread_id != owner_thread_id
            || result.callback_thread_id != owner_thread_id
        {
            return Err(format!(
                "accepted marker/FIFO/thread 결과가 다릅니다: index={index} marker={} sequence={} execution_order={} owner_tid={} callback_tid={}",
                result.marker,
                result.sequence,
                result.execution_order,
                result.owner_thread_id,
                result.callback_thread_id
            ));
        }
        previous_sequence = result.sequence;
    }

    let queue_before_recovery = lock(&session.scheduler.state).queue.len();
    if queue_before_recovery != 0 {
        return Err(format!(
            "accepted 응답 수집 뒤 queue가 비어 있지 않습니다: actual={queue_before_recovery}"
        ));
    }

    let recovery = session.eval(
        "if (globalThis.__spinonQueueSaturationAccepted !== 64 || globalThis.__spinonQueueSaturationOverflow === true) { throw new Error('queue saturation marker mismatch'); } spinon.createNode(globalThis.__spinonQueueSaturationAccepted, 'queue-saturation-recovered');",
        TaskPriority::UserVisible,
    );
    let recovery_owner =
        report_field(&recovery.report, "owner_tid").and_then(|value| value.parse::<u64>().ok());
    let recovery_callback =
        report_field(&recovery.report, "callback_tid").and_then(|value| value.parse::<u64>().ok());
    if recovery.status != OK
        || report_field(&recovery.report, "last_node_id") != Some("64")
        || recovery_owner.is_none_or(|thread_id| thread_id == 0)
        || recovery_callback != recovery_owner
    {
        return Err(format!(
            "queue drain 후 재접수 검증 실패: status={} report={}",
            recovery.status, recovery.report
        ));
    }

    let final_queue_len = lock(&session.scheduler.state).queue.len();
    if final_queue_len != 0 {
        return Err(format!(
            "재접수 완료 뒤 queue가 비어 있지 않습니다: actual={final_queue_len}"
        ));
    }

    Ok(format!(
        "queue_saturation_probe=PASS capacity={QUEUE_CAPACITY} accepted={} overflow_status={ERR_QUEUE_FULL} overflow_returned_before_cancel=true overflow_queue_len={queue_after_overflow} overflow_marker=not-run completed={} recovered=PASS owner_tid={owner_thread_id}",
        accepted_results.len(),
        accepted_results.len()
    ))
}

fn enqueue_accepted_marker(
    session: &RuntimeSession,
    marker: i32,
) -> Result<PendingPriorityProbe, OperationResponse> {
    let source = CString::new(format!(
        "globalThis.__spinonQueueSaturationAccepted=(globalThis.__spinonQueueSaturationAccepted||0)+1;spinon.createNode(globalThis.__spinonQueueSaturationAccepted,'queue-accepted-{marker}');"
    ))
    .expect("큐 포화 검증 JavaScript에 NUL 문자가 없습니다");
    let (response, trace_cookie) = enqueue(
        session,
        TaskPriority::UserVisible,
        |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
            sequence,
            trace_cookie,
            source,
            caller_thread_id,
            reply,
        },
    )?;
    Ok(PendingPriorityProbe {
        priority: TaskPriority::UserVisible,
        marker,
        response,
        trace_cookie,
    })
}

fn enqueue_overflow_marker(
    session: &RuntimeSession,
) -> Result<(mpsc::Receiver<OperationResponse>, u32), OperationResponse> {
    let source = CString::new(
        "globalThis.__spinonQueueSaturationOverflow=true;spinon.createNode(999,'queue-overflow');",
    )
    .expect("큐 포화 검증 JavaScript에 NUL 문자가 없습니다");
    enqueue(
        session,
        TaskPriority::UserVisible,
        |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
            sequence,
            trace_cookie,
            source,
            caller_thread_id,
            reply,
        },
    )
}

fn wait_until_idle(session: &RuntimeSession) -> Result<(), String> {
    let deadline = Instant::now() + BLOCKER_TIMEOUT;
    while lock(&session.control).active && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if lock(&session.control).active {
        return Err("포화 검증 시작 전에 V8 실행기가 유휴 상태가 되지 않았습니다".to_owned());
    }
    let queued = lock(&session.scheduler.state).queue.len();
    if queued != 0 {
        return Err(format!(
            "포화 검증 시작 전에 대기 작업이 남아 있습니다: queue_len={queued}"
        ));
    }
    Ok(())
}

fn wait_until_active(session: &RuntimeSession, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while !lock(&session.control).active && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    lock(&session.control).active
}

fn fail_and_cleanup(
    session: &RuntimeSession,
    blocker: JoinHandle<OperationResponse>,
    accepted: Vec<PendingPriorityProbe>,
    overflow: Option<PendingPriorityProbe>,
) -> String {
    abort_priority_probe(session);
    let blocker_result = join_with_timeout(blocker, "차단 작업");
    let response_result = drain_after_abort(accepted, overflow);
    format!(
        "blocker_cleanup={} response_cleanup={}",
        cleanup_result(&blocker_result),
        cleanup_result(&response_result)
    )
}

fn drain_after_abort(
    accepted: Vec<PendingPriorityProbe>,
    overflow: Option<PendingPriorityProbe>,
) -> Result<(), String> {
    let deadline = Instant::now() + CLEANUP_TIMEOUT;
    let mut missing = Vec::new();
    for task in accepted.into_iter().chain(overflow) {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match task.response.recv_timeout(remaining) {
            Ok(_) => finish_reply_handoff(task.trace_cookie),
            Err(error) => missing.push(format!("marker={} error={error}", task.marker)),
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "pending 응답 정리 시간 초과: {}",
            missing.join(",")
        ))
    }
}

fn join_with_timeout<T>(handle: JoinHandle<T>, name: &str) -> Result<T, String> {
    let deadline = Instant::now() + THREAD_JOIN_TIMEOUT;
    while !handle.is_finished() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if !handle.is_finished() {
        drop(handle);
        return Err(format!(
            "{name} thread가 {}초 내에 끝나지 않아 회수하지 못했습니다",
            THREAD_JOIN_TIMEOUT.as_secs()
        ));
    }
    handle
        .join()
        .map_err(|_| format!("{name} thread가 비정상 종료됐습니다"))
}

fn cleanup_result<T>(result: &Result<T, String>) -> &str {
    match result {
        Ok(_) => "완료",
        Err(_) => "실패",
    }
}

fn overflow_task_if_admitted(
    admission: Result<(mpsc::Receiver<OperationResponse>, u32), OperationResponse>,
) -> Option<PendingPriorityProbe> {
    admission
        .ok()
        .map(|(response, trace_cookie)| PendingPriorityProbe {
            priority: TaskPriority::UserVisible,
            marker: 999,
            response,
            trace_cookie,
        })
}

fn collect_accepted_results(
    accepted: Vec<PendingPriorityProbe>,
) -> Result<Vec<PriorityProbeResult>, String> {
    let deadline = Instant::now() + RESPONSE_TIMEOUT;
    let mut results = Vec::with_capacity(accepted.len());
    let mut first_error = None;
    for task in accepted {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let response = match task.response.recv_timeout(remaining) {
            Ok(response) => {
                finish_reply_handoff(task.trace_cookie);
                response
            }
            Err(error) => {
                first_error.get_or_insert_with(|| {
                    format!("accepted marker={} 응답 시간 초과: {error}", task.marker)
                });
                continue;
            }
        };
        match parse_priority_probe_result(task.priority, task.marker, &response) {
            Ok(result) => results.push(result),
            Err(error) => {
                first_error.get_or_insert(error);
            }
        }
    }
    match first_error {
        Some(error) => Err(error),
        None => Ok(results),
    }
}

fn collect_overflow_result(task: PendingPriorityProbe) -> Result<(), String> {
    let response = task
        .response
        .recv_timeout(RESPONSE_TIMEOUT)
        .map_err(|error| format!("예기치 않게 수락된 overflow command 응답 시간 초과: {error}"))?;
    finish_reply_handoff(task.trace_cookie);
    Err(format!(
        "overflow command가 실행됐습니다: status={} report={}",
        response.status, response.report
    ))
}

fn report_field<'a>(report: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&prefix))
}
