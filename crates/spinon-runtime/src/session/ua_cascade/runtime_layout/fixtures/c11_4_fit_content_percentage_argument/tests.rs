use super::validate_grid_input;
use spinon_core::NodeId;
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingContributionTrace, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
    LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrackState,
};

use super::trace_validation::validate_rows_pass;

fn expected_grid_input() -> GridContainerInput {
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(60.0)],
        columns_line_names: vec![vec![], vec![]],
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Percentage(0.5),
        )]),
        auto_flow: GridAutoFlow::Row,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

fn expected_rows_pass() -> LayoutGridTrackSizingPassTrace {
    let stage_bases = [
        (LayoutGridTrackSizingStageKind::Initialize, 0.0),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            0.0,
        ),
        (LayoutGridTrackSizingStageKind::IntrinsicSizingComplete, 0.0),
        (LayoutGridTrackSizingStageKind::MaximizeTracks, 150.0),
        (LayoutGridTrackSizingStageKind::ExpandFlexibleTracks, 150.0),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            150.0,
        ),
    ];
    let stages = stage_bases
        .into_iter()
        .map(|(stage, base)| LayoutGridTrackSizingStageTrace {
            stage,
            tracks: vec![
                LayoutGridTrackSizingTrackState {
                    internal_index: 0,
                    is_gutter: true,
                    is_collapsed: true,
                    base_size: 0.0,
                    growth_limit: Some(0.0),
                    infinitely_growable: false,
                },
                LayoutGridTrackSizingTrackState {
                    internal_index: 1,
                    is_gutter: false,
                    is_collapsed: false,
                    base_size: base,
                    growth_limit: Some(400.0),
                    infinitely_growable: false,
                },
                LayoutGridTrackSizingTrackState {
                    internal_index: 2,
                    is_gutter: true,
                    is_collapsed: true,
                    base_size: 0.0,
                    growth_limit: Some(0.0),
                    infinitely_growable: false,
                },
            ],
        })
        .collect();
    LayoutGridTrackSizingPassTrace {
        sequence: 1,
        axis: LayoutGridTrackSizingAxis::Rows,
        stages,
        contributions: vec![
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
        ],
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    }
}

#[test]
fn accepts_only_the_pinned_definite_height_percentage_grid_input() {
    let root = NodeId::new(1).unwrap();
    assert!(validate_grid_input(root, &expected_grid_input()).is_ok());

    let mut changed_percentage = expected_grid_input();
    changed_percentage.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.51),
    )]);
    assert!(validate_grid_input(root, &changed_percentage).is_err());

    let mut explicit_row = expected_grid_input();
    explicit_row.rows.push(GridTrackSize::Auto);
    assert!(validate_grid_input(root, &explicit_row).is_err());
}

#[test]
fn accepts_only_the_exact_percentage_row_stage_and_contribution_trace() {
    let item = NodeId::new(2).unwrap();
    let expected = expected_rows_pass();
    assert!(validate_rows_pass(&expected, item).is_ok());

    let mut wrong_stage_order = expected.clone();
    wrong_stage_order.stages.swap(2, 3);
    assert!(validate_rows_pass(&wrong_stage_order, item).is_err());

    let mut wrong_row_base = expected.clone();
    wrong_row_base.stages[3].tracks[1].base_size = 149.0;
    assert!(validate_rows_pass(&wrong_row_base, item).is_err());

    let mut wrong_min_content = expected.clone();
    wrong_min_content.contributions[1].value = 399.0;
    assert!(validate_rows_pass(&wrong_min_content, item).is_err());

    assert!(validate_rows_pass(&expected, NodeId::new(3).unwrap()).is_err());
}
