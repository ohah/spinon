use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_auto_placement::{
    TestGridReservationSite as GridReservationSite, inject_failure_once,
};
use crate::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridItemInput,
    GridPlacementInput, GridPlacementMode, GridTrackSize, LayoutAlignContent, LayoutDimension,
    LayoutDisplay, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingStageKind, LayoutInputRevision, LayoutLengthPercentage, LayoutNode,
    LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
};

#[path = "grid_auto_placement/content_alignment_tests.rs"]
mod content_alignment_tests;
#[path = "grid_auto_placement/root_html_column_flow_calc_tests.rs"]
mod root_html_column_flow_calc_tests;
#[path = "grid_auto_placement/root_html_column_flow_fraction_tests.rs"]
mod root_html_column_flow_fraction_tests;
#[path = "grid_auto_placement/root_html_implicit_rows_tests.rs"]
mod root_html_implicit_rows_tests;
#[path = "grid_auto_placement/root_html_one_axis_placement_tests.rs"]
mod root_html_one_axis_placement_tests;

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

fn root_html_column_flow_grid(item_count: usize) -> LayoutInput {
    let mut input = sparse_grid(2, 2, item_count);
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(60.0);
    input.viewport = Viewport {
        width: 100.0,
        height: 60.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(70.0)];
    container.rows = vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(40.0)];
    container.auto_flow = GridAutoFlow::Column;
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = GridAutoColumns::Auto;
    container.placement_mode = GridPlacementMode::RootHtmlColumnFlowV1;
    for (index, child) in input.nodes.iter_mut().skip(1).enumerate() {
        child.style.display = LayoutDisplay::Block;
        child.style.width = LayoutDimension::Fixed([30.0, 30.0, 70.0, 70.0, 10.0][index]);
        child.style.height = LayoutDimension::Fixed([20.0, 40.0, 20.0, 40.0, 10.0][index]);
    }
    input
}

fn root_html_column_flow_with_auto_columns(
    item_count: usize,
    auto_columns: GridAutoColumns,
) -> LayoutInput {
    let mut input = sparse_grid(2, 2, item_count);
    input.nodes[0].style.width = LayoutDimension::Fixed(175.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(63.0);
    input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(5.0),
        row: LayoutLengthPercentage::length(3.0),
    };
    input.viewport = Viewport {
        width: 175.0,
        height: 63.0,
    };
    for child in input.nodes.iter_mut().skip(1) {
        child.style.display = LayoutDisplay::Block;
        child.style.width = LayoutDimension::Fixed(1.0);
        child.style.height = LayoutDimension::Fixed(1.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)];
    container.rows = vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(40.0)];
    container.auto_flow = GridAutoFlow::Column;
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = auto_columns;
    container.placement_mode = GridPlacementMode::RootHtmlColumnFlowV1;
    input
}

#[test]
fn root_html_column_flow_places_source_order_down_each_explicit_column() {
    let input = root_html_column_flow_grid(4);

    for (node, expected) in [
        (2, [0.0, 0.0, 30.0, 20.0]),
        (3, [0.0, 20.0, 30.0, 40.0]),
        (4, [30.0, 0.0, 70.0, 20.0]),
        (5, [30.0, 20.0, 70.0, 40.0]),
    ] {
        let actual = frame(&input, node);
        assert_eq!(
            [actual.x, actual.y, actual.width, actual.height],
            expected,
            "node {node}"
        );
    }
}

#[test]
fn root_html_column_flow_applies_each_gap_once_on_its_own_axis() {
    let mut input = root_html_column_flow_grid(4);
    input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(5.0),
        row: LayoutLengthPercentage::length(3.0),
    };

    for (node, expected) in [
        (2, [0.0, 0.0, 30.0, 20.0]),
        (3, [0.0, 23.0, 30.0, 40.0]),
        (4, [35.0, 0.0, 70.0, 20.0]),
        (5, [35.0, 23.0, 70.0, 40.0]),
    ] {
        let actual = frame(&input, node);
        assert_eq!(
            [actual.x, actual.y, actual.width, actual.height],
            expected,
            "node {node}"
        );
    }
}

