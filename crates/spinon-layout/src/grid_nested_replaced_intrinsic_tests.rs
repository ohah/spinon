use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignItems,
    LayoutDimension, LayoutDisplay, LayoutGap, LayoutGridItemIntrinsicSource,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind, LayoutIntrinsicSize,
    LayoutIntrinsicSizeSnapshot, LayoutLengthPercentage, LayoutSourceRevision, LayoutStyle,
    ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, Viewport, WhiteSpaceMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

#[derive(Clone)]
struct Case {
    id: &'static str,
    width: LayoutDimension,
    columns: Vec<GridTrackSize>,
    nested_column_end: i32,
    outer_width: f32,
    outer_tracks: &'static [f32],
    nested_width: f32,
    contribution_span: u16,
    flex_control: bool,
}

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nested: HostNodeHandle,
    text_item: HostNodeHandle,
    image: HostNodeHandle,
}

fn fixture(owner: u64) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let text_item = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, tag, parent) in [
        (root, "main", HostParent::Root),
        (nested, "div", HostParent::Node(root)),
        (text_item, "div", HostParent::Node(nested)),
        (image, "img", HostParent::Node(nested)),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: tag.to_owned(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "src").unwrap(),
            value: "https://spinon.invalid/c14-1-png-basic.png".into(),
        })
        .push(DocumentOperation::CreateText {
            node: text,
            data: "XX XXXXXXX".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(text_item),
            node: text,
            before: None,
        });
    document.commit(batch).unwrap();
    Fixture {
        document,
        root,
        nested,
        text_item,
        image,
    }
}

fn columns(trace: &crate::LayoutGridTrackSizingTrace, expected: &[f32], case: &str) {
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{case}: missing Columns pass"));
    let actual = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final Columns stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len(), "{case}: Columns track count");
    for (actual, expected) in actual.iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 1.0 / 64.0,
            "{case}: column expected {expected}, got {actual}"
        );
    }
}

