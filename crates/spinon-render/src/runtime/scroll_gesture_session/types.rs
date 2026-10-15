use spinon_core::NodeId;

use super::super::{RuntimeRenderKey, RuntimeScrollDeltaOutcome};

/// C13.3 session에 넘기는 style cascade의 제한 touch-action 값입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeTouchAction {
    /// 두 물리 축 pan을 허용합니다.
    Auto,
    /// direct pan을 허용하지 않습니다.
    None,
    /// 수평 pan만 허용합니다.
    PanX,
    /// 수직 pan만 허용합니다.
    PanY,
    /// 계산 값이 현재 gesture profile에서 지원되지 않습니다.
    Unsupported,
}

/// 시작 시 touch-action으로 허용한 물리 축 집합입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeGestureAxes {
    pub(super) x: bool,
    pub(super) y: bool,
}

impl RuntimeGestureAxes {
    /// 수평 축이 허용되는지 반환합니다.
    pub const fn x(self) -> bool {
        self.x
    }

    /// 수직 축이 허용되는지 반환합니다.
    pub const fn y(self) -> bool {
        self.y
    }

    pub(super) const fn intersection(self, other: Self) -> Self {
        Self {
            x: self.x && other.x,
            y: self.y && other.y,
        }
    }

    pub(super) const fn is_empty(self) -> bool {
        !self.x && !self.y
    }
}

/// native adapter가 CSS px로 정규화한 direct-pan 시작 입력입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollGestureBegin {
    pub(super) expected_key: RuntimeRenderKey,
    pub(super) surface_generation: u64,
    pub(super) sequence: u64,
    pub(super) gesture_id: u64,
    pub(super) pointer_id: u64,
    pub(super) point_x_css_px: f32,
    pub(super) point_y_css_px: f32,
    pub(super) coordinate_scale: f32,
    pub(super) resolved_touch_action: RuntimeTouchAction,
}

impl RuntimeScrollGestureBegin {
    /// revision과 CSS viewport 좌표가 지정된 Begin 입력을 만듭니다.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        expected_key: RuntimeRenderKey,
        surface_generation: u64,
        sequence: u64,
        gesture_id: u64,
        pointer_id: u64,
        point_x_css_px: f32,
        point_y_css_px: f32,
        coordinate_scale: f32,
        resolved_touch_action: RuntimeTouchAction,
    ) -> Self {
        Self {
            expected_key,
            surface_generation,
            sequence,
            gesture_id,
            pointer_id,
            point_x_css_px,
            point_y_css_px,
            coordinate_scale,
            resolved_touch_action,
        }
    }

    pub const fn expected_key(self) -> RuntimeRenderKey {
        self.expected_key
    }

    pub const fn surface_generation(self) -> u64 {
        self.surface_generation
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    pub const fn gesture_id(self) -> u64 {
        self.gesture_id
    }

    pub const fn pointer_id(self) -> u64 {
        self.pointer_id
    }
}

/// native slop 경계 또는 다음 cumulative pan translation입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollGesturePanSample {
    pub(super) expected_key: RuntimeRenderKey,
    pub(super) surface_generation: u64,
    pub(super) sequence: u64,
    pub(super) gesture_id: u64,
    pub(super) pointer_id: u64,
    pub(super) coordinate_scale: f32,
    pub(super) cumulative_x_css_px: f32,
    pub(super) cumulative_y_css_px: f32,
}

impl RuntimeScrollGesturePanSample {
    /// 누적 CSS px translation을 가진 pan 입력을 만듭니다.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        expected_key: RuntimeRenderKey,
        surface_generation: u64,
        sequence: u64,
        gesture_id: u64,
        pointer_id: u64,
        coordinate_scale: f32,
        cumulative_x_css_px: f32,
        cumulative_y_css_px: f32,
    ) -> Self {
        Self {
            expected_key,
            surface_generation,
            sequence,
            gesture_id,
            pointer_id,
            coordinate_scale,
            cumulative_x_css_px,
            cumulative_y_css_px,
        }
    }

    pub const fn expected_key(self) -> RuntimeRenderKey {
        self.expected_key
    }

    pub const fn surface_generation(self) -> u64 {
        self.surface_generation
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    pub const fn gesture_id(self) -> u64 {
        self.gesture_id
    }

    pub const fn pointer_id(self) -> u64 {
        self.pointer_id
    }

    pub const fn cumulative_x_css_px(self) -> f32 {
        self.cumulative_x_css_px
    }

    pub const fn cumulative_y_css_px(self) -> f32 {
        self.cumulative_y_css_px
    }
}

