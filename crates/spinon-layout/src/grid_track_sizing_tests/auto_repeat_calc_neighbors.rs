use super::{grid_input, id};
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutCalcId, LayoutCssMath, LayoutCssMathProperty,
    LayoutCssMathValue, LayoutDimension, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingStageKind, LayoutInput, LayoutLengthPercentage, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

enum Neighbor {
    Px(f32),
    Calc(LayoutCssMath),
}

fn input(
    axis: LayoutGridTrackSizingAxis,
    prefix: Option<Neighbor>,
    suffix: Option<Neighbor>,
    mode: GridAutoRepeatMode,
    occupied: bool,
    indefinite_axis: bool,
) -> LayoutInput {
    let property = match axis {
        LayoutGridTrackSizingAxis::Columns => LayoutCssMathProperty::GridTemplateColumns,
        LayoutGridTrackSizingAxis::Rows => LayoutCssMathProperty::GridTemplateRows,
    };
    let mut css_math = Vec::new();
    let mut next_calc_id = 7_100;
    let mut to_track = |neighbor: Neighbor| match neighbor {
        Neighbor::Px(value) => GridTrackSize::LengthPx(value),
        Neighbor::Calc(expression) => {
            let calc_id = LayoutCalcId(next_calc_id);
            next_calc_id += 1;
            css_math.push(LayoutCssMathValue {
                id: calc_id,
                node_id: id(1),
                property,
                expression,
            });
            GridTrackSize::Calc(calc_id)
        }
    };

    let mut tracks = Vec::new();
    if let Some(prefix) = prefix {
        tracks.push(to_track(prefix));
    }
    let start = tracks.len();
    tracks.push(GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    });
    let end = tracks.len();
    if let Some(suffix) = suffix {
        tracks.push(to_track(suffix));
    }

    let (columns, rows, width, height) = match axis {
        LayoutGridTrackSizingAxis::Columns => {
            (tracks, vec![GridTrackSize::LengthPx(12.0)], 240.0, 48.0)
        }
        LayoutGridTrackSizingAxis::Rows => {
            (vec![GridTrackSize::LengthPx(12.0)], tracks, 48.0, 240.0)
        }
    };
    let gap = LayoutGap {
        column: LayoutLengthPercentage::length(4.0),
        row: LayoutLengthPercentage::length(4.0),
    };
    let mut input = grid_input(columns, rows, usize::from(occupied), gap);
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.width = if axis == LayoutGridTrackSizingAxis::Columns && indefinite_axis {
        LayoutDimension::Auto
    } else {
        LayoutDimension::Fixed(width)
    };
    input.nodes[0].style.height = if axis == LayoutGridTrackSizingAxis::Rows && indefinite_axis {
        LayoutDimension::Auto
    } else {
        LayoutDimension::Fixed(height)
    };
    input.viewport = Viewport { width, height };
    input.root_sizing = if indefinite_axis {
        RootSizingPolicy::ResolveWithin
    } else {
        RootSizingPolicy::Match
    };
    input.css_math = css_math;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    let auto_repeat = GridAutoRepeat {
        mode,
        start,
        end,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(auto_repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(auto_repeat),
    }

    if occupied {
        input.nodes[1].style.width =
            LayoutDimension::Fixed(if axis == LayoutGridTrackSizingAxis::Columns {
                2.0
            } else {
                2.0
            });
        input.nodes[1].style.height =
            LayoutDimension::Fixed(if axis == LayoutGridTrackSizingAxis::Rows {
                2.0
            } else {
                2.0
            });
        let line_start = (start + 1) as i32;
        let placement = match axis {
            LayoutGridTrackSizingAxis::Columns => GridItemInput {
                column_start: GridPlacementInput::Line(line_start),
                column_end: GridPlacementInput::Line(line_start + 1),
                ..GridItemInput::default()
            },
            LayoutGridTrackSizingAxis::Rows => GridItemInput {
                row_start: GridPlacementInput::Line(line_start),
                row_end: GridPlacementInput::Line(line_start + 1),
                ..GridItemInput::default()
            },
        };
        input.grid_items.insert(id(2), placement);
    }
    input
}

fn sum(length: f32, percentage: f32, reverse: bool) -> LayoutCssMath {
    let length = LayoutCssMath::LengthPx(length);
    let percentage = LayoutCssMath::Percentage(percentage);
    LayoutCssMath::Sum(if reverse {
        vec![percentage, length]
    } else {
        vec![length, percentage]
    })
}

fn final_tracks(input: &LayoutInput, axis: LayoutGridTrackSizingAxis) -> Vec<(bool, f32)> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    let pass = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap()
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == axis)
        .unwrap();
    let stage = pass
        .stages
        .iter()
        .find(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::ExpandFlexibleTracks
                    | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
            )
        })
        .unwrap();
    stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

fn assert_tracks(input: &LayoutInput, axis: LayoutGridTrackSizingAxis, expected: &[(bool, f32)]) {
    let actual = final_tracks(input, axis);
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(actual.0, expected.0, "track {index} collapse");
        assert!(
            (actual.1 - expected.1).abs() <= 1.0 / 64.0,
            "track {index}: {} != {}",
            actual.1,
            expected.1
        );
    }
}

fn assert_rejected(input: &LayoutInput, property: &'static str) {
    let result = TaffyLayoutEngine.compute(input);
    assert!(
        matches!(
            &result,
            Err(crate::LayoutError::InvalidGridTrackList { node, property: actual })
                if *node == id(1) && *actual == property
        ),
        "expected InvalidGridTrackList for {property}, got {result:?}"
    );
}

