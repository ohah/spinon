use spinon_render::{
    RuntimeScrollGestureBegin, RuntimeScrollGestureOutcome, RuntimeScrollGesturePanSample,
    RuntimeScrollGestureSession, RuntimeScrollGestureState, RuntimeTouchAction,
};

use super::super::{SystemReserve, build_runtime_render_snapshot_with_reserve};
use super::scroll_gesture_runtime_inputs_with_viewport;

fn policy_scene(
    inner_action: &str,
    content_action: &str,
) -> (
    spinon_render::RuntimeRenderSnapshot,
    Vec<spinon_core::NodeId>,
) {
    let styles = [
        "position:relative;box-sizing:border-box;width:200px;height:160px;overflow:auto;touch-action:none;background-color:#101827".to_owned(),
        "position:relative;width:400px;height:300px;background-color:#344256".to_owned(),
        format!("position:absolute;left:20px;top:20px;box-sizing:border-box;width:80px;height:60px;overflow:auto;overscroll-behavior-x:contain;overscroll-behavior-y:auto;touch-action:{inner_action};background-color:#1b3441"),
        format!("position:relative;width:160px;height:120px;touch-action:{content_action};background-color:#17253a"),
    ];
    let css = styles.iter().map(String::as_str).collect::<Vec<_>>();
    let (input, nodes) = scroll_gesture_runtime_inputs_with_viewport(
        &css,
        &[None, Some(0), Some(1), Some(2)],
        300.0,
        220.0,
    );
    let snapshot = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    (snapshot, nodes)
}

#[test]
fn computed_overscroll_policies_reach_the_router_per_axis() {
    let (snapshot, nodes) = policy_scene("auto", "pan-x");
    assert_eq!(snapshot.scroll_containers().len(), 2);
    let inner = snapshot
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[2])
        .unwrap();
    assert!(inner.input_policy().user_scrollable_x());
    assert!(inner.input_policy().user_scrollable_y());
    assert_eq!(
        inner.input_policy().overscroll_x(),
        spinon_render::RuntimeOverscrollBehavior::Contain
    );
    assert_eq!(
        inner.input_policy().overscroll_y(),
        spinon_render::RuntimeOverscrollBehavior::Auto
    );

    let outcome = snapshot
        .route_scroll_delta_at(
            snapshot.key().scroll_state_revision(),
            35.0,
            35.0,
            100.0,
            100.0,
        )
        .unwrap()
        .unwrap();
    let moved = outcome.snapshot();
    let inner = moved
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[2])
        .unwrap();
    let root = moved
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[0])
        .unwrap();
    assert_eq!(
        (inner.offset_x_css_px(), inner.offset_y_css_px()),
        (80.0, 60.0)
    );
    assert_eq!(
        (root.offset_x_css_px(), root.offset_y_css_px()),
        (0.0, 40.0)
    );
}

#[test]
fn touch_action_intersects_target_to_nearest_owner_and_freezes_pan_axes() {
    let (snapshot, nodes) = policy_scene("auto", "pan-x");
    let action = snapshot.touch_action_at_css_point(35.0, 35.0).unwrap();
    assert_eq!(action, RuntimeTouchAction::PanX);

    let mut vertical = RuntimeScrollGestureSession::default();
    vertical
        .begin(
            &snapshot,
            RuntimeScrollGestureBegin::new(snapshot.key(), 1, 1, 1, 1, 35.0, 35.0, 1.0, action),
        )
        .unwrap();
    assert_eq!(
        vertical
            .pan_began(
                &snapshot,
                RuntimeScrollGesturePanSample::new(snapshot.key(), 1, 2, 1, 1, 1.0, 0.0, -20.0),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::PanRejected
    );
    assert_eq!(vertical.state(), RuntimeScrollGestureState::PanRejected);
    let inner = snapshot
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[2])
        .unwrap();
    assert_eq!(
        (inner.offset_x_css_px(), inner.offset_y_css_px()),
        (0.0, 0.0)
    );

    let mut horizontal = RuntimeScrollGestureSession::default();
    horizontal
        .begin(
            &snapshot,
            RuntimeScrollGestureBegin::new(snapshot.key(), 1, 1, 2, 1, 35.0, 35.0, 1.0, action),
        )
        .unwrap();
    assert!(matches!(
        horizontal
            .pan_began(
                &snapshot,
                RuntimeScrollGesturePanSample::new(snapshot.key(), 1, 2, 2, 1, 1.0, -12.0, 0.0),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::PanStarted { .. }
    ));
}

#[test]
fn touch_action_intersects_intermediate_nodes_but_stops_at_nearest_scroll_owner() {
    let (limited, _) = policy_scene("pan-y", "pan-x");
    assert_eq!(
        limited.touch_action_at_css_point(35.0, 35.0),
        Some(RuntimeTouchAction::None)
    );

    let (unsupported, _) = policy_scene("auto", "manipulation");
    assert_eq!(
        unsupported.touch_action_at_css_point(35.0, 35.0),
        Some(RuntimeTouchAction::Unsupported)
    );
}
