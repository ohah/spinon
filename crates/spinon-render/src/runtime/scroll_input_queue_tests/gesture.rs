use super::*;

#[test]
fn begin_reserves_terminal_capacity_before_queueing_and_returns_it_after_terminal() {
    let key = key();
    let mut queue = RuntimeScrollGestureInputQueue::new(1, 1).unwrap();
    let first = identity(1, 4, 7);
    let second = identity(2, 5, 7);

    assert_eq!(
        queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(queue.terminal_reservations_in_use(), 1);
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 2, 2, 5, 7)),
        Err(RuntimeScrollInputQueueError::RegularQueueFull)
    );
    assert_eq!(queue.terminal_reservations_in_use(), 1);

    accept_begin(&mut queue, first).unwrap();
    assert_eq!(
        queue.enqueue_end(RuntimeScrollGestureTerminal::new(3, 1, 4, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::GestureTerminal {
            kind: RuntimeScrollGestureTerminalKind::End,
            ..
        })
    ));
    assert_eq!(queue.terminal_reservations_in_use(), 0);
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 4, 2, 5, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(queue.terminal_reservations_in_use(), 1);
    assert_ne!(first, second);
}

#[test]
fn pending_events_wait_for_begin_acceptance_and_failed_begin_discards_them() {
    let key = key();
    let active = identity(1, 4, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 1, 4, 7, 0.0, -1.0), 9)
        .unwrap();
    queue
        .enqueue_move(pan_sample(key, 3, 1, 4, 7, 3.0, 0.0), 9)
        .unwrap();
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::GestureBegin(_))
    ));
    assert_eq!(queue.pop_next().unwrap(), None);
    queue
        .complete_begin(active, RuntimeScrollGestureBeginDisposition::Rejected)
        .unwrap();
    assert_eq!(queue.regular_len(), 0);
    assert_eq!(queue.terminal_reservations_in_use(), 0);
    assert_eq!(queue.pop_next().unwrap(), None);
    assert_eq!(
        queue.complete_begin(active, RuntimeScrollGestureBeginDisposition::Rejected),
        Err(RuntimeScrollInputQueueError::UnknownGesture)
    );
}

#[test]
fn adjacent_moves_coalesce_only_with_matching_identity_and_epoch() {
    let key = key();
    let active = identity(3, 8, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 3, 8, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();

    queue
        .enqueue_pan_began(pan_sample(key, 2, 3, 8, 7, 0.0, -2.0), 5)
        .unwrap();
    queue.pop_next().unwrap();
    queue
        .enqueue_move(pan_sample(key, 3, 3, 8, 7, 0.0, -5.0), 5)
        .unwrap();
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 4, 3, 8, 7, 0.0, -12.5), 5),
        Ok(RuntimeScrollInputQueueAdmission::Coalesced {
            represented_samples: 2,
        })
    );

    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::Move {
            sample,
            coalesced_samples: 2,
            ..
        }) if sample.sequence() == 4 && sample.cumulative_y_css_px() == -12.5
    ));
    assert_eq!(queue.diagnostics().coalesced_move_samples(), 1);
}

#[test]
fn move_epoch_and_intervening_event_are_hard_coalescing_barriers() {
    let key = key();
    let styled_key = key_with_style(key, StyleRevision::INITIAL.checked_next().unwrap());
    let active = identity(5, 3, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(4, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 5, 3, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 5, 3, 7, 0.0, -1.0), 2)
        .unwrap();
    queue.pop_next().unwrap();
    queue
        .enqueue_move(pan_sample(key, 3, 5, 3, 7, 0.0, -3.0), 2)
        .unwrap();
    queue
        .enqueue_move(pan_sample(styled_key, 4, 5, 3, 7, 0.0, -6.0), 2)
        .unwrap();
    queue
        .enqueue_wheel(wheel_sample(styled_key, 5, 20.0, 20.0, 0.0, 1.0, 2))
        .unwrap();
    queue
        .enqueue_move(pan_sample(styled_key, 6, 5, 3, 7, 0.0, -9.0), 2)
        .unwrap();
    assert_eq!(queue.regular_len(), 4);
    assert_eq!(queue.diagnostics().coalesced_move_samples(), 0);

    let events = (0..4)
        .map(|_| queue.pop_next().unwrap().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        events
            .iter()
            .map(|event| event.sequence())
            .collect::<Vec<_>>(),
        vec![3, 4, 5, 6]
    );
}

#[test]
fn move_after_terminal_and_terminal_for_another_identity_are_rejected() {
    let key = key();
    let active = identity(8, 6, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 8, 6, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 8, 6, 7, 0.0, -1.0), 0)
        .unwrap();
    queue.pop_next().unwrap();
    assert_eq!(
        queue.enqueue_end(RuntimeScrollGestureTerminal::new(3, 8, 6, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 4, 8, 6, 7, 0.0, -1.0), 0),
        Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued)
    );
    assert_eq!(
        queue.enqueue_cancel(
            RuntimeScrollGestureTerminal::new(5, 9, 6, 7),
            RuntimeScrollGestureCancelReason::HostCancelled,
        ),
        Err(RuntimeScrollInputQueueError::UnknownGesture)
    );
    assert_eq!(queue.terminal_reservations_in_use(), 1);
}

#[test]
fn move_requires_one_pan_begin_for_its_gesture() {
    let key = key();
    let active = identity(11, 12, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 11, 12, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 2, 11, 12, 7, 0.0, -1.0), 1),
        Err(RuntimeScrollInputQueueError::PanNotStarted)
    );
    queue
        .enqueue_pan_began(pan_sample(key, 3, 11, 12, 7, 0.0, -2.0), 1)
        .unwrap();
    assert_eq!(
        queue.enqueue_pan_began(pan_sample(key, 4, 11, 12, 7, 0.0, -3.0), 1),
        Err(RuntimeScrollInputQueueError::PanAlreadyStarted)
    );
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 5, 11, 12, 7, 0.0, -4.0), 1),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
}

#[test]
fn suppressed_begin_keeps_its_terminal_reservation_until_contact_end() {
    let key = key();
    let tracking = identity(1, 4, 7);
    let suppressed = identity(2, 5, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 2).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    queue.enqueue_begin(begin_input(key, 2, 2, 5, 7)).unwrap();
    accept_begin(&mut queue, tracking).unwrap();
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::GestureBegin(_))
    ));
    queue
        .complete_begin(suppressed, RuntimeScrollGestureBeginDisposition::Suppressed)
        .unwrap();
    assert_eq!(queue.terminal_reservations_in_use(), 2);
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(3, 2, 5, 7))
        .unwrap();
    queue
        .enqueue_cancel(
            RuntimeScrollGestureTerminal::new(4, 1, 4, 7),
            RuntimeScrollGestureCancelReason::SecondPointer,
        )
        .unwrap();
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 3);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 4);
    assert_eq!(queue.terminal_reservations_in_use(), 0);
}

#[test]
fn duplicate_or_zero_gesture_identity_does_not_create_an_extra_reservation() {
    let key = key();
    let active = identity(1, 4, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 2).unwrap();
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 1, 0, 4, 7)),
        Err(RuntimeScrollInputQueueError::InvalidGestureIdentity)
    );
    queue.enqueue_begin(begin_input(key, 2, 1, 4, 7)).unwrap();
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 3, 1, 4, 7)),
        Err(RuntimeScrollInputQueueError::DuplicateGestureIdentity)
    );
    assert_eq!(queue.regular_len(), 1);
    assert_eq!(queue.terminal_reservations_in_use(), 1);
    accept_begin(&mut queue, active).unwrap();
}
