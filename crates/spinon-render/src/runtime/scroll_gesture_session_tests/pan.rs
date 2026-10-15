use super::super::*;
use super::fixture::*;

#[test]
fn begin_freezes_clip_aware_target_owner_chain_and_axis_through_cumulative_pan() {
    let mut session = RuntimeScrollGestureSession::default();
    let initial = scene(
        key(),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let began = session
        .begin(
            &initial,
            begin(&initial, 1, 1, 9, 4, RuntimeTouchAction::PanY),
        )
        .unwrap();
    let RuntimeScrollGestureOutcome::Began {
        target,
        owner_chain,
        allowed_axes,
    } = began
    else {
        panic!("Begin은 고정한 target과 owner chain을 반환해야 합니다");
    };
    assert_eq!(target, node(TARGET));
    assert_eq!(owner_chain, [node(INNER), node(OUTER)]);
    assert!(!allowed_axes.x());
    assert!(allowed_axes.y());
    assert_eq!(session.state(), RuntimeScrollGestureState::Tracking);

    let first = session
        .pan_began(&initial, pan(&initial, 2, 1, 9, 4, 0.0, -10.0))
        .unwrap();
    let RuntimeScrollGestureOutcome::PanStarted { axes, delta } = first else {
        panic!("pan 시작 입력이 accept되어야 합니다");
    };
    assert!(!axes.x());
    assert!(axes.y());
    assert_eq!(delta.target(), node(TARGET));
    assert_eq!(delta.snapshot().key().scroll_state_revision().get(), 1);
    assert_eq!(
        delta.snapshot().scroll_containers()[0].offset_y_css_px(),
        5.0
    );
    assert_eq!(
        delta.snapshot().scroll_containers()[1].offset_y_css_px(),
        30.0
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Panning(axes));
    assert_eq!(
        session.pan_began(
            delta.snapshot(),
            pan(delta.snapshot(), 3, 1, 9, 4, 0.0, -10.0),
        ),
        Err(RuntimeScrollGestureError::PanAlreadyStarted)
    );

    let same = session
        .pan_moved(
            delta.snapshot(),
            pan(delta.snapshot(), 3, 1, 9, 4, 0.0, -10.0),
        )
        .unwrap();
    let RuntimeScrollGestureOutcome::Moved { delta: Some(same) } = same else {
        panic!("cumulative translation 반복은 no-op sample이어야 합니다");
    };
    assert!(same.changes().is_empty());
    assert_eq!(same.snapshot().key().scroll_state_revision().get(), 1);

    let moved = session
        .pan_moved(
            same.snapshot(),
            pan(same.snapshot(), 4, 1, 9, 4, 0.0, -15.0),
        )
        .unwrap();
    let RuntimeScrollGestureOutcome::Moved { delta: Some(moved) } = moved else {
        panic!("새 cumulative translation은 delta 한 번을 적용해야 합니다");
    };
    assert_eq!(
        moved.snapshot().scroll_containers()[0].offset_y_css_px(),
        10.0
    );
    assert_eq!(moved.snapshot().key().scroll_state_revision().get(), 2);
}
#[test]
fn disallowed_dominant_axis_rejects_pan_and_never_relocks_later() {
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
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::PanX),
        )
        .unwrap();
    assert_eq!(
        session
            .pan_began(&initial, pan(&initial, 2, 1, 2, 1, 0.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::PanRejected
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::PanRejected);
    assert_eq!(
        session
            .pan_moved(&initial, pan(&initial, 3, 1, 2, 1, -20.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::PanRejected
    );
    assert_eq!(
        initial.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );
    assert_eq!(
        session.end(terminal(4, 1, 2, 1)).unwrap(),
        RuntimeScrollGestureOutcome::Ended
    );

    let mut none = RuntimeScrollGestureSession::default();
    none.begin(
        &initial,
        begin(&initial, 1, 4, 5, 1, RuntimeTouchAction::None),
    )
    .unwrap();
    assert_eq!(
        none.pan_began(&initial, pan(&initial, 2, 4, 5, 1, 0.0, -8.0))
            .unwrap(),
        RuntimeScrollGestureOutcome::PanRejected
    );
    assert_eq!(
        initial.key().scroll_state_revision(),
        ScrollStateRevision::INITIAL
    );

    let mut tie = RuntimeScrollGestureSession::default();
    tie.begin(
        &initial,
        begin(&initial, 1, 2, 3, 1, RuntimeTouchAction::PanX),
    )
    .unwrap();
    let outcome = tie
        .pan_began(&initial, pan(&initial, 2, 2, 3, 1, -8.0, 8.0))
        .unwrap();
    let RuntimeScrollGestureOutcome::PanStarted { axes, .. } = outcome else {
        panic!("대각선 동률에서는 허용된 x 축으로 시작해야 합니다");
    };
    assert!(axes.x());
    assert!(!axes.y());
}
#[test]
fn style_only_revision_keeps_start_action_and_uses_next_sample_policy() {
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
    let mut session = RuntimeScrollGestureSession::default();
    session
        .begin(
            &initial,
            begin(&initial, 1, 1, 2, 1, RuntimeTouchAction::PanX),
        )
        .unwrap();

    let next_style = initial_key.style_revision().checked_next().unwrap();
    let styled = scene(
        with_style(initial_key, next_style),
        120.0,
        policy(
            RuntimeOverscrollBehavior::Contain,
            RuntimeOverscrollBehavior::Auto,
        ),
        None,
    );
    let outcome = session
        .pan_began(&styled, pan(&styled, 2, 1, 2, 1, -100.0, 0.0))
        .unwrap();
    let RuntimeScrollGestureOutcome::PanStarted { axes, delta } = outcome else {
        panic!("style-only revision은 frozen horizontal action을 유지합니다");
    };
    assert!(axes.x());
    assert!(!axes.y());
    assert_eq!(
        delta.snapshot().scroll_containers()[1].offset_x_css_px(),
        60.0
    );
    assert_eq!(
        delta.snapshot().scroll_containers()[0].offset_x_css_px(),
        0.0
    );
    assert_eq!(delta.remaining_x_css_px(), 50.0);
}

#[test]
fn non_finite_pan_does_not_advance_sequence_or_change_session_state() {
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
        session.pan_began(&initial, pan(&initial, 2, 1, 2, 1, f32::NAN, 0.0)),
        Err(RuntimeScrollGestureError::InvalidPanTranslation)
    );
    assert_eq!(session.state(), RuntimeScrollGestureState::Tracking);

    let started = session
        .pan_began(&initial, pan(&initial, 2, 1, 2, 1, 0.0, -8.0))
        .unwrap();
    assert!(matches!(
        started,
        RuntimeScrollGestureOutcome::PanStarted { .. }
    ));
}
