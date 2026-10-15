mod state;
mod style_invalidation;

use super::super::{
    ERR_ARGUMENT, ERR_LAYOUT, ERR_STALE, RuntimeGpuHost, lock, scene_matches_layout_key,
};
use super::c13_3_scroll_gesture::RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE;
use spinon_core::NodeId;
use spinon_render::{
    RuntimeOverscrollBehavior, RuntimeRenderSnapshot, RuntimeScrollGestureBeginDisposition,
    RuntimeScrollGestureOutcome, RuntimeScrollGesturePanSample, RuntimeScrollGestureTerminalKind,
    RuntimeScrollInputQueueEvent,
};
use spinon_runtime::RuntimeSession;
pub(in crate::runtime_gpu) use state::C13_3GestureState;
use state::{C13_3GestureFixtureScenario, C13_3GestureOwners};
use std::sync::Arc;

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn new_c13_3_scroll_gesture_fixture(
        scenario_id: i32,
    ) -> Result<(Self, String), String> {
        let scenario = C13_3GestureFixtureScenario::try_from(scenario_id)?;
        let (session, report) =
            RuntimeSession::new_runtime_gpu_block_flex_scroll_gesture_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.c13_3_gesture_state = Some(Arc::new(C13_3GestureState::new(scenario)?));
        Ok((host, format!("{report} scenario={}", scenario.label())))
    }

    pub(in crate::runtime_gpu) fn c13_3_gesture_state(&self) -> Option<Arc<C13_3GestureState>> {
        self.c13_3_gesture_state.as_ref().map(Arc::clone)
    }

    pub(in crate::runtime_gpu) fn eval_c13_3_scroll_gesture_fixture(
        &self,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let state = self.c13_3_gesture_state.as_ref().ok_or((
            ERR_ARGUMENT,
            "C13.3 gesture fixture host가 아닙니다".to_owned(),
        ))?;
        let source_length = state
            .scenario
            .script_prelude()
            .len()
            .checked_add(RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE.len())
            .ok_or((
                ERR_ARGUMENT,
                "C13.3 fixture source 크기가 넘쳤습니다".to_owned(),
            ))?;
        let mut source = String::new();
        source.try_reserve_exact(source_length).map_err(|_| {
            (
                ERR_ARGUMENT,
                "C13.3 fixture source 메모리를 확보할 수 없습니다".to_owned(),
            )
        })?;
        source.push_str(state.scenario.script_prelude());
        source.push_str(RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE);
        let report = self.eval(&source, layout_timeout_millis)?;
        if !report.contains("layout=ready") {
            return Err((
                ERR_LAYOUT,
                format!("C13.3 gesture fixture layout가 준비되지 않았습니다: {report}"),
            ));
        }
        let _operation = lock(&self.operation);
        let (sequence, current) = self.presentation.lock_current();
        let initial = current
            .as_ref()
            .map(|published| Arc::clone(&published.snapshot))
            .ok_or((ERR_LAYOUT, "C13.3 render snapshot이 없습니다".to_owned()))?;
        drop(current);
        if self.presentation_sequence() != sequence {
            return Err((ERR_STALE, "C13.3 scene이 평가 중 무효화됐습니다".to_owned()));
        }
        let layout = self.session.layout_snapshot();
        let completed = layout
            .completed
            .filter(|completed| {
                completed.render_snapshot.is_some()
                    && scene_matches_layout_key(initial.as_ref(), completed.key)
            })
            .ok_or((ERR_STALE, "C13.3 layout과 render key가 다릅니다".to_owned()))?;
        let frames = &completed.frames;
        if frames.len() != 11 || initial.scroll_containers().len() != 3 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.3 fixed scene 크기가 다릅니다: frames={} owners={}",
                    frames.len(),
                    initial.scroll_containers().len()
                ),
            ));
        }
        let owners = C13_3GestureOwners {
            root: node_id_at(frames, 1)?,
            outer: node_id_at(frames, 3)?,
            inner: node_id_at(frames, 5)?,
            inner_content: node_id_at(frames, 6)?,
        };
        let scene = node_id_at(frames, 0)?;
        let root_content = node_id_at(frames, 2)?;
        let outer_content = node_id_at(frames, 4)?;
        let expected = [
            (owners.root, Some(scene), 158.0, 238.0),
            (owners.outer, Some(root_content), 106.0, 186.0),
            (owners.inner, Some(outer_content), 86.0, 106.0),
        ];
        for (owner_id, parent, max_x, max_y) in expected {
            let owner = initial
                .scroll_containers()
                .iter()
                .find(|owner| owner.owner() == owner_id)
                .ok_or((
                    ERR_LAYOUT,
                    format!("C13.3 scroll owner {owner_id}가 없습니다"),
                ))?;
            if owner.parent() != parent
                || owner.max_offset_x_css_px() != max_x
                || owner.max_offset_y_css_px() != max_y
            {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.3 owner geometry 불일치 owner={owner_id} parent={:?} range=({}, {})",
                        owner.parent(),
                        owner.max_offset_x_css_px(),
                        owner.max_offset_y_css_px()
                    ),
                ));
            }
        }
        let (expected_inner_x, expected_inner_y) = state.scenario.inner_overscroll();
        let mut interactive = (*initial).clone();
        for (owner_id, _, _, _) in expected {
            let owner = interactive
                .scroll_containers()
                .iter()
                .find(|owner| owner.owner() == owner_id)
                .ok_or((
                    ERR_LAYOUT,
                    format!("C13.3 입력 owner {owner_id}가 없습니다"),
                ))?;
            if !owner.input_policy().user_scrollable_x()
                || !owner.input_policy().user_scrollable_y()
            {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.3 computed direct-scroll policy 불일치 owner={owner_id} policy={:?}",
                        owner.input_policy()
                    ),
                ));
            }
            let expected_overscroll = if owner_id == owners.inner {
                (expected_inner_x, expected_inner_y)
            } else {
                (
                    RuntimeOverscrollBehavior::Auto,
                    RuntimeOverscrollBehavior::Auto,
                )
            };
            if owner.input_policy().overscroll_x() != expected_overscroll.0
                || owner.input_policy().overscroll_y() != expected_overscroll.1
            {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.3 computed overscroll 불일치 scenario={} owner={owner_id} expected={:?}/{:?} actual={:?}/{:?}",
                        state.scenario.label(),
                        expected_overscroll.0,
                        expected_overscroll.1,
                        owner.input_policy().overscroll_x(),
                        owner.input_policy().overscroll_y()
                    ),
                ));
            }
        }
        let expected_touch_action = state.scenario.expected_touch_action();
        let actual_touch_action = interactive.touch_action_at_css_point(100.0, 100.0);
        if actual_touch_action != Some(expected_touch_action) {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.3 computed touch-action 불일치 scenario={} expected={expected_touch_action:?} actual={actual_touch_action:?}",
                    state.scenario.label()
                ),
            ));
        }
        if state.scenario.seed_inner_boundary() {
            let seed = interactive
                .route_scroll_delta_at(
                    interactive.key().scroll_state_revision(),
                    100.0,
                    100.0,
                    86.0,
                    106.0,
                )
                .map_err(|error| (ERR_LAYOUT, format!("C13.3 boundary seed 오류: {error}")))?
                .ok_or((
                    ERR_LAYOUT,
                    "C13.3 boundary seed hit-test가 inner scene을 놓쳤습니다".to_owned(),
                ))?;
            if seed.remaining_x_css_px() != 0.0 || seed.remaining_y_css_px() != 0.0 {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.3 boundary seed에 미소비 delta가 있습니다: {},{}",
                        seed.remaining_x_css_px(),
                        seed.remaining_y_css_px()
                    ),
                ));
            }
            interactive = seed.into_snapshot();
        }

        let expected_offsets = if state.scenario.seed_inner_boundary() {
            [(0.0, 0.0), (0.0, 0.0), (86.0, 106.0)]
        } else {
            [(0.0, 0.0); 3]
        };
        for (owner_id, expected_offset) in [
            (owners.root, expected_offsets[0]),
            (owners.outer, expected_offsets[1]),
            (owners.inner, expected_offsets[2]),
        ] {
            let owner = interactive
                .scroll_containers()
                .iter()
                .find(|owner| owner.owner() == owner_id)
                .ok_or((
                    ERR_LAYOUT,
                    format!("C13.3 seed owner {owner_id}가 없습니다"),
                ))?;
            if (owner.offset_x_css_px(), owner.offset_y_css_px()) != expected_offset {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.3 fixture seed offset 불일치 scenario={} owner={owner_id} expected={expected_offset:?} actual=({}, {})",
                        state.scenario.label(),
                        owner.offset_x_css_px(),
                        owner.offset_y_css_px()
                    ),
                ));
            }
        }
        let published_sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        if !self
            .presentation
            .publish(published_sequence, interactive.clone())
        {
            return Err((
                ERR_STALE,
                "C13.3 initial snapshot publish가 stale입니다".to_owned(),
            ));
        }
        *lock(&state.owners) = Some(owners);
        state.publish_snapshot(interactive.clone());
        Ok(format!(
            "c13_3_gesture_fixture=ready scenario={} policy_inner={} fixture_seed={} viewport_css_px=400x240 owners=root:{},outer:{},inner:{} ranges=root:158,238;outer:106,186;inner:86,106 touch_action_effective={} touch_action_inner={} touch_action_inner_content={} overscroll=style_projected presentation_sequence={} scroll_state_revision={} input_queue_capacity=64 terminal_capacity=8 {}",
            state.scenario.label(),
            state.scenario.policy_label(),
            if state.scenario.seed_inner_boundary() {
                "inner-max"
            } else {
                "none"
            },
            owners.root,
            owners.outer,
            owners.inner,
            state.scenario.expected_touch_action_label(),
            state.scenario.inner_touch_action(),
            state.scenario.inner_content_touch_action(),
            published_sequence,
            interactive.key().scroll_state_revision().get(),
            c13_3_offsets(&interactive, owners)
        ))
    }

    pub(in crate::runtime_gpu) fn process_c13_3_scroll_gesture_queue(
        &self,
        max_events: usize,
    ) -> Result<String, (i32, String)> {
        let _operation = lock(&self.operation);
        let state = self.c13_3_gesture_state.as_ref().ok_or((
            ERR_ARGUMENT,
            "C13.3 gesture fixture host가 아닙니다".to_owned(),
        ))?;
        let mut traces = Vec::new();
        let mut processed = 0usize;
        for _ in 0..max_events {
            let event = lock(&state.queue)
                .pop_next()
                .map_err(|error| (ERR_LAYOUT, format!("C13.3 queue dequeue 실패: {error}")))?;
            let Some(event) = event else { break };
            processed += 1;
            let snapshot = state
                .current_snapshot()
                .map_err(|error| (ERR_LAYOUT, error))?;
            let sequence = event.sequence();
            match event {
                RuntimeScrollInputQueueEvent::GestureBegin(input) => {
                    let identity = input.into();
                    let outcome = lock(&state.session).begin(&snapshot, input);
                    let disposition = match &outcome {
                        Ok(RuntimeScrollGestureOutcome::Began { .. }) => {
                            RuntimeScrollGestureBeginDisposition::Tracking
                        }
                        Ok(_) => RuntimeScrollGestureBeginDisposition::Suppressed,
                        Err(_) => RuntimeScrollGestureBeginDisposition::Rejected,
                    };
                    lock(&state.queue)
                        .complete_begin(identity, disposition)
                        .map_err(|error| {
                            (ERR_LAYOUT, format!("C13.3 Begin gate 확정 실패: {error}"))
                        })?;
                    traces.push(format!(
                        "event=begin sequence={sequence} gesture={} pointer={} result={}",
                        input.gesture_id(),
                        input.pointer_id(),
                        c13_3_outcome_summary(&outcome)
                    ));
                }
                RuntimeScrollInputQueueEvent::PanBegan {
                    sample,
                    policy_epoch,
                } => {
                    let outcome = lock(&state.session).pan_began(&snapshot, sample);
                    let mut published = false;
                    if let Ok(RuntimeScrollGestureOutcome::PanStarted { delta, .. }) = &outcome {
                        published = self.publish_c13_3_delta(state, delta, &snapshot)?;
                    }
                    traces.push(format!(
                        "event=pan_began sequence={sequence} gesture={} pointer={} cumulative_css_px={},{} policy_epoch={} result={} published={published}",
                        sample.gesture_id(), sample.pointer_id(),
                        sample.cumulative_x_css_px(), sample.cumulative_y_css_px(), policy_epoch,
                        c13_3_outcome_summary(&outcome)
                    ));
                }
                RuntimeScrollInputQueueEvent::Move {
                    sample,
                    policy_epoch,
                    coalesced_samples,
                } => {
                    if policy_epoch != sample.expected_key().style_revision().get() {
                        traces.push(format!(
                            "event=move sequence={sequence} gesture={} pointer={} rejected=sample-policy-epoch-mismatch event_epoch={policy_epoch} sample_epoch={}",
                            sample.gesture_id(),
                            sample.pointer_id(),
                            sample.expected_key().style_revision().get()
                        ));
                        continue;
                    }
                    if policy_epoch != snapshot.key().style_revision().get() {
                        traces.push(format!(
                            "event=move sequence={sequence} gesture={} pointer={} discarded=stale-policy-epoch sample_epoch={policy_epoch} current_epoch={}",
                            sample.gesture_id(),
                            sample.pointer_id(),
                            snapshot.key().style_revision().get()
                        ));
                        continue;
                    }
                    let rebased = RuntimeScrollGesturePanSample::new(
                        snapshot.key(),
                        sample.surface_generation(),
                        sample.sequence(),
                        sample.gesture_id(),
                        sample.pointer_id(),
                        sample.coordinate_scale(),
                        sample.cumulative_x_css_px(),
                        sample.cumulative_y_css_px(),
                    );
                    let outcome = lock(&state.session).pan_moved(&snapshot, rebased);
                    let mut published = false;
                    if let Ok(RuntimeScrollGestureOutcome::Moved { delta: Some(delta) }) = &outcome
                    {
                        published = self.publish_c13_3_delta(state, delta, &snapshot)?;
                    }
                    traces.push(format!(
                        "event=move sequence={sequence} gesture={} pointer={} cumulative_css_px={},{} policy_epoch={} coalesced_samples={} result={} published={published}",
                        sample.gesture_id(), sample.pointer_id(),
                        sample.cumulative_x_css_px(), sample.cumulative_y_css_px(),
                        policy_epoch, coalesced_samples, c13_3_outcome_summary(&outcome)
                    ));
                }
                RuntimeScrollInputQueueEvent::GestureTerminal { terminal, kind } => {
                    let outcome = match kind {
                        RuntimeScrollGestureTerminalKind::End => lock(&state.session).end(terminal),
                        RuntimeScrollGestureTerminalKind::Cancel(reason) => {
                            lock(&state.session).cancel(terminal, reason)
                        }
                    };
                    traces.push(format!(
                        "event=terminal sequence={sequence} gesture={} pointer={} kind={kind:?} result={}",
                        terminal.gesture_id(), terminal.pointer_id(),
                        c13_3_outcome_summary(&outcome)
                    ));
                }
                RuntimeScrollInputQueueEvent::WheelDelta { sample, .. } => {
                    traces.push(format!(
                        "event=wheel sequence={sequence} rejected=fixture_adapter_not_enabled target={}",
                        sample.target()
                    ));
                }
            }
        }
        let snapshot = state
            .current_snapshot()
            .map_err(|error| (ERR_LAYOUT, error))?;
        let owners = state.owners().map_err(|error| (ERR_LAYOUT, error))?;
        let queue_len = lock(&state.queue).regular_len();
        Ok(format!(
            "c13_3_input_batch=processed processed_count={processed} pending_regular={queue_len} {} traces=[{}]",
            c13_3_offsets(&snapshot, owners),
            traces.join(";")
        ))
    }

    fn publish_c13_3_delta(
        &self,
        state: &C13_3GestureState,
        delta: &spinon_render::RuntimeScrollDeltaOutcome,
        before: &RuntimeRenderSnapshot,
    ) -> Result<bool, (i32, String)> {
        let updated = delta.snapshot();
        if updated.key().scroll_state_revision() == before.key().scroll_state_revision() {
            return Ok(false);
        }
        let sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        if !self.presentation.publish(sequence, updated.clone()) {
            return Err((
                ERR_STALE,
                "C13.3 gesture snapshot publish가 stale입니다".to_owned(),
            ));
        }
        state.publish_snapshot(updated.clone());
        Ok(true)
    }
}

