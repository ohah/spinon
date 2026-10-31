use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_auto_placement::{
    TestGridReservationSite as GridReservationSite, inject_failure_once,
};
use crate::{
    GridAutoRows, GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackSize, LayoutDimension, LayoutEngine, LayoutGap, LayoutInputRevision,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine,
    Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn sparse_grid(columns: usize, rows: usize, item_count: usize) -> LayoutInput {
    let root = id(1);
    let children = (0..item_count)
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(columns as f32 * 40.0),
            height: LayoutDimension::Fixed(rows as f32 * 24.0 + 64.0),
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(children.into_iter().map(|child| LayoutNode {
        id: child,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(18.0),
            height: LayoutDimension::Fixed(18.0),
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
            width: columns as f32 * 40.0,
            height: rows as f32 * 24.0 + 64.0,
        },
        root_sizing: crate::RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(40.0); columns],
                rows: vec![GridTrackSize::LengthPx(20.0); rows],
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                justify_items: Default::default(),
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::LengthPx(20.0),
                auto_columns: crate::GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::SparseRow,
                track_sizing_trace_enabled: false,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

fn placement(
    row_start: GridPlacementInput,
    row_end: GridPlacementInput,
    column_start: GridPlacementInput,
    column_end: GridPlacementInput,
) -> GridItemInput {
    GridItemInput {
        row_start,
        row_end,
        column_start,
        column_end,
    }
}

fn frame(input: &LayoutInput, node: u64) -> crate::LayoutFrame {
    TaffyLayoutEngine.compute(input).unwrap().frames[&id(node)]
}

#[test]
fn mixed_definite_and_auto_items_place_definite_items_first() {
    let mut input = sparse_grid(2, 2, 3);
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );

    assert_eq!([frame(&input, 2).x, frame(&input, 2).y], [40.0, 0.0]);
    assert_eq!([frame(&input, 3).x, frame(&input, 3).y], [0.0, 0.0]);
    assert_eq!([frame(&input, 4).x, frame(&input, 4).y], [0.0, 20.0]);
}

#[test]
fn sparse_auto_items_follow_order_and_preserve_source_order_for_ties() {
    let mut input = sparse_grid(3, 2, 4);
    input.nodes[1].style.order = 2;
    input.nodes[2].style.order = -1;
    input.nodes[3].style.order = -1;

    assert_eq!([frame(&input, 3).x, frame(&input, 3).y], [0.0, 0.0]);
    assert_eq!([frame(&input, 4).x, frame(&input, 4).y], [40.0, 0.0]);
    assert_eq!([frame(&input, 5).x, frame(&input, 5).y], [80.0, 0.0]);
    assert_eq!([frame(&input, 2).x, frame(&input, 2).y], [0.0, 20.0]);
}

#[test]
fn sparse_order_is_rejected_when_any_visible_item_has_partial_placement() {
    let mut input = sparse_grid(2, 2, 2);
    input.nodes[1].style.order = 1;
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridOrder(child)) if child == id(2)
    ));
}

#[test]
fn row_sparse_cursor_does_not_backfill_a_hole_and_respects_spans() {
    let mut input = sparse_grid(3, 1, 4);
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Span(2),
        ),
    );

    assert_eq!([frame(&input, 2).x, frame(&input, 2).y], [0.0, 0.0]);
    assert_eq!([frame(&input, 3).x, frame(&input, 3).y], [0.0, 20.0]);
    assert_eq!([frame(&input, 4).x, frame(&input, 4).y], [40.0, 0.0]);
    assert_eq!([frame(&input, 5).x, frame(&input, 5).y], [80.0, 20.0]);
}

#[test]
fn column_locked_items_wrap_to_the_next_row_when_cursor_column_decreases() {
    let mut input = sparse_grid(3, 1, 2);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );

    assert_eq!([frame(&input, 2).x, frame(&input, 2).y], [40.0, 0.0]);
    assert_eq!([frame(&input, 3).x, frame(&input, 3).y], [0.0, 20.0]);
}

