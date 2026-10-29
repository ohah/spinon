use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, Revision, StyleRevision, WritingMode};

use super::{node, text_engine};
use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode, GridTrackSize,
    ItemAlignmentPosition, LayoutAlignItems, LayoutAlignSelf, LayoutDimension, LayoutDisplay,
    LayoutGap, LayoutInput, LayoutInputRevision, LayoutLengthPercentage, LayoutNode,
    LayoutSourceRevision, LayoutStyle, ParagraphInput, ParagraphStyle, RootSizingPolicy,
    TaffyLayoutEngine, TextSource, Viewport, WhiteSpaceMode, WordBreakMode,
};

fn baseline_grid_input(
    container_alignment: LayoutAlignItems,
    item_alignment: LayoutAlignSelf,
    reverse_source_order: bool,
    small_text: &str,
    large_text: &str,
) -> LayoutInput {
    let grid = node(2);
    let small = node(3);
    let large = node(4);
    let child_style = LayoutStyle {
        display: LayoutDisplay::Block,
        width: LayoutDimension::Auto,
        height: LayoutDimension::Auto,
        align_self: item_alignment,
        ..LayoutStyle::default()
    };
    let mut input = LayoutInput {
        root: node(1),
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 68.0,
            height: 60.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes: vec![
            LayoutNode {
                id: node(1),
                children: vec![grid],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Fixed(68.0),
                    height: LayoutDimension::Fixed(60.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: grid,
                children: if reverse_source_order {
                    vec![large, small]
                } else {
                    vec![small, large]
                },
                style: LayoutStyle {
                    display: LayoutDisplay::Grid,
                    width: LayoutDimension::MaxContent,
                    align_items: container_alignment,
                    gap: LayoutGap {
                        column: LayoutLengthPercentage::length(8.0),
                        ..LayoutGap::default()
                    },
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: small,
                children: Vec::new(),
                style: child_style,
            },
            LayoutNode {
                id: large,
                children: Vec::new(),
                style: child_style,
            },
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            grid,
            GridContainerInput {
                columns: vec![GridTrackSize::Auto, GridTrackSize::Auto],
                rows: vec![GridTrackSize::Auto],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        grid_items: BTreeMap::from([
            (
                small,
                GridItemInput {
                    row_start: GridPlacementInput::Line(1),
                    row_end: GridPlacementInput::Line(2),
                    column_start: GridPlacementInput::Line(if reverse_source_order {
                        2
                    } else {
                        1
                    }),
                    column_end: GridPlacementInput::Line(if reverse_source_order { 3 } else { 2 }),
                },
            ),
            (
                large,
                GridItemInput {
                    row_start: GridPlacementInput::Line(1),
                    row_end: GridPlacementInput::Line(2),
                    column_start: GridPlacementInput::Line(if reverse_source_order {
                        1
                    } else {
                        2
                    }),
                    column_end: GridPlacementInput::Line(if reverse_source_order { 2 } else { 3 }),
                },
            ),
        ]),
        text_paragraphs: BTreeMap::new(),
    };
    input = input
        .with_text_paragraphs([
            paragraph(small, node(5), 10.0, small_text),
            paragraph(large, node(6), 20.0, large_text),
        ])
        .unwrap();
    input
}

fn paragraph(
    node_id: spinon_core::NodeId,
    source_id: spinon_core::NodeId,
    font_size: f32,
    text: &str,
) -> ParagraphInput {
    ParagraphInput {
        node: node_id,
        sources: vec![TextSource {
            node: source_id,
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            writing_mode: WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size,
            line_height: font_size,
            white_space: if text.contains('\n') {
                WhiteSpaceMode::PreLine
            } else {
                WhiteSpaceMode::NoWrap
            },
            word_break: WordBreakMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    }
}

#[test]
fn direct_ahem_text_grid_items_share_their_first_baseline() {
    let input = baseline_grid_input(
        LayoutAlignItems::FirstBaseline,
        LayoutAlignSelf::Auto,
        false,
        "XX",
        "XX",
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();
    let grid = output.frames[&node(2)];
    let small = output.frames[&node(3)];
    let large = output.frames[&node(4)];

    assert_eq!((grid.width, grid.height), (68.0, 20.0));
    assert_eq!(
        (small.x, small.y, small.width, small.height),
        (0.0, 8.0, 20.0, 10.0)
    );
    assert_eq!(
        (large.x, large.y, large.width, large.height),
        (28.0, 0.0, 40.0, 20.0)
    );
    assert_eq!(output.text_layouts[&node(3)].layout.lines[0].baseline, 8.0);
    assert_eq!(output.text_layouts[&node(4)].layout.lines[0].baseline, 16.0);
}

#[test]
fn direct_ahem_text_grid_items_honor_align_self_first_baseline() {
    let input = baseline_grid_input(
        LayoutAlignItems::Position {
            position: ItemAlignmentPosition::Start,
            safety: None,
        },
        LayoutAlignSelf::FirstBaseline,
        false,
        "XX",
        "XX",
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();

    assert_eq!(output.frames[&node(3)].y, 8.0);
    assert_eq!(output.frames[&node(4)].y, 0.0);
}

#[test]
fn grid_start_alignment_does_not_consume_text_baselines() {
    let input = baseline_grid_input(
        LayoutAlignItems::Position {
            position: ItemAlignmentPosition::Start,
            safety: None,
        },
        LayoutAlignSelf::Auto,
        false,
        "XX",
        "XX",
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();

    assert_eq!(output.frames[&node(3)].y, 0.0);
    assert_eq!(output.frames[&node(4)].y, 0.0);
}

#[test]
fn source_order_does_not_change_the_first_baseline_group() {
    let input = baseline_grid_input(
        LayoutAlignItems::FirstBaseline,
        LayoutAlignSelf::Auto,
        true,
        "XX",
        "XX",
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();

    assert_eq!(
        (output.frames[&node(3)].x, output.frames[&node(3)].y),
        (48.0, 8.0)
    );
    assert_eq!(
        (output.frames[&node(4)].x, output.frames[&node(4)].y),
        (0.0, 0.0)
    );
}

#[test]
fn multiline_text_first_baseline_is_rejected_without_partial_layout() {
    let input = baseline_grid_input(
        LayoutAlignItems::FirstBaseline,
        LayoutAlignSelf::Auto,
        false,
        "XX\nXX",
        "XX",
    );

    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &text_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph {
            node: item,
            reason: "C11.4 Grid first-baseline text requires exactly one line"
        }) if item == node(3)
    ));
}

#[test]
fn first_baseline_text_with_nonzero_box_edges_is_rejected() {
    let mut input = baseline_grid_input(
        LayoutAlignItems::FirstBaseline,
        LayoutAlignSelf::Auto,
        false,
        "XX",
        "XX",
    );
    input.nodes[2].style.padding.top = LayoutLengthPercentage::length(1.0);

    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &text_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph {
            node: item,
            reason: "C11.4 Grid first-baseline text requires a direct leaf with zero box edges"
        }) if item == node(3)
    ));
}
