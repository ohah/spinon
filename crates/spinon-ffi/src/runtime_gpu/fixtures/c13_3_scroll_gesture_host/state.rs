use super::super::super::lock;
use spinon_core::NodeId;
use spinon_render::{
    RuntimeOverscrollBehavior, RuntimeRenderSnapshot, RuntimeScrollGestureBegin,
    RuntimeScrollGestureCancelReason, RuntimeScrollGestureInputQueue,
    RuntimeScrollGesturePanSample, RuntimeScrollGestureSession, RuntimeScrollGestureTerminal,
    RuntimeTouchAction,
};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy)]
pub(super) struct C13_3GestureOwners {
    pub(in crate::runtime_gpu) root: NodeId,
    pub(in crate::runtime_gpu) outer: NodeId,
    pub(in crate::runtime_gpu) inner: NodeId,
}

struct C13_3GestureMotion {
    pan_began: bool,
    surface_generation: u64,
    gesture_id: u64,
    pointer_id: u64,
    coordinate_scale: f32,
    cumulative_x_css_px: f32,
    cumulative_y_css_px: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum C13_3GestureFixtureScenario {
    AutoZero,
    AutoBoundary,
    ContainXAutoYBoundary,
    TouchActionNone,
    TouchActionPanX,
    TouchActionPanY,
    TouchActionTargetPanX,
    TouchActionIntersectionNone,
}

impl TryFrom<i32> for C13_3GestureFixtureScenario {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AutoZero),
            1 => Ok(Self::AutoBoundary),
            2 => Ok(Self::ContainXAutoYBoundary),
            3 => Ok(Self::TouchActionNone),
            4 => Ok(Self::TouchActionPanX),
            5 => Ok(Self::TouchActionPanY),
            6 => Ok(Self::TouchActionTargetPanX),
            7 => Ok(Self::TouchActionIntersectionNone),
            _ => Err(format!("알 수 없는 C13.3 gesture scenario id={value}")),
        }
    }
}

impl C13_3GestureFixtureScenario {
    pub(super) const fn script_prelude(self) -> &'static str {
        match self {
            Self::AutoZero => "globalThis.__spinonC133GesturePolicyScenario = 0;\n",
            Self::AutoBoundary => "globalThis.__spinonC133GesturePolicyScenario = 1;\n",
            Self::ContainXAutoYBoundary => "globalThis.__spinonC133GesturePolicyScenario = 2;\n",
            Self::TouchActionNone => "globalThis.__spinonC133GesturePolicyScenario = 3;\n",
            Self::TouchActionPanX => "globalThis.__spinonC133GesturePolicyScenario = 4;\n",
            Self::TouchActionPanY => "globalThis.__spinonC133GesturePolicyScenario = 5;\n",
            Self::TouchActionTargetPanX => "globalThis.__spinonC133GesturePolicyScenario = 6;\n",
            Self::TouchActionIntersectionNone => {
                "globalThis.__spinonC133GesturePolicyScenario = 7;\n"
            }
        }
    }

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::AutoZero => "auto-zero",
            Self::AutoBoundary => "auto-boundary",
            Self::ContainXAutoYBoundary => "contain-x-auto-y-boundary",
            Self::TouchActionNone => "touch-action-none",
            Self::TouchActionPanX => "touch-action-pan-x",
            Self::TouchActionPanY => "touch-action-pan-y",
            Self::TouchActionTargetPanX => "touch-action-target-pan-x",
            Self::TouchActionIntersectionNone => "touch-action-intersection-none",
        }
    }

    pub(super) const fn policy_label(self) -> &'static str {
        match self {
            Self::AutoZero
            | Self::AutoBoundary
            | Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY
            | Self::TouchActionTargetPanX
            | Self::TouchActionIntersectionNone => "auto/auto",
            Self::ContainXAutoYBoundary => "contain/auto",
        }
    }

    pub(super) const fn seed_inner_boundary(self) -> bool {
        matches!(self, Self::AutoBoundary | Self::ContainXAutoYBoundary)
    }

    pub(super) const fn inner_overscroll(
        self,
    ) -> (RuntimeOverscrollBehavior, RuntimeOverscrollBehavior) {
        match self {
            Self::AutoZero | Self::AutoBoundary => (
                RuntimeOverscrollBehavior::Auto,
                RuntimeOverscrollBehavior::Auto,
            ),
            Self::ContainXAutoYBoundary => (
                RuntimeOverscrollBehavior::Contain,
                RuntimeOverscrollBehavior::Auto,
            ),
            Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY
            | Self::TouchActionTargetPanX
            | Self::TouchActionIntersectionNone => (
                RuntimeOverscrollBehavior::Auto,
                RuntimeOverscrollBehavior::Auto,
            ),
        }
    }

    pub(super) const fn inner_touch_action(self) -> &'static str {
        match self {
            Self::TouchActionNone => "none",
            Self::TouchActionPanX => "pan-x",
            Self::TouchActionIntersectionNone => "pan-y",
            Self::TouchActionPanY => "pan-y",
            Self::AutoZero
            | Self::AutoBoundary
            | Self::ContainXAutoYBoundary
            | Self::TouchActionTargetPanX => "auto",
        }
    }

    pub(super) const fn inner_content_touch_action(self) -> &'static str {
        match self {
            Self::TouchActionTargetPanX | Self::TouchActionIntersectionNone => "pan-x",
            Self::AutoZero
            | Self::AutoBoundary
            | Self::ContainXAutoYBoundary
            | Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY => "auto",
        }
    }

    pub(super) const fn expected_touch_action(self) -> RuntimeTouchAction {
        match self {
            Self::TouchActionNone | Self::TouchActionIntersectionNone => RuntimeTouchAction::None,
            Self::TouchActionPanX => RuntimeTouchAction::PanX,
            Self::TouchActionPanY => RuntimeTouchAction::PanY,
            Self::TouchActionTargetPanX => RuntimeTouchAction::PanX,
            Self::AutoZero | Self::AutoBoundary | Self::ContainXAutoYBoundary => {
                RuntimeTouchAction::Auto
            }
        }
    }

    pub(super) const fn expected_touch_action_label(self) -> &'static str {
        match self.expected_touch_action() {
            RuntimeTouchAction::Auto => "auto",
            RuntimeTouchAction::None => "none",
            RuntimeTouchAction::PanX => "pan-x",
            RuntimeTouchAction::PanY => "pan-y",
            RuntimeTouchAction::Unsupported => "unsupported",
        }
    }
}

