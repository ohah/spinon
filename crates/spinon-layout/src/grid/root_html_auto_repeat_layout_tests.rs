use super::{grid_input, id};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutDimension, LayoutEngine, LayoutError,
    LayoutGap, LayoutLengthPercentage, TaffyLayoutEngine,
};

fn root_html_auto_repeat_input(
    minima: &[f32],
    mode: GridAutoRepeatMode,
    width: f32,
    gap: f32,
    item_count: usize,
    item_width: LayoutDimension,
) -> crate::LayoutInput {
    let minima = minima
        .iter()
        .copied()
        .map(GridTrackMinSizingFunction::LengthPx)
        .collect::<Vec<_>>();
    root_html_auto_repeat_input_with_minima(&minima, mode, width, gap, item_count, item_width)
}

fn root_html_auto_repeat_input_with_minima(
    minima: &[GridTrackMinSizingFunction],
    mode: GridAutoRepeatMode,
    width: f32,
    gap: f32,
    item_count: usize,
    item_width: LayoutDimension,
) -> crate::LayoutInput {
    let columns = minima
        .iter()
        .copied()
        .map(|minimum| GridTrackSize::MinMax {
            min: minimum,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        })
        .collect::<Vec<_>>();
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: columns.len(),
        line_names: Vec::new(),
    };
    let mut input = grid_input(
        columns.clone(),
        vec![GridTrackSize::LengthPx(24.0)],
        item_count,
        LayoutGap {
            column: LayoutLengthPercentage::LengthPx(gap),
            ..LayoutGap::default()
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(24.0);
    input.viewport.width = width;
    input.viewport.height = 24.0;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns_auto_repeat = Some(repeat);
    container.placement_mode = GridPlacementMode::RootHtmlAutoRepeatV1;
    for item in input.nodes.iter_mut().skip(1) {
        item.style.width = item_width;
        item.style.height = LayoutDimension::Fixed(10.0);
    }
    input
}

fn root_html_auto_repeat_rows_input(
    minimum: GridTrackMinSizingFunction,
    height: f32,
    gap: f32,
    item_count: usize,
) -> crate::LayoutInput {
    let rows = vec![GridTrackSize::MinMax {
        min: minimum,
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    }];
    let repeat = GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: rows.len(),
        line_names: Vec::new(),
    };
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        rows.clone(),
        item_count,
        LayoutGap {
            row: LayoutLengthPercentage::LengthPx(gap),
            ..LayoutGap::default()
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport.width = 40.0;
    input.viewport.height = height;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.rows_auto_repeat = Some(repeat);
    container.placement_mode = GridPlacementMode::RootHtmlAutoRepeatV1;
    for item in input.nodes.iter_mut().skip(1) {
        item.style.width = LayoutDimension::Fixed(10.0);
        item.style.height = LayoutDimension::Fixed(10.0);
    }
    input
}

#[test]
fn auto_fill_zero_minimum_uses_the_ua_floor_for_count_and_keeps_track_origins() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFill,
        11.0,
        1.0,
        6,
        LayoutDimension::Fixed(0.0),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, child) in (2..=7).map(id).enumerate() {
        assert_eq!(output.frames[&child].x, (index * 2) as f32);
    }
}

#[test]
fn auto_fit_zero_minimum_collapses_empty_tracks_after_item_placement() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFit,
        11.0,
        0.0,
        1,
        LayoutDimension::Auto,
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let item = output.frames[&id(2)];
    assert_eq!((item.x, item.width), (0.0, 11.0));
}

#[test]
fn percentage_minimum_tracks_match_columns_rows_pair_and_zero_minimum_geometry() {
    let input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(0.1)],
        GridAutoRepeatMode::AutoFill,
        200.0,
        5.0,
        4,
        LayoutDimension::Fixed(10.0),
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, expected_x) in [0.0_f32, 25.625, 51.25, 76.875].into_iter().enumerate() {
        assert_eq!(output.frames[&id(index as u64 + 2)].x, expected_x);
    }

    let input = root_html_auto_repeat_input_with_minima(
        &[
            GridTrackMinSizingFunction::Percentage(0.1),
            GridTrackMinSizingFunction::Percentage(0.05),
        ],
        GridAutoRepeatMode::AutoFill,
        200.0,
        2.0,
        6,
        LayoutDimension::Fixed(10.0),
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, expected_x) in [0.0_f32, 22.0, 40.4, 62.4, 80.8, 102.8]
        .into_iter()
        .enumerate()
    {
        let actual_x = output.frames[&id(index as u64 + 2)].x;
        assert!(
            (actual_x - expected_x).abs() <= 0.02,
            "item {index}: {actual_x} vs {expected_x}"
        );
    }

    let input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(0.0)],
        GridAutoRepeatMode::AutoFill,
        200.0,
        5.0,
        3,
        LayoutDimension::Fixed(10.0),
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, expected_x) in [0.0_f32, 6.015625, 12.046875].into_iter().enumerate() {
        let actual_x = output.frames[&id(index as u64 + 2)].x;
        assert!(
            (actual_x - expected_x).abs() <= 0.02,
            "item {index}: {actual_x} vs {expected_x}"
        );
    }

    let input = root_html_auto_repeat_rows_input(
        GridTrackMinSizingFunction::Percentage(0.2),
        160.0,
        4.0,
        4,
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, expected_y) in [0.0_f32, 41.0, 82.0, 123.0].into_iter().enumerate() {
        assert_eq!(output.frames[&id(index as u64 + 2)].y, expected_y);
    }
}

#[test]
fn percentage_auto_fit_and_over_one_hundred_percent_track_keep_used_sizes() {
    let input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(0.1)],
        GridAutoRepeatMode::AutoFit,
        200.0,
        5.0,
        1,
        LayoutDimension::Auto,
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let item = output.frames[&id(2)];
    assert_eq!((item.x, item.width), (0.0, 200.0));

    let input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(1.2)],
        GridAutoRepeatMode::AutoFill,
        200.0,
        0.0,
        1,
        LayoutDimension::Auto,
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert!((output.frames[&id(2)].width - 240.0).abs() <= 0.02);
}

#[test]
fn percentage_minimum_rejects_unsupported_maximum_and_indefinite_axis() {
    let mut input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(0.1)],
        GridAutoRepeatMode::AutoFill,
        200.0,
        5.0,
        1,
        LayoutDimension::Fixed(10.0),
    );
    input.grid_containers.get_mut(&id(1)).unwrap().columns[0] = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Percentage(0.1),
        max: GridTrackMaxSizingFunction::Auto,
    };
    assert!(TaffyLayoutEngine.compute(&input).is_err());

    let mut input = root_html_auto_repeat_input_with_minima(
        &[GridTrackMinSizingFunction::Percentage(0.1)],
        GridAutoRepeatMode::AutoFill,
        200.0,
        5.0,
        1,
        LayoutDimension::Fixed(10.0),
    );
    input.nodes[0].style.width = LayoutDimension::Auto;
    assert!(TaffyLayoutEngine.compute(&input).is_err());
}

#[test]
fn over_limit_zero_minimum_rejects_before_producing_layout_output() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFill,
        10_001.0,
        0.0,
        0,
        LayoutDimension::Fixed(0.0),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackExpansionOverflow(node)) if node == id(1)
    ));
}
