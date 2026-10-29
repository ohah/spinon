use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    FlexDirection, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap, LayoutInputRevision,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine,
    Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn grid_input(
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    item_count: usize,
    gap: LayoutGap,
) -> LayoutInput {
    let root = id(1);
    let children = (0..item_count)
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(320.0),
            height: LayoutDimension::Fixed(200.0),
            gap,
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(children.into_iter().map(|child| LayoutNode {
        id: child,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(20.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    }));
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 200.0,
        },
        root_sizing: crate::RootSizingPolicy::Match,
        nodes,
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
                auto_columns: crate::GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::Existing,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

#[test]
fn definite_length_fraction_and_gap_tracks_auto_place_in_source_order() {
    let input = grid_input(
        vec![GridTrackSize::LengthPx(100.0), GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(80.0), GridTrackSize::Fraction(1.0)],
        4,
        LayoutGap {
            column: LayoutLengthPercentage::length(5.0),
            row: LayoutLengthPercentage::length(10.0),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&id(2)].x, 0.0);
    assert_eq!(output.frames[&id(2)].y, 0.0);
    assert_eq!(output.frames[&id(3)].x, 105.0);
    assert_eq!(output.frames[&id(3)].y, 0.0);
    assert_eq!(output.frames[&id(4)].x, 0.0);
    assert_eq!(output.frames[&id(4)].y, 90.0);
    assert_eq!(output.frames[&id(5)].x, 105.0);
    assert_eq!(output.frames[&id(5)].y, 90.0);
    assert!(output.grid_track_sizing_traces.is_empty());
}

#[test]
fn empty_grid_uses_explicit_tracks_without_creating_items() {
    let input = grid_input(
        vec![
            GridTrackSize::LengthPx(80.0),
            GridTrackSize::LengthPx(100.0),
        ],
        vec![GridTrackSize::LengthPx(90.0)],
        0,
        LayoutGap::default(),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames.len(), 1);
    let frame = &output.frames[&id(1)];
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 320.0, 200.0]
    );
}

#[test]
fn template_area_mode_stretches_definite_auto_sized_items_without_widening_dense_mode() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(80.0)],
        vec![GridTrackSize::LengthPx(40.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(120.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
    input.viewport = Viewport {
        width: 120.0,
        height: 40.0,
    };
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.nodes[1].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[1].style.min_height = LayoutDimension::Fixed(0.0);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGrid;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (40.0, 0.0, 80.0, 40.0)
    );

    let mut dense = input.clone();
    dense
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::DenseAutoPlacement;
    assert!(matches!(
        TaffyLayoutEngine.compute(&dense),
        Err(LayoutError::UnsupportedGridItemSize(node)) if node == id(2)
    ));

    let mut dense_zero_minimum = input.clone();
    dense_zero_minimum.nodes[1].style.width = LayoutDimension::Fixed(40.0);
    dense_zero_minimum.nodes[1].style.height = LayoutDimension::Fixed(40.0);
    dense_zero_minimum
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::DenseAutoPlacement;
    assert!(matches!(
        TaffyLayoutEngine.compute(&dense_zero_minimum),
        Err(LayoutError::UnsupportedGridItemConstraint {
            node,
            property: "min-width",
        }) if node == id(2)
    ));

    for property in ["min-width", "min-height", "max-width", "max-height"] {
        let mut constrained_area_item = input.clone();
        let style = &mut constrained_area_item.nodes[1].style;
        match property {
            "min-width" => style.min_width = LayoutDimension::Fixed(1.0),
            "min-height" => style.min_height = LayoutDimension::Fixed(1.0),
            "max-width" => style.max_width = LayoutDimension::Fixed(1.0),
            "max-height" => style.max_height = LayoutDimension::Fixed(1.0),
            _ => unreachable!(),
        }
        assert!(matches!(
            TaffyLayoutEngine.compute(&constrained_area_item),
            Err(LayoutError::UnsupportedGridItemConstraint {
                node,
                property: actual,
            }) if node == id(2) && actual == property
        ));
    }

    let mut auto_placed = input;
    auto_placed.grid_items.clear();
    assert!(matches!(
        TaffyLayoutEngine.compute(&auto_placed),
        Err(LayoutError::UnsupportedGridItemSize(node)) if node == id(2)
    ));
}

#[test]
fn template_area_rows_continue_fixed_px_pattern_after_area_expansion() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(30.0)],
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(6.0)],
        3,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(30.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(80.0);
    input.viewport = Viewport {
        width: 30.0,
        height: 80.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    container.template_areas = Some(GridTemplateAreasInput {
        rows: 2,
        columns: 1,
        areas: vec![
            GridTemplateAreaInput {
                name: "known".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            },
            GridTemplateAreaInput {
                name: "area-row".to_owned(),
                row_start: 1,
                row_end: 2,
                column_start: 0,
                column_end: 1,
            },
        ],
    });
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(8.0),
        GridTrackSize::LengthPx(6.0),
    ]);
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![
        GridTrackSize::LengthPx(9.0),
        GridTrackSize::LengthPx(10.0),
    ]);
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    input.grid_items.insert(
        id(3),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(3),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    input.grid_items.insert(
        id(4),
        GridItemInput {
            row_start: GridPlacementInput::Line(4),
            row_end: GridPlacementInput::Span(1),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&id(2)].y, 0.0);
    assert_eq!(output.frames[&id(3)].y, 20.0);
    assert_eq!(output.frames[&id(4)].y, 34.0);
    assert_eq!(output.frames[&id(4)].height, 10.0);
}

