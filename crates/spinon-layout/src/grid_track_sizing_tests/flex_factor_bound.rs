use super::{grid_input, id};
use crate::{
    GRID_MAX_TRACKS, GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutDimension,
    LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind,
    TaffyLayoutEngine, Viewport,
};

fn minmax_px_flex(minimum: f32, factor: f32) -> GridTrackSize {
    GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(minimum),
        max: GridTrackMaxSizingFunction::Fraction(factor),
    }
}

fn set_track_sizing_profile(input: &mut crate::LayoutInput) {
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
}

fn set_auto_repeat_axis(
    input: &mut crate::LayoutInput,
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    start: usize,
    end: usize,
) {
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
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
}

fn axis_property(axis: LayoutGridTrackSizingAxis) -> &'static str {
    match axis {
        LayoutGridTrackSizingAxis::Columns => "grid-template-columns",
        LayoutGridTrackSizingAxis::Rows => "grid-template-rows",
    }
}

fn tracks_at_stage(
    input: &crate::LayoutInput,
    axis: LayoutGridTrackSizingAxis,
    target_stage: LayoutGridTrackSizingStageKind,
) -> Vec<(bool, f32)> {
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
        .find(|stage| stage.stage == target_stage)
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

fn zero_and_sub_one_flex_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![minmax_px_flex(0.0, 0.0), minmax_px_flex(0.0, 0.5)],
            vec![GridTrackSize::LengthPx(120.0)],
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![minmax_px_flex(0.0, 0.0), minmax_px_flex(0.0, 0.5)],
        ),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(200.0);
    input.viewport = Viewport {
        width: 200.0,
        height: 200.0,
    };
    set_track_sizing_profile(&mut input);
    input
}

fn flex_overflow_without_repeat_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let overflow_tracks = vec![minmax_px_flex(1.0, 1.0e38); 4];
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => {
            (overflow_tracks, vec![GridTrackSize::LengthPx(120.0)])
        }
        LayoutGridTrackSizingAxis::Rows => (vec![GridTrackSize::LengthPx(120.0)], overflow_tracks),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    set_track_sizing_profile(&mut input);
    input
}

fn flex_overflow_at_auto_repeat_ceiling_input(
    axis: LayoutGridTrackSizingAxis,
) -> crate::LayoutInput {
    let repeated = minmax_px_flex(1.0, 1.0e38);
    let (columns, rows, repeat_start, repeat_end) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![GridTrackSize::LengthPx(1.0), repeated],
            vec![GridTrackSize::LengthPx(120.0)],
            1,
            2,
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![GridTrackSize::LengthPx(1.0), repeated],
            1,
            2,
        ),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    match axis {
        LayoutGridTrackSizingAxis::Columns => {
            input.nodes[0].style.width = LayoutDimension::Fixed(GRID_MAX_TRACKS as f32);
            input.nodes[0].style.height = LayoutDimension::Fixed(120.0);
        }
        LayoutGridTrackSizingAxis::Rows => {
            input.nodes[0].style.width = LayoutDimension::Fixed(120.0);
            input.nodes[0].style.height = LayoutDimension::Fixed(GRID_MAX_TRACKS as f32);
        }
    }
    input.viewport = Viewport {
        width: match axis {
            LayoutGridTrackSizingAxis::Columns => GRID_MAX_TRACKS as f32,
            LayoutGridTrackSizingAxis::Rows => 120.0,
        },
        height: match axis {
            LayoutGridTrackSizingAxis::Columns => 120.0,
            LayoutGridTrackSizingAxis::Rows => GRID_MAX_TRACKS as f32,
        },
    };
    set_track_sizing_profile(&mut input);
    set_auto_repeat_axis(
        &mut input,
        axis,
        GridAutoRepeatMode::AutoFill,
        repeat_start,
        repeat_end,
    );
    input
}

fn fixed_min_flex_neighbors_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let (columns, rows, start, end, width, height) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![
                minmax_px_flex(12.0, 0.5),
                minmax_px_flex(24.0, 1.0),
                minmax_px_flex(16.0, 2.0),
            ],
            vec![GridTrackSize::LengthPx(120.0)],
            1,
            2,
            220.0,
            120.0,
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![
                minmax_px_flex(12.0, 0.5),
                minmax_px_flex(24.0, 1.0),
                minmax_px_flex(16.0, 2.0),
            ],
            1,
            2,
            120.0,
            220.0,
        ),
    };
    let gap = LayoutGap {
        column: crate::LayoutLengthPercentage::length(4.0),
        row: crate::LayoutLengthPercentage::length(4.0),
    };
    let mut input = grid_input(columns, rows, 0, gap);
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport = Viewport { width, height };
    set_track_sizing_profile(&mut input);
    set_auto_repeat_axis(
        &mut input,
        axis,
        crate::GridAutoRepeatMode::AutoFill,
        start,
        end,
    );
    input
}

#[test]
fn zero_and_sub_one_flex_factors_remain_valid_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = zero_and_sub_one_flex_input(axis);
        let actual = tracks_at_stage(
            &input,
            axis,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        );
        assert_eq!(actual.len(), 2, "{axis:?} flex track count");
        assert!(actual.iter().all(|(collapsed, _)| !collapsed));
        assert!((actual[0].1 - 0.0).abs() <= 0.02, "{axis:?}: {actual:?}");
        assert!((actual[1].1 - 100.0).abs() <= 0.02, "{axis:?}: {actual:?}");
    }
}

#[test]
fn flex_factor_sum_overflow_is_rejected_with_axis_property_context() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = flex_overflow_without_repeat_input(axis);
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrackList { node, property })
                if node == id(1) && property == axis_property(axis)
        ));
    }
}

#[test]
fn auto_repeat_flex_factor_bound_uses_the_track_ceiling_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = flex_overflow_at_auto_repeat_ceiling_input(axis);
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrackList { node, property })
                if node == id(1) && property == axis_property(axis)
        ));
    }
}

#[test]
fn fixed_minimum_flex_maximum_neighbors_size_auto_repeat_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = fixed_min_flex_neighbors_input(axis);
        let tracks = tracks_at_stage(
            &input,
            axis,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        );
        assert_eq!(tracks.len(), 8, "{axis:?} repeat count uses fixed minima");
        assert!(tracks.iter().all(|(collapsed, _)| !collapsed), "{axis:?}");
    }
}
