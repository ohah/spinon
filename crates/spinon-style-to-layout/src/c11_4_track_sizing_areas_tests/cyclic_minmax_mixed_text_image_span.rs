use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind as ContributionKind,
    LayoutGridTrackSizingStageKind as StageKind,
};

use super::Fixture;
#[path = "cyclic_minmax_mixed_text_image_span/cases.rs"]
mod cases;
use cases::{
    A, B, C, CASES, Case, Item, assert_close, assert_frame, assert_size, compute,
    compute_without_fixture_alignment_opt_in, fixture,
};

struct ExpectedContribution {
    item: Item,
    axis: LayoutGridTrackSizingAxis,
    kind: ContributionKind,
    span: u16,
    value: f32,
}

#[test]
fn cyclic_minmax_mixed_text_image_span_matches_fixed_geometry_and_trace() {
    for case in CASES {
        let fixture = fixture(&case);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        assert_size(
            [root.width, root.height],
            case.root,
            case.id,
            "root border box",
        );

        for item in [A, B, C] {
            let frame = output.layout.frames[&fixture.nodes[item.id].id()];
            assert_frame(
                [frame.x, frame.y, frame.width, frame.height],
                case.frames[item.index],
                case.id,
                item.key,
            );
        }

        assert_text_lines(&output, &fixture, &case);
        assert_intrinsic_sources(&output, &fixture, &case);
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: Grid trace missing", case.id));
        assert_axis_tracks(
            trace,
            LayoutGridTrackSizingAxis::Columns,
            &case.columns,
            case.id,
        );
        assert_axis_tracks(trace, LayoutGridTrackSizingAxis::Rows, &case.rows, case.id);
        assert_contributions(&fixture, trace, &case);
        if case.intrinsic {
            assert_intrinsic_stages(trace, &fixture, &case);
        } else {
            assert_definite_stages(trace, &case);
        }
    }
}

#[test]
fn replaced_image_start_alignment_requires_the_fixed_fixture_projection() {
    let fixture = fixture(&CASES[2]);
    let error = compute_without_fixture_alignment_opt_in(&fixture).unwrap_err();
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            property: "align-self",
            value,
            ..
        } if value == "start"
    ));
}

fn assert_text_lines(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    for item in [A, C] {
        let text = output
            .layout
            .text_layouts
            .get(&fixture.nodes[item.id].id())
            .unwrap_or_else(|| panic!("{}: {} text layout missing", case.id, item.key));
        let expected = case.lines[if item.key == "a" { 0 } else { 1 }];
        assert_eq!(
            text.layout.lines.len(),
            expected.len(),
            "{}: {} line count",
            case.id,
            item.key
        );
        for (line, width) in text.layout.lines.iter().zip(expected) {
            assert_close(
                line.advance,
                *width,
                case.id,
                &format!("{} line width", item.key),
            );
        }
    }
}

fn assert_intrinsic_sources(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    assert_eq!(
        output.layout.grid_item_intrinsic_inputs.len(),
        3,
        "{}: complete item set",
        case.id
    );
    for item in [A, B, C] {
        let input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.nodes[item.id].id())
            .unwrap_or_else(|| panic!("{}: {} intrinsic input missing", case.id, item.key));
        assert_eq!(
            input.source_order,
            case.order
                .iter()
                .position(|candidate| candidate.id == item.id)
                .unwrap(),
            "{}: {} source order",
            case.id,
            item.key
        );
        match (item.key, &input.source) {
            ("b", LayoutGridItemIntrinsicSource::Replaced { natural }) => {
                assert_close(natural.width.unwrap(), 24.0, case.id, "image natural width");
                assert_close(
                    natural.height.unwrap(),
                    16.0,
                    case.id,
                    "image natural height",
                );
                assert_close(
                    natural.natural_aspect_ratio().unwrap(),
                    1.5,
                    case.id,
                    "image ratio",
                );
            }
            ("a" | "c", LayoutGridItemIntrinsicSource::Text { sizes }) => {
                assert_close(
                    sizes.min_content(),
                    20.0,
                    case.id,
                    &format!("{} min-content", item.key),
                );
                let max = if item.key == "a" { 60.0 } else { 110.0 };
                assert_close(
                    sizes.max_content(),
                    max,
                    case.id,
                    &format!("{} max-content", item.key),
                );
            }
            _ => panic!(
                "{}: {} intrinsic source kind changed: {:?}",
                case.id, item.key, input.source
            ),
        }
    }
}

fn assert_contributions(
    fixture: &Fixture,
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    case: &Case,
) {
    let expected_inline = [(A, 20.0), (B, 40.0), (C, 20.0)];
    for (item, expected) in expected_inline {
        assert_contribution(
            trace,
            fixture,
            ExpectedContribution {
                item,
                axis: LayoutGridTrackSizingAxis::Columns,
                kind: ContributionKind::MinContent,
                span: item.span,
                value: expected,
            },
            case.id,
        );
    }
    assert_contribution(
        trace,
        fixture,
        ExpectedContribution {
            item: B,
            axis: LayoutGridTrackSizingAxis::Columns,
            kind: ContributionKind::Minimum,
            span: 2,
            value: 40.0,
        },
        case.id,
    );
    assert_contribution(
        trace,
        fixture,
        ExpectedContribution {
            item: B,
            axis: LayoutGridTrackSizingAxis::Rows,
            kind: ContributionKind::Minimum,
            span: 2,
            value: 80.0 / 3.0,
        },
        case.id,
    );
    assert_contribution(
        trace,
        fixture,
        ExpectedContribution {
            item: B,
            axis: LayoutGridTrackSizingAxis::Rows,
            kind: ContributionKind::MinContent,
            span: 2,
            value: 80.0 / 3.0,
        },
        case.id,
    );
    for (item, value) in [(A, case.line_heights[0]), (C, case.line_heights[1])] {
        assert_contribution(
            trace,
            fixture,
            ExpectedContribution {
                item,
                axis: LayoutGridTrackSizingAxis::Rows,
                kind: ContributionKind::MinContent,
                span: 2,
                value,
            },
            case.id,
        );
    }
}

