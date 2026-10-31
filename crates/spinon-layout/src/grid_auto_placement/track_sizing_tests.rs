use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::grid_auto_placement::{
    TestGridReservationSite as GridReservationSite, inject_failure_once,
};
use crate::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridItemInput,
    GridPlacementInput as P, GridPlacementMode, GridTrackSize, LayoutDimension, LayoutDisplay,
    LayoutEngine, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn item(row_start: P, row_end: P, column_start: P, column_end: P) -> GridItemInput {
    GridItemInput {
        row_start,
        row_end,
        column_start,
        column_end,
    }
}

fn grid(
    flow: GridAutoFlow,
    columns: &[f32],
    rows: &[f32],
    auto_rows: GridAutoRows,
    auto_columns: GridAutoColumns,
    child_count: usize,
) -> LayoutInput {
    let root = id(1);
    let children = (0..child_count)
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(400.0),
            height: LayoutDimension::Fixed(400.0),
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(children.iter().map(|child| LayoutNode {
        id: *child,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(1.0),
            height: LayoutDimension::Fixed(1.0),
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
            width: 400.0,
            height: 400.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: columns
                    .iter()
                    .copied()
                    .map(GridTrackSize::LengthPx)
                    .collect(),
                rows: rows.iter().copied().map(GridTrackSize::LengthPx).collect(),
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                justify_items: Default::default(),
                auto_flow: flow,
                auto_rows,
                auto_columns,
                placement_mode: GridPlacementMode::TrackSizing,
                track_sizing_trace_enabled: false,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

fn positions(input: &LayoutInput, count: usize) -> Vec<(f32, f32)> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    (0..count)
        .map(|index| {
            let frame = output.frames[&id(index as u64 + 2)];
            (frame.x, frame.y)
        })
        .collect()
}

#[test]
fn negative_track_lists_cycle_backwards_before_the_explicit_grid() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[10.0, 20.0],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(50.0),
        ]),
        3,
    );
    input.grid_items.insert(
        id(2),
        item(P::Line(1), P::Line(2), P::Line(-5), P::Line(-2)),
    );
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));
    input
        .grid_items
        .insert(id(4), item(P::Line(1), P::Line(2), P::Line(5), P::Line(6)));

    assert_eq!(
        positions(&input, 3),
        [(0.0, 0.0), (90.0, 0.0), (190.0, 0.0)]
    );
}

#[test]
fn signed_named_implicit_mode_expands_fixed_track_lists_on_both_sides() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[30.0, 40.0],
        &[20.0],
        GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(7.0),
        ]),
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(7.0),
        ]),
        2,
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridSignedImplicitTracks;
    for child in [id(2), id(3)] {
        input.nodes[child.get() as usize - 1].style.width = LayoutDimension::Auto;
        input.nodes[child.get() as usize - 1].style.min_width = LayoutDimension::Fixed(0.0);
    }
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.nodes[1].style.min_height = LayoutDimension::Fixed(0.0);
    input.grid_items.insert(
        id(2),
        item(P::Line(-4), P::Line(-2), P::Line(-5), P::Line(-3)),
    );
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(4), P::Line(6)));

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    let leading = output.frames[&id(2)];
    assert_eq!(
        (leading.x, leading.y, leading.width, leading.height),
        (0.0, 0.0, 22.0, 22.0)
    );
    let trailing = output.frames[&id(3)];
    assert_eq!(
        (trailing.x, trailing.y, trailing.width, trailing.height),
        (107.0, 22.0, 22.0, 1.0)
    );

    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect::<BTreeMap<_, _>>();
    let normalized = crate::grid_placement::normalize_grid_layout(&input, &index).unwrap();
    assert_eq!(
        normalized.expanded_rows[&id(1)],
        [
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(20.0),
        ]
    );
    assert_eq!(
        normalized.expanded_columns[&id(1)],
        [
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(15.0),
        ]
    );
}

#[test]
fn signed_named_implicit_mode_rejects_auto_placement() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[30.0, 40.0],
        &[20.0],
        GridAutoRows::LengthPx(20.0),
        GridAutoColumns::LengthPx(7.0),
        1,
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridSignedImplicitTracks;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::GridImplicitTrackRequired(node)) if node == id(1)
    ));
}

