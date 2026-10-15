use spinon_core::{EnvironmentRevision, HostDocument, NodeId, ScrollStateRevision, StyleRevision};

use super::{
    RuntimeRenderKey, RuntimeScrollGestureBegin, RuntimeScrollGestureBeginDisposition,
    RuntimeScrollGestureCancelReason, RuntimeScrollGestureIdentity, RuntimeScrollGestureInputQueue,
    RuntimeScrollGesturePanSample, RuntimeScrollGestureTerminal, RuntimeScrollGestureTerminalKind,
    RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueError, RuntimeScrollInputQueueEvent,
    RuntimeScrollMomentumPhase, RuntimeScrollWheelSample, RuntimeScrollWheelSource,
    RuntimeScrollWheelUnit, RuntimeTouchAction,
};

#[derive(Clone, Copy)]
struct WheelSpec {
    point: (f32, f32),
    css_delta: (f32, f32),
    raw_delta: (f32, f32),
    unit: RuntimeScrollWheelUnit,
    scale: f32,
    target: NodeId,
    policy_epoch: u64,
    source: RuntimeScrollWheelSource,
    phase: RuntimeScrollMomentumPhase,
    modifiers: u32,
}

fn key() -> RuntimeRenderKey {
    let document = HostDocument::new().expect("test document").snapshot();
    RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
}

fn key_with_style(key: RuntimeRenderKey, style: StyleRevision) -> RuntimeRenderKey {
    RuntimeRenderKey::new(
        key.generation(),
        key.document_revision(),
        key.render_tree_revision(),
        style,
        key.environment_revision(),
    )
    .with_intrinsic_size_revision(key.intrinsic_size_revision())
    .with_scroll_state_revision(key.scroll_state_revision())
}

fn begin_input(
    key: RuntimeRenderKey,
    sequence: u64,
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
) -> RuntimeScrollGestureBegin {
    RuntimeScrollGestureBegin::new(
        key,
        surface_generation,
        sequence,
        gesture_id,
        pointer_id,
        10.0,
        20.0,
        1.0,
        RuntimeTouchAction::Auto,
    )
}

fn pan_sample(
    key: RuntimeRenderKey,
    sequence: u64,
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
    cumulative_x: f32,
    cumulative_y: f32,
) -> RuntimeScrollGesturePanSample {
    RuntimeScrollGesturePanSample::new(
        key,
        surface_generation,
        sequence,
        gesture_id,
        pointer_id,
        1.0,
        cumulative_x,
        cumulative_y,
    )
}

fn identity(
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
) -> RuntimeScrollGestureIdentity {
    RuntimeScrollGestureIdentity::new(gesture_id, pointer_id, surface_generation)
}

fn wheel_sample(
    key: RuntimeRenderKey,
    sequence: u64,
    point_x: f32,
    point_y: f32,
    delta_x: f32,
    delta_y: f32,
    policy_epoch: u64,
) -> RuntimeScrollWheelSample {
    wheel_with_spec(
        key,
        sequence,
        WheelSpec {
            point: (point_x, point_y),
            css_delta: (delta_x, delta_y),
            raw_delta: (delta_x, delta_y),
            unit: RuntimeScrollWheelUnit::CssPixels,
            scale: 1.0,
            target: NodeId::new(99).expect("nonzero target id"),
            policy_epoch,
            source: RuntimeScrollWheelSource::TrackpadScroll,
            phase: RuntimeScrollMomentumPhase::Change,
            modifiers: 0,
        },
    )
}

fn wheel_with_spec(
    key: RuntimeRenderKey,
    sequence: u64,
    spec: WheelSpec,
) -> RuntimeScrollWheelSample {
    wheel_with_basis(key, key.scroll_state_revision(), sequence, spec)
}

fn wheel_with_basis(
    key: RuntimeRenderKey,
    basis_scroll_revision: ScrollStateRevision,
    sequence: u64,
    spec: WheelSpec,
) -> RuntimeScrollWheelSample {
    RuntimeScrollWheelSample::new(
        sequence,
        7,
        key,
        basis_scroll_revision,
        spec.point.0,
        spec.point.1,
        spec.css_delta.0,
        spec.css_delta.1,
        spec.raw_delta.0,
        spec.raw_delta.1,
        spec.unit,
        spec.scale,
        spec.target,
        spec.policy_epoch,
        spec.source,
        spec.phase,
        spec.modifiers,
    )
}

fn accept_begin(
    queue: &mut RuntimeScrollGestureInputQueue,
    expected: RuntimeScrollGestureIdentity,
) -> Result<(), RuntimeScrollInputQueueError> {
    assert!(matches!(
        queue.pop_next()?,
        Some(RuntimeScrollInputQueueEvent::GestureBegin(_))
    ));
    queue.complete_begin(expected, RuntimeScrollGestureBeginDisposition::Tracking)
}

#[path = "scroll_input_queue_tests/gesture.rs"]
mod gesture;
#[path = "scroll_input_queue_tests/pressure.rs"]
mod pressure;
#[path = "scroll_input_queue_tests/terminal.rs"]
mod terminal;
#[path = "scroll_input_queue_tests/wheel.rs"]
mod wheel;
