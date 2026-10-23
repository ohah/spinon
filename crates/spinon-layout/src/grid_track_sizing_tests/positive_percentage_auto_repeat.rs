use super::{grid_input, id};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutBorder, LayoutBoxSizing, LayoutDimension,
    LayoutEngine, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind, LayoutInput,
    LayoutLengthPercentage, RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

fn input_with_track(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    box_sizing: LayoutBoxSizing,
    minimum: GridTrackMinSizingFunction,
) -> LayoutInput {
    let track = GridTrackSize::MinMax {
        min: minimum,
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (vec![track], vec![GridTrackSize::LengthPx(120.0)]),
        LayoutGridTrackSizingAxis::Rows => (vec![GridTrackSize::LengthPx(120.0)], vec![track]),
    };
    let mut input = grid_input(columns, rows, 1, Default::default());
    let (specified_size, viewport_size) = match box_sizing {
        LayoutBoxSizing::ContentBox => (120.0, 120.0),
        LayoutBoxSizing::BorderBox => (124.0, 124.0),
    };
    input.nodes[0].style.box_sizing = box_sizing;
    input.nodes[0].style.width = LayoutDimension::Fixed(specified_size);
    input.nodes[0].style.height = LayoutDimension::Fixed(specified_size);
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    input.viewport = Viewport {
        width: viewport_size,
        height: viewport_size,
    };
    input.root_sizing = RootSizingPolicy::Match;
    match axis {
        LayoutGridTrackSizingAxis::Columns => {
            input.nodes[1].style.width = LayoutDimension::Auto;
            input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
        }
        LayoutGridTrackSizingAxis::Rows => {
            input.nodes[1].style.width = LayoutDimension::Fixed(20.0);
            input.nodes[1].style.height = LayoutDimension::Auto;
        }
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
    input
}

fn initial_tracks(input: &LayoutInput, axis: LayoutGridTrackSizingAxis) -> Vec<(bool, f32)> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
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
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

#[test]
fn positive_percentage_auto_repeat_matches_fixed_controls_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            for box_sizing in [LayoutBoxSizing::ContentBox, LayoutBoxSizing::BorderBox] {
                let percentage_input = input_with_track(
                    axis,
                    mode,
                    box_sizing,
                    GridTrackMinSizingFunction::Percentage(0.1),
                );
                let fixed_input = input_with_track(
                    axis,
                    mode,
                    box_sizing,
                    GridTrackMinSizingFunction::LengthPx(12.0),
                );
                let percentage_output = TaffyLayoutEngine.compute(&percentage_input).unwrap();
                let fixed_output = TaffyLayoutEngine.compute(&fixed_input).unwrap();
                let percentage_frame = &percentage_output.frames[&id(2)];
                let fixed_frame = &fixed_output.frames[&id(2)];
                let tracks = initial_tracks(&percentage_input, axis);
                let collapsed_count = tracks.iter().filter(|(collapsed, _)| *collapsed).count();

                assert_eq!(tracks.len(), 10, "{axis:?}/{mode:?}/{box_sizing:?}");
                assert_eq!(
                    collapsed_count,
                    if mode == GridAutoRepeatMode::AutoFill {
                        0
                    } else {
                        9
                    },
                    "{axis:?}/{mode:?}/{box_sizing:?} collapsed tracks"
                );
                assert_eq!(percentage_frame, fixed_frame);
                match axis {
                    LayoutGridTrackSizingAxis::Columns => {
                        assert_eq!(percentage_frame.x, 2.0);
                        assert_eq!(
                            percentage_frame.width,
                            if mode == GridAutoRepeatMode::AutoFill {
                                12.0
                            } else {
                                120.0
                            }
                        );
                    }
                    LayoutGridTrackSizingAxis::Rows => {
                        assert_eq!(percentage_frame.y, 2.0);
                        assert_eq!(
                            percentage_frame.height,
                            if mode == GridAutoRepeatMode::AutoFill {
                                12.0
                            } else {
                                120.0
                            }
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn percentage_minimum_with_fixed_maximum_keeps_auto_repeat_count() {
    let mut input = input_with_track(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        LayoutBoxSizing::ContentBox,
        GridTrackMinSizingFunction::Percentage(0.1),
    );
    input.grid_containers.get_mut(&id(1)).unwrap().columns[0] = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Percentage(0.1),
        max: GridTrackMaxSizingFunction::LengthPx(12.0),
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let tracks = initial_tracks(&input, LayoutGridTrackSizingAxis::Columns);

    assert_eq!(tracks.len(), 10);
    assert!(
        tracks
            .iter()
            .all(|(collapsed, size)| !collapsed && *size == 12.0)
    );
    assert_eq!(output.frames[&id(2)].x, 2.0);
    assert_eq!(output.frames[&id(2)].width, 12.0);
}

#[test]
fn percentage_minimum_overflow_matches_fixed_control_past_outer_border() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let percentage_input = input_with_track(
            axis,
            GridAutoRepeatMode::AutoFill,
            LayoutBoxSizing::ContentBox,
            GridTrackMinSizingFunction::Percentage(1.5),
        );
        let mut percentage_input = percentage_input;
        let fixed_input = input_with_track(
            axis,
            GridAutoRepeatMode::AutoFill,
            LayoutBoxSizing::ContentBox,
            GridTrackMinSizingFunction::LengthPx(180.0),
        );
        let mut fixed_input = fixed_input;
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                percentage_input.nodes[1].style.width = LayoutDimension::Fixed(180.0);
                fixed_input.nodes[1].style.width = LayoutDimension::Fixed(180.0);
            }
            LayoutGridTrackSizingAxis::Rows => {
                percentage_input.nodes[1].style.height = LayoutDimension::Fixed(180.0);
                fixed_input.nodes[1].style.height = LayoutDimension::Fixed(180.0);
            }
        }
        let percentage_output = TaffyLayoutEngine.compute(&percentage_input).unwrap();
        let fixed_output = TaffyLayoutEngine.compute(&fixed_input).unwrap();
        let percentage_frame = &percentage_output.frames[&id(2)];
        let fixed_frame = &fixed_output.frames[&id(2)];
        let tracks = initial_tracks(&percentage_input, axis);

        assert_eq!(tracks.len(), 1, "{axis:?}");
        assert_eq!(tracks[0], (false, 180.0), "{axis:?} minimum track");
        assert_eq!(percentage_frame, fixed_frame);
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                assert_eq!(percentage_frame.x, 2.0);
                assert_eq!(percentage_frame.width, 180.0);
                assert_eq!(percentage_frame.x + percentage_frame.width - 124.0, 58.0);
            }
            LayoutGridTrackSizingAxis::Rows => {
                assert_eq!(percentage_frame.y, 2.0);
                assert_eq!(percentage_frame.height, 180.0);
                assert_eq!(percentage_frame.y + percentage_frame.height - 124.0, 58.0);
            }
        }
    }
}

