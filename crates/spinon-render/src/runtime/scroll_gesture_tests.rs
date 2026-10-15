use spinon_core::{EnvironmentRevision, HostDocument, NodeId, ScrollStateRevision, StyleRevision};

use super::{
    CssAxisClip, CssClip, CssRect, CssSize, RuntimeOverscrollBehavior, RuntimePaint,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeScrollClipLayer,
    RuntimeScrollContainer, RuntimeScrollInputPolicy, RuntimeScrollRenderMetadata,
};

#[path = "scroll_gesture_tests/policy.rs"]
mod policy;

fn key() -> RuntimeRenderKey {
    let document = HostDocument::new().unwrap().snapshot();
    RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
}

fn policy(
    user_scrollable: bool,
    overscroll_x: RuntimeOverscrollBehavior,
    overscroll_y: RuntimeOverscrollBehavior,
) -> RuntimeScrollInputPolicy {
    policy_axes(user_scrollable, user_scrollable, overscroll_x, overscroll_y)
}

fn policy_axes(
    user_scrollable_x: bool,
    user_scrollable_y: bool,
    overscroll_x: RuntimeOverscrollBehavior,
    overscroll_y: RuntimeOverscrollBehavior,
) -> RuntimeScrollInputPolicy {
    RuntimeScrollInputPolicy::new(
        user_scrollable_x,
        user_scrollable_y,
        overscroll_x,
        overscroll_y,
    )
}

fn scroll_scene(
    inner_policy: RuntimeScrollInputPolicy,
) -> (RuntimeRenderSnapshot, NodeId, NodeId, NodeId) {
    scroll_scene_with_inner_width(inner_policy, 70.0)
}

fn scroll_scene_with_inner_width(
    inner_policy: RuntimeScrollInputPolicy,
    inner_scrollable_width: f32,
) -> (RuntimeRenderSnapshot, NodeId, NodeId, NodeId) {
    let outer = NodeId::new(51).unwrap();
    let inner = NodeId::new(52).unwrap();
    let target = NodeId::new(53).unwrap();
    let outer_frame = CssRect::new(0.0, 0.0, 180.0, 180.0).unwrap();
    let inner_frame = CssRect::new(20.0, 20.0, 120.0, 120.0).unwrap();
    let target_frame = CssRect::new(30.0, 30.0, 40.0, 40.0).unwrap();
    let outer_clip = super::super::CssClip::unconstrained();
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
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    ));
    let inner_container = RuntimeScrollContainer::new(
        inner,
        CssRect::new(20.0, 20.0, 60.0, 60.0).unwrap(),
        inner_scrollable_width,
        90.0,
        (inner_scrollable_width - 60.0).max(0.0),
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
    let metadata = vec![
        RuntimeScrollRenderMetadata::new(
            outer_frame,
            Vec::new(),
            vec![RuntimeScrollClipLayer::new(
                outer_clip,
                outer_clip,
                Vec::new(),
            )],
        ),
        RuntimeScrollRenderMetadata::new(
            inner_frame,
            vec![outer],
            vec![RuntimeScrollClipLayer::new(
                outer_clip,
                outer_clip,
                Vec::new(),
            )],
        ),
        RuntimeScrollRenderMetadata::new(
            target_frame,
            vec![inner, outer],
            vec![RuntimeScrollClipLayer::new(
                outer_clip,
                outer_clip,
                Vec::new(),
            )],
        ),
    ];
    let snapshot = RuntimeRenderSnapshot::new_with_scroll_metadata(
        key(),
        CssSize::new(200.0, 200.0).unwrap(),
        boxes,
        vec![outer_container, inner_container],
        metadata,
    )
    .unwrap();
    (snapshot, outer, inner, target)
}

