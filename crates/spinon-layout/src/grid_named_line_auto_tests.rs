use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutInputRevision, LayoutNode,
    LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn grid_input(
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    root_width: f32,
    root_height: f32,
    item_width: f32,
    item_height: f32,
) -> LayoutInput {
    let root = id(1);
    let item = id(2);
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: root_width,
            height: root_height,
        },
        root_sizing: crate::RootSizingPolicy::Match,
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![item],
                style: LayoutStyle {
                    display: LayoutDisplay::Grid,
                    width: LayoutDimension::Fixed(root_width),
                    height: LayoutDimension::Fixed(root_height),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: item,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Fixed(item_width),
                    height: LayoutDimension::Fixed(item_height),
                    ..LayoutStyle::default()
                },
            },
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns,
                rows,
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                justify_items: Default::default(),
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::Auto,
                auto_columns: GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::TemplateGridPositiveNamedLineAutoImplicitTracks,
                track_sizing_trace_enabled: true,
            },
        )]),
        grid_items: BTreeMap::new(),
        text_paragraphs: BTreeMap::new(),
    }
}

fn final_axis_track_sizes(
    output: &crate::LayoutOutput,
    axis: LayoutGridTrackSizingAxis,
) -> Vec<f32> {
    let pass = output.grid_track_sizing_traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap();
    pass.stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

#[test]
fn positive_named_line_auto_tracks_size_both_axes_and_preserve_fixed_control() {
    let mut columns = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        80.0,
        20.0,
        30.0,
        10.0,
    );
    columns.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(4),
            column_end: GridPlacementInput::Line(5),
        },
    );
    let output = TaffyLayoutEngine.compute(&columns).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [80.0, 0.0, 30.0, 10.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
        [40.0, 40.0, 0.0, 30.0]
    );

    let container = columns.grid_containers.get_mut(&id(1)).unwrap();
    container.auto_columns = GridAutoColumns::LengthPx(30.0);
    let output = TaffyLayoutEngine.compute(&columns).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [110.0, 0.0, 30.0, 10.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
        [40.0, 40.0, 30.0, 30.0]
    );

    let mut rows = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        40.0,
        20.0,
        20.0,
        25.0,
    );
    rows.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(3),
            row_end: GridPlacementInput::Line(4),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let output = TaffyLayoutEngine.compute(&rows).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 20.0, 20.0, 25.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [20.0, 0.0, 25.0]
    );
}

#[test]
fn positive_named_line_auto_mode_rejects_an_auto_sized_sibling() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        80.0,
        20.0,
        30.0,
        10.0,
    );
    input.nodes[0].children.push(id(3));
    input.nodes.push(LayoutNode {
        id: id(3),
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    });
    let placement = GridItemInput {
        row_start: GridPlacementInput::Line(1),
        row_end: GridPlacementInput::Line(2),
        column_start: GridPlacementInput::Line(4),
        column_end: GridPlacementInput::Line(5),
    };
    input.grid_items.insert(id(2), placement.clone());
    input.grid_items.insert(id(3), placement);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridItemSize(node)) if node == id(3)
    ));
}

#[test]
fn fixed_only_named_line_mode_rejects_no_area_auto_implicit_tracks() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        80.0,
        20.0,
        30.0,
        10.0,
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(4),
            column_end: GridPlacementInput::Line(5),
        },
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoColumns { node, .. }) if node == id(1)
    ));
}
