use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind as ContributionKind, LayoutGridTrackSizingStageKind,
};

#[path = "cyclic_minmax_calc_cross_axis_mixed_span_groups/cases.rs"]
mod cases;
use self::cases::{A, B, C, CASES, Case, ITEMS, Item, compute, fixture};
use super::Fixture;

#[test]
fn cyclic_minmax_calc_mixed_span_groups_match_fixed_geometry_and_trace() {
    for case in &CASES {
        for (index, item) in ITEMS.iter().enumerate() {
            assert_close(
                case.block_contributions[index],
                case.line_counts[index] as f32 * 10.0,
                case.id,
                &format!("{} Ahem line-height contribution", item.key),
            );
        }
        let fixture = fixture(case);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        assert_frame(
            [root.width, root.height],
            case.root,
            case.id,
            "root border box",
        );
        for (index, item) in ITEMS.iter().enumerate() {
            let frame = output.layout.frames[&fixture.nodes[item.id].id()];
            assert_frame(
                [frame.x, frame.y, frame.width, frame.height],
                case.item_frames[index],
                case.id,
                &format!("item {} frame", item.key),
            );
        }
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            &case.columns,
            case.id,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            &case.rows,
            case.id,
        );
        assert_intrinsic_sources(&output, &fixture, case);
        assert_contributions(&output, &fixture, case);
        if case.width == "min-content" {
            assert_span_ordered_column_growth(&output, &fixture, case.id);
        }
    }
}

fn assert_intrinsic_sources(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    let expected = [(20.0, 60.0, 30.0), (40.0, 40.0, 10.0), (20.0, 110.0, 40.0)];
    for (index, item) in ITEMS.iter().enumerate() {
        let input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.nodes[item.id].id())
            .unwrap_or_else(|| panic!("{}: missing intrinsic input for {}", case.id, item.key));
        let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
            panic!(
                "{}: {} must use direct text intrinsic input",
                case.id, item.key
            );
        };
        let (min_content, max_content, min_height) = expected[index];
        for (label, actual, value) in [
            ("min-content width", sizes.min_content(), min_content),
            ("max-content width", sizes.max_content(), max_content),
            ("min-content height", sizes.min_content_height(), min_height),
            ("max-content height", sizes.max_content_height(), 10.0),
        ] {
            assert_close(actual, value, case.id, &format!("{} {label}", item.key));
        }
        let expected_order = case
            .order
            .iter()
            .position(|ordered| ordered.id == item.id)
            .unwrap();
        assert_eq!(
            input.source_order, expected_order,
            "{}: {} source order",
            case.id, item.key
        );
        assert!(
            sizes.font_registry_revision().0 > 0,
            "{}: {} Ahem revision",
            case.id,
            item.key
        );
    }
}

fn assert_contributions(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track-sizing trace", case.id));
    let last_columns = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap();
    let last_rows = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap();
    assert!(
        last_columns < last_rows,
        "{}: Columns must size before Rows",
        case.id
    );
    let expected_spans = [2, 2, 3];
    let expected_inline_min_content = [20.0, 40.0, 20.0];
    let expected_inline_max_content = [60.0, 40.0, 110.0];
    for (axis, minima, min_contents, max_contents, spans) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            case.minimum_inline,
            expected_inline_min_content,
            expected_inline_max_content,
            expected_spans,
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            case.minimum_block,
            case.block_contributions,
            case.block_contributions,
            [2, 2, 2],
        ),
    ] {
        let passes = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == axis)
            .collect::<Vec<_>>();
        assert!(!passes.is_empty(), "{}: missing {axis:?} passes", case.id);
        for (index, item) in ITEMS.iter().enumerate() {
            let node = fixture.nodes[item.id].id();
            for (kind, expected) in [
                (ContributionKind::Minimum, minima[index]),
                (ContributionKind::MinContent, min_contents[index]),
                (ContributionKind::MaxContent, max_contents[index]),
            ] {
                assert!(
                    passes
                        .iter()
                        .any(|pass| pass.contributions.iter().any(|contribution| {
                            contribution.item == node
                                && contribution.kind == kind
                                && contribution.span == spans[index]
                                && (contribution.value - expected).abs() <= 0.05
                        })),
                    "{}: {axis:?} {} {kind:?} span-{} expected {expected}",
                    case.id,
                    item.key,
                    spans[index]
                );
            }
        }
    }
}

fn assert_span_ordered_column_growth(
    output: &crate::StyleLayoutOutput,
    fixture: &Fixture,
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{scene}: missing Grid track-sizing trace"));
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{scene}: initial Columns pass missing"));
    let batch = |span| {
        columns
            .stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.stage,
                    LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                        span: observed,
                        ..
                    } if observed == span
                )
            })
            .unwrap_or_else(|| panic!("{scene}: span-{span} intrinsic batch missing"))
    };
    let tracks = |stage: &spinon_layout::LayoutGridTrackSizingStageTrace| {
        stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>()
    };
    let initialized = columns
        .stages
        .iter()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
        .unwrap_or_else(|| panic!("{scene}: initial Columns state missing"));
    let span2 = batch(2);
    let span3 = batch(3);
    assert_eq!(
        tracks(initialized),
        [2.0, 2.0, 2.0],
        "{scene}: initial bases"
    );
    assert_eq!(tracks(span2), [9.0, 19.0, 19.0], "{scene}: span-2 bases");
    assert_eq!(
        tracks(span3),
        [9.0, 19.0, 19.0],
        "{scene}: span-3 must not grow bases"
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
            .map(|contribution| contribution.value)
            .unwrap_or_else(|| panic!("{scene}: {} Minimum contribution missing", item.key))
    };
    let a_share = (minimum_for(A, 2) - 2.0 - 2.0 - 2.0) / 2.0;
    let b_share = (minimum_for(B, 2) - 2.0 - 2.0 - 2.0) / 2.0;
    assert_close(a_share, 7.0, scene, "A planned increase per track");
    assert_close(b_share, 17.0, scene, "B planned increase per track");
    assert!(
        minimum_for(C, 3) <= 51.0,
        "{scene}: C span-3 contribution exceeds existing 51px area"
    );
    assert_eq!(
        tracks(span2),
        tracks(span3),
        "{scene}: span-3 C changed established span-2 bases"
    );
}

fn assert_axis_tracks(
    output: &crate::StyleLayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid)
        .unwrap_or_else(|| panic!("{scene}: missing Grid track-sizing trace"));
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} stage"));
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (index, (track, expected)) in tracks.iter().zip(expected).enumerate() {
        assert_close(
            track.base_size,
            *expected,
            scene,
            &format!("{axis:?} track {}", index + 1),
        );
    }
}

fn assert_frame<const N: usize>(actual: [f32; N], expected: [f32; N], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(
            *actual,
            expected,
            scene,
            &format!("{label} component {index}"),
        );
    }
}

fn assert_close(actual: f32, expected: f32, scene: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{scene}: {label} expected {expected}, got {actual}"
    );
}
