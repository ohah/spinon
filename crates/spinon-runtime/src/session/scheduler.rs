use super::report::append_early_error_context;
use super::trace::{TraceSection, send_reply};
use super::{Command, OperationResponse, QUEUE_CAPACITY, TaskPriority, lock};
use spinon_core::PriorityQueue;
use std::sync::{Condvar, Mutex};
use std::time::Instant;

pub(super) struct SchedulerState {
    pub(super) queue: PriorityQueue<QueuedCommand>,
    pub(super) stopped: bool,
}

pub(super) struct QueuedCommand {
    pub(super) command: Command,
    pub(super) enqueued_at: Instant,
    pub(super) submission_lock_wait_us: u128,
    pub(super) control_lock_wait_us: u128,
    pub(super) scheduler_lock_wait_us: u128,
}

pub(super) struct TaskScheduler {
    pub(super) state: Mutex<SchedulerState>,
    available: Condvar,
}

pub(super) enum EnqueueError {
    Full,
    Stopped,
}

impl TaskScheduler {
    pub(super) fn new() -> Self {
        Self {
            state: Mutex::new(SchedulerState {
                queue: PriorityQueue::new(),
                stopped: false,
            }),
            available: Condvar::new(),
        }
    }

    pub(super) fn try_enqueue(
        &self,
        priority: TaskPriority,
        command: Command,
        submission_lock_wait_us: u128,
        control_lock_wait_us: u128,
    ) -> Result<(), EnqueueError> {
        let _trace = TraceSection::new(c"SpinonR05:scheduler-enqueue");
        let lock_started_at = Instant::now();
        let mut state = lock(&self.state);
        let scheduler_lock_wait_us = lock_started_at.elapsed().as_micros();
        if state.stopped {
            return Err(EnqueueError::Stopped);
        }
        if state.queue.len() >= QUEUE_CAPACITY {
            return Err(EnqueueError::Full);
        }

        state.queue.push(
            priority,
            QueuedCommand {
                command,
                // 큐 잠금과 용량 검사를 통과한 뒤, 삽입 직전에 기록해 잠금 대기를 제외합니다.
                enqueued_at: Instant::now(),
                submission_lock_wait_us,
                control_lock_wait_us,
                scheduler_lock_wait_us,
            },
        );
        self.available.notify_one();
        Ok(())
    }

    pub(super) fn receive(&self) -> Option<QueuedCommand> {
        let mut state = lock(&self.state);
        loop {
            if let Some(command) = state.queue.pop_next() {
                return Some(command);
            }

            if state.stopped {
                return None;
            }

            state = {
                let _trace = TraceSection::new(c"SpinonR05:actor-condvar-wait");
                self.available
                    .wait(state)
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            };
        }
    }

    pub(super) fn stop(&self) {
        let mut state = lock(&self.state);
        state.stopped = true;
        self.available.notify_all();
    }

    pub(super) fn reject_pending(&self, status: i32, report: &str) {
        let mut state = lock(&self.state);
        while let Some(command) = state.queue.pop_next() {
            let (sequence, trace_cookie, operation, caller_thread_id, reply) = match command.command
            {
                Command::Eval {
                    sequence,
                    trace_cookie,
                    caller_thread_id,
                    reply,
                    ..
                } => (sequence, trace_cookie, "eval", caller_thread_id, reply),
                Command::Dispatch {
                    sequence,
                    trace_cookie,
                    caller_thread_id,
                    reply,
                    ..
                } => (sequence, trace_cookie, "dispatch", caller_thread_id, reply),
            };
            let mut response = OperationResponse {
                status,
                report: report.to_owned(),
            };
            append_early_error_context(
                &mut response,
                sequence,
                trace_cookie,
                operation,
                caller_thread_id,
                None,
            );
            send_reply(reply, response, trace_cookie);
        }
    }
}