fn assert_binding_mismatch(input: &LayoutInput) {
    assert!(matches!(
        TaffyLayoutEngine.compute(input),
        Err(crate::LayoutError::CssMathBindingMismatch {
            node,
            property: "grid-template-columns",
            id: LayoutCalcId(7_100),
        }) if node == id(1)
    ));
}

#[test]
fn two_term_calc_neighbors_size_and_collapse_on_both_axes() {
    let both_fill = [
        (false, 22.0),
        (false, 47.0),
        (false, 47.0),
        (false, 47.0),
        (false, 47.0),
        (false, 10.0),
    ];
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = input(
            axis,
            Some(Neighbor::Calc(sum(10.0, 0.05, false))),
            Some(Neighbor::Calc(sum(10.0, 0.0, true))),
            GridAutoRepeatMode::AutoFill,
            false,
            false,
        );
        assert_tracks(&input, axis, &both_fill);
    }

    let prefix = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        None,
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    assert_tracks(
        &prefix,
        LayoutGridTrackSizingAxis::Columns,
        &[
            (false, 22.0),
            (false, 50.5),
            (false, 50.5),
            (false, 50.5),
            (false, 50.5),
        ],
    );

    let suffix = input(
        LayoutGridTrackSizingAxis::Rows,
        None,
        Some(Neighbor::Calc(sum(10.0, 0.0, true))),
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    assert_tracks(
        &suffix,
        LayoutGridTrackSizingAxis::Rows,
        &[
            (false, 42.0),
            (false, 42.0),
            (false, 42.0),
            (false, 42.0),
            (false, 42.0),
            (false, 10.0),
        ],
    );

    let empty_fit = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        Some(Neighbor::Calc(sum(10.0, 0.0, true))),
        GridAutoRepeatMode::AutoFit,
        false,
        false,
    );
    assert_tracks(
        &empty_fit,
        LayoutGridTrackSizingAxis::Columns,
        &[
            (false, 22.0),
            (true, 0.0),
            (true, 0.0),
            (true, 0.0),
            (true, 0.0),
            (false, 10.0),
        ],
    );
}

#[test]
fn auto_fit_calc_neighbors_keep_the_collapsed_run_boundary_gutter() {
    let input = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        Some(Neighbor::Calc(sum(4.0, 0.025, true))),
        GridAutoRepeatMode::AutoFit,
        true,
        false,
    );
    assert_tracks(
        &input,
        LayoutGridTrackSizingAxis::Columns,
        &[
            (false, 22.0),
            (false, 200.0),
            (true, 0.0),
            (true, 0.0),
            (true, 0.0),
            (false, 10.0),
        ],
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        (output.frames[&id(2)].x, output.frames[&id(2)].y),
        (26.0, 0.0)
    );
}

#[test]
fn calc_neighbor_range_clamps_only_the_final_supported_value() {
    for (expression, expected_prefix, expected_repeat) in [
        (sum(-2.0, 0.05, false), 10.0, 42.0),
        (
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(0.05),
                LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(2.0))),
            ]),
            10.0,
            42.0,
        ),
        (sum(-20.0, 0.05, false), 0.0, 44.0),
    ] {
        let input = input(
            LayoutGridTrackSizingAxis::Columns,
            Some(Neighbor::Calc(expression)),
            None,
            GridAutoRepeatMode::AutoFill,
            false,
            false,
        );
        let expected = std::iter::once((false, expected_prefix))
            .chain(std::iter::repeat_n((false, expected_repeat), 5))
            .collect::<Vec<_>>();
        assert_tracks(&input, LayoutGridTrackSizingAxis::Columns, &expected);
    }
}

#[test]
fn calc_neighbor_rejects_invalid_shapes_basis_identity_and_overflow() {
    let negative_percentage = LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(20.0),
        LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.05))),
    ]);
    let three_terms = LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(10.0),
        LayoutCssMath::Percentage(0.05),
        LayoutCssMath::LengthPx(1.0),
    ]);
    let non_finite = LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(f32::NAN),
        LayoutCssMath::Percentage(0.05),
    ]);
    let unrepresentable = LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(10.0),
        LayoutCssMath::Percentage(f32::MAX),
    ]);

    for expression in [
        negative_percentage,
        three_terms,
        non_finite,
        unrepresentable,
    ] {
        let input = input(
            LayoutGridTrackSizingAxis::Columns,
            Some(Neighbor::Calc(expression)),
            None,
            GridAutoRepeatMode::AutoFill,
            false,
            false,
        );
        assert_rejected(&input, "grid-template-columns");
    }

    let mut missing = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        None,
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    missing.css_math.clear();
    assert!(matches!(
        TaffyLayoutEngine.compute(&missing),
        Err(crate::LayoutError::MissingCssMath {
            node,
            property: "grid-template-columns",
            id: LayoutCalcId(7_100),
        }) if node == id(1)
    ));

    let mut wrong_node = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        None,
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    wrong_node.css_math[0].node_id = id(2);
    assert_binding_mismatch(&wrong_node);

    let mut wrong_property = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        None,
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    wrong_property.css_math[0].property = LayoutCssMathProperty::GridTemplateRows;
    assert_binding_mismatch(&wrong_property);

    let indefinite = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        None,
        GridAutoRepeatMode::AutoFill,
        false,
        true,
    );
    assert_rejected(&indefinite, "grid-template-columns");

    let aggregate_overflow = input(
        LayoutGridTrackSizingAxis::Columns,
        Some(Neighbor::Calc(sum(10.0, 0.05, false))),
        Some(Neighbor::Px(f32::MAX)),
        GridAutoRepeatMode::AutoFill,
        false,
        false,
    );
    assert_rejected(&aggregate_overflow, "grid-template-columns");
}
