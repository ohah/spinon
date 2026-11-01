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
fn positive_named_line_pair_auto_tracks_cover_equal_reverse_span_and_rows() {
    let mut columns = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        80.0,
        20.0,
        30.0,
        10.0,
    );
    columns
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridPositiveNamedLinePairAutoImplicitTracks;

    let set_columns = |input: &mut LayoutInput, start, end| {
        input.grid_items.insert(
            id(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(start),
                column_end: GridPlacementInput::Line(end),
            },
        );
    };

    for (start, end, expected_tracks) in [
        (4, 5, vec![40.0, 40.0, 0.0, 30.0]),
        (4, 4, vec![40.0, 40.0, 0.0, 30.0]),
        (6, 4, vec![40.0, 40.0, 0.0, 15.0, 15.0]),
        (4, 6, vec![40.0, 40.0, 0.0, 15.0, 15.0]),
    ] {
        set_columns(&mut columns, start, end);
        let output = TaffyLayoutEngine.compute(&columns).unwrap();
        let frame = output.frames[&id(2)];
        assert_eq!(
            [frame.x, frame.y, frame.width, frame.height],
            [80.0, 0.0, 30.0, 10.0],
            "column lines {start}/{end}"
        );
        assert_eq!(
            final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
            expected_tracks,
            "column lines {start}/{end}"
        );
    }

    let mut rows = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(20.0)],
        40.0,
        40.0,
        20.0,
        25.0,
    );
    rows.grid_containers.get_mut(&id(1)).unwrap().placement_mode =
        GridPlacementMode::TemplateGridPositiveNamedLinePairAutoImplicitTracks;
    rows.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(4),
            row_end: GridPlacementInput::Line(5),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let output = TaffyLayoutEngine.compute(&rows).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 40.0, 20.0, 25.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [20.0, 20.0, 0.0, 25.0]
    );
}

#[test]
fn negative_named_line_pair_tracks_expand_startward_and_size_auto_contributions() {
    let mut columns = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        160.0,
        160.0,
        17.0,
        10.0,
    );
    columns.nodes[0].style.align_content = Some(crate::LayoutAlignContent::FlexStart);
    columns.nodes[0].style.justify_content = crate::LayoutJustifyContent::FlexStart;
    let container = columns.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode =
        GridPlacementMode::TemplateGridNegativeNamedLinePairAutoImplicitTracks;
    container.auto_columns = GridAutoColumns::TrackList(vec![
        GridTrackSize::LengthPx(5.0),
        GridTrackSize::LengthPx(7.0),
    ]);
    for (start, end, expected_tracks) in [
        (-8, -5, vec![5.0, 7.0, 5.0, 7.0, 20.0, 30.0, 40.0]),
        (-5, -8, vec![5.0, 7.0, 5.0, 7.0, 20.0, 30.0, 40.0]),
        (-6, -5, vec![5.0, 7.0, 20.0, 30.0, 40.0]),
        (-6, -6, vec![5.0, 7.0, 20.0, 30.0, 40.0]),
        (-9, -6, vec![7.0, 5.0, 7.0, 5.0, 7.0, 20.0, 30.0, 40.0]),
    ] {
        columns.grid_items.insert(
            id(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(start),
                column_end: GridPlacementInput::Line(end),
            },
        );
        let output = TaffyLayoutEngine.compute(&columns).unwrap();
        let frame = output.frames[&id(2)];
        assert_eq!(
            [frame.x, frame.y, frame.width, frame.height],
            [0.0, 0.0, 17.0, 10.0],
            "column lines {start}/{end}"
        );
        assert_eq!(
            final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
            expected_tracks,
            "column lines {start}/{end}"
        );
    }

    columns
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_columns = GridAutoColumns::Auto;
    columns.nodes[1].style.width = LayoutDimension::Fixed(18.0);
    columns.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(-8),
            column_end: GridPlacementInput::Line(-5),
        },
    );
    let output = TaffyLayoutEngine.compute(&columns).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 18.0, 10.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
        [6.0, 6.0, 6.0, 0.0, 20.0, 30.0, 40.0]
    );

    let mut rows = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        160.0,
        160.0,
        10.0,
        17.0,
    );
    rows.nodes[0].style.align_content = Some(crate::LayoutAlignContent::FlexStart);
    rows.nodes[0].style.justify_content = crate::LayoutJustifyContent::FlexStart;
    let container = rows.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode =
        GridPlacementMode::TemplateGridNegativeNamedLinePairAutoImplicitTracks;
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(5.0),
        GridTrackSize::LengthPx(7.0),
    ]);
    rows.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(-8),
            row_end: GridPlacementInput::Line(-5),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let output = TaffyLayoutEngine.compute(&rows).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 10.0, 17.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [5.0, 7.0, 5.0, 7.0, 20.0, 30.0, 40.0]
    );

    rows.grid_containers.get_mut(&id(1)).unwrap().auto_rows = GridAutoRows::Auto;
    rows.nodes[1].style.height = LayoutDimension::Fixed(18.0);
    let output = TaffyLayoutEngine.compute(&rows).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 10.0, 18.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [6.0, 6.0, 6.0, 0.0, 20.0, 30.0, 40.0]
    );
}

