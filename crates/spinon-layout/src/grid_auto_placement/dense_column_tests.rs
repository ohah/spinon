use std::collections::BTreeMap;

use crate::grid_auto_placement::{TestGridReservationSite, inject_failure_once};
use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

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
    column_sizes: &[f32],
    row_sizes: &[f32],
    auto_rows: GridAutoRows,
    auto_columns: GridAutoColumns,
    children: &[(f32, f32)],
) -> LayoutInput {
    let root = id(1);
    let child_ids = (0..children.len())
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: child_ids.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(column_sizes.iter().sum()),
            height: LayoutDimension::Fixed(row_sizes.iter().sum()),
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(
        child_ids
            .iter()
            .zip(children)
            .map(|(child, (width, height))| LayoutNode {
                id: *child,
                children: Vec::new(),
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(*width),
                    height: LayoutDimension::Fixed(*height),
                    ..LayoutStyle::default()
                },
            }),
    );
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: column_sizes.iter().sum(),
            height: row_sizes.iter().sum(),
        },
        root_sizing: RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: column_sizes
                    .iter()
                    .copied()
                    .map(GridTrackSize::LengthPx)
                    .collect(),
                rows: row_sizes
                    .iter()
                    .copied()
                    .map(GridTrackSize::LengthPx)
                    .collect(),
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                justify_items: Default::default(),
                auto_flow: flow,
                auto_rows,
                auto_columns,
                placement_mode: GridPlacementMode::DenseAutoPlacement,
                track_sizing_trace_enabled: false,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

fn root_html_dense(mut input: LayoutInput) -> LayoutInput {
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlDenseAutoPlacementV1;
    input.nodes[0].style.box_sizing = crate::LayoutBoxSizing::ContentBox;
    for node in input.nodes.iter_mut().skip(1) {
        node.style.display = LayoutDisplay::Block;
    }
    input
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
fn row_dense_fills_previous_hole_while_sparse_keeps_cursor_order() {
    let children = [(84.0, 18.0), (84.0, 18.0), (18.0, 18.0)];
    let mut dense = grid(
        GridAutoFlow::RowDense,
        &[40.0, 40.0, 40.0],
        &[20.0, 20.0],
        GridAutoRows::LengthPx(20.0),
        GridAutoColumns::Auto,
        &children,
    );
    dense
        .grid_items
        .insert(id(2), item(P::Auto, P::Auto, P::Auto, P::Span(2)));
    dense
        .grid_items
        .insert(id(3), item(P::Auto, P::Auto, P::Auto, P::Span(2)));
    assert_eq!(positions(&dense, 3), [(0.0, 0.0), (0.0, 20.0), (80.0, 0.0)]);

    dense.grid_containers.get_mut(&id(1)).unwrap().auto_flow = GridAutoFlow::Row;
    assert_eq!(
        positions(&dense, 3),
        [(0.0, 0.0), (0.0, 20.0), (80.0, 20.0)]
    );
}

#[test]
fn column_dense_fills_previous_hole_and_column_sparse_keeps_cursor_order() {
    let children = [(18.0, 40.0), (18.0, 40.0), (18.0, 18.0)];
    let mut dense = grid(
        GridAutoFlow::ColumnDense,
        &[40.0, 40.0],
        &[20.0, 20.0, 20.0],
        GridAutoRows::Auto,
        GridAutoColumns::LengthPx(28.0),
        &children,
    );
    dense
        .grid_items
        .insert(id(2), item(P::Auto, P::Span(2), P::Auto, P::Auto));
    dense
        .grid_items
        .insert(id(3), item(P::Auto, P::Span(2), P::Auto, P::Auto));
    assert_eq!(positions(&dense, 3), [(0.0, 0.0), (40.0, 0.0), (0.0, 40.0)]);

    dense.grid_containers.get_mut(&id(1)).unwrap().auto_flow = GridAutoFlow::Column;
    assert_eq!(
        positions(&dense, 3),
        [(0.0, 0.0), (40.0, 0.0), (40.0, 40.0)]
    );
}

#[test]
fn column_flow_expands_positive_implicit_columns_with_fixed_auto_size() {
    let input = grid(
        GridAutoFlow::Column,
        &[40.0],
        &[20.0, 20.0],
        GridAutoRows::Auto,
        GridAutoColumns::LengthPx(28.0),
        &[(18.0, 18.0); 5],
    );
    assert_eq!(
        positions(&input, 5),
        [
            (0.0, 0.0),
            (0.0, 20.0),
            (40.0, 0.0),
            (40.0, 20.0),
            (68.0, 0.0)
        ]
    );
}

#[test]
fn root_html_dense_mode_uses_row_and_column_dense_normalization() {
    let children = [(84.0, 18.0), (84.0, 18.0), (18.0, 18.0)];
    let mut row = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0, 40.0, 40.0],
        &[20.0, 20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &children,
    ));
    row.grid_items
        .insert(id(2), item(P::Auto, P::Auto, P::Auto, P::Span(2)));
    row.grid_items
        .insert(id(3), item(P::Auto, P::Auto, P::Auto, P::Span(2)));
    assert_eq!(positions(&row, 3), [(0.0, 0.0), (0.0, 20.0), (80.0, 0.0)]);

    let children = [(18.0, 40.0), (18.0, 40.0), (18.0, 18.0)];
    let mut column = root_html_dense(grid(
        GridAutoFlow::ColumnDense,
        &[40.0, 40.0],
        &[20.0, 20.0, 20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &children,
    ));
    column
        .grid_items
        .insert(id(2), item(P::Auto, P::Span(2), P::Auto, P::Auto));
    column
        .grid_items
        .insert(id(3), item(P::Auto, P::Span(2), P::Auto, P::Auto));
    assert_eq!(
        positions(&column, 3),
        [(0.0, 0.0), (40.0, 0.0), (0.0, 40.0)]
    );
}

