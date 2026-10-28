use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingContributionTrace,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
    LayoutGridTrackSizingTrackState,
};

use super::trace_validation::{validate_columns_pass, validate_rows_pass};
use super::validate_grid_input;

fn expected_grid_input() -> GridContainerInput {
    let fit_content = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0));
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(60.0)],
        columns_line_names: vec![vec![], vec![]],
        auto_columns: GridAutoColumns::TrackList(vec![fit_content]),
        auto_rows: GridAutoRows::TrackList(vec![fit_content]),
        auto_flow: GridAutoFlow::Row,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

fn expected_pass(axis: LayoutGridTrackSizingAxis, sequence: u16) -> LayoutGridTrackSizingPassTrace {
    let bases: Vec<Vec<f32>> = match axis {
        LayoutGridTrackSizingAxis::Columns => vec![
            vec![60.0, 0.0],
            vec![60.0, 0.0],
            vec![60.0, 0.0],
            vec![60.0, 40.0],
            vec![60.0, 40.0],
            vec![60.0, 40.0],
        ],
        LayoutGridTrackSizingAxis::Rows => {
            vec![
                vec![0.0],
                vec![10.0],
                vec![10.0],
                vec![10.0],
                vec![10.0],
                vec![10.0],
            ]
        }
    };
    let stages = [
        LayoutGridTrackSizingStageKind::Initialize,
        LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
            span: 1,
            crosses_flexible_track: false,
        },
        LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        LayoutGridTrackSizingStageKind::MaximizeTracks,
        LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        LayoutGridTrackSizingStageKind::StretchAutoTracks {
            applied: axis == LayoutGridTrackSizingAxis::Rows,
        },
    ]
    .into_iter()
    .zip(bases)
    .map(|(stage, sizes)| {
        let mut tracks = Vec::with_capacity(sizes.len() * 2 + 1);
        for index in 0..(sizes.len() * 2 + 1) {
            let is_gutter = index % 2 == 0;
            let is_collapsed_boundary_gutter =
                is_gutter && (index == 0 || index + 1 == sizes.len() * 2 + 1);
            tracks.push(LayoutGridTrackSizingTrackState {
                internal_index: index as u16,
                is_gutter,
                is_collapsed: is_collapsed_boundary_gutter,
                base_size: if is_gutter { 0.0 } else { sizes[index / 2] },
                growth_limit: is_gutter.then_some(0.0),
                infinitely_growable: false,
            });
        }
        LayoutGridTrackSizingStageTrace { stage, tracks }
    })
    .collect();
    let contributions = if axis == LayoutGridTrackSizingAxis::Columns {
        vec![
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::Minimum,
                span: 1,
                crosses_flexible_track: false,
                value: 0.0,
                minimum_source: Some(
                    LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum,
                ),
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MinContent,
                span: 1,
                crosses_flexible_track: false,
                value: 400.0,
                minimum_source: None,
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MaxContent,
                span: 1,
                crosses_flexible_track: false,
                value: 400.0,
                minimum_source: None,
            },
        ]
    } else {
        vec![
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::Minimum,
                span: 1,
                crosses_flexible_track: false,
                value: 10.0,
                minimum_source: Some(
                    LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                        spans_auto_min_track: true,
                        spans_multiple_tracks: false,
                        spans_flexible_max_track: false,
                        content_based: true,
                        is_compressible_replaced: false,
                        fixed_track_sum: None,
                        fixed_gutter_sum: None,
                        fixed_track_limit: None,
                    },
                ),
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MinContent,
                span: 1,
                crosses_flexible_track: false,
                value: 10.0,
                minimum_source: None,
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MaxContent,
                span: 1,
                crosses_flexible_track: false,
                value: 10.0,
                minimum_source: None,
            },
        ]
    };
    LayoutGridTrackSizingPassTrace {
        sequence,
        axis,
        stages,
        contributions,
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    }
}

#[test]
fn accepts_only_the_pinned_implicit_fit_content_track_input() {
    let root = NodeId::new(1).unwrap();
    assert!(validate_grid_input(root, &expected_grid_input()).is_ok());

    let mut changed_limit = expected_grid_input();
    changed_limit.auto_columns = GridAutoColumns::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(41.0),
    )]);
    assert!(validate_grid_input(root, &changed_limit).is_err());

    let mut added_explicit_column = expected_grid_input();
    added_explicit_column
        .columns
        .push(GridTrackSize::LengthPx(40.0));
    assert!(validate_grid_input(root, &added_explicit_column).is_err());

    let mut changed_flow = expected_grid_input();
    changed_flow.auto_flow = GridAutoFlow::Column;
    assert!(validate_grid_input(root, &changed_flow).is_err());
}

#[test]
fn accepts_only_the_pinned_column_trace_and_contributions() {
    let item = NodeId::new(2).unwrap();
    let expected = expected_pass(LayoutGridTrackSizingAxis::Columns, 0);
    assert!(validate_columns_pass(&expected, item).is_ok());

    let mut wrong_stage_base = expected.clone();
    wrong_stage_base.stages[3].tracks[3].base_size = 39.0;
    assert!(validate_columns_pass(&wrong_stage_base, item).is_err());

    let mut collapsed_active_gutter = expected.clone();
    collapsed_active_gutter.stages[0].tracks[2].is_collapsed = true;
    assert!(validate_columns_pass(&collapsed_active_gutter, item).is_err());

    let mut active_boundary_gutter = expected.clone();
    active_boundary_gutter.stages[0].tracks[0].is_collapsed = false;
    assert!(validate_columns_pass(&active_boundary_gutter, item).is_err());

    let mut wrong_item_minimum = expected.clone();
    wrong_item_minimum.contributions[0].value = 40.0;
    assert!(validate_columns_pass(&wrong_item_minimum, item).is_err());

    let mut wrong_max_content = expected.clone();
    wrong_max_content.contributions[2].value = 399.0;
    assert!(validate_columns_pass(&wrong_max_content, item).is_err());

    assert!(validate_columns_pass(&expected, NodeId::new(3).unwrap()).is_err());
}

#[test]
fn accepts_only_the_pinned_row_cross_axis_trace() {
    let item = NodeId::new(2).unwrap();
    let expected = expected_pass(LayoutGridTrackSizingAxis::Rows, 1);
    assert!(validate_rows_pass(&expected, item).is_ok());

    let mut wrong_row_base = expected.clone();
    wrong_row_base.stages[2].tracks[1].base_size = 9.0;
    assert!(validate_rows_pass(&wrong_row_base, item).is_err());

    let mut extra_row = expected.clone();
    extra_row.stages[2]
        .tracks
        .push(LayoutGridTrackSizingTrackState {
            internal_index: 3,
            is_gutter: false,
            is_collapsed: false,
            base_size: 10.0,
            growth_limit: None,
            infinitely_growable: false,
        });
    assert!(validate_rows_pass(&extra_row, item).is_err());

    let mut wrong_minimum_source = expected.clone();
    wrong_minimum_source.contributions[0].minimum_source =
        Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum);
    assert!(validate_rows_pass(&wrong_minimum_source, item).is_err());

    let mut wrong_max_content = expected;
    wrong_max_content.contributions[2].value = 9.0;
    assert!(validate_rows_pass(&wrong_max_content, item).is_err());
}
