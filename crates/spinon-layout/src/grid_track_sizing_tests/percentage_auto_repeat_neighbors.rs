use super::{grid_input, id};
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutBorder, LayoutBoxSizing, LayoutDimension,
    LayoutEdges, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingStageKind, LayoutInput, LayoutLengthPercentage, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

fn input_with_neighbors(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    prefix: Option<f32>,
    suffix: Option<f32>,
    border_box_axis: bool,
    occupied: bool,
) -> LayoutInput {
    let mut tracks = Vec::new();
    if let Some(value) = prefix {
        tracks.push(GridTrackSize::Percentage(value));
    }
    let start = tracks.len();
    tracks.push(GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(40.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    });
    let end = tracks.len();
    if let Some(value) = suffix {
        tracks.push(GridTrackSize::Percentage(value));
    }

    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (tracks, vec![GridTrackSize::LengthPx(12.0)]),
        LayoutGridTrackSizingAxis::Rows => (vec![GridTrackSize::LengthPx(12.0)], tracks),
    };
    let gap = LayoutGap {
        column: LayoutLengthPercentage::length(4.0),
        row: LayoutLengthPercentage::length(4.0),
    };
    let mut input = grid_input(columns, rows, usize::from(occupied), gap);
    let (width, height) = match (axis, border_box_axis) {
        (LayoutGridTrackSizingAxis::Columns, false) => (260.0, 48.0),
        (LayoutGridTrackSizingAxis::Rows, false) => (48.0, 260.0),
        (LayoutGridTrackSizingAxis::Columns, true) => (136.0, 48.0),
        (LayoutGridTrackSizingAxis::Rows, true) => (48.0, 136.0),
    };
    input.nodes[0].style.box_sizing = LayoutBoxSizing::BorderBox;
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.padding = LayoutEdges {
        top: LayoutLengthPercentage::length(8.0),
        right: LayoutLengthPercentage::length(8.0),
        bottom: LayoutLengthPercentage::length(8.0),
        left: LayoutLengthPercentage::length(8.0),
    };
    input.nodes[0].style.border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    input.viewport = Viewport { width, height };
    input.root_sizing = RootSizingPolicy::Match;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    let repeat = GridAutoRepeat {
        mode,
        start,
        end,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }

    if occupied {
        input.nodes[1].style.width = LayoutDimension::Fixed(2.0);
        input.nodes[1].style.height = LayoutDimension::Fixed(2.0);
        let placement = match axis {
            LayoutGridTrackSizingAxis::Columns => GridItemInput {
                column_start: GridPlacementInput::Line((start + 1) as i32),
                column_end: GridPlacementInput::Line((start + 2) as i32),
                ..GridItemInput::default()
            },
            LayoutGridTrackSizingAxis::Rows => GridItemInput {
                row_start: GridPlacementInput::Line((start + 1) as i32),
                row_end: GridPlacementInput::Line((start + 2) as i32),
                ..GridItemInput::default()
            },
        };
        input.grid_items.insert(id(2), placement);
    }
    input
}

