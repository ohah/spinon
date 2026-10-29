use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingContributionTrace,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
    LayoutGridTrackSizingTrackState,
};

use super::validation::{validate_columns_pass, validate_grid_input, validate_rows_pass};
use super::{CASES, Case};

fn expected_grid_input(case: Case) -> GridContainerInput {
    let limit = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(case.limit));
    GridContainerInput {
        rows: vec![GridTrackSize::LengthPx(60.0)],
        rows_line_names: vec![vec![], vec![]],
        auto_columns: GridAutoColumns::TrackList(vec![limit]),
        auto_rows: GridAutoRows::Auto,
        auto_flow: GridAutoFlow::Column,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

fn minimum_source(case: Case) -> LayoutGridTrackSizingMinimumContributionSource {
    if case.automatic_minimum {
        LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
            spans_auto_min_track: true,
            spans_multiple_tracks: false,
            spans_flexible_max_track: false,
            content_based: true,
            is_compressible_replaced: false,
            fixed_track_sum: None,
            fixed_gutter_sum: None,
            fixed_track_limit: None,
        }
    } else {
        LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum
    }
}

fn expected_columns_pass(case: Case) -> LayoutGridTrackSizingPassTrace {
    let minimum = if case.automatic_minimum { 100.0 } else { 0.0 };
    LayoutGridTrackSizingPassTrace {
        sequence: 0,
        axis: LayoutGridTrackSizingAxis::Columns,
        stages: vec![LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::MaximizeTracks,
            tracks: vec![LayoutGridTrackSizingTrackState {
                internal_index: 1,
                is_gutter: false,
                is_collapsed: false,
                base_size: case.expected_column,
                growth_limit: Some(100.0),
                infinitely_growable: false,
            }],
        }],
        contributions: vec![
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::Minimum,
                span: 1,
                crosses_flexible_track: false,
                value: minimum,
                minimum_source: Some(minimum_source(case)),
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MinContent,
                span: 1,
                crosses_flexible_track: false,
                value: 100.0,
                minimum_source: None,
            },
            LayoutGridTrackSizingContributionTrace {
                item: NodeId::new(2).unwrap(),
                kind: LayoutGridTrackSizingContributionKind::MaxContent,
                span: 1,
                crosses_flexible_track: false,
                value: 100.0,
                minimum_source: None,
            },
        ],
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    }
}

fn expected_rows_pass() -> LayoutGridTrackSizingPassTrace {
    LayoutGridTrackSizingPassTrace {
        sequence: 1,
        axis: LayoutGridTrackSizingAxis::Rows,
        stages: [
            LayoutGridTrackSizingStageKind::Initialize,
            LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn,
        ]
        .into_iter()
        .map(|stage| LayoutGridTrackSizingStageTrace {
            stage,
            tracks: vec![LayoutGridTrackSizingTrackState {
                internal_index: 1,
                is_gutter: false,
                is_collapsed: false,
                base_size: 60.0,
                growth_limit: Some(60.0),
                infinitely_growable: false,
            }],
        })
        .collect(),
        contributions: Vec::new(),
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    }
}

#[test]
fn pins_four_column_limit_scenes_to_the_fit_content_formula() {
    assert_eq!(CASES.len(), 4);
    for case in CASES {
        let minimum: f32 = if case.automatic_minimum { 100.0 } else { 0.0 };
        let expected = minimum.max(case.limit.min(100.0));
        assert_eq!(case.expected_column, expected, "{}", case.scene_id);
        assert!(
            validate_grid_input(NodeId::new(1).unwrap(), &expected_grid_input(case), case,).is_ok()
        );
        assert!(
            validate_columns_pass(&expected_columns_pass(case), NodeId::new(2).unwrap(), case)
                .is_ok()
        );
    }
    assert_eq!(CASES[2].root_width, 300.0);
    assert!(CASES[3].automatic_minimum);
    assert_eq!(CASES[3].limit, 40.0);
    assert_eq!(CASES[3].expected_column, 100.0);
}

#[test]
fn rejects_changed_column_limits_flow_and_explicit_tracks() {
    let case = CASES[0];
    let root = NodeId::new(1).unwrap();
    let mut changed_limit = expected_grid_input(case);
    changed_limit.auto_columns = GridAutoColumns::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(41.0),
    )]);
    assert!(validate_grid_input(root, &changed_limit, case).is_err());

    let mut changed_flow = expected_grid_input(case);
    changed_flow.auto_flow = GridAutoFlow::Row;
    assert!(validate_grid_input(root, &changed_flow, case).is_err());

    let mut explicit_column = expected_grid_input(case);
    explicit_column.columns.push(GridTrackSize::LengthPx(60.0));
    assert!(validate_grid_input(root, &explicit_column, case).is_err());
}

#[test]
fn rejects_minimum_contribution_or_raw_growth_limit_confusion() {
    let case = CASES[0];
    let item = NodeId::new(2).unwrap();
    let expected = expected_columns_pass(case);
    assert!(validate_columns_pass(&expected, item, case).is_ok());

    let mut wrong_minimum = expected.clone();
    wrong_minimum.contributions[0].value = 1.0;
    assert!(validate_columns_pass(&wrong_minimum, item, case).is_err());

    let mut wrong_max_content = expected.clone();
    wrong_max_content.contributions[2].value = 99.0;
    assert!(validate_columns_pass(&wrong_max_content, item, case).is_err());

    let mut clamped_growth_limit = expected.clone();
    clamped_growth_limit.stages[0].tracks[0].growth_limit = Some(40.0);
    assert!(validate_columns_pass(&clamped_growth_limit, item, case).is_err());

    let mut wrong_base = expected;
    wrong_base.stages[0].tracks[0].base_size = 39.0;
    assert!(validate_columns_pass(&wrong_base, item, case).is_err());
}

#[test]
fn enforces_the_automatic_minimum_floor_and_the_cross_axis_row() {
    let control = CASES[3];
    let expected = expected_columns_pass(control);
    assert!(validate_columns_pass(&expected, NodeId::new(2).unwrap(), control).is_ok());

    let mut removed_floor = expected.clone();
    removed_floor.contributions[0].value = 40.0;
    assert!(validate_columns_pass(&removed_floor, NodeId::new(2).unwrap(), control).is_err());

    let mut wrong_minimum_source = expected;
    wrong_minimum_source.contributions[0].minimum_source =
        Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum);
    assert!(
        validate_columns_pass(&wrong_minimum_source, NodeId::new(2).unwrap(), control).is_err()
    );

    assert!(validate_rows_pass(&expected_rows_pass()).is_ok());
    let mut wrong_row = expected_rows_pass();
    wrong_row.stages[0].tracks[0].base_size = 59.0;
    assert!(validate_rows_pass(&wrong_row).is_err());
}