#[test]
fn root_html_dense_auto_implicit_track_is_allowed_only_without_expansion() {
    let one_item = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0)],
    ));
    assert_eq!(positions(&one_item, 1), [(0.0, 0.0)]);

    let two_items = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0); 2],
    ));
    assert!(matches!(
        TaffyLayoutEngine.compute(&two_items),
        Err(crate::LayoutError::UnsupportedGridAutoRows { .. })
    ));
}

#[test]
fn root_html_column_dense_auto_columns_is_rejected_when_expansion_is_needed() {
    let two_items = root_html_dense(grid(
        GridAutoFlow::ColumnDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0); 2],
    ));
    assert!(matches!(
        TaffyLayoutEngine.compute(&two_items),
        Err(crate::LayoutError::UnsupportedGridAutoColumns { .. })
    ));
}

#[test]
fn root_html_dense_expanded_row_reservation_failure_is_atomic_and_recovers() {
    let input = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::LengthPx(20.0),
        GridAutoColumns::Auto,
        &[(18.0, 18.0); 2],
    ));

    std::thread::spawn(move || {
        let engine = TaffyLayoutEngine;
        let guard = inject_failure_once(TestGridReservationSite::ExpandedRows);
        let result = engine.compute(&input);
        assert!(
            crate::test_allocator::take_failure_observed(),
            "Root HTML dense 확장 row가 실제 reserve 할당을 실패시켜야 합니다"
        );
        assert!(matches!(
            result,
            Err(crate::LayoutError::GridAllocationFailure {
                node,
                property: "grid-template-rows"
            }) if node == id(1)
        ));
        drop(guard);

        let recovered = engine
            .compute(&input)
            .expect("같은 worker의 다음 Root HTML dense 계산은 복구해야 합니다");
        assert!(recovered.frames.contains_key(&id(3)));
        assert!(
            !crate::test_allocator::take_failure_observed(),
            "복구 계산에서 allocator failpoint이 다시 발생하면 안 됩니다"
        );
    })
    .join()
    .expect("Root HTML dense reservation failure worker는 panic 없이 끝나야 합니다");
}

#[test]
fn root_html_dense_rejects_grid_items_with_missing_node_handles() {
    let mut input = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0)],
    ));
    input.grid_items.insert(id(99), GridItemInput::default());

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnknownGridItem(node)) if node == id(99)
    ));
}

