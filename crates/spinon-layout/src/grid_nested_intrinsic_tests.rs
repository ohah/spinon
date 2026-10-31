use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
    LayoutDimension, LayoutDisplay, LayoutGap, LayoutGridItemIntrinsicSource,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind, LayoutInput,
    LayoutLengthPercentage, LayoutStyle, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine,
    Viewport, WhiteSpaceMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

#[derive(Clone)]
struct NestedGridCase {
    id: &'static str,
    root_width: LayoutDimension,
    outer_columns: Vec<GridTrackSize>,
    nested_column_end: i32,
    expected_outer_width: f32,
    expected_outer_columns: &'static [f32],
    expected_nested_width: f32,
}

struct NestedGridDocument {
    document: HostDocument,
    root: HostNodeHandle,
    nested: HostNodeHandle,
    items: [HostNodeHandle; 2],
}

fn nested_grid_document(owner: u64) -> NestedGridDocument {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let first_item = document.reserve_node_handle().unwrap();
    let second_item = document.reserve_node_handle().unwrap();
    let first_text = document.reserve_node_handle().unwrap();
    let second_text = document.reserve_node_handle().unwrap();

    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, local_name) in [
        (root, "main"),
        (nested, "div"),
        (first_item, "div"),
        (second_item, "div"),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: local_name.to_owned(),
        });
    }
    for (node, text) in [(first_text, "XX XXXXXXX"), (second_text, "XXXXXX")] {
        batch.push(DocumentOperation::CreateText {
            node,
            data: text.into(),
        });
    }
    for (parent, node) in [
        (HostParent::Root, root),
        (HostParent::Node(root), nested),
        (HostParent::Node(nested), first_item),
        (HostParent::Node(nested), second_item),
        (HostParent::Node(first_item), first_text),
        (HostParent::Node(second_item), second_text),
    ] {
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    NestedGridDocument {
        document,
        root,
        nested,
        items: [first_item, second_item],
    }
}

fn gap(column: f32) -> LayoutGap {
    LayoutGap {
        row: LayoutLengthPercentage::ZERO,
        column: LayoutLengthPercentage::LengthPx(column),
    }
}

