use super::trace::finish_reply_handoff;
use super::{
    Command, ERR_CANCELLED, OK, OperationResponse, RuntimeSession, TaskPriority, cancel_control,
    enqueue, lock,
};
use std::ffi::CString;
use std::sync::{Arc, mpsc::Receiver};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(any(target_os = "android", target_os = "ios", test))]
mod saturation;
#[cfg(any(target_os = "android", target_os = "ios", test))]
mod stream;

struct PendingPriorityProbe {
    priority: TaskPriority,
    marker: i32,
    response: Receiver<OperationResponse>,
    trace_cookie: u32,
}

struct PriorityProbeResult {
    priority: TaskPriority,
    marker: i32,
    sequence: u64,
    execution_order: u64,
    owner_thread_id: u64,
    callback_thread_id: u64,
    #[cfg(any(target_os = "android", target_os = "ios", test))]
    queue_residence_us: u64,
}

/// 실제 V8 Isolate에서 세 우선순위의 선택 순서와 같은 우선순위 FIFO를 확인합니다.
/// 앱 작성자용 API가 아니라 시뮬레이터 검증에 쓰는 내부 진단 경로입니다.
pub fn run_priority_probe() -> Result<String, String> {
    const ACTIVE_TIMEOUT: Duration = Duration::from_secs(2);
    const RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

    let (session, _) = RuntimeSession::new()?;
    let session = Arc::new(session);
    let blocker_session = Arc::clone(&session);
    let blocker = thread::Builder::new()
        .name("spinon-priority-probe".to_owned())
        .spawn(move || blocker_session.eval("while (true) {}", TaskPriority::Background))
        .map_err(|error| format!("우선순위 검증 작업 생성 실패: {error}"))?;

    let active_deadline = Instant::now() + ACTIVE_TIMEOUT;
    while !lock(&session.control).active && Instant::now() < active_deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if !lock(&session.control).active {
        abort_priority_probe(&session);
        let _ = blocker.join();
        return Err("우선순위 검증의 V8 차단 작업이 시간 안에 시작되지 않았습니다".to_owned());
    }

    // 낮은 등급부터 섞어 접수합니다. 실행 순서는 이 접수 순서와 달라야 합니다.
    let planned = [
        (TaskPriority::Background, 201),
        (TaskPriority::UserVisible, 101),
        (TaskPriority::UserBlocking, 1),
        (TaskPriority::Background, 202),
        (TaskPriority::UserBlocking, 2),
        (TaskPriority::UserVisible, 102),
    ];
    let mut pending = Vec::with_capacity(planned.len());
    for (priority, marker) in planned {
        let source = CString::new(format!(
            "globalThis.__spinonPriorityProbeOrder=(globalThis.__spinonPriorityProbeOrder||0)+1;spinon.createNode(globalThis.__spinonPriorityProbeOrder,'probe-{marker}');"
        ))
        .expect("우선순위 검증 JavaScript에는 NUL 문자가 없습니다");
        let response = enqueue(
            &session,
            priority,
            |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
                sequence,
                trace_cookie,
                source,
                caller_thread_id,
                reply,
            },
        );
        match response {
            Ok((response, trace_cookie)) => pending.push(PendingPriorityProbe {
                priority,
                marker,
                response,
                trace_cookie,
            }),
            Err(error) => {
                abort_priority_probe(&session);
                let _ = blocker.join();
                return Err(format!("우선순위 검증 작업 접수 실패: {}", error.report));
            }
        }
    }

    let cancel_status = session.cancel();
    let blocker_response = match blocker.join() {
        Ok(response) => response,
        Err(_) => {
            abort_priority_probe(&session);
            return Err("우선순위 검증의 차단 작업 스레드가 비정상 종료됐습니다".to_owned());
        }
    };
    if cancel_status != OK || blocker_response.status != ERR_CANCELLED {
        abort_priority_probe(&session);
        return Err(format!(
            "우선순위 검증 차단 해제 실패: cancel_status={cancel_status} blocker_status={}",
            blocker_response.status
        ));
    }

    let mut results = Vec::with_capacity(pending.len());
    for pending in pending {
        let response = match pending.response.recv_timeout(RESPONSE_TIMEOUT) {
            Ok(response) => {
                finish_reply_handoff(pending.trace_cookie);
                response
            }
            Err(error) => {
                abort_priority_probe(&session);
                return Err(format!("우선순위 검증 작업 응답 시간 초과: {error}"));
            }
        };
        let result = match parse_priority_probe_result(pending.priority, pending.marker, &response)
        {
            Ok(result) => result,
            Err(error) => {
                abort_priority_probe(&session);
                return Err(error);
            }
        };
        results.push(result);
    }

    let mut expected = results.iter().collect::<Vec<_>>();
    expected.sort_by_key(|result| (priority_rank(result.priority), result.sequence));
    let mut observed = results.iter().collect::<Vec<_>>();
    observed.sort_by_key(|result| result.execution_order);

    let order_is_contiguous = observed
        .iter()
        .enumerate()
        .all(|(index, result)| result.execution_order == index as u64 + 1);
    let priority_and_fifo_match = observed
        .iter()
        .map(|result| (result.priority, result.marker))
        .eq(expected
            .iter()
            .map(|result| (result.priority, result.marker)));
    let owner_thread_id = observed.first().map(|result| result.owner_thread_id);
    let thread_ownership_matches = observed.iter().all(|result| {
        Some(result.owner_thread_id) == owner_thread_id
            && result.callback_thread_id == result.owner_thread_id
    });
    if !order_is_contiguous || !priority_and_fifo_match || !thread_ownership_matches {
        return Err(format!(
            "priority_probe=FAIL order={} contiguous={} priority_fifo={} owner_thread={}",
            format_priority_probe_order(&observed),
            order_is_contiguous,
            priority_and_fifo_match,
            thread_ownership_matches
        ));
    }

    Ok(format!(
        "priority_probe=PASS blocker_status={} cancel_status={} order={} owner_tid={}",
        blocker_response.status,
        cancel_status,
        format_priority_probe_order(&observed),
        owner_thread_id.unwrap_or_default()
    ))
}

