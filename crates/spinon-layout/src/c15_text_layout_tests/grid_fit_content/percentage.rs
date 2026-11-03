use super::*;

#[test]
fn ahem_fit_content_percentage_columns_resolve_against_definite_width() {
    for (percentage, expected) in [
        (0.0, 20.0),
        (0.05, 20.0),
        (0.2, 40.0),
        (0.6, 110.0),
        (1.2, 110.0),
    ] {
        let mut input = fit_content_input(40.0, 200.0, 60.0);
        let grid = input.grid_containers.get_mut(&node(1)).unwrap();
        grid.columns = vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Percentage(percentage),
        )];
        let output = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &text_engine())
            .expect("definite Root HTML percentage column limits should size");

        assert_eq!(
            final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
            [expected],
            "fit-content({}%) column used track",
            percentage * 100.0
        );
        assert_eq!(output.frames[&node(2)].width, expected);
    }
}

#[test]
fn ahem_fit_content_percentage_over_100_cap_keeps_the_available_space_ceiling() {
    // The cap resolves to 60px, but the definite grid has only 50px of available space.
    let mut input = fit_content_input(40.0, 50.0, 60.0);
    let grid = input.grid_containers.get_mut(&node(1)).unwrap();
    grid.columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(1.2),
    )];
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .expect("finite over-100% fit-content cap should size against its resolved basis");

    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
        [50.0],
        "the 60px fit-content cap cannot grow a definite 50px grid beyond its available space"
    );
    assert_eq!(output.frames[&node(2)].width, 50.0);
}

#[test]
fn ahem_fit_content_percentage_uses_content_box_basis_with_a_border() {
    let text_paragraph = input(30.0, "A A", WhiteSpaceMode::Normal)
        .text_paragraphs
        .remove(&node(2))
        .expect("Ahem text input should provide a paragraph");
    let mut input = fit_content_input(0.0, 30.0, 20.0);
    input.text_paragraphs.insert(node(2), text_paragraph);
    input.nodes[0].style.box_sizing = LayoutBoxSizing::ContentBox;
    input.nodes[0].style.border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    let grid = input.grid_containers.get_mut(&node(1)).unwrap();
    grid.columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.5),
    )];
    grid.placement_mode = GridPlacementMode::TrackSizing;

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .expect("percentage fit-content should resolve from the bordered grid content box");

    assert_eq!(output.frames[&node(1)].width, 34.0);
    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
        [15.0],
        "50% of the 30px content box is 15px; the 34px border box would produce 17px"
    );
    assert_eq!(output.frames[&node(2)].width, 15.0);
}

#[test]
fn ahem_fit_content_percentage_cap_does_not_subtract_the_grid_gap() {
    let mut input = fit_content_input(40.0, 200.0, 60.0);
    input.nodes[0].style.gap.column = crate::LayoutLengthPercentage::LengthPx(20.0);
    let grid = input.grid_containers.get_mut(&node(1)).unwrap();
    grid.columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.5),
    )];
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .expect("percentage fit-content cap should use the content width, not width minus gap");

    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
        [100.0],
        "50% of the 200px content width is 100px; the 20px gap is not subtracted"
    );
}

#[test]
fn ahem_fit_content_percentage_rows_keep_minimum_floor_at_zero_and_over_100_percent() {
    for (percentage, height) in [(0.0, 100.0), (0.2, 100.0), (1.2, 100.0), (1.0, 0.0)] {
        let mut input = fit_content_input(40.0, 20.0, height);
        if height == 0.0 {
            // The viewport remains valid while the Grid's own definite height is zero.
            input.viewport.height = 100.0;
            input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
        }
        let grid = input.grid_containers.get_mut(&node(1)).unwrap();
        grid.columns = vec![GridTrackSize::LengthPx(20.0)];
        grid.rows = vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Percentage(percentage),
        )];
        let output = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &text_engine())
            .expect("definite Root HTML percentage row limits should size");

        assert_eq!(
            final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Rows),
            [40.0],
            "fit-content({}%) row with {height}px basis keeps its Ahem minimum floor",
            percentage * 100.0
        );
        assert_eq!(output.frames[&node(2)].height, 40.0);
    }
}

#[test]
fn ahem_fit_content_percentage_fixed_repeat_tracks_size_independently() {
    let mut input = fit_content_input(40.0, 200.0, 60.0);
    let grid = input.grid_containers.get_mut(&node(1)).unwrap();
    grid.columns = vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
    ];
    add_second_text_item(&mut input);
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .expect("definite percentage limits in expanded fixed repeat should size");

    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
        [40.0, 40.0]
    );
    assert_eq!(output.frames[&node(2)].width, 40.0);
    assert_eq!(output.frames[&node(4)].x, 40.0);
    assert_eq!(output.frames[&node(4)].width, 40.0);
}

