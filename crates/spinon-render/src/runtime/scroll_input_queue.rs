use std::collections::VecDeque;

use super::{RuntimeScrollGestureCancelReason, RuntimeScrollGestureTerminal};

mod types;

pub use types::{
    RuntimeScrollGestureBeginDisposition, RuntimeScrollGestureIdentity,
    RuntimeScrollGestureTerminalKind, RuntimeScrollInputQueueAdmission,
    RuntimeScrollInputQueueDiagnostics, RuntimeScrollInputQueueError, RuntimeScrollInputQueueEvent,
    RuntimeScrollMomentumPhase, RuntimeScrollWheelSample, RuntimeScrollWheelSource,
    RuntimeScrollWheelUnit,
};
use types::{valid_identity, valid_wheel_sample};

struct TerminalReservation {
    identity: RuntimeScrollGestureIdentity,
    begin_dequeued: bool,
    begin_accepted: bool,
    pan_began_queued: bool,
    pending_terminal: Option<RuntimeScrollInputQueueEvent>,
}

/// Renderer 계산과 분리해 native callback 입력을 보관하는 bounded single-owner queue입니다.
///
/// Platform adapter는 이 queue를 짧은 동기화 경계로 감싸고, dequeue 뒤 session/render 처리를
/// lock 밖에서 해야 합니다. Queue 자체는 OS callback thread나 worker를 기다리지 않습니다.
pub struct RuntimeScrollGestureInputQueue {
    regular_capacity: usize,
    regular: VecDeque<RuntimeScrollInputQueueEvent>,
    terminal_reservations: Vec<Option<TerminalReservation>>,
    last_observed_sequence: Option<u64>,
    last_accepted_sequence: Option<u64>,
    diagnostics: RuntimeScrollInputQueueDiagnostics,
}

impl RuntimeScrollGestureInputQueue {
    /// regular event 수와 동시에 소유할 수 있는 terminal slot 수를 고정합니다.
    pub fn new(
        regular_capacity: usize,
        terminal_capacity: usize,
    ) -> Result<Self, RuntimeScrollInputQueueError> {
        if regular_capacity == 0 || terminal_capacity == 0 {
            return Err(RuntimeScrollInputQueueError::InvalidCapacity);
        }
        let mut regular = VecDeque::new();
        regular
            .try_reserve_exact(regular_capacity)
            .map_err(|_| RuntimeScrollInputQueueError::InvalidCapacity)?;
        let mut terminal_reservations = Vec::new();
        terminal_reservations
            .try_reserve_exact(terminal_capacity)
            .map_err(|_| RuntimeScrollInputQueueError::InvalidCapacity)?;
        terminal_reservations.resize_with(terminal_capacity, || None);
        Ok(Self {
            regular_capacity,
            regular,
            terminal_reservations,
            last_observed_sequence: None,
            last_accepted_sequence: None,
            diagnostics: RuntimeScrollInputQueueDiagnostics::default(),
        })
    }

    /// Terminal slot과 regular slot을 함께 확보한 뒤 Begin을 추가합니다.
    pub fn enqueue_begin(
        &mut self,
        input: super::RuntimeScrollGestureBegin,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        let identity = input.into();
        if !valid_identity(identity) {
            return Err(RuntimeScrollInputQueueError::InvalidGestureIdentity);
        }
        self.observe_sequence(input.sequence())?;
        if self.reservation_index(identity).is_some() {
            return Err(RuntimeScrollInputQueueError::DuplicateGestureIdentity);
        }
        if self.regular.len() >= self.regular_capacity {
            return Err(RuntimeScrollInputQueueError::RegularQueueFull);
        }
        let Some(slot) = self.terminal_reservations.iter().position(Option::is_none) else {
            return Err(RuntimeScrollInputQueueError::TerminalSlotsFull);
        };

        self.terminal_reservations[slot] = Some(TerminalReservation {
            identity,
            begin_dequeued: false,
            begin_accepted: false,
            pan_began_queued: false,
            pending_terminal: None,
        });
        self.regular
            .push_back(RuntimeScrollInputQueueEvent::GestureBegin(input));
        self.record_accepted_sequence(input.sequence());
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    }

