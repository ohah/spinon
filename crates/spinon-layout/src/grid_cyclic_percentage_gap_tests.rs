use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode, GridTrackSize,
    LayoutAlignContent, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue, LayoutDimension,
    LayoutDisplay, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis, LayoutInput,
    LayoutJustifyContent, LayoutLengthPercentage, LayoutStyle, TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct GapCase {
    id: &'static str,
    width: LayoutDimension,
    height: LayoutDimension,
    min_width: Option<f32>,
    min_height: Option<f32>,
    row_gap: LayoutLengthPercentage,
    column_gap: LayoutLengthPercentage,
    css_math: Vec<(u32, LayoutCssMathProperty, f32)>,
    expected_grid: (f32, f32),
    expected_gaps: (f32, f32),
    expected_second_column_x: f32,
    expected_second_row_y: f32,
    expected_overflow: (f32, f32),
}

fn document_with_two_by_two_items(
    owner: u64,
) -> (
    HostDocument,
    HostNodeHandle,
    HostNodeHandle,
    Vec<HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let items = (0..4)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: grid,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: grid,
            before: None,
        });
    for item in &items {
        batch
            .push(DocumentOperation::CreateElement {
                node: *item,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(grid),
                node: *item,
                before: None,
            });
    }
    document.commit(batch).unwrap();
    (document, root, grid, items)
}

fn placement(row: i32, column: i32) -> GridItemInput {
    GridItemInput {
        row_start: GridPlacementInput::Line(row),
        row_end: GridPlacementInput::Line(row + 1),
        column_start: GridPlacementInput::Line(column),
        column_end: GridPlacementInput::Line(column + 1),
    }
}

