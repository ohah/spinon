use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind as ContributionKind,
    LayoutGridTrackSizingStageKind as StageKind,
};

use super::cases::{
    A, B, C, Case, MAX_CONTENT_CASES, assert_close, assert_frame, assert_size, compute, fixture,
};
use super::{
    ExpectedContribution, Fixture, assert_axis_tracks, assert_contribution,
    assert_intrinsic_sources, assert_text_lines, stage_bases,
};

#[test]
fn cyclic_minmax_max_content_mixed_text_image_span_matches_fixed_geometry_and_trace() {
    for case in MAX_CONTENT_CASES {
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
        assert_max_content_contributions(&fixture, trace, &case);
        assert_max_content_stages(trace, &case);
    }
}

fn assert_max_content_contributions(
    fixture: &Fixture,
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    case: &Case,
) {
    for (item, min_content, max_content) in [(A, 20.0, 60.0), (B, 40.0, 40.0), (C, 20.0, 110.0)] {
        for (kind, value) in [
            (ContributionKind::Minimum, min_content),
            (ContributionKind::MinContent, min_content),
            (ContributionKind::MaxContent, max_content),
        ] {
            assert_contribution(
                trace,
                fixture,
                ExpectedContribution {
                    item,
                    axis: LayoutGridTrackSizingAxis::Columns,
                    kind,
                    span: item.span,
                    value,
                },
                case.id,
            );
        }
    }
    for (item, contribution) in [(A, 10.0), (B, 80.0 / 3.0), (C, 10.0)] {
        for kind in [
            ContributionKind::Minimum,
            ContributionKind::MinContent,
            ContributionKind::MaxContent,
        ] {
            assert_contribution(
                trace,
                fixture,
                ExpectedContribution {
                    item,
                    axis: LayoutGridTrackSizingAxis::Rows,
                    kind,
                    span: 2,
                    value: contribution,
                },
                case.id,
            );
        }
    }
}

fn assert_max_content_stages(trace: &spinon_layout::LayoutGridTrackSizingTrace, case: &Case) {
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
    for (pass, axis, initial) in [
        (
            &trace.passes[0],
            LayoutGridTrackSizingAxis::Columns,
            &[2.0, 2.0, 2.0][..],
        ),
        (
            &trace.passes[1],
            LayoutGridTrackSizingAxis::Rows,
            &[2.0, 2.0][..],
        ),
    ] {
        assert_eq!(pass.axis, axis, "{}: initial {axis:?} pass", case.id);
        assert_eq!(
            stage_bases(pass, StageKind::Initialize, case.id),
            initial,
            "{}: cyclic percentage initial bases for {axis:?}",
            case.id
        );
    }
    assert_eq!(
        stage_bases(
            &trace.passes[0],
            trace.passes[0].stages.last().unwrap().stage,
            case.id,
        ),
        [24.0, 34.0, 48.0],
        "{}: cyclic max-content columns account for percentage minima and intrinsic items",
        case.id
    );
    for (index, (actual, expected)) in stage_bases(
        &trace.passes[2],
        trace.passes[2].stages.last().unwrap().stage,
        case.id,
    )
    .iter()
    .zip(case.columns)
    .enumerate()
    {
        assert_close(
            *actual,
            expected,
            case.id,
            &format!("resolved max-content column {index}"),
        );
    }
    for base in stage_bases(&trace.passes[3], StageKind::Initialize, case.id) {
        assert_close(
            base,
            14.0 / 3.0,
            case.id,
            "resolved cyclic row minimum basis",
        );
    }
    for base in stage_bases(
        &trace.passes[3],
        trace.passes[3].stages.last().unwrap().stage,
        case.id,
    ) {
        assert_close(
            base,
            37.0 / 3.0,
            case.id,
            "final image-contributed row track",
        );
    }
}