#[test]
fn root_html_dense_rejects_nonzero_order_and_negative_lines() {
    let mut ordered = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0)],
    ));
    ordered.nodes[1].style.order = 1;
    assert!(matches!(
        TaffyLayoutEngine.compute(&ordered),
        Err(crate::LayoutError::UnsupportedGridOrder(node)) if node == id(2)
    ));

    let mut negative_line = root_html_dense(grid(
        GridAutoFlow::RowDense,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0)],
    ));
    negative_line
        .grid_items
        .insert(id(2), item(P::Line(-1), P::Auto, P::Auto, P::Auto));
    assert!(matches!(
        TaffyLayoutEngine.compute(&negative_line),
        Err(crate::LayoutError::InvalidGridPlacement {
            node,
            property: "grid-row-start",
            ..
        }) if node == id(2)
    ));
}

#[test]
fn row_flow_expands_positive_implicit_rows_with_fixed_auto_size() {
    let input = grid(
        GridAutoFlow::RowDense,
        &[40.0, 40.0],
        &[20.0],
        GridAutoRows::LengthPx(25.0),
        GridAutoColumns::Auto,
        &[(18.0, 18.0); 5],
    );
    assert_eq!(
        positions(&input, 5),
        [
            (0.0, 0.0),
            (40.0, 0.0),
            (0.0, 20.0),
            (40.0, 20.0),
            (0.0, 45.0)
        ]
    );
}

#[test]
fn grid_auto_columns_auto_rejects_when_an_implicit_column_is_needed() {
    let input = grid(
        GridAutoFlow::Column,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::Auto,
        &[(18.0, 18.0); 2],
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridAutoColumns { .. })
    ));
}

#[test]
fn automatic_column_span_at_10000_tracks_succeeds_and_10001_fails_closed() {
    let mut maximum = grid(
        GridAutoFlow::RowDense,
        &[1.0],
        &[1.0],
        GridAutoRows::Auto,
        GridAutoColumns::LengthPx(1.0),
        &[(10_000.0, 1.0)],
    );
    maximum.grid_items.insert(
        id(2),
        item(P::Line(1), P::Line(2), P::Auto, P::Span(10_000)),
    );
    assert!(TaffyLayoutEngine.compute(&maximum).is_ok());

    maximum.nodes[1].style.width = LayoutDimension::Fixed(10_001.0);
    maximum.grid_items.insert(
        id(2),
        item(P::Line(1), P::Line(2), P::Auto, P::Span(10_001)),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&maximum),
        Err(crate::LayoutError::GridTrackLimitExceeded {
            property: "grid-template-columns",
            count: 10_001,
            maximum: 10_000,
            ..
        })
    ));
}

#[test]
fn dense_column_expanded_column_reservation_failure_returns_error_and_recovers_on_same_worker() {
    let input = grid(
        GridAutoFlow::Column,
        &[40.0],
        &[20.0],
        GridAutoRows::Auto,
        GridAutoColumns::LengthPx(28.0),
        &[(18.0, 18.0); 2],
    );

    std::thread::spawn(move || {
        let engine = TaffyLayoutEngine;
        let guard = inject_failure_once(TestGridReservationSite::ExpandedColumns);
        let result = engine.compute(&input);
        let observed_failure = crate::test_allocator::take_failure_observed();
        eprintln!(
            "GRID_RESERVATION_FAILURE site=ExpandedColumns observed={observed_failure} expected_node={} expected_property=grid-template-columns returned_error={}",
            id(1),
            matches!(result, Err(crate::LayoutError::GridAllocationFailure { .. }))
        );
        assert!(observed_failure, "ExpandedColumns가 실제 allocator 할당을 실패시켜야 합니다");
        assert!(matches!(
            result,
            Err(crate::LayoutError::GridAllocationFailure { node, property: "grid-template-columns" })
                if node == id(1)
        ));
        drop(guard);

        let recovered = engine
            .compute(&input)
            .expect("같은 worker에서 다음 정상 dense·column layout은 복구해야 합니다");
        assert!(recovered.frames.contains_key(&id(3)));
        assert!(
            !crate::test_allocator::take_failure_observed(),
            "복구 계산은 allocator failpoint 없이 수행되어야 합니다"
        );
    })
    .join()
    .expect("ExpandedColumns 실패 주입 worker는 panic 없이 끝나야 합니다");
}