#[test]
fn root_html_fit_content_percentage_tracks_reject_indefinite_axes() {
    let mut column_input = root_html_fit_content_fixed_input(200.0, 60.0, 40.0, 20.0);
    let mut column_grid_node = column_input.nodes[0].clone();
    let mut column_item = column_input.nodes[1].clone();
    column_grid_node.style.display = crate::LayoutDisplay::Flex;
    column_grid_node.style.flex_direction = crate::FlexDirection::Row;
    column_grid_node.style.align_items = crate::LayoutAlignItems::FlexStart;
    column_grid_node.children = vec![node(3)];
    column_item.id = node(3);
    column_grid_node.style.width = LayoutDimension::Fixed(200.0);
    column_grid_node.style.height = LayoutDimension::Fixed(60.0);
    column_item.style.display = crate::LayoutDisplay::Grid;
    column_item.style.width = LayoutDimension::Auto;
    column_item.style.height = LayoutDimension::Fixed(40.0);
    column_item.children = vec![node(4)];
    let column_grid = column_input.grid_containers.remove(&node(1)).unwrap();
    column_input.grid_items.clear();
    column_input.nodes = vec![
        column_grid_node,
        column_item,
        LayoutNode {
            id: node(4),
            children: Vec::new(),
            style: LayoutStyle {
                width: LayoutDimension::Fixed(40.0),
                height: LayoutDimension::Fixed(20.0),
                ..LayoutStyle::default()
            },
        },
    ];
    column_input.grid_containers.insert(node(3), column_grid);
    column_input
        .grid_containers
        .get_mut(&node(3))
        .unwrap()
        .columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.2),
    )];
    column_input.grid_items.insert(
        node(4),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    column_input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
    let column_error = TaffyLayoutEngine
        .compute_with_text_layout_engine(&column_input, &text_engine())
        .expect_err("percentage fit-content columns require a definite width");
    assert!(matches!(
        column_error,
        crate::LayoutError::IndefiniteGridBasis { axis: "width", .. }
    ));

    let mut row_input = root_html_fit_content_fixed_input(100.0, 100.0, 20.0, 40.0);
    let mut row_root = row_input.nodes[0].clone();
    let mut row_grid_node = row_input.nodes[1].clone();
    row_root.children = vec![node(3)];
    row_root.style.display = crate::LayoutDisplay::Block;
    row_grid_node.id = node(3);
    row_grid_node.style.display = crate::LayoutDisplay::Grid;
    row_grid_node.style.width = LayoutDimension::Fixed(20.0);
    row_grid_node.style.height = LayoutDimension::Auto;
    row_grid_node.children = vec![node(4)];
    let row_grid = row_input.grid_containers.remove(&node(1)).unwrap();
    row_input.grid_items.clear();
    row_input.nodes = vec![
        row_root,
        row_grid_node,
        LayoutNode {
            id: node(4),
            children: Vec::new(),
            style: LayoutStyle {
                width: LayoutDimension::Fixed(20.0),
                height: LayoutDimension::Fixed(40.0),
                ..LayoutStyle::default()
            },
        },
    ];
    row_input.grid_containers.insert(node(3), row_grid);
    let row_grid = row_input.grid_containers.get_mut(&node(3)).unwrap();
    row_grid.columns = vec![GridTrackSize::LengthPx(20.0)];
    row_grid.rows = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.2),
    )];
    row_input.grid_items.insert(
        node(4),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let row_error = TaffyLayoutEngine
        .compute_with_text_layout_engine(&row_input, &text_engine())
        .expect_err("percentage fit-content rows require a definite height");
    assert!(matches!(
        row_error,
        crate::LayoutError::IndefiniteGridBasis { axis: "height", .. }
    ));
}

#[test]
fn root_html_fit_content_percentage_tracks_reject_negative_and_non_finite_limits() {
    for percentage in [-0.01, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut input = root_html_fit_content_fixed_input(200.0, 60.0, 40.0, 20.0);
        let grid = input.grid_containers.get_mut(&node(1)).unwrap();
        grid.columns = vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Percentage(percentage),
        )];
        let error = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &text_engine())
            .expect_err("negative or non-finite percentage limits must fail before layout");
        assert!(
            matches!(
                error,
                crate::LayoutError::InvalidGridTrack {
                    property: "grid-template-columns",
                    ..
                }
            ),
            "unexpected invalid percentage rejection: {error:?}"
        );
    }
}

#[test]
fn root_html_fit_content_percentage_columns_rows_and_zero_basis_are_admitted() {
    let mut columns = root_html_fit_content_fixed_input(200.0, 60.0, 60.0, 20.0);
    columns.grid_containers.get_mut(&node(1)).unwrap().columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.2),
    )];
    let output = TaffyLayoutEngine.compute(&columns).unwrap();
    assert_eq!(output.frames[&node(2)].width, 60.0);

    let mut rows = root_html_fit_content_fixed_input(20.0, 100.0, 20.0, 40.0);
    let row_grid = rows.grid_containers.get_mut(&node(1)).unwrap();
    row_grid.columns = vec![GridTrackSize::LengthPx(20.0)];
    row_grid.rows = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.2),
    )];
    let output = TaffyLayoutEngine.compute(&rows).unwrap();
    assert_eq!(output.frames[&node(2)].height, 40.0);

    let mut zero_basis = root_html_fit_content_fixed_input(0.0, 20.0, 20.0, 20.0);
    zero_basis.viewport.width = 100.0;
    zero_basis.root_sizing = crate::RootSizingPolicy::ResolveWithin;
    zero_basis
        .grid_containers
        .get_mut(&node(1))
        .unwrap()
        .columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(1.0),
    )];
    let output = TaffyLayoutEngine.compute(&zero_basis).unwrap();
    assert_eq!(output.frames[&node(2)].width, 20.0);
}
