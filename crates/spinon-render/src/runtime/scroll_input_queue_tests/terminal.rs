use super::*;

#[test]
fn terminal_is_delivered_in_sequence_order_even_when_regular_queue_is_full() {
    let key = key();
    let mut queue = RuntimeScrollGestureInputQueue::new(1, 1).unwrap();
    let active = identity(1, 4, 7);
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 1, 4, 7, 0.0, -4.0), 9)
        .unwrap();
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(3, 1, 4, 7))
        .unwrap();

    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::PanBegan { .. })
    ));
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::GestureTerminal {
            terminal,
            kind: RuntimeScrollGestureTerminalKind::End,
        }) if terminal.sequence() == 3
    ));
    assert_eq!(queue.terminal_reservations_in_use(), 0);
}

#[test]
fn end_waits_behind_a_queued_move_when_regular_capacity_is_full() {
    let key = key();
    let active = identity(4, 6, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 4, 6, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 4, 6, 7, 0.0, -2.0), 1)
        .unwrap();
    queue.pop_next().unwrap();
    queue
        .enqueue_move(pan_sample(key, 3, 4, 6, 7, 0.0, -5.0), 1)
        .unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 4, 30.0, 30.0, 0.0, 1.0, 1))
        .unwrap();
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(5, 4, 6, 7))
        .unwrap();

    let first = queue.pop_next().unwrap().unwrap();
    let second = queue.pop_next().unwrap().unwrap();
    let terminal = queue.pop_next().unwrap().unwrap();
    assert_eq!(
        (first.sequence(), second.sequence(), terminal.sequence()),
        (3, 4, 5)
    );
    assert!(matches!(first, RuntimeScrollInputQueueEvent::Move { .. }));
    assert!(matches!(
        terminal,
        RuntimeScrollInputQueueEvent::GestureTerminal {
            kind: RuntimeScrollGestureTerminalKind::End,
            ..
        }
    ));
}

#[test]
fn terminal_cannot_be_queued_twice_and_sequence_must_increase() {
    let key = key();
    let active = identity(9, 1, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 9, 1, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    let terminal = RuntimeScrollGestureTerminal::new(2, 9, 1, 7);
    queue
        .enqueue_cancel(terminal, RuntimeScrollGestureCancelReason::HostCancelled)
        .unwrap();
    assert_eq!(
        queue.enqueue_end(RuntimeScrollGestureTerminal::new(3, 9, 1, 7)),
        Err(RuntimeScrollInputQueueError::TerminalAlreadyQueued)
    );
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 2, 1.0, 1.0, 0.0, 1.0, 0)),
        Err(RuntimeScrollInputQueueError::NonMonotonicSequence)
    );
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::GestureTerminal {
            kind: RuntimeScrollGestureTerminalKind::Cancel(
                RuntimeScrollGestureCancelReason::HostCancelled
            ),
            ..
        })
    ));
    assert_eq!(queue.terminal_reservations_in_use(), 0);
}

#[test]
fn terminal_slots_from_distinct_gestures_are_consumed_by_sequence() {
    let key = key();
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 2).unwrap();
    let first = identity(1, 4, 7);
    let second = identity(2, 5, 7);
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    queue.enqueue_begin(begin_input(key, 2, 2, 5, 7)).unwrap();
    accept_begin(&mut queue, first).unwrap();
    accept_begin(&mut queue, second).unwrap();
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(3, 1, 4, 7))
        .unwrap();
    queue
        .enqueue_cancel(
            RuntimeScrollGestureTerminal::new(4, 2, 5, 7),
            RuntimeScrollGestureCancelReason::HostCancelled,
        )
        .unwrap();

    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 3);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 4);
    assert_eq!(queue.terminal_reservations_in_use(), 0);
}

#[test]
fn terminal_slots_are_coalescing_barriers_for_other_gesture_and_wheel_streams() {
    let key = key();
    let active = identity(6, 3, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(5, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 6, 3, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 2, 5.0, 6.0, 0.0, 1.0, 0))
        .unwrap();
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(3, 6, 3, 7))
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 4, 5.0, 6.0, 0.0, 2.0, 0)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(queue.regular_len(), 2);
    assert_eq!(queue.diagnostics().coalesced_wheel_samples(), 0);

    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 2);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 3);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 4);
}

#[test]
fn terminal_sequence_between_moves_prevents_reordering_through_coalescing() {
    let key = key();
    let first = identity(1, 4, 7);
    let second = identity(2, 5, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(4, 2).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    queue.enqueue_begin(begin_input(key, 2, 2, 5, 7)).unwrap();
    accept_begin(&mut queue, first).unwrap();
    accept_begin(&mut queue, second).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 3, 1, 4, 7, 0.0, -2.0), 0)
        .unwrap();
    queue.pop_next().unwrap();
    queue
        .enqueue_move(pan_sample(key, 4, 1, 4, 7, 0.0, -4.0), 0)
        .unwrap();
    queue
        .enqueue_end(RuntimeScrollGestureTerminal::new(5, 2, 5, 7))
        .unwrap();
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 6, 1, 4, 7, 0.0, -6.0), 0),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(queue.regular_len(), 2);
    assert_eq!(queue.diagnostics().coalesced_move_samples(), 0);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 4);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 5);
    assert_eq!(queue.pop_next().unwrap().unwrap().sequence(), 6);
}
