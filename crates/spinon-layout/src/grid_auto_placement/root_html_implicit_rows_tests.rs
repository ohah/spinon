use super::*;

fn root_html_implicit_grid(
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    height: f32,
    column_gap: f32,
    row_gap: f32,
    item_widths: [f32; 3],
) -> LayoutInput {
    let mut input = sparse_grid(2, 1, item_widths.len());
    input.nodes[0].style.width = LayoutDimension::Fixed(120.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(column_gap),
        row: LayoutLengthPercentage::length(row_gap),
    };
    input.viewport.width = 120.0;
    input.viewport.height = height;
    for (node, width) in input.nodes.iter_mut().skip(1).zip(item_widths) {
        node.style.display = LayoutDisplay::Block;
        node.style.width = LayoutDimension::Fixed(width);
        node.style.height = LayoutDimension::Fixed(10.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = columns;
    container.rows = rows;
    container.auto_rows = GridAutoRows::LengthPx(15.0);
    container.placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;
    input
}

#[test]
fn root_html_fixed_implicit_rows_combine_direct_fraction_and_percentage_tracks() {
    let fraction_columns = root_html_implicit_grid(
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        35.0,
        0.0,
        0.0,
        [10.0; 3],
    );
    assert_eq!(
        [frame(&fraction_columns, 3).x, frame(&fraction_columns, 3).y],
        [40.0, 0.0]
    );
    assert_eq!(
        [frame(&fraction_columns, 4).x, frame(&fraction_columns, 4).y],
        [0.0, 20.0]
    );

    let percentage_columns = root_html_implicit_grid(
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Percentage(0.75),
        ],
        vec![GridTrackSize::LengthPx(20.0)],
        40.0,
        5.0,
        5.0,
        [10.0, 100.0, 10.0],
    );
    let second = frame(&percentage_columns, 3);
    let third = frame(&percentage_columns, 4);
    assert_eq!([second.x, second.y, second.width], [35.0, 0.0, 100.0]);
    assert_eq!(
        [third.x, third.y, third.width, third.height],
        [0.0, 25.0, 10.0, 10.0]
    );
    assert_eq!(second.x + second.width - 120.0, 15.0);

    let fraction_rows = root_html_implicit_grid(
        vec![GridTrackSize::LengthPx(120.0)],
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)],
        60.0,
        0.0,
        0.0,
        [10.0; 3],
    );
    assert_eq!(
        [
            frame(&fraction_rows, 2).y,
            frame(&fraction_rows, 3).y,
            frame(&fraction_rows, 4).y,
        ],
        [0.0, 15.0, 45.0]
    );

    let percentage_rows = root_html_implicit_grid(
        vec![GridTrackSize::LengthPx(120.0)],
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Percentage(0.25),
        ],
        80.0,
        0.0,
        0.0,
        [10.0; 3],
    );
    assert_eq!(
        [
            frame(&percentage_rows, 2).y,
            frame(&percentage_rows, 3).y,
            frame(&percentage_rows, 4).y,
        ],
        [0.0, 20.0, 40.0]
    );
}
