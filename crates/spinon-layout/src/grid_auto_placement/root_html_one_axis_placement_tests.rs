use super::*;

fn one_axis_grid(columns: usize, rows: usize, item_count: usize) -> LayoutInput {
    let mut input = sparse_grid(columns, rows, item_count);
    for item in input.nodes.iter_mut().skip(1) {
        item.style.display = LayoutDisplay::Block;
        item.style.width = LayoutDimension::Auto;
        item.style.height = LayoutDimension::Auto;
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = GridAutoColumns::Auto;
    container.placement_mode = GridPlacementMode::RootHtmlOneAxisAutoPlacementV1;
    input
}

fn mixed_explicit_one_axis_grid(columns: usize, rows: usize, item_count: usize) -> LayoutInput {
    let mut input = one_axis_grid(columns, rows, item_count);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlMixedExplicitOneAxisAutoPlacementV1;
    input
}

fn mixed_cross_axis_grid(columns: usize, rows: usize, item_count: usize) -> LayoutInput {
    let mut input = one_axis_grid(columns, rows, item_count);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlMixedCrossAxisAutoPlacementV1;
    input
}

#[test]
fn root_html_mixed_cross_axis_placement_orders_all_three_sparse_phases() {
    let mut input = mixed_cross_axis_grid(3, 3, 5);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    input.grid_items.insert(
        id(5),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );

    for (node, expected) in [
        (2, [0.0, 20.0, 40.0, 20.0]),
        (3, [0.0, 0.0, 40.0, 20.0]),
        (4, [0.0, 40.0, 40.0, 20.0]),
        (5, [40.0, 0.0, 40.0, 20.0]),
        (6, [40.0, 40.0, 40.0, 20.0]),
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
fn root_html_mixed_cross_axis_placement_requires_both_axes_and_rejects_implicit_extent() {
    let mut row_only = mixed_cross_axis_grid(3, 2, 2);
    row_only.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    row_only.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&row_only),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut column_only = mixed_cross_axis_grid(3, 2, 2);
    for node in [2, 3] {
        column_only.grid_items.insert(
            id(node),
            placement(
                GridPlacementInput::Auto,
                GridPlacementInput::Auto,
                GridPlacementInput::Line(1),
                GridPlacementInput::Line(2),
            ),
        );
    }
    assert!(matches!(
        TaffyLayoutEngine.compute(&column_only),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut implicit_row = mixed_cross_axis_grid(3, 2, 2);
    implicit_row.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(4),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    implicit_row.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(TaffyLayoutEngine.compute(&implicit_row).is_err());

    let mut implicit_column = mixed_cross_axis_grid(3, 2, 2);
    implicit_column.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    implicit_column.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(4),
            GridPlacementInput::Line(5),
        ),
    );
    assert!(TaffyLayoutEngine.compute(&implicit_column).is_err());
}

#[test]
fn root_html_mixed_cross_axis_placement_resolves_negative_lines_inside_explicit_grid() {
    let mut input = mixed_cross_axis_grid(3, 2, 2);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(-2),
            GridPlacementInput::Line(-1),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(-3),
            GridPlacementInput::Line(-2),
        ),
    );

    for (node, expected) in [(2, [0.0, 20.0, 40.0, 20.0]), (3, [40.0, 0.0, 40.0, 20.0])] {
        let actual = frame(&input, node);
        assert_eq!(
            [actual.x, actual.y, actual.width, actual.height],
            expected,
            "node {node}"
        );
    }
}

#[test]
fn root_html_mixed_cross_axis_placement_rejects_invalid_lines_spans_and_named_placement() {
    let mut zero_span = mixed_cross_axis_grid(3, 2, 2);
    zero_span.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Span(0),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    zero_span.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&zero_span),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut named_line = mixed_cross_axis_grid(3, 2, 2);
    named_line.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::NamedLine {
                name: "named".to_owned(),
                occurrence: 1,
            },
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    named_line.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&named_line),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut out_of_range_negative_line = mixed_cross_axis_grid(3, 2, 2);
    out_of_range_negative_line.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(-4),
            GridPlacementInput::Line(-3),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    out_of_range_negative_line.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&out_of_range_negative_line),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));
}

