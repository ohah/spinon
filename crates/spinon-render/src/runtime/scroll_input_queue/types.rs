use spinon_core::{NodeId, ScrollStateRevision};

use super::super::{
    RuntimeRenderKey, RuntimeScrollGestureBegin, RuntimeScrollGestureCancelReason,
    RuntimeScrollGesturePanSample, RuntimeScrollGestureTerminal,
};

/// 입력 queue에서 한 pointer gesture를 고유하게 식별합니다.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RuntimeScrollGestureIdentity {
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
}

impl RuntimeScrollGestureIdentity {
    /// Gesture, pointer 및 표면 세대를 묶어 identity를 만듭니다.
    pub const fn new(gesture_id: u64, pointer_id: u64, surface_generation: u64) -> Self {
        Self {
            gesture_id,
            pointer_id,
            surface_generation,
        }
    }

    /// Gesture ID를 반환합니다.
    pub const fn gesture_id(self) -> u64 {
        self.gesture_id
    }

    /// Native pointer ID를 반환합니다.
    pub const fn pointer_id(self) -> u64 {
        self.pointer_id
    }

    /// 입력을 발생시킨 surface generation을 반환합니다.
    pub const fn surface_generation(self) -> u64 {
        self.surface_generation
    }
}

impl From<RuntimeScrollGestureBegin> for RuntimeScrollGestureIdentity {
    fn from(input: RuntimeScrollGestureBegin) -> Self {
        Self::new(
            input.gesture_id(),
            input.pointer_id(),
            input.surface_generation(),
        )
    }
}

impl From<RuntimeScrollGesturePanSample> for RuntimeScrollGestureIdentity {
    fn from(input: RuntimeScrollGesturePanSample) -> Self {
        Self::new(
            input.gesture_id(),
            input.pointer_id(),
            input.surface_generation(),
        )
    }
}

impl From<RuntimeScrollGestureTerminal> for RuntimeScrollGestureIdentity {
    fn from(input: RuntimeScrollGestureTerminal) -> Self {
        Self::new(
            input.gesture_id(),
            input.pointer_id(),
            input.surface_generation(),
        )
    }
}

/// Wheel sample을 coalesce할 때 구별하는 입력 의도입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollWheelSource {
    /// Mouse wheel source입니다.
    MouseWheel,
    /// Scroll intent를 가진 trackpad source입니다.
    TrackpadScroll,
}

/// Host가 정규화한 trackpad momentum 단계입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollMomentumPhase {
    /// 별도 momentum phase가 없습니다.
    None,
    /// Momentum sequence가 시작됐습니다.
    Begin,
    /// Momentum sequence가 진행 중입니다.
    Change,
    /// Momentum sequence가 끝났습니다.
    End,
}

/// Queue에 들어오기 전 OS가 보고한 delta 단위입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollWheelUnit {
    /// CSS pixel 단위로 이미 정규화된 값입니다.
    CssPixels,
    /// Android native physical pixel 단위입니다.
    AndroidPhysicalPixels,
    /// UIKit point 단위입니다.
    UIKitPoints,
    /// OS wheel line 단위입니다.
    WheelLines,
    /// OS wheel page 단위입니다.
    WheelPages,
}

/// Snapshot에서 hit target을 정한 뒤 queue에 넣는 CSS px wheel 입력입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollWheelSample {
    sequence: u64,
    surface_generation: u64,
    expected_key: RuntimeRenderKey,
    basis_scroll_revision: ScrollStateRevision,
    point_x_css_px: f32,
    point_y_css_px: f32,
    delta_x_css_px: f32,
    delta_y_css_px: f32,
    raw_delta_x: f32,
    raw_delta_y: f32,
    source_unit: RuntimeScrollWheelUnit,
    coordinate_scale: f32,
    target: NodeId,
    policy_epoch: u64,
    source: RuntimeScrollWheelSource,
    momentum_phase: RuntimeScrollMomentumPhase,
    modifier_flags: u32,
}

