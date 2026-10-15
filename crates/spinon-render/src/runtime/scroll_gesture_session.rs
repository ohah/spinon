use std::collections::BTreeMap;

use spinon_core::{
    DocumentRevision, NodeId, RenderTreeRevision, ScrollStateRevision, StyleRevision,
};

use super::RuntimeRenderSnapshot;

mod errors;
mod geometry;
mod types;

pub use errors::RuntimeScrollGestureError;
use geometry::RuntimeScrollGestureGeometryKey;
pub use types::{
    RuntimeGestureAxes, RuntimeScrollGestureBegin, RuntimeScrollGestureCancelReason,
    RuntimeScrollGestureOutcome, RuntimeScrollGesturePanSample, RuntimeScrollGestureState,
    RuntimeScrollGestureTerminal, RuntimeTouchAction,
};

/// 입력 순서, pointer 수명과 frozen gesture policy를 소유하는 단일-thread session입니다.
///
/// UI/native queue 동기화는 포함하지 않습니다. `Begin`의 resolved touch-action은 hit target에서
/// nearest scroll owner까지 style cascade가 계산한 제한 profile 결과여야 합니다.
#[derive(Default)]
pub struct RuntimeScrollGestureSession {
    active: Option<ActiveGesture>,
    suppressed_pointers: BTreeMap<u64, SuppressedPointer>,
    last_input_sequence: Option<u64>,
    last_gesture_id: Option<u64>,
}

#[derive(Clone, Copy)]
struct SuppressedPointer {
    gesture_id: u64,
    surface_generation: u64,
}

struct ActiveGesture {
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
    coordinate_scale: f32,
    target: NodeId,
    geometry: RuntimeScrollGestureGeometryKey,
    expected_document_revision: DocumentRevision,
    expected_render_tree_revision: RenderTreeRevision,
    expected_style_revision: StyleRevision,
    allowed_axes: RuntimeGestureAxes,
    phase: ActivePhase,
    last_cumulative_x_css_px: f32,
    last_cumulative_y_css_px: f32,
    expected_scroll_revision: ScrollStateRevision,
}

#[derive(Clone, Copy)]
enum ActivePhase {
    Tracking,
    Panning(RuntimeGestureAxes),
    PanRejected,
}

impl RuntimeScrollGestureSession {
    /// active pan, rejected pan 또는 terminal 대기 상태를 반환합니다.
    pub fn state(&self) -> RuntimeScrollGestureState {
        if !self.suppressed_pointers.is_empty() {
            return RuntimeScrollGestureState::Suppressed;
        }
        match self.active.as_ref().map(|active| active.phase) {
            Some(ActivePhase::Tracking) => RuntimeScrollGestureState::Tracking,
            Some(ActivePhase::Panning(axes)) => RuntimeScrollGestureState::Panning(axes),
            Some(ActivePhase::PanRejected) => RuntimeScrollGestureState::PanRejected,
            None => RuntimeScrollGestureState::Idle,
        }
    }

