use super::super::super::super::lock;
use super::C13_3GestureState;
use spinon_core::ScrollStateRevision;
use spinon_render::{
    RuntimeRenderKey, RuntimeScrollInputQueueAdmission, RuntimeScrollMomentumPhase,
    RuntimeScrollWheelSample, RuntimeScrollWheelSource, RuntimeScrollWheelUnit,
};

pub(in crate::runtime_gpu) struct C13_3WheelInput {
    pub(in crate::runtime_gpu) scroll_intent: bool,
    pub(in crate::runtime_gpu) surface_generation: u64,
    pub(in crate::runtime_gpu) point_x_css_px: f32,
    pub(in crate::runtime_gpu) point_y_css_px: f32,
    pub(in crate::runtime_gpu) delta_x_css_px: f32,
    pub(in crate::runtime_gpu) delta_y_css_px: f32,
    pub(in crate::runtime_gpu) raw_delta_x: f32,
    pub(in crate::runtime_gpu) raw_delta_y: f32,
    pub(in crate::runtime_gpu) source_unit: RuntimeScrollWheelUnit,
    pub(in crate::runtime_gpu) coordinate_scale: f32,
    pub(in crate::runtime_gpu) source: RuntimeScrollWheelSource,
    pub(in crate::runtime_gpu) momentum_phase: RuntimeScrollMomentumPhase,
    pub(in crate::runtime_gpu) modifier_flags: u32,
}

#[derive(Default)]
pub(super) struct C13_3WheelState {
    latest_surface_generation: Option<u64>,
    lineage: Option<ScrollLineage>,
}

struct ScrollLineage {
    base_key: RuntimeRenderKey,
    latest_revision: ScrollStateRevision,
    latest_sequence: u64,
    surface_generation: u64,
}

impl C13_3GestureState {
    pub(super) fn note_surface_generation(&self, generation: u64) {
        let _surface_update = self.lock_surface_update();
        let mut wheel = lock(&self.wheel);
        if wheel
            .latest_surface_generation
            .is_none_or(|current| generation > current)
        {
            wheel.latest_surface_generation = Some(generation);
            wheel.lineage = None;
        }
    }

    pub(in crate::runtime_gpu) fn enqueue_wheel(
        &self,
        input: C13_3WheelInput,
    ) -> Result<String, String> {
        let C13_3WheelInput {
            scroll_intent,
            surface_generation,
            point_x_css_px,
            point_y_css_px,
            delta_x_css_px,
            delta_y_css_px,
            raw_delta_x,
            raw_delta_y,
            source_unit,
            coordinate_scale,
            source,
            momentum_phase,
            modifier_flags,
        } = input;
        if !scroll_intent {
            return Err("C13.3 magnification/zoom intent는 scroll queue에서 거부됩니다".to_owned());
        }
        self.note_surface_generation(surface_generation);
        if !self.is_latest_surface_generation(surface_generation) {
            return Err(format!(
                "C13.3 stale wheel surface generation: input={surface_generation} latest={:?}",
                lock(&self.wheel).latest_surface_generation
            ));
        }
        if !point_x_css_px.is_finite() || !point_y_css_px.is_finite() {
            return Err("C13.3 wheel point가 유한하지 않습니다".to_owned());
        }
        let snapshot = self.current_snapshot()?;
        let Some(target) = snapshot
            .hit_test_css_point(point_x_css_px, point_y_css_px)
            .map(|render_box| render_box.node_id())
        else {
            return Err("C13.3 wheel point가 clip-aware hit target을 찾지 못했습니다".to_owned());
        };
        let sequence = self.next_sequence()?;
        let key = snapshot.key();
        let sample = RuntimeScrollWheelSample::new(
            sequence,
            surface_generation,
            key,
            key.scroll_state_revision(),
            point_x_css_px,
            point_y_css_px,
            delta_x_css_px,
            delta_y_css_px,
            raw_delta_x,
            raw_delta_y,
            source_unit,
            coordinate_scale,
            target,
            key.style_revision().get(),
            source,
            momentum_phase,
            modifier_flags,
        );
        let admission = lock(&self.queue)
            .enqueue_wheel(sample)
            .map_err(|error| format!("C13.3 WheelDelta 거부: {error}"))?;
        let admitted = matches!(
            admission,
            RuntimeScrollInputQueueAdmission::Queued
                | RuntimeScrollInputQueueAdmission::Coalesced { .. }
        );
        Ok(format!(
            "sequence={sequence} admitted={admitted} admission={admission:?} target={} point_css_px={point_x_css_px},{point_y_css_px} delta_css_px={delta_x_css_px},{delta_y_css_px} raw_delta={raw_delta_x},{raw_delta_y} unit={source_unit:?} source={source:?} scale={coordinate_scale} phase={momentum_phase:?} modifiers={modifier_flags} key={key:?} basis_revision={}",
            target.get(),
            key.scroll_state_revision().get()
        ))
    }