/// Gesture terminal event의 순서와 identity입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeScrollGestureTerminal {
    pub(super) sequence: u64,
    pub(super) gesture_id: u64,
    pub(super) pointer_id: u64,
    pub(super) surface_generation: u64,
}

impl RuntimeScrollGestureTerminal {
    /// End 또는 Cancel의 순서와 pointer identity를 만듭니다.
    pub const fn new(
        sequence: u64,
        gesture_id: u64,
        pointer_id: u64,
        surface_generation: u64,
    ) -> Self {
        Self {
            sequence,
            gesture_id,
            pointer_id,
            surface_generation,
        }
    }

    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    pub const fn gesture_id(self) -> u64 {
        self.gesture_id
    }

    pub const fn pointer_id(self) -> u64 {
        self.pointer_id
    }

    pub const fn surface_generation(self) -> u64 {
        self.surface_generation
    }
}

/// Session 상태를 바꾼 accepted input의 결과입니다.
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeScrollGestureOutcome {
    /// Begin에서 확정한 hit target, owner chain과 허용 축입니다.
    Began {
        target: NodeId,
        owner_chain: Vec<NodeId>,
        allowed_axes: RuntimeGestureAxes,
    },
    /// 첫 pan 입력을 수락해 active 축과 delta 결과를 반환합니다.
    PanStarted {
        axes: RuntimeGestureAxes,
        delta: RuntimeScrollDeltaOutcome,
    },
    /// 시작 방향이 touch-action으로 허용되지 않아 pan을 거부했습니다.
    PanRejected,
    /// 누적 translation을 처리한 결과입니다. Tracking 중에는 delta가 없습니다.
    Moved {
        delta: Option<RuntimeScrollDeltaOutcome>,
    },
    /// revision, geometry, pointer 또는 host 경계로 gesture가 취소됐습니다.
    Cancelled(RuntimeScrollGestureCancelReason),
    /// active pointer의 정상 terminal입니다.
    Ended,
    /// 취소 후 억제하던 pointer 하나가 끝났습니다.
    SuppressedPointerEnded { blocked_contacts_remaining: usize },
}

/// 현재 core session의 단일 pointer 상태입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollGestureState {
    /// 활성 gesture나 억제 중인 pointer가 없습니다.
    Idle,
    /// Begin을 받고 native pan 시작을 기다립니다.
    Tracking,
    /// 지정된 축으로 pan 중입니다.
    Panning(RuntimeGestureAxes),
    /// 첫 pan 방향이 허용되지 않아 해당 gesture를 종료 대기합니다.
    PanRejected,
    /// 취소된 contact의 terminal을 기다립니다.
    Suppressed,
}

/// Session이 active gesture를 종료한 이유입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollGestureCancelReason {
    /// 두 번째 pointer가 active gesture에 들어왔습니다.
    SecondPointer,
    /// frozen target/clip/owner/range geometry가 바뀌었습니다.
    GeometryChanged,
    /// 문서 generation 또는 tree/document revision이 바뀌었습니다.
    DocumentOrTreeChanged,
    /// 환경 revision이 바뀌었습니다.
    EnvironmentChanged,
    /// surface generation 또는 coordinate scale이 바뀌었습니다.
    SurfaceGenerationChanged,
    /// session이 소유하지 않은 scroll revision이 관찰됐습니다.
    ExternalScrollUpdate,
    /// host가 pointer stream을 취소했습니다.
    HostCancelled,
    /// native pan recognizer가 실패했습니다.
    NativeRecognizerFailed,
    /// snapshot delta router가 오류를 반환했습니다.
    RenderFailure,
    /// bounded native input queue가 gesture motion을 수용하지 못했습니다.
    InputQueueSaturated,
}
