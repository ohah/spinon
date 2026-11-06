use super::*;
use crate::{ContentAlignmentPosition, LayoutAlignContent};
use serde_json::Value;

const EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-precomparison-2026-11-07/expected-values.json"
);
const TOLERANCE: f32 = 1.0 / 32.0;

fn alignment(value: &str) -> LayoutAlignContent {
    match value {
        "normal" => LayoutAlignContent::Normal,
        "stretch" => LayoutAlignContent::Stretch,
        "start" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::Start,
            safety: None,
        },
        "flex-start" => LayoutAlignContent::FlexStart,
        "end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::End,
            safety: None,
        },
        "flex-end" => LayoutAlignContent::FlexEnd,
        "center" => LayoutAlignContent::Center,
        "space-between" => LayoutAlignContent::SpaceBetween,
        "space-around" => LayoutAlignContent::SpaceAround,
        "space-evenly" => LayoutAlignContent::SpaceEvenly,
        other => panic!("unexpected alignment in expected values: {other}"),
    }
}

fn expected_frame(value: &Value) -> [f32; 4] {
    value
        .as_array()
        .expect("expected item rect should be an array")
        .iter()
        .map(|component| {
            component
                .as_f64()
                .expect("expected item coordinate should be numeric") as f32
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("expected item rect should have four components")
}

fn expected_values(value: &Value) -> Vec<f32> {
    value
        .as_array()
        .expect("expected tracks should be an array")
        .iter()
        .map(|value| value.as_f64().expect("track size should be numeric") as f32)
        .collect()
}

#[test]
fn root_html_auto_row_stretch_matches_percentage_gap_alignment_matrix() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("percentage row-gap expected values should be valid JSON");
    let cases = expected["expectedCases"]
        .as_array()
        .expect("expected matrix should list cases");
    assert_eq!(cases.len(), 70);

    for case in cases {
        let percentage = case["percentage"]
            .as_f64()
            .expect("percentage should be numeric") as f32;
        let align = case["alignment"]
            .as_str()
            .expect("alignment should be a string");
        let mut input = super::grid_input(
            vec![GridTrackSize::LengthPx(40.0)],
            vec![GridTrackSize::LengthPx(10.0)],
            3,
            LayoutGap {
                row: LayoutLengthPercentage::Percentage(percentage / 100.0),
                column: LayoutLengthPercentage::ZERO,
            },
        );
        input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.align_items = crate::LayoutAlignItems::Normal;
        input.nodes[0].style.align_content = Some(alignment(align));
        input.viewport = Viewport {
            width: 40.0,
            height: 40.0,
        };
        for child in &mut input.nodes[1..] {
            child.style.width = LayoutDimension::Fixed(40.0);
            child.style.height = LayoutDimension::Fixed(10.0);
        }
        let container = input
            .grid_containers
            .get_mut(&id(1))
            .expect("root Grid should exist");
        container.placement_mode = GridPlacementMode::RootHtmlAutoRowStretchV1;
        container.track_sizing_trace_enabled = true;

        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{percentage}% / {align} should layout: {error:?}"));
        let expected_items = case["itemRects"]
            .as_array()
            .expect("case should contain three expected item rects");
        assert_eq!(expected_items.len(), 3);
        for (index, expected_item) in expected_items.iter().enumerate() {
            let actual = output.frames[&id(index as u64 + 2)];
            let expected = expected_frame(expected_item);
            assert!(
                (actual.x - expected[0]).abs() <= TOLERANCE
                    && (actual.y - expected[1]).abs() <= TOLERANCE
                    && (actual.width - expected[2]).abs() <= TOLERANCE,
                "{percentage}% / {align} item {index}: expected position/width {:?}, got {:?}",
                &expected[..3],
                [actual.x, actual.y, actual.width, actual.height]
            );
            assert_eq!(actual.height, 10.0, "fixed item {index} keeps its height");
        }
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .expect("Root HTML Grid should expose its track sizing trace");
        let row_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
            .expect("trace should include row sizing");
        let stretch_stage = row_pass
            .stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.stage,
                    crate::LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                )
            })
            .expect("trace should include the auto-row stretch stage");
        let actual_tracks = stretch_stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        let expected_tracks = expected_values(&case["tracksPx"]);
        assert_eq!(actual_tracks.len(), expected_tracks.len());
        for (index, (actual, expected)) in actual_tracks.iter().zip(expected_tracks).enumerate() {
            assert!(
                (actual - expected).abs() <= TOLERANCE,
                "{percentage}% / {align} row {index}: expected {expected}, got {actual}"
            );
        }
    }
}
