use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind as ContributionKind,
    LayoutGridTrackSizingStageKind as StageKind, LayoutIntrinsicMeasurementSource,
};

use super::super::Fixture;
use super::{A, B, C, Case, Item};

pub(super) fn assert_replaced_intrinsic_inputs(
    output: &crate::StyleLayoutOutput,
    fixture: &Fixture,
    case: &Case,
) {
    for item in [A, B, C] {
        let input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.nodes[item.id].id())
            .unwrap_or_else(|| panic!("{}: {} intrinsic input missing", case.id, item.id));
        let LayoutGridItemIntrinsicSource::Replaced { natural } = &input.source else {
            panic!(
                "{}: {} did not use replaced intrinsic size",
                case.id, item.id
            );
        };
        let expected_source_order = case
            .order
            .iter()
            .position(|ordered| ordered.id == item.id)
            .expect("image identity belongs to scene");
        assert_eq!(
            input.source_order, expected_source_order,
            "{}: {} DOM source order",
            case.id, item.id
        );
        assert_close(
            natural.width.expect("natural image width"),
            24.0,
            case.id,
            "natural image width",
        );
        assert_close(
            natural.height.expect("natural image height"),
            16.0,
            case.id,
            "natural image height",
        );
        assert_close(
            natural
                .natural_aspect_ratio()
                .expect("natural image aspect ratio"),
            1.5,
            case.id,
            "natural image aspect ratio",
        );
    }
}

pub(super) fn assert_replaced_contributions(
    output: &crate::StyleLayoutOutput,
    fixture: &Fixture,
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    scene: &str,
) {
    let measurements = &output.layout.intrinsic_measurements;
    for item in [A, B, C] {
        assert!(
            measurements.iter().any(|measurement| {
                measurement.node == fixture.nodes[item.id].id()
                    && matches!(&measurement.source, LayoutIntrinsicMeasurementSource::Replaced { natural }
                        if natural.width.is_some_and(|width| (width - 24.0).abs() <= 1.0 / 64.0)
                            && natural.height.is_some_and(|height| (height - 16.0).abs() <= 1.0 / 64.0))
            }),
            "{scene}: {} replaced measurement missing",
            item.id
        );
        for (axis, expected, span) in [
            (LayoutGridTrackSizingAxis::Columns, item.inline, item.span),
            (LayoutGridTrackSizingAxis::Rows, item.height, 2),
        ] {
            let observed = trace
                .passes
                .iter()
                .filter(|pass| pass.axis == axis)
                .flat_map(|pass| pass.contributions.iter())
                .filter(|contribution| contribution.item == fixture.nodes[item.id].id())
                .map(|contribution| (contribution.kind, contribution.span, contribution.value))
                .collect::<Vec<_>>();
            let passes = trace.passes.iter().filter(|pass| pass.axis == axis);
            let kinds: &[ContributionKind] = if scene.starts_with("intrinsic") {
                &[
                    ContributionKind::Minimum,
                    ContributionKind::MinContent,
                    ContributionKind::MaxContent,
                ]
            } else {
                &[ContributionKind::MinContent, ContributionKind::MaxContent]
            };
            for kind in kinds {
                assert!(
                    passes
                        .clone()
                        .any(|pass| pass.contributions.iter().any(|contribution| {
                            contribution.item == fixture.nodes[item.id].id()
                                && contribution.kind == *kind
                                && contribution.span == span
                                && (contribution.value - expected).abs() <= 1.0 / 64.0
                        })),
                    "{scene}: {axis:?} {} {kind:?} contribution {expected} on span {span} missing; observed: {observed:?}",
                    item.id,
                );
            }
        }
    }
}