#[test]
fn implicit_rows_repeat_fixed_track_size_and_keep_row_gap() {
    let mut input = sparse_grid(2, 1, 3);
    input.grid_containers.get_mut(&id(1)).unwrap().auto_rows = GridAutoRows::LengthPx(12.0);
    input.nodes[0].style.gap = LayoutGap {
        row: LayoutLengthPercentage::length(4.0),
        ..LayoutGap::default()
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let third = output.frames[&id(4)];
    assert_eq!([third.x, third.y, third.height], [0.0, 24.0, 18.0]);
}

#[test]
fn implicit_auto_sizing_and_explicit_column_overflow_fail_closed() {
    let mut implicit_auto_rows = sparse_grid(2, 1, 3);
    implicit_auto_rows
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::Auto;
    assert!(matches!(
        TaffyLayoutEngine.compute(&implicit_auto_rows),
        Err(LayoutError::UnsupportedGridAutoRows { .. })
    ));

    let mut implicit_column = sparse_grid(2, 1, 1);
    implicit_column.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(4),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&implicit_column),
        Err(LayoutError::InvalidGridPlacement { .. })
    ));
}

#[test]
fn sparse_placement_normalizes_reversed_equal_and_negative_lines() {
    let mut input = sparse_grid(3, 2, 3);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(-2),
            GridPlacementInput::Line(-1),
        ),
    );
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Span(1),
            GridPlacementInput::Span(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        [output.frames[&id(2)].x, output.frames[&id(2)].y],
        [0.0, 0.0]
    );
    assert_eq!(
        [output.frames[&id(3)].x, output.frames[&id(3)].y],
        [80.0, 20.0]
    );
    assert_eq!(
        [output.frames[&id(4)].x, output.frames[&id(4)].y],
        [40.0, 0.0]
    );
}

#[test]
fn implicit_track_cap_is_checked_before_expanding_rows() {
    let mut input = sparse_grid(1, 1, 1);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Span(10_001),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackLimitExceeded {
            count: 10_001,
            maximum: 10_000,
            ..
        })
    ));
}

#[test]
fn auto_placed_intrinsic_item_size_is_rejected_and_next_layout_recovers() {
    let mut invalid = sparse_grid(2, 1, 1);
    invalid.nodes[1].style.width = LayoutDimension::Auto;
    assert!(matches!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::UnsupportedGridItemSize(_))
    ));

    let recovered = sparse_grid(2, 1, 1);
    assert_eq!([frame(&recovered, 2).x, frame(&recovered, 2).y], [0.0, 0.0]);
}

#[test]
fn actual_grid_vec_reservation_failures_return_errors_and_recover_on_the_same_worker() {
    let mut definite_item = sparse_grid(2, 1, 1);
    definite_item.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    let cases = vec![
        (
            GridReservationSite::VisibleItemList,
            sparse_grid(2, 1, 1),
            id(1),
            "grid-placement-occupancy",
        ),
        (
            GridReservationSite::RowLockedCursor,
            sparse_grid(2, 1, 1),
            id(1),
            "grid-placement-occupancy",
        ),
        (
            GridReservationSite::ExpandedRows,
            sparse_grid(1, 1, 2),
            id(1),
            "grid-template-rows",
        ),
        (
            GridReservationSite::OccupancyRows,
            definite_item.clone(),
            id(2),
            "grid-placement-occupancy",
        ),
        (
            GridReservationSite::OccupancyIntervals,
            definite_item,
            id(2),
            "grid-placement-occupancy",
        ),
    ];

    std::thread::spawn(move || {
        let engine = TaffyLayoutEngine;
        for (site, input, expected_node, expected_property) in cases {
            let guard = inject_failure_once(site);
            let result = engine.compute(&input);
            let observed_failure = crate::test_allocator::take_failure_observed();
            eprintln!(
                "GRID_RESERVATION_FAILURE site={site:?} observed={observed_failure} expected_node={expected_node} expected_property={expected_property} returned_error={}",
                matches!(result, Err(LayoutError::GridAllocationFailure { .. }))
            );
            assert!(
                observed_failure,
                "{site:?}는 실제 allocator allocation을 실패시켜야 합니다"
            );
            assert!(matches!(
                result,
                Err(LayoutError::GridAllocationFailure { node, property })
                    if node == expected_node && property == expected_property
            ));
            drop(guard);

            let recovered = engine
                .compute(&input)
                .expect("같은 worker의 다음 정상 계산은 회복해야 합니다");
            assert!(
                recovered.frames.contains_key(&expected_node),
                "실패 뒤 정상 계산은 대상 node의 frame을 반환해야 합니다"
            );
            assert!(
                !crate::test_allocator::take_failure_observed(),
                "회복 계산은 allocator failpoint 없이 수행되어야 합니다"
            );
        }
    })
    .join()
    .expect("Grid reservation failure worker는 panic 없이 끝나야 합니다");
}
