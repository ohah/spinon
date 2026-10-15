use super::state::C13_3GestureState;
use super::wheel;
use super::{
    ERR_ARGUMENT, ERR_LAYOUT, ERR_STALE, RuntimeGpuHost, c13_3_offsets, c13_3_outcome_summary, lock,
};
use spinon_render::{
    RuntimeRenderSnapshot, RuntimeScrollGestureBeginDisposition, RuntimeScrollGestureOutcome,
    RuntimeScrollGesturePanSample, RuntimeScrollGestureTerminalKind, RuntimeScrollInputQueueEvent,
};

impl RuntimeGpuHost {
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
                        published = self.publish_c13_3_delta(
                            state,
                            delta,
                            &snapshot,
                            sample.sequence(),
                            sample.surface_generation(),
                        )?;
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
                        published = self.publish_c13_3_delta(
                            state,
                            delta,
                            &snapshot,
                            sample.sequence(),
                            sample.surface_generation(),
                        )?;
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
                RuntimeScrollInputQueueEvent::WheelDelta {
                    sample,
                    coalesced_samples,
                } => {
                    traces.push(wheel::process_c13_3_wheel(
                        self,
                        state,
                        &snapshot,
                        sequence,
                        sample,
                        coalesced_samples,
                    )?);
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

    pub(super) fn publish_c13_3_delta(
        &self,
        state: &C13_3GestureState,
        delta: &spinon_render::RuntimeScrollDeltaOutcome,
        before: &RuntimeRenderSnapshot,
        input_sequence: u64,
        surface_generation: u64,
    ) -> Result<bool, (i32, String)> {
        let updated = delta.snapshot();
        if updated.key().scroll_state_revision() == before.key().scroll_state_revision() {
            return Ok(false);
        }
        let _surface_update = state.lock_surface_update();
        if !state.is_latest_surface_generation(surface_generation) {
            return Err((
                ERR_STALE,
                format!(
                    "C13.3 snapshot publish가 오래된 surface generation입니다: input={surface_generation}"
                ),
            ));
        }
        let presentation_sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        if !self
            .presentation
            .publish(presentation_sequence, updated.clone())
        {
            return Err((
                ERR_STALE,
                "C13.3 gesture snapshot publish가 stale입니다".to_owned(),
            ));
        }
        state.publish_input_snapshot(
            before.key(),
            updated.clone(),
            input_sequence,
            surface_generation,
        );
        Ok(true)
    }
}
