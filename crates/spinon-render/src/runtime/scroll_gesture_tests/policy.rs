use super::{
    RuntimeOverscrollBehavior, ScrollStateRevision, policy, policy_axes, scroll_scene,
    scroll_scene_with_inner_width,
};

#[test]
fn overscroll_contain_stops_only_its_axis() {
    let inner_policy = policy(
        true,
        RuntimeOverscrollBehavior::Contain,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, _) = scroll_scene(inner_policy);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 20.0, 20.0)
        .unwrap()
        .unwrap();
    let containers = outcome.snapshot().scroll_containers();

    assert_eq!(containers[0].owner(), outer);
    assert_eq!(containers[0].offset_x_css_px(), 0.0);
    assert_eq!(containers[0].offset_y_css_px(), 15.0);
    assert_eq!(containers[1].owner(), inner);
    assert_eq!(containers[1].offset_x_css_px(), 10.0);
    assert_eq!(containers[1].offset_y_css_px(), 30.0);
    assert_eq!(outcome.remaining_x_css_px(), 20.0);
    assert_eq!(outcome.remaining_y_css_px(), 0.0);
    assert_eq!(outcome.changes().len(), 2);
    assert_eq!(outcome.changes()[0].consumed_y_css_px(), 5.0);
    assert_eq!(outcome.changes()[1].consumed_x_css_px(), 0.0);
    assert_eq!(outcome.changes()[1].consumed_y_css_px(), 15.0);
}

#[test]
fn user_scrollability_is_independent_per_axis() {
    let inner_policy = policy_axes(
        false,
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, _) = scroll_scene(inner_policy);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 20.0, 20.0)
        .unwrap()
        .unwrap();
    let containers = outcome.snapshot().scroll_containers();

    assert_eq!(containers[0].owner(), outer);
    assert_eq!(containers[0].offset_x_css_px(), 20.0);
    assert_eq!(containers[0].offset_y_css_px(), 15.0);
    assert_eq!(containers[1].owner(), inner);
    assert_eq!(containers[1].offset_x_css_px(), 10.0);
    assert_eq!(containers[1].offset_y_css_px(), 30.0);
}

#[test]
fn overscroll_none_and_contain_both_stop_only_the_configured_axis() {
    for overscroll_x in [
        RuntimeOverscrollBehavior::Contain,
        RuntimeOverscrollBehavior::None,
    ] {
        let inner_policy = policy_axes(true, true, overscroll_x, RuntimeOverscrollBehavior::Auto);
        let (snapshot, outer, inner, _) = scroll_scene(inner_policy);
        let outcome = snapshot
            .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 20.0, 20.0)
            .unwrap()
            .unwrap();

        assert_eq!(outcome.snapshot().scroll_containers()[0].owner(), outer);
        assert_eq!(
            outcome.snapshot().scroll_containers()[0].offset_x_css_px(),
            0.0
        );
        assert_eq!(
            outcome.snapshot().scroll_containers()[0].offset_y_css_px(),
            15.0
        );
        assert_eq!(outcome.snapshot().scroll_containers()[1].owner(), inner);
        assert_eq!(outcome.remaining_x_css_px(), 20.0);
        assert_eq!(outcome.remaining_y_css_px(), 0.0);
    }
}

#[test]
fn axis_without_user_scroll_range_forwards_delta_to_parent() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, _) = scroll_scene_with_inner_width(auto, 60.0);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 12.0, 0.0)
        .unwrap()
        .unwrap();

    assert_eq!(outcome.snapshot().scroll_containers()[0].owner(), outer);
    assert_eq!(
        outcome.snapshot().scroll_containers()[0].offset_x_css_px(),
        12.0
    );
    assert_eq!(outcome.snapshot().scroll_containers()[1].owner(), inner);
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].max_offset_x_css_px(),
        0.0
    );
    assert_eq!(outcome.remaining_x_css_px(), 0.0);
    assert_eq!(outcome.changes().len(), 1);
    assert_eq!(outcome.changes()[0].owner(), outer);
}

#[test]
fn hidden_boundary_does_not_consume_but_auto_forwards_remainder() {
    let hidden_policy = policy(
        false,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, _) = scroll_scene(hidden_policy);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 0.0, 20.0)
        .unwrap()
        .unwrap();

    assert_eq!(outcome.snapshot().scroll_containers()[0].owner(), outer);
    assert_eq!(
        outcome.snapshot().scroll_containers()[0].offset_y_css_px(),
        20.0
    );
    assert_eq!(outcome.snapshot().scroll_containers()[1].owner(), inner);
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].offset_y_css_px(),
        25.0
    );
    assert_eq!(outcome.changes().len(), 1);
    assert_eq!(outcome.changes()[0].owner(), outer);
    assert_eq!(outcome.changes()[0].consumed_y_css_px(), 20.0);
}

#[test]
fn hidden_boundary_can_stop_chaining_without_consuming() {
    let hidden_policy = policy_axes(
        false,
        false,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Contain,
    );
    let (snapshot, outer, inner, _) = scroll_scene(hidden_policy);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 0.0, 20.0)
        .unwrap()
        .unwrap();

    assert_eq!(outcome.snapshot().scroll_containers()[0].owner(), outer);
    assert_eq!(
        outcome.snapshot().scroll_containers()[0].offset_y_css_px(),
        0.0
    );
    assert_eq!(outcome.snapshot().scroll_containers()[1].owner(), inner);
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].offset_y_css_px(),
        25.0
    );
    assert_eq!(outcome.remaining_y_css_px(), 20.0);
    assert!(outcome.changes().is_empty());
}