#[test]
fn root_html_column_flow_rejects_implicit_column_overflow_before_placement() {
    let input = root_html_column_flow_grid(5);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::GridImplicitTrackRequired(node)) if node == id(1)
    ));
}

#[test]
fn root_html_column_flow_repeats_fixed_px_implicit_column_list() {
    for (item_count, expected) in [
        (5, vec![(6, [80.0, 0.0])]),
        (
            8,
            vec![
                (6, [80.0, 0.0]),
                (7, [80.0, 23.0]),
                (8, [100.0, 0.0]),
                (9, [100.0, 23.0]),
            ],
        ),
    ] {
        let input = root_html_column_flow_with_auto_columns(
            item_count,
            GridAutoColumns::TrackList(vec![
                GridTrackSize::LengthPx(15.0),
                GridTrackSize::LengthPx(25.0),
            ]),
        );
        let output = TaffyLayoutEngine
            .compute(&input)
            .expect("fixed px list should produce only the required implicit columns");
        for (node, expected_origin) in expected {
            let actual = output.frames[&id(node)];
            assert_eq!([actual.x, actual.y], expected_origin, "node {node}");
        }
    }
}

#[test]
fn root_html_column_flow_expands_fixed_px_scalar_for_each_implicit_column() {
    let input = root_html_column_flow_with_auto_columns(8, GridAutoColumns::LengthPx(12.0));
    let output = TaffyLayoutEngine
        .compute(&input)
        .expect("one fixed px auto-column size should repeat for implicit columns");

    for (node, expected) in [
        (6, [80.0, 0.0]),
        (7, [80.0, 23.0]),
        (8, [97.0, 0.0]),
        (9, [97.0, 23.0]),
    ] {
        let actual = output.frames[&id(node)];
        assert_eq!([actual.x, actual.y], expected, "node {node}");
    }
}

#[test]
fn root_html_column_flow_repeats_percentage_auto_columns_from_the_definite_width() {
    let mut input = root_html_column_flow_with_auto_columns(
        12,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::Percentage(0.1),
            GridTrackSize::Percentage(0.15),
        ]),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    input.viewport.width = 240.0;
    let output = TaffyLayoutEngine
        .compute(&input)
        .expect("percentage auto-column list should resolve against the definite 240px width");

    for (node, expected) in [
        (2, [0.0, 0.0]),
        (3, [0.0, 23.0]),
        (4, [35.0, 0.0]),
        (5, [35.0, 23.0]),
        (6, [80.0, 0.0]),
        (7, [80.0, 23.0]),
        (8, [109.0, 0.0]),
        (9, [109.0, 23.0]),
        (10, [150.0, 0.0]),
        (11, [150.0, 23.0]),
        (12, [179.0, 0.0]),
        (13, [179.0, 23.0]),
    ] {
        let actual = output.frames[&id(node)];
        assert_eq!([actual.x, actual.y], expected, "node {node}");
    }
}

#[test]
fn root_html_column_flow_repeats_percentage_scalar_mixed_zero_and_overflow_tracks() {
    for (auto_columns, expected_x) in [
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(0.1)]),
            [80.0, 109.0, 138.0, 167.0],
        ),
        (
            GridAutoColumns::TrackList(vec![
                GridTrackSize::LengthPx(10.0),
                GridTrackSize::Percentage(0.15),
            ]),
            [80.0, 95.0, 136.0, 151.0],
        ),
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(0.0)]),
            [80.0, 85.0, 90.0, 95.0],
        ),
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(1.2)]),
            [80.0, 373.0, 666.0, 959.0],
        ),
    ] {
        let mut input = root_html_column_flow_with_auto_columns(12, auto_columns);
        input.nodes[0].style.width = LayoutDimension::Fixed(240.0);
        input.viewport.width = 240.0;
        let output = TaffyLayoutEngine
            .compute(&input)
            .expect("percentage track list should repeat without clamping or subtracting the gap");
        for (index, expected) in expected_x.into_iter().enumerate() {
            let node = 6 + (index as u64 * 2);
            let actual = output.frames[&id(node)];
            assert_eq!(actual.x, expected, "node {node}");
        }
    }
}

