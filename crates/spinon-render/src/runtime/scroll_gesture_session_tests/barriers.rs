use super::super::*;
use super::fixture::*;

#[test]
fn geometry_change_cancels_and_blocks_pointer_until_its_terminal() {
    let initial = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let mut session = RuntimeScrollGestureSession::default();
    session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let changed_geometry = scene(
        initial.key(),
        140.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    assert_eq!(
        session
            .pan_began(
                &changed_geometry,
                pan(&changed_geometry, 2, 1, 2, 1, 0.0, -12.0),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::GeometryChanged)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Suppressed);
    assert_eq!(
        session.pan_moved(
            &changed_geometry,
            pan(&changed_geometry, 3, 1, 2, 1, 0.0, -32.0),
        ),
        Err(RuntimeScrollGestureError::NoActiveGesture)
    );
    assert_eq!(
        changed_geometry.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );
    assert_eq!(
        session
            .begin(
                &changed_geometry,
                begin(&changed_geometry, 3, 2, 3, 1, RuntimeTouchAction::Auto),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::SecondPointer)
    );
    assert_eq!(
        session.end(terminal(4, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 1,
        }
    );
    assert_eq!(
        session.end(terminal(5, 2, 3, 1)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 0,
        }
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
    assert!(matches!(
        session.begin(
            &changed_geometry,
            begin(&changed_geometry, 6, 3, 4, 1, RuntimeTouchAction::Auto),
        ),
        Ok(RuntimeScrollGestureOutcome::Began { .. })
    ));
}

#[test]
fn document_generation_change_cancels_active_gesture() {
    let initial = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let changed_generation = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let mut session = RuntimeScrollGestureSession::default();
    session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    assert_eq!(
        session
            .pan_began(
                &changed_generation,
                pan(&changed_generation, 2, 1, 2, 1, 0.0, -8.0),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(
            RuntimeScrollGestureCancelReason::DocumentOrTreeChanged
        )
    );
}

#[test]
fn external_scroll_revision_cancels_without_applying_the_late_pan() {
    let initial = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let mut session = RuntimeScrollGestureSession::default();
    session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let external = initial
        .route_scroll_delta_for_target(ScrollStateRevision::INITIAL, node(INNER), 0.0, 5.0)
        .unwrap();
    let externally_scrolled = external.snapshot();
    assert_eq!(
        session
            .pan_began(
                externally_scrolled,
                pan(externally_scrolled, 2, 1, 2, 1, 0.0, -8.0),
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(
            RuntimeScrollGestureCancelReason::ExternalScrollUpdate
        )
    );
    assert_eq!(
        externally_scrolled.scroll_containers()[1].offset_y_css_px(),
        30.0
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Suppressed);
    session.end(terminal(3, 1, 2, 1)).unwrap();
}
#[test]
fn environment_and_surface_barriers_cancel_active_gesture() {
    let initial_key = key();
    let initial = scene(
        initial_key,
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let mut environment_session = RuntimeScrollGestureSession::default();
    environment_session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let environment = scene(
        with_environment(
            initial_key,
            initial_key.environment_revision().checked_next().unwrap(),
        ),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    assert_eq!(
        environment_session
            .pan_began(&environment, pan(&environment, 2, 1, 2, 1, 0.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(
            RuntimeScrollGestureCancelReason::EnvironmentChanged
        )
    );

    let mut surface_session = RuntimeScrollGestureSession::default();
    surface_session
        .begin(
            &initial,
            begin(&initial, 1, 2, 4, 7, RuntimeTouchAction::Auto),
        )
        .unwrap();
    assert_eq!(
        surface_session
            .pan_began(&initial, pan(&initial, 2, 2, 4, 8, 0.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(
            RuntimeScrollGestureCancelReason::SurfaceGenerationChanged
        )
    );
}
