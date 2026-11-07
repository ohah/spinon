use super::*;
use crate::{AlignmentSafety, ContentAlignmentPosition, LayoutAlignContent, LayoutCalcId};
use serde_json::Value;

const EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/expected.json"
);
const MATH_ID: LayoutCalcId = LayoutCalcId(706);
const TOLERANCE: f32 = 1.0 / 64.0;

fn alignment(value: &str) -> LayoutAlignContent {
    match value {
        "end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::End,
            safety: None,
        },
        "safe flex-end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::FlexEnd,
            safety: Some(AlignmentSafety::Safe),
        },
        "unsafe flex-end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::FlexEnd,
            safety: Some(AlignmentSafety::Unsafe),
        },
        other => panic!("unexpected alignment: {other}"),
    }
}

fn input_for(alignment_value: &str, overflow: bool) -> LayoutInput {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(10.0)],
        3,
        LayoutGap {
            row: if overflow {
                LayoutLengthPercentage::Calc(MATH_ID)
            } else {
                LayoutLengthPercentage::ZERO
            },
            column: LayoutLengthPercentage::ZERO,
        },
    );
    input.viewport = Viewport {
        width: 40.0,
        height: 40.0,
    };
    input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
    input.nodes[0].style.align_items = crate::LayoutAlignItems::Normal;
    input.nodes[0].style.align_content = Some(alignment(alignment_value));
    for child in &mut input.nodes[1..] {
        child.style.width = LayoutDimension::Fixed(40.0);
        child.style.height = LayoutDimension::Fixed(10.0);
    }
    let container = input
        .grid_containers
        .get_mut(&id(1))
        .expect("Root Grid should exist");
    container.placement_mode = GridPlacementMode::RootHtmlAutoRowStretchV1;
    container.track_sizing_trace_enabled = true;
    if overflow {
        input.css_math.push(crate::LayoutCssMathValue {
            id: MATH_ID,
            node_id: id(1),
            property: crate::LayoutCssMathProperty::RowGap,
            expression: crate::LayoutCssMath::Sum(vec![
                crate::LayoutCssMath::Percentage(1.2),
                crate::LayoutCssMath::Negate(Box::new(crate::LayoutCssMath::LengthPx(2.0))),
            ]),
        });
    }
    input
}

#[test]
fn root_html_auto_row_safe_flex_end_matches_chrome_and_independent_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("flex-end expected values should be valid JSON");
    let alignments = expected["alignmentValues"]
        .as_array()
        .expect("expected values should list alignments");

    for (gap_case, overflow) in [("positive", false), ("overflow", true)] {
        for alignment_value in alignments {
            let alignment_value = alignment_value
                .as_str()
                .expect("alignment should be a string");
            let expected_frames = expected["expectedFramesByAlignmentAndGapCase"][alignment_value]
                [gap_case]
                .as_array()
                .expect("expected values should list item frames");
            let input = input_for(alignment_value, overflow);
            assert!(
                super::super::root_html_intrinsic_align_content_is_supported(
                    alignment(alignment_value),
                    GridPlacementMode::RootHtmlAutoRowStretchV1,
                ),
                "{alignment_value} should be admitted by this fixed profile"
            );
            let output = TaffyLayoutEngine
                .compute(&input)
                .unwrap_or_else(|error| panic!("{gap_case} / {alignment_value}: {error:?}"));
            for (index, expected_frame) in expected_frames.iter().enumerate() {
                let frame = output.frames[&id(index as u64 + 2)];
                for (axis, actual, expected) in [
                    ("x", frame.x, expected_frame["x"].as_f64().unwrap() as f32),
                    ("y", frame.y, expected_frame["y"].as_f64().unwrap() as f32),
                    (
                        "width",
                        frame.width,
                        expected_frame["width"].as_f64().unwrap() as f32,
                    ),
                    (
                        "height",
                        frame.height,
                        expected_frame["height"].as_f64().unwrap() as f32,
                    ),
                ] {
                    assert!(
                        (actual - expected).abs() <= TOLERANCE,
                        "{gap_case} / {alignment_value} / item {index} {axis}: expected {expected}, got {actual}"
                    );
                }
            }

            let trace = output
                .grid_track_sizing_traces
                .iter()
                .find(|trace| trace.node == id(1))
                .expect("Root Grid should expose its track sizing trace");
            let stretch_stage = trace
                .passes
                .iter()
                .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
                .and_then(|pass| {
                    pass.stages.iter().find(|stage| {
                        matches!(
                            stage.stage,
                            crate::LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                        )
                    })
                })
                .expect("trace should include auto-row stretch");
            let rows = stretch_stage
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            assert_eq!(rows, [10.0, 10.0, 10.0]);
            let gutters = stretch_stage
                .tracks
                .iter()
                .filter(|track| track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            let expected_gap = if overflow { 46.0 } else { 0.0 };
            assert_eq!(gutters, [expected_gap, expected_gap]);
        }
    }
}