    pub(in crate::runtime_gpu::fixtures::c13_3_scroll_gesture_host) fn wheel_sample_matches_lineage(
        &self,
        sample: RuntimeScrollWheelSample,
        current_key: RuntimeRenderKey,
    ) -> bool {
        let wheel = lock(&self.wheel);
        if wheel.latest_surface_generation != Some(sample.surface_generation())
            || !same_non_scroll_key(sample.expected_key(), current_key)
        {
            return false;
        }
        if sample.expected_key() == current_key {
            return true;
        }
        let Some(lineage) = wheel.lineage.as_ref() else {
            return false;
        };
        let basis = sample.basis_scroll_revision().get();
        same_non_scroll_key(lineage.base_key, sample.expected_key())
            && lineage.surface_generation == sample.surface_generation()
            && lineage.latest_revision == current_key.scroll_state_revision()
            && basis >= lineage.base_key.scroll_state_revision().get()
            && basis <= lineage.latest_revision.get()
            && sample.sequence() > lineage.latest_sequence
    }

    pub(in crate::runtime_gpu::fixtures::c13_3_scroll_gesture_host) fn publish_input_snapshot(
        &self,
        before_key: RuntimeRenderKey,
        snapshot: spinon_render::RuntimeRenderSnapshot,
        sequence: u64,
        surface_generation: u64,
    ) {
        let updated_key = snapshot.key();
        let mut wheel = lock(&self.wheel);
        if wheel.latest_surface_generation == Some(surface_generation)
            && same_non_scroll_key(before_key, updated_key)
            && updated_key.scroll_state_revision() != before_key.scroll_state_revision()
        {
            let lineage = match wheel.lineage.take() {
                Some(mut lineage)
                    if lineage.latest_revision == before_key.scroll_state_revision()
                        && lineage.surface_generation == surface_generation
                        && same_non_scroll_key(lineage.base_key, before_key) =>
                {
                    lineage.latest_revision = updated_key.scroll_state_revision();
                    lineage.latest_sequence = sequence;
                    lineage
                }
                _ => ScrollLineage {
                    base_key: before_key,
                    latest_revision: updated_key.scroll_state_revision(),
                    latest_sequence: sequence,
                    surface_generation,
                },
            };
            wheel.lineage = Some(lineage);
        } else {
            wheel.lineage = None;
        }
        *lock(&self.snapshot) = Some(snapshot);
    }

    pub(in crate::runtime_gpu::fixtures::c13_3_scroll_gesture_host) fn is_latest_surface_generation(
        &self,
        generation: u64,
    ) -> bool {
        lock(&self.wheel).latest_surface_generation == Some(generation)
    }

    pub(super) fn reset_wheel_lineage(&self) {
        lock(&self.wheel).lineage = None;
    }
}

fn same_non_scroll_key(left: RuntimeRenderKey, right: RuntimeRenderKey) -> bool {
    left.generation() == right.generation()
        && left.document_revision() == right.document_revision()
        && left.render_tree_revision() == right.render_tree_revision()
        && left.style_revision() == right.style_revision()
        && left.environment_revision() == right.environment_revision()
        && left.intrinsic_size_revision() == right.intrinsic_size_revision()
}

#[cfg(test)]
mod tests {
    use super::same_non_scroll_key;
    use spinon_core::{
        EnvironmentRevision, HostDocument, HostDocumentSnapshot, ScrollStateRevision, StyleRevision,
    };
    use spinon_render::RuntimeRenderKey;

    fn key(
        document: &HostDocumentSnapshot,
        scroll_revision: ScrollStateRevision,
    ) -> RuntimeRenderKey {
        RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
        .with_scroll_state_revision(scroll_revision)
    }

    #[test]
    fn scroll_revision_is_the_only_key_component_allowed_by_queue_lineage() {
        let document = HostDocument::new().unwrap().snapshot();
        assert!(same_non_scroll_key(
            key(&document, ScrollStateRevision::INITIAL),
            key(
                &document,
                ScrollStateRevision::INITIAL.checked_next().unwrap()
            )
        ));
        let initial = key(&document, ScrollStateRevision::INITIAL);
        let changed_style = RuntimeRenderKey::new(
            initial.generation(),
            initial.document_revision(),
            initial.render_tree_revision(),
            StyleRevision::INITIAL.checked_next().unwrap(),
            EnvironmentRevision::INITIAL,
        );
        assert!(!same_non_scroll_key(initial, changed_style));
        let changed_environment = RuntimeRenderKey::new(
            initial.generation(),
            initial.document_revision(),
            initial.render_tree_revision(),
            initial.style_revision(),
            EnvironmentRevision::INITIAL.checked_next().unwrap(),
        );
        assert!(!same_non_scroll_key(initial, changed_environment));
    }
}