    /// 현재 snapshot hit test로 pointer gesture를 시작하고 target/owner chain을 고정합니다.
    pub fn begin(
        &mut self,
        snapshot: &RuntimeRenderSnapshot,
        input: RuntimeScrollGestureBegin,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.validate_sequence(input.sequence)?;
        if let Some((active_pointer, active_gesture, active_surface_generation)) =
            self.active.as_ref().map(|active| {
                (
                    active.pointer_id,
                    active.gesture_id,
                    active.surface_generation,
                )
            })
        {
            if active_pointer == input.pointer_id {
                return Err(RuntimeScrollGestureError::GestureAlreadyActive);
            }
            self.active = None;
            self.suppressed_pointers.insert(
                active_pointer,
                SuppressedPointer {
                    gesture_id: active_gesture,
                    surface_generation: active_surface_generation,
                },
            );
            self.suppressed_pointers.insert(
                input.pointer_id,
                SuppressedPointer {
                    gesture_id: input.gesture_id,
                    surface_generation: input.surface_generation,
                },
            );
            self.last_input_sequence = Some(input.sequence);
            return Ok(RuntimeScrollGestureOutcome::Cancelled(
                RuntimeScrollGestureCancelReason::SecondPointer,
            ));
        }
        if !self.suppressed_pointers.is_empty() {
            if self.suppressed_pointers.contains_key(&input.pointer_id) {
                return Err(RuntimeScrollGestureError::DuplicatePointer);
            }
            self.suppressed_pointers.insert(
                input.pointer_id,
                SuppressedPointer {
                    gesture_id: input.gesture_id,
                    surface_generation: input.surface_generation,
                },
            );
            self.last_input_sequence = Some(input.sequence);
            return Ok(RuntimeScrollGestureOutcome::Cancelled(
                RuntimeScrollGestureCancelReason::SecondPointer,
            ));
        }
        if input.gesture_id == 0
            || self
                .last_gesture_id
                .is_some_and(|last| input.gesture_id <= last)
        {
            return Err(RuntimeScrollGestureError::InvalidGestureId);
        }
        if snapshot.key() != input.expected_key {
            return Err(RuntimeScrollGestureError::StaleExpectedSnapshot);
        }
        if !input.point_x_css_px.is_finite() || !input.point_y_css_px.is_finite() {
            return Err(RuntimeScrollGestureError::InvalidPoint);
        }
        if !input.coordinate_scale.is_finite() || input.coordinate_scale <= 0.0 {
            return Err(RuntimeScrollGestureError::InvalidCoordinateScale);
        }
        let allowed_axes = axes_for_touch_action(input.resolved_touch_action)?;
        let Some(target) = snapshot
            .hit_test_css_point(input.point_x_css_px, input.point_y_css_px)
            .map(|render_box| render_box.node_id())
        else {
            return Err(RuntimeScrollGestureError::HitTestMiss);
        };
        let Some(geometry) = snapshot.gesture_geometry_key(target, input.surface_generation) else {
            return Err(RuntimeScrollGestureError::InvalidGeometry);
        };
        let owner_chain = geometry.owner_chain.clone();
        self.active = Some(ActiveGesture {
            gesture_id: input.gesture_id,
            pointer_id: input.pointer_id,
            surface_generation: input.surface_generation,
            coordinate_scale: input.coordinate_scale,
            target,
            geometry,
            expected_document_revision: snapshot.key().document_revision(),
            expected_render_tree_revision: snapshot.key().render_tree_revision(),
            expected_style_revision: snapshot.key().style_revision(),
            allowed_axes,
            phase: ActivePhase::Tracking,
            last_cumulative_x_css_px: 0.0,
            last_cumulative_y_css_px: 0.0,
            expected_scroll_revision: snapshot.key().scroll_state_revision(),
        });
        self.last_input_sequence = Some(input.sequence);
        self.last_gesture_id = Some(input.gesture_id);
        Ok(RuntimeScrollGestureOutcome::Began {
            target,
            owner_chain,
            allowed_axes,
        })
    }

    /// Platform recognizer가 보고한 첫 cumulative translation을 적용합니다.
    ///
    /// Native recognizer의 시작 sample이 0 translation이면 축을 잠그지 않고,
    /// adapter가 전달하는 다음 nonzero `pan_began` sample에서 허용 축을 판정합니다.
    pub fn pan_began(
        &mut self,
        snapshot: &RuntimeRenderSnapshot,
        sample: RuntimeScrollGesturePanSample,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.validate_sequence(sample.sequence)?;
        if let Some(reason) = self.validate_sample(snapshot, sample)? {
            return Ok(RuntimeScrollGestureOutcome::Cancelled(reason));
        }
        let active = self
            .active
            .as_ref()
            .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
        if !matches!(active.phase, ActivePhase::Tracking) {
            return Err(RuntimeScrollGestureError::PanAlreadyStarted);
        }
        validate_pan_translation(sample.cumulative_x_css_px, sample.cumulative_y_css_px)?;
        if is_zero_translation(sample.cumulative_x_css_px, sample.cumulative_y_css_px) {
            self.last_input_sequence = Some(sample.sequence);
            return Ok(RuntimeScrollGestureOutcome::Moved { delta: None });
        }
        self.start_pan(snapshot, sample)
    }