#[test]
fn percentage_auto_repeat_rejects_zero_indefinite_and_clamped_axes_before_taffy() {
    for (label, mutate) in [
        (
            "zero percentage",
            Box::new(|input: &mut LayoutInput| {
                let track = &mut input.grid_containers.get_mut(&id(1)).unwrap().columns[0];
                *track = GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::Percentage(0.0),
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                };
            }) as Box<dyn Fn(&mut LayoutInput)>,
        ),
        (
            "indefinite width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::Auto;
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "zero content width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::Fixed(0.0);
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "cyclic percentage width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::CyclicMinContentPercent(0.5);
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "intrinsic width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::MaxContent;
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "automatic minimum",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.min_width = LayoutDimension::Auto;
            }),
        ),
        (
            "constrained maximum",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.max_width = LayoutDimension::Fixed(100.0);
            }),
        ),
        (
            "non-positive border-box content",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.box_sizing = LayoutBoxSizing::BorderBox;
                input.nodes[0].style.width = LayoutDimension::Fixed(124.0);
                input.nodes[0].style.padding.left = LayoutLengthPercentage::LengthPx(60.0);
                input.nodes[0].style.padding.right = LayoutLengthPercentage::LengthPx(60.0);
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "non-finite resolved minimum",
            Box::new(|input: &mut LayoutInput| {
                input.grid_containers.get_mut(&id(1)).unwrap().columns[0] = GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::Percentage(f32::MAX),
                    max: GridTrackMaxSizingFunction::Fraction(1.0),
                };
            }),
        ),
    ] {
        let mut input = input_with_track(
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            LayoutBoxSizing::BorderBox,
            GridTrackMinSizingFunction::Percentage(0.1),
        );
        mutate(&mut input);

        assert!(
            matches!(
                TaffyLayoutEngine.compute(&input),
                Err(crate::LayoutError::InvalidGridTrackList {
                    node,
                    property: "grid-template-columns",
                }) if node == id(1)
            ),
            "{label} must fail closed before Taffy"
        );
    }
}