#[test]
fn root_html_column_flow_percentage_columns_accept_a_definite_zero_width_basis() {
    let mut input = root_html_column_flow_with_auto_columns(
        12,
        GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(0.5)]),
    );
    let mut grid = input.nodes.remove(0);
    grid.style.width = LayoutDimension::Fixed(0.0);
    let wrapper = LayoutNode {
        id: id(100),
        children: vec![id(1)],
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(175.0),
            height: LayoutDimension::Fixed(63.0),
            ..LayoutStyle::default()
        },
    };
    input.root = id(100);
    input.nodes.insert(0, wrapper);
    input.nodes.insert(1, grid);
    input.viewport.width = 175.0;
    let output = TaffyLayoutEngine
        .compute(&input)
        .expect("zero is a definite percentage basis");
    for (node, expected) in [(6, 80.0), (8, 85.0), (10, 90.0), (12, 95.0)] {
        assert_eq!(output.frames[&id(node)].x, expected, "node {node}");
    }
}

#[test]
fn root_html_column_flow_percentage_columns_reject_constrained_or_math_widths() {
    let make_input = || {
        root_html_column_flow_with_auto_columns(
            8,
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(0.25)]),
        )
    };
    let mut indefinite = make_input();
    indefinite.nodes[0].style.width = LayoutDimension::Auto;
    assert!(TaffyLayoutEngine.compute(&indefinite).is_err());

    let mut minimum = make_input();
    minimum.nodes[0].style.min_width = LayoutDimension::Fixed(1.0);
    assert!(matches!(
        TaffyLayoutEngine.compute(&minimum),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            property: "min-width",
            ..
        })
    ));

    let mut maximum = make_input();
    maximum.nodes[0].style.max_width = LayoutDimension::Fixed(200.0);
    assert!(matches!(
        TaffyLayoutEngine.compute(&maximum),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            property: "max-width",
            ..
        })
    ));

    let mut ratio = make_input();
    ratio.nodes[0].style.aspect_ratio = Some(1.0);
    assert!(matches!(
        TaffyLayoutEngine.compute(&ratio),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            property: "aspect-ratio",
            ..
        })
    ));
}

#[test]
fn root_html_column_flow_percentage_columns_reject_aggregate_f32_overflow() {
    let mut input = root_html_column_flow_with_auto_columns(
        8,
        GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(1.0)]),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(f32::MAX);
    input.viewport.width = f32::MAX;
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridTrack {
            property: "grid-auto-columns",
            ..
        })
    ));
}

#[test]
fn root_html_column_flow_keeps_implicit_sizing_when_explicit_cells_are_sufficient() {
    for auto_columns in [
        GridAutoColumns::Auto,
        GridAutoColumns::LengthPx(12.0),
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(25.0),
        ]),
    ] {
        let input = root_html_column_flow_with_auto_columns(4, auto_columns);
        assert!(TaffyLayoutEngine.compute(&input).is_ok());
    }
}

