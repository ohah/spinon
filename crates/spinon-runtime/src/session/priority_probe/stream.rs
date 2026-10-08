use super::super::trace::finish_reply_handoff;
use super::super::{
    Command, ERR_CANCELLED, OK, OperationResponse, QUEUE_CAPACITY, RuntimeSession, TaskPriority,
    enqueue, lock,
};
use super::{
    PendingPriorityProbe, PriorityProbeResult, abort_priority_probe, parse_priority_probe_result,
};
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct PriorityStreamSummary {
    background_sequence: u64,
    background_execution_order: u64,
    background_queue_wait_us: u64,
    accepted_high_tasks: usize,
    owner_thread_id: u64,
}

pub(super) fn run_priority_stream_probe(
    session: &Arc<RuntimeSession>,
    expected_first_execution_order: u64,
) -> Result<String, String> {
    const INITIAL_HIGH_TASKS: usize = 63;
    const LATE_HIGH_TASKS: usize = 1_024;
    const QUEUE_ROOM_TIMEOUT: Duration = Duration::from_secs(10);
    const RESPONSE_TIMEOUT: Duration = Duration::from_secs(60);

    let idle_deadline = Instant::now() + Duration::from_secs(2);
    while lock(&session.control).active && Instant::now() < idle_deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if lock(&session.control).active {
        return Err(
            "우선순위 유입 검증 시작 전 V8 실행기가 유휴 상태가 되지 않았습니다".to_owned(),
        );
    }

    let blocker_session = Arc::clone(session);
    let blocker = thread::Builder::new()
        .name("spinon-priority-stream-blocker".to_owned())
        .spawn(move || blocker_session.eval("while (true) {}", TaskPriority::Background))
        .map_err(|error| format!("우선순위 유입 검증 차단 작업 생성 실패: {error}"))?;

    let active_deadline = Instant::now() + Duration::from_secs(2);
    while !lock(&session.control).active && Instant::now() < active_deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if !lock(&session.control).active {
        abort_priority_probe(session);
        let _ = blocker.join();
        return Err("우선순위 유입 검증의 V8 차단 작업이 시간 안에 시작되지 않았습니다".to_owned());
    }

    let started_at = Instant::now();
    let background = match enqueue_priority_marker(session, TaskPriority::Background, 10_000, false)
    {
        Ok(task) => task,
        Err(error) => {
            abort_priority_probe(session);
            let _ = blocker.join();
            return Err(format!("낮은 우선순위 작업 접수 실패: {}", error.report));
        }
    };
    let mut pending = vec![background];
    for index in 0..INITIAL_HIGH_TASKS {
        let marker = 20_000 + index as i32;
        match enqueue_priority_marker(session, TaskPriority::UserBlocking, marker, true) {
            Ok(task) => pending.push(task),
            Err(error) => {
                abort_priority_probe(session);
                let _ = blocker.join();
                return Err(format!(
                    "초기 높은 우선순위 작업 접수 실패: {}",
                    error.report
                ));
            }
        }
    }

    let queued = lock(&session.scheduler.state).queue.len();
    if queued != INITIAL_HIGH_TASKS + 1 {
        abort_priority_probe(session);
        let _ = blocker.join();
        return Err(format!(
            "우선순위 유입 검증의 초기 큐 깊이가 다릅니다: expected={} actual={queued}",
            INITIAL_HIGH_TASKS + 1
        ));
    }

    let producer_session = Arc::clone(session);
    let producer = match thread::Builder::new()
        .name("spinon-priority-stream-producer".to_owned())
        .spawn(move || {
            let mut produced = Vec::with_capacity(LATE_HIGH_TASKS);
            let queue_deadline = Instant::now() + QUEUE_ROOM_TIMEOUT;
            for index in 0..LATE_HIGH_TASKS {
                wait_for_priority_queue_room(&producer_session, queue_deadline)?;
                let marker = 30_000 + index as i32;
                let task = enqueue_priority_marker(
                    &producer_session,
                    TaskPriority::UserBlocking,
                    marker,
                    true,
                )
                .map_err(|error| format!("지속 유입 작업 접수 실패: {}", error.report))?;
                produced.push(task);
            }
            Ok::<_, String>(produced)
        }) {
        Ok(producer) => producer,
        Err(error) => {
            abort_priority_probe(session);
            let _ = blocker.join();
            return Err(format!("우선순위 유입 검증 생산자 생성 실패: {error}"));
        }
    };

    let cancel_status = session.cancel();
    let blocker_response = match blocker.join() {
        Ok(response) => response,
        Err(_) => {
            abort_priority_probe(session);
            let _ = producer.join();
            return Err("우선순위 유입 검증 차단 작업 스레드가 비정상 종료됐습니다".to_owned());
        }
    };
    if cancel_status != OK || blocker_response.status != ERR_CANCELLED {
        abort_priority_probe(session);
        let _ = producer.join();
        return Err(format!(
            "우선순위 유입 검증 차단 해제 실패: cancel_status={cancel_status} blocker_status={}",
            blocker_response.status
        ));
    }

    match producer.join() {
        Ok(Ok(produced)) => pending.extend(produced),
        Ok(Err(error)) => {
            abort_priority_probe(session);
            return Err(error);
        }
        Err(_) => {
            abort_priority_probe(session);
            return Err("우선순위 유입 검증 생산자 스레드가 비정상 종료됐습니다".to_owned());
        }
    }

    let mut results = Vec::with_capacity(pending.len());
    let response_deadline = Instant::now() + RESPONSE_TIMEOUT;
    for task in pending {
        let remaining = response_deadline.saturating_duration_since(Instant::now());
        let response = match task.response.recv_timeout(remaining) {
            Ok(response) => {
                finish_reply_handoff(task.trace_cookie);
                response
            }
            Err(error) => {
                abort_priority_probe(session);
                return Err(format!("우선순위 유입 작업 응답 시간 초과: {error}"));
            }
        };
        let result = match parse_priority_probe_result(task.priority, task.marker, &response) {
            Ok(result) => result,
            Err(error) => {
                abort_priority_probe(session);
                return Err(error);
            }
        };
        results.push(result);
    }

    let summary = validate_priority_stream_results(
        &results,
        INITIAL_HIGH_TASKS + LATE_HIGH_TASKS,
        expected_first_execution_order,
    )?;

    let elapsed_ms = started_at.elapsed().as_millis();
    Ok(format!(
        "priority_stream_probe=PASS capacity={} initial_high={} late_high={} accepted_high={} background_seq={} background_order={} background_wait_us={} elapsed_ms={} owner_tid={}",
        INITIAL_HIGH_TASKS + 1,
        INITIAL_HIGH_TASKS,
        LATE_HIGH_TASKS,
        summary.accepted_high_tasks,
        summary.background_sequence,
        summary.background_execution_order,
        summary.background_queue_wait_us,
        elapsed_ms,
        summary.owner_thread_id
    ))
}