fn assert_contribution(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    fixture: &Fixture,
    expected: ExpectedContribution,
    scene: &str,
) {
    assert!(
        trace
            .passes
            .iter()
            .filter(|pass| pass.axis == expected.axis)
            .any(|pass| {
                pass.contributions.iter().any(|entry| {
                    entry.item == fixture.nodes[expected.item.id].id()
                        && entry.kind == expected.kind
                        && entry.span == expected.span
                        && (entry.value - expected.value).abs() <= 1.0 / 64.0
                })
            }),
        "{scene}: {} {:?} {:?} contribution {} missing",
        expected.item.key,
        expected.axis,
        expected.kind,
        expected.value
    );
}

fn assert_intrinsic_stages(
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
        "{}: cyclic and resolved axis order",
        case.id
    );
    let columns = &trace.passes[0];
    assert_eq!(
        intrinsic_spans(columns),
        [2, 3],
        "{}: span groups are ordered",
        case.id
    );
    assert_eq!(
        stage_bases(columns, StageKind::Initialize, case.id),
        [2.0, 2.0, 2.0]
    );
    assert_eq!(
        stage_bases_for_span(columns, 2, case.id),
        [9.0, 19.0, 19.0],
        "{}: text/image maximum planned increase",
        case.id
    );
    assert_eq!(
        stage_bases_for_span(columns, 3, case.id),
        [9.0, 19.0, 19.0],
        "{}: C adds no further growth",
        case.id
    );
    assert_eq!(intrinsic_spans(&trace.passes[1]), [2]);
    assert_eq!(
        stage_bases(&trace.passes[1], StageKind::Initialize, case.id),
        [2.0, 2.0]
    );
    assert_close(
        stage_bases_for_span(&trace.passes[1], 2, case.id)[0],
        37.0 / 3.0,
        case.id,
        "first intrinsic row",
    );
    assert_close(
        stage_bases_for_span(&trace.passes[1], 2, case.id)[1],
        37.0 / 3.0,
        case.id,
        "second intrinsic row",
    );
    for base in stage_bases(&trace.passes[2], StageKind::Initialize, case.id) {
        assert_close(base, 7.1, case.id, "resolved column minimum basis");
    }
    for base in stage_bases(&trace.passes[3], StageKind::Initialize, case.id) {
        assert_close(base, 14.0 / 3.0, case.id, "resolved row minimum basis");
    }
    let span3 = trace.passes[0]
        .contributions
        .iter()
        .find(|entry| {
            entry.item == fixture.nodes[C.id].id()
                && entry.kind == ContributionKind::MinContent
                && entry.span == 3
        })
        .unwrap();
    assert_close(
        span3.value,
        20.0,
        case.id,
        "C min-content span-3 contribution",
    );
}

fn assert_definite_stages(trace: &spinon_layout::LayoutGridTrackSizingTrace, case: &Case) {
    assert_eq!(
        trace
            .passes
            .iter()
            .map(|pass| pass.axis)
            .collect::<Vec<_>>(),
        [
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingAxis::Rows,
        ],
        "{}: definite width uses Columns then cyclic-auto and resolved Rows passes",
        case.id
    );
    assert_eq!(intrinsic_spans(&trace.passes[0]), [2, 3]);
    assert_eq!(intrinsic_spans(&trace.passes[1]), [2]);
    assert_eq!(intrinsic_spans(&trace.passes[2]), [2]);
    assert_eq!(
        stage_bases(&trace.passes[0], StageKind::Initialize, case.id),
        [6.0, 6.0, 6.0]
    );
    assert_eq!(
        stage_bases(&trace.passes[1], StageKind::Initialize, case.id),
        [2.0, 2.0]
    );
    assert_eq!(
        stage_bases(&trace.passes[2], StageKind::Initialize, case.id),
        [6.0, 6.0]
    );
    assert_eq!(
        stage_bases(
            &trace.passes[0],
            trace.passes[0].stages.last().unwrap().stage,
            case.id
        ),
        case.columns
    );
    assert_eq!(
        stage_bases(
            &trace.passes[1],
            trace.passes[1].stages.last().unwrap().stage,
            case.id
        ),
        case.rows
    );
    assert_eq!(
        stage_bases(
            &trace.passes[2],
            trace.passes[2].stages.last().unwrap().stage,
            case.id
        ),
        case.rows
    );
}

fn assert_axis_tracks(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let pass = trace
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == axis)
        .unwrap();
    let actual = stage_bases(pass, pass.stages.last().unwrap().stage, scene);
    assert_eq!(
        actual.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(
            *actual,
            *expected,
            scene,
            &format!("{axis:?} track {index}"),
        );
    }
}

fn intrinsic_spans(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> Vec<u16> {
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
    stage: StageKind,
    scene: &str,
) -> Vec<f32> {
    pass.stages
        .iter()
        .find(|entry| entry.stage == stage)
        .unwrap_or_else(|| panic!("{scene}: missing {stage:?} stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn stage_bases_for_span(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    span: u16,
    scene: &str,
) -> Vec<f32> {
    let stage = pass.stages.iter().find(|entry| matches!(
        entry.stage, StageKind::IntrinsicItemBatch { span: candidate, .. } if candidate == span
    )).unwrap_or_else(|| panic!("{scene}: missing span-{span} stage"));
    stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}