fn tracks_at_expand_stage(
    input: &LayoutInput,
    axis: LayoutGridTrackSizingAxis,
    scene: &str,
) -> Vec<(bool, f32)> {
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
        .unwrap_or_else(|| {
            let recorded = pass
                .stages
                .iter()
                .map(|stage| format!("{:?}", stage.stage))
                .collect::<Vec<_>>();
            panic!("{scene} ({axis:?}): missing final sizing stage; recorded stages: {recorded:?}");
        });
    stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

fn assert_tracks(actual: &[(bool, f32)], expected: &[(bool, f32)], scene: &str) {
    assert_eq!(actual.len(), expected.len(), "{scene} track count");
    for (index, ((actual_collapsed, actual_size), (expected_collapsed, expected_size))) in
        actual.iter().zip(expected).enumerate()
    {
        assert_eq!(
            actual_collapsed, expected_collapsed,
            "{scene} track {index} collapse"
        );
        assert!(
            (actual_size - expected_size).abs() <= 1.0 / 64.0,
            "{scene} track {index}: {actual_size} != {expected_size}"
        );
    }
}

#[test]
fn definite_percentage_neighbors_match_the_fixed_thirteen_scene_matrix() {
    let both_fill = [
        (false, 24.0),
        (false, 46.0),
        (false, 46.0),
        (false, 46.0),
        (false, 46.0),
        (false, 12.0),
    ];
    let first_fit = [
        (false, 24.0),
        (false, 196.0),
        (true, 0.0),
        (true, 0.0),
        (true, 0.0),
        (false, 12.0),
    ];
    let empty_fit = [
        (false, 24.0),
        (true, 0.0),
        (true, 0.0),
        (true, 0.0),
        (true, 0.0),
        (false, 12.0),
    ];
    let prefix = [
        (false, 30.0),
        (false, 48.5),
        (false, 48.5),
        (false, 48.5),
        (false, 48.5),
    ];
    let suffix = [
        (false, 48.5),
        (false, 48.5),
        (false, 48.5),
        (false, 48.5),
        (false, 30.0),
    ];
    let overflow = [(false, 192.0), (false, 40.0), (false, 72.0)];
    let zero_prefix = [
        (false, 0.0),
        (false, 40.8),
        (false, 40.8),
        (false, 40.8),
        (false, 40.8),
        (false, 40.8),
        (false, 12.0),
    ];
    let border_box = [
        (false, 11.59375),
        (false, 43.296875),
        (false, 43.296875),
        (false, 5.796875),
    ];

    let cases = [
        (
            "col-fill-both",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            Some(0.1),
            Some(0.05),
            false,
            false,
            both_fill.as_slice(),
        ),
        (
            "row-fill-both",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFill,
            Some(0.1),
            Some(0.05),
            false,
            false,
            both_fill.as_slice(),
        ),
        (
            "col-fill-prefix",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            Some(0.125),
            None,
            false,
            false,
            prefix.as_slice(),
        ),
        (
            "row-fill-suffix",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFill,
            None,
            Some(0.125),
            false,
            false,
            suffix.as_slice(),
        ),
        (
            "col-fit-first",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFit,
            Some(0.1),
            Some(0.05),
            false,
            true,
            first_fit.as_slice(),
        ),
        (
            "row-fit-first",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFit,
            Some(0.1),
            Some(0.05),
            false,
            true,
            first_fit.as_slice(),
        ),
        (
            "col-fit-empty",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFit,
            Some(0.1),
            Some(0.05),
            false,
            false,
            empty_fit.as_slice(),
        ),
        (
            "row-fit-empty",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFit,
            Some(0.1),
            Some(0.05),
            false,
            false,
            empty_fit.as_slice(),
        ),
        (
            "col-fill-overflow",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            Some(0.8),
            Some(0.3),
            false,
            false,
            overflow.as_slice(),
        ),
        (
            "row-fill-overflow",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFill,
            Some(0.8),
            Some(0.3),
            false,
            false,
            overflow.as_slice(),
        ),
        (
            "col-fill-zero-prefix",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            Some(0.0),
            Some(0.05),
            false,
            false,
            zero_prefix.as_slice(),
        ),
        (
            "col-fill-border-box",
            LayoutGridTrackSizingAxis::Columns,
            GridAutoRepeatMode::AutoFill,
            Some(0.1),
            Some(0.05),
            true,
            false,
            border_box.as_slice(),
        ),
        (
            "row-fill-border-box",
            LayoutGridTrackSizingAxis::Rows,
            GridAutoRepeatMode::AutoFill,
            Some(0.1),
            Some(0.05),
            true,
            false,
            border_box.as_slice(),
        ),
    ];

    for (scene, axis, mode, prefix, suffix, border_box, occupied, expected) in cases {
        let input = input_with_neighbors(axis, mode, prefix, suffix, border_box, occupied);
        assert_tracks(
            &tracks_at_expand_stage(&input, axis, scene),
            expected,
            scene,
        );
        if occupied {
            let output = TaffyLayoutEngine.compute(&input).unwrap();
            let frame = &output.frames[&id(2)];
            let (expected_x, expected_y) = match axis {
                LayoutGridTrackSizingAxis::Columns => (38.0, 10.0),
                LayoutGridTrackSizingAxis::Rows => (10.0, 38.0),
            };
            assert!(
                (frame.x - expected_x).abs() <= 1.0 / 64.0,
                "{scene} x: {}",
                frame.x
            );
            assert!(
                (frame.y - expected_y).abs() <= 1.0 / 64.0,
                "{scene} y: {}",
                frame.y
            );
            assert!(
                (frame.width - 2.0).abs() <= 1.0 / 64.0,
                "{scene} width: {}",
                frame.width
            );
            assert!(
                (frame.height - 2.0).abs() <= 1.0 / 64.0,
                "{scene} height: {}",
                frame.height
            );
        }
    }
}

#[test]
fn percentage_neighbors_reject_indefinite_non_finite_and_negative_inputs() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let mut indefinite = input_with_neighbors(
            axis,
            GridAutoRepeatMode::AutoFill,
            Some(0.1),
            None,
            false,
            false,
        );
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                indefinite.nodes[0].style.width = LayoutDimension::Auto;
            }
            LayoutGridTrackSizingAxis::Rows => {
                indefinite.nodes[0].style.height = LayoutDimension::Auto;
            }
        }
        assert!(
            TaffyLayoutEngine.compute(&indefinite).is_err(),
            "{axis:?} indefinite percentage basis must fail closed"
        );

        for value in [-0.1, f32::NAN, f32::INFINITY, f32::MAX] {
            let invalid = input_with_neighbors(
                axis,
                GridAutoRepeatMode::AutoFill,
                Some(value),
                None,
                false,
                false,
            );
            assert!(
                TaffyLayoutEngine.compute(&invalid).is_err(),
                "{axis:?} invalid percentage neighbor {value:?} must fail closed"
            );
        }
    }
}

#[test]
fn percentage_neighbors_with_individually_finite_sum_overflow_fail_closed() {
    let mut input = input_with_neighbors(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        Some(f32::MAX / 3.0),
        Some(f32::MAX / 3.0),
        false,
        false,
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(22.0);
    input.viewport = Viewport {
        width: 22.0,
        height: 48.0,
    };

    assert!(
        (f32::MAX / 3.0 * 2.0).is_finite(),
        "each neighbor must individually resolve to a finite f32 value"
    );
    assert!(
        TaffyLayoutEngine.compute(&input).is_err(),
        "the combined prefix and suffix must fail closed when their sum overflows f32"
    );
}
