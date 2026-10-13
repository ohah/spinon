use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    FlexDirection, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap, LayoutInputRevision,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine,
    Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn grid_input(
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    item_count: usize,
    gap: LayoutGap,
) -> LayoutInput {
    let root = id(1);
    let children = (0..item_count)
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(320.0),
            height: LayoutDimension::Fixed(200.0),
            gap,
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(children.into_iter().map(|child| LayoutNode {
        id: child,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(20.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    }));
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 200.0,
        },
        root_sizing: crate::RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns,
                rows,
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::Auto,
                auto_columns: crate::GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::Existing,
            },
        )]),
        grid_items: BTreeMap::new(),
    }
}

#[test]
fn definite_length_fraction_and_gap_tracks_auto_place_in_source_order() {
    let input = grid_input(
        vec![GridTrackSize::LengthPx(100.0), GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(80.0), GridTrackSize::Fraction(1.0)],
        4,
        LayoutGap {
            column: LayoutLengthPercentage::length(5.0),
            row: LayoutLengthPercentage::length(10.0),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&id(2)].x, 0.0);
    assert_eq!(output.frames[&id(2)].y, 0.0);
    assert_eq!(output.frames[&id(3)].x, 105.0);
    assert_eq!(output.frames[&id(3)].y, 0.0);
    assert_eq!(output.frames[&id(4)].x, 0.0);
    assert_eq!(output.frames[&id(4)].y, 90.0);
    assert_eq!(output.frames[&id(5)].x, 105.0);
    assert_eq!(output.frames[&id(5)].y, 90.0);
}

#[test]
fn empty_grid_uses_explicit_tracks_without_creating_items() {
    let input = grid_input(
        vec![
            GridTrackSize::LengthPx(80.0),
            GridTrackSize::LengthPx(100.0),
        ],
        vec![GridTrackSize::LengthPx(90.0)],
        0,
        LayoutGap::default(),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames.len(), 1);
    let frame = &output.frames[&id(1)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 320.0, 200.0]
    );
}

#[test]
fn nested_grid_containers_classify_auto_and_explicit_items_independently() {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(160.0),
            GridTrackSize::LengthPx(160.0),
        ],
        vec![GridTrackSize::LengthPx(200.0)],
        2,
        LayoutGap::default(),
    );
    let outer_item = id(2);
    let nested_grid = id(3);
    let nested_item = id(4);
    input.nodes[2] = LayoutNode {
        id: nested_grid,
        children: vec![nested_item],
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(160.0),
            height: LayoutDimension::Fixed(120.0),
            ..LayoutStyle::default()
        },
    };
    input.nodes.push(LayoutNode {
        id: nested_item,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Auto,
            height: LayoutDimension::Auto,
            ..LayoutStyle::default()
        },
    });
    input.grid_containers.insert(
        nested_grid,
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(160.0)],
            rows: vec![GridTrackSize::LengthPx(120.0)],
            auto_flow: GridAutoFlow::Row,
            auto_rows: GridAutoRows::Auto,
            auto_columns: crate::GridAutoColumns::Auto,
            placement_mode: GridPlacementMode::Existing,
        },
    );
    input.grid_items.insert(
        nested_item,
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert!(output.frames.contains_key(&outer_item));
    assert!(output.frames.contains_key(&nested_grid));
    assert!(output.frames.contains_key(&nested_item));
}

#[test]
fn hidden_grid_child_does_not_trigger_validation_for_unsupported_placement() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    let hidden_child = id(2);
    input.nodes[1].style.display = LayoutDisplay::None;
    input.grid_items.insert(
        hidden_child,
        GridItemInput {
            row_start: GridPlacementInput::Line(0),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert!(output.frames.contains_key(&id(1)));
    let hidden_frame = &output.frames[&hidden_child];
    assert_eq!((hidden_frame.width, hidden_frame.height), (0.0, 0.0));
}

#[test]
fn hidden_explicit_item_does_not_disable_implicit_track_rejection_for_visible_auto_items() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        3,
        LayoutGap::default(),
    );
    let hidden_child = id(2);
    input.nodes[1].style.display = LayoutDisplay::None;
    input.grid_items.insert(
        hidden_child,
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridImplicitTrackRequired(id(1)))
    );
}

#[test]
fn percentage_tracks_use_the_definite_grid_content_axes() {
    let input = grid_input(
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0),
        ],
        vec![GridTrackSize::Percentage(0.5), GridTrackSize::Fraction(1.0)],
        4,
        LayoutGap::default(),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&id(3)].x, 80.0);
    assert_eq!(output.frames[&id(4)].y, 100.0);
    assert_eq!(output.frames[&id(5)].x, 80.0);
    assert_eq!(output.frames[&id(5)].y, 100.0);
}

#[test]
fn auto_flow_outside_c11_1_fails_closed_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(160.0), GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(100.0), GridTrackSize::Fraction(1.0)],
        1,
        LayoutGap::default(),
    );
    input.grid_containers.get_mut(&id(1)).unwrap().auto_flow = GridAutoFlow::Column;

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoFlow(id(1)))
    );
}

#[test]
fn nonzero_grid_item_order_fails_closed_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[1].style.order = 1;

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridOrder(id(2)))
    );
}

#[test]
fn grid_track_count_above_taffy_limit_fails_instead_of_being_truncated() {
    let input = grid_input(
        vec![GridTrackSize::LengthPx(1.0); GRID_MAX_TRACKS + 1],
        vec![GridTrackSize::LengthPx(200.0)],
        0,
        LayoutGap::default(),
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackLimitExceeded {
            node: id(1),
            property: "grid-template-columns",
            count: GRID_MAX_TRACKS + 1,
            maximum: GRID_MAX_TRACKS,
        })
    );
}

#[test]
fn grid_layout_preserves_child_flex_dispatch() {
    let flex = id(2);
    let item = id(3);
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[1] = LayoutNode {
        id: flex,
        children: vec![item],
        style: LayoutStyle {
            display: LayoutDisplay::Flex,
            flex_direction: FlexDirection::Row,
            width: LayoutDimension::Fixed(100.0),
            height: LayoutDimension::Fixed(40.0),
            ..LayoutStyle::default()
        },
    };
    input.nodes.push(LayoutNode {
        id: item,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(20.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    });

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&flex].width, 100.0);
    assert_eq!(output.frames[&item].width, 20.0);
}