#[test]
fn negative_line_resolution_and_reverse_track_lists_work_on_both_axes() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[10.0, 20.0],
        &[12.0, 18.0],
        GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(5.0),
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(9.0),
        ]),
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(50.0),
        ]),
        3,
    );
    input.grid_items.insert(
        id(2),
        item(P::Line(-5), P::Line(-3), P::Line(-5), P::Line(-3)),
    );
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));
    input.grid_items.insert(
        id(4),
        item(P::Line(-3), P::Line(-5), P::Line(-3), P::Line(-5)),
    );

    assert_eq!(positions(&input, 3), [(0.0, 0.0), (90.0, 16.0), (0.0, 0.0)]);
}

#[test]
fn auto_placement_starts_at_the_leading_implicit_row() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[10.0, 10.0],
        &[10.0, 10.0],
        GridAutoRows::LengthPx(5.0),
        GridAutoColumns::LengthPx(5.0),
        3,
    );
    input.grid_items.insert(
        id(2),
        item(P::Line(-4), P::Line(-3), P::Line(1), P::Line(2)),
    );
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));

    assert_eq!(positions(&input, 3), [(0.0, 0.0), (0.0, 5.0), (10.0, 0.0)]);
}

#[test]
fn column_auto_placement_starts_at_the_leading_implicit_column() {
    let mut input = grid(
        GridAutoFlow::Column,
        &[10.0, 10.0],
        &[10.0, 10.0],
        GridAutoRows::LengthPx(5.0),
        GridAutoColumns::LengthPx(5.0),
        3,
    );
    input.grid_items.insert(
        id(2),
        item(P::Line(2), P::Line(3), P::Line(-4), P::Line(-3)),
    );
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));

    assert_eq!(positions(&input, 3), [(0.0, 10.0), (5.0, 0.0), (0.0, 0.0)]);
}

#[test]
fn span_against_a_negative_definite_line_creates_leading_tracks() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[10.0, 20.0],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(50.0),
        ]),
        2,
    );
    input
        .grid_items
        .insert(id(2), item(P::Line(1), P::Line(2), P::Span(2), P::Line(-3)));
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));

    assert_eq!(positions(&input, 2), [(0.0, 0.0), (90.0, 0.0)]);
}

#[test]
fn dense_and_sparse_cursors_keep_distinct_hole_placement_with_leading_columns() {
    let mut dense = grid(
        GridAutoFlow::RowDense,
        &[10.0, 10.0],
        &[10.0, 10.0],
        GridAutoRows::LengthPx(10.0),
        GridAutoColumns::LengthPx(5.0),
        4,
    );
    dense.grid_items.insert(
        id(2),
        item(P::Line(3), P::Line(4), P::Line(-4), P::Line(-3)),
    );
    dense
        .grid_items
        .insert(id(3), item(P::Auto, P::Auto, P::Span(2), P::Auto));
    dense
        .grid_items
        .insert(id(4), item(P::Auto, P::Auto, P::Span(2), P::Auto));
    dense
        .grid_items
        .insert(id(5), item(P::Auto, P::Auto, P::Auto, P::Auto));
    let dense_positions = positions(&dense, 4);

    let mut sparse = dense.clone();
    sparse.grid_containers.get_mut(&id(1)).unwrap().auto_flow = GridAutoFlow::Row;
    let sparse_positions = positions(&sparse, 4);

    assert_eq!(dense_positions[3], (15.0, 0.0));
    assert_eq!(sparse_positions[3], (15.0, 10.0));
}

#[test]
fn empty_explicit_grid_uses_its_single_line_as_the_negative_track_anchor() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(50.0),
        ]),
        2,
    );
    input
        .grid_items
        .insert(id(2), item(P::Line(1), P::Line(2), P::Line(-2), P::Line(1)));
    input
        .grid_items
        .insert(id(3), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));

    assert_eq!(positions(&input, 2), [(0.0, 0.0), (50.0, 0.0)]);
}

