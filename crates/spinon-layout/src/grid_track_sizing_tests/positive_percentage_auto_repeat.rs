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

fn definite_maximum_input(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    minimum: GridTrackMinSizingFunction,
    maximum: GridTrackMaxSizingFunction,
) -> LayoutInput {
    let mut input = input_with_track(
        axis,
        mode,
        LayoutBoxSizing::ContentBox,
        GridTrackMinSizingFunction::LengthPx(16.0),
    );
    input.nodes[0].style.box_sizing = LayoutBoxSizing::ContentBox;
    input.nodes[0].style.width = LayoutDimension::Fixed(184.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(184.0);
    input.nodes[0].style.border = LayoutBorder::default();
    input.nodes[0].style.gap = crate::LayoutGap {
        column: LayoutLengthPercentage::length(8.0),
        row: LayoutLengthPercentage::length(8.0),
    };
    input.viewport = Viewport {
        width: 184.0,
        height: 184.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    let repeated_track = GridTrackSize::MinMax {
        min: minimum,
        max: maximum,
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns[0] = repeated_track,
        LayoutGridTrackSizingAxis::Rows => container.rows[0] = repeated_track,
    }
    input
}

fn calc_maximum_input(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    property: crate::LayoutCssMathProperty,
) -> LayoutInput {
    let mut input = definite_maximum_input(
        axis,
        mode,
        GridTrackMinSizingFunction::LengthPx(16.0),
        GridTrackMaxSizingFunction::Calc(crate::LayoutCalcId(7)),
    );
    input.css_math = vec![crate::LayoutCssMathValue {
        id: crate::LayoutCalcId(7),
        node_id: id(1),
        property,
        expression: crate::LayoutCssMath::Sum(vec![
            crate::LayoutCssMath::LengthPx(20.0),
            crate::LayoutCssMath::Percentage(0.1),
        ]),
    }];
    input
}

fn assert_axis_frame_size(input: &LayoutInput, axis: LayoutGridTrackSizingAxis, expected: f32) {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    let frame = &output.frames[&id(2)];
    let actual = match axis {
        LayoutGridTrackSizingAxis::Columns => frame.width,
        LayoutGridTrackSizingAxis::Rows => frame.height,
    };
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{axis:?}: {actual} != {expected}"
    );
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
fn definite_percentage_maximum_uses_the_content_axis_and_gap_for_auto_repeat() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            let input = definite_maximum_input(
                axis,
                mode,
                GridTrackMinSizingFunction::LengthPx(16.0),
                GridTrackMaxSizingFunction::Percentage(0.25),
            );
            let tracks = initial_tracks(&input, axis);
            let collapsed_count = tracks.iter().filter(|(collapsed, _)| *collapsed).count();

            assert_eq!(tracks.len(), 3, "{axis:?}/{mode:?} repeat count");
            assert_eq!(
                collapsed_count,
                if mode == GridAutoRepeatMode::AutoFill {
                    0
                } else {
                    2
                },
                "{axis:?}/{mode:?} auto-fit collapse"
            );
            assert_axis_frame_size(&input, axis, 46.0);
        }
    }
}

#[test]
fn percentage_maxima_below_the_minimum_and_zero_are_floored_to_the_minimum() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            for maximum in [0.2, 0.0] {
                let percentage = definite_maximum_input(
                    axis,
                    mode,
                    GridTrackMinSizingFunction::Percentage(0.25),
                    GridTrackMaxSizingFunction::Percentage(maximum),
                );
                let fixed = definite_maximum_input(
                    axis,
                    mode,
                    GridTrackMinSizingFunction::LengthPx(46.0),
                    GridTrackMaxSizingFunction::LengthPx(46.0),
                );
                let tracks = initial_tracks(&percentage, axis);
                let fixed_tracks = initial_tracks(&fixed, axis);
                assert_eq!(tracks.len(), 3, "{axis:?}/{mode:?}/{maximum} count");
                assert_eq!(
                    tracks
                        .iter()
                        .map(|(collapsed, _)| collapsed)
                        .collect::<Vec<_>>(),
                    fixed_tracks
                        .iter()
                        .map(|(collapsed, _)| collapsed)
                        .collect::<Vec<_>>(),
                    "{axis:?}/{mode:?}/{maximum} collapse matches fixed control"
                );
                assert_axis_frame_size(&percentage, axis, 46.0);
                assert_axis_frame_size(&fixed, axis, 46.0);
            }
        }
    }
}