pub(in crate::runtime_gpu) struct C13_3GestureState {
    pub(super) queue: Mutex<RuntimeScrollGestureInputQueue>,
    pub(super) session: Mutex<RuntimeScrollGestureSession>,
    pub(super) snapshot: Mutex<Option<RuntimeRenderSnapshot>>,
    pub(super) owners: Mutex<Option<C13_3GestureOwners>>,
    next_input_sequence: AtomicU64,
    pub(super) scenario: C13_3GestureFixtureScenario,
}

impl C13_3GestureState {
    pub(super) fn new(scenario: C13_3GestureFixtureScenario) -> Result<Self, String> {
        let queue = RuntimeScrollGestureInputQueue::new(64, 8)
            .map_err(|error| format!("C13.3 input queue 생성 실패: {error}"))?;
        Ok(Self {
            queue: Mutex::new(queue),
            session: Mutex::new(RuntimeScrollGestureSession::default()),
            snapshot: Mutex::new(None),
            owners: Mutex::new(None),
            next_input_sequence: AtomicU64::new(0),
            scenario,
        })
    }

    fn next_sequence(&self) -> Result<u64, String> {
        self.next_input_sequence
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                value.checked_add(1)
            })
            .map(|previous| previous + 1)
            .map_err(|_| "C13.3 input sequence가 소진됐습니다".to_owned())
    }

    pub(in crate::runtime_gpu) fn current_snapshot(&self) -> Result<RuntimeRenderSnapshot, String> {
        lock(&self.snapshot)
            .as_ref()
            .cloned()
            .ok_or_else(|| "C13.3 fixture snapshot이 아직 준비되지 않았습니다".to_owned())
    }

    pub(in crate::runtime_gpu) fn enqueue_begin(
        &self,
        surface_generation: u64,
        gesture_id: u64,
        pointer_id: u64,
        x_css_px: f32,
        y_css_px: f32,
        coordinate_scale: f32,
    ) -> Result<String, String> {
        let snapshot = self.current_snapshot()?;
        let sequence = self.next_sequence()?;
        let resolved_touch_action = snapshot
            .touch_action_at_css_point(x_css_px, y_css_px)
            .unwrap_or(RuntimeTouchAction::Unsupported);
        let input = RuntimeScrollGestureBegin::new(
            snapshot.key(),
            surface_generation,
            sequence,
            gesture_id,
            pointer_id,
            x_css_px,
            y_css_px,
            coordinate_scale,
            resolved_touch_action,
        );
        let admission = lock(&self.queue)
            .enqueue_begin(input)
            .map_err(|error| format!("C13.3 Begin 거부: {error}"))?;
        Ok(format!(
            "sequence={sequence} admission={admission:?} key={:?}",
            snapshot.key()
        ))
    }

    pub(in crate::runtime_gpu) fn enqueue_pan_began(
        &self,
        surface_generation: u64,
        gesture_id: u64,
        pointer_id: u64,
        coordinate_scale: f32,
        cumulative_x_css_px: f32,
        cumulative_y_css_px: f32,
    ) -> Result<String, String> {
        self.enqueue_motion(C13_3GestureMotion {
            pan_began: true,
            surface_generation,
            gesture_id,
            pointer_id,
            coordinate_scale,
            cumulative_x_css_px,
            cumulative_y_css_px,
        })
    }

    pub(in crate::runtime_gpu) fn enqueue_move(
        &self,
        surface_generation: u64,
        gesture_id: u64,
        pointer_id: u64,
        coordinate_scale: f32,
        cumulative_x_css_px: f32,
        cumulative_y_css_px: f32,
    ) -> Result<String, String> {
        self.enqueue_motion(C13_3GestureMotion {
            pan_began: false,
            surface_generation,
            gesture_id,
            pointer_id,
            coordinate_scale,
            cumulative_x_css_px,
            cumulative_y_css_px,
        })
    }

    fn enqueue_motion(&self, motion: C13_3GestureMotion) -> Result<String, String> {
        let snapshot = self.current_snapshot()?;
        let sequence = self.next_sequence()?;
        let sample = RuntimeScrollGesturePanSample::new(
            snapshot.key(),
            motion.surface_generation,
            sequence,
            motion.gesture_id,
            motion.pointer_id,
            motion.coordinate_scale,
            motion.cumulative_x_css_px,
            motion.cumulative_y_css_px,
        );
        let policy_epoch = snapshot.key().style_revision().get();
        let admission = if motion.pan_began {
            lock(&self.queue).enqueue_pan_began(sample, policy_epoch)
        } else {
            lock(&self.queue).enqueue_move(sample, policy_epoch)
        }
        .map_err(|error| format!("C13.3 motion 거부: {error}"))?;
        Ok(format!(
            "sequence={sequence} admission={admission:?} policy_epoch={policy_epoch}"
        ))
    }

    pub(in crate::runtime_gpu) fn enqueue_terminal(
        &self,
        end: bool,
        surface_generation: u64,
        gesture_id: u64,
        pointer_id: u64,
        reason: RuntimeScrollGestureCancelReason,
    ) -> Result<String, String> {
        let sequence = self.next_sequence()?;
        let terminal =
            RuntimeScrollGestureTerminal::new(sequence, gesture_id, pointer_id, surface_generation);
        let admission = if end {
            lock(&self.queue).enqueue_end(terminal)
        } else {
            lock(&self.queue).enqueue_cancel(terminal, reason)
        }
        .map_err(|error| format!("C13.3 terminal 거부: {error}"))?;
        Ok(format!(
            "sequence={sequence} kind={} admission={admission:?} reason={reason:?}",
            if end { "end" } else { "cancel" }
        ))
    }

    pub(super) fn publish_snapshot(&self, snapshot: RuntimeRenderSnapshot) {
        *lock(&self.snapshot) = Some(snapshot);
    }

    pub(super) fn owners(&self) -> Result<C13_3GestureOwners, String> {
        lock(&self.owners)
            .as_ref()
            .copied()
            .ok_or_else(|| "C13.3 owner identity가 준비되지 않았습니다".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::C13_3GestureFixtureScenario as Scenario;
    use spinon_render::RuntimeOverscrollBehavior::{Auto, Contain};

    #[test]
    fn policy_scenario_ids_map_without_fallback() {
        let cases = [
            (
                0,
                "auto-zero",
                "auto/auto",
                false,
                (Auto, Auto),
                "auto",
                "auto",
                "auto",
            ),
            (
                1,
                "auto-boundary",
                "auto/auto",
                true,
                (Auto, Auto),
                "auto",
                "auto",
                "auto",
            ),
            (
                2,
                "contain-x-auto-y-boundary",
                "contain/auto",
                true,
                (Contain, Auto),
                "auto",
                "auto",
                "auto",
            ),
            (
                3,
                "touch-action-none",
                "auto/auto",
                false,
                (Auto, Auto),
                "none",
                "none",
                "auto",
            ),
            (
                4,
                "touch-action-pan-x",
                "auto/auto",
                false,
                (Auto, Auto),
                "pan-x",
                "pan-x",
                "auto",
            ),
            (
                5,
                "touch-action-pan-y",
                "auto/auto",
                false,
                (Auto, Auto),
                "pan-y",
                "pan-y",
                "auto",
            ),
            (
                6,
                "touch-action-target-pan-x",
                "auto/auto",
                false,
                (Auto, Auto),
                "pan-x",
                "auto",
                "pan-x",
            ),
            (
                7,
                "touch-action-intersection-none",
                "auto/auto",
                false,
                (Auto, Auto),
                "none",
                "pan-y",
                "pan-x",
            ),
        ];
        for (
            id,
            label,
            policy,
            seeded,
            overscroll,
            touch_action,
            inner_touch_action,
            inner_content_touch_action,
        ) in cases
        {
            let scenario = Scenario::try_from(id).expect("known scenario id");
            assert_eq!(scenario.label(), label);
            assert_eq!(scenario.policy_label(), policy);
            assert_eq!(scenario.seed_inner_boundary(), seeded);
            assert_eq!(scenario.inner_overscroll(), overscroll);
            assert_eq!(scenario.expected_touch_action_label(), touch_action);
            assert_eq!(scenario.inner_touch_action(), inner_touch_action);
            assert_eq!(
                scenario.inner_content_touch_action(),
                inner_content_touch_action
            );
            let expected_prelude_assignment = format!("= {id};");
            assert!(
                scenario
                    .script_prelude()
                    .contains(expected_prelude_assignment.as_str())
            );
        }
        assert!(Scenario::try_from(99).is_err());
        assert!(Scenario::try_from(-1).is_err());
    }
}