#[test]
fn template_area_columns_continue_fixed_px_pattern_after_area_expansion() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(5.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        3,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(80.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(30.0);
    input.viewport = Viewport {
        width: 80.0,
        height: 30.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    container.template_areas = Some(GridTemplateAreasInput {
        rows: 1,
        columns: 2,
        areas: vec![
            GridTemplateAreaInput {
                name: "known".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            },
            GridTemplateAreaInput {
                name: "area-col".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 1,
                column_end: 2,
            },
        ],
    });
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(11.0),
        GridTrackSize::LengthPx(13.0),
    ]);
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![
        GridTrackSize::LengthPx(7.0),
        GridTrackSize::LengthPx(5.0),
    ]);
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    input.grid_items.insert(
        id(3),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );
    input.grid_items.insert(
        id(4),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(4),
            column_end: GridPlacementInput::Span(1),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&id(2)].x, 0.0);
    assert_eq!(output.frames[&id(3)].x, 20.0);
    assert_eq!(output.frames[&id(4)].x, 32.0);
    assert_eq!(output.frames[&id(4)].width, 20.0);
}

#[test]
fn template_grid_auto_track_lists_require_area_while_named_mode_keeps_existing_px_support() {
    let mut no_area = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    no_area
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGrid;
    no_area.grid_containers.get_mut(&id(1)).unwrap().auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(10.0),
        GridTrackSize::LengthPx(12.0),
    ]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&no_area),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));

    let mut named_mode = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    let container = named_mode.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    container.template_areas = Some(GridTemplateAreasInput {
        rows: 1,
        columns: 1,
        areas: vec![GridTemplateAreaInput {
            name: "known".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 1,
        }],
    });
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(10.0),
        GridTrackSize::LengthPx(12.0),
    ]);
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![
        GridTrackSize::LengthPx(8.0),
        GridTrackSize::LengthPx(9.0),
    ]);
    named_mode.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    assert!(TaffyLayoutEngine.compute(&named_mode).is_ok());

    named_mode
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::MinContent]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&named_mode),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));
}

#[test]
fn trailing_template_area_named_lines_use_default_auto_tracks() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    input.viewport = Viewport {
        width: 80.0,
        height: 30.0,
    };
    input.nodes[0].style.width = LayoutDimension::Fixed(80.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(30.0);
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.nodes[1].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[1].style.min_height = LayoutDimension::Fixed(0.0);
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    container.template_areas = Some(GridTemplateAreasInput {
        rows: 1,
        columns: 2,
        areas: vec![GridTemplateAreaInput {
            name: "known".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 1,
        }],
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(4),
            column_start: GridPlacementInput::Line(3),
            column_end: GridPlacementInput::Line(5),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        (
            output.frames[&id(1)].x,
            output.frames[&id(1)].y,
            output.frames[&id(1)].width,
            output.frames[&id(1)].height,
        ),
        (0.0, 0.0, 80.0, 30.0)
    );
    assert_eq!(
        (
            output.frames[&id(2)].x,
            output.frames[&id(2)].y,
            output.frames[&id(2)].width,
            output.frames[&id(2)].height,
        ),
        (80.0, 30.0, 0.0, 0.0)
    );
}

#[test]
fn implicit_auto_tracks_still_require_template_areas_and_signed_mode_rejects_them() {
    let mut no_area = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    no_area
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TemplateGridImplicitTracks;
    no_area.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(4),
            column_start: GridPlacementInput::Line(3),
            column_end: GridPlacementInput::Line(5),
        },
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&no_area),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));

    let mut signed = grid_input(
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    let container = signed.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TemplateGridSignedImplicitTracks;
    container.template_areas = Some(GridTemplateAreasInput {
        rows: 1,
        columns: 2,
        areas: vec![GridTemplateAreaInput {
            name: "known".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 1,
        }],
    });
    signed.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(-2),
            row_end: GridPlacementInput::Line(1),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&signed),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));
}

#[test]
fn nested_grid_containers_classify_auto_and_explicit_items_independently() {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(160.0),
            GridTrackSize::LengthPx(160.0),
        ],
        vec![GridTrackSize::LengthPx(200.0)],
        2,
        LayoutGap::default(),
    );
    let outer_item = id(2);
    let nested_grid = id(3);
    let nested_item = id(4);
    input.nodes[2] = LayoutNode {
        id: nested_grid,
        children: vec![nested_item],
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(160.0),
            height: LayoutDimension::Fixed(120.0),
            ..LayoutStyle::default()
        },
    };
    input.nodes.push(LayoutNode {
        id: nested_item,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Auto,
            height: LayoutDimension::Auto,
            ..LayoutStyle::default()
        },
    });
    input.grid_containers.insert(
        nested_grid,
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(160.0)],
            rows: vec![GridTrackSize::LengthPx(120.0)],
            columns_auto_repeat: None,
            rows_auto_repeat: None,
            columns_line_names: Vec::new(),
            rows_line_names: Vec::new(),
            template_areas: None,
            justify_items: Default::default(),
            auto_flow: GridAutoFlow::Row,
            auto_rows: GridAutoRows::Auto,
            auto_columns: crate::GridAutoColumns::Auto,
            placement_mode: GridPlacementMode::Existing,
        },
    );
    input.grid_items.insert(
        nested_item,
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert!(output.frames.contains_key(&outer_item));
    assert!(output.frames.contains_key(&nested_grid));
    assert!(output.frames.contains_key(&nested_item));
}

