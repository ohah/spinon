use super::super::super::lock;
use spinon_core::NodeId;
use spinon_render::{
    RuntimeRenderSnapshot, RuntimeScrollGestureBegin, RuntimeScrollGestureCancelReason,
    RuntimeScrollGestureInputQueue, RuntimeScrollGesturePanSample, RuntimeScrollGestureSession,
    RuntimeScrollGestureTerminal, RuntimeTouchAction,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

pub(super) mod wheel;
use super::scenario::C13_3GestureFixtureScenario;
use wheel::C13_3WheelState;

#[derive(Clone, Copy)]
pub(super) struct C13_3GestureOwners {
    pub(in crate::runtime_gpu) root: NodeId,
    pub(in crate::runtime_gpu) outer: NodeId,
    pub(in crate::runtime_gpu) inner: NodeId,
    pub(in crate::runtime_gpu) inner_content: NodeId,
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

pub(in crate::runtime_gpu) struct C13_3GestureState {
    pub(super) queue: Mutex<RuntimeScrollGestureInputQueue>,
    pub(super) session: Mutex<RuntimeScrollGestureSession>,
    pub(super) snapshot: Mutex<Option<RuntimeRenderSnapshot>>,
    pub(super) owners: Mutex<Option<C13_3GestureOwners>>,
    wheel: Mutex<C13_3WheelState>,
    surface_update: Mutex<()>,
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
            wheel: Mutex::new(C13_3WheelState::default()),
            surface_update: Mutex::new(()),
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

    pub(in crate::runtime_gpu) fn ready_snapshot(&self) -> Option<RuntimeRenderSnapshot> {
        if lock(&self.owners).is_none() {
            return None;
        }
        lock(&self.snapshot).clone()
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
        self.note_surface_generation(surface_generation);
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
        self.note_surface_generation(motion.surface_generation);
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
        self.note_surface_generation(surface_generation);
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

    pub(in crate::runtime_gpu) fn publish_snapshot(&self, snapshot: RuntimeRenderSnapshot) {
        let _surface_update = self.lock_surface_update();
        self.reset_wheel_lineage();
        *lock(&self.snapshot) = Some(snapshot);
    }

    pub(in crate::runtime_gpu) fn lock_surface_update(&self) -> MutexGuard<'_, ()> {
        lock(&self.surface_update)
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

    #[test]
    fn style_invalidation_scenarios_have_fixed_modes_and_start_policies() {
        let cases = [
            (8, "style-policy-invalidation", Some(8), true, "pan-x"),
            (9, "geometry-invalidation", Some(9), true, "auto"),
            (10, "paint-invalidation", Some(10), false, "auto"),
        ];
        for (id, label, mode, seeded, touch_action) in cases {
            let scenario = Scenario::try_from(id).expect("known style scenario");
            assert_eq!(scenario.label(), label);
            assert_eq!(scenario.style_invalidation_mode(), mode);
            assert_eq!(
                scenario.style_invalidation_source().is_some(),
                mode.is_some()
            );
            assert_eq!(scenario.seed_inner_boundary(), seeded);
            assert_eq!(scenario.expected_touch_action_label(), touch_action);
            assert!(scenario.script_prelude().contains("__spinonC133StyleMode"));
        }
    }
}
