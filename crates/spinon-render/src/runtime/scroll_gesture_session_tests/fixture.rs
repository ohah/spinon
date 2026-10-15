use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};

use super::super::{
    CssAxisClip, CssClip, CssRect, CssSize, RuntimeOverscrollBehavior, RuntimePaint,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeScrollClipLayer,
    RuntimeScrollContainer, RuntimeScrollGestureBegin, RuntimeScrollGesturePanSample,
    RuntimeScrollGestureTerminal, RuntimeScrollInputPolicy, RuntimeScrollRenderMetadata,
    RuntimeTouchAction,
};

pub(super) const OUTER: u64 = 51;
pub(super) const INNER: u64 = 52;
pub(super) const TARGET: u64 = 53;

pub(super) fn key() -> RuntimeRenderKey {
    let document = HostDocument::new().unwrap().snapshot();
    RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
}

pub(super) fn with_style(key: RuntimeRenderKey, style: StyleRevision) -> RuntimeRenderKey {
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

pub(super) fn with_environment(
    key: RuntimeRenderKey,
    environment: EnvironmentRevision,
) -> RuntimeRenderKey {
    RuntimeRenderKey::new(
        key.generation(),
        key.document_revision(),
        key.render_tree_revision(),
        key.style_revision(),
        environment,
    )
    .with_intrinsic_size_revision(key.intrinsic_size_revision())
    .with_scroll_state_revision(key.scroll_state_revision())
}

pub(super) fn policy(
    overscroll_x: RuntimeOverscrollBehavior,
    overscroll_y: RuntimeOverscrollBehavior,
) -> RuntimeScrollInputPolicy {
    RuntimeScrollInputPolicy::new(true, true, overscroll_x, overscroll_y)
}

pub(super) fn scene(
    key: RuntimeRenderKey,
    inner_scrollable_width: f32,
    inner_policy: RuntimeScrollInputPolicy,
    clip_target_x_at: Option<f32>,
) -> RuntimeRenderSnapshot {
    let outer = NodeId::new(OUTER).unwrap();
    let inner = NodeId::new(INNER).unwrap();
    let target = NodeId::new(TARGET).unwrap();
    let outer_frame = CssRect::new(0.0, 0.0, 180.0, 180.0).unwrap();
    let inner_frame = CssRect::new(20.0, 20.0, 120.0, 120.0).unwrap();
    let target_frame = CssRect::new(30.0, 30.0, 40.0, 40.0).unwrap();
    let unconstrained = CssClip::unconstrained();
    let outer_container = RuntimeScrollContainer::new(
        outer,
        CssRect::new(0.0, 0.0, 100.0, 100.0).unwrap(),
        200.0,
        200.0,
        0.0,
        0.0,
    )
    .unwrap()
    .with_input_policy(policy(
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    ));
    let inner_container = RuntimeScrollContainer::new(
        inner,
        CssRect::new(20.0, 20.0, 60.0, 60.0).unwrap(),
        inner_scrollable_width,
        90.0,
        10.0,
        25.0,
    )
    .unwrap()
    .with_parent(Some(outer))
    .with_input_policy(inner_policy);
    let boxes = vec![
        RuntimeRenderBox::new(outer, outer_frame, RuntimePaint::None, 0),
        RuntimeRenderBox::new(inner, inner_frame, RuntimePaint::None, 1),
        RuntimeRenderBox::new(target, target_frame, RuntimePaint::None, 2),
    ];
    let target_clip = clip_target_x_at.map_or(unconstrained, |max| {
        unconstrained.with_x(CssAxisClip::new(0.0, max).unwrap())
    });
    let metadata = vec![
        RuntimeScrollRenderMetadata::new(
            outer_frame,
            Vec::new(),
            vec![RuntimeScrollClipLayer::new(
                unconstrained,
                unconstrained,
                Vec::new(),
            )],
        ),
        RuntimeScrollRenderMetadata::new(
            inner_frame,
            vec![outer],
            vec![RuntimeScrollClipLayer::new(
                unconstrained,
                unconstrained,
                Vec::new(),
            )],
        ),
        RuntimeScrollRenderMetadata::new(
            target_frame,
            vec![inner, outer],
            vec![RuntimeScrollClipLayer::new(
                unconstrained,
                target_clip,
                Vec::new(),
            )],
        ),
    ];
    RuntimeRenderSnapshot::new_with_scroll_metadata(
        key,
        CssSize::new(200.0, 200.0).unwrap(),
        boxes,
        vec![outer_container, inner_container],
        metadata,
    )
    .unwrap()
}

pub(super) fn begin(
    snapshot: &RuntimeRenderSnapshot,
    sequence: u64,
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
    touch_action: RuntimeTouchAction,
) -> RuntimeScrollGestureBegin {
    RuntimeScrollGestureBegin::new(
        snapshot.key(),
        surface_generation,
        sequence,
        gesture_id,
        pointer_id,
        35.0,
        35.0,
        1.0,
        touch_action,
    )
}

pub(super) fn pan(
    snapshot: &RuntimeRenderSnapshot,
    sequence: u64,
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
    cumulative_x: f32,
    cumulative_y: f32,
) -> RuntimeScrollGesturePanSample {
    RuntimeScrollGesturePanSample::new(
        snapshot.key(),
        surface_generation,
        sequence,
        gesture_id,
        pointer_id,
        1.0,
        cumulative_x,
        cumulative_y,
    )
}

pub(super) fn terminal(
    sequence: u64,
    gesture_id: u64,
    pointer_id: u64,
    surface_generation: u64,
) -> RuntimeScrollGestureTerminal {
    RuntimeScrollGestureTerminal::new(sequence, gesture_id, pointer_id, surface_generation)
}

pub(super) fn node(id: u64) -> NodeId {
    NodeId::new(id).unwrap()
}