#[test]
fn length_percentage_calc_maximum_is_typed_to_its_axis_and_keeps_repeat_count() {
    for (axis, property) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            crate::LayoutCssMathProperty::GridTemplateColumns,
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            crate::LayoutCssMathProperty::GridTemplateRows,
        ),
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            let input = calc_maximum_input(axis, mode, property);
            let tracks = initial_tracks(&input, axis);
            assert_eq!(tracks.len(), 4, "{axis:?}/{mode:?} repeat count");
            assert_axis_frame_size(&input, axis, 38.4);
        }
    }

    let mismatched = calc_maximum_input(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        crate::LayoutCssMathProperty::GridTemplateRows,
    );
    let mismatch_result = TaffyLayoutEngine.compute(&mismatched);
    assert!(
        matches!(
            &mismatch_result,
        Err(crate::LayoutError::CssMathBindingMismatch {
                node,
                property: "grid-template-columns",
            id: crate::LayoutCalcId(7),
            }) if *node == id(1)
        ),
        "mismatched calc identity must fail closed: {mismatch_result:?}"
    );

    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            let property = match axis {
                LayoutGridTrackSizingAxis::Columns => {
                    crate::LayoutCssMathProperty::GridTemplateColumns
                }
                LayoutGridTrackSizingAxis::Rows => crate::LayoutCssMathProperty::GridTemplateRows,
            };
            let mut input = calc_maximum_input(axis, mode, property);
            input.css_math[0].expression = crate::LayoutCssMath::Sum(vec![
                crate::LayoutCssMath::Percentage(0.1),
                crate::LayoutCssMath::LengthPx(20.0),
            ]);
            assert_eq!(initial_tracks(&input, axis).len(), 4, "{axis:?}/{mode:?}");
            assert_axis_frame_size(&input, axis, 38.4);
        }
    }
}

#[test]
fn percentage_and_calc_maximum_use_border_box_content_size_after_insets() {
    for (axis, maximum, expected) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            GridTrackMaxSizingFunction::Percentage(0.25),
            44.0,
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            GridTrackMaxSizingFunction::Calc(crate::LayoutCalcId(7)),
            37.6,
        ),
    ] {
        let mut input = definite_maximum_input(
            axis,
            GridAutoRepeatMode::AutoFill,
            GridTrackMinSizingFunction::LengthPx(16.0),
            maximum,
        );
        input.nodes[0].style.box_sizing = LayoutBoxSizing::BorderBox;
        input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
        input.nodes[0].style.height = LayoutDimension::Fixed(200.0);
        input.nodes[0].style.padding = crate::LayoutEdges {
            top: LayoutLengthPercentage::length(5.0),
            right: LayoutLengthPercentage::length(11.0),
            bottom: LayoutLengthPercentage::length(13.0),
            left: LayoutLengthPercentage::length(7.0),
        };
        input.nodes[0].style.border = LayoutBorder {
            top: 2.0,
            right: 4.0,
            bottom: 4.0,
            left: 2.0,
        };
        input.viewport = Viewport {
            width: 200.0,
            height: 200.0,
        };
        if matches!(maximum, GridTrackMaxSizingFunction::Calc(_)) {
            input.css_math = vec![crate::LayoutCssMathValue {
                id: crate::LayoutCalcId(7),
                node_id: id(1),
                property: match axis {
                    LayoutGridTrackSizingAxis::Columns => {
                        crate::LayoutCssMathProperty::GridTemplateColumns
                    }
                    LayoutGridTrackSizingAxis::Rows => {
                        crate::LayoutCssMathProperty::GridTemplateRows
                    }
                },
                expression: crate::LayoutCssMath::Sum(vec![
                    crate::LayoutCssMath::LengthPx(20.0),
                    crate::LayoutCssMath::Percentage(0.1),
                ]),
            }];
        }

        let tracks = initial_tracks(&input, axis);
        assert_eq!(
            tracks.len(),
            if expected == 44.0 { 3 } else { 4 },
            "{axis:?}"
        );
        assert_axis_frame_size(&input, axis, expected);
    }
}

