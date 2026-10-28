use super::validate_grid_input;
use spinon_core::NodeId;
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingContributionTrace, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
    LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrackState,
};

use super::{trace_validation::validate_rows_pass, validate_layout_math};

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

fn expected_calc_grid_input() -> GridContainerInput {
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(60.0)],
        columns_line_names: vec![vec![], vec![]],
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Calc(spinon_layout::LayoutCalcId(7)),
        )]),
        auto_flow: GridAutoFlow::Row,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

fn expected_rows_pass(row_size: f32) -> LayoutGridTrackSizingPassTrace {
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
        (LayoutGridTrackSizingStageKind::MaximizeTracks, row_size),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            row_size,
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            row_size,
        ),
    ];
    let stages = stage_bases
        .into_iter()
        .map(|(stage, base)| {
            let row_growth_limit = if stage == LayoutGridTrackSizingStageKind::Initialize {
                None
            } else {
                Some(400.0)
            };
            LayoutGridTrackSizingStageTrace {
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
                        growth_limit: row_growth_limit,
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
            }
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
    assert!(validate_grid_input(root, &expected_grid_input(), &super::PERCENTAGE_SCENE).is_ok());

    let mut changed_percentage = expected_grid_input();
    changed_percentage.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::Percentage(0.51),
    )]);
    assert!(validate_grid_input(root, &changed_percentage, &super::PERCENTAGE_SCENE).is_err());

    let mut explicit_row = expected_grid_input();
    explicit_row.rows.push(GridTrackSize::Auto);
    assert!(validate_grid_input(root, &explicit_row, &super::PERCENTAGE_SCENE).is_err());
}

#[test]
fn accepts_only_the_exact_percentage_row_stage_and_contribution_trace() {
    let item = NodeId::new(2).unwrap();
    let expected = expected_rows_pass(150.0);
    assert!(validate_rows_pass(&expected, item, 150.0).is_ok());

    let mut wrong_stage_order = expected.clone();
    wrong_stage_order.stages.swap(2, 3);
    assert!(validate_rows_pass(&wrong_stage_order, item, 150.0).is_err());

    let mut wrong_row_base = expected.clone();
    wrong_row_base.stages[3].tracks[1].base_size = 149.0;
    assert!(validate_rows_pass(&wrong_row_base, item, 150.0).is_err());

    let mut wrong_growth_limit = expected.clone();
    wrong_growth_limit.stages[3].tracks[1].growth_limit = Some(401.0);
    assert!(validate_rows_pass(&wrong_growth_limit, item, 150.0).is_err());

    let mut initial_growth_limit_resolved_too_early = expected.clone();
    initial_growth_limit_resolved_too_early.stages[0].tracks[1].growth_limit = Some(400.0);
    assert!(validate_rows_pass(&initial_growth_limit_resolved_too_early, item, 150.0).is_err());

    let mut unexpectedly_growable = expected.clone();
    unexpectedly_growable.stages[3].tracks[1].infinitely_growable = true;
    assert!(validate_rows_pass(&unexpectedly_growable, item, 150.0).is_err());

    let mut wrong_min_content = expected.clone();
    wrong_min_content.contributions[1].value = 399.0;
    assert!(validate_rows_pass(&wrong_min_content, item, 150.0).is_err());

    assert!(validate_rows_pass(&expected, NodeId::new(3).unwrap(), 150.0).is_err());
}

#[test]
fn accepts_only_the_calc_fit_content_row_input_and_35px_trace() {
    let root = NodeId::new(1).unwrap();
    let item = NodeId::new(2).unwrap();
    let calc_input = expected_calc_grid_input();
    assert!(validate_grid_input(root, &calc_input, &super::CALC_SCENE).is_ok());

    let mut percentage_input = calc_input.clone();
    percentage_input.auto_rows = expected_grid_input().auto_rows;
    assert!(validate_grid_input(root, &percentage_input, &super::CALC_SCENE).is_err());

    let mut extra_track = calc_input;
    extra_track.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(
            spinon_layout::LayoutCalcId(7),
        )),
        GridTrackSize::Auto,
    ]);
    assert!(validate_grid_input(root, &extra_track, &super::CALC_SCENE).is_err());

    let expected = expected_rows_pass(35.0);
    assert!(validate_rows_pass(&expected, item, 35.0).is_ok());

    let mut wrong_final_base = expected.clone();
    wrong_final_base.stages[3].tracks[1].base_size = 36.0;
    assert!(validate_rows_pass(&wrong_final_base, item, 35.0).is_err());
}

#[test]
fn binds_calc_handle_to_the_exact_row_owner_property_and_expression() {
    let root = NodeId::new(1).unwrap();
    let input = expected_calc_grid_input();
    let expected = LayoutCssMathValue {
        id: spinon_layout::LayoutCalcId(7),
        node_id: root,
        property: LayoutCssMathProperty::GridTemplateRows,
        expression: LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(5.0),
        ]),
    };
    assert!(
        validate_layout_math(
            root,
            &input,
            std::slice::from_ref(&expected),
            &super::CALC_SCENE,
        )
        .is_ok()
    );

    let mut stale_handle = expected.clone();
    stale_handle.id = spinon_layout::LayoutCalcId(8);
    assert!(
        validate_layout_math(
            root,
            &input,
            std::slice::from_ref(&stale_handle),
            &super::CALC_SCENE,
        )
        .is_err()
    );

    let mut wrong_owner = expected.clone();
    wrong_owner.node_id = NodeId::new(2).unwrap();
    assert!(
        validate_layout_math(
            root,
            &input,
            std::slice::from_ref(&wrong_owner),
            &super::CALC_SCENE,
        )
        .is_err()
    );

    let mut wrong_property = expected.clone();
    wrong_property.property = LayoutCssMathProperty::GridTemplateColumns;
    assert!(
        validate_layout_math(
            root,
            &input,
            std::slice::from_ref(&wrong_property),
            &super::CALC_SCENE,
        )
        .is_err()
    );

    let mut wrong_expression = expected.clone();
    wrong_expression.expression = LayoutCssMath::Sum(vec![
        LayoutCssMath::Percentage(0.1),
        LayoutCssMath::LengthPx(4.0),
    ]);
    assert!(
        validate_layout_math(
            root,
            &input,
            std::slice::from_ref(&wrong_expression),
            &super::CALC_SCENE,
        )
        .is_err()
    );

    let extra_entry = [expected.clone(), expected];
    assert!(validate_layout_math(root, &input, &extra_entry, &super::CALC_SCENE).is_err());
    assert!(
        validate_layout_math(root, &expected_grid_input(), &[], &super::PERCENTAGE_SCENE,).is_ok()
    );
}