    fn start_pan(
        &mut self,
        snapshot: &RuntimeRenderSnapshot,
        sample: RuntimeScrollGesturePanSample,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        let active = self
            .active
            .as_ref()
            .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
        if !matches!(active.phase, ActivePhase::Tracking) {
            return Err(RuntimeScrollGestureError::PanAlreadyStarted);
        }
        let dominant_axes = dominant_axes(sample.cumulative_x_css_px, sample.cumulative_y_css_px);
        let axes = active.allowed_axes.intersection(dominant_axes);
        if axes.is_empty() {
            if let Some(active) = self.active.as_mut() {
                active.phase = ActivePhase::PanRejected;
            }
            self.last_input_sequence = Some(sample.sequence);
            return Ok(RuntimeScrollGestureOutcome::PanRejected);
        }
        let delta_x = if axes.x() {
            -sample.cumulative_x_css_px
        } else {
            0.0
        };
        let delta_y = if axes.y() {
            -sample.cumulative_y_css_px
        } else {
            0.0
        };
        let result = snapshot.route_scroll_delta_for_target(
            active.expected_scroll_revision,
            active.target,
            delta_x,
            delta_y,
        );
        let outcome = match result {
            Ok(outcome) => outcome,
            Err(error) => {
                self.cancel_active_for_render_failure(sample.sequence);
                return Err(RuntimeScrollGestureError::Render(error));
            }
        };
        let active = self
            .active
            .as_mut()
            .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
        active.phase = ActivePhase::Panning(axes);
        active.last_cumulative_x_css_px = sample.cumulative_x_css_px;
        active.last_cumulative_y_css_px = sample.cumulative_y_css_px;
        active.expected_scroll_revision = outcome.snapshot().key().scroll_state_revision();
        self.last_input_sequence = Some(sample.sequence);
        Ok(RuntimeScrollGestureOutcome::PanStarted {
            axes,
            delta: outcome,
        })
    }

    /// 직전 accepted cumulative translation과의 차이만 scroll에 적용합니다.
    pub fn pan_moved(
        &mut self,
        snapshot: &RuntimeRenderSnapshot,
        sample: RuntimeScrollGesturePanSample,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.validate_sequence(sample.sequence)?;
        if let Some(reason) = self.validate_sample(snapshot, sample)? {
            return Ok(RuntimeScrollGestureOutcome::Cancelled(reason));
        }
        validate_pan_translation(sample.cumulative_x_css_px, sample.cumulative_y_css_px)?;
        let active = self
            .active
            .as_ref()
            .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
        match active.phase {
            ActivePhase::Tracking => {
                self.last_input_sequence = Some(sample.sequence);
                Ok(RuntimeScrollGestureOutcome::Moved { delta: None })
            }
            ActivePhase::PanRejected => {
                self.last_input_sequence = Some(sample.sequence);
                Ok(RuntimeScrollGestureOutcome::PanRejected)
            }
            ActivePhase::Panning(axes) => {
                let delta_x = if axes.x() {
                    -(sample.cumulative_x_css_px - active.last_cumulative_x_css_px)
                } else {
                    0.0
                };
                let delta_y = if axes.y() {
                    -(sample.cumulative_y_css_px - active.last_cumulative_y_css_px)
                } else {
                    0.0
                };
                if !delta_x.is_finite() || !delta_y.is_finite() {
                    return Err(RuntimeScrollGestureError::InvalidPanTranslation);
                }
                let result = snapshot.route_scroll_delta_for_target(
                    active.expected_scroll_revision,
                    active.target,
                    delta_x,
                    delta_y,
                );
                let outcome = match result {
                    Ok(outcome) => outcome,
                    Err(error) => {
                        self.cancel_active_for_render_failure(sample.sequence);
                        return Err(RuntimeScrollGestureError::Render(error));
                    }
                };
                let active = self
                    .active
                    .as_mut()
                    .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
                active.last_cumulative_x_css_px = sample.cumulative_x_css_px;
                active.last_cumulative_y_css_px = sample.cumulative_y_css_px;
                active.expected_scroll_revision = outcome.snapshot().key().scroll_state_revision();
                self.last_input_sequence = Some(sample.sequence);
                Ok(RuntimeScrollGestureOutcome::Moved {
                    delta: Some(outcome),
                })
            }
        }
    }

