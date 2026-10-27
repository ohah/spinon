use super::{LayoutGridTrackSizingStageKind, SpanStageExpectation, StageSnapshotExpectation};

fn unique_stage<'a>(
    pass: &'a crate::LayoutGridTrackSizingPassTrace,
    predicate: impl Fn(&LayoutGridTrackSizingStageKind) -> bool,
    case: &str,
    label: &str,
) -> (&'a crate::LayoutGridTrackSizingStageTrace, usize) {
    let matching = pass
        .stages
        .iter()
        .enumerate()
        .filter(|(_, stage)| predicate(&stage.stage))
        .collect::<Vec<_>>();
    assert_eq!(matching.len(), 1, "{case}: exactly one {label} stage");
    let (index, stage) = matching[0];
    (stage, index)
}

fn assert_stage_snapshot(
    stage: &crate::LayoutGridTrackSizingStageTrace,
    expected_base_sizes: &[f32],
    expected_growth_limits: &[Option<f32>],
    case: &str,
    label: &str,
) {
    assert_eq!(
        expected_base_sizes.len(),
        expected_growth_limits.len(),
        "{case}: {label} expected vector lengths"
    );
    assert_eq!(
        stage.tracks.len(),
        expected_base_sizes.len() * 2 + 1,
        "{case}: {label} should alternate gutters and CSS tracks with boundary gutters"
    );
    for (index, track) in stage.tracks.iter().enumerate() {
        let is_boundary_gutter = (index == 0 || index + 1 == stage.tracks.len()) && index % 2 == 0;
        assert_eq!(
            track.is_gutter,
            index % 2 == 0,
            "{case}: {label} track entry {index} gutter pattern"
        );
        assert_eq!(
            track.is_collapsed, is_boundary_gutter,
            "{case}: {label} track entry {index} collapsed flag"
        );
    }
    let css_tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        css_tracks.len(),
        expected_base_sizes.len(),
        "{case}: {label} CSS track count"
    );
    for (index, (track, expected_base)) in css_tracks.iter().zip(expected_base_sizes).enumerate() {
        assert!(
            track.base_size.is_finite(),
            "{case}: {label} base_sizes[{index}] is not finite: {}",
            track.base_size
        );
        assert!(
            (track.base_size - expected_base).abs() <= 1.0e-4,
            "{case}: {label} base_sizes[{index}] expected {expected_base}, got {}",
            track.base_size
        );
        match (expected_growth_limits[index], track.growth_limit) {
            // Taffy's trace serializer encodes every non-finite limit as None. This
            // checks that projection only; it does not distinguish +infinity from NaN/-infinity.
            (None, None) => {}
            (Some(expected), Some(actual)) => {
                assert!(
                    actual.is_finite(),
                    "{case}: {label} growth_limits[{index}] is not finite: {actual}"
                );
                assert!(
                    (actual - expected).abs() <= 1.0e-4,
                    "{case}: {label} growth_limits[{index}] expected {expected}, got {actual}"
                );
            }
            (expected, actual) => panic!(
                "{case}: {label} growth_limits[{index}] expected {expected:?}, got {actual:?}"
            ),
        }
    }
    for (index, gutter) in stage
        .tracks
        .iter()
        .filter(|track| track.is_gutter)
        .enumerate()
    {
        assert!(
            gutter.base_size.is_finite() && gutter.base_size.abs() <= 1.0e-4,
            "{case}: {label} zero-gap gutter {index} expected 0, got {}",
            gutter.base_size
        );
    }
}

fn fixed_span_stage_expectation(span_batches: &[u16]) -> SpanStageExpectation {
    let zeros = [0.0; 4];
    let intrinsic_batches = span_batches
        .iter()
        .map(|span| {
            let growth_limits = match *span {
                2 => [Some(40.0), Some(60.0), Some(60.0), None],
                3 => [
                    Some(160.0 / 3.0),
                    Some(220.0 / 3.0),
                    Some(220.0 / 3.0),
                    None,
                ],
                _ => panic!("unexpected batch span {span}"),
            };
            StageSnapshotExpectation {
                base_sizes: zeros,
                growth_limits,
            }
        })
        .collect::<Vec<_>>();
    let (final_sizes, remaining_free_space_px) = match span_batches {
        [2] => ([40.0, 60.0, 60.0, 0.0], 60.0),
        [2, 3] => ([160.0 / 3.0, 220.0 / 3.0, 220.0 / 3.0, 0.0], 20.0),
        _ => panic!("unexpected span batch sequence: {span_batches:?}"),
    };
    let final_growth_limits = final_sizes.map(Some);

    SpanStageExpectation {
        span_batches: span_batches.to_vec(),
        initialize: StageSnapshotExpectation {
            base_sizes: zeros,
            growth_limits: [None; 4],
        },
        intrinsic_batches,
        intrinsic_sizing_complete: StageSnapshotExpectation {
            base_sizes: zeros,
            growth_limits: final_growth_limits,
        },
        maximize_tracks: StageSnapshotExpectation {
            base_sizes: final_sizes,
            growth_limits: final_growth_limits,
        },
        available_block_size_px: 220.0,
        remaining_free_space_px,
    }
}