#[test]
fn root_html_one_axis_placement_places_column_locked_items_in_sparse_row_order() {
    let mut input = one_axis_grid(3, 3, 3);
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
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );

    for (node, expected) in [
        (2, [40.0, 0.0, 40.0, 20.0]),
        (3, [40.0, 20.0, 40.0, 20.0]),
        (4, [0.0, 40.0, 40.0, 20.0]),
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
fn root_html_one_axis_placement_places_row_locked_items_within_their_rows() {
    let mut input = one_axis_grid(3, 2, 3);
    for (node, row_start, row_end) in [(2, 1, 2), (3, 1, 2), (4, 2, 3)] {
        input.grid_items.insert(
            id(node),
            placement(
                GridPlacementInput::Line(row_start),
                GridPlacementInput::Line(row_end),
                GridPlacementInput::Auto,
                GridPlacementInput::Auto,
            ),
        );
    }

    for (node, expected) in [
        (2, [0.0, 0.0, 40.0, 20.0]),
        (3, [40.0, 0.0, 40.0, 20.0]),
        (4, [0.0, 20.0, 40.0, 20.0]),
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
fn root_html_one_axis_placement_preserves_spans_and_negative_lines() {
    let mut span_input = one_axis_grid(3, 2, 2);
    span_input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Span(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    span_input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    let first = frame(&span_input, 2);
    let second = frame(&span_input, 3);
    assert_eq!(
        [first.x, first.y, first.width, first.height],
        [0.0, 0.0, 40.0, 40.0]
    );
    assert_eq!(
        [second.x, second.y, second.width, second.height],
        [40.0, 20.0, 40.0, 20.0]
    );

    let mut negative_line_input = one_axis_grid(3, 2, 1);
    negative_line_input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(-2),
            GridPlacementInput::Line(-1),
        ),
    );
    let item = frame(&negative_line_input, 2);
    assert_eq!(
        [item.x, item.y, item.width, item.height],
        [80.0, 0.0, 40.0, 20.0]
    );
}

#[test]
fn root_html_one_axis_placement_rejects_mixed_axis_and_implicit_extent() {
    let mut mixed_axes = one_axis_grid(3, 2, 2);
    mixed_axes.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    mixed_axes.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&mixed_axes),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut implicit_row = one_axis_grid(3, 2, 1);
    implicit_row.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(4),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&implicit_row),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut implicit_auto_row = one_axis_grid(3, 2, 3);
    implicit_auto_row
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::LengthPx(10.0);
    for node in [2, 3, 4] {
        implicit_auto_row.grid_items.insert(
            id(node),
            placement(
                GridPlacementInput::Auto,
                GridPlacementInput::Auto,
                GridPlacementInput::Line(1),
                GridPlacementInput::Line(2),
            ),
        );
    }
    assert!(matches!(
        TaffyLayoutEngine.compute(&implicit_auto_row),
        Err(crate::LayoutError::GridImplicitTrackRequired(_))
    ));

    let mut implicit_auto_column = one_axis_grid(3, 2, 4);
    for node in [2, 3, 4, 5] {
        implicit_auto_column.grid_items.insert(
            id(node),
            placement(
                GridPlacementInput::Line(1),
                GridPlacementInput::Line(2),
                GridPlacementInput::Auto,
                GridPlacementInput::Auto,
            ),
        );
    }
    assert!(matches!(
        TaffyLayoutEngine.compute(&implicit_auto_column),
        Err(crate::LayoutError::GridImplicitTrackRequired(_))
    ));
}

#[test]
fn root_html_one_axis_placement_rejects_all_auto_and_fully_explicit_items() {
    let all_auto = one_axis_grid(3, 2, 1);
    assert!(matches!(
        TaffyLayoutEngine.compute(&all_auto),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut fully_explicit = one_axis_grid(3, 2, 1);
    fully_explicit.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&fully_explicit),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));
}

#[test]
fn root_html_mixed_placement_reserves_full_items_before_row_locked_items() {
    let mut input = mixed_explicit_one_axis_grid(3, 2, 3);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );

    for (node, expected) in [
        (2, [0.0, 0.0, 40.0, 20.0]),
        (3, [40.0, 0.0, 40.0, 20.0]),
        (4, [80.0, 0.0, 40.0, 20.0]),
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
fn root_html_mixed_placement_reserves_full_items_before_column_locked_spans() {
    let mut input = mixed_explicit_one_axis_grid(3, 3, 2);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(2),
            GridPlacementInput::Span(2),
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );

    let partial = frame(&input, 2);
    let full = frame(&input, 3);
    assert_eq!(
        [partial.x, partial.y, partial.width, partial.height],
        [40.0, 20.0, 80.0, 20.0]
    );
    assert_eq!(
        [full.x, full.y, full.width, full.height],
        [40.0, 0.0, 40.0, 20.0]
    );
}

#[test]
fn root_html_mixed_placement_rejects_missing_item_class_mixed_axes_and_implicit_extent() {
    let mut only_partial = mixed_explicit_one_axis_grid(3, 2, 1);
    only_partial.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&only_partial),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut only_full = mixed_explicit_one_axis_grid(3, 2, 1);
    only_full.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&only_full),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut mixed_axes = mixed_explicit_one_axis_grid(3, 2, 3);
    mixed_axes.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    mixed_axes.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    mixed_axes.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&mixed_axes),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut implicit_row = mixed_explicit_one_axis_grid(3, 2, 2);
    implicit_row
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::LengthPx(7.0);
    implicit_row.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(4),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    implicit_row.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    let error = TaffyLayoutEngine.compute(&implicit_row).unwrap_err();
    assert!(matches!(
        error,
        crate::LayoutError::InvalidGridPlacement { .. }
    ));
}

#[test]
fn root_html_mixed_placement_rejects_implicit_column_extent() {
    let mut input = mixed_explicit_one_axis_grid(3, 2, 2);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(4),
            GridPlacementInput::Line(5),
        ),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));
}