    /// sequence와 identity가 맞는 active pointer를 정상 종료합니다.
    pub fn end(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.finish(terminal, None)
    }

    /// terminal을 소비하고 이미 게시된 offset을 유지한 채 active pointer를 취소합니다.
    pub fn cancel(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
        reason: RuntimeScrollGestureCancelReason,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.finish(terminal, Some(reason))
    }

    fn finish(
        &mut self,
        terminal: RuntimeScrollGestureTerminal,
        reason: Option<RuntimeScrollGestureCancelReason>,
    ) -> Result<RuntimeScrollGestureOutcome, RuntimeScrollGestureError> {
        self.validate_sequence(terminal.sequence)?;
        if let Some(active) = self.active.as_ref() {
            if active.gesture_id != terminal.gesture_id || active.pointer_id != terminal.pointer_id
            {
                return Err(RuntimeScrollGestureError::GestureIdentityMismatch);
            }
            if active.surface_generation != terminal.surface_generation {
                self.active = None;
                self.last_input_sequence = Some(terminal.sequence);
                return Ok(RuntimeScrollGestureOutcome::Cancelled(
                    RuntimeScrollGestureCancelReason::SurfaceGenerationChanged,
                ));
            }
            self.active = None;
            self.last_input_sequence = Some(terminal.sequence);
            return Ok(reason.map_or(
                RuntimeScrollGestureOutcome::Ended,
                RuntimeScrollGestureOutcome::Cancelled,
            ));
        }
        let Some(suppressed) = self.suppressed_pointers.get(&terminal.pointer_id) else {
            return Err(RuntimeScrollGestureError::UnknownPointer);
        };
        if suppressed.gesture_id != terminal.gesture_id
            || suppressed.surface_generation != terminal.surface_generation
        {
            return Err(RuntimeScrollGestureError::GestureIdentityMismatch);
        }
        self.suppressed_pointers.remove(&terminal.pointer_id);
        self.last_input_sequence = Some(terminal.sequence);
        Ok(RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: self.suppressed_pointers.len(),
        })
    }

    fn validate_sample(
        &mut self,
        snapshot: &RuntimeRenderSnapshot,
        sample: RuntimeScrollGesturePanSample,
    ) -> Result<Option<RuntimeScrollGestureCancelReason>, RuntimeScrollGestureError> {
        let active = self
            .active
            .as_ref()
            .ok_or(RuntimeScrollGestureError::NoActiveGesture)?;
        if active.gesture_id != sample.gesture_id || active.pointer_id != sample.pointer_id {
            return Err(RuntimeScrollGestureError::GestureIdentityMismatch);
        }
        if active.surface_generation != sample.surface_generation
            || active.coordinate_scale.to_bits() != sample.coordinate_scale.to_bits()
        {
            self.cancel_active(sample.sequence);
            return Ok(Some(
                RuntimeScrollGestureCancelReason::SurfaceGenerationChanged,
            ));
        }
        if snapshot.key() != sample.expected_key {
            return Err(RuntimeScrollGestureError::StaleExpectedSnapshot);
        }
        let target = active.target;
        let expected_generation = active.geometry.generation;
        let expected_environment_revision = active.geometry.environment_revision;
        let expected_document_revision = active.expected_document_revision;
        let expected_tree_revision = active.expected_render_tree_revision;
        let expected_style_revision = active.expected_style_revision;
        let expected_scroll_revision = active.expected_scroll_revision;
        let expected_geometry = active.geometry.clone();
        let key = snapshot.key();
        if key.generation() != expected_generation
            || key.document_revision() < expected_document_revision
            || key.render_tree_revision() < expected_tree_revision
            || key.style_revision() < expected_style_revision
        {
            self.cancel_active(sample.sequence);
            return Ok(Some(
                RuntimeScrollGestureCancelReason::DocumentOrTreeChanged,
            ));
        }
        if key.environment_revision() != expected_environment_revision {
            self.cancel_active(sample.sequence);
            return Ok(Some(RuntimeScrollGestureCancelReason::EnvironmentChanged));
        }
        let Some(current_geometry) =
            snapshot.gesture_geometry_key(target, sample.surface_generation)
        else {
            self.cancel_active(sample.sequence);
            return Ok(Some(RuntimeScrollGestureCancelReason::GeometryChanged));
        };
        if current_geometry != expected_geometry {
            self.cancel_active(sample.sequence);
            return Ok(Some(RuntimeScrollGestureCancelReason::GeometryChanged));
        }
        let document_projection_changed = key.document_revision() != expected_document_revision
            || key.render_tree_revision() != expected_tree_revision;
        if document_projection_changed && key.style_revision() == expected_style_revision {
            self.cancel_active(sample.sequence);
            return Ok(Some(
                RuntimeScrollGestureCancelReason::DocumentOrTreeChanged,
            ));
        }
        if key.scroll_state_revision() != expected_scroll_revision {
            self.cancel_active(sample.sequence);
            return Ok(Some(RuntimeScrollGestureCancelReason::ExternalScrollUpdate));
        }
        if let Some(active) = self.active.as_mut() {
            active.expected_document_revision = key.document_revision();
            active.expected_render_tree_revision = key.render_tree_revision();
            active.expected_style_revision = key.style_revision();
        }
        Ok(None)
    }

    fn cancel_active(&mut self, sequence: u64) {
        if let Some(active) = self.active.take() {
            self.suppressed_pointers.insert(
                active.pointer_id,
                SuppressedPointer {
                    gesture_id: active.gesture_id,
                    surface_generation: active.surface_generation,
                },
            );
        }
        self.last_input_sequence = Some(sequence);
    }

    fn cancel_active_for_render_failure(&mut self, sequence: u64) {
        self.cancel_active(sequence);
    }

    fn validate_sequence(&self, sequence: u64) -> Result<(), RuntimeScrollGestureError> {
        if self
            .last_input_sequence
            .is_some_and(|last| sequence <= last)
        {
            return Err(RuntimeScrollGestureError::NonMonotonicInputSequence);
        }
        Ok(())
    }
}

