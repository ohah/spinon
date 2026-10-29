use super::{grid_input, id};
use crate::{
    GRID_MAX_TRACKS, GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackSize,
    LayoutDimension, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingStageKind, TaffyLayoutEngine, Viewport,
};

const TRACK_CEILING_CASES: [(f32, usize); 3] =
    [(9_999.0, 9_999), (10_000.0, 10_000), (10_001.0, 10_000)];

fn auto_repeat_input(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    basis: f32,
    tracks: Vec<GridTrackSize>,
    repeat_start: usize,
    repeat_end: usize,
) -> crate::LayoutInput {
    let fixed_axis = vec![GridTrackSize::LengthPx(1.0)];
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (tracks, fixed_axis),
        LayoutGridTrackSizingAxis::Rows => (fixed_axis, tracks),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    let (width, height) = match axis {
        LayoutGridTrackSizingAxis::Columns => (basis, 1.0),
        LayoutGridTrackSizingAxis::Rows => (1.0, basis),
    };
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport = Viewport { width, height };

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    let repeat = GridAutoRepeat {
        mode,
        start: repeat_start,
        end: repeat_end,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
    input
}

fn initialize_tracks(
    output: &crate::LayoutOutput,
    axis: LayoutGridTrackSizingAxis,
    label: &str,
) -> (usize, Vec<(bool, f32)>, usize) {
    assert_eq!(
        output.grid_track_sizing_traces.len(),
        1,
        "{label}: trace count"
    );
    let trace = &output.grid_track_sizing_traces[0];
    assert_eq!(trace.node, id(1), "{label}: trace node");
    let trace_state_count = trace
        .passes
        .iter()
        .flat_map(|pass| &pass.stages)
        .map(|stage| stage.tracks.len())
        .sum();
    let (resolved_count, pass) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            trace.used_columns.explicit_track_count,
            trace.passes.iter().rev().find(|pass| pass.axis == axis),
        ),
        LayoutGridTrackSizingAxis::Rows => (
            trace.used_rows.explicit_track_count,
            trace.passes.iter().rev().find(|pass| pass.axis == axis),
        ),
    };
    let pass = pass.unwrap_or_else(|| panic!("{label}: missing final {axis:?} pass"));
    let stages = pass
        .stages
        .iter()
        .filter(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
        .collect::<Vec<_>>();
    assert_eq!(
        stages.len(),
        1,
        "{label}: final pass Initialize stage count"
    );
    let tracks = stages[0]
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect();
    (resolved_count as usize, tracks, trace_state_count)
}

#[test]
fn auto_repeat_track_count_and_empty_fit_collapse_stop_at_the_ceiling_on_both_axes() {
    assert_eq!(GRID_MAX_TRACKS, 10_000);
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for mode in [GridAutoRepeatMode::AutoFill, GridAutoRepeatMode::AutoFit] {
            for (basis, expected_count) in TRACK_CEILING_CASES {
                let label = format!("{axis:?} {mode:?} {basis}px");
                let input =
                    auto_repeat_input(axis, mode, basis, vec![GridTrackSize::LengthPx(1.0)], 0, 1);
                let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                    panic!("{label}: expected bounded layout, got {error:?}")
                });
                let (resolved_count, tracks, trace_state_count) =
                    initialize_tracks(&output, axis, &label);
                assert_eq!(resolved_count, expected_count, "{label}: resolved count");
                assert_eq!(tracks.len(), expected_count, "{label}: stage track count");
                assert!(
                    trace_state_count < 100_000,
                    "{label}: {trace_state_count} trace states"
                );
                assert!(
                    tracks.iter().all(|(collapsed, _)| {
                        *collapsed == (mode == GridAutoRepeatMode::AutoFit)
                    }),
                    "{label}: empty auto-fit collapse state"
                );
            }
        }
    }
}

#[test]
fn auto_repeat_ceiling_preserves_prefix_and_truncates_suffix_in_source_order() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        for (basis, expected_repeat_count, expected_last_track) in
            [(10_003.0, 9_998, 3.0), (10_004.0, 9_999, 1.0)]
        {
            let label = format!("{axis:?} {basis}px source order");
            let input = auto_repeat_input(
                axis,
                GridAutoRepeatMode::AutoFill,
                basis,
                vec![
                    GridTrackSize::LengthPx(2.0),
                    GridTrackSize::LengthPx(1.0),
                    GridTrackSize::LengthPx(3.0),
                ],
                1,
                2,
            );
            let output = TaffyLayoutEngine
                .compute(&input)
                .unwrap_or_else(|error| panic!("{label}: expected bounded layout, got {error:?}"));
            let (resolved_count, tracks, trace_state_count) =
                initialize_tracks(&output, axis, &label);
            assert_eq!(resolved_count, GRID_MAX_TRACKS, "{label}: resolved count");
            assert_eq!(tracks.len(), GRID_MAX_TRACKS, "{label}: stage track count");
            assert!(
                trace_state_count < 100_000,
                "{label}: {trace_state_count} trace states"
            );
            assert_eq!(
                tracks.first().map(|track| track.1),
                Some(2.0),
                "{label}: prefix"
            );
            assert_eq!(
                tracks.iter().filter(|track| track.1 == 1.0).count(),
                expected_repeat_count,
                "{label}: repeated count"
            );
            assert_eq!(
                tracks.last().map(|track| track.1),
                Some(expected_last_track),
                "{label}: suffix preservation/truncation"
            );
            assert!(
                tracks.iter().all(|track| !track.0),
                "{label}: auto-fill collapse"
            );
        }
    }
}
