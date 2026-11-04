use super::*;

fn column_grid(columns: usize, rows: usize, item_count: usize) -> LayoutInput {
    let mut input = mixed_cross_axis_grid(columns, rows, item_count);
    input.nodes[0].style.width = LayoutDimension::Fixed(columns as f32 * 40.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(rows as f32 * 30.0);
    input.viewport = Viewport {
        width: columns as f32 * 40.0,
        height: rows as f32 * 30.0,
    };
    for item in input.nodes.iter_mut().skip(1) {
        item.style.width = LayoutDimension::Fixed(40.0);
        item.style.height = LayoutDimension::Fixed(30.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(40.0); columns];
    container.rows = vec![GridTrackSize::LengthPx(30.0); rows];
    container.auto_flow = GridAutoFlow::Column;
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = GridAutoColumns::Auto;
    container.placement_mode = GridPlacementMode::RootHtmlMixedCrossAxisColumnAutoPlacementV1;
    input
}

fn assert_frames(input: &LayoutInput, expected: &[(u64, [f32; 4])]) {
    for &(node, expected) in expected {
        let actual = frame(input, node);
        assert_eq!(
            [actual.x, actual.y, actual.width, actual.height],
            expected,
            "node {node}"
        );
    }
}

#[test]
fn column_mixed_cross_axis_places_definite_locked_and_auto_items_in_sparse_phases() {
    let mut input = column_grid(4, 3, 6);
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
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
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
    input.grid_items.insert(
        id(5),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    input.grid_items.insert(
        id(6),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );

    assert_frames(
        &input,
        &[
            (2, [0.0, 0.0, 40.0, 30.0]),
            (3, [40.0, 30.0, 40.0, 30.0]),
            (4, [40.0, 0.0, 40.0, 30.0]),
            (5, [0.0, 30.0, 40.0, 30.0]),
            (6, [80.0, 0.0, 40.0, 30.0]),
            (7, [80.0, 30.0, 40.0, 30.0]),
        ],
    );
}

#[test]
fn column_mixed_cross_axis_accounts_for_two_axis_spans_and_cursor_occupancy() {
    let mut input = column_grid(6, 3, 5);
    input.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Span(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.nodes[1].style.height = LayoutDimension::Fixed(60.0);
    input.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Span(2),
        ),
    );
    input.nodes[2].style.width = LayoutDimension::Fixed(80.0);
    input.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(3),
            GridPlacementInput::Line(4),
        ),
    );
    input.grid_items.insert(
        id(5),
        placement(
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(3),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    input.grid_items.insert(
        id(6),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(4),
            GridPlacementInput::Line(5),
        ),
    );

    assert_frames(
        &input,
        &[
            (2, [160.0, 0.0, 40.0, 60.0]),
            (3, [0.0, 0.0, 80.0, 30.0]),
            (4, [80.0, 30.0, 40.0, 30.0]),
            (5, [200.0, 30.0, 40.0, 30.0]),
            (6, [120.0, 0.0, 40.0, 30.0]),
        ],
    );
}

#[test]
fn column_mixed_cross_axis_resolves_negative_lines_and_rejects_implicit_extent() {
    let mut negative = column_grid(4, 3, 3);
    negative.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    negative.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(-2),
            GridPlacementInput::Line(-1),
        ),
    );
    negative.grid_items.insert(
        id(4),
        placement(
            GridPlacementInput::Line(-2),
            GridPlacementInput::Line(-1),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    assert_frames(
        &negative,
        &[
            (2, [0.0, 0.0, 40.0, 30.0]),
            (3, [120.0, 0.0, 40.0, 30.0]),
            (4, [0.0, 60.0, 40.0, 30.0]),
        ],
    );

    let mut outside = column_grid(4, 3, 2);
    outside.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(4),
            GridPlacementInput::Line(5),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    outside.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(5),
            GridPlacementInput::Line(6),
        ),
    );
    let outside_result = TaffyLayoutEngine.compute(&outside);
    assert!(
        matches!(
            outside_result,
            Err(crate::LayoutError::InvalidGridPlacement {
                node,
                property: "grid-row-end",
                ..
            }) if node == id(2)
        ),
        "{outside_result:?}"
    );
}

#[test]
fn column_mixed_cross_axis_requires_both_lock_classes_and_auto_implicit_columns() {
    let mut one_axis = column_grid(4, 3, 2);
    for node in [2, 3] {
        one_axis.grid_items.insert(
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
        TaffyLayoutEngine.compute(&one_axis),
        Err(crate::LayoutError::InvalidGridPlacement { .. })
    ));

    let mut auto_columns = column_grid(4, 3, 2);
    auto_columns
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_columns = GridAutoColumns::LengthPx(12.0);
    auto_columns.grid_items.insert(
        id(2),
        placement(
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
        ),
    );
    auto_columns.grid_items.insert(
        id(3),
        placement(
            GridPlacementInput::Auto,
            GridPlacementInput::Auto,
            GridPlacementInput::Line(1),
            GridPlacementInput::Line(2),
        ),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&auto_columns),
        Err(crate::LayoutError::UnsupportedGridAutoColumns { node, .. }) if node == id(1)
    ));
}
