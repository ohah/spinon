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
