use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Operation, StyleRevision, Tree};

use crate::{
    GridAutoFlow, GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackSize, LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutJustifyContent,
    LayoutStyle, ParagraphInput, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, TextSource,
    Viewport, WhiteSpaceMode,
};

const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

fn node(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

#[test]
fn grid_stretched_auto_height_text_box_may_exceed_paragraph_line_box() {
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .push(Operation::Create {
            id: node(1),
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: node(1),
            parent: None,
            index: 0,
        })
        .push(Operation::Create {
            id: node(2),
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: node(2),
            parent: Some(node(1)),
            index: 0,
        });
    tree.commit(batch).unwrap();

    let styles = BTreeMap::from([
        (
            node(1),
            LayoutStyle {
                display: LayoutDisplay::Grid,
                width: LayoutDimension::Fixed(160.0),
                height: LayoutDimension::Fixed(40.0),
                align_content: Some(LayoutAlignContent::FlexStart),
                justify_content: LayoutJustifyContent::FlexStart,
                ..LayoutStyle::default()
            },
        ),
        (
            node(2),
            LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Auto,
                height: LayoutDimension::Auto,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let mut input = crate::LayoutInput::from_tree(
        &tree,
        Viewport {
            width: 160.0,
            height: 40.0,
        },
        &styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            node(1),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(24.0), GridTrackSize::MinContent],
                rows: vec![GridTrackSize::LengthPx(40.0)],
                auto_flow: GridAutoFlow::Column,
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(
            node(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(2),
                column_end: GridPlacementInput::Line(3),
            },
        )]),
    );
    input = input
        .with_text_paragraphs([ParagraphInput {
            node: node(2),
            sources: vec![TextSource {
                node: node(3),
                inline_ancestors: Vec::new(),
                text: "XX XXXXXXX".to_owned(),
            }],
            style: ParagraphStyle {
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                ..ParagraphStyle::default()
            },
            inline_styles: BTreeMap::new(),
            inline_run_styles: BTreeMap::new(),
        }])
        .unwrap();

    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .unwrap();

    let item = output.frames[&node(2)];
    assert_eq!(
        (item.x, item.y, item.width, item.height),
        (24.0, 0.0, 70.0, 40.0)
    );
    assert_eq!(output.text_layouts[&node(2)].layout.height, 20.0);
}

#[test]
fn grid_auto_height_with_zero_min_height_may_overflow_text_measure() {
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .push(Operation::Create {
            id: node(1),
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: node(1),
            parent: None,
            index: 0,
        })
        .push(Operation::Create {
            id: node(2),
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: node(2),
            parent: Some(node(1)),
            index: 0,
        });
    tree.commit(batch).unwrap();

    let styles = BTreeMap::from([
        (
            node(1),
            LayoutStyle {
                display: LayoutDisplay::Grid,
                width: LayoutDimension::Fixed(48.0),
                height: LayoutDimension::Fixed(20.0),
                ..LayoutStyle::default()
            },
        ),
        (
            node(2),
            LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Auto,
                height: LayoutDimension::Auto,
                min_height: LayoutDimension::Fixed(0.0),
                ..LayoutStyle::default()
            },
        ),
    ]);
    let mut input = crate::LayoutInput::from_tree(
        &tree,
        Viewport {
            width: 48.0,
            height: 20.0,
        },
        &styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            node(1),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(48.0)],
                rows: vec![GridTrackSize::LengthPx(20.0)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(
            node(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        )]),
    );
    input = input
        .with_text_paragraphs([ParagraphInput {
            node: node(2),
            sources: vec![TextSource {
                node: node(3),
                inline_ancestors: Vec::new(),
                text: "XX XXXXX".to_owned(),
            }],
            style: ParagraphStyle {
                font_family: "Ahem".to_owned(),
                font_size: 12.0,
                line_height: 12.0,
                white_space: WhiteSpaceMode::Normal,
                ..ParagraphStyle::default()
            },
            inline_styles: BTreeMap::new(),
            inline_run_styles: BTreeMap::new(),
        }])
        .unwrap();

    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .unwrap();

    assert_eq!(output.frames[&node(2)].height, 20.0);
    assert_eq!(output.text_layouts[&node(2)].layout.height, 24.0);
}