#[test]
fn negative_named_line_pair_mode_rejects_invalid_shapes_and_track_kinds() {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(40.0),
        ],
        vec![GridTrackSize::LengthPx(20.0)],
        160.0,
        20.0,
        17.0,
        10.0,
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridNegativeNamedLinePairAutoImplicitTracks;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(-8),
            column_end: GridPlacementInput::Line(-5),
        },
    );
    assert!(TaffyLayoutEngine.compute(&input).is_ok());

    input.grid_items.get_mut(&id(2)).unwrap().column_start = GridPlacementInput::Line(-8);
    input.grid_items.get_mut(&id(2)).unwrap().column_end = GridPlacementInput::Line(-1);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement { node, .. }) if node == id(1)
    ));

    input.grid_items.get_mut(&id(2)).unwrap().column_start = GridPlacementInput::Line(-10_002);
    input.grid_items.get_mut(&id(2)).unwrap().column_end = GridPlacementInput::Line(-10_001);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-columns",
            count: 10_001,
            maximum: 10_000,
        }) if node == id(1)
    ));

    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(-4),
            row_end: GridPlacementInput::Line(-3),
            column_start: GridPlacementInput::Line(-8),
            column_end: GridPlacementInput::Line(-5),
        },
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement { node, .. }) if node == id(1)
    ));

    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(-8),
            column_end: GridPlacementInput::Line(-5),
        },
    );
    input.nodes[0].children.push(id(3));
    input.nodes.push(LayoutNode {
        id: id(3),
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(10.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    });
    input.grid_items.insert(
        id(3),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement { node, .. }) if node == id(1)
    ));

    input.nodes[0].children.pop();
    input.nodes.pop();
    input.grid_items.remove(&id(3));
    input.grid_containers.get_mut(&id(1)).unwrap().auto_columns =
        GridAutoColumns::TrackList(vec![GridTrackSize::Auto]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoColumns { node, .. }) if node == id(1)
    ));

    input.grid_containers.get_mut(&id(1)).unwrap().auto_columns =
        GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(-1.0)]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        }) if node == id(1)
    ));

    input.grid_containers.get_mut(&id(1)).unwrap().auto_columns =
        GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(f32::INFINITY)]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        }) if node == id(1)
    ));
}

#[test]
fn positive_named_line_pair_auto_mode_rejects_invalid_pair_shapes() {
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
        .placement_mode = GridPlacementMode::TemplateGridPositiveNamedLinePairAutoImplicitTracks;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(4),
            column_end: GridPlacementInput::Auto,
        },
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement {
            node,
            property: "grid-column-start",
            ..
        }) if node == id(1)
    ));
}

#[test]
fn positive_named_line_pair_auto_mode_obeys_implicit_track_cap() {
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
        .placement_mode = GridPlacementMode::TemplateGridPositiveNamedLinePairAutoImplicitTracks;
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .track_sizing_trace_enabled = false;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(4),
            column_end: GridPlacementInput::Line(10_001),
        },
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&id(2)].x, 80.0);

    input.grid_items.get_mut(&id(2)).unwrap().column_end = GridPlacementInput::Line(10_002);
    let over_cap = TaffyLayoutEngine.compute(&input);
    assert!(over_cap.is_err(), "over-cap pair returned: {over_cap:?}");
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
