use super::trace::finish_reply_handoff;
use super::{
    Command, ERR_CANCELLED, ERR_CLOSED, RuntimeSession, TaskPriority, cancel_control,
    current_thread_id, enqueue, lock,
};
use std::ffi::CString;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

const ACTIVE_TIMEOUT: Duration = Duration::from_secs(3);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(3);
const QUEUED_COMMANDS: usize = 3;

impl RuntimeSession {
    pub(super) fn shutdown(&self) -> Result<(), &'static str> {
        let _shutdown = lock(&self.shutdown_gate);
        {
            let _submission = lock(&self.submission);
            lock(&self.control).closing = true;
        }
        let _ = cancel_control(&self.control);
        self.scheduler.stop();
        let actor_failed = lock(&self.worker)
            .take()
            .is_some_and(|worker| worker.join().is_err());
        let cascade_result = self.ua_cascade.shutdown();
        if actor_failed {
            self.scheduler.reject_pending(
                super::ERR_WORKER,
                "V8 실행기 오류로 대기 명령을 실행하지 않았습니다",
            );
            let mut control = lock(&self.control);
            control.active = false;
            control.runtime = None;
            return Err("V8 실행기 스레드가 정상 종료되지 않았습니다");
        }
        cascade_result?;
        Ok(())
    }
}