fn run(case: &Case, owner: u64) {
    let fixture = fixture(owner);
    let snapshot = fixture.document.snapshot();
    let source = LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    };
    let intrinsic = LayoutIntrinsicSizeSnapshot::new(
        source,
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [(fixture.image.id(), LayoutIntrinsicSize::new(24.0, 16.0))],
    )
    .unwrap();
    let gap = LayoutGap {
        row: LayoutLengthPercentage::ZERO,
        column: LayoutLengthPercentage::LengthPx(4.0),
    };
    let styles = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                display: LayoutDisplay::Grid,
                width: case.width,
                height: LayoutDimension::Fixed(20.0),
                gap,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.nested.id(),
            LayoutStyle {
                display: LayoutDisplay::Grid,
                height: LayoutDimension::Fixed(20.0),
                align_items: LayoutAlignItems::FlexStart,
                gap,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.text_item.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.image.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let paragraphs = BTreeMap::from([(
        fixture.text_item.id(),
        ParagraphStyle {
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::Normal,
            ..ParagraphStyle::default()
        },
    )]);
    let input = crate::LayoutInput::from_host_document_with_viewport_containing_block_and_text_paragraphs_ignoring_collapsible_whitespace(
        &snapshot,
        fixture.root,
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
        BTreeMap::from([
            (
                fixture.root.id(),
                GridContainerInput {
                    columns: case.columns.clone(),
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
        BTreeMap::from([
            (
                fixture.nested.id(),
                GridItemInput {
                    row_start: GridPlacementInput::Line(1),
                    row_end: GridPlacementInput::Line(2),
                    column_start: GridPlacementInput::Line(1),
                    column_end: if case.nested_column_end == 3 {
                        GridPlacementInput::Line(3)
                    } else {
                        GridPlacementInput::Line(2)
                    },
                },
            ),
            (
                fixture.text_item.id(),
                GridItemInput {
                    row_start: GridPlacementInput::Line(1),
                    row_end: GridPlacementInput::Line(2),
                    column_start: GridPlacementInput::Line(1),
                    column_end: GridPlacementInput::Line(2),
                },
            ),
            (
                fixture.image.id(),
                GridItemInput {
                    row_start: GridPlacementInput::Line(1),
                    row_end: GridPlacementInput::Line(2),
                    column_start: GridPlacementInput::Line(2),
                    column_end: GridPlacementInput::Line(3),
                },
            ),
        ]),
    );
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot_and_text_layout_engine(
            &input,
            &intrinsic,
            &text_engine,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let frame = |node: HostNodeHandle| output.frames[&node.id()];
    assert_eq!(output.revision.intrinsic_size(), intrinsic.revision());
    assert!((frame(fixture.root).width - case.outer_width).abs() <= 1.0 / 64.0);
    assert!((frame(fixture.root).height - 20.0).abs() <= 1.0 / 64.0);
    assert!((frame(fixture.nested).width - case.nested_width).abs() <= 1.0 / 64.0);
    assert!((frame(fixture.text_item).width - 70.0).abs() <= 1.0 / 64.0);
    assert!((frame(fixture.text_item).height - 20.0).abs() <= 1.0 / 64.0);
    assert!((frame(fixture.image).x - frame(fixture.nested).x - 74.0).abs() <= 1.0 / 64.0);
    assert_eq!(
        (frame(fixture.image).width, frame(fixture.image).height),
        (24.0, 16.0)
    );

    let inputs = &output.grid_item_intrinsic_inputs;
    assert_eq!(inputs.len(), 2, "{}: inner Grid leaf count", case.id);
    assert_eq!(inputs[0].node, fixture.text_item.id());
    assert_eq!(inputs[1].node, fixture.image.id());
    assert!(
        inputs
            .iter()
            .all(|input| input.grid_container == fixture.nested.id())
    );
    assert!(inputs.iter().all(|input| input.revision == output.revision));
    let LayoutGridItemIntrinsicSource::Text { sizes } = &inputs[0].source else {
        panic!("{}: text input source", case.id);
    };
    assert!((sizes.min_content() - 70.0).abs() <= 1.0 / 64.0);
    assert!((sizes.max_content() - 100.0).abs() <= 1.0 / 64.0);
    let LayoutGridItemIntrinsicSource::Replaced { natural } = inputs[1].source else {
        panic!("{}: image input source", case.id);
    };
    assert_eq!((natural.width, natural.height), (Some(24.0), Some(16.0)));

    let outer_trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap();
    let nested_trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.nested.id())
        .unwrap();
    columns(outer_trace, case.outer_tracks, case.id);
    columns(nested_trace, &[70.0, 24.0], case.id);
    let contributions = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap()
        .contributions
        .iter()
        .filter(|contribution| contribution.item == fixture.nested.id())
        .collect::<Vec<_>>();
    if case.flex_control {
        assert!(contributions.iter().any(|contribution| {
            contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                && contribution.span == case.contribution_span
                && contribution.value.abs() <= 1.0 / 64.0
        }));
    } else {
        assert!(contributions.iter().any(|contribution| {
            contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                && contribution.span == case.contribution_span
                && (contribution.value - 98.0).abs() <= 1.0 / 64.0
        }));
    }
}

#[test]
fn nested_grid_text_and_replaced_contributions_match_chrome_oracle() {
    let cases = [
        Case {
            id: "span-two",
            width: LayoutDimension::MaxContent,
            columns: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
            nested_column_end: 3,
            outer_width: 98.0,
            outer_tracks: &[47.0, 47.0],
            nested_width: 98.0,
            contribution_span: 2,
            flex_control: false,
        },
        Case {
            id: "one-track",
            width: LayoutDimension::MaxContent,
            columns: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
            nested_column_end: 2,
            outer_width: 102.0,
            outer_tracks: &[98.0, 0.0],
            nested_width: 98.0,
            contribution_span: 1,
            flex_control: false,
        },
        Case {
            id: "definite-flex-control",
            width: LayoutDimension::Fixed(120.0),
            columns: vec![
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
            outer_width: 120.0,
            outer_tracks: &[58.0, 58.0],
            nested_width: 120.0,
            contribution_span: 2,
            flex_control: true,
        },
    ];
    for (index, case) in cases.iter().enumerate() {
        run(case, 2_100 + index as u64);
    }
}
