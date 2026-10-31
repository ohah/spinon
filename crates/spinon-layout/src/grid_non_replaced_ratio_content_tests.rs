use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridPlacementMode, GridTrackSize, LayoutBorder, LayoutBoxSizing,
    LayoutDimension, LayoutDisplay, LayoutEdges, LayoutGridTrackSizingAxis, LayoutInput,
    LayoutLengthPercentage, LayoutStyle, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine,
    Viewport, WhiteSpaceMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

struct Case {
    id: &'static str,
    ratio: Option<f32>,
    content_box_ratio: bool,
    min_width: LayoutDimension,
    max_width: LayoutDimension,
    min_height: LayoutDimension,
    max_height: LayoutDimension,
    edges: bool,
    columns: [f32; 2],
    row: f32,
    item: (f32, f32),
}

fn fixture(owner: u64) -> (HostDocument, HostNodeHandle, [HostNodeHandle; 2]) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let items = [
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
    ];
    let texts = [
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
    ];
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "main".to_owned(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    for ((item, text), value) in items.into_iter().zip(texts).zip(["AAAA", "A"]) {
        batch.push(DocumentOperation::CreateElement {
            node: item,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: item,
            before: None,
        });
        batch.push(DocumentOperation::CreateText {
            node: text,
            data: value.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(item),
            node: text,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, items)
}

fn run(case: &Case, owner: u64) {
    let (document, root, items) = fixture(owner);
    let snapshot = document.snapshot();
    // LayoutStyle's ContentBox mode accepts min/max values in content-box units. The Chrome
    // oracle authors these constraints with box-sizing:border-box, so normalize their values
    // before feeding this lower-level layout fixture.
    let horizontal_edges = if case.edges { 10.0 } else { 0.0 };
    let vertical_edges = if case.edges { 6.0 } else { 0.0 };
    let constraint = |value, edges| {
        if case.content_box_ratio {
            into_content_box(value, edges)
        } else {
            value
        }
    };
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(100.0),
        height: LayoutDimension::Auto,
        ..LayoutStyle::default()
    };
    let item_style = LayoutStyle {
        display: LayoutDisplay::Block,
        min_width: constraint(case.min_width, horizontal_edges),
        max_width: constraint(case.max_width, horizontal_edges),
        min_height: constraint(case.min_height, vertical_edges),
        max_height: constraint(case.max_height, vertical_edges),
        aspect_ratio: case.ratio,
        box_sizing: if case.content_box_ratio {
            LayoutBoxSizing::ContentBox
        } else {
            LayoutBoxSizing::BorderBox
        },
        padding: if case.edges {
            LayoutEdges {
                top: LayoutLengthPercentage::LengthPx(2.0),
                right: LayoutLengthPercentage::LengthPx(4.0),
                bottom: LayoutLengthPercentage::LengthPx(2.0),
                left: LayoutLengthPercentage::LengthPx(4.0),
            }
        } else {
            LayoutEdges::default()
        },
        border: if case.edges {
            LayoutBorder {
                top: 1.0,
                right: 1.0,
                bottom: 1.0,
                left: 1.0,
            }
        } else {
            LayoutBorder::default()
        },
        ..LayoutStyle::default()
    };
    let styles = BTreeMap::from([
        (root.id(), root_style),
        (items[0].id(), item_style),
        (
            items[1].id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let paragraph = ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: WhiteSpaceMode::Normal,
        ..ParagraphStyle::default()
    };
    let paragraphs = BTreeMap::from([
        (items[0].id(), paragraph.clone()),
        (items[1].id(), paragraph),
    ]);
    let input = LayoutInput::from_host_document_with_viewport_containing_block_and_text_paragraphs_ignoring_collapsible_whitespace(
        &snapshot,
        root,
        Viewport {
            width: 400.0,
            height: 200.0,
        },
        &styles,
        &paragraphs,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::MaxContent, GridTrackSize::MaxContent],
                rows: vec![GridTrackSize::Auto],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    close(
        output.frames[&root.id()].height,
        case.row,
        case.id,
        "root.height",
    );
    for (index, expected) in [case.item, (10.0, 10.0)].into_iter().enumerate() {
        let frame = output.frames[&items[index].id()];
        close(frame.width, expected.0, case.id, "item.width");
        if index == 0 {
            close(frame.height, expected.1, case.id, "item.height");
        }
    }
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: missing track trace", case.id));
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing columns pass", case.id))
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: missing final columns stage", case.id))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(columns.len(), 2, "{}: column track count", case.id);
    for (actual, expected) in columns.iter().zip(case.columns) {
        close(*actual, expected, case.id, "column track");
    }
}

fn close(actual: f32, expected: f32, case: &str, field: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {field} expected {expected}, got {actual}"
    );
}

fn auto() -> LayoutDimension {
    LayoutDimension::Auto
}

fn px(value: f32) -> LayoutDimension {
    LayoutDimension::Fixed(value)
}

fn into_content_box(value: LayoutDimension, edges: f32) -> LayoutDimension {
    match value {
        LayoutDimension::Fixed(value) => LayoutDimension::Fixed((value - edges).max(0.0)),
        _ => value,
    }
}

#[test]
fn direct_text_ratio_min_max_cases_match_the_pinned_chrome_geometry() {
    let cases = [
        Case {
            id: "numeric-baseline",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: auto(),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [40.0, 10.0],
            row: 20.0,
            item: (40.0, 20.0),
        },
        Case {
            id: "min-width-60",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "min-width-45",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(45.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [45.0, 10.0],
            row: 22.5,
            item: (45.0, 22.5),
        },
        Case {
            id: "max-width-30-min-width-0",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(0.0),
            max_width: px(30.0),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [30.0, 10.0],
            row: 15.0,
            item: (30.0, 15.0),
        },
        Case {
            id: "min-height-30",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: auto(),
            max_width: auto(),
            min_height: px(30.0),
            max_height: auto(),
            edges: false,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "max-height-15-min-height-0",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: auto(),
            max_width: auto(),
            min_height: px(0.0),
            max_height: px(15.0),
            edges: false,
            columns: [40.0, 10.0],
            row: 15.0,
            item: (40.0, 15.0),
        },
        Case {
            id: "max-height-15-min-height-0-min-width-0",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(0.0),
            max_width: auto(),
            min_height: px(0.0),
            max_height: px(15.0),
            edges: false,
            columns: [30.0, 10.0],
            row: 15.0,
            item: (30.0, 15.0),
        },
        Case {
            id: "max-height-17-75-min-controls",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(0.0),
            max_width: auto(),
            min_height: px(0.0),
            max_height: px(17.75),
            edges: false,
            columns: [35.5, 10.0],
            row: 17.75,
            item: (35.5, 17.75),
        },
        Case {
            id: "min-width-60-clamped-by-max-height-20",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: px(20.0),
            edges: false,
            columns: [60.0, 10.0],
            row: 20.0,
            item: (60.0, 20.0),
        },
        Case {
            id: "min-height-20-clamps-transferred-max-width-30",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: auto(),
            max_width: px(30.0),
            min_height: px(20.0),
            max_height: auto(),
            edges: false,
            columns: [30.0, 10.0],
            row: 20.0,
            item: (30.0, 20.0),
        },
        Case {
            id: "source-min-width-60-over-max-width-40",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(60.0),
            max_width: px(40.0),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "source-min-height-30-over-max-height-20",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: auto(),
            max_width: auto(),
            min_height: px(30.0),
            max_height: px(20.0),
            edges: false,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "auto-ratio-min-width-60",
            ratio: Some(2.0),
            content_box_ratio: true,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "no-ratio-min-width-60",
            ratio: None,
            content_box_ratio: false,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: false,
            columns: [60.0, 10.0],
            row: 10.0,
            item: (60.0, 10.0),
        },
        Case {
            id: "no-ratio-min-height-30",
            ratio: None,
            content_box_ratio: false,
            min_width: auto(),
            max_width: auto(),
            min_height: px(30.0),
            max_height: auto(),
            edges: false,
            columns: [40.0, 10.0],
            row: 30.0,
            item: (40.0, 30.0),
        },
        Case {
            id: "numeric-border-box-min-width-60",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: true,
            columns: [60.0, 10.0],
            row: 30.0,
            item: (60.0, 30.0),
        },
        Case {
            id: "auto-content-box-min-width-60",
            ratio: Some(2.0),
            content_box_ratio: true,
            min_width: px(60.0),
            max_width: auto(),
            min_height: auto(),
            max_height: auto(),
            edges: true,
            columns: [60.0, 10.0],
            row: 31.0,
            item: (60.0, 31.0),
        },
        Case {
            id: "numeric-border-box-max-height-15-min-controls",
            ratio: Some(2.0),
            content_box_ratio: false,
            min_width: px(0.0),
            max_width: auto(),
            min_height: px(0.0),
            max_height: px(15.0),
            edges: true,
            columns: [30.0, 10.0],
            row: 15.0,
            item: (30.0, 15.0),
        },
        Case {
            id: "auto-content-box-max-height-15-min-controls",
            ratio: Some(2.0),
            content_box_ratio: true,
            min_width: px(0.0),
            max_width: auto(),
            min_height: px(0.0),
            max_height: px(15.0),
            edges: true,
            columns: [28.0, 10.0],
            row: 15.0,
            item: (28.0, 15.0),
        },
    ];
    for (index, case) in cases.iter().enumerate() {
        run(case, 90_000 + index as u64);
    }
}