fn run_case(case: &GapCase, owner: u64) {
    let (document, root, grid, items) = document_with_two_by_two_items(owner);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Block,
        width: LayoutDimension::Fixed(300.0),
        height: LayoutDimension::Fixed(200.0),
        ..LayoutStyle::default()
    };
    let grid_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: case.width,
        height: case.height,
        min_width: case
            .min_width
            .map_or(LayoutDimension::Auto, LayoutDimension::Fixed),
        min_height: case
            .min_height
            .map_or(LayoutDimension::Auto, LayoutDimension::Fixed),
        gap: LayoutGap {
            row: case.row_gap,
            column: case.column_gap,
        },
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let mut styles = BTreeMap::from([(root.id(), root_style), (grid.id(), grid_style)]);
    for item in &items {
        styles.insert(
            item.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        );
    }
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: 300.0,
            height: 200.0,
        },
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_css_math(
        case.css_math
            .iter()
            .map(|(id, property, percentage)| LayoutCssMathValue {
                id: crate::LayoutCalcId(*id),
                node_id: grid.id(),
                property: *property,
                expression: LayoutCssMath::Sum(vec![
                    LayoutCssMath::LengthPx(5.0),
                    LayoutCssMath::Percentage(*percentage),
                ]),
            })
            .collect(),
    )
    .with_grid_layout(
        BTreeMap::from([(
            grid.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(60.0)],
                rows: vec![GridTrackSize::LengthPx(10.0), GridTrackSize::LengthPx(10.0)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([
            (items[0].id(), placement(1, 1)),
            (items[1].id(), placement(1, 2)),
            (items[2].id(), placement(2, 1)),
            (items[3].id(), placement(2, 2)),
        ]),
    );
    let output = TaffyLayoutEngine
        .compute(&input)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let grid_frame = output.frames[&grid.id()];
    let second_column = output.frames[&items[1].id()];
    let second_row = output.frames[&items[2].id()];
    let last_item = output.frames[&items[3].id()];
    for (actual, expected, property) in [
        (grid_frame.width, case.expected_grid.0, "grid.width"),
        (grid_frame.height, case.expected_grid.1, "grid.height"),
        (
            second_column.x - grid_frame.x - 40.0,
            case.expected_gaps.0,
            "column-gap.used",
        ),
        (
            second_row.y - grid_frame.y - 10.0,
            case.expected_gaps.1,
            "row-gap.used",
        ),
        (
            second_column.x - grid_frame.x,
            case.expected_second_column_x,
            "second-column.x",
        ),
        (
            second_row.y - grid_frame.y,
            case.expected_second_row_y,
            "second-row.y",
        ),
        (
            (last_item.x + last_item.width - (grid_frame.x + grid_frame.width)).max(0.0),
            case.expected_overflow.0,
            "overflow.right",
        ),
        (
            (last_item.y + last_item.height - (grid_frame.y + grid_frame.height)).max(0.0),
            case.expected_overflow.1,
            "overflow.bottom",
        ),
    ] {
        assert_close(actual, expected, case.id, property);
    }

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track trace", case.id));
    assert_trace_gutter(
        trace,
        LayoutGridTrackSizingAxis::Columns,
        case.expected_gaps.0,
        case.id,
    );
    assert_trace_gutter(
        trace,
        LayoutGridTrackSizingAxis::Rows,
        case.expected_gaps.1,
        case.id,
    );
}

fn assert_trace_gutter(
    trace: &crate::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: f32,
    case: &str,
) {
    let final_pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{case}: missing {axis:?} sizing pass"));
    let final_stage = final_pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final {axis:?} sizing stage"));
    let active_gutters = final_stage
        .tracks
        .iter()
        .filter(|track| track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        active_gutters.len(),
        1,
        "{case}: expected one active {axis:?} gutter in the final trace"
    );
    assert_close(
        active_gutters[0].base_size,
        expected,
        case,
        match axis {
            LayoutGridTrackSizingAxis::Columns => "trace.column-gap.final",
            LayoutGridTrackSizingAxis::Rows => "trace.row-gap.final",
        },
    );
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 0.001,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

#[test]
fn chrome155_percentage_grid_gaps_resolve_after_intrinsic_sizing() {
    let cases = [
        GapCase {
            id: "definite-percent",
            width: LayoutDimension::Fixed(100.0),
            height: LayoutDimension::Fixed(40.0),
            min_width: None,
            min_height: None,
            row_gap: LayoutLengthPercentage::percent(0.25),
            column_gap: LayoutLengthPercentage::percent(0.1),
            css_math: Vec::new(),
            expected_grid: (100.0, 40.0),
            expected_gaps: (10.0, 10.0),
            expected_second_column_x: 50.0,
            expected_second_row_y: 20.0,
            expected_overflow: (10.0, 0.0),
        },
        GapCase {
            id: "cyclic-percent",
            width: LayoutDimension::MaxContent,
            height: LayoutDimension::MaxContent,
            min_width: None,
            min_height: None,
            row_gap: LayoutLengthPercentage::percent(0.25),
            column_gap: LayoutLengthPercentage::percent(0.1),
            css_math: Vec::new(),
            expected_grid: (100.0, 20.0),
            expected_gaps: (10.0, 5.0),
            expected_second_column_x: 50.0,
            expected_second_row_y: 15.0,
            expected_overflow: (10.0, 5.0),
        },
        GapCase {
            id: "definite-calc",
            width: LayoutDimension::Fixed(100.0),
            height: LayoutDimension::Fixed(40.0),
            min_width: None,
            min_height: None,
            row_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(1)),
            column_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(2)),
            css_math: vec![
                (1, LayoutCssMathProperty::RowGap, 0.25),
                (2, LayoutCssMathProperty::ColumnGap, 0.1),
            ],
            expected_grid: (100.0, 40.0),
            expected_gaps: (15.0, 15.0),
            expected_second_column_x: 55.0,
            expected_second_row_y: 25.0,
            expected_overflow: (15.0, 0.0),
        },
        GapCase {
            id: "cyclic-calc",
            width: LayoutDimension::MaxContent,
            height: LayoutDimension::MaxContent,
            min_width: None,
            min_height: None,
            row_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(1)),
            column_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(2)),
            css_math: vec![
                (1, LayoutCssMathProperty::RowGap, 0.25),
                (2, LayoutCssMathProperty::ColumnGap, 0.1),
            ],
            expected_grid: (105.0, 25.0),
            expected_gaps: (15.5, 11.25),
            expected_second_column_x: 55.5,
            expected_second_row_y: 21.25,
            expected_overflow: (10.5, 6.25),
        },
        GapCase {
            id: "cyclic-percent-min-basis",
            width: LayoutDimension::MaxContent,
            height: LayoutDimension::MaxContent,
            min_width: Some(120.0),
            min_height: Some(30.0),
            row_gap: LayoutLengthPercentage::percent(0.25),
            column_gap: LayoutLengthPercentage::percent(0.1),
            css_math: Vec::new(),
            expected_grid: (120.0, 30.0),
            expected_gaps: (12.0, 7.5),
            expected_second_column_x: 52.0,
            expected_second_row_y: 17.5,
            expected_overflow: (0.0, 0.0),
        },
        GapCase {
            id: "cyclic-calc-min-basis",
            width: LayoutDimension::MaxContent,
            height: LayoutDimension::MaxContent,
            min_width: Some(120.0),
            min_height: Some(30.0),
            row_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(1)),
            column_gap: LayoutLengthPercentage::Calc(crate::LayoutCalcId(2)),
            css_math: vec![
                (1, LayoutCssMathProperty::RowGap, 0.25),
                (2, LayoutCssMathProperty::ColumnGap, 0.1),
            ],
            expected_grid: (120.0, 30.0),
            expected_gaps: (17.0, 12.5),
            expected_second_column_x: 57.0,
            expected_second_row_y: 22.5,
            expected_overflow: (0.0, 2.5),
        },
    ];

    for (index, case) in cases.iter().enumerate() {
        run_case(case, 20261020 + index as u64);
    }
}
