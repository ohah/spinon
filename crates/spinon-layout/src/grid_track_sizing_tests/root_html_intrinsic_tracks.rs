use super::*;
use crate::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};

fn root_html_intrinsic_input(columns: Vec<GridTrackSize>, width: f32) -> LayoutInput {
    let mut input = super::grid_input(
        columns,
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.viewport.width = width;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    container.track_sizing_trace_enabled = true;
    input
}

fn assert_column_tracks(columns: Vec<GridTrackSize>, width: f32, expected: &[f32], case: &str) {
    let input = root_html_intrinsic_input(columns, width);
    let output = TaffyLayoutEngine
        .compute(&input)
        .unwrap_or_else(|error| panic!("{case}: {error:?}"));
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap_or_else(|| panic!("{case}: missing Grid track sizing trace"));
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{case}: missing columns sizing pass"));
    assert!(pass.stages.iter().any(|stage| {
        matches!(
            stage.stage,
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
        )
    }));
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final columns stage"));
    let actual = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len(), "{case}: track count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.02,
            "{case}: columns[{index}] expected {expected}, got {actual}"
        );
    }
}

#[test]
fn root_html_intrinsic_tracks_match_fixed_input_expectations() {
    assert_column_tracks(
        vec![GridTrackSize::MinContent],
        320.0,
        &[20.0],
        "min-content",
    );
    assert_column_tracks(
        vec![GridTrackSize::MaxContent],
        320.0,
        &[20.0],
        "max-content",
    );
    assert_column_tracks(vec![GridTrackSize::Auto], 320.0, &[320.0], "auto stretch");
    assert_column_tracks(
        vec![GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::MinContent,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        }],
        160.0,
        &[160.0],
        "intrinsic minimum with flex maximum",
    );
    assert_column_tracks(
        vec![
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::Percentage(0.5),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(100.0),
        ],
        160.0,
        &[80.0, 100.0],
        "percentage minimum with flex maximum",
    );
}

#[test]
fn root_html_intrinsic_tracks_reject_mixed_auto_and_one_axis_placement() {
    let mut input = super::grid_input(
        vec![GridTrackSize::MinContent, GridTrackSize::MaxContent],
        vec![GridTrackSize::LengthPx(30.0)],
        2,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    input.grid_items.insert(
        id(3),
        GridItemInput {
            column_start: GridPlacementInput::Line(2),
            ..GridItemInput::default()
        },
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement { .. })
    ));
}