#[test]
fn percentage_auto_repeat_maximum_requires_a_definite_positive_content_axis() {
    for (label, mutate) in [
        (
            "indefinite width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::Auto;
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }) as Box<dyn Fn(&mut LayoutInput)>,
        ),
        (
            "zero width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::Fixed(0.0);
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
        (
            "constrained maximum",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.max_width = LayoutDimension::Fixed(100.0);
            }),
        ),
        (
            "zero border-box content",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.box_sizing = LayoutBoxSizing::BorderBox;
                input.nodes[0].style.padding.left = LayoutLengthPercentage::length(100.0);
                input.nodes[0].style.padding.right = LayoutLengthPercentage::length(100.0);
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }),
        ),
    ] {
        let mut input = definite_maximum_input(
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            GridTrackMinSizingFunction::LengthPx(16.0),
            GridTrackMaxSizingFunction::Percentage(0.25),
        );
        mutate(&mut input);
        let result = TaffyLayoutEngine.compute(&input);
        assert!(
            matches!(
                result,
                Err(crate::LayoutError::InvalidGridTrackList {
                    node,
                    property: "grid-template-columns",
                }) if node == id(1)
            ),
            "{label} percentage maximum must fail before Taffy sizing"
        );
    }
}

#[test]
fn auto_repeat_calc_maximum_rejects_unlisted_tree_shapes() {
    for expression in [
        crate::LayoutCssMath::Max(vec![
            crate::LayoutCssMath::LengthPx(20.0),
            crate::LayoutCssMath::Percentage(0.1),
        ]),
        crate::LayoutCssMath::Sum(vec![
            crate::LayoutCssMath::LengthPx(20.0),
            crate::LayoutCssMath::Percentage(-0.1),
        ]),
        crate::LayoutCssMath::Sum(vec![
            crate::LayoutCssMath::LengthPx(-20.0),
            crate::LayoutCssMath::Percentage(0.1),
        ]),
        crate::LayoutCssMath::Sum(vec![
            crate::LayoutCssMath::LengthPx(20.0),
            crate::LayoutCssMath::Percentage(0.1),
            crate::LayoutCssMath::LengthPx(1.0),
        ]),
    ] {
        let mut input = calc_maximum_input(
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            crate::LayoutCssMathProperty::GridTemplateColumns,
        );
        input.css_math[0].expression = expression;

        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrackList {
                node,
                property: "grid-template-columns",
            }) if node == id(1)
        ));
    }
}

#[test]
fn finite_percentage_maximum_that_overflows_f32_resolution_fails_closed() {
    let input = definite_maximum_input(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        GridTrackMinSizingFunction::LengthPx(16.0),
        GridTrackMaxSizingFunction::Percentage(f32::MAX),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        }) if node == id(1)
    ));
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
fn percentage_auto_repeat_rejects_indefinite_and_clamped_axes_before_taffy() {
    for (label, mutate) in [
        (
            "indefinite width",
            Box::new(|input: &mut LayoutInput| {
                input.nodes[0].style.width = LayoutDimension::Auto;
                input.root_sizing = RootSizingPolicy::ResolveWithin;
            }) as Box<dyn Fn(&mut LayoutInput)>,
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