pub(super) fn assert_intrinsic_span_stages(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    fixture: &Fixture,
    case: &Case,
) {
    assert_eq!(
        trace
            .passes
            .iter()
            .map(|pass| pass.axis)
            .collect::<Vec<_>>(),
        [
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingAxis::Rows,
        ],
        "{}: cyclic-auto columns and rows must precede resolved columns and rows",
        case.id
    );
    let columns = &trace.passes[0];
    let rows = &trace.passes[1];
    assert_eq!(
        intrinsic_batch_spans(columns),
        [2, 3],
        "{}: span-2 must be processed before span-3",
        case.id
    );
    assert_eq!(
        intrinsic_batch_spans(rows),
        [2],
        "{}: all row items must share one span-2 batch",
        case.id
    );
    assert_eq!(
        stage_bases(columns, StageKind::Initialize, case.id),
        [2.0, 2.0, 2.0],
        "{}: cyclic percentage columns act as auto",
        case.id
    );
    assert_eq!(
        stage_bases_for_span(columns, 2, case.id),
        [5.0, 8.0, 8.0],
        "{}: span-2 A/B per-track maximum",
        case.id
    );
    assert_eq!(
        stage_bases_for_span(columns, 2, case.id)
            .iter()
            .zip(stage_bases(columns, StageKind::Initialize, case.id))
            .map(|(after, before)| after - before)
            .collect::<Vec<_>>(),
        [3.0, 6.0, 6.0],
        "{}: span-2 planned increases combine by per-track maximum",
        case.id
    );
    assert_eq!(
        stage_bases_for_span(columns, 3, case.id),
        [5.0, 8.0, 8.0],
        "{}: span-3 C must not grow the established columns",
        case.id
    );
    assert_eq!(
        stage_bases(rows, StageKind::Initialize, case.id),
        [2.0, 2.0],
        "{}: cyclic percentage rows act as auto",
        case.id
    );
    assert_eq!(
        stage_bases_for_span(rows, 2, case.id),
        [7.0, 7.0],
        "{}: row span-2 per-track maximum",
        case.id
    );
    let minimum_for = |item: Item, span| {
        columns
            .contributions
            .iter()
            .find(|contribution| {
                contribution.item == fixture.nodes[item.id].id()
                    && contribution.kind == ContributionKind::Minimum
                    && contribution.span == span
            })
            .unwrap_or_else(|| panic!("{}: {} Minimum contribution missing", case.id, item.id))
            .value
    };
    let a_share = (minimum_for(A, 2) - 2.0 - 2.0 - 2.0) / 2.0;
    let b_share = (minimum_for(B, 2) - 2.0 - 2.0 - 2.0) / 2.0;
    assert_close(a_share, 3.0, case.id, "A planned increase per track");
    assert_close(b_share, 6.0, case.id, "B planned increase per track");
    for (item, expected_share) in [(A, 1.0), (B, 3.0), (C, 5.0)] {
        let minimum = rows
            .contributions
            .iter()
            .find(|contribution| {
                contribution.item == fixture.nodes[item.id].id()
                    && contribution.kind == ContributionKind::Minimum
                    && contribution.span == 2
            })
            .unwrap_or_else(|| panic!("{}: {} row Minimum contribution missing", case.id, item.id))
            .value;
        assert_close(
            (minimum - 2.0 - 2.0 - 2.0) / 2.0,
            expected_share,
            case.id,
            &format!("{} row planned increase per track", item.id),
        );
    }
    assert_eq!(
        minimum_for(C, 3),
        24.0,
        "{}: C natural width contribution",
        case.id
    );
    assert!(
        [5.0, 8.0, 8.0].iter().sum::<f32>() + 4.0 >= minimum_for(C, 3),
        "{}: span-3 C exceeds established track and gutter area",
        case.id
    );
    let resolved_columns = &trace.passes[2];
    let resolved_rows = &trace.passes[3];
    assert_eq!(
        stage_bases(resolved_columns, StageKind::Initialize, case.id),
        [4.5, 4.5, 4.5],
        "{}: resolved column minimum basis uses final 25px content size",
        case.id
    );
    assert_eq!(
        stage_bases(resolved_rows, StageKind::Initialize, case.id),
        [3.6, 3.6],
        "{}: resolved row minimum basis uses final 16px content size",
        case.id
    );
}

pub(super) fn assert_definite_control_stages(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    case: &Case,
) {
    assert_eq!(
        trace
            .passes
            .iter()
            .map(|pass| pass.axis)
            .collect::<Vec<_>>(),
        [
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingAxis::Rows
        ],
        "{}: definite control uses one Columns and Rows pass",
        case.id
    );
    assert_eq!(
        stage_bases(&trace.passes[0], StageKind::Initialize, case.id),
        [10.0, 10.0, 10.0],
        "{}: resolved definite column minimum",
        case.id
    );
    assert_eq!(
        stage_bases(&trace.passes[1], StageKind::Initialize, case.id),
        [10.0, 10.0],
        "{}: resolved definite row minimum",
        case.id
    );
    assert_eq!(
        intrinsic_batch_spans(&trace.passes[0]),
        [2, 3],
        "{}: definite column contribution spans",
        case.id
    );
    assert_eq!(
        intrinsic_batch_spans(&trace.passes[1]),
        [2],
        "{}: definite row contribution spans",
        case.id
    );
    assert_eq!(
        stage_bases(
            &trace.passes[0],
            trace.passes[0].stages.last().unwrap().stage,
            case.id
        ),
        case.columns,
        "{}: definite columns remain at their resolved minimum",
        case.id
    );
    assert_eq!(
        stage_bases(
            &trace.passes[1],
            trace.passes[1].stages.last().unwrap().stage,
            case.id
        ),
        case.rows,
        "{}: definite rows remain at their resolved minimum",
        case.id
    );
}

fn intrinsic_batch_spans(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> Vec<u16> {
    pass.stages
        .iter()
        .filter_map(|stage| match stage.stage {
            StageKind::IntrinsicItemBatch { span, .. } => Some(span),
            _ => None,
        })
        .collect()
}

fn stage_bases(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    kind: StageKind,
    scene: &str,
) -> Vec<f32> {
    pass.stages
        .iter()
        .find(|stage| stage.stage == kind)
        .unwrap_or_else(|| panic!("{scene}: missing {kind:?} stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn stage_bases_for_span(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    expected_span: u16,
    scene: &str,
) -> Vec<f32> {
    let stage = pass
        .stages
        .iter()
        .find(|stage| {
            matches!(stage.stage, StageKind::IntrinsicItemBatch { span, .. } if span == expected_span)
        })
        .unwrap_or_else(|| panic!("{scene}: missing span-{expected_span} batch"));
    stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

pub(super) fn assert_axis_tracks(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let final_stage = pass.stages.last().expect("final track sizing stage");
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (actual, expected) in tracks.iter().zip(expected) {
        assert_close(*actual, *expected, scene, "track base size");
    }
}

pub(super) fn assert_frame<const N: usize>(
    actual: [f32; N],
    expected: [f32; N],
    scene: &str,
    label: &str,
) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert_close(*actual, expected, scene, label);
    }
}

pub(super) fn assert_close(actual: f32, expected: f32, scene: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{scene}: {label} expected {expected}, got {actual}"
    );
}