impl RuntimeScrollWheelSample {
    /// Raw unit과 CSS px delta, snapshot target stamp를 함께 보존하는 sample을 만듭니다.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        sequence: u64,
        surface_generation: u64,
        expected_key: RuntimeRenderKey,
        basis_scroll_revision: ScrollStateRevision,
        point_x_css_px: f32,
        point_y_css_px: f32,
        delta_x_css_px: f32,
        delta_y_css_px: f32,
        raw_delta_x: f32,
        raw_delta_y: f32,
        source_unit: RuntimeScrollWheelUnit,
        coordinate_scale: f32,
        target: NodeId,
        policy_epoch: u64,
        source: RuntimeScrollWheelSource,
        momentum_phase: RuntimeScrollMomentumPhase,
        modifier_flags: u32,
    ) -> Self {
        Self {
            sequence,
            surface_generation,
            expected_key,
            basis_scroll_revision,
            point_x_css_px,
            point_y_css_px,
            delta_x_css_px,
            delta_y_css_px,
            raw_delta_x,
            raw_delta_y,
            source_unit,
            coordinate_scale,
            target,
            policy_epoch,
            source,
            momentum_phase,
            modifier_flags,
        }
    }

    /// Queue의 순서 번호를 반환합니다.
    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    /// Sample에 stamp된 surface generation을 반환합니다.
    pub const fn surface_generation(self) -> u64 {
        self.surface_generation
    }

    /// Hit target과 policy 계산에 사용한 snapshot key를 반환합니다.
    pub const fn expected_key(self) -> RuntimeRenderKey {
        self.expected_key
    }

    /// Enqueue 시점의 scroll revision을 반환합니다.
    pub const fn basis_scroll_revision(self) -> ScrollStateRevision {
        self.basis_scroll_revision
    }

    /// CSS viewport X 좌표를 반환합니다.
    pub const fn point_x_css_px(self) -> f32 {
        self.point_x_css_px
    }

    /// CSS viewport Y 좌표를 반환합니다.
    pub const fn point_y_css_px(self) -> f32 {
        self.point_y_css_px
    }

    /// 정규화된 CSS px X delta를 반환합니다.
    pub const fn delta_x_css_px(self) -> f32 {
        self.delta_x_css_px
    }

    /// 정규화된 CSS px Y delta를 반환합니다.
    pub const fn delta_y_css_px(self) -> f32 {
        self.delta_y_css_px
    }

    /// 원본 단위의 X delta를 반환합니다.
    pub const fn raw_delta_x(self) -> f32 {
        self.raw_delta_x
    }

    /// 원본 단위의 Y delta를 반환합니다.
    pub const fn raw_delta_y(self) -> f32 {
        self.raw_delta_y
    }

    /// Raw delta가 보고된 단위를 반환합니다.
    pub const fn source_unit(self) -> RuntimeScrollWheelUnit {
        self.source_unit
    }

    /// Point·delta CSS 변환에 사용한 scale을 반환합니다.
    pub const fn coordinate_scale(self) -> f32 {
        self.coordinate_scale
    }

    /// Enqueue 시점에 결정한 hit target을 반환합니다.
    pub const fn target(self) -> NodeId {
        self.target
    }

    /// Computed input policy epoch를 반환합니다.
    pub const fn policy_epoch(self) -> u64 {
        self.policy_epoch
    }

    /// Host input source를 반환합니다.
    pub const fn source(self) -> RuntimeScrollWheelSource {
        self.source
    }

    /// Host가 제공한 momentum phase를 반환합니다.
    pub const fn momentum_phase(self) -> RuntimeScrollMomentumPhase {
        self.momentum_phase
    }

    /// Host의 modifier bit flags를 반환합니다.
    pub const fn modifier_flags(self) -> u32 {
        self.modifier_flags
    }

    pub(super) fn with_coalesced_deltas(
        self,
        sequence: u64,
        delta_x_css_px: f32,
        delta_y_css_px: f32,
        raw_delta_x: f32,
        raw_delta_y: f32,
    ) -> Self {
        Self {
            sequence,
            delta_x_css_px,
            delta_y_css_px,
            raw_delta_x,
            raw_delta_y,
            ..self
        }
    }

    pub(super) fn coalesce_key_eq(self, other: Self) -> bool {
        self.surface_generation == other.surface_generation
            && self.expected_key == other.expected_key
            && self.basis_scroll_revision == other.basis_scroll_revision
            && canonical_bits(self.point_x_css_px) == canonical_bits(other.point_x_css_px)
            && canonical_bits(self.point_y_css_px) == canonical_bits(other.point_y_css_px)
            && self.target == other.target
            && self.policy_epoch == other.policy_epoch
            && self.source == other.source
            && self.momentum_phase == other.momentum_phase
            && self.source_unit == other.source_unit
            && canonical_bits(self.coordinate_scale) == canonical_bits(other.coordinate_scale)
            && self.modifier_flags == other.modifier_flags
    }
}

/// 예약된 terminal control slot에 들어가는 End 또는 Cancel입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollGestureTerminalKind {
    /// 정상 touch release입니다.
    End,
    /// Explicit host/native cancel입니다.
    Cancel(RuntimeScrollGestureCancelReason),
}

/// Native session이 Begin 입력을 보유한 방식입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollGestureBeginDisposition {
    /// Session이 pointer를 Tracking 상태로 보유합니다.
    Tracking,
    /// Session이 cancelled/suppressed contact를 terminal까지 보유합니다.
    Suppressed,
    /// Session이 Begin identity를 보유하지 않습니다.
    Rejected,
}