    /// Native slop을 넘은 첫 cumulative translation을 regular queue에 넣습니다.
    pub fn enqueue_pan_began(
        &mut self,
        sample: super::RuntimeScrollGesturePanSample,
        policy_epoch: u64,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        let identity = sample.into();
        self.prepare_gesture_input(identity, sample.sequence())?;
        let index = self
            .reservation_index(identity)
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        let reservation = self.terminal_reservations[index]
            .as_ref()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        if reservation.pan_began_queued {
            return Err(RuntimeScrollInputQueueError::PanAlreadyStarted);
        }
        if reservation.pending_terminal.is_some() {
            return Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued);
        }
        if self.regular.len() >= self.regular_capacity {
            self.queue_capacity_cancel(identity, sample.sequence())?;
            return Ok(RuntimeScrollInputQueueAdmission::GestureCancelledForCapacity);
        }
        self.regular
            .push_back(RuntimeScrollInputQueueEvent::PanBegan {
                sample,
                policy_epoch,
            });
        self.terminal_reservations[index]
            .as_mut()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?
            .pan_began_queued = true;
        self.record_accepted_sequence(sample.sequence());
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    }

    /// PanBegan 뒤의 cumulative translation을 병합하거나 regular queue에 넣습니다.
    pub fn enqueue_move(
        &mut self,
        sample: super::RuntimeScrollGesturePanSample,
        policy_epoch: u64,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        let identity = sample.into();
        let previous_accepted_sequence = self.last_accepted_sequence;
        self.prepare_gesture_input(identity, sample.sequence())?;
        if self.terminal_is_pending(identity) {
            return Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued);
        }
        let index = self
            .reservation_index(identity)
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        if !self.terminal_reservations[index]
            .as_ref()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?
            .pan_began_queued
        {
            return Err(RuntimeScrollInputQueueError::PanNotStarted);
        }
        if let Some(RuntimeScrollInputQueueEvent::Move {
            sample: previous,
            policy_epoch: previous_epoch,
            coalesced_samples,
        }) = self.regular.back_mut()
            && RuntimeScrollGestureIdentity::from(*previous) == identity
            && *previous_epoch == policy_epoch
            && previous.expected_key() == sample.expected_key()
            && previous_accepted_sequence == Some(previous.sequence())
        {
            let represented_samples = coalesced_samples.saturating_add(1);
            *previous = sample;
            *coalesced_samples = represented_samples;
            self.diagnostics.record_move_coalescing();
            self.record_accepted_sequence(sample.sequence());
            return Ok(RuntimeScrollInputQueueAdmission::Coalesced {
                represented_samples,
            });
        }
        if self.regular.len() >= self.regular_capacity {
            self.queue_capacity_cancel(identity, sample.sequence())?;
            return Ok(RuntimeScrollInputQueueAdmission::GestureCancelledForCapacity);
        }
        self.regular.push_back(RuntimeScrollInputQueueEvent::Move {
            sample,
            policy_epoch,
            coalesced_samples: 1,
        });
        self.record_accepted_sequence(sample.sequence());
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    }

    /// Gesture End를 미리 예약한 terminal slot에 넣습니다.
    pub fn enqueue_end(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        self.enqueue_terminal(terminal, RuntimeScrollGestureTerminalKind::End)
    }

    /// Gesture Cancel을 미리 예약한 terminal slot에 넣습니다.
    pub fn enqueue_cancel(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
        reason: RuntimeScrollGestureCancelReason,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        self.enqueue_terminal(terminal, RuntimeScrollGestureTerminalKind::Cancel(reason))
    }

    /// Snapshot target과 basis가 고정된 normalized wheel sample을 넣거나 거부합니다.
    pub fn enqueue_wheel(
        &mut self,
        sample: RuntimeScrollWheelSample,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        if !valid_wheel_sample(sample) {
            return Err(RuntimeScrollInputQueueError::InvalidWheelSample);
        }
        let previous_accepted_sequence = self.last_accepted_sequence;
        self.observe_sequence(sample.sequence())?;
        if let Some(RuntimeScrollInputQueueEvent::WheelDelta {
            sample: previous,
            coalesced_samples,
        }) = self.regular.back_mut()
            && previous.coalesce_key_eq(sample)
            && previous_accepted_sequence == Some(previous.sequence())
            && let Some((delta_x, delta_y, raw_x, raw_y)) = checked_sum_deltas(*previous, sample)
        {
            let represented_samples = coalesced_samples.saturating_add(1);
            *previous =
                sample.with_coalesced_deltas(sample.sequence(), delta_x, delta_y, raw_x, raw_y);
            *coalesced_samples = represented_samples;
            self.diagnostics.record_wheel_coalescing();
            self.record_accepted_sequence(sample.sequence());
            return Ok(RuntimeScrollInputQueueAdmission::Coalesced {
                represented_samples,
            });
        }
        if self.regular.len() >= self.regular_capacity {
            self.diagnostics.record_rejected_wheel_sample();
            return Ok(RuntimeScrollInputQueueAdmission::WheelRejectedForCapacity);
        }
        self.regular
            .push_back(RuntimeScrollInputQueueEvent::WheelDelta {
                sample,
                coalesced_samples: 1,
            });
        self.record_accepted_sequence(sample.sequence());
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    }

    /// Begin 결과를 확정합니다. `Tracking`과 `Suppressed`는 terminal reservation을 유지합니다.
    /// `Rejected`면 해당 identity의 대기 입력과 terminal slot을 반환합니다.
    pub fn complete_begin(
        &mut self,
        identity: RuntimeScrollGestureIdentity,
        disposition: RuntimeScrollGestureBeginDisposition,
    ) -> Result<(), RuntimeScrollInputQueueError> {
        let index = self
            .reservation_index(identity)
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        let reservation = self.terminal_reservations[index]
            .as_mut()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        if !reservation.begin_dequeued {
            return Err(RuntimeScrollInputQueueError::BeginNotConsumed);
        }
        if reservation.begin_accepted {
            return Err(RuntimeScrollInputQueueError::BeginAlreadyCompleted);
        }
        if disposition != RuntimeScrollGestureBeginDisposition::Rejected {
            reservation.begin_accepted = true;
            return Ok(());
        }

        self.regular
            .retain(|event| event.gesture_identity() != Some(identity));
        self.terminal_reservations[index] = None;
        Ok(())
    }

    /// Sequence가 가장 작은 regular/terminal 입력을 하나 꺼냅니다.
    pub fn pop_next(
        &mut self,
    ) -> Result<Option<RuntimeScrollInputQueueEvent>, RuntimeScrollInputQueueError> {
        if self
            .terminal_reservations
            .iter()
            .flatten()
            .any(|reservation| reservation.begin_dequeued && !reservation.begin_accepted)
        {
            return Ok(None);
        }

        let regular_sequence = self.regular.front().map(|event| event.sequence());
        let terminal_index = self.oldest_pending_terminal_index();
        let terminal_sequence = terminal_index
            .and_then(|index| self.terminal_reservations[index].as_ref())
            .and_then(|reservation| reservation.pending_terminal)
            .map(|event| event.sequence());

        if terminal_sequence
            .is_some_and(|terminal| regular_sequence.is_none_or(|regular| terminal < regular))
        {
            let index = terminal_index.ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
            let event = self.terminal_reservations[index]
                .as_mut()
                .and_then(|reservation| reservation.pending_terminal.take())
                .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
            self.terminal_reservations[index] = None;
            return Ok(Some(event));
        }

        let Some(event) = self.regular.pop_front() else {
            return Ok(None);
        };
        if let RuntimeScrollInputQueueEvent::GestureBegin(input) = event {
            let identity = input.into();
            let index = self
                .reservation_index(identity)
                .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
            self.terminal_reservations[index]
                .as_mut()
                .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?
                .begin_dequeued = true;
        } else if let Some(identity) = event.gesture_identity() {
            let index = self
                .reservation_index(identity)
                .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
            let reservation = self.terminal_reservations[index]
                .as_ref()
                .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
            if !reservation.begin_accepted {
                return Err(RuntimeScrollInputQueueError::BeginNotConsumed);
            }
        }
        Ok(Some(event))
    }

    /// Regular event queue에 현재 대기 중인 입력 수를 반환합니다.
    pub fn regular_len(&self) -> usize {
        self.regular.len()
    }

    /// 생성 시 고정한 regular event capacity를 반환합니다.
    pub const fn regular_capacity(&self) -> usize {
        self.regular_capacity
    }

    /// 현재 사용 중인 gesture terminal reservation 수를 반환합니다.
    pub fn terminal_reservations_in_use(&self) -> usize {
        self.terminal_reservations
            .iter()
            .filter(|reservation| reservation.is_some())
            .count()
    }

    /// Coalescing·포화·wheel 거부 누적 진단을 복사해 반환합니다.
    pub const fn diagnostics(&self) -> RuntimeScrollInputQueueDiagnostics {
        self.diagnostics
    }

    fn enqueue_terminal(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
        kind: RuntimeScrollGestureTerminalKind,
    ) -> Result<RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError> {
        let identity = terminal.into();
        self.prepare_gesture_input(identity, terminal.sequence())?;
        let index = self
            .reservation_index(identity)
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        let reservation = self.terminal_reservations[index]
            .as_mut()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        if reservation.pending_terminal.is_some() {
            return Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued);
        }
        reservation.pending_terminal =
            Some(RuntimeScrollInputQueueEvent::GestureTerminal { terminal, kind });
        self.record_accepted_sequence(terminal.sequence());
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    }

    fn queue_capacity_cancel(
        &mut self,
        identity: RuntimeScrollGestureIdentity,
        sequence: u64,
    ) -> Result<(), RuntimeScrollInputQueueError> {
        let terminal = RuntimeScrollGestureTerminal::new(
            sequence,
            identity.gesture_id(),
            identity.pointer_id(),
            identity.surface_generation(),
        );
        let index = self
            .reservation_index(identity)
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        let reservation = self.terminal_reservations[index]
            .as_mut()
            .ok_or(RuntimeScrollInputQueueError::UnknownGesture)?;
        if reservation.pending_terminal.is_some() {
            return Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued);
        }
        reservation.pending_terminal = Some(RuntimeScrollInputQueueEvent::GestureTerminal {
            terminal,
            kind: RuntimeScrollGestureTerminalKind::Cancel(
                RuntimeScrollGestureCancelReason::InputQueueSaturated,
            ),
        });
        self.record_accepted_sequence(sequence);
        self.diagnostics.record_gesture_capacity_cancellation();
        Ok(())
    }

    fn prepare_gesture_input(
        &mut self,
        identity: RuntimeScrollGestureIdentity,
        sequence: u64,
    ) -> Result<(), RuntimeScrollInputQueueError> {
        if !valid_identity(identity) {
            return Err(RuntimeScrollInputQueueError::InvalidGestureIdentity);
        }
        self.observe_sequence(sequence)?;
        if self.reservation_index(identity).is_none() {
            return Err(RuntimeScrollInputQueueError::UnknownGesture);
        }
        Ok(())
    }

    fn observe_sequence(&mut self, sequence: u64) -> Result<(), RuntimeScrollInputQueueError> {
        if self
            .last_observed_sequence
            .is_some_and(|last| sequence <= last)
        {
            return Err(RuntimeScrollInputQueueError::NonMonotonicSequence);
        }
        self.last_observed_sequence = Some(sequence);
        Ok(())
    }

    fn record_accepted_sequence(&mut self, sequence: u64) {
        self.last_accepted_sequence = Some(sequence);
    }

    fn reservation_index(&self, identity: RuntimeScrollGestureIdentity) -> Option<usize> {
        self.terminal_reservations
            .iter()
            .position(|reservation| reservation.as_ref().is_some_and(|r| r.identity == identity))
    }

    fn terminal_is_pending(&self, identity: RuntimeScrollGestureIdentity) -> bool {
        self.reservation_index(identity)
            .and_then(|index| self.terminal_reservations[index].as_ref())
            .is_some_and(|reservation| reservation.pending_terminal.is_some())
    }

    fn oldest_pending_terminal_index(&self) -> Option<usize> {
        self.terminal_reservations
            .iter()
            .enumerate()
            .filter_map(|(index, reservation)| {
                let event = reservation.as_ref()?.pending_terminal?;
                Some((index, event.sequence()))
            })
            .min_by_key(|(_, sequence)| *sequence)
            .map(|(index, _)| index)
    }
}

fn checked_sum_deltas(
    previous: RuntimeScrollWheelSample,
    next: RuntimeScrollWheelSample,
) -> Option<(f32, f32, f32, f32)> {
    fn sum(left: f32, right: f32) -> Option<f32> {
        let value = f64::from(left) + f64::from(right);
        (value.is_finite() && value.abs() <= f64::from(f32::MAX)).then_some(value as f32)
    }

    Some((
        sum(previous.delta_x_css_px(), next.delta_x_css_px())?,
        sum(previous.delta_y_css_px(), next.delta_y_css_px())?,
        sum(previous.raw_delta_x(), next.raw_delta_x())?,
        sum(previous.raw_delta_y(), next.raw_delta_y())?,
    ))
}