pub(super) fn assert_span_stage_snapshots(
    pass: &crate::LayoutGridTrackSizingPassTrace,
    case_id: &str,
    expected_span_batches: &[u16],
) {
    let expected = fixed_span_stage_expectation(expected_span_batches);
    assert_span_stage_snapshots_with_expectation(pass, case_id, &expected);
}

pub(super) fn assert_span_stage_snapshots_with_expectation(
    pass: &crate::LayoutGridTrackSizingPassTrace,
    case_id: &str,
    expected: &SpanStageExpectation,
) {
    assert_eq!(
        expected.intrinsic_batches.len(),
        expected.span_batches.len(),
        "{case_id}: expected one snapshot per normative span batch"
    );
    let mut expected_stages = vec![LayoutGridTrackSizingStageKind::Initialize];
    expected_stages.extend(expected.span_batches.iter().map(|span| {
        LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
            span: *span,
            crosses_flexible_track: false,
        }
    }));
    expected_stages.extend([
        LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        LayoutGridTrackSizingStageKind::MaximizeTracks,
        LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: false },
    ]);
    assert_eq!(
        pass.stages
            .iter()
            .map(|stage| stage.stage)
            .collect::<Vec<_>>(),
        expected_stages,
        "{case_id}: complete stage sequence"
    );

    let (initialize, initialize_index) = unique_stage(
        pass,
        |kind| *kind == LayoutGridTrackSizingStageKind::Initialize,
        case_id,
        "Initialize",
    );
    assert_stage_snapshot(
        initialize,
        &expected.initialize.base_sizes,
        &expected.initialize.growth_limits,
        case_id,
        "Initialize",
    );

    let mut ordered_indices = vec![initialize_index];
    for (span, snapshot) in expected
        .span_batches
        .iter()
        .zip(&expected.intrinsic_batches)
    {
        let (batch, index) = unique_stage(
            pass,
            |kind| {
                *kind
                    == LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                        span: *span,
                        crosses_flexible_track: false,
                    }
            },
            case_id,
            &format!("non-flex span-{span} batch"),
        );
        ordered_indices.push(index);
        assert_stage_snapshot(
            batch,
            &snapshot.base_sizes,
            &snapshot.growth_limits,
            case_id,
            &format!("span-{span} batch"),
        );
    }

    let (complete, complete_index) = unique_stage(
        pass,
        |kind| *kind == LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        case_id,
        "IntrinsicSizingComplete",
    );
    let (maximize, maximize_index) = unique_stage(
        pass,
        |kind| *kind == LayoutGridTrackSizingStageKind::MaximizeTracks,
        case_id,
        "MaximizeTracks",
    );
    ordered_indices.extend([complete_index, maximize_index]);
    assert!(
        ordered_indices
            .windows(2)
            .all(|indices| indices[0] < indices[1]),
        "{case_id}: Initialize, span batches, IntrinsicSizingComplete, and MaximizeTracks are out of order",
    );

    assert_stage_snapshot(
        complete,
        &expected.intrinsic_sizing_complete.base_sizes,
        &expected.intrinsic_sizing_complete.growth_limits,
        case_id,
        "IntrinsicSizingComplete",
    );
    assert_stage_snapshot(
        maximize,
        &expected.maximize_tracks.base_sizes,
        &expected.maximize_tracks.growth_limits,
        case_id,
        "MaximizeTracks",
    );
    let observed_remaining_space = expected.available_block_size_px
        - maximize
            .tracks
            .iter()
            .map(|track| track.base_size)
            .sum::<f32>();
    assert!(
        observed_remaining_space.is_finite()
            && (observed_remaining_space - expected.remaining_free_space_px).abs() <= 1.0e-4,
        "{case_id}: MaximizeTracks remaining free space expected {}, got {observed_remaining_space}",
        expected.remaining_free_space_px
    );
}