/// Queue에서 소비할 입력 이벤트입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuntimeScrollInputQueueEvent {
    /// 새 pointer stream을 시작합니다.
    GestureBegin(RuntimeScrollGestureBegin),
    /// Native slop/recognizer가 pan 시작을 확인했습니다.
    PanBegan {
        sample: RuntimeScrollGesturePanSample,
        policy_epoch: u64,
    },
    /// 누적 direct-pan translation입니다.
    Move {
        sample: RuntimeScrollGesturePanSample,
        policy_epoch: u64,
        coalesced_samples: u32,
    },
    /// 예약 slot에서 처리하는 End/Cancel입니다.
    GestureTerminal {
        terminal: RuntimeScrollGestureTerminal,
        kind: RuntimeScrollGestureTerminalKind,
    },
    /// 정규화된 mouse wheel/trackpad delta입니다.
    WheelDelta {
        sample: RuntimeScrollWheelSample,
        coalesced_samples: u32,
    },
}

impl RuntimeScrollInputQueueEvent {
    pub const fn sequence(self) -> u64 {
        match self {
            Self::GestureBegin(input) => input.sequence(),
            Self::PanBegan { sample, .. } | Self::Move { sample, .. } => sample.sequence(),
            Self::GestureTerminal { terminal, .. } => terminal.sequence(),
            Self::WheelDelta { sample, .. } => sample.sequence(),
        }
    }

    pub fn gesture_identity(self) -> Option<RuntimeScrollGestureIdentity> {
        match self {
            Self::GestureBegin(input) => Some(input.into()),
            Self::PanBegan { sample, .. } | Self::Move { sample, .. } => Some(sample.into()),
            Self::GestureTerminal { terminal, .. } => Some(terminal.into()),
            Self::WheelDelta { .. } => None,
        }
    }
}

/// Enqueue 시 capacity/coalescing 결과입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollInputQueueAdmission {
    /// 새 input event가 queue에 들어갔습니다.
    Queued,
    /// 새 sample이 tail sample과 병합됐습니다.
    Coalesced { represented_samples: u32 },
    /// Gesture motion을 받을 수 없어 reserved cancel이 들어갔습니다.
    GestureCancelledForCapacity,
    /// Wheel sample 전체가 capacity에서 거부됐습니다.
    WheelRejectedForCapacity,
}

/// Queue pressure와 coalescing을 누적하는 진단 카운터입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RuntimeScrollInputQueueDiagnostics {
    coalesced_move_samples: u64,
    coalesced_wheel_samples: u64,
    gesture_capacity_cancellations: u64,
    rejected_wheel_samples: u64,
}

impl RuntimeScrollInputQueueDiagnostics {
    /// Coalescing으로 흡수한 Move 수를 반환합니다.
    pub const fn coalesced_move_samples(self) -> u64 {
        self.coalesced_move_samples
    }

    /// Coalescing으로 흡수한 wheel sample 수를 반환합니다.
    pub const fn coalesced_wheel_samples(self) -> u64 {
        self.coalesced_wheel_samples
    }

    /// Capacity로 예약 Cancel한 gesture 수를 반환합니다.
    pub const fn gesture_capacity_cancellations(self) -> u64 {
        self.gesture_capacity_cancellations
    }

    /// Capacity에서 거부한 wheel sample 수를 반환합니다.
    pub const fn rejected_wheel_samples(self) -> u64 {
        self.rejected_wheel_samples
    }

    pub(super) fn record_move_coalescing(&mut self) {
        self.coalesced_move_samples = self.coalesced_move_samples.saturating_add(1);
    }

    pub(super) fn record_wheel_coalescing(&mut self) {
        self.coalesced_wheel_samples = self.coalesced_wheel_samples.saturating_add(1);
    }

    pub(super) fn record_gesture_capacity_cancellation(&mut self) {
        self.gesture_capacity_cancellations = self.gesture_capacity_cancellations.saturating_add(1);
    }

    pub(super) fn record_rejected_wheel_sample(&mut self) {
        self.rejected_wheel_samples = self.rejected_wheel_samples.saturating_add(1);
    }
}

/// Bounded queue의 입력 오류입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollInputQueueError {
    InvalidCapacity,
    NonMonotonicSequence,
    InvalidGestureIdentity,
    DuplicateGestureIdentity,
    RegularQueueFull,
    TerminalSlotsFull,
    UnknownGesture,
    TerminalAlreadyQueued,
    BeginNotConsumed,
    BeginAlreadyCompleted,
    PanNotStarted,
    PanAlreadyStarted,
    InvalidWheelSample,
}

impl std::fmt::Display for RuntimeScrollInputQueueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "scroll input queue error: {self:?}")
    }
}

impl std::error::Error for RuntimeScrollInputQueueError {}

pub(super) fn valid_identity(identity: RuntimeScrollGestureIdentity) -> bool {
    identity.gesture_id != 0
}

pub(super) fn valid_wheel_sample(sample: RuntimeScrollWheelSample) -> bool {
    sample.point_x_css_px.is_finite()
        && sample.point_y_css_px.is_finite()
        && sample.delta_x_css_px.is_finite()
        && sample.delta_y_css_px.is_finite()
        && sample.raw_delta_x.is_finite()
        && sample.raw_delta_y.is_finite()
        && sample.coordinate_scale.is_finite()
        && sample.coordinate_scale > 0.0
        && sample.basis_scroll_revision == sample.expected_key.scroll_state_revision()
}

pub(super) fn canonical_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}
