use super::*;

fn wrap_root_html_grid_in_fixed_parent(input: &mut LayoutInput) {
    let grid = input.nodes.remove(0);
    let parent = id(100);
    input.nodes.insert(
        0,
        crate::LayoutNode {
            id: parent,
            children: vec![grid.id],
            style: crate::LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(100.0),
                ..crate::LayoutStyle::default()
            },
        },
    );
    input.nodes.insert(1, grid);
    input.root = parent;
    input.viewport.width = 100.0;
    input.viewport.height = 100.0;
}

fn root_html_single_column_rows(
    height: f32,
    item_heights: &[f32],
    explicit_row: f32,
    row_gap: f32,
    auto_rows: Vec<GridTrackSize>,
) -> LayoutInput {
    let mut input = root_html_implicit_row_list_grid(height, item_heights, auto_rows);
    input.nodes[0].style.gap.row = LayoutLengthPercentage::length(row_gap);
    input.viewport.height = height;
    for node in input.nodes.iter_mut().skip(1) {
        node.style.width = LayoutDimension::Fixed(10.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(100.0)];
    container.rows = vec![GridTrackSize::LengthPx(explicit_row)];
    input
}

#[test]
fn root_html_fractional_implicit_row_list_cycles_from_first_implicit_row() {
    let input = root_html_implicit_row_list_grid(
        100.0,
        &[20.0, 20.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0],
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let frames = (2..=11)
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
            [0.0, 25.0, 40.0, 10.0],
            [45.0, 25.0, 55.0, 10.0],
            [0.0, 40.0, 40.0, 10.0],
            [45.0, 40.0, 55.0, 10.0],
            [0.0, 65.0, 40.0, 10.0],
            [45.0, 65.0, 55.0, 10.0],
            [0.0, 80.0, 40.0, 10.0],
            [45.0, 80.0, 55.0, 10.0],
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
fn root_html_mixed_px_percentage_fraction_rows_allocate_remaining_space() {
    let input = root_html_implicit_row_list_grid(
        120.0,
        &[20.0, 20.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0],
        vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0),
        ],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let frames = (2..=11)
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
            [0.0, 25.0, 40.0, 10.0],
            [45.0, 25.0, 55.0, 10.0],
            [0.0, 45.0, 40.0, 10.0],
            [45.0, 45.0, 55.0, 10.0],
            [0.0, 80.0, 40.0, 10.0],
            [45.0, 80.0, 55.0, 10.0],
            [0.0, 105.0, 40.0, 10.0],
            [45.0, 105.0, 55.0, 10.0],
        ]
    );
    assert_eq!(result.frames[&id(1)].height, 120.0);
}

#[test]
fn root_html_zero_fractional_track_keeps_automatic_minimum_and_remaining_fr_share() {
    let input = root_html_single_column_rows(
        40.0,
        &[10.0, 10.0, 0.0, 0.0],
        10.0,
        5.0,
        vec![GridTrackSize::Fraction(0.0), GridTrackSize::Fraction(1.0)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let starts_and_heights = (2..=5)
        .map(|node| {
            let frame = result.frames[&id(node)];
            [frame.y, frame.height]
        })
        .collect::<Vec<_>>();

    assert_eq!(
        starts_and_heights,
        vec![[0.0, 10.0], [15.0, 10.0], [30.0, 0.0], [40.0, 0.0]]
    );
    assert_eq!(result.frames[&id(1)].height, 40.0);
}

#[test]
fn root_html_subunit_fractional_implicit_rows_preserve_trailing_space() {
    let input = root_html_single_column_rows(
        100.0,
        &[20.0, 10.0, 10.0],
        20.0,
        5.0,
        vec![GridTrackSize::Fraction(0.25)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    let starts = (2..=4)
        .map(|node| result.frames[&id(node)].y)
        .collect::<Vec<_>>();

    assert_eq!(starts, vec![0.0, 25.0, 47.5]);
    assert_eq!(result.frames[&id(1)].height, 100.0);
}

#[test]
fn root_html_fractional_implicit_row_preserves_fixed_item_minimum_overflow() {
    let input = root_html_single_column_rows(
        30.0,
        &[20.0, 40.0],
        20.0,
        5.0,
        vec![GridTrackSize::Fraction(1.0)],
    );
    let result = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(result.frames[&id(3)].y, 25.0);
    assert_eq!(result.frames[&id(3)].height, 40.0);
    assert_eq!(result.frames[&id(1)].height, 30.0);
    assert_eq!(
        result.frames[&id(3)].y + result.frames[&id(3)].height - 30.0,
        35.0
    );
}

#[test]
fn root_html_fraction_plus_overflowing_percentage_rows_are_not_clamped() {
    let mut input = root_html_single_column_rows(
        100.0,
        &[20.0, 120.0, 0.0],
        20.0,
        5.0,
        vec![GridTrackSize::Percentage(1.2), GridTrackSize::Fraction(1.0)],
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.viewport.width = 100.0;
    let result = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        [result.frames[&id(3)].y, result.frames[&id(3)].height],
        [25.0, 120.0]
    );
    assert_eq!(
        [result.frames[&id(4)].y, result.frames[&id(4)].height],
        [150.0, 0.0]
    );
    assert_eq!(result.frames[&id(1)].height, 100.0);
}

#[test]
fn root_html_fractional_implicit_row_resolves_zero_height_to_zero() {
    let mut input = root_html_single_column_rows(
        0.0,
        &[0.0, 0.0],
        0.0,
        0.0,
        vec![GridTrackSize::Fraction(1.0)],
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.viewport.width = 100.0;
    wrap_root_html_grid_in_fixed_parent(&mut input);
    let result = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(result.frames[&id(1)].height, 0.0);
    assert_eq!(result.frames[&id(3)].y, 0.0);
    assert_eq!(result.frames[&id(3)].height, 0.0);
}

#[test]
fn root_html_fractional_auto_rows_reject_indefinite_constrained_and_aspect_ratio_height() {
    for (change, property) in [
        (0, "height"),
        (1, "min-height"),
        (2, "max-height"),
        (3, "aspect-ratio"),
    ] {
        let mut input = root_html_single_column_rows(
            100.0,
            &[20.0, 10.0],
            20.0,
            5.0,
            vec![GridTrackSize::Fraction(1.0)],
        );
        let style = &mut input.nodes[0].style;
        match change {
            0 => style.height = LayoutDimension::Auto,
            1 => style.min_height = LayoutDimension::Fixed(10.0),
            2 => style.max_height = LayoutDimension::Fixed(100.0),
            3 => style.aspect_ratio = Some(1.0),
            _ => unreachable!(),
        }
        wrap_root_html_grid_in_fixed_parent(&mut input);
        let error = TaffyLayoutEngine.compute(&input).unwrap_err();
        assert!(
            matches!(
                &error,
                LayoutError::UnsupportedGridContainerStyle { property: actual, .. } if *actual == property
            ),
            "change {change} should reject {property}, got {error:?}"
        );
    }
}

#[test]
fn root_html_fractional_auto_rows_reject_invalid_factors() {
    for factor in [-1.0, f32::NAN, f32::INFINITY] {
        let input = root_html_single_column_rows(
            100.0,
            &[20.0, 10.0],
            20.0,
            5.0,
            vec![GridTrackSize::Fraction(factor)],
        );
        let error = TaffyLayoutEngine.compute(&input).unwrap_err();
        assert!(
            matches!(
                error,
                LayoutError::InvalidGridTrack {
                    property: "grid-auto-rows",
                    ..
                }
            ),
            "factor {factor:?} should be rejected"
        );
    }
}
