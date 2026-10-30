use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostDocumentSnapshot, HostNodeHandle, HostParent, OwnerId, StyleRevision,
};

use crate::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackSize, LayoutDimension, LayoutDisplay, LayoutError, LayoutGridItemIntrinsicSource,
    LayoutInput, LayoutStyle, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "c11_4_anonymous_text/white_space.rs"]
mod white_space;

#[derive(Clone, Copy)]
enum Child<'a> {
    Text(&'a str),
    Block { id: &'a str, width: f32 },
    Hidden { id: &'a str, text: &'a str },
}

fn fixture(
    children: &[Child<'_>],
    height: f32,
) -> (
    HostDocument,
    HostNodeHandle,
    Vec<HostNodeHandle>,
    BTreeMap<spinon_core::NodeId, LayoutStyle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    let mut handles = Vec::with_capacity(children.len());
    let mut styles = BTreeMap::from([(
        root.id(),
        LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(120.0),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    )]);
    for child in children {
        let handle = document.reserve_node_handle().unwrap();
        match child {
            Child::Text(text) => {
                batch.push(DocumentOperation::CreateText {
                    node: handle,
                    data: (*text).into(),
                });
            }
            Child::Block { id: _, width } => {
                batch.push(DocumentOperation::CreateElement {
                    node: handle,
                    namespace: HTML.to_owned(),
                    local_name: "div".to_owned(),
                });
                styles.insert(
                    handle.id(),
                    LayoutStyle {
                        display: LayoutDisplay::Block,
                        width: LayoutDimension::Fixed(*width),
                        height: LayoutDimension::Fixed(10.0),
                        ..LayoutStyle::default()
                    },
                );
            }
            Child::Hidden { id: _, text } => {
                batch.push(DocumentOperation::CreateElement {
                    node: handle,
                    namespace: HTML.to_owned(),
                    local_name: "span".to_owned(),
                });
                styles.insert(
                    handle.id(),
                    LayoutStyle {
                        display: LayoutDisplay::None,
                        ..LayoutStyle::default()
                    },
                );
                let hidden_text = document.reserve_node_handle().unwrap();
                batch
                    .push(DocumentOperation::CreateText {
                        node: hidden_text,
                        data: (*text).into(),
                    })
                    .push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(handle),
                        node: hidden_text,
                        before: None,
                    });
            }
        }
        if let Child::Block { id, .. } = child {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "id").unwrap(),
                value: (*id).into(),
            });
        }
        if let Child::Hidden { id, .. } = child {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "id").unwrap(),
                value: (*id).into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: handle,
            before: None,
        });
        handles.push(handle);
    }
    document.commit(batch).unwrap();
    (document, root, handles, styles)
}

fn input(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<spinon_core::NodeId, LayoutStyle>,
    paragraph: ParagraphStyle,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
) -> LayoutInput {
    input_result(snapshot, root, styles, paragraph, columns, rows)
        .expect("valid C11.4 anonymous Grid text input")
}

fn input_result(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<spinon_core::NodeId, LayoutStyle>,
    paragraph: ParagraphStyle,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
) -> Result<LayoutInput, LayoutError> {
    let input = LayoutInput::from_host_document_with_viewport_containing_block_and_c11_4_anonymous_grid_text_items(
        snapshot,
        root,
        Viewport {
            width: 120.0,
            height: 40.0,
        },
        styles,
        crate::HostTextStyleProfile {
            paragraphs: &BTreeMap::from([(root.id(), paragraph)]),
            inline_runs: &BTreeMap::new(),
        },
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )?;
    Ok(input.with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns,
                rows,
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::Auto,
                auto_columns: GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    ))
}

fn paragraph_style() -> ParagraphStyle {
    paragraph_style_with_white_space(crate::WhiteSpaceMode::Normal)
}

fn paragraph_style_with_white_space(white_space: crate::WhiteSpaceMode) -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space,
        ..ParagraphStyle::default()
    }
}

fn ahem_engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

fn assert_frame(
    frames: &BTreeMap<spinon_core::NodeId, crate::LayoutFrame>,
    node: spinon_core::NodeId,
    expected: (f32, f32, f32, f32),
) {
    let frame = frames[&node];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        expected,
        "frame for node {}",
        node.get()
    );
}

#[test]
fn adjacent_text_nodes_become_one_anonymous_item_with_full_provenance() {
    let (document, root, children, styles) = fixture(&[Child::Text("XX"), Child::Text("XX")], 10.0);
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MaxContent, GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    let grid = input
        .nodes()
        .iter()
        .find(|node| node.id == root.id())
        .unwrap();
    assert_eq!(grid.children, vec![children[0].id()]);

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 1);
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[0].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX"), (children[1].id(), "XX")]
    );
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (40.0, 40.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 40.0, 10.0));
}

