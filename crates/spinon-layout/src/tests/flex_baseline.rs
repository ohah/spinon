use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, StyleRevision, Tree};

use crate::{
    FlexDirection, FlexWrap, LayoutAlignContent, LayoutAlignItems, LayoutDimension, LayoutDisplay,
    LayoutEdges, LayoutEngine, LayoutError, LayoutGap, LayoutInput, LayoutLengthPercentage,
    LayoutStyle, TaffyLayoutEngine, TextDirection, Viewport,
};

#[test]
fn zero_main_size_items_that_exactly_fit_share_one_baseline_line() {
    let (root, first, second) = ids();
    let input = input(root, first, second, 100.0, 0.0);
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let first_frame = output.frames[&first];
    let second_frame = output.frames[&second];

    assert_eq!(first_frame.x, 0.0);
    assert_eq!(second_frame.x, 0.0);
    assert_eq!(
        first_frame.y + first_frame.height,
        second_frame.y + second_frame.height
    );
}

#[test]
fn positive_main_gap_can_wrap_zero_size_items_at_the_same_main_coordinate() {
    let (root, first, second) = ids();
    let input = input(root, first, second, 4.0, 5.0);
    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&first].x, 0.0);
    assert_eq!(output.frames[&second].x, 0.0);
    assert_eq!(output.frames[&first].y, 0.0);
    assert_eq!(output.frames[&second].y, 20.0);
}

#[test]
fn rtl_baseline_fails_closed_instead_of_reusing_ltr_geometry() {
    let (root, first, second) = ids();
    let mut input = input(root, first, second, 100.0, 0.0);
    input.nodes[0].style.direction = TextDirection::Rtl;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedBaseline { node, .. }) if node == root
    ));
}

#[test]
fn ambiguous_negative_main_margin_fails_closed() {
    let (root, first, second) = ids();
    let mut input = input(root, first, second, 100.0, 0.0);
    input.nodes[1].style.width = LayoutDimension::Fixed(10.0);
    input.nodes[2].style.width = LayoutDimension::Fixed(10.0);
    input.nodes[1].style.margin = LayoutEdges {
        right: LayoutLengthPercentage::length(-10.0),
        ..LayoutEdges::default()
    };

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedBaseline { node, .. }) if node == root
    ));
}

#[test]
fn first_baseline_content_alignment_rejects_auto_sized_items() {
    let (root, first, second) = ids();
    let mut input = input(root, first, second, 100.0, 0.0);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::FirstBaseline);
    input.nodes[1].style.width = LayoutDimension::Auto;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedBaseline { node, .. }) if node == root
    ));
}

#[test]
fn display_none_baseline_container_does_not_reject_its_hidden_auto_sized_subtree() {
    let (root, hidden, child) = ids();
    let mut input = input(root, hidden, child, 100.0, 0.0);
    input.nodes[0].children = vec![hidden];
    input.nodes[1].children = vec![child];
    input.nodes[1].style.display = LayoutDisplay::None;
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.nodes[1].style.align_content = Some(LayoutAlignContent::FirstBaseline);

    assert!(TaffyLayoutEngine.compute(&input).is_ok());
}

fn ids() -> (
    spinon_core::NodeId,
    spinon_core::NodeId,
    spinon_core::NodeId,
) {
    (
        super::node_id(101),
        super::node_id(102),
        super::node_id(103),
    )
}

fn input(
    root: spinon_core::NodeId,
    first: spinon_core::NodeId,
    second: spinon_core::NodeId,
    width: f32,
    column_gap: f32,
) -> LayoutInput {
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .create(root, "div")
        .insert(root, None, 0)
        .create(first, "div")
        .insert(first, Some(root), 0)
        .create(second, "div")
        .insert(second, Some(root), 1);
    tree.commit(batch).unwrap();

    let styles = BTreeMap::from([
        (
            root,
            LayoutStyle {
                display: LayoutDisplay::Flex,
                width: LayoutDimension::Fixed(width),
                height: LayoutDimension::Fixed(80.0),
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                align_items: LayoutAlignItems::FirstBaseline,
                align_content: Some(LayoutAlignContent::FlexStart),
                gap: LayoutGap {
                    row: LayoutLengthPercentage::ZERO,
                    column: LayoutLengthPercentage::length(column_gap),
                },
                ..LayoutStyle::default()
            },
        ),
        (
            first,
            LayoutStyle {
                width: LayoutDimension::Fixed(0.0),
                height: LayoutDimension::Fixed(20.0),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                direction: TextDirection::Ltr,
                ..LayoutStyle::default()
            },
        ),
        (
            second,
            LayoutStyle {
                width: LayoutDimension::Fixed(0.0),
                height: LayoutDimension::Fixed(30.0),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                direction: TextDirection::Ltr,
                ..LayoutStyle::default()
            },
        ),
    ]);
    LayoutInput::from_tree(
        &tree,
        Viewport {
            width,
            height: 80.0,
        },
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
}