#[cfg(feature = "c13-3-gesture-fixture-test")]
#[test]
fn fixture_input_policies_preserve_offsets_geometry_and_revision() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, _) = scroll_scene(auto);
    let before = snapshot.scroll_containers()[1];
    let updated = snapshot
        .with_scroll_input_policies(&[(
            inner,
            policy(
                true,
                RuntimeOverscrollBehavior::Contain,
                RuntimeOverscrollBehavior::Auto,
            ),
        )])
        .unwrap();

    assert_eq!(updated.key(), snapshot.key());
    assert_eq!(
        updated.scroll_containers()[0],
        snapshot.scroll_containers()[0]
    );
    assert_eq!(updated.scroll_containers()[1].owner(), before.owner());
    assert_eq!(
        updated.scroll_containers()[1].offset_x_css_px(),
        before.offset_x_css_px()
    );
    assert_eq!(
        updated.scroll_containers()[1].offset_y_css_px(),
        before.offset_y_css_px()
    );
    assert_eq!(updated.scroll_containers()[1].parent(), Some(outer));
    assert_eq!(
        updated.scroll_containers()[1].input_policy().overscroll_x(),
        RuntimeOverscrollBehavior::Contain
    );
    assert_eq!(updated.boxes(), snapshot.boxes());
}

#[test]
fn routes_inner_remainder_to_outer_and_publishes_one_revision() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, target) = scroll_scene(auto);
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 0.0, 20.0)
        .unwrap()
        .unwrap();

    assert_eq!(outcome.target(), target);
    assert_eq!(outcome.snapshot().key().scroll_state_revision().get(), 1);
    assert_eq!(outcome.snapshot().scroll_containers()[0].owner(), outer);
    assert_eq!(
        outcome.snapshot().scroll_containers()[0].offset_y_css_px(),
        15.0
    );
    assert_eq!(outcome.snapshot().scroll_containers()[1].owner(), inner);
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].offset_y_css_px(),
        30.0
    );
    assert_eq!(outcome.remaining_y_css_px(), 0.0);
    assert_eq!(outcome.changes().len(), 2);
    assert_eq!(outcome.changes()[0].owner(), inner);
    assert_eq!(outcome.changes()[0].consumed_y_css_px(), 5.0);
    assert_eq!(outcome.changes()[1].owner(), outer);
    assert_eq!(outcome.changes()[1].consumed_y_css_px(), 15.0);
}

#[test]
fn fractional_and_reverse_deltas_keep_css_px_precision_and_clamp_at_zero() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, _, inner, target) = scroll_scene(auto);
    let down = snapshot
        .route_scroll_delta_for_target(ScrollStateRevision::INITIAL, target, 0.0, 1.25)
        .unwrap();
    assert_eq!(
        down.snapshot().scroll_containers()[1].offset_y_css_px(),
        26.25
    );
    assert_eq!(down.snapshot().key().scroll_state_revision().get(), 1);

    let reverse = down
        .snapshot()
        .route_scroll_delta_for_target(
            down.snapshot().key().scroll_state_revision(),
            inner,
            0.0,
            -30.0,
        )
        .unwrap();
    assert_eq!(
        reverse.snapshot().scroll_containers()[1].offset_y_css_px(),
        0.0
    );
    assert_eq!(
        reverse.snapshot().scroll_containers()[0].offset_y_css_px(),
        0.0
    );
    assert_eq!(reverse.remaining_y_css_px(), -3.75);
    assert_eq!(reverse.snapshot().key().scroll_state_revision().get(), 2);
}

#[test]
fn target_scroll_owner_is_inclusive_and_its_policy_is_preserved_by_offsets() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, _, inner, _) = scroll_scene(auto);
    let outcome = snapshot
        .route_scroll_delta_for_target(ScrollStateRevision::INITIAL, inner, 0.0, 10.0)
        .unwrap();

    assert_eq!(outcome.target(), inner);
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].offset_y_css_px(),
        30.0
    );
    assert!(
        outcome.snapshot().scroll_containers()[1]
            .input_policy()
            .user_scrollable_y()
    );
}

#[test]
fn scroll_container_construction_disables_user_input_by_default() {
    let owner = NodeId::new(55).unwrap();
    let container = RuntimeScrollContainer::new(
        owner,
        CssRect::new(0.0, 0.0, 20.0, 20.0).unwrap(),
        40.0,
        40.0,
        0.0,
        0.0,
    )
    .unwrap();

    assert!(!container.input_policy().user_scrollable_x());
    assert!(!container.input_policy().user_scrollable_y());
}