fn axes_for_touch_action(
    action: RuntimeTouchAction,
) -> Result<RuntimeGestureAxes, RuntimeScrollGestureError> {
    match action {
        RuntimeTouchAction::Auto => Ok(RuntimeGestureAxes { x: true, y: true }),
        RuntimeTouchAction::None => Ok(RuntimeGestureAxes { x: false, y: false }),
        RuntimeTouchAction::PanX => Ok(RuntimeGestureAxes { x: true, y: false }),
        RuntimeTouchAction::PanY => Ok(RuntimeGestureAxes { x: false, y: true }),
        RuntimeTouchAction::Unsupported => Err(RuntimeScrollGestureError::UnsupportedTouchAction),
    }
}

fn dominant_axes(x: f32, y: f32) -> RuntimeGestureAxes {
    if x.abs() > y.abs() {
        RuntimeGestureAxes { x: true, y: false }
    } else if y.abs() > x.abs() {
        RuntimeGestureAxes { x: false, y: true }
    } else {
        RuntimeGestureAxes { x: true, y: true }
    }
}

fn is_zero_translation(x: f32, y: f32) -> bool {
    x == 0.0 && y == 0.0
}

fn validate_pan_translation(x: f32, y: f32) -> Result<(), RuntimeScrollGestureError> {
    if !x.is_finite() || !y.is_finite() {
        return Err(RuntimeScrollGestureError::InvalidPanTranslation);
    }
    Ok(())
}
