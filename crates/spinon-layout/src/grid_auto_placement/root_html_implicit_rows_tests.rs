use super::*;

#[path = "root_html_implicit_rows_tests/percentage.rs"]
mod percentage;

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

fn root_html_implicit_row_list_grid(
    height: f32,
    item_heights: &[f32],
    auto_rows: Vec<GridTrackSize>,
) -> LayoutInput {
    let mut input = sparse_grid(2, 1, item_heights.len());
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(5.0),
        row: LayoutLengthPercentage::length(5.0),
    };
    input.viewport.width = 100.0;
    input.viewport.height = height;
    for (index, (node, height)) in input
        .nodes
        .iter_mut()
        .skip(1)
        .zip(item_heights.iter().copied())
        .enumerate()
    {
        node.style.display = LayoutDisplay::Block;
        node.style.width = LayoutDimension::Fixed(if index % 2 == 0 { 40.0 } else { 55.0 });
        node.style.height = LayoutDimension::Fixed(height);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(55.0)];
    container.rows = vec![GridTrackSize::LengthPx(20.0)];
    container.auto_rows = GridAutoRows::TrackList(auto_rows);
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

#[test]
fn root_html_fixed_px_implicit_row_list_cycles_from_first_implicit_row() {
    let input = root_html_implicit_row_list_grid(
        100.0,
        &[20.0, 20.0, 15.0, 15.0, 25.0, 25.0, 15.0, 15.0],
        vec![GridTrackSize::LengthPx(15.0), GridTrackSize::LengthPx(25.0)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let frames = (2..=9)
        .map(|node| {
            let frame = result.frames[&id(node)];
            [frame.x, frame.y, frame.width, frame.height]
        })
        .collect::<Vec<_>>();

    assert_eq!(
        frames,
        vec![
            [0.0, 0.0, 40.0, 20.0],
            [45.0, 0.0, 55.0, 20.0],
            [0.0, 25.0, 40.0, 15.0],
            [45.0, 25.0, 55.0, 15.0],
            [0.0, 45.0, 40.0, 25.0],
            [45.0, 45.0, 55.0, 25.0],
            [0.0, 75.0, 40.0, 15.0],
            [45.0, 75.0, 55.0, 15.0],
        ]
    );
    assert_eq!(
        result.frames[&id(1)],
        crate::LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        }
    );
}

#[test]
fn root_html_fixed_px_implicit_row_list_accepts_zero_and_cycles_after_it() {
    let input = root_html_implicit_row_list_grid(
        45.0,
        &[20.0, 20.0, 0.0, 0.0, 15.0, 15.0],
        vec![GridTrackSize::LengthPx(0.0), GridTrackSize::LengthPx(15.0)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let frames = (2..=7)
        .map(|node| {
            let frame = result.frames[&id(node)];
            [frame.x, frame.y, frame.width, frame.height]
        })
        .collect::<Vec<_>>();

    assert_eq!(
        frames,
        vec![
            [0.0, 0.0, 40.0, 20.0],
            [45.0, 0.0, 55.0, 20.0],
            [0.0, 25.0, 40.0, 0.0],
            [45.0, 25.0, 55.0, 0.0],
            [0.0, 30.0, 40.0, 15.0],
            [45.0, 30.0, 55.0, 15.0],
        ]
    );
}

#[test]
fn root_html_fixed_px_implicit_row_list_rejects_unsupported_and_invalid_entries() {
    for (auto_rows, expect_invalid_track) in [
        (
            vec![GridTrackSize::LengthPx(15.0), GridTrackSize::Fraction(1.0)],
            false,
        ),
        (vec![GridTrackSize::LengthPx(-1.0)], true),
        (vec![GridTrackSize::LengthPx(f32::NAN)], true),
        (vec![GridTrackSize::LengthPx(f32::INFINITY)], true),
        (vec![GridTrackSize::Percentage(-0.5)], true),
        (vec![GridTrackSize::Percentage(f32::NAN)], true),
        (vec![GridTrackSize::Percentage(f32::INFINITY)], true),
    ] {
        let input = root_html_implicit_row_list_grid(
            60.0,
            &[20.0, 20.0, 15.0, 15.0, 15.0, 15.0],
            auto_rows,
        );
        let error = TaffyLayoutEngine
            .compute(&input)
            .expect_err("unsupported implicit-row track list must fail closed");
        if expect_invalid_track {
            assert!(matches!(
                error,
                LayoutError::InvalidGridTrack {
                    property: "grid-auto-rows",
                    ..
                }
            ));
        } else {
            assert!(matches!(error, LayoutError::UnsupportedGridAutoRows { .. }));
        }
    }
}

#[test]
fn root_html_fixed_px_implicit_row_list_rejects_invalid_values_without_implicit_rows() {
    for auto_rows in [
        vec![GridTrackSize::LengthPx(-1.0)],
        vec![GridTrackSize::LengthPx(f32::NAN)],
        vec![GridTrackSize::LengthPx(f32::INFINITY)],
    ] {
        let input = root_html_implicit_row_list_grid(100.0, &[20.0, 20.0], auto_rows);
        let error = TaffyLayoutEngine
            .compute(&input)
            .expect_err("invalid auto-row lists must fail even when no implicit row is needed");
        assert!(matches!(
            error,
            LayoutError::InvalidGridTrack {
                property: "grid-auto-rows",
                ..
            }
        ));
    }
}

#[test]
fn root_html_fixed_px_implicit_row_list_checks_list_limit_without_implicit_rows() {
    let auto_rows = vec![GridTrackSize::LengthPx(1.0); crate::grid::GRID_MAX_TRACKS + 1];
    let input = root_html_implicit_row_list_grid(100.0, &[20.0, 20.0], auto_rows);
    let error = TaffyLayoutEngine
        .compute(&input)
        .expect_err("oversized auto-row lists must fail even when no implicit row is needed");

    assert!(matches!(
        error,
        LayoutError::GridTrackLimitExceeded {
            property: "grid-auto-rows",
            count,
            maximum,
            ..
        } if count == crate::grid::GRID_MAX_TRACKS + 1 && maximum == crate::grid::GRID_MAX_TRACKS
    ));
}

#[test]
fn root_html_fixed_px_implicit_row_list_checks_resource_limit_before_expansion() {
    let mut input = sparse_grid(1, 1, crate::grid::GRID_MAX_TRACKS + 1);
    for node in input.nodes.iter_mut().skip(1) {
        node.style.display = LayoutDisplay::Block;
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(15.0),
        GridTrackSize::LengthPx(25.0),
    ]);
    container.placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridImplicitTrackRequired(node)) if node == id(1)
    ));
}