fn validate_priority_stream_results(
    results: &[PriorityProbeResult],
    expected_high_tasks: usize,
    expected_first_execution_order: u64,
) -> Result<PriorityStreamSummary, String> {
    let backgrounds = results
        .iter()
        .filter(|result| result.priority == TaskPriority::Background)
        .collect::<Vec<_>>();
    let mut high_results = results
        .iter()
        .filter(|result| result.priority == TaskPriority::UserBlocking)
        .collect::<Vec<_>>();
    high_results.sort_by_key(|result| result.execution_order);
    let Some(background) = backgrounds.first().copied() else {
        return Err("priority_stream_probe=FAIL missing_background=true".to_owned());
    };

    let task_count_matches = results.len() == expected_high_tasks + 1;
    let priority_counts_match = backgrounds.len() == 1 && high_results.len() == expected_high_tasks;
    let high_fifo_matches = high_results
        .windows(2)
        .all(|pair| pair[0].sequence < pair[1].sequence);
    let high_starts_at_expected_order = high_results
        .first()
        .is_some_and(|result| result.execution_order == expected_first_execution_order);
    let high_execution_is_contiguous = high_results
        .windows(2)
        .all(|pair| pair[0].execution_order + 1 == pair[1].execution_order);
    let high_tasks_overtook_background = high_results.iter().all(|result| {
        result.sequence > background.sequence && result.execution_order < background.execution_order
    });
    let background_was_last = high_results
        .last()
        .is_some_and(|result| result.execution_order + 1 == background.execution_order);
    let owner_thread_matches = background.owner_thread_id > 0
        && results.iter().all(|result| {
            result.owner_thread_id == background.owner_thread_id
                && result.callback_thread_id == background.owner_thread_id
        });
    if !task_count_matches
        || !priority_counts_match
        || !high_fifo_matches
        || !high_starts_at_expected_order
        || !high_execution_is_contiguous
        || !high_tasks_overtook_background
        || !background_was_last
        || !owner_thread_matches
    {
        return Err(format!(
            "priority_stream_probe=FAIL task_count={} priority_counts={} high_fifo={} high_starts_expected={} high_contiguous={} high_overtook_background={} background_last={} owner_thread={}",
            task_count_matches,
            priority_counts_match,
            high_fifo_matches,
            high_starts_at_expected_order,
            high_execution_is_contiguous,
            high_tasks_overtook_background,
            background_was_last,
            owner_thread_matches
        ));
    }

    Ok(PriorityStreamSummary {
        background_sequence: background.sequence,
        background_execution_order: background.execution_order,
        background_queue_wait_us: background.queue_residence_us,
        accepted_high_tasks: high_results.len(),
        owner_thread_id: background.owner_thread_id,
    })
}