#[test]
fn root_html_column_flow_rejects_invalid_auto_column_tracks() {
    let cases = [
        (
            GridAutoColumns::LengthPx(-1.0),
            "negative scalar",
            "invalid",
        ),
        (
            GridAutoColumns::LengthPx(f32::NAN),
            "non-finite scalar",
            "invalid",
        ),
        (
            GridAutoColumns::TrackList(Vec::new()),
            "empty track list",
            "list",
        ),
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(-0.5)]),
            "negative percentage track",
            "invalid",
        ),
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(f32::NAN)]),
            "non-finite percentage track",
            "invalid",
        ),
        (
            GridAutoColumns::TrackList(vec![GridTrackSize::Auto]),
            "intrinsic track",
            "unsupported",
        ),
        (
            GridAutoColumns::TrackList(vec![
                GridTrackSize::LengthPx(1.0);
                crate::GRID_MAX_TRACKS + 1
            ]),
            "oversized track list",
            "limit",
        ),
    ];
    for (auto_columns, label, expected) in cases {
        let input = root_html_column_flow_with_auto_columns(5, auto_columns);
        let result = TaffyLayoutEngine.compute(&input);
        match expected {
            "invalid" => assert!(
                matches!(
                    result,
                    Err(LayoutError::InvalidGridTrack {
                        property: "grid-auto-columns",
                        ..
                    })
                ),
                "{label}: {result:?}"
            ),
            "list" => assert!(
                matches!(
                    result,
                    Err(LayoutError::InvalidGridTrackList {
                        property: "grid-auto-columns",
                        ..
                    })
                ),
                "{label}: {result:?}"
            ),
            "unsupported" => assert!(
                matches!(result, Err(LayoutError::UnsupportedGridAutoColumns { .. })),
                "{label}: {result:?}"
            ),
            "limit" => assert!(
                matches!(
                    result,
                    Err(LayoutError::GridTrackLimitExceeded {
                        node,
                        property: "grid-auto-columns",
                        count: 10_001,
                        maximum: 10_000,
                    }) if node == id(1)
                ),
                "{label}: {result:?}"
            ),
            _ => unreachable!("test case must name its expected error"),
        }
    }
}

#[test]
fn root_html_column_flow_rejects_required_implicit_columns_over_track_limit() {
    let mut input = sparse_grid(1, 1, crate::GRID_MAX_TRACKS + 1);
    input.nodes[0].style.width = LayoutDimension::Fixed(30.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(20.0);
    input.viewport = Viewport {
        width: 30.0,
        height: 20.0,
    };
    for child in input.nodes.iter_mut().skip(1) {
        child.style.display = LayoutDisplay::Block;
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(30.0)];
    container.rows = vec![GridTrackSize::LengthPx(20.0)];
    container.auto_flow = GridAutoFlow::Column;
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = GridAutoColumns::LengthPx(10.0);
    container.placement_mode = GridPlacementMode::RootHtmlColumnFlowV1;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-columns",
            count: 10_001,
            maximum: 10_000,
        }) if node == id(1)
    ));
}

#[test]
fn root_html_column_flow_expanded_column_reservation_failure_recovers() {
    let input = root_html_column_flow_with_auto_columns(
        6,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(25.0),
        ]),
    );

    std::thread::spawn(move || {
        let engine = TaffyLayoutEngine;
        let guard = inject_failure_once(GridReservationSite::ExpandedColumns);
        let result = engine.compute(&input);
        let observed_failure = crate::test_allocator::take_failure_observed();
        assert!(
            observed_failure,
            "ExpandedColumns must hit a real reservation"
        );
        assert!(matches!(
            result,
            Err(LayoutError::GridAllocationFailure {
                node,
                property: "grid-template-columns"
            }) if node == id(1)
        ));
        drop(guard);

        let recovered = engine
            .compute(&input)
            .expect("the next layout on the same worker should recover");
        assert!(recovered.frames.contains_key(&id(7)));
        assert!(!crate::test_allocator::take_failure_observed());
    })
    .join()
    .expect("implicit column allocation failure worker should recover");
}

