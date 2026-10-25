use super::{input, node, text_engine};
use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutBorder, LayoutCalcId, LayoutCssMath,
    LayoutCssMathProperty, LayoutCssMathValue, LayoutDimension, LayoutEdges, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingStageKind, LayoutJustifyContent, LayoutLengthPercentage,
    TaffyLayoutEngine, WhiteSpaceMode, WordBreakMode,
};

#[test]
fn chrome155_spanning_auto_min_flex_tracks_match_text_contribution_and_frames() {
    // Expected values come from the saved Chrome 155 spanning-auto-min-flex-tracks observation.
    let mut grid_input = input(100.0, "XXXXXXXXXXXXXXXXXXXX", WhiteSpaceMode::NoWrap);
    grid_input.nodes[0].style.display = crate::LayoutDisplay::Grid;
    grid_input.nodes[0].style.height = LayoutDimension::Fixed(30.0);
    grid_input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(4.0),
        ..LayoutGap::default()
    };
    grid_input.nodes[0].style.align_content = Some(LayoutAlignContent::FlexStart);
    grid_input.nodes[0].style.justify_content = LayoutJustifyContent::FlexStart;
    grid_input.viewport.height = 30.0;
    grid_input.nodes[1].style.width = LayoutDimension::Auto;
    grid_input.nodes[1].style.height = LayoutDimension::Auto;
    grid_input.grid_containers.insert(
        node(1),
        GridContainerInput {
            columns: vec![
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(0.0),
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                },
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(0.0),
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                },
            ],
            rows: vec![GridTrackSize::Auto],
            placement_mode: GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        },
    );
    grid_input.grid_items.insert(
        node(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(3),
        },
    );

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&grid_input, &text_engine())
        .unwrap();
    let grid = output.frames[&node(1)];
    let item = output.frames[&node(2)];
    assert_eq!((grid.width, grid.height), (100.0, 30.0));
    assert_eq!(
        (item.x, item.y, item.width, item.height),
        (0.0, 0.0, 100.0, 10.0)
    );

    let intrinsic = output
        .grid_item_intrinsic_inputs
        .iter()
        .find(|input| input.node == node(2))
        .expect("the spanning text item should retain its intrinsic input")
        .contribution_input()
        .unwrap();
    assert_eq!(intrinsic.width.min_content, Some(200.0));
    assert_eq!(intrinsic.width.max_content, Some(200.0));

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == node(1))
        .expect("the Grid should retain its track sizing trace");
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .expect("the Grid should contain a column sizing pass");
    let final_column_sizes = columns
        .stages
        .last()
        .expect("the column sizing pass should have a final stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(final_column_sizes, [48.0, 48.0]);
    assert!(columns.stages.iter().any(|stage| {
        matches!(
            stage.stage,
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
        )
    }));

    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("the Grid should contain a row sizing pass");
    let final_row_sizes = rows
        .stages
        .last()
        .expect("the row sizing pass should have a final stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(final_row_sizes, [10.0]);

    let max_content_trace = columns.contributions.iter().find(|contribution| {
        contribution.item == node(2)
            && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
    });
    if let Some(contribution) = max_content_trace {
        assert_eq!(contribution.span, 2);
        assert_eq!(contribution.value, 200.0);
    }
}

struct ChromeTextCase {
    id: &'static str,
    text: &'static str,
    white_space: WhiteSpaceMode,
    word_break: WordBreakMode,
    columns: GridTrackSize,
    grid: (f32, f32),
    min_width: LayoutDimension,
    max_width: LayoutDimension,
    horizontal_padding: f32,
    horizontal_border: f32,
    intrinsic: (f32, f32),
    expected_columns: f32,
    expected_item: (f32, f32),
    expected_outer_min_content: f32,
    expected_outer_max_content: f32,
}

#[test]
fn chrome155_text_track_oracle_cases_match_contributions_traces_and_frames() {
    let cases = [
        ChromeTextCase {
            id: "auto-text-max-content",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::Auto,
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 100.0,
            expected_item: (100.0, 10.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "min-content-text-normal",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 70.0,
            expected_item: (70.0, 20.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "max-content-text-normal",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MaxContent,
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 100.0,
            expected_item: (100.0, 10.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "fit-content-text-minimum",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 70.0,
            expected_item: (70.0, 20.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "fit-content-calc-limit",
            text: "XXXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::BreakAll,
            columns: GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(LayoutCalcId(1))),
            grid: (100.0, 120.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (10.0, 80.0),
            expected_columns: 15.0,
            expected_item: (15.0, 80.0),
            expected_outer_min_content: 10.0,
            expected_outer_max_content: 80.0,
        },
        ChromeTextCase {
            id: "min-content-text-edge",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 50.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 4.0,
            horizontal_border: 1.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 80.0,
            expected_item: (80.0, 20.0),
            expected_outer_min_content: 80.0,
            expected_outer_max_content: 110.0,
        },
        ChromeTextCase {
            id: "min-content-text-min-width",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 50.0),
            min_width: LayoutDimension::Fixed(90.0),
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 90.0,
            expected_item: (90.0, 20.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "min-content-text-max-width",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 50.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Fixed(40.0),
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (70.0, 100.0),
            expected_columns: 40.0,
            expected_item: (40.0, 20.0),
            expected_outer_min_content: 70.0,
            expected_outer_max_content: 100.0,
        },
        ChromeTextCase {
            id: "min-content-text-break-all",
            text: "XXXXXXXX",
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::BreakAll,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (10.0, 80.0),
            expected_columns: 10.0,
            expected_item: (10.0, 80.0),
            expected_outer_min_content: 10.0,
            expected_outer_max_content: 80.0,
        },
        ChromeTextCase {
            id: "min-content-text-nowrap",
            text: "XX XXXXXXX",
            white_space: WhiteSpaceMode::NoWrap,
            word_break: WordBreakMode::Normal,
            columns: GridTrackSize::MinContent,
            grid: (120.0, 40.0),
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            horizontal_padding: 0.0,
            horizontal_border: 0.0,
            intrinsic: (100.0, 100.0),
            expected_columns: 100.0,
            expected_item: (100.0, 10.0),
            expected_outer_min_content: 100.0,
            expected_outer_max_content: 100.0,
        },
    ];

    for case in &cases {
        let mut input = input(case.grid.0, case.text, case.white_space);
        input.nodes[0].style.display = crate::LayoutDisplay::Grid;
        input.nodes[0].style.width = LayoutDimension::Fixed(case.grid.0);
        input.nodes[0].style.height = LayoutDimension::Fixed(case.grid.1);
        input.nodes[0].style.align_content = Some(LayoutAlignContent::FlexStart);
        input.nodes[0].style.justify_content = LayoutJustifyContent::FlexStart;
        input.nodes[1].style.width = LayoutDimension::Auto;
        input.nodes[1].style.height = LayoutDimension::Auto;
        input.nodes[1].style.min_width = case.min_width;
        input.nodes[1].style.max_width = case.max_width;
        input.nodes[1].style.padding = LayoutEdges {
            left: LayoutLengthPercentage::length(case.horizontal_padding),
            right: LayoutLengthPercentage::length(case.horizontal_padding),
            ..LayoutEdges::default()
        };
        input.nodes[1].style.border = LayoutBorder {
            left: case.horizontal_border,
            right: case.horizontal_border,
            ..LayoutBorder::default()
        };
        input.viewport.width = case.grid.0;
        input.viewport.height = case.grid.1;
        let crate::TextParagraphInput::Paragraph(mut paragraph) =
            input.text_paragraphs.remove(&node(2)).unwrap()
        else {
            panic!("{}: expected plain Ahem paragraph input", case.id);
        };
        paragraph.style.word_break = case.word_break;
        input
            .text_paragraphs
            .insert(node(2), crate::TextParagraphInput::Paragraph(paragraph));
        input.grid_containers.insert(
            node(1),
            GridContainerInput {
                columns: vec![case.columns],
                rows: vec![GridTrackSize::Auto],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        );
        input.grid_items.insert(
            node(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        );

        if case.id == "fit-content-calc-limit" {
            input = input.with_css_math(vec![LayoutCssMathValue {
                id: LayoutCalcId(1),
                node_id: node(1),
                property: LayoutCssMathProperty::GridTemplateColumns,
                expression: LayoutCssMath::Sum(vec![
                    LayoutCssMath::Percentage(0.1),
                    LayoutCssMath::LengthPx(5.0),
                ]),
            }]);
        }

        let output = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &text_engine())
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
        let grid = output.frames[&node(1)];
        let item = output.frames[&node(2)];
        assert_close(grid.width, case.grid.0, case.id, "grid.width");
        assert_close(grid.height, case.grid.1, case.id, "grid.height");
        assert_close(item.x, 0.0, case.id, "item.x");
        assert_close(item.y, 0.0, case.id, "item.y");
        assert_close(item.width, case.expected_item.0, case.id, "item.width");
        assert_close(item.height, case.expected_item.1, case.id, "item.height");

        let intrinsic = output
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == node(2))
            .unwrap_or_else(|| panic!("{}: missing text intrinsic input", case.id))
            .contribution_input()
            .unwrap();
        assert_eq!(
            intrinsic.width.min_content,
            Some(case.intrinsic.0),
            "{}",
            case.id
        );
        assert_eq!(
            intrinsic.width.max_content,
            Some(case.intrinsic.1),
            "{}",
            case.id
        );
        assert_eq!(
            intrinsic
                .width
                .outer_size(Some(case.intrinsic.0), node(2), "width"),
            Ok(Some(case.expected_outer_min_content)),
            "{}: outer min-content input",
            case.id
        );
        assert_eq!(
            intrinsic
                .width
                .outer_size(Some(case.intrinsic.1), node(2), "width"),
            Ok(Some(case.expected_outer_max_content)),
            "{}: outer max-content input",
            case.id
        );

        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == node(1))
            .unwrap_or_else(|| panic!("{}: missing Grid track trace", case.id));
        assert_text_track_sizes(
            trace,
            LayoutGridTrackSizingAxis::Columns,
            &[case.expected_columns],
            case.id,
        );
        assert_text_track_sizes(
            trace,
            LayoutGridTrackSizingAxis::Rows,
            &[case.expected_item.1],
            case.id,
        );
    }
}

fn assert_text_track_sizes(
    trace: &crate::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    case: &str,
) {
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{case}: missing {axis:?} pass"));
    assert!(
        pass.stages.iter().any(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                    | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
            )
        }),
        "{case}: missing {axis:?} completion marker"
    );
    let final_sizes = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final {axis:?} stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        final_sizes.len(),
        expected.len(),
        "{case}: {axis:?} track count"
    );
    for (index, (actual, expected)) in final_sizes.iter().zip(expected).enumerate() {
        assert_close(*actual, *expected, case, &format!("{axis:?}[{index}]"));
    }
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{case}: {property} expected {expected}, got {actual}"
    );
}