fn run_case(case: &NestedGridCase, owner: u64) {
    let fixture = nested_grid_document(owner);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: case.root_width,
        height: LayoutDimension::Fixed(20.0),
        align_content: Some(LayoutAlignContent::FlexStart),
        gap: gap(4.0),
        ..LayoutStyle::default()
    };
    let nested_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        height: LayoutDimension::Fixed(20.0),
        gap: gap(4.0),
        ..LayoutStyle::default()
    };
    let item_styles = fixture.items.map(|item| {
        (
            item.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        )
    });
    let styles = BTreeMap::from([
        (fixture.root.id(), root_style),
        (fixture.nested.id(), nested_style),
        item_styles[0],
        item_styles[1],
    ]);
    let paragraph_style = ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: WhiteSpaceMode::Normal,
        ..ParagraphStyle::default()
    };
    let paragraph_styles = BTreeMap::from([
        (fixture.items[0].id(), paragraph_style.clone()),
        (fixture.items[1].id(), paragraph_style),
    ]);
    let input = LayoutInput::from_host_document_with_viewport_containing_block_and_text_paragraphs_ignoring_collapsible_whitespace(
        &fixture.document.snapshot(),
        fixture.root,
        Viewport {
            width: 400.0,
            height: 200.0,
        },
        &styles,
        &paragraph_styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([
            (
                fixture.root.id(),
                GridContainerInput {
                    columns: case.outer_columns.clone(),
                    rows: vec![GridTrackSize::MaxContent],
                    placement_mode: GridPlacementMode::TrackSizing,
                    ..GridContainerInput::default()
                },
            ),
            (
                fixture.nested.id(),
                GridContainerInput {
                    columns: vec![GridTrackSize::MinContent, GridTrackSize::MaxContent],
                    rows: vec![GridTrackSize::MaxContent],
                    placement_mode: GridPlacementMode::TrackSizing,
                    ..GridContainerInput::default()
                },
            ),
        ]),
        BTreeMap::from([(
            fixture.nested.id(),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(case.nested_column_end),
            },
        )]),
    );
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));

    let root_frame = output.frames[&fixture.root.id()];
    let nested_frame = output.frames[&fixture.nested.id()];
    assert_close(
        root_frame.width,
        case.expected_outer_width,
        case.id,
        "outer.width",
    );
    assert_close(
        nested_frame.width,
        case.expected_nested_width,
        case.id,
        "nested.width",
    );
    assert_close(root_frame.height, 20.0, case.id, "outer.height");
    assert_close(nested_frame.height, 20.0, case.id, "nested.height");

    let outer_trace = trace_for(&output, fixture.root.id(), case.id);
    let nested_trace = trace_for(&output, fixture.nested.id(), case.id);
    assert_tracks(
        outer_trace,
        LayoutGridTrackSizingAxis::Columns,
        case.expected_outer_columns,
        case.id,
    );
    assert_tracks(
        nested_trace,
        LayoutGridTrackSizingAxis::Columns,
        &[70.0, 60.0],
        case.id,
    );

    for (item, expected_width) in fixture.items.iter().zip([70.0, 60.0]) {
        let frame = output.frames[&item.id()];
        assert_close(frame.width, expected_width, case.id, "inner item width");
        assert_close(frame.height, 20.0, case.id, "inner item height");
    }
    assert_close(
        output.frames[&fixture.items[1].id()].x - nested_frame.x,
        74.0,
        case.id,
        "second inner item x",
    );

    let leaf_inputs = output
        .grid_item_intrinsic_inputs
        .iter()
        .filter(|item| item.grid_container == fixture.nested.id())
        .collect::<Vec<_>>();
    assert_eq!(leaf_inputs.len(), 2, "{}: inner leaf inputs", case.id);
    assert!(
        output
            .grid_item_intrinsic_inputs
            .iter()
            .all(|item| item.node != fixture.nested.id()),
        "{}: nested Grid is not a text/image leaf input",
        case.id
    );
    for (item, (expected_min, expected_max)) in
        leaf_inputs.iter().zip([(70.0, 100.0), (60.0, 60.0)])
    {
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{}: inner leaf must retain text intrinsic source", case.id);
        };
        assert_close(
            sizes.min_content(),
            expected_min,
            case.id,
            "leaf min-content",
        );
        assert_close(
            sizes.max_content(),
            expected_max,
            case.id,
            "leaf max-content",
        );
    }

    assert!(
        output
            .intrinsic_measurements
            .iter()
            .all(|measurement| measurement.node != fixture.nested.id()),
        "{}: non-leaf nested Grid must not be reported as a text/replaced leaf measurement",
        case.id
    );

    let outer_columns = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap();
    if case.id != "definite-flex-control" {
        assert!(
            outer_columns.contributions.iter().any(|contribution| {
                contribution.item == fixture.nested.id()
                    && contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                    && contribution.span == (case.nested_column_end - 1) as u16
                    && (contribution.value - 134.0).abs() <= 1.0 / 64.0
            }),
            "{}: outer contribution trace must identify the nested Grid and its 134px min-content value: {:#?}",
            case.id,
            outer_columns.contributions
        );
    } else {
        assert!(
            outer_columns.contributions.iter().any(|contribution| {
                contribution.item == fixture.nested.id()
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                    && contribution.span == (case.nested_column_end - 1) as u16
                    && contribution.value.abs() <= 1.0 / 64.0
            }),
            "{}: definite flex control must record the nested Grid's zero minimum contribution: {:#?}",
            case.id,
            outer_columns.contributions
        );
        assert!(
            outer_columns.contributions.iter().all(|contribution| {
                contribution.item != fixture.nested.id()
                    || (contribution.value >= 0.0 && contribution.value <= 120.0)
            }),
            "{}: overflowing inner tracks must not become parent contribution: {:#?}",
            case.id,
            outer_columns.contributions
        );
    }

    let recorded_spans = outer_columns
        .stages
        .iter()
        .filter_map(|stage| match stage.stage {
            crate::LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    assert!(
        recorded_spans.contains(&((case.nested_column_end - 1) as u16)),
        "{}: outer Grid trace retains the nested item's span",
        case.id
    );
}

fn trace_for<'a>(
    output: &'a crate::LayoutOutput,
    node: spinon_core::NodeId,
    case: &str,
) -> &'a crate::LayoutGridTrackSizingTrace {
    output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == node)
        .unwrap_or_else(|| panic!("{case}: missing Grid trace for node {}", node.get()))
}

fn assert_tracks(
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
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final {axis:?} stage"));
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len(), "{case}: {axis:?} track count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(*actual, *expected, case, &format!("{axis:?}[{index}]"));
    }
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

#[test]
fn nested_grid_intrinsic_contributions_match_chrome_without_leaf_fallback() {
    let cases = [
        NestedGridCase {
            id: "span-two",
            root_width: LayoutDimension::MaxContent,
            outer_columns: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
            nested_column_end: 3,
            expected_outer_width: 134.0,
            expected_outer_columns: &[65.0, 65.0],
            expected_nested_width: 134.0,
        },
        NestedGridCase {
            id: "one-track",
            root_width: LayoutDimension::MaxContent,
            outer_columns: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
            nested_column_end: 2,
            expected_outer_width: 138.0,
            expected_outer_columns: &[134.0, 0.0],
            expected_nested_width: 134.0,
        },
        NestedGridCase {
            id: "definite-flex-control",
            root_width: LayoutDimension::Fixed(120.0),
            outer_columns: vec![
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::Auto,
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                },
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::Auto,
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                },
            ],
            nested_column_end: 3,
            expected_outer_width: 120.0,
            expected_outer_columns: &[58.0, 58.0],
            expected_nested_width: 120.0,
        },
    ];
    for (index, case) in cases.iter().enumerate() {
        run_case(case, 2_000 + index as u64);
    }
}