/// 실제 V8에서 활성 평가 취소, 대기 명령 거부, 종료 후 호출 거부를 확인합니다.
pub fn run_shutdown_probe() -> Result<String, String> {
    let (session, ready_report) = RuntimeSession::new()?;
    let owner_thread_id = report_field(&ready_report, "owner_tid")
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| format!("초기화 보고의 owner_tid가 올바르지 않습니다: {ready_report}"))?;
    let session = Arc::new(session);
    let active_session = Arc::clone(&session);
    let active = thread::Builder::new()
        .name("spinon-shutdown-active".to_owned())
        .spawn(move || active_session.eval("while (true) {}", TaskPriority::Background))
        .map_err(|error| format!("종료 검증의 활성 평가 스레드 생성 실패: {error}"))?;

    if !wait_until_active(&session, ACTIVE_TIMEOUT) {
        // 평가가 아직 큐에만 있다면 cancel은 실행 중 작업이 없다고 반환할 수 있다.
        // 먼저 세션을 닫아 대기 명령이 시작되지 못하게 한 뒤 평가 호출자를 회수한다.
        let shutdown_session = Arc::clone(&session);
        let (shutdown_sender, shutdown_receiver) = mpsc::sync_channel(1);
        let shutdown = thread::Builder::new()
            .name("spinon-shutdown-timeout-cleanup".to_owned())
            .spawn(move || {
                let result = shutdown_session.shutdown();
                let _ = shutdown_sender.send((result, current_thread_id()));
            });
        let shutdown = match shutdown {
            Ok(shutdown) => shutdown,
            Err(error) => {
                let _ = session.shutdown();
                let _ = active.join();
                return Err(format!("종료 검증 정리 thread 생성 실패: {error}"));
            }
        };
        let shutdown_result = shutdown_receiver.recv_timeout(SHUTDOWN_TIMEOUT);
        let shutdown_result = match shutdown_result {
            Ok((result, _)) => result,
            Err(error) => {
                let _ = session.cancel();
                return Err(format!(
                    "활성 평가 관찰 실패 뒤 세션 정리 시간 초과: {error}"
                ));
            }
        };
        if shutdown.join().is_err() {
            let _ = session.shutdown();
            let _ = active.join();
            return Err("종료 검증 정리 thread가 비정상 종료됐습니다".to_owned());
        }
        if let Err(error) = shutdown_result {
            let _ = active.join();
            return Err(format!("활성 평가 관찰 실패 뒤 세션 정리 실패: {error}"));
        }
        let _ = active.join();
        return Err("종료 검증 전에 무한 평가가 시작되지 않았습니다".to_owned());
    }

    let mut queued = Vec::with_capacity(QUEUED_COMMANDS);
    for index in 0..QUEUED_COMMANDS {
        let source = CString::new(
            "globalThis.__spinonShutdownProbeRan = (globalThis.__spinonShutdownProbeRan || 0) + 1;",
        )
        .expect("검증 JavaScript에 NUL 문자가 없습니다");
        let response = enqueue(
            &session,
            if index == 0 {
                TaskPriority::UserBlocking
            } else {
                TaskPriority::Background
            },
            move |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
                sequence,
                trace_cookie,
                source,
                caller_thread_id,
                reply,
            },
        );
        match response {
            Ok((response, trace_cookie)) => queued.push((response, trace_cookie)),
            Err(error) => {
                let _ = session.shutdown();
                let _ = active.join();
                return Err(format!("종료 검증 대기 명령 접수 실패: {}", error.report));
            }
        }
    }

    let shutdown_session = Arc::clone(&session);
    let (shutdown_sender, shutdown_receiver) = mpsc::sync_channel(1);
    let shutdown = match thread::Builder::new()
        .name("spinon-shutdown-controller".to_owned())
        .spawn(move || {
            let shutdown_started = Instant::now();
            let result = shutdown_session.shutdown();
            let shutdown_thread_id = current_thread_id();
            let shutdown_duration_ms = shutdown_started.elapsed().as_millis();
            let _ = shutdown_sender.send((result, shutdown_thread_id, shutdown_duration_ms));
        }) {
        Ok(shutdown) => shutdown,
        Err(error) => {
            let _ = session.shutdown();
            let _ = active.join();
            return Err(format!("종료 검증 제어 스레드 생성 실패: {error}"));
        }
    };

    let (shutdown_result, shutdown_thread_id, shutdown_duration_ms) =
        match shutdown_receiver.recv_timeout(SHUTDOWN_TIMEOUT) {
            Ok(result) => result,
            Err(error) => {
                let _ = session.cancel();
                return Err(format!("실제 V8 세션 종료 응답 시간 초과: {error}"));
            }
        };
    if shutdown.join().is_err() {
        let _ = session.shutdown();
        let _ = active.join();
        return Err("종료 검증 제어 스레드가 비정상 종료됐습니다".to_owned());
    }
    if let Err(error) = shutdown_result {
        let _ = active.join();
        return Err(error.to_owned());
    }
    if shutdown_thread_id == owner_thread_id {
        return Err("종료 제어가 V8 owner thread와 분리되지 않았습니다".to_owned());
    }

    let active_response = active
        .join()
        .map_err(|_| "종료 검증의 활성 평가 스레드가 비정상 종료됐습니다".to_owned())?;
    if active_response.status != ERR_CANCELLED {
        return Err(format!(
            "활성 평가는 취소 상태를 반환해야 합니다: status={} report={}",
            active_response.status, active_response.report
        ));
    }

    let mut queued_statuses = Vec::with_capacity(queued.len());
    for (response, trace_cookie) in queued {
        let response = response
            .recv_timeout(RESPONSE_TIMEOUT)
            .map_err(|error| format!("대기 명령 종료 응답 시간 초과: {error}"))?;
        finish_reply_handoff(trace_cookie);
        if response.status != ERR_CLOSED
            || !response
                .report
                .contains("세션 종료 중이라 명령을 실행하지 않았습니다")
        {
            return Err(format!(
                "종료 전에 접수한 대기 명령이 실행되거나 잘못된 결과를 받았습니다: status={} report={}",
                response.status, response.report
            ));
        }
        queued_statuses.push(response.status);
    }

    let post_eval_status = session.eval("1 + 1", TaskPriority::UserVisible).status;
    let post_dispatch_status = session.dispatch(1, TaskPriority::UserBlocking).status;
    if post_eval_status != ERR_CLOSED || post_dispatch_status != ERR_CLOSED {
        return Err(format!(
            "닫힌 세션이 새 명령을 거부하지 않았습니다: eval={post_eval_status} dispatch={post_dispatch_status}"
        ));
    }

    session
        .shutdown()
        .map_err(|error| format!("반복 종료 호출 실패: {error}"))?;
    let control = lock(&session.control);
    let runtime_released = control.runtime.is_none();
    let inactive = !control.active;
    let closing = control.closing;
    drop(control);
    let worker_joined = lock(&session.worker).is_none();
    if !runtime_released || !inactive || !closing || !worker_joined {
        return Err(format!(
            "종료 뒤 세션 상태가 닫히지 않았습니다: runtime_released={runtime_released} inactive={inactive} closing={closing} worker_joined={worker_joined}"
        ));
    }

    Ok(format!(
        "shutdown_probe=PASS active_status={} queued_statuses={queued_statuses:?} post_eval_status={post_eval_status} post_dispatch_status={post_dispatch_status} close_ms={shutdown_duration_ms} repeat_close=PASS runtime_released={runtime_released} worker_joined={worker_joined} owner_tid={owner_thread_id} shutdown_tid={shutdown_thread_id}",
        active_response.status,
    ))
}

