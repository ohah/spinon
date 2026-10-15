use super::*;

#[test]
fn wheel_samples_coalesce_only_at_the_same_point_target_revision_and_policy() {
    let key = key();
    let target = NodeId::new(99).unwrap();
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 1, 0.0, 20.0, 1.25, -2.0, 7))
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 2, -0.0, 20.0, 2.5, -4.0, 7)),
        Ok(RuntimeScrollInputQueueAdmission::Coalesced {
            represented_samples: 2,
        })
    );
    queue
        .enqueue_wheel(wheel_sample(key, 3, 1.0, 20.0, 1.0, 0.0, 7))
        .unwrap();

    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::WheelDelta {
            sample,
            coalesced_samples: 2,
        }) if sample.sequence() == 2
            && sample.target() == target
            && sample.delta_x_css_px() == 3.75
            && sample.delta_y_css_px() == -6.0
    ));
    assert_eq!(queue.diagnostics().coalesced_wheel_samples(), 1);
}

#[test]
fn invalid_or_overflowing_wheel_deltas_do_not_mutate_a_queued_sample() {
    let key = key();
    let mut queue = RuntimeScrollGestureInputQueue::new(1, 1).unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 1, 1.0, 2.0, f32::MAX, 0.0, 4))
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 2, 1.0, 2.0, f32::MAX, 0.0, 4)),
        Ok(RuntimeScrollInputQueueAdmission::WheelRejectedForCapacity)
    );
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 3, 1.0, 2.0, f32::NAN, 0.0, 4)),
        Err(RuntimeScrollInputQueueError::InvalidWheelSample)
    );
    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::WheelDelta { sample, .. })
            if sample.sequence() == 1 && sample.delta_x_css_px() == f32::MAX
    ));
    assert_eq!(queue.diagnostics().rejected_wheel_samples(), 1);
}