#[test]
fn hidden_grid_child_does_not_trigger_validation_for_unsupported_placement() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    let hidden_child = id(2);
    input.nodes[1].style.display = LayoutDisplay::None;
    input.grid_items.insert(
        hidden_child,
        GridItemInput {
            row_start: GridPlacementInput::Line(0),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert!(output.frames.contains_key(&id(1)));
    let hidden_frame = &output.frames[&hidden_child];
    assert_eq!((hidden_frame.width, hidden_frame.height), (0.0, 0.0));
}

#[test]
fn hidden_explicit_item_does_not_disable_implicit_track_rejection_for_visible_auto_items() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        3,
        LayoutGap::default(),
    );
    let hidden_child = id(2);
    input.nodes[1].style.display = LayoutDisplay::None;
    input.grid_items.insert(
        hidden_child,
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridImplicitTrackRequired(id(1)))
    );
}

#[test]
fn percentage_tracks_use_the_definite_grid_content_axes() {
    let input = grid_input(
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0),
        ],
        vec![GridTrackSize::Percentage(0.5), GridTrackSize::Fraction(1.0)],
        4,
        LayoutGap::default(),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&id(3)].x, 80.0);
    assert_eq!(output.frames[&id(4)].y, 100.0);
    assert_eq!(output.frames[&id(5)].x, 80.0);
    assert_eq!(output.frames[&id(5)].y, 100.0);
}

#[test]
fn auto_flow_outside_c11_1_fails_closed_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(160.0), GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(100.0), GridTrackSize::Fraction(1.0)],
        1,
        LayoutGap::default(),
    );
    input.grid_containers.get_mut(&id(1)).unwrap().auto_flow = GridAutoFlow::Column;

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoFlow(id(1)))
    );
}

#[test]
fn nonzero_grid_item_order_fails_closed_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[1].style.order = 1;

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridOrder(id(2)))
    );
}

#[test]
fn grid_track_count_above_taffy_limit_fails_instead_of_being_truncated() {
    let input = grid_input(
        vec![GridTrackSize::LengthPx(1.0); GRID_MAX_TRACKS + 1],
        vec![GridTrackSize::LengthPx(200.0)],
        0,
        LayoutGap::default(),
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackLimitExceeded {
            node: id(1),
            property: "grid-template-columns",
            count: GRID_MAX_TRACKS + 1,
            maximum: GRID_MAX_TRACKS,
        })
    );
}

#[test]
fn auto_repeat_line_metadata_above_budget_fails_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::MinMax {
            min: crate::GridTrackMinSizingFunction::LengthPx(1.0),
            max: crate::GridTrackMaxSizingFunction::Fraction(1.0),
        }],
        vec![GridTrackSize::LengthPx(200.0)],
        0,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(crate::GridAutoRepeat {
        mode: crate::GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: vec![vec!["x".repeat(1024 * 1024)], Vec::new()],
    });

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList {
            node: id(1),
            property: "grid-template-columns",
        })
    );
}

#[test]
fn auto_repeat_expanded_line_metadata_above_budget_fails_before_taffy() {
    let mut input = grid_input(
        vec![GridTrackSize::MinMax {
            min: crate::GridTrackMinSizingFunction::LengthPx(1.0),
            max: crate::GridTrackMaxSizingFunction::Fraction(1.0),
        }],
        vec![GridTrackSize::LengthPx(200.0)],
        0,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(crate::GridAutoRepeat {
        mode: crate::GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: vec![vec!["x".repeat(16 * 1024 * 1024 / 10_000 + 1)], Vec::new()],
    });

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList {
            node: id(1),
            property: "grid-template-columns",
        })
    );
}

#[test]
fn grid_layout_preserves_child_flex_dispatch() {
    let flex = id(2);
    let item = id(3);
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(320.0)],
        vec![GridTrackSize::LengthPx(200.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[1] = LayoutNode {
        id: flex,
        children: vec![item],
        style: LayoutStyle {
            display: LayoutDisplay::Flex,
            flex_direction: FlexDirection::Row,
            width: LayoutDimension::Fixed(100.0),
            height: LayoutDimension::Fixed(40.0),
            ..LayoutStyle::default()
        },
    };
    input.nodes.push(LayoutNode {
        id: item,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(20.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    });

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&flex].width, 100.0);
    assert_eq!(output.frames[&item].width, 20.0);
}