fn node_id_at(
    frames: &[spinon_runtime::RuntimeLayoutFrame],
    index: usize,
) -> Result<NodeId, (i32, String)> {
    frames
        .get(index)
        .and_then(|frame| NodeId::new(frame.node_id))
        .ok_or((
            ERR_LAYOUT,
            format!("C13.3 fixture frame {index}가 없습니다"),
        ))
}

fn c13_3_offsets(snapshot: &RuntimeRenderSnapshot, owners: C13_3GestureOwners) -> String {
    let offset = |node| {
        snapshot
            .scroll_containers()
            .iter()
            .find(|owner| owner.owner() == node)
            .map(|owner| format!("{},{}", owner.offset_x_css_px(), owner.offset_y_css_px()))
            .unwrap_or_else(|| "missing".to_owned())
    };
    format!(
        "scroll_state_revision={} offsets=root:{};outer:{};inner:{}",
        snapshot.key().scroll_state_revision().get(),
        offset(owners.root),
        offset(owners.outer),
        offset(owners.inner)
    )
}

fn c13_3_outcome_summary<T: std::fmt::Debug>(
    outcome: &Result<RuntimeScrollGestureOutcome, T>,
) -> String {
    let outcome = match outcome {
        Ok(outcome) => outcome,
        Err(error) => return format!("error={error:?}"),
    };
    match outcome {
        RuntimeScrollGestureOutcome::Began {
            target,
            owner_chain,
            allowed_axes,
        } => format!(
            "began target={} owner_chain={owner_chain:?} allowed_axes={allowed_axes:?}",
            target.get()
        ),
        RuntimeScrollGestureOutcome::PanStarted { axes, delta } => {
            format!("pan_started axes={axes:?} {}", c13_3_delta_summary(delta))
        }
        RuntimeScrollGestureOutcome::PanRejected => "pan_rejected".to_owned(),
        RuntimeScrollGestureOutcome::Moved { delta: Some(delta) } => {
            format!("moved {}", c13_3_delta_summary(delta))
        }
        RuntimeScrollGestureOutcome::Moved { delta: None } => "moved no_delta".to_owned(),
        RuntimeScrollGestureOutcome::Cancelled(reason) => format!("cancelled {reason:?}"),
        RuntimeScrollGestureOutcome::Ended => "ended".to_owned(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining,
        } => format!("suppressed_pointer_ended remaining={blocked_contacts_remaining}"),
    }
}

fn c13_3_delta_summary(delta: &spinon_render::RuntimeScrollDeltaOutcome) -> String {
    format!(
        "target={} revision={} changes={} remainder_css_px={:.2},{:.2}",
        delta.target().get(),
        delta.snapshot().key().scroll_state_revision().get(),
        delta.changes().len(),
        delta.remaining_x_css_px(),
        delta.remaining_y_css_px()
    )
}