#[test]
fn root_html_column_flow_rejects_order_and_authored_placement() {
    let mut ordered = root_html_column_flow_grid(4);
    ordered.nodes[1].style.order = 1;
    assert!(matches!(
        TaffyLayoutEngine.compute(&ordered),
        Err(crate::LayoutError::UnsupportedGridOrder(node)) if node == id(2)
    ));

    let mut placed = root_html_column_flow_grid(4);
    placed.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Auto,
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&placed),
        Err(crate::LayoutError::InvalidGridPlacement { node, .. }) if node == id(2)
    ));
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
fn grid_root_html_fixed_implicit_rows_reject_resource_limit_before_expansion() {
    let mut input = sparse_grid(1, 1, super::GRID_MAX_TRACKS + 1);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::GridImplicitTrackRequired(node)) if node == id(1)
    ));
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
fn root_html_intrinsic_track_sizing_stretches_an_automatic_implicit_row() {
    let mut input = sparse_grid(1, 1, 2);
    input.viewport.height = 40.0;
    input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::Normal);
    input.nodes[1].style.display = LayoutDisplay::Block;
    input.nodes[2].style.display = LayoutDisplay::Block;
    input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
    input.nodes[2].style.height = LayoutDimension::Auto;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(40.0)];
    container.rows = vec![GridTrackSize::LengthPx(10.0)];
    container.auto_rows = GridAutoRows::Auto;
    container.placement_mode = GridPlacementMode::RootHtmlAutoRowStretchV1;
    container.track_sizing_trace_enabled = true;

    let output = TaffyLayoutEngine
        .compute(&input)
        .expect("the restricted Root HTML intrinsic route should size an auto row");
    let row_item = output.frames[&id(3)];
    assert_eq!(
        [row_item.x, row_item.y, row_item.width, row_item.height],
        [0.0, 10.0, 18.0, 30.0]
    );
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .expect("the Grid should retain a track-sizing trace");
    assert_eq!(trace.used_rows.used_track_list, "10px 30px");
    let row_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("the Grid should trace its row sizing pass");
    let stretch = row_pass
        .stages
        .iter()
        .find(|stage| {
            stage.stage == LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true }
        })
        .expect("align-content:normal should run the auto-track stretch stage");
    let stretch_tracks = stretch
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(stretch_tracks, [10.0, 30.0]);
}

