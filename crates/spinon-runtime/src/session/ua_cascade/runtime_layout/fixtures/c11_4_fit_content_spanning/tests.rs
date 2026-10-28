use super::{CHILD_HEIGHT, FIT_LIMIT, WIDTH, validate_grid_input, validate_rows_pass};
use spinon_core::NodeId;
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingContributionTrace, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
    LayoutGridTrackSizingTrackState,
};

fn expected_input() -> GridContainerInput {
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(WIDTH)],
        columns_line_names: vec![vec![], vec![]],
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(FIT_LIMIT),
        )]),
        auto_flow: GridAutoFlow::Row,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

#[test]
fn accepts_only_the_pinned_fit_content_span_grid_input() {
    let root = NodeId::new(1).unwrap();
    assert!(validate_grid_input(root, &expected_input()).is_ok());

    let mut changed_limit = expected_input();
    changed_limit.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(FIT_LIMIT + 1.0),
    )]);
    assert!(validate_grid_input(root, &changed_limit).is_err());

    let mut explicit_row = expected_input();
    explicit_row.rows.push(GridTrackSize::Auto);
    assert!(validate_grid_input(root, &explicit_row).is_err());

    let mut changed_columns = expected_input();
    changed_columns.columns[0] = GridTrackSize::LengthPx(WIDTH + 1.0);
    assert!(validate_grid_input(root, &changed_columns).is_err());
}

#[test]
fn validates_a_span_two_contribution_and_two_capped_tracks() {
    let item = NodeId::new(2).unwrap();
    let zero = vec![
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
            base_size: 0.0,
            growth_limit: None,
            infinitely_growable: false,
        },
        LayoutGridTrackSizingTrackState {
            internal_index: 2,
            is_gutter: true,
            is_collapsed: false,
            base_size: 0.0,
            growth_limit: Some(0.0),
            infinitely_growable: false,
        },
        LayoutGridTrackSizingTrackState {
            internal_index: 3,
            is_gutter: false,
            is_collapsed: false,
            base_size: 0.0,
            growth_limit: None,
            infinitely_growable: false,
        },
        LayoutGridTrackSizingTrackState {
            internal_index: 4,
            is_gutter: true,
            is_collapsed: true,
            base_size: 0.0,
            growth_limit: Some(0.0),
            infinitely_growable: false,
        },
    ];
    let mut intrinsic = zero.clone();
    intrinsic[1].growth_limit = Some(FIT_LIMIT);
    intrinsic[3].growth_limit = Some(FIT_LIMIT);
    let final_tracks = [0.0, FIT_LIMIT, 0.0, FIT_LIMIT, 0.0]
        .into_iter()
        .enumerate()
        .map(|(index, base_size)| LayoutGridTrackSizingTrackState {
            internal_index: index as u16,
            is_gutter: index % 2 == 0,
            is_collapsed: index == 0 || index == 4,
            base_size,
            growth_limit: Some(if index % 2 == 0 { 0.0 } else { FIT_LIMIT }),
            infinitely_growable: false,
        })
        .collect::<Vec<_>>();
    let stages = vec![
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::Initialize,
            tracks: zero.clone(),
        },
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 2,
                crosses_flexible_track: false,
            },
            tracks: intrinsic.clone(),
        },
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            tracks: intrinsic,
        },
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::MaximizeTracks,
            tracks: final_tracks.clone(),
        },
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            tracks: final_tracks.clone(),
        },
        LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            tracks: final_tracks,
        },
    ];
    let pass = LayoutGridTrackSizingPassTrace {
        sequence: 1,
        axis: LayoutGridTrackSizingAxis::Rows,
        stages,
        contributions: vec![LayoutGridTrackSizingContributionTrace {
            item,
            kind: LayoutGridTrackSizingContributionKind::MaxContent,
            span: 2,
            crosses_flexible_track: false,
            value: CHILD_HEIGHT,
            minimum_source: None,
        }],
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    };
    assert!(validate_rows_pass(&pass, item).is_ok());

    let mut changed_contribution = pass.clone();
    changed_contribution.contributions[0].span = 1;
    assert!(validate_rows_pass(&changed_contribution, item).is_err());

    let mut changed_growth_limit = pass.clone();
    changed_growth_limit.stages[1].tracks[1].growth_limit = Some(FIT_LIMIT + 1.0);
    assert!(validate_rows_pass(&changed_growth_limit, item).is_err());

    let mut infinitely_growable = pass.clone();
    infinitely_growable.stages[1].tracks[1].infinitely_growable = true;
    assert!(validate_rows_pass(&infinitely_growable, item).is_err());
}
