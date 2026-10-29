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

fn named_auto_repeat_input(item: GridItemInput) -> LayoutInput {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(40.0),
            GridTrackSize::LengthPx(10.0),
        ],
        vec![GridTrackSize::LengthPx(80.0)],
        1,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 1,
        end: 2,
        line_names: vec![vec!["item-start".into()], vec!["item-end".into()]],
    });
    container.columns_line_names = vec![
        vec!["container-start".into()],
        vec!["repeat-start".into()],
        vec!["repeat-end".into()],
        vec!["container-end".into()],
    ];
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.grid_items.insert(id(2), item);
    input
}

#[test]
fn auto_repeat_named_lines_resolve_after_repeat_expansion() {
    let cases = [
        (
            GridPlacementInput::NamedLine {
                name: "item-start".into(),
                occurrence: 2,
            },
            GridPlacementInput::NamedLine {
                name: "item-end".into(),
                occurrence: 2,
            },
            60.0,
            40.0,
        ),
        (
            GridPlacementInput::NamedLine {
                name: "item-start".into(),
                occurrence: -1,
            },
            GridPlacementInput::NamedLine {
                name: "item-end".into(),
                occurrence: -1,
            },
            260.0,
            40.0,
        ),
        (
            GridPlacementInput::NamedLine {
                name: "item-start".into(),
                occurrence: 2,
            },
            GridPlacementInput::SpanNamed {
                name: "item-end".into(),
                count: 2,
            },
            60.0,
            80.0,
        ),
        (
            GridPlacementInput::NamedLine {
                name: "repeat-start".into(),
                occurrence: 0,
            },
            GridPlacementInput::NamedLine {
                name: "item-end".into(),
                occurrence: 1,
            },
            20.0,
            40.0,
        ),
    ];

    for (case_index, (column_start, column_end, expected_x, expected_width)) in
        cases.into_iter().enumerate()
    {
        let input = named_auto_repeat_input(GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start,
            column_end,
        });
        let output = TaffyLayoutEngine.compute(&input).unwrap();
        let frame = output.frames[&id(2)];
        assert_eq!(frame.x, expected_x, "case {case_index}");
        assert_eq!(frame.width, expected_width, "case {case_index}");
    }
}

#[test]
fn auto_repeat_template_area_extends_the_explicit_grid_and_resolves_area_lines() {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(20.0),
        ],
        vec![GridTrackSize::LengthPx(20.0)],
        1,
        LayoutGap {
            row: LayoutLengthPercentage::length(0.0),
            column: LayoutLengthPercentage::length(4.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(20.0);
    input.viewport.width = 100.0;
    input.viewport.height = 20.0;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 1,
        end: 2,
        line_names: vec![Vec::new(), Vec::new()],
    });
    container.auto_columns = crate::GridAutoColumns::LengthPx(20.0);
    container.template_areas = Some(crate::GridTemplateAreasInput {
        rows: 1,
        columns: 5,
        areas: vec![crate::GridTemplateAreaInput {
            name: "suffix".into(),
            row_start: 0,
            row_end: 1,
            column_start: 4,
            column_end: 5,
        }],
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::NamedLine {
                name: "suffix-start".into(),
                occurrence: 1,
            },
            column_end: GridPlacementInput::NamedLine {
                name: "suffix-end".into(),
                occurrence: 1,
            },
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (96.0, 0.0, 20.0, 10.0)
    );
}