#[test]
fn root_html_two_implicit_auto_rows_respect_supported_content_alignment() {
    let cases = [
        (
            LayoutAlignContent::Normal,
            [10.0, 15.0, 15.0],
            true,
            [
                [0.0, 0.0, 40.0, 10.0],
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 25.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::Stretch,
            [10.0, 15.0, 15.0],
            true,
            [
                [0.0, 0.0, 40.0, 10.0],
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 25.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::Position {
                position: crate::ContentAlignmentPosition::Start,
                safety: None,
            },
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 0.0, 40.0, 10.0],
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 20.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::FlexStart,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 0.0, 40.0, 10.0],
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 20.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::Position {
                position: crate::ContentAlignmentPosition::End,
                safety: None,
            },
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 20.0, 40.0, 10.0],
                [0.0, 30.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::FlexEnd,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 10.0, 40.0, 10.0],
                [0.0, 20.0, 40.0, 10.0],
                [0.0, 30.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::Center,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 5.0, 40.0, 10.0],
                [0.0, 15.0, 40.0, 10.0],
                [0.0, 25.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::SpaceBetween,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 0.0, 40.0, 10.0],
                [0.0, 15.0, 40.0, 10.0],
                [0.0, 30.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::SpaceAround,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 5.0 / 3.0, 40.0, 10.0],
                [0.0, 15.0, 40.0, 10.0],
                [0.0, 85.0 / 3.0, 40.0, 10.0],
            ],
        ),
        (
            LayoutAlignContent::SpaceEvenly,
            [10.0, 10.0, 10.0],
            false,
            [
                [0.0, 2.5, 40.0, 10.0],
                [0.0, 15.0, 40.0, 10.0],
                [0.0, 27.5, 40.0, 10.0],
            ],
        ),
    ];
    for (align_content, expected_rows, stretch_applied, expected_frames) in cases {
        let mut input = sparse_grid(1, 1, 3);
        input.viewport = Viewport {
            width: 40.0,
            height: 40.0,
        };
        input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.align_content = Some(align_content);
        for node in input.nodes.iter_mut().skip(1) {
            node.style.display = LayoutDisplay::Block;
            node.style.width = LayoutDimension::Fixed(40.0);
            node.style.height = LayoutDimension::Fixed(10.0);
        }
        let container = input.grid_containers.get_mut(&id(1)).unwrap();
        container.columns = vec![GridTrackSize::LengthPx(40.0)];
        container.rows = vec![GridTrackSize::LengthPx(10.0)];
        container.auto_rows = GridAutoRows::Auto;
        container.placement_mode = GridPlacementMode::RootHtmlAutoRowStretchV1;
        container.track_sizing_trace_enabled = true;

        super::validate_root_html_intrinsic_tracks_contract(
            id(1),
            input.nodes[0].style,
            &input.grid_containers[&id(1)],
        )
        .unwrap_or_else(|error| {
            panic!("intrinsic validation rejected {align_content:?}: {error:?}")
        });

        let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
            panic!("alignment {align_content:?} should be accepted: {error:?}")
        });
        let frames = [id(2), id(3), id(4)].map(|node| {
            let frame = output.frames[&node];
            [frame.x, frame.y, frame.width, frame.height]
        });
        for (index, (actual, expected)) in frames.iter().zip(expected_frames).enumerate() {
            for (actual, expected) in actual.iter().zip(expected) {
                assert!(
                    (*actual - expected).abs() <= 1.0 / 32.0,
                    "alignment {align_content:?}, frame {index}: expected {expected:?}, got {actual:?}"
                );
            }
        }

        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .expect("the Grid should retain its track-sizing trace");
        assert_eq!(trace.used_rows.explicit_track_count, 1);
        assert_eq!(trace.used_rows.positive_implicit_track_count, 2);
        let row_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .expect("the Grid should retain a row sizing pass");
        let stretch_stages = row_pass
            .stages
            .iter()
            .filter(|stage| {
                matches!(
                    stage.stage,
                    LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(stretch_stages.len(), 1);
        let stretch_stage = stretch_stages[0];
        let LayoutGridTrackSizingStageKind::StretchAutoTracks { applied } = stretch_stage.stage
        else {
            unreachable!("the selected stage must record auto-track stretch")
        };
        assert_eq!(
            applied, stretch_applied,
            "alignment {align_content:?} must report whether auto tracks stretched"
        );
        let actual_rows = stretch_stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(actual_rows.len(), expected_rows.len());
        for (actual, expected) in actual_rows.iter().zip(expected_rows) {
            assert!(
                (*actual - expected).abs() <= 1.0 / 32.0,
                "alignment {align_content:?}: expected row sizes {expected_rows:?}, got {actual_rows:?}"
            );
        }
    }
}

#[test]
fn root_html_auto_row_stretch_gap_matrix_matches_gutters_and_overflow() {
    let alignments = [
        ("normal", LayoutAlignContent::Normal),
        ("stretch", LayoutAlignContent::Stretch),
        (
            "start",
            LayoutAlignContent::Position {
                position: crate::ContentAlignmentPosition::Start,
                safety: None,
            },
        ),
        ("flex-start", LayoutAlignContent::FlexStart),
        (
            "end",
            LayoutAlignContent::Position {
                position: crate::ContentAlignmentPosition::End,
                safety: None,
            },
        ),
        ("flex-end", LayoutAlignContent::FlexEnd),
        ("center", LayoutAlignContent::Center),
        ("space-between", LayoutAlignContent::SpaceBetween),
        ("space-around", LayoutAlignContent::SpaceAround),
        ("space-evenly", LayoutAlignContent::SpaceEvenly),
    ];
    let make_input = |align_content, row_gap| {
        let mut input = sparse_grid(1, 1, 3);
        input.viewport = Viewport {
            width: 40.0,
            height: 40.0,
        };
        input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
        input.nodes[0].style.align_content = Some(align_content);
        input.nodes[0].style.gap.row = LayoutLengthPercentage::LengthPx(row_gap);
        for node in input.nodes.iter_mut().skip(1) {
            node.style.display = LayoutDisplay::Block;
            node.style.width = LayoutDimension::Fixed(40.0);
            node.style.height = LayoutDimension::Fixed(10.0);
        }
        let container = input.grid_containers.get_mut(&id(1)).unwrap();
        container.columns = vec![GridTrackSize::LengthPx(40.0)];
        container.rows = vec![GridTrackSize::LengthPx(10.0)];
        container.auto_rows = GridAutoRows::Auto;
        container.placement_mode = GridPlacementMode::RootHtmlAutoRowStretchV1;
        container.track_sizing_trace_enabled = true;
        input
    };

    for row_gap in [0.0, 2.5, 5.0, 7.5] {
        // The 40px content box contains three 10px rows and two fixed gutters.
        let free_space = 10.0 - 2.0 * row_gap;
        for (name, align_content) in alignments {
            let stretches = matches!(
                align_content,
                LayoutAlignContent::Normal | LayoutAlignContent::Stretch
            ) && free_space > 0.0;
            let row_sizes = if stretches {
                [10.0, 10.0 + free_space / 2.0, 10.0 + free_space / 2.0]
            } else {
                [10.0, 10.0, 10.0]
            };
            let base_y = [
                0.0,
                row_sizes[0] + row_gap,
                row_sizes[0] + row_gap + row_sizes[1] + row_gap,
            ];
            let expected_y = match align_content {
                LayoutAlignContent::Normal | LayoutAlignContent::Stretch => base_y,
                LayoutAlignContent::Position {
                    position: crate::ContentAlignmentPosition::End,
                    ..
                }
                | LayoutAlignContent::FlexEnd => base_y.map(|y| y + free_space),
                LayoutAlignContent::Center => base_y.map(|y| y + free_space / 2.0),
                LayoutAlignContent::SpaceBetween if free_space > 0.0 => [
                    base_y[0],
                    base_y[1] + free_space / 2.0,
                    base_y[2] + free_space,
                ],
                LayoutAlignContent::SpaceAround if free_space > 0.0 => [
                    base_y[0] + free_space / 6.0,
                    base_y[1] + free_space / 2.0,
                    base_y[2] + 5.0 * free_space / 6.0,
                ],
                LayoutAlignContent::SpaceEvenly if free_space > 0.0 => [
                    base_y[0] + free_space / 4.0,
                    base_y[1] + free_space / 2.0,
                    base_y[2] + 3.0 * free_space / 4.0,
                ],
                // Distributed alignments use their safe start fallback when space is negative.
                LayoutAlignContent::SpaceBetween
                | LayoutAlignContent::SpaceAround
                | LayoutAlignContent::SpaceEvenly => base_y,
                LayoutAlignContent::Position {
                    position: crate::ContentAlignmentPosition::Start,
                    ..
                }
                | LayoutAlignContent::FlexStart => base_y,
                unsupported => panic!("unexpected alignment profile variant {unsupported:?}"),
            };
            let input = make_input(align_content, row_gap);
            super::validate_root_html_intrinsic_tracks_contract(
                id(1),
                input.nodes[0].style,
                &input.grid_containers[&id(1)],
            )
            .unwrap_or_else(|error| {
                panic!("{name}, row-gap:{row_gap}px: contract rejected input: {error:?}")
            });
            let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                panic!("{name}, row-gap:{row_gap}px: layout rejected input: {error:?}")
            });
            let frames = [id(2), id(3), id(4)].map(|node| {
                let frame = output.frames[&node];
                [frame.x, frame.y, frame.width, frame.height]
            });
            for (index, (actual, expected_y)) in frames.iter().zip(expected_y).enumerate() {
                assert_eq!(actual[0], 0.0, "{name}, gap {row_gap}: frame {index} x");
                assert!(
                    (actual[1] - expected_y).abs() <= 1.0 / 32.0,
                    "{name}, row-gap:{row_gap}px, frame {index}: expected y={expected_y}, got {actual:?}"
                );
                assert_eq!(
                    actual[2], 40.0,
                    "{name}, gap {row_gap}: frame {index} width"
                );
                assert_eq!(
                    actual[3], 10.0,
                    "{name}, gap {row_gap}: frame {index} height"
                );
            }
            let trace = output
                .grid_track_sizing_traces
                .iter()
                .find(|trace| trace.node == id(1))
                .unwrap_or_else(|| panic!("{name}, gap {row_gap}: missing track trace"));
            let stretch_stage = trace
                .passes
                .iter()
                .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
                .and_then(|pass| {
                    pass.stages.iter().find(|stage| {
                        matches!(
                            stage.stage,
                            LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                        )
                    })
                })
                .unwrap_or_else(|| panic!("{name}, gap {row_gap}: missing stretch stage"));
            let actual_rows = stretch_stage
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            assert_eq!(actual_rows.len(), 3, "{name}, gap {row_gap}: track count");
            for (index, (actual, expected)) in actual_rows.iter().zip(row_sizes).enumerate() {
                assert!(
                    (actual - expected).abs() <= 1.0 / 32.0,
                    "{name}, row-gap:{row_gap}px, row {index}: expected {expected}, got {actual}"
                );
            }
        }
    }

    for (property, gap) in [
        (
            "row-gap",
            LayoutGap {
                row: LayoutLengthPercentage::LengthPx(7.5001),
                column: LayoutLengthPercentage::ZERO,
            },
        ),
        (
            "row-gap",
            LayoutGap {
                row: LayoutLengthPercentage::Percentage(0.05),
                column: LayoutLengthPercentage::ZERO,
            },
        ),
        (
            "column-gap",
            LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::LengthPx(0.5),
            },
        ),
    ] {
        let mut input = make_input(LayoutAlignContent::Normal, 0.0);
        input.nodes[0].style.gap = gap;
        assert!(
            matches!(
                TaffyLayoutEngine.compute(&input),
                Err(crate::LayoutError::UnsupportedGridContainerStyle {
                    node,
                    property: rejected_property,
                }) if node == id(1) && rejected_property == property
            ),
            "{property}={gap:?} must remain outside the bounded profile"
        );
    }
}