#[test]
fn stale_or_non_finite_samples_fail_without_changing_the_source_snapshot() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, _, _, target) = scroll_scene(auto);
    let stale_revision = ScrollStateRevision::INITIAL.checked_next().unwrap();

    assert!(matches!(
        snapshot.route_scroll_delta_for_target(stale_revision, target, 0.0, 10.0),
        Err(super::RuntimeRenderError::StaleScrollRevision)
    ));
    assert!(matches!(
        snapshot.route_scroll_delta_for_target(ScrollStateRevision::INITIAL, target, f32::NAN, 0.0),
        Err(super::RuntimeRenderError::InvalidScrollDelta)
    ));
    assert!(matches!(
        snapshot.route_scroll_delta_at(ScrollStateRevision::INITIAL, f32::INFINITY, 35.0, 0.0, 0.0),
        Err(super::RuntimeRenderError::InvalidScrollPoint)
    ));
    assert!(matches!(
        snapshot.route_scroll_delta_for_target(
            ScrollStateRevision::INITIAL,
            NodeId::new(99).unwrap(),
            0.0,
            10.0
        ),
        Err(super::RuntimeRenderError::InvalidScrollTarget(_))
    ));
    assert_eq!(
        snapshot.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );
    assert_eq!(snapshot.scroll_containers()[1].offset_y_css_px(), 25.0);
}

#[test]
fn viewport_hit_miss_and_noop_do_not_advance_scroll_revision() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, _, _, _) = scroll_scene(auto);
    assert!(
        snapshot
            .route_scroll_delta_at(ScrollStateRevision::INITIAL, 199.0, 199.0, 20.0, 20.0)
            .unwrap()
            .is_none()
    );
    let outcome = snapshot
        .route_scroll_delta_at(ScrollStateRevision::INITIAL, 35.0, 35.0, 0.0, 0.0)
        .unwrap()
        .unwrap();
    assert!(outcome.changes().is_empty());
    assert_eq!(
        outcome.snapshot().key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );
}

#[test]
fn saturated_inner_and_outer_boundaries_keep_revision_and_report_remainder() {
    let auto = policy(
        true,
        RuntimeOverscrollBehavior::Auto,
        RuntimeOverscrollBehavior::Auto,
    );
    let (snapshot, outer, inner, target) = scroll_scene(auto);
    let saturated = snapshot
        .with_scroll_offsets(
            ScrollStateRevision::INITIAL,
            &[(outer, 100.0, 100.0), (inner, 10.0, 30.0)],
        )
        .unwrap();
    let revision = saturated.key().scroll_state_revision();
    let outcome = saturated
        .route_scroll_delta_for_target(revision, target, 20.0, 20.0)
        .unwrap();

    assert!(outcome.changes().is_empty());
    assert_eq!(outcome.remaining_x_css_px(), 20.0);
    assert_eq!(outcome.remaining_y_css_px(), 20.0);
    assert_eq!(outcome.snapshot().key().scroll_state_revision(), revision);
    assert_eq!(
        outcome.snapshot().scroll_containers()[0].offset_x_css_px(),
        100.0
    );
    assert_eq!(
        outcome.snapshot().scroll_containers()[1].offset_y_css_px(),
        30.0
    );
}

#[test]
fn hit_test_clip_miss_does_not_select_a_scroll_target() {
    let target = NodeId::new(54).unwrap();
    let frame = CssRect::new(0.0, 0.0, 100.0, 100.0).unwrap();
    let clip = CssClip::unconstrained().with_x(CssAxisClip::new(0.0, 20.0).unwrap());
    let snapshot = RuntimeRenderSnapshot::new(
        key(),
        CssSize::new(100.0, 100.0).unwrap(),
        vec![RuntimeRenderBox::new(target, frame, RuntimePaint::None, 0).with_hit_test_clip(clip)],
    )
    .unwrap();

    assert!(
        snapshot
            .route_scroll_delta_at(ScrollStateRevision::INITIAL, 30.0, 10.0, 0.0, 20.0)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        snapshot.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );
}