/// Android·iOS 시뮬레이터의 실제 V8 세션에서 높은 등급 유입 중 낮은 등급 작업의 선택 순서를 확인합니다.
#[cfg(any(target_os = "android", target_os = "ios", test))]
pub fn run_priority_fairness_probe() -> Result<String, String> {
    let (session, _) = RuntimeSession::new()?;
    let session = Arc::new(session);
    let result = match stream::run_priority_stream_probe(&session, 1) {
        Ok(fairness_report) => match saturation::run_priority_saturation_probe(&session) {
            Ok(saturation_report) => Ok(format!("{fairness_report} {saturation_report}")),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    };
    match result {
        Ok(report) => Ok(report),
        Err(error) => {
            abort_priority_probe(&session);
            Err(error)
        }
    }
}

fn abort_priority_probe(session: &RuntimeSession) {
    lock(&session.control).closing = true;
    let _ = cancel_control(&session.control);
    session.scheduler.stop();
}

fn parse_priority_probe_result(
    priority: TaskPriority,
    marker: i32,
    response: &OperationResponse,
) -> Result<PriorityProbeResult, String> {
    if response.status != OK {
        return Err(format!(
            "우선순위 검증 작업 실패: priority={} marker={} status={} report={}",
            priority_name(priority),
            marker,
            response.status,
            response.report
        ));
    }
    let parse = |key: &str| -> Result<u64, String> {
        report_field(&response.report, key)
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("검증 응답에 {key} 값이 없습니다: {}", response.report))
    };
    let execution_order = parse("last_node_id")?;
    let sequence = parse("seq")?;
    let owner_thread_id = parse("owner_tid")?;
    let callback_thread_id = parse("callback_tid")?;
    #[cfg(any(target_os = "android", target_os = "ios", test))]
    let queue_residence_us = parse("queue_residence_us")?;
    if execution_order == 0 {
        return Err(format!(
            "검증 작업이 V8 콜백을 실행하지 않았습니다: {}",
            response.report
        ));
    }
    Ok(PriorityProbeResult {
        priority,
        marker,
        sequence,
        execution_order,
        owner_thread_id,
        callback_thread_id,
        #[cfg(any(target_os = "android", target_os = "ios", test))]
        queue_residence_us,
    })
}

fn report_field<'a>(report: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&prefix))
}

const fn priority_rank(priority: TaskPriority) -> u8 {
    match priority {
        TaskPriority::UserBlocking => 0,
        TaskPriority::UserVisible => 1,
        TaskPriority::Background => 2,
    }
}

const fn priority_name(priority: TaskPriority) -> &'static str {
    match priority {
        TaskPriority::UserBlocking => "user-blocking",
        TaskPriority::UserVisible => "user-visible",
        TaskPriority::Background => "background",
    }
}

fn format_priority_probe_order(results: &[&PriorityProbeResult]) -> String {
    let order = results
        .iter()
        .map(|result| {
            format!(
                "{}:{}#seq{}",
                priority_name(result.priority),
                result.marker,
                result.sequence
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{order}]")
}