#[test]
fn non_track_sizing_profiles_still_reject_an_empty_explicit_track_list() {
    let mut input = grid(
        GridAutoFlow::Row,
        &[],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        0,
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::DenseAutoPlacement;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridTrackList {
            property: "grid-template-columns",
            ..
        })
    ));
}

#[test]
fn line_zero_and_more_than_ten_thousand_total_tracks_fail_closed() {
    let mut invalid_line = grid(
        GridAutoFlow::Row,
        &[10.0],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        1,
    );
    invalid_line
        .grid_items
        .insert(id(2), item(P::Auto, P::Auto, P::Line(0), P::Line(1)));
    assert!(matches!(
        TaffyLayoutEngine.compute(&invalid_line),
        Err(crate::LayoutError::InvalidGridPlacement {
            property: "grid-column-start",
            ..
        })
    ));

    let mut over_limit = grid(
        GridAutoFlow::Row,
        &[1.0],
        &[1.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        1,
    );
    over_limit.grid_items.insert(
        id(2),
        item(P::Line(1), P::Line(2), P::Line(-10_002), P::Line(-10_001)),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&over_limit),
        Err(crate::LayoutError::GridTrackLimitExceeded {
            property: "grid-template-columns",
            count: 10_001,
            maximum: 10_000,
            ..
        })
    ));

    for (start_line, end_line) in [(i32::MIN, i32::MIN + 1), (i32::MAX - 1, i32::MAX)] {
        let mut extreme_line = grid(
            GridAutoFlow::Row,
            &[1.0],
            &[1.0],
            GridAutoRows::Auto,
            GridAutoColumns::Auto,
            1,
        );
        extreme_line.grid_items.insert(
            id(2),
            item(
                P::Line(1),
                P::Line(2),
                P::Line(start_line),
                P::Line(end_line),
            ),
        );
        assert!(matches!(
            TaffyLayoutEngine.compute(&extreme_line),
            Err(crate::LayoutError::GridTrackLimitExceeded {
                property: "grid-template-columns",
                maximum: 10_000,
                ..
            })
        ));
    }
}

#[test]
fn track_sizing_reservation_failures_return_errors_and_recover_on_same_worker() {
    let visible_items = grid(
        GridAutoFlow::Row,
        &[10.0],
        &[10.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        1,
    );

    let mut row_locked = visible_items.clone();
    row_locked
        .grid_items
        .insert(id(2), item(P::Line(1), P::Line(2), P::Auto, P::Auto));

    let mut definite_item = visible_items.clone();
    definite_item
        .grid_items
        .insert(id(2), item(P::Line(1), P::Line(2), P::Line(1), P::Line(2)));

    let mut leading_rows = visible_items.clone();
    leading_rows
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    leading_rows.grid_items.insert(
        id(2),
        item(P::Line(-3), P::Line(-2), P::Line(1), P::Line(2)),
    );

    let mut leading_columns = visible_items.clone();
    leading_columns
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_columns = GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    leading_columns.grid_items.insert(
        id(2),
        item(P::Line(1), P::Line(2), P::Line(-3), P::Line(-2)),
    );

    let cases = vec![
        (
            GridReservationSite::VisibleItemList,
            visible_items.clone(),
            id(1),
            "grid-placement-occupancy",
        ),
        (
            GridReservationSite::RowLockedCursor,
            row_locked,
            id(1),
            "grid-placement-occupancy",
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
        (
            GridReservationSite::ExpandedRows,
            leading_rows,
            id(1),
            "grid-template-rows",
        ),
        (
            GridReservationSite::ExpandedColumns,
            leading_columns,
            id(1),
            "grid-template-columns",
        ),
    ];

    std::thread::spawn(move || {
        let engine = TaffyLayoutEngine;
        for (site, input, expected_node, expected_property) in cases {
            let guard = inject_failure_once(site);
            let result = engine.compute(&input);
            let observed_failure = crate::test_allocator::take_failure_observed();
            assert!(
                observed_failure,
                "{site:?}는 실제 allocator 할당을 실패시켜야 합니다"
            );
            assert!(matches!(
                result,
                Err(crate::LayoutError::GridAllocationFailure { node, property })
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
    .expect("track sizing reservation failure worker는 panic 없이 끝나야 합니다");
}