#[test]
fn overflowing_wheel_coalesce_falls_back_to_two_ordered_samples_when_space_remains() {
    let key = key();
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    queue
        .enqueue_wheel(wheel_sample(key, 1, 1.0, 2.0, f32::MAX, 0.0, 4))
        .unwrap();
    assert_eq!(
        queue.enqueue_wheel(wheel_sample(key, 2, 1.0, 2.0, f32::MAX, 0.0, 4)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
    assert_eq!(queue.regular_len(), 2);
    let first = queue.pop_next().unwrap().unwrap();
    let second = queue.pop_next().unwrap().unwrap();
    assert_eq!((first.sequence(), second.sequence()), (1, 2));
}

#[test]
fn invalid_wheel_scale_raw_delta_and_basis_are_rejected_before_queue_mutation() {
    let key = key();
    let base = WheelSpec {
        point: (5.0, 8.0),
        css_delta: (0.0, 1.0),
        raw_delta: (0.0, 1.0),
        unit: RuntimeScrollWheelUnit::UIKitPoints,
        scale: 2.0,
        target: NodeId::new(99).unwrap(),
        policy_epoch: 1,
        source: RuntimeScrollWheelSource::TrackpadScroll,
        phase: RuntimeScrollMomentumPhase::Change,
        modifiers: 0,
    };
    let mut queue = RuntimeScrollGestureInputQueue::new(2, 1).unwrap();
    for (sequence, invalid) in [
        WheelSpec { scale: 0.0, ..base },
        WheelSpec {
            scale: f32::INFINITY,
            ..base
        },
        WheelSpec {
            raw_delta: (f32::NAN, 1.0),
            ..base
        },
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            queue.enqueue_wheel(wheel_with_spec(key, sequence as u64 + 1, invalid)),
            Err(RuntimeScrollInputQueueError::InvalidWheelSample)
        );
        assert_eq!(queue.regular_len(), 0);
    }
    let next_revision = key
        .scroll_state_revision()
        .checked_next()
        .expect("next scroll revision");
    assert_eq!(
        queue.enqueue_wheel(wheel_with_basis(key, next_revision, 4, base,)),
        Err(RuntimeScrollInputQueueError::InvalidWheelSample)
    );
    assert_eq!(queue.regular_len(), 0);
    assert_eq!(
        queue.enqueue_wheel(wheel_with_spec(key, 5, base)),
        Ok(RuntimeScrollInputQueueAdmission::Queued)
    );
}

#[test]
fn wheel_target_and_scroll_revision_are_coalescing_barriers() {
    let key = key();
    let next_revision = key.scroll_state_revision().checked_next().unwrap();
    let next_key = key.with_scroll_state_revision(next_revision);
    let base = WheelSpec {
        point: (4.0, 6.0),
        css_delta: (0.0, 1.0),
        raw_delta: (0.0, 1.0),
        unit: RuntimeScrollWheelUnit::CssPixels,
        scale: 1.0,
        target: NodeId::new(99).unwrap(),
        policy_epoch: 3,
        source: RuntimeScrollWheelSource::MouseWheel,
        phase: RuntimeScrollMomentumPhase::None,
        modifiers: 0,
    };
    let mut queue = RuntimeScrollGestureInputQueue::new(3, 1).unwrap();
    queue.enqueue_wheel(wheel_with_spec(key, 1, base)).unwrap();
    queue
        .enqueue_wheel(wheel_with_spec(
            key,
            2,
            WheelSpec {
                target: NodeId::new(100).unwrap(),
                ..base
            },
        ))
        .unwrap();
    queue
        .enqueue_wheel(wheel_with_spec(next_key, 3, base))
        .unwrap();
    assert_eq!(queue.regular_len(), 3);
    assert_eq!(queue.diagnostics().coalesced_wheel_samples(), 0);
}

#[test]
fn wheel_coalescing_keeps_raw_units_and_rejects_metadata_barriers() {
    let key = key();
    let target = NodeId::new(99).unwrap();
    let mut queue = RuntimeScrollGestureInputQueue::new(5, 1).unwrap();
    let base = WheelSpec {
        point: (8.0, 13.0),
        css_delta: (1.0, 2.0),
        raw_delta: (2.0, 4.0),
        unit: RuntimeScrollWheelUnit::UIKitPoints,
        scale: 2.0,
        target,
        policy_epoch: 4,
        source: RuntimeScrollWheelSource::TrackpadScroll,
        phase: RuntimeScrollMomentumPhase::Change,
        modifiers: 1,
    };
    queue.enqueue_wheel(wheel_with_spec(key, 1, base)).unwrap();
    let mut matching = base;
    matching.css_delta = (3.0, 4.0);
    matching.raw_delta = (6.0, 8.0);
    assert_eq!(
        queue.enqueue_wheel(wheel_with_spec(key, 2, matching)),
        Ok(RuntimeScrollInputQueueAdmission::Coalesced {
            represented_samples: 2,
        })
    );

    let barriers = [
        WheelSpec {
            unit: RuntimeScrollWheelUnit::AndroidPhysicalPixels,
            ..base
        },
        WheelSpec { scale: 3.0, ..base },
        WheelSpec {
            source: RuntimeScrollWheelSource::MouseWheel,
            ..base
        },
    ];
    for (index, barrier) in barriers.into_iter().enumerate() {
        queue
            .enqueue_wheel(wheel_with_spec(key, 3 + index as u64, barrier))
            .unwrap();
    }
    assert_eq!(queue.regular_len(), 4);
    assert_eq!(queue.diagnostics().coalesced_wheel_samples(), 1);

    assert!(matches!(
        queue.pop_next().unwrap(),
        Some(RuntimeScrollInputQueueEvent::WheelDelta { sample, .. })
            if sample.sequence() == 2
                && sample.raw_delta_x() == 8.0
                && sample.raw_delta_y() == 12.0
                && sample.source_unit() == RuntimeScrollWheelUnit::UIKitPoints
                && sample.coordinate_scale() == 2.0
                && sample.modifier_flags() == 1
    ));
}
