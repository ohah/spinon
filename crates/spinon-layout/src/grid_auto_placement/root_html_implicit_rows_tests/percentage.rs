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

#[test]
fn root_html_percentage_implicit_row_list_cycles_from_first_implicit_row() {
    let input = root_html_implicit_row_list_grid(
        100.0,
        &[20.0, 20.0, 10.0, 10.0, 20.0, 20.0, 10.0, 10.0, 20.0, 20.0],
        vec![
            GridTrackSize::Percentage(0.1),
            GridTrackSize::Percentage(0.2),
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
            [0.0, 40.0, 40.0, 20.0],
            [45.0, 40.0, 55.0, 20.0],
            [0.0, 65.0, 40.0, 10.0],
            [45.0, 65.0, 55.0, 10.0],
            [0.0, 80.0, 40.0, 20.0],
            [45.0, 80.0, 55.0, 20.0],
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
fn root_html_mixed_px_percentage_implicit_rows_resolve_against_definite_height_and_overflow_visibly()
 {
    let input = root_html_implicit_row_list_grid(
        120.0,
        &[20.0, 20.0, 15.0, 15.0, 30.0, 30.0, 15.0, 15.0, 30.0, 30.0],
        vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::Percentage(0.25),
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
            [0.0, 25.0, 40.0, 15.0],
            [45.0, 25.0, 55.0, 15.0],
            [0.0, 45.0, 40.0, 30.0],
            [45.0, 45.0, 55.0, 30.0],
            [0.0, 80.0, 40.0, 15.0],
            [45.0, 80.0, 55.0, 15.0],
            [0.0, 100.0, 40.0, 30.0],
            [45.0, 100.0, 55.0, 30.0],
        ]
    );
    assert_eq!(
        result.frames[&id(1)],
        crate::LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 120.0,
        }
    );
    assert_eq!(
        result.frames[&id(11)].y + result.frames[&id(11)].height,
        130.0
    );
}

#[test]
fn root_html_percentage_auto_rows_compose_with_explicit_percentage_and_fraction_rows() {
    let mut input = root_html_implicit_row_list_grid(
        80.0,
        &[20.0, 20.0, 42.0, 42.0, 8.0, 8.0],
        vec![GridTrackSize::Percentage(0.1)],
    );
    input.grid_containers.get_mut(&id(1)).unwrap().rows = vec![
        GridTrackSize::Percentage(0.25),
        GridTrackSize::Fraction(1.0),
    ];
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
            [0.0, 25.0, 40.0, 42.0],
            [45.0, 25.0, 55.0, 42.0],
            [0.0, 72.0, 40.0, 8.0],
            [45.0, 72.0, 55.0, 8.0],
        ]
    );
}

#[test]
fn root_html_zero_height_percentage_implicit_row_is_zero_but_item_can_overflow() {
    let mut input = root_html_implicit_row_list_grid(
        0.0,
        &[0.0, 0.0, 7.0],
        vec![GridTrackSize::Percentage(0.5)],
    );
    input.grid_containers.get_mut(&id(1)).unwrap().rows = vec![GridTrackSize::LengthPx(0.0)];
    wrap_root_html_grid_in_fixed_parent(&mut input);
    let result = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        result.frames[&id(4)],
        crate::LayoutFrame {
            x: 0.0,
            y: 5.0,
            width: 40.0,
            height: 7.0,
        }
    );
}

#[test]
fn root_html_percentage_implicit_row_rejects_indefinite_constrained_or_aspect_ratio_height() {
    for (change, property) in [
        (0, "height"),
        (1, "min-height"),
        (2, "max-height"),
        (3, "aspect-ratio"),
    ] {
        let mut input = root_html_implicit_row_list_grid(
            100.0,
            &[20.0, 20.0, 10.0, 10.0],
            vec![GridTrackSize::Percentage(0.1)],
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
fn root_html_percentage_implicit_row_rejects_individual_and_expanded_range_overflow() {
    let individual = root_html_implicit_row_list_grid(
        f32::MAX,
        &[20.0, 20.0, 10.0, 10.0],
        vec![GridTrackSize::Percentage(2.0)],
    );
    let mut individual = individual;
    wrap_root_html_grid_in_fixed_parent(&mut individual);
    let individual_error = TaffyLayoutEngine.compute(&individual).unwrap_err();
    assert!(
        matches!(
            &individual_error,
            LayoutError::InvalidGridTrack {
                property: "grid-auto-rows",
                ..
            }
        ),
        "individual percentage overflow: {individual_error:?}"
    );

    let mut aggregate = root_html_implicit_row_list_grid(
        f32::MAX / 2.0,
        &[20.0, 20.0, 10.0, 10.0, 10.0, 10.0],
        vec![GridTrackSize::Percentage(1.000_001)],
    );
    wrap_root_html_grid_in_fixed_parent(&mut aggregate);
    let aggregate_error = TaffyLayoutEngine.compute(&aggregate).unwrap_err();
    assert!(
        matches!(
            &aggregate_error,
            LayoutError::InvalidGridTrack {
                property: "grid-auto-rows",
                ..
            }
        ),
        "aggregate percentage overflow: {aggregate_error:?}"
    );
}
