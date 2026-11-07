use super::*;
use crate::{
    ContentAlignmentPosition, LayoutAlignContent, LayoutCalcId, LayoutCssMath,
    LayoutCssMathProperty, LayoutCssMathValue,
};
use serde_json::Value;

const EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/expected-values.json"
);
const TOLERANCE: f32 = 1.0 / 32.0;
const MATH_ID: LayoutCalcId = LayoutCalcId(701);

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

fn expression(value: &str) -> LayoutCssMath {
    match value {
        "calc(1px - 10%)" => LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(1.0),
            LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.1))),
        ]),
        "calc(6.25% + 2px)" | "calc(2px + 6.25%)" => LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.0625),
            LayoutCssMath::LengthPx(2.0),
        ]),
        "calc(25% - 2px)" => LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.25),
            LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(2.0))),
        ]),
        "calc(10% - 1px)" => LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(1.0))),
        ]),
        "calc(120% - 2px)" => LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(1.2),
            LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(2.0))),
        ]),
        other => panic!("unexpected calc expression in expected values: {other}"),
    }
}

fn make_input(item_count: usize, calc: LayoutCssMath, align_content: &str) -> LayoutInput {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(10.0)],
        item_count,
        LayoutGap {
            row: LayoutLengthPercentage::Calc(MATH_ID),
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
    input.css_math.push(LayoutCssMathValue {
        id: MATH_ID,
        node_id: id(1),
        property: LayoutCssMathProperty::RowGap,
        expression: calc,
    });
    input
}

#[test]
fn root_html_auto_row_stretch_matches_calc_gap_alignment_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("calc row-gap expected values should be valid JSON");
    let groups = expected["groups"]
        .as_array()
        .expect("expected matrix should list expression groups");
    assert_eq!(groups.len(), 6);
    let mut compared = 0;

    for group in groups {
        let css_expression = group["expression"]
            .as_str()
            .expect("calc expression should be a string");
        let expression = expression(css_expression);
        let item_count_cases = group["itemCountCases"]
            .as_array()
            .expect("expected group should cover item counts");
        assert_eq!(item_count_cases.len(), 2);
        for item_count_case in item_count_cases {
            let item_count = item_count_case["itemCount"]
                .as_u64()
                .expect("item count should be numeric") as usize;
            assert!(matches!(item_count, 2 | 3));
            let cases = item_count_case["cases"]
                .as_array()
                .expect("item count group should list alignment cases");
            assert_eq!(cases.len(), 10);
            for case in cases {
                let align_content = case["alignContent"]
                    .as_str()
                    .expect("alignment should be a string");
                let layout_input = make_input(item_count, expression.clone(), align_content);
                let output = TaffyLayoutEngine
                    .compute(&layout_input)
                    .unwrap_or_else(|error| {
                        panic!("{css_expression} / {item_count} items / {align_content}: {error:?}")
                    });
                let expected_rects = case["itemRects"]
                    .as_array()
                    .expect("expected case should contain item rectangles");
                assert_eq!(expected_rects.len(), item_count);
                for (index, expected_rect) in expected_rects.iter().enumerate() {
                    let expected = expected_rect
                        .as_array()
                        .expect("expected rectangle should be an array");
                    let frame = output.frames[&id(index as u64 + 2)];
                    let expected: [f32; 4] = expected
                        .iter()
                        .map(|component| {
                            component
                                .as_f64()
                                .expect("rectangle component should be numeric")
                                as f32
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("rectangle should have four components");
                    for (axis, actual, expected) in [
                        ("x", frame.x, expected[0]),
                        ("y", frame.y, expected[1]),
                        ("width", frame.width, expected[2]),
                        ("fixed-item-height", frame.height, 10.0),
                    ] {
                        assert!(
                            (actual - expected).abs() <= TOLERANCE,
                            "{css_expression} / {item_count} items / {align_content} item {index} {axis}: expected {expected}, got {actual}"
                        );
                        compared += 1;
                    }
                }
                let expected_rows = case["usedRowsPx"]
                    .as_array()
                    .expect("expected case should contain used row sizes");
                let trace = output
                    .grid_track_sizing_traces
                    .iter()
                    .find(|trace| trace.node == id(1))
                    .expect("Root HTML Grid should expose its track-sizing trace");
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
                    .expect("trace should report the auto-track stretch stage");
                let actual_rows = stretch_stage
                    .tracks
                    .iter()
                    .filter(|track| !track.is_gutter && !track.is_collapsed)
                    .map(|track| track.base_size)
                    .collect::<Vec<_>>();
                assert_eq!(actual_rows.len(), expected_rows.len());
                for (actual, expected) in actual_rows.iter().zip(expected_rows) {
                    let expected = expected
                        .as_f64()
                        .expect("expected row size should be numeric")
                        as f32;
                    assert!(
                        (actual - expected).abs() <= TOLERANCE,
                        "{css_expression} / {item_count} items / {align_content} row: expected {expected}, got {actual}"
                    );
                    compared += 1;
                }
            }
        }
    }
    assert_eq!(compared, 1_500);
}

#[test]
fn root_html_auto_row_stretch_calc_gap_rejects_missing_or_misbound_math() {
    let calc = expression("calc(6.25% + 2px)");
    let mut input = make_input(3, calc.clone(), "start");
    input.css_math.clear();
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::MissingCssMath { node, property: "row-gap", id: MATH_ID }) if node == id(1)
    ));

    let mut input = make_input(3, calc.clone(), "start");
    input.css_math[0].node_id = id(2);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::CssMathBindingMismatch { node, property: "row-gap", id: MATH_ID }) if node == id(1)
    ));

    let mut input = make_input(3, calc.clone(), "start");
    input.css_math[0].property = LayoutCssMathProperty::ColumnGap;
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::CssMathBindingMismatch { node, property: "row-gap", id: MATH_ID }) if node == id(1)
    ));

    let mut input = make_input(3, calc, "start");
    input.css_math.push(LayoutCssMathValue {
        id: MATH_ID,
        node_id: id(1),
        property: LayoutCssMathProperty::RowGap,
        expression: expression("calc(6.25% + 2px)"),
    });
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::DuplicateCssMathId)
    ));
}

#[test]
fn root_html_auto_row_stretch_calc_gap_rejects_unsupported_math_and_profile_shapes() {
    let mut input = make_input(
        3,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(1.0),
            LayoutCssMath::LengthPx(2.0),
        ]),
        "start",
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, property: "row-gap" }) if node == id(1)
    ));

    input = make_input(3, expression("calc(6.25% + 2px)"), "start");
    input.nodes[0].style.height = LayoutDimension::Auto;
    let result = TaffyLayoutEngine.compute(&input);
    assert!(
        matches!(
            result,
            Err(crate::LayoutError::RootSizeMismatch { axis: "height" })
        ),
        "an indefinite block-size basis should keep the established root-size error: {result:?}"
    );
}