fn wait_until_active(session: &RuntimeSession, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if lock(&session.control).active {
            return true;
        }
        thread::sleep(Duration::from_millis(1));
    }
    lock(&session.control).active
}

fn report_field<'a>(report: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&prefix))
}

#[cfg(test)]
mod tests {
    use super::finish_reply_handoff;
    use super::{ERR_CANCELLED, ERR_CLOSED, RuntimeSession, TaskPriority, enqueue, lock};
    use crate::session::{Command, RuntimeControl, TaskScheduler};
    use std::ffi::CString;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

    struct ShutdownGuard(Arc<RuntimeSession>);

    impl Drop for ShutdownGuard {
        fn drop(&mut self) {
            let _ = self.0.shutdown();
        }
    }

    #[test]
    fn idle_shutdown_releases_the_runtime_and_rejects_later_calls() {
        let session = Arc::new(
            RuntimeSession::new()
                .expect("가짜 V8 세션을 만들어야 합니다")
                .0,
        );
        let _guard = ShutdownGuard(Arc::clone(&session));

        session.shutdown().expect("유휴 세션을 종료해야 합니다");

        assert!(lock(&session.control).closing);
        assert!(!lock(&session.control).active);
        assert!(lock(&session.control).runtime.is_none());
        assert!(lock(&session.worker).is_none());
        assert_eq!(
            session
                .eval("after-idle-close", TaskPriority::UserVisible)
                .status,
            ERR_CLOSED
        );
    }

    #[test]
    fn shutdown_cancels_active_eval_rejects_queued_work_and_joins_owner() {
        let session = Arc::new(
            RuntimeSession::new()
                .expect("가짜 V8 세션을 만들어야 합니다")
                .0,
        );
        let _guard = ShutdownGuard(Arc::clone(&session));
        let active_session = Arc::clone(&session);
        let active = thread::spawn(move || active_session.eval("hang", TaskPriority::Background));
        wait_until_active(&session);

        let mut pending = Vec::new();
        for sequence in 0..3 {
            let source = CString::new(format!("queued-{sequence}"))
                .expect("검증 원본에 NUL 문자가 없습니다");
            let response = enqueue(
                &session,
                TaskPriority::UserVisible,
                move |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
                    sequence,
                    trace_cookie,
                    source,
                    caller_thread_id,
                    reply,
                },
            );
            let (response, trace_cookie) = match response {
                Ok(response) => response,
                Err(error) => panic!("종료 전에 세 명령을 큐에 넣어야 합니다: {}", error.report),
            };
            pending.push((response, trace_cookie));
        }
        assert_eq!(lock(&session.scheduler.state).queue.len(), 3);

        let closer_session = Arc::clone(&session);
        let closer = thread::spawn(move || closer_session.shutdown());
        closer
            .join()
            .expect("종료 호출 thread가 정상 종료되어야 합니다")
            .expect("런타임 작업자를 join해야 합니다");
        assert_eq!(active.join().unwrap().status, ERR_CANCELLED);
        for (response, trace_cookie) in pending {
            let response = response.recv_timeout(Duration::from_secs(2)).unwrap();
            finish_reply_handoff(trace_cookie);
            assert_eq!(response.status, ERR_CLOSED);
        }

