use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap, LayoutInputRevision,
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
fn track_sizing_profile_honors_explicit_item_min_size() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(100.0)],
        vec![GridTrackSize::LengthPx(80.0)],
        1,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    input.nodes[1].style.width = LayoutDimension::Fixed(20.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
    input.nodes[1].style.min_width = LayoutDimension::Fixed(90.0);

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frame = output.frames[&id(2)];

    assert_eq!(frame.width, 90.0);
}

#[test]
fn track_sizing_profile_accepts_zero_fraction_tracks() {
    let mut input = grid_input(
        vec![GridTrackSize::Fraction(0.0), GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::LengthPx(80.0)],
        2,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    input.nodes[1].style.width = LayoutDimension::Fixed(0.0);
    input.nodes[2].style.width = LayoutDimension::Fixed(20.0);

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames[&id(3)].x, 0.0);
    let trace = &output.grid_track_sizing_traces[0];
    assert_eq!(trace.node, id(1));
    assert!(trace.passes.iter().any(|pass| {
        pass.axis == LayoutGridTrackSizingAxis::Columns
            && pass.stages.iter().any(|stage| {
                stage.stage == LayoutGridTrackSizingStageKind::Initialize
                    && !stage.tracks.is_empty()
            })
            && pass
                .stages
                .iter()
                .any(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
    }));
    assert!(trace.passes.iter().any(|pass| {
        pass.axis == LayoutGridTrackSizingAxis::Rows
            && pass
                .stages
                .iter()
                .any(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
    }));
}

#[test]
fn auto_repeat_rejects_named_lines_through_the_layout_api() {
    let mut columns = grid_input(
        vec![GridTrackSize::LengthPx(80.0)],
        vec![GridTrackSize::LengthPx(80.0)],
        1,
        LayoutGap::default(),
    );
    let container = columns.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: container.columns.len(),
    });
    container.columns_line_names = vec![vec!["start".into()], vec!["end".into()]];
    assert!(matches!(
        TaffyLayoutEngine.compute(&columns),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        }) if node == id(1)
    ));

    let mut rows = grid_input(
        vec![GridTrackSize::LengthPx(80.0)],
        vec![GridTrackSize::LengthPx(80.0)],
        1,
        LayoutGap::default(),
    );
    let container = rows.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 0,
        end: container.rows.len(),
    });
    container.rows_line_names = vec![vec!["top".into()], vec!["bottom".into()]];
    assert!(matches!(
        TaffyLayoutEngine.compute(&rows),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-rows",
        }) if node == id(1)
    ));
}

#[test]
fn auto_repeat_accepts_independent_repeats_on_both_axes() {
    let repeated = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(10.0),
        ],
        vec![
            GridTrackSize::LengthPx(12.0),
            repeated,
            GridTrackSize::LengthPx(8.0),
        ],
        0,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 1,
        end: 2,
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 2,
    });

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
    assert!(trace.passes.iter().any(|pass| {
        pass.axis == LayoutGridTrackSizingAxis::Columns
            && pass
                .stages
                .iter()
                .any(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
    }));
    assert!(trace.passes.iter().any(|pass| {
        pass.axis == LayoutGridTrackSizingAxis::Rows
            && pass
                .stages
                .iter()
                .any(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
    }));
}

#[test]
fn auto_fit_collapsed_gutters_preserve_explicit_suffix_line_geometry() {
    let repeated = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(10.0),
        ],
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(10.0),
        ],
        2,
        LayoutGap {
            column: LayoutLengthPercentage::length(8.0),
            row: LayoutLengthPercentage::length(8.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(200.0);
    input.viewport = Viewport {
        width: 200.0,
        height: 200.0,
    };
    input.nodes[1].style.width = LayoutDimension::Fixed(12.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(12.0);
    input.nodes[2].style.width = LayoutDimension::Fixed(12.0);
    input.nodes[2].style.height = LayoutDimension::Fixed(12.0);

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(3),
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );
    input.grid_items.insert(
        id(3),
        GridItemInput {
            row_start: GridPlacementInput::Line(-2),
            row_end: GridPlacementInput::Line(-1),
            column_start: GridPlacementInput::Line(-2),
            column_end: GridPlacementInput::Line(-1),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        (output.frames[&id(2)].x, output.frames[&id(2)].y),
        (28.0, 28.0)
    );
    assert_eq!(
        (output.frames[&id(3)].x, output.frames[&id(3)].y),
        (190.0, 190.0)
    );
}

#[test]
fn auto_repeat_rejects_an_invalid_second_axis_independently() {
    let repeated = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(10.0),
        ],
        vec![
            GridTrackSize::LengthPx(12.0),
            repeated,
            GridTrackSize::LengthPx(8.0),
        ],
        0,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 1,
        end: 2,
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 2,
        end: 4,
    });

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-rows",
        }) if node == id(1)
    ));
}

#[test]
fn auto_repeat_rejects_an_invalid_first_axis_independently() {
    let repeated = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(10.0),
        ],
        vec![
            GridTrackSize::LengthPx(12.0),
            repeated,
            GridTrackSize::LengthPx(8.0),
        ],
        0,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 2,
        end: 4,
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 2,
    });

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        }) if node == id(1)
    ));
}
