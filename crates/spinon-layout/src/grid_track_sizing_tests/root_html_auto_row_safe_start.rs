use super::*;
use crate::{
    AlignmentSafety, ContentAlignmentPosition, LayoutAlignContent, LayoutCalcId, LayoutCssMath,
    LayoutCssMathProperty, LayoutCssMathValue,
};
use serde_json::Value;

const EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-auto-row-safe-start-precomparison-2026-11-07/expected-values.json"
);
const MATH_ID: LayoutCalcId = LayoutCalcId(706);
const TOLERANCE: f32 = 1.0 / 64.0;

fn alignment(value: &str) -> LayoutAlignContent {
    let safety = match value {
        "start" => None,
        "safe start" => Some(AlignmentSafety::Safe),
        "unsafe start" => Some(AlignmentSafety::Unsafe),
        other => panic!("unexpected alignment: {other}"),
    };
    LayoutAlignContent::Position {
        position: ContentAlignmentPosition::Start,
        safety,
    }
}

fn input_for(scene: &Value) -> LayoutInput {
    let scene_id = scene["id"].as_str().expect("scene should have an id");
    let overflow = scene_id.ends_with("overflow");
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
    input.nodes[0].style.align_content = Some(alignment(
        scene["computed_align_content"]
            .as_str()
            .expect("computed alignment should be text"),
    ));
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
        input.css_math.push(LayoutCssMathValue {
            id: MATH_ID,
            node_id: id(1),
            property: LayoutCssMathProperty::RowGap,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(1.2),
                LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(2.0))),
            ]),
        });
    }
    input
}

#[test]
fn root_html_auto_row_safe_start_matches_chrome_and_independent_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("safe-start expected values should be valid JSON");
    let scenes = expected["scenes"]
        .as_array()
        .expect("expected values should list scenes");

    for scene in scenes {
        let input = input_for(scene);
        let id_name = scene["id"].as_str().unwrap();
        let content_alignment = scene["computed_align_content"].as_str().unwrap();
        assert!(
            super::super::root_html_intrinsic_align_content_is_supported(
                alignment(content_alignment),
                GridPlacementMode::RootHtmlAutoRowStretchV1
            ),
            "{content_alignment} should be admitted by this fixed profile"
        );
        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{id_name}: {error:?}"));
        for (index, expected_y) in scene["item_y_px"].as_array().unwrap().iter().enumerate() {
            let expected_y = expected_y.as_f64().unwrap() as f32;
            let frame = output.frames[&id(index as u64 + 2)];
            assert!(
                (frame.y - expected_y).abs() <= TOLERANCE,
                "{id_name} item {index}: expected y={expected_y}, got {}",
                frame.y
            );
            assert_eq!(frame.width, 40.0);
            assert_eq!(frame.height, 10.0);
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
            .expect("trace should include row sizing")
            .stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.stage,
                    crate::LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                )
            })
            .expect("trace should include auto-row stretch");
        let rows = stretch_stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(rows, [10.0, 10.0, 10.0], "alignment must not resize tracks");
        let gutters = stretch_stage
            .tracks
            .iter()
            .filter(|track| track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        let expected_gap = if id_name.ends_with("overflow") {
            46.0
        } else {
            0.0
        };
        assert_eq!(gutters, [expected_gap, expected_gap]);
    }
}

#[test]
fn root_html_auto_row_safe_start_still_rejects_baseline_alignment() {
    let mut input = input_for(&serde_json::from_str::<Value>(EXPECTED).unwrap()["scenes"][0]);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::FirstBaseline);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "align-content"
        }) if node == id(1)
    ));
}