#[test]
fn two_axis_auto_repeat_template_area_extends_both_axes_and_resolves_area_lines() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(20.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        1,
        LayoutGap {
            row: LayoutLengthPercentage::length(4.0),
            column: LayoutLengthPercentage::length(4.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(40.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(40.0);
    input.nodes[1].style.width = LayoutDimension::Fixed(8.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(8.0);
    input.viewport.width = 40.0;
    input.viewport.height = 40.0;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.auto_columns = crate::GridAutoColumns::LengthPx(16.0);
    container.auto_rows = GridAutoRows::LengthPx(12.0);
    container.template_areas = Some(crate::GridTemplateAreasInput {
        rows: 3,
        columns: 3,
        areas: vec![crate::GridTemplateAreaInput {
            name: "hero".into(),
            row_start: 2,
            row_end: 3,
            column_start: 2,
            column_end: 3,
        }],
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::NamedLine {
                name: "hero-start".into(),
                occurrence: 1,
            },
            row_end: GridPlacementInput::NamedLine {
                name: "hero-end".into(),
                occurrence: 1,
            },
            column_start: GridPlacementInput::NamedLine {
                name: "hero-start".into(),
                occurrence: 1,
            },
            column_end: GridPlacementInput::NamedLine {
                name: "hero-end".into(),
                occurrence: 1,
            },
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frame = output.frames[&id(2)];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (44.0, 40.0, 8.0, 8.0)
    );
}

fn implicit_fallback_auto_repeat_input(
    columns_axis: bool,
    repeat_start: GridPlacementInput,
    repeat_end: GridPlacementInput,
) -> LayoutInput {
    let mut input = grid_input(
        if columns_axis {
            vec![GridTrackSize::LengthPx(40.0)]
        } else {
            vec![GridTrackSize::LengthPx(20.0)]
        },
        if columns_axis {
            vec![GridTrackSize::LengthPx(20.0)]
        } else {
            vec![GridTrackSize::LengthPx(40.0)]
        },
        1,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(if columns_axis { 125.0 } else { 20.0 });
    input.nodes[0].style.height = LayoutDimension::Fixed(if columns_axis { 20.0 } else { 125.0 });
    input.viewport.width = if columns_axis { 125.0 } else { 20.0 };
    input.viewport.height = if columns_axis { 20.0 } else { 125.0 };
    input.nodes[1].style.width = if columns_axis {
        LayoutDimension::Auto
    } else {
        LayoutDimension::Fixed(20.0)
    };
    input.nodes[1].style.height = LayoutDimension::Fixed(10.0);

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::LengthPx(20.0);
    container.auto_columns = crate::GridAutoColumns::LengthPx(20.0);
    if columns_axis {
        container.columns_auto_repeat = Some(GridAutoRepeat {
            mode: GridAutoRepeatMode::AutoFill,
            start: 0,
            end: 1,
            line_names: vec![vec!["cell".into()], vec!["cell".into()]],
        });
        container.columns_line_names = vec![Vec::new(), Vec::new()];
        input.grid_items.insert(
            id(2),
            GridItemInput {
                column_start: repeat_start,
                column_end: repeat_end,
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
            },
        );
    } else {
        container.rows_auto_repeat = Some(GridAutoRepeat {
            mode: GridAutoRepeatMode::AutoFill,
            start: 0,
            end: 1,
            line_names: vec![vec!["cell".into()], vec!["cell".into()]],
        });
        container.rows_line_names = vec![Vec::new(), Vec::new()];
        input.grid_items.insert(
            id(2),
            GridItemInput {
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
                row_start: repeat_start,
                row_end: repeat_end,
            },
        );
    }
    input
}

#[test]
fn auto_repeat_named_placement_uses_implicit_lines_when_name_is_missing_or_short() {
    let named = |name: &str, occurrence| GridPlacementInput::NamedLine {
        name: name.to_owned(),
        occurrence,
    };
    let span = |name: &str, count| GridPlacementInput::SpanNamed {
        name: name.to_owned(),
        count,
    };
    let cases = [
        (
            true,
            named("ghost", 5),
            named("ghost", 6),
            (220.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            named("cell", 5),
            named("cell", 6),
            (140.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            named("ghost", -5),
            named("ghost", -4),
            (0.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            named("cell", -5),
            named("cell", -4),
            (0.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            GridPlacementInput::Line(4),
            span("ghost", 1),
            (120.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            span("ghost", 1),
            GridPlacementInput::Line(1),
            (0.0, 0.0, 20.0, 10.0),
        ),
        (
            true,
            GridPlacementInput::Line(3),
            span("cell", 2),
            (80.0, 0.0, 60.0, 10.0),
        ),
        (
            true,
            span("cell", 2),
            GridPlacementInput::Line(2),
            (0.0, 0.0, 60.0, 10.0),
        ),
        (
            false,
            named("ghost", 5),
            named("ghost", 6),
            (0.0, 220.0, 20.0, 10.0),
        ),
        (
            false,
            named("ghost", -5),
            named("ghost", -4),
            (0.0, 0.0, 20.0, 10.0),
        ),
    ];

    for (case_index, (columns_axis, start, end, expected)) in cases.into_iter().enumerate() {
        let input = implicit_fallback_auto_repeat_input(columns_axis, start, end);
        let output = TaffyLayoutEngine.compute(&input).unwrap();
        let frame = output.frames[&id(2)];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "case {case_index}"
        );
    }
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
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 2,
        line_names: Vec::new(),
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
fn auto_fit_definite_minimum_uses_same_repeat_count_on_both_axes() {
    let repeated_minmax = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let tracks = vec![
        GridTrackSize::LengthPx(20.0),
        repeated_minmax,
        GridTrackSize::LengthPx(20.0),
        GridTrackSize::LengthPx(10.0),
    ];
    let mut input = grid_input(
        tracks.clone(),
        tracks,
        0,
        LayoutGap {
            column: LayoutLengthPercentage::length(8.0),
            row: LayoutLengthPercentage::length(8.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::MaxContent;
    input.nodes[0].style.height = LayoutDimension::MaxContent;
    input.nodes[0].style.min_width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(200.0);
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
        line_names: Vec::new(),
    });

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
    let initialized_tracks = |axis| {
        let pass = trace
            .passes
            .iter()
            .rev()
            .find(|pass| pass.axis == axis)
            .unwrap();
        pass.stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
            .unwrap()
            .tracks
            .clone()
    };
    let columns = initialized_tracks(LayoutGridTrackSizingAxis::Columns);
    let rows = initialized_tracks(LayoutGridTrackSizingAxis::Rows);
    let column_axis_tracks = columns
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    let row_axis_tracks = rows
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();

    assert_eq!(output.frames[&id(1)].width, 200.0);
    assert_eq!(output.frames[&id(1)].height, 200.0);
    assert_eq!(column_axis_tracks.len(), 8);
    assert_eq!(row_axis_tracks.len(), 8);
    for (axis, tracks, gutters) in [
        ("columns", &columns, column_axis_tracks),
        ("rows", &rows, row_axis_tracks),
    ] {
        assert!(
            !gutters[0].is_collapsed,
            "{axis} explicit prefix must remain"
        );
        assert!(gutters[1..7].iter().all(|track| track.is_collapsed));
        assert!(
            !gutters[7].is_collapsed,
            "{axis} explicit suffix must remain"
        );
        for internal_index in (2..=14).step_by(2) {
            let gutter = tracks
                .iter()
                .find(|track| track.internal_index == internal_index)
                .unwrap();
            let is_collapsed = internal_index != 2;
            assert_eq!(
                gutter.is_collapsed, is_collapsed,
                "{axis} gutter {internal_index} must represent the single leading boundary of the empty auto-fit run"
            );
            assert_eq!(gutter.base_size, if is_collapsed { 0.0 } else { 8.0 });
        }
    }
}

#[test]
fn auto_repeat_minimum_size_repeat_counts_match_axis_and_fill_mode_at_boundaries() {
    let repeated_minmax = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let tracks = vec![
        GridTrackSize::LengthPx(20.0),
        repeated_minmax,
        GridTrackSize::LengthPx(20.0),
        GridTrackSize::LengthPx(10.0),
    ];

    for (min_size, expected_repetitions) in [(190.0, 2), (200.0, 3), (342.0, 4)] {
        let mut observed_counts = Vec::new();
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            let mut input = grid_input(
                tracks.clone(),
                tracks.clone(),
                0,
                LayoutGap {
                    column: LayoutLengthPercentage::length(8.0),
                    row: LayoutLengthPercentage::length(8.0),
                },
            );
            input.nodes[0].style.width = LayoutDimension::MaxContent;
            input.nodes[0].style.height = LayoutDimension::MaxContent;
            input.nodes[0].style.min_width = LayoutDimension::Fixed(min_size);
            input.nodes[0].style.min_height = LayoutDimension::Fixed(min_size);
            input.root_sizing = crate::RootSizingPolicy::ResolveWithin;

            let container = input.grid_containers.get_mut(&id(1)).unwrap();
            container.placement_mode = GridPlacementMode::TrackSizing;
            container.columns_auto_repeat = Some(GridAutoRepeat {
                mode,
                start: 1,
                end: 3,
                line_names: Vec::new(),
            });
            container.rows_auto_repeat = Some(GridAutoRepeat {
                mode,
                start: 1,
                end: 3,
                line_names: Vec::new(),
            });

            let output = TaffyLayoutEngine.compute(&input).unwrap();
            assert!(
                output.frames[&id(1)].width >= min_size,
                "column minimum-size {min_size} must be met; got {} for {mode:?}",
                output.frames[&id(1)].width
            );
            assert!(
                output.frames[&id(1)].height >= min_size,
                "row minimum-size {min_size} must be met; got {} for {mode:?}",
                output.frames[&id(1)].height
            );
            let trace = output
                .grid_track_sizing_traces
                .iter()
                .find(|trace| trace.node == id(1))
                .unwrap();
            for axis in [
                LayoutGridTrackSizingAxis::Columns,
                LayoutGridTrackSizingAxis::Rows,
            ] {
                let pass = trace
                    .passes
                    .iter()
                    .rev()
                    .find(|pass| pass.axis == axis)
                    .unwrap();
                let initialized = pass
                    .stages
                    .iter()
                    .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
                    .unwrap();
                let count = initialized
                    .tracks
                    .iter()
                    .filter(|track| !track.is_gutter)
                    .count();
                assert_eq!(
                    count,
                    2 + expected_repetitions * 2,
                    "{axis:?} {mode:?} at min-size {min_size} must keep the spec-derived repeated range"
                );
                observed_counts.push((axis, mode, count));
            }
        }
        assert_eq!(observed_counts[0].2, observed_counts[2].2);
        assert_eq!(observed_counts[1].2, observed_counts[3].2);
        assert_eq!(observed_counts[0].2, observed_counts[1].2);
        assert_eq!(observed_counts[2].2, observed_counts[3].2);
    }
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
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
        line_names: Vec::new(),
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
fn auto_fit_end_repeat_collapses_leading_gutter_after_prefix_when_all_tracks_collapse() {
    let repeated = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            repeated,
            GridTrackSize::LengthPx(20.0),
        ],
        vec![GridTrackSize::LengthPx(40.0)],
        0,
        LayoutGap {
            column: LayoutLengthPercentage::length(8.0),
            row: LayoutLengthPercentage::length(8.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::MaxContent;
    input.nodes[0].style.min_width = LayoutDimension::Fixed(200.0);
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 3,
        line_names: Vec::new(),
    });

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
    let columns = trace
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap()
        .stages
        .iter()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
        .unwrap()
        .tracks
        .clone();
    let leading_gutter = columns
        .iter()
        .find(|track| track.internal_index == 2)
        .unwrap();

    assert!(leading_gutter.is_collapsed);
    assert_eq!(leading_gutter.base_size, 0.0);
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
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 2,
        end: 4,
        line_names: Vec::new(),
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
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 1,
        end: 2,
        line_names: Vec::new(),
    });

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        }) if node == id(1)
    ));
}

#[path = "grid_track_sizing_tests/positive_percentage_auto_repeat.rs"]
mod positive_percentage_auto_repeat;

#[path = "grid_track_sizing_tests/auto_repeat_fixed_maximum.rs"]
mod auto_repeat_fixed_maximum;

#[path = "grid_track_sizing_tests/auto_repeat_zero_fixed_maximum.rs"]
mod auto_repeat_zero_fixed_maximum;

#[path = "grid_track_sizing_tests/cyclic_percentage_auto_repeat_rows.rs"]
mod cyclic_percentage_auto_repeat_rows;

#[path = "grid_track_sizing_tests/percentage_auto_repeat_neighbors.rs"]
mod percentage_auto_repeat_neighbors;

#[path = "grid_track_sizing_tests/auto_repeat_calc_neighbors.rs"]
mod auto_repeat_calc_neighbors;

#[path = "grid_track_sizing_tests/zero_percent_auto_repeat_minimum.rs"]
mod zero_percent_auto_repeat_minimum;

#[path = "grid_track_sizing_tests/flex_factor_bound.rs"]
mod flex_factor_bound;

#[path = "grid_track_sizing_tests/auto_repeat_track_ceiling.rs"]
mod auto_repeat_track_ceiling;
