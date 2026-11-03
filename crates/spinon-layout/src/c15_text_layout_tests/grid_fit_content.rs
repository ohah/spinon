use super::{input, node, text_engine};
use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutDimension, LayoutGridTrackSizingAxis,
    LayoutNode, LayoutStyle, TaffyLayoutEngine, TextParagraphInput, WhiteSpaceMode,
};

fn fit_content_input(limit: f32, width: f32, height: f32) -> crate::LayoutInput {
    let mut input = input(width, "AA AA AA AA", WhiteSpaceMode::Normal);
    input.nodes[0].style.display = crate::LayoutDisplay::Grid;
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport.width = width;
    input.viewport.height = height;
    input.nodes[0].style.align_content = Some(crate::LayoutAlignContent::FlexStart);
    input.nodes[0].style.justify_content = crate::LayoutJustifyContent::FlexStart;
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.grid_containers.insert(
        node(1),
        GridContainerInput {
            columns: vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(limit),
            )],
            rows: vec![GridTrackSize::LengthPx(height)],
            placement_mode: GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        },
    );
    input.grid_items.insert(
        node(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    input
}

fn final_track_sizes(
    output: &crate::LayoutOutput,
    node_id: crate::NodeId,
    axis: LayoutGridTrackSizingAxis,
) -> Vec<f32> {
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == node_id)
        .expect("Grid track sizing trace should be recorded");
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .expect("requested Grid axis should be traced");
    pass.stages
        .last()
        .expect("Grid axis should have a final stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn add_second_text_item(input: &mut crate::LayoutInput) {
    let second_node = node(4);
    let second_text_node = node(5);
    input.nodes[0].children.push(second_node);
    input.nodes.push(LayoutNode {
        id: second_node,
        children: Vec::new(),
        style: LayoutStyle {
            display: crate::LayoutDisplay::Block,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Auto,
            ..LayoutStyle::default()
        },
    });
    let TextParagraphInput::Paragraph(mut paragraph) = input.text_paragraphs[&node(2)].clone()
    else {
        panic!("input helper should produce a text paragraph");
    };
    paragraph.node = second_node;
    paragraph.sources[0].node = second_text_node;
    input
        .text_paragraphs
        .insert(second_node, TextParagraphInput::Paragraph(paragraph));
    input.grid_items.insert(
        second_node,
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );
}

#[test]
fn ahem_text_fit_content_columns_respect_minimum_cap_and_maximum_contributions() {
    for (limit, expected) in [(0.0, 20.0), (10.0, 20.0), (40.0, 40.0), (120.0, 110.0)] {
        let input = fit_content_input(limit, 200.0, 60.0);
        let output = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &text_engine())
            .unwrap();

        assert_eq!(
            final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
            [expected],
            "fit-content({limit}px) used track size"
        );
        let intrinsic = output
            .grid_item_intrinsic_inputs
            .iter()
            .find(|measurement| measurement.node == node(2))
            .expect("Ahem Grid item intrinsic input should be recorded")
            .contribution_input()
            .unwrap();
        assert_eq!(intrinsic.width.min_content, Some(20.0));
        assert_eq!(intrinsic.width.max_content, Some(110.0));
    }
}

#[test]
fn ahem_text_fixed_repeat_fit_content_tracks_are_sized_independently() {
    let mut input = fit_content_input(40.0, 200.0, 60.0);
    input.grid_containers.get_mut(&node(1)).unwrap().columns = vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
        GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
    ];
    add_second_text_item(&mut input);
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();

    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Columns),
        [40.0, 40.0]
    );
    assert_eq!(output.frames[&node(2)].width, 40.0);
    assert_eq!(output.frames[&node(4)].x, 40.0);
    assert_eq!(output.frames[&node(4)].width, 40.0);
}

#[test]
fn ahem_wrapped_min_content_sets_a_fit_content_row_floor_above_its_cap() {
    let mut input = fit_content_input(20.0, 20.0, 20.0);
    input.grid_containers.get_mut(&node(1)).unwrap().columns = vec![GridTrackSize::LengthPx(20.0)];
    input.grid_containers.get_mut(&node(1)).unwrap().rows = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(20.0),
    )];
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();

    assert_eq!(
        final_track_sizes(&output, node(1), LayoutGridTrackSizingAxis::Rows),
        [40.0]
    );
    assert_eq!(output.frames[&node(2)].height, 40.0);
    let intrinsic = output
        .grid_item_intrinsic_inputs
        .iter()
        .find(|measurement| measurement.node == node(2))
        .expect("Ahem row item intrinsic input should be recorded")
        .contribution_input()
        .unwrap();
    assert_eq!(intrinsic.height.min_content, Some(40.0));
    assert_eq!(intrinsic.height.max_content, Some(10.0));
}

#[test]
fn root_html_fit_content_mode_rejects_percentage_limits() {
    let mut input = fit_content_input(40.0, 200.0, 60.0);
    let grid = input.grid_containers.get_mut(&node(1)).unwrap();
    grid.placement_mode = GridPlacementMode::RootHtmlFitContentV1;
    grid.columns = vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.5),
    )];

    let error = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .expect_err("Root HTML fit-content mode must reject percentage limits");
    assert!(
        matches!(
            error,
            crate::LayoutError::InvalidGridTrack {
                property: "grid-template-columns",
                ..
            }
        ),
        "unexpected Root HTML fit-content rejection: {error:?}"
    );
}
