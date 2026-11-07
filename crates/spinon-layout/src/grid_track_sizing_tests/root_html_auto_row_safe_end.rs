use super::*;
use crate::{
    AlignmentSafety, ContentAlignmentPosition, LayoutAlignContent, LayoutCalcId, LayoutCssMath,
    LayoutCssMathProperty, LayoutCssMathValue,
};
use serde_json::Value;

const EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/expected-values.json"
);
const MATH_ID: LayoutCalcId = LayoutCalcId(705);
const TOLERANCE: f32 = 1.0 / 64.0;

fn alignment(value: &str) -> LayoutAlignContent {
    match value {
        "safe end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::End,
            safety: Some(AlignmentSafety::Safe),
        },
        "unsafe end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::End,
            safety: Some(AlignmentSafety::Unsafe),
        },
        "end" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::End,
            safety: None,
        },
        "safe center" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::Center,
            safety: Some(AlignmentSafety::Safe),
        },
        "unsafe center" => LayoutAlignContent::Position {
            position: ContentAlignmentPosition::Center,
            safety: Some(AlignmentSafety::Unsafe),
        },
        other => panic!("unexpected alignment: {other}"),
    }
}

fn make_input(align_content: &str, overflow: bool) -> LayoutInput {
    let row_gap = if overflow {
        LayoutLengthPercentage::Calc(MATH_ID)
    } else {
        LayoutLengthPercentage::ZERO
    };
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(10.0)],
        3,
        LayoutGap {
            row: row_gap,
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
    input.nodes[0].style.align_content = Some(alignment(align_content));
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
fn root_html_auto_row_safe_end_matches_chrome_and_independent_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("safe-end expected values should be valid JSON");
    let scenes = [
        ("positiveSpace", "safe end"),
        ("positiveSpace", "unsafe end"),
        ("positiveSpace", "end"),
        ("overflow", "safe end"),
        ("overflow", "unsafe end"),
        ("overflow", "end"),
        ("overflow", "safe center"),
        ("overflow", "unsafe center"),
    ];
    for (group_name, align_content) in scenes {
        let group = &expected[group_name];
        let overflow = group_name == "overflow";
        let expected_y = group["expectedYRelativeCssPx"][align_content]
            .as_array()
            .expect("expected scene should list item y offsets");
        let input = make_input(align_content, overflow);
        assert!(
            super::super::root_html_intrinsic_align_content_is_supported(
                alignment(align_content),
                GridPlacementMode::RootHtmlAutoRowStretchV1
            ),
            "{align_content} should be admitted by this fixed profile"
        );
        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{group_name} / {align_content}: {error:?}"));
        for (index, expected) in expected_y.iter().enumerate() {
            let expected = expected.as_f64().expect("y offset should be numeric") as f32;
            let frame = output.frames[&id(index as u64 + 2)];
            assert!(
                (frame.y - expected).abs() <= TOLERANCE,
                "{group_name} / {align_content} item {index}: expected y={expected}, got {}",
                frame.y
            );
            assert_eq!(
                frame.height, 10.0,
                "fixed row item height should not change"
            );
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
            .expect("trace should include the auto-row stretch stage");
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
        let expected_gutter = if overflow { 46.0 } else { 0.0 };
        assert_eq!(gutters, [expected_gutter, expected_gutter]);
    }
}

#[test]
fn root_html_auto_row_safe_end_keeps_baseline_alignment_rejected() {
    let mut input = make_input("end", true);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::FirstBaseline);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "align-content"
        }) if node == id(1)
    ));
}