        assert_eq!(
            session
                .eval("after-close", TaskPriority::UserVisible)
                .status,
            ERR_CLOSED
        );
        assert_eq!(
            session.dispatch(9, TaskPriority::UserBlocking).status,
            ERR_CLOSED
        );
        session.shutdown().expect("반복 종료는 멱등이어야 합니다");
        assert!(lock(&session.control).runtime.is_none());
        assert!(!lock(&session.control).active);
        assert!(lock(&session.control).closing);
        assert!(lock(&session.worker).is_none());
    }

    #[test]
    fn submit_and_multiple_close_race_never_leaves_a_caller_waiting() {
        const CALLERS: usize = 12;
        const CLOSERS: usize = 4;
        let session = Arc::new(
            RuntimeSession::new()
                .expect("가짜 V8 세션을 만들어야 합니다")
                .0,
        );
        let _guard = ShutdownGuard(Arc::clone(&session));
        let active_session = Arc::clone(&session);
        let active = thread::spawn(move || active_session.eval("hang", TaskPriority::Background));
        wait_until_active(&session);

        let start = Arc::new(std::sync::Barrier::new(CALLERS + CLOSERS + 1));
        let mut callers = Vec::with_capacity(CALLERS);
        for index in 0..CALLERS {
            let session = Arc::clone(&session);
            let start = Arc::clone(&start);
            callers.push(thread::spawn(move || {
                start.wait();
                session.eval(&format!("race-{index}"), TaskPriority::UserVisible)
            }));
        }
        let mut closers = Vec::with_capacity(CLOSERS);
        for _ in 0..CLOSERS {
            let closer_session = Arc::clone(&session);
            let closer_start = Arc::clone(&start);
            closers.push(thread::spawn(move || {
                closer_start.wait();
                closer_session.shutdown()
            }));
        }
        start.wait();

        for closer in closers {
            closer
                .join()
                .expect("각 종료 경쟁 thread가 정상 종료되어야 합니다")
                .expect("동시 종료 경쟁에서 작업자를 한 번만 join해야 합니다");
        }
        assert_eq!(active.join().unwrap().status, ERR_CANCELLED);
        for caller in callers {
            assert_eq!(
                caller.join().unwrap().status,
                ERR_CLOSED,
                "닫기 경쟁 호출은 종료 오류로 끝나야 합니다"
            );
        }
        assert!(lock(&session.worker).is_none());
    }

    #[test]
    fn worker_panic_rejects_pending_commands_and_clears_stale_runtime_state() {
        let scheduler = Arc::new(TaskScheduler::new());
        let (reply, response) = std::sync::mpsc::sync_channel(1);
        let (dispatch_reply, dispatch_response) = std::sync::mpsc::sync_channel(1);
        assert!(
            scheduler
                .try_enqueue(
                    TaskPriority::Background,
                    Command::Eval {
                        sequence: 1,
                        trace_cookie: 1001,
                        source: CString::new("pending").unwrap(),
                        caller_thread_id: 101,
                        reply,
                    },
                    0,
                    0,
                )
                .is_ok()
        );
        assert!(
            scheduler
                .try_enqueue(
                    TaskPriority::UserBlocking,
                    Command::Dispatch {
                        sequence: 2,
                        trace_cookie: 1002,
                        node_id: 9,
                        caller_thread_id: 202,
                        reply: dispatch_reply,
                    },
                    0,
                    0,
                )
                .is_ok()
        );
        let worker = thread::spawn(|| panic!("검증용 실행기 panic"));
        let session = RuntimeSession {
            scheduler: Arc::clone(&scheduler),
            worker: Mutex::new(Some(worker)),
            control: Arc::new(Mutex::new(RuntimeControl {
                runtime: Some(1),
                active: false,
                cancel_generation: 0,
                closing: false,
            })),
            submission: Mutex::new(()),
            shutdown_gate: Mutex::new(()),
            next_sequence: std::sync::atomic::AtomicU64::new(0),
            ua_cascade: super::super::RuntimeUaCascadeCoordinator::new()
                .expect("테스트 CSS worker를 만들어야 합니다"),
        };

        assert!(session.shutdown().is_err());
        for (response, trace_cookie, operation, caller_thread_id) in [
            (response, 1001, "eval", 101),
            (dispatch_response, 1002, "dispatch", 202),
        ] {
            let response = response
                .recv_timeout(Duration::from_secs(2))
                .expect("대기 호출자는 실행기 오류를 받아야 합니다");
            finish_reply_handoff(trace_cookie);
            assert_eq!(response.status, crate::session::ERR_WORKER);
            assert!(
                response
                    .report
                    .contains(&format!("trace_cookie={trace_cookie}"))
            );
            assert!(response.report.contains(&format!("op={operation}")));
            assert!(
                response
                    .report
                    .contains(&format!("status={}", response.status))
            );
            assert!(
                response
                    .report
                    .contains(&format!("caller_tid={caller_thread_id}"))
            );
        }
        assert!(lock(&session.control).runtime.is_none());
        assert!(!lock(&session.control).active);
        assert!(lock(&session.control).closing);
        assert!(lock(&session.worker).is_none());
        assert!(lock(&scheduler.state).stopped);
        assert_eq!(lock(&scheduler.state).queue.len(), 0);
    }

    fn wait_until_active(session: &RuntimeSession) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while !lock(&session.control).active && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(1));
        }
        assert!(lock(&session.control).active);
    }
}