#[test]
fn root_html_intrinsic_track_sizing_does_not_admit_automatic_implicit_rows() {
    let mut input = sparse_grid(1, 1, 2);
    input.viewport.height = 40.0;
    input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
    input.nodes[2].style.height = LayoutDimension::Fixed(20.0);
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(40.0)];
    container.rows = vec![GridTrackSize::LengthPx(10.0)];
    container.auto_rows = GridAutoRows::Auto;
    container.placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;

    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect::<BTreeMap<_, _>>();
    let hidden = crate::grid_placement::hidden_subtrees(&input, &index);
    let node = &input.nodes[index[&id(1)]];
    let container = &input.grid_containers[&id(1)];
    let result = crate::grid_auto_placement::normalize_sparse_grid_container(
        &input, &index, &hidden, node, container,
    );
    assert!(
        matches!(result, Err(LayoutError::UnsupportedGridAutoRows { .. })),
        "legacy intrinsic track route unexpectedly accepted auto implicit rows: {:?}",
        result.as_ref().err()
    );
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
    let mut fixed_row_list = sparse_grid(1, 1, 2);
    let fixed_row_list_container = fixed_row_list.grid_containers.get_mut(&id(1)).unwrap();
    fixed_row_list_container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(15.0),
        GridTrackSize::LengthPx(25.0),
    ]);
    fixed_row_list_container.placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;
    let mut percentage_row_list = sparse_grid(1, 1, 2);
    let percentage_row_list_container =
        percentage_row_list.grid_containers.get_mut(&id(1)).unwrap();
    percentage_row_list_container.auto_rows =
        GridAutoRows::TrackList(vec![GridTrackSize::Percentage(0.25)]);
    percentage_row_list_container.placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;
    for node in fixed_row_list.nodes.iter_mut().skip(1) {
        node.style.display = LayoutDisplay::Block;
    }
    for node in percentage_row_list.nodes.iter_mut().skip(1) {
        node.style.display = LayoutDisplay::Block;
    }

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
            GridReservationSite::ExpandedRows,
            fixed_row_list,
            id(1),
            "grid-template-rows",
        ),
        (
            GridReservationSite::ExpandedRows,
            percentage_row_list,
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
