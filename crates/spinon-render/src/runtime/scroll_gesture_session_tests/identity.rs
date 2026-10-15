use super::super::*;
use super::fixture::*;

#[test]
fn second_pointer_cancels_until_every_contact_ends() {
    let snapshot = scene(
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
            &snapshot,
            begin(&snapshot, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let second_pointer = RuntimeScrollGestureBegin::new(
        snapshot.key(),
        1,
        2,
        1,
        3,
        f32::NAN,
        0.0,
        1.0,
        RuntimeTouchAction::Unsupported,
    );
    assert_eq!(
        session.begin(&snapshot, second_pointer).unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::SecondPointer)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Suppressed);
    assert_eq!(
        session.end(terminal(3, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 1,
        }
    );
    assert_eq!(
        session.end(terminal(4, 1, 3, 1)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 0,
        }
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
}

#[test]
fn suppressed_contact_id_does_not_consume_a_future_gesture_id() {
    let snapshot = scene(
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
            &snapshot,
            begin(&snapshot, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let suppressed_contact = RuntimeScrollGestureBegin::new(
        snapshot.key(),
        1,
        2,
        u64::MAX,
        3,
        f32::NAN,
        0.0,
        1.0,
        RuntimeTouchAction::Unsupported,
    );
    assert_eq!(
        session.begin(&snapshot, suppressed_contact).unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::SecondPointer)
    );
    session.end(terminal(3, 1, 2, 1)).unwrap();
    session.end(terminal(4, u64::MAX, 3, 1)).unwrap();
    assert!(matches!(
        session.begin(
            &snapshot,
            begin(&snapshot, 5, 2, 4, 1, RuntimeTouchAction::Auto)
        ),
        Ok(RuntimeScrollGestureOutcome::Began { .. })
    ));
}

#[test]
fn stale_surface_terminal_cannot_release_a_suppressed_contact() {
    let snapshot = scene(
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
            &snapshot,
            begin(&snapshot, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let second = RuntimeScrollGestureBegin::new(
        snapshot.key(),
        2,
        2,
        1,
        3,
        f32::NAN,
        0.0,
        1.0,
        RuntimeTouchAction::Unsupported,
    );
    assert_eq!(
        session.begin(&snapshot, second).unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::SecondPointer)
    );
    assert_eq!(
        session.end(terminal(3, 1, 2, 2)),
        Err(RuntimeScrollGestureError::GestureIdentityMismatch)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Suppressed);
    assert_eq!(
        session.end(terminal(3, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 1,
        }
    );
    assert_eq!(
        session.end(terminal(4, 1, 3, 2)).unwrap(),
        RuntimeScrollGestureOutcome::SuppressedPointerEnded {
            blocked_contacts_remaining: 0,
        }
    );
}

#[test]
fn invalid_begin_duplicate_terminal_and_non_monotonic_sequence_fail_closed() {
    let snapshot = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        Some(34.0),
    );
    let mut session = RuntimeScrollGestureSession::default();
    assert_eq!(
        session.begin(
            &snapshot,
            RuntimeScrollGestureBegin::new(
                snapshot.key(),
                1,
                1,
                1,
                2,
                200.0,
                35.0,
                1.0,
                RuntimeTouchAction::Auto,
            ),
        ),
        Err(RuntimeScrollGestureError::HitTestMiss)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);

    let clear = scene(
        snapshot.key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    assert_eq!(
        session.begin(
            &clear,
            begin(&clear, 1, 1, 2, 1, RuntimeTouchAction::Unsupported),
        ),
        Err(RuntimeScrollGestureError::UnsupportedTouchAction)
    );
    session
        .begin(&clear, begin(&clear, 1, 1, 2, 1, RuntimeTouchAction::Auto))
        .unwrap();
    assert_eq!(
        session.end(terminal(2, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::Ended
    );
    assert_eq!(
        session.end(terminal(3, 1, 2, 1)),
        Err(RuntimeScrollGestureError::UnknownPointer)
    );
    assert_eq!(
        session.begin(&clear, begin(&clear, 2, 2, 3, 1, RuntimeTouchAction::Auto)),
        Err(RuntimeScrollGestureError::NonMonotonicInputSequence)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
}
#[test]
fn clip_and_coordinate_scale_change_cancel_without_retargeting() {
    let initial = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let clipped = scene(
        initial.key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        Some(34.0),
    );
    let mut clip_session = RuntimeScrollGestureSession::default();
    clip_session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    assert_eq!(
        clip_session
            .pan_began(&clipped, pan(&clipped, 2, 1, 2, 1, 0.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::GeometryChanged)
    );

    let mut scale_session = RuntimeScrollGestureSession::default();
    scale_session
        .begin(
            &initial,
            begin(&initial, 1, 2, 3, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    let changed_scale =
        RuntimeScrollGesturePanSample::new(initial.key(), 1, 2, 2, 3, 2.0, 0.0, -8.0);
    assert_eq!(
        scale_session.pan_began(&initial, changed_scale).unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(
            RuntimeScrollGestureCancelReason::SurfaceGenerationChanged
        )
    );
}

#[test]
fn stale_and_invalid_begin_inputs_leave_session_idle() {
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
    let stale_key = initial
        .key()
        .with_scroll_state_revision(ScrollStateRevision::INITIAL.checked_next().unwrap());
    assert_eq!(
        session.begin(
            &initial,
            RuntimeScrollGestureBegin::new(
                stale_key,
                1,
                1,
                1,
                2,
                35.0,
                35.0,
                1.0,
                RuntimeTouchAction::Auto,
            ),
        ),
        Err(RuntimeScrollGestureError::StaleExpectedSnapshot)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
    assert_eq!(
        session.begin(
            &initial,
            RuntimeScrollGestureBegin::new(
                initial.key(),
                1,
                1,
                1,
                2,
                f32::NAN,
                35.0,
                1.0,
                RuntimeTouchAction::Auto,
            ),
        ),
        Err(RuntimeScrollGestureError::InvalidPoint)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
    assert_eq!(
        session.begin(
            &initial,
            RuntimeScrollGestureBegin::new(
                initial.key(),
                1,
                1,
                1,
                2,
                35.0,
                35.0,
                0.0,
                RuntimeTouchAction::Auto,
            ),
        ),
        Err(RuntimeScrollGestureError::InvalidCoordinateScale)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Idle);
    session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
}

#[test]
fn mismatched_terminal_is_rejected_and_host_cancel_ignores_late_move() {
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
    assert_eq!(
        session.end(terminal(2, 9, 2, 1)),
        Err(RuntimeScrollGestureError::GestureIdentityMismatch)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Tracking);
    assert_eq!(
        session.end(terminal(2, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::Ended
    );
    assert_eq!(
        session.pan_moved(&initial, pan(&initial, 3, 1, 2, 1, 0.0, -8.0)),
        Err(RuntimeScrollGestureError::NoActiveGesture)
    );
    assert_eq!(
        initial.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );

    let mut cancelled = RuntimeScrollGestureSession::default();
    cancelled
        .begin(
            &initial,
            begin(&initial, 1, 2, 3, 1, RuntimeTouchAction::Auto),
        )
        .unwrap();
    assert_eq!(
        cancelled
            .cancel(
                terminal(2, 2, 3, 1),
                RuntimeScrollGestureCancelReason::HostCancelled,
            )
            .unwrap(),
        RuntimeScrollGestureOutcome::Cancelled(RuntimeScrollGestureCancelReason::HostCancelled)
    );
    assert_eq!(
        cancelled.pan_moved(&initial, pan(&initial, 3, 2, 3, 1, 0.0, -8.0)),
        Err(RuntimeScrollGestureError::NoActiveGesture)
    );
    assert_eq!(cancelled.state(), RuntimeScrollGestureState::Idle);
}

#[test]
fn duplicate_active_pointer_and_stale_pan_leave_the_original_session_usable() {
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
    assert_eq!(
        session.begin(
            &initial,
            begin(&initial, 2, 2, 2, 1, RuntimeTouchAction::Auto),
        ),
        Err(RuntimeScrollGestureError::GestureAlreadyActive)
    );
    let stale_key = initial
        .key()
        .with_scroll_state_revision(ScrollStateRevision::INITIAL.checked_next().unwrap());
    let stale_pan = RuntimeScrollGesturePanSample::new(stale_key, 1, 2, 1, 2, 1.0, 0.0, -8.0);
    assert_eq!(
        session.pan_began(&initial, stale_pan),
        Err(RuntimeScrollGestureError::StaleExpectedSnapshot)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Tracking);
    assert!(matches!(
        session.pan_began(&initial, pan(&initial, 2, 1, 2, 1, 0.0, -8.0)),
        Ok(RuntimeScrollGestureOutcome::PanStarted { .. })
    ));
}