#[test]
fn leading_collapsible_text_keeps_provenance_but_anchors_item_to_first_content_node() {
    let (document, root, children, styles) = fixture(
        &[Child::Text(""), Child::Text(" \t\n"), Child::Text("XX")],
        10.0,
    );
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MinContent, GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    let grid = input
        .nodes()
        .iter()
        .find(|node| node.id == root.id())
        .unwrap();
    assert_eq!(grid.children, vec![children[2].id()]);

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 1);
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[2].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (children[0].id(), ""),
            (children[1].id(), " \t\n"),
            (children[2].id(), "XX"),
        ]
    );
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (20.0, 20.0));
    assert_frame(&output.frames, children[2].id(), (0.0, 0.0, 20.0, 10.0));
}

#[test]
fn non_fixture_host_document_projection_still_rejects_direct_grid_text() {
    let (document, root, children, styles) = fixture(&[Child::Text("XX")], 10.0);
    let error = LayoutInput::from_host_document_with_viewport_containing_block(
        &document.snapshot(),
        root,
        Viewport {
            width: 120.0,
            height: 40.0,
        },
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        LayoutError::UnsupportedTextNode(node) if node == children[0].id()
    ));
}

#[test]
fn direct_block_child_splits_anonymous_runs_without_losing_source_order() {
    let (document, root, children, styles) = fixture(
        &[
            Child::Text("XX"),
            Child::Block {
                id: "separator",
                width: 30.0,
            },
            Child::Text("XXXX"),
        ],
        10.0,
    );
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MinContent; 3],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    let grid = input
        .nodes()
        .iter()
        .find(|node| node.id == root.id())
        .unwrap();
    assert_eq!(
        grid.children,
        vec![children[0].id(), children[1].id(), children[2].id()]
    );

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 2);
    assert_eq!(
        output
            .grid_item_intrinsic_inputs
            .iter()
            .map(|item| (item.node, item.source_order))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), 0), (children[2].id(), 2)]
    );
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 10.0));
    assert_frame(&output.frames, children[1].id(), (20.0, 0.0, 30.0, 10.0));
    assert_frame(&output.frames, children[2].id(), (50.0, 0.0, 40.0, 10.0));
}

#[test]
fn direct_display_none_child_does_not_split_anonymous_text_run() {
    let (document, root, children, styles) = fixture(
        &[
            Child::Text("XX"),
            Child::Hidden {
                id: "hidden-separator",
                text: "ignored",
            },
            Child::Text("XX"),
        ],
        10.0,
    );
    let snapshot = document.snapshot();
    let hidden_text = snapshot.children(children[1]).unwrap().next().unwrap();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MaxContent, GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    let grid = input
        .nodes()
        .iter()
        .find(|node| node.id == root.id())
        .unwrap();
    assert_eq!(grid.children, vec![children[0].id()]);

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 1);
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[0].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX"), (children[2].id(), "XX")]
    );
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (40.0, 40.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 40.0, 10.0));
    assert!(!output.frames.contains_key(&children[1].id()));
    assert!(!output.frames.contains_key(&hidden_text.id()));
}

#[test]
fn anonymous_grid_text_still_rejects_nonzero_order() {
    let (document, root, _, mut styles) = fixture(&[Child::Text("XX\nXX")], 10.0);
    styles.get_mut(&root.id()).unwrap().order = 1;
    let snapshot = document.snapshot();

    let reordered = input_result(
        &snapshot,
        root,
        &styles,
        paragraph_style_with_white_space(crate::WhiteSpaceMode::PreLine),
        vec![GridTrackSize::MinContent],
        vec![GridTrackSize::Auto],
    )
    .unwrap_err();
    assert!(
        matches!(reordered, LayoutError::UnsupportedTextParagraph { node, .. } if node == root.id())
    );
}

#[test]
fn anonymous_grid_pre_wrap_still_rejects_nonzero_order() {
    let (document, root, _, mut styles) = fixture(&[Child::Text("XX  XX")], 10.0);
    styles.get_mut(&root.id()).unwrap().order = 1;
    let snapshot = document.snapshot();

    let reordered = input_result(
        &snapshot,
        root,
        &styles,
        paragraph_style_with_white_space(crate::WhiteSpaceMode::PreWrap),
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    )
    .unwrap_err();
    assert!(
        matches!(reordered, LayoutError::UnsupportedTextParagraph { node, .. } if node == root.id())
    );
}

#[test]
fn collapsible_whitespace_between_blocks_does_not_create_an_item() {
    let (document, root, children, styles) = fixture(
        &[
            Child::Block {
                id: "first",
                width: 10.0,
            },
            Child::Text(" \t\n"),
            Child::Block {
                id: "second",
                width: 10.0,
            },
        ],
        10.0,
    );
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    let grid = input
        .nodes()
        .iter()
        .find(|node| node.id == root.id())
        .unwrap();
    assert_eq!(grid.children, vec![children[0].id(), children[2].id()]);

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    assert!(output.grid_item_intrinsic_inputs.is_empty());
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 10.0, 10.0));
    assert_frame(&output.frames, children[2].id(), (20.0, 0.0, 10.0, 10.0));
}
