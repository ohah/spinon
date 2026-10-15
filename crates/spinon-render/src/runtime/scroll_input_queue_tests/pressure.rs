use super::*;

#[test]
fn move_saturation_uses_reserved_cancel_and_preserves_earlier_sequence_order() {
    let key = key();
    let active = identity(2, 9, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 2, 9, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();

    queue
        .enqueue_pan_began(pan_sample(key, 2, 2, 9, 7, 0.0, -3.0), 1)
        .unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 3, 2.0, 3.0, 0.0, 1.0, 1))
        .unwrap();
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 4, 2, 9, 7, 0.0, -8.0), 1),
        Ok(RuntimeScrollInputQueueAdmission::GestureCancelledForCapacity)
    );
    assert_eq!(queue.regular_len(), 2);
    assert_eq!(queue.terminal_reservations_in_use(), 1);

    let first = queue.pop_next().unwrap().unwrap();
    let second = queue.pop_next().unwrap().unwrap();
    let terminal = queue.pop_next().unwrap().unwrap();
    assert_eq!(
        (first.sequence(), second.sequence(), terminal.sequence()),
        (2, 3, 4)
    );
    assert!(matches!(
        terminal,
        RuntimeScrollInputQueueEvent::GestureTerminal {
            kind: RuntimeScrollGestureTerminalKind::Cancel(
                RuntimeScrollGestureCancelReason::InputQueueSaturated
            ),
            ..
        }
    ));
    assert_eq!(queue.diagnostics().gesture_capacity_cancellations(), 1);
    assert_eq!(queue.terminal_reservations_in_use(), 0);
}

#[test]
fn wheel_capacity_rejects_whole_sample_and_counts_it_without_cancelling_gesture() {
    let key = key();
    let active = identity(1, 3, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 1, 3, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 1, 3, 7, 0.0, -2.0), 4)
        .unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 3, 10.0, 10.0, 2.0, 0.0, 4))
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 4, 11.0, 10.0, 4.0, 0.0, 4)),
        Ok(RuntimeScrollInputQueueAdmission::WheelRejectedForCapacity)
    );
    assert_eq!(queue.regular_len(), 2);
    assert_eq!(queue.terminal_reservations_in_use(), 1);
    assert_eq!(queue.diagnostics().rejected_wheel_samples(), 1);
    assert_eq!(queue.diagnostics().gesture_capacity_cancellations(), 0);
}

#[test]
fn terminal_slot_exhaustion_and_invalid_capacity_leave_no_partial_begin() {
    let key = key();
    assert_eq!(
        RuntimeScrollGestureInputQueue::new(0, 1).err(),
        Some(RuntimeScrollInputQueueError::InvalidCapacity)
    );
    assert_eq!(
        RuntimeScrollGestureInputQueue::new(1, 0).err(),
        Some(RuntimeScrollInputQueueError::InvalidCapacity)
    );
    assert_eq!(
        RuntimeScrollGestureInputQueue::new(usize::MAX, 1).err(),
        Some(RuntimeScrollInputQueueError::InvalidCapacity)
    );

    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 1, 4, 7)).unwrap();
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 2, 2, 5, 7)),
        Err(RuntimeScrollInputQueueError::TerminalSlotsFull)
    );
    assert_eq!(queue.regular_len(), 1);
    assert_eq!(queue.terminal_reservations_in_use(), 1);
    accept_begin(&mut queue, identity(1, 4, 7)).unwrap();
    queue
        .enqueue_cancel(
            RuntimeScrollGestureTerminal::new(3, 1, 4, 7),
            RuntimeScrollGestureCancelReason::HostCancelled,
        )
        .unwrap();
    queue.pop_next().unwrap();
    assert_eq!(queue.terminal_reservations_in_use(), 0);
    assert_eq!(
        queue.enqueue_begin(begin_input(key, 4, 2, 5, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
}

#[test]
fn rejected_wheel_sample_does_not_split_adjacent_accepted_moves() {
    let key = key();
    let active = identity(3, 8, 7);
    let mut queue = RuntimeScrollGestureInputQueue::new(1, 1).unwrap();
    queue.enqueue_begin(begin_input(key, 1, 3, 8, 7)).unwrap();
    accept_begin(&mut queue, active).unwrap();
    queue
        .enqueue_pan_began(pan_sample(key, 2, 3, 8, 7, 0.0, -2.0), 1)
        .unwrap();
    queue.pop_next().unwrap();
    queue
        .enqueue_move(pan_sample(key, 3, 3, 8, 7, 0.0, -3.0), 1)
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 4, 50.0, 50.0, 0.0, 2.0, 1)),
        Ok(RuntimeScrollInputQueueAdmission::WheelRejectedForCapacity)
    );
    assert_eq!(
        queue.enqueue_move(pan_sample(key, 5, 3, 8, 7, 0.0, -5.0), 1),
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
        }) if sample.sequence() == 5 && sample.cumulative_y_css_px() == -5.0
    ));
}