fn enqueue_priority_marker(
    session: &RuntimeSession,
    priority: TaskPriority,
    marker: i32,
    add_work: bool,
) -> Result<PendingPriorityProbe, OperationResponse> {
    let work = if add_work {
        "for(let until=Date.now()+3;Date.now()<until;){};"
    } else {
        ""
    };
    let source = CString::new(format!(
        "{work}globalThis.__spinonPriorityProbeOrder=(globalThis.__spinonPriorityProbeOrder||0)+1;spinon.createNode(globalThis.__spinonPriorityProbeOrder,'probe-{marker}');"
    ))
    .expect("우선순위 검증 JavaScript에는 NUL 문자가 없습니다");
    let (response, trace_cookie) = enqueue(
        session,
        priority,
        |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
            sequence,
            trace_cookie,
            source,
            caller_thread_id,
            reply,
        },
    )?;
    Ok(PendingPriorityProbe {
        priority,
        marker,
        response,
        trace_cookie,
    })
}

fn wait_for_priority_queue_room(session: &RuntimeSession, deadline: Instant) -> Result<(), String> {
    loop {
        if Instant::now() >= deadline {
            return Err("우선순위 유입 검증에서 큐 공간 대기 시간이 초과됐습니다".to_owned());
        }
        let state = lock(&session.scheduler.state);
        if state.stopped {
            return Err("우선순위 유입 검증 중 실행기가 종료됐습니다".to_owned());
        }
        let has_room = state.queue.len() < QUEUE_CAPACITY;
        drop(state);
        if has_room {
            return Ok(());
        }
        thread::sleep(
            Duration::from_millis(1).min(deadline.saturating_duration_since(Instant::now())),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::super::PriorityProbeResult;
    use super::validate_priority_stream_results;
    use crate::session::TaskPriority;

    fn result(
        priority: TaskPriority,
        marker: i32,
        sequence: u64,
        execution_order: u64,
        callback_thread_id: u64,
    ) -> PriorityProbeResult {
        PriorityProbeResult {
            priority,
            marker,
            sequence,
            execution_order,
            owner_thread_id: 17,
            callback_thread_id,
            queue_residence_us: 123,
        }
    }

    fn valid_results() -> Vec<PriorityProbeResult> {
        vec![
            result(TaskPriority::Background, 10_000, 9, 10, 17),
            result(TaskPriority::UserBlocking, 20_000, 10, 7, 17),
            result(TaskPriority::UserBlocking, 20_001, 11, 8, 17),
            result(TaskPriority::UserBlocking, 30_000, 12, 9, 17),
        ]
    }

    #[test]
    fn accepts_new_high_priority_work_before_older_background_work() {
        let summary = validate_priority_stream_results(&valid_results(), 3, 7)
            .expect("높은 등급 작업 뒤에서 낮은 작업을 처리한 결과를 받아야 합니다");

        assert_eq!(summary.background_sequence, 9);
        assert_eq!(summary.background_execution_order, 10);
        assert_eq!(summary.background_queue_wait_us, 123);
        assert_eq!(summary.accepted_high_tasks, 3);
        assert_eq!(summary.owner_thread_id, 17);
    }

    #[test]
    fn rejects_background_work_that_runs_before_later_high_priority_work() {
        let mut results = valid_results();
        results[0].execution_order = 8;
        results[3].execution_order = 10;

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("뒤늦게 접수한 높은 작업이 낮은 작업보다 먼저 와야 합니다");
        assert!(error.contains("high_overtook_background=false"));
    }

    #[test]
    fn rejects_high_priority_fifo_inversion() {
        let mut results = valid_results();
        results[1].sequence = 11;
        results[2].sequence = 10;

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("같은 등급의 접수 순서를 지켜야 합니다");
        assert!(error.contains("high_fifo=false"));
    }

    #[test]
    fn rejects_unexpected_first_execution_order() {
        let error = validate_priority_stream_results(&valid_results(), 3, 8)
            .expect_err("첫 높은 등급 작업은 예상한 실행 순서에서 시작해야 합니다");
        assert!(error.contains("high_starts_expected=false"));
    }

    #[test]
    fn rejects_gaps_in_high_priority_execution_order() {
        let mut results = valid_results();
        results[2].execution_order = 10;
        results[0].execution_order = 11;

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("높은 등급 작업과 마지막 background 사이에 실행 순서 공백이 없어야 합니다");
        assert!(error.contains("high_contiguous=false"));
    }

    #[test]
    fn rejects_callback_execution_on_a_different_owner_thread() {
        let mut results = valid_results();
        results[2].callback_thread_id = 18;

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("모든 V8 callback은 같은 owner thread에서 실행되어야 합니다");
        assert!(error.contains("owner_thread=false"));
    }

    #[test]
    fn rejects_a_zero_owner_thread_id() {
        let mut results = valid_results();
        results[0].owner_thread_id = 0;

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("owner thread ID가 0이면 유효한 실행 증거가 아닙니다");
        assert!(error.contains("owner_thread=false"));
    }

    #[test]
    fn rejects_missing_high_priority_work() {
        let mut results = valid_results();
        results.pop();

        let error = validate_priority_stream_results(&results, 3, 7)
            .expect_err("요청한 모든 높은 등급 작업이 있어야 합니다");
        assert!(error.contains("task_count=false"));
    }
}
