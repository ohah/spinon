use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
};

use super::super::super::c11_4_nested_grid_replaced_item::unsupported;
use super::{CHILD_WIDTH, Case, EPSILON, ROOT_HEIGHT, RuntimeLayoutFailure, failure};

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let fit_limit = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(case.limit));
    if !input.columns.is_empty()
        || input.rows != [GridTrackSize::LengthPx(ROOT_HEIGHT)]
        || input.auto_columns != GridAutoColumns::TrackList(vec![fit_limit])
        || input.auto_rows != GridAutoRows::Auto
        || input.auto_flow != GridAutoFlow::Column
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || input
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || input.rows_line_names.len() != 2
        || input.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            root,
            case.scene_id,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_columns_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    if pass.axis != LayoutGridTrackSizingAxis::Columns
        || pass.sequence != 0
        || pass.stages.is_empty()
        || pass.fixture_phase.is_some()
        || pass.fixture_auto_repeat_count.is_some()
        || pass.auto_repeat_count.is_some()
        || pass.fixture_percentage_basis.is_some()
    {
        return Err(failure("column sizing stages are missing"));
    }
    let item_contributions = pass
        .contributions
        .iter()
        .filter(|contribution| contribution.item == item && contribution.span == 1)
        .collect::<Vec<_>>();
    if item_contributions.len() != 3 {
        return Err(failure(
            "expected one minimum, min-content, and max-content contribution",
        ));
    }
    let minimum = item_contributions
        .iter()
        .find(|contribution| contribution.kind == LayoutGridTrackSizingContributionKind::Minimum)
        .ok_or_else(|| failure("minimum contribution missing"))?;
    let min_content = item_contributions
        .iter()
        .find(|contribution| contribution.kind == LayoutGridTrackSizingContributionKind::MinContent)
        .ok_or_else(|| failure("min-content contribution missing"))?;
    let max_content = item_contributions
        .iter()
        .find(|contribution| contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent)
        .ok_or_else(|| failure("max-content contribution missing"))?;
    let minimum_matches = if case.automatic_minimum {
        (minimum.value - CHILD_WIDTH).abs() <= EPSILON
            && matches!(
                minimum.minimum_source.as_ref(),
                Some(
                    LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                        content_based: true,
                        ..
                    }
                )
            )
    } else {
        (minimum.value).abs() <= EPSILON
            && minimum.minimum_source
                == Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum)
    };
    if !minimum_matches
        || (min_content.value - CHILD_WIDTH).abs() > EPSILON
        || (max_content.value - CHILD_WIDTH).abs() > EPSILON
        || item_contributions
            .iter()
            .any(|contribution| contribution.crosses_flexible_track)
    {
        return Err(failure(
            "column minimum or intrinsic contributions differ from fixture",
        ));
    }
    let maximize = pass
        .stages
        .iter()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::MaximizeTracks)
        .ok_or_else(|| failure("MaximizeTracks stage is missing"))?;
    let tracks = maximize
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    if tracks.len() != 1 {
        return Err(failure("MaximizeTracks must contain one active column"));
    }
    let track = tracks[0];
    if track.internal_index != 1
        || (track.base_size - case.expected_column).abs() > EPSILON
        || track.growth_limit != Some(CHILD_WIDTH)
        || track.infinitely_growable
    {
        return Err(failure(&format!(
            "fit-content column sizing differs: limit={} expected_base={} actual_base={} expected_growth_limit={} actual_growth_limit={:?} infinitely_growable={}",
            case.limit,
            case.expected_column,
            track.base_size,
            CHILD_WIDTH,
            track.growth_limit,
            track.infinitely_growable,
        )));
    }
    Ok(())
}

pub(super) fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
) -> Result<(), RuntimeLayoutFailure> {
    if pass.axis != LayoutGridTrackSizingAxis::Rows
        || pass.sequence != 1
        || pass.stages.len() != 2
        || !pass.contributions.is_empty()
        || pass.stages[0].stage != LayoutGridTrackSizingStageKind::Initialize
        || pass.stages[1].stage != LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
    {
        return Err(failure(
            "explicit fixed row early-return trace differs from the fixture",
        ));
    }
    for stage in &pass.stages {
        let tracks = stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .collect::<Vec<_>>();
        if tracks.len() != 1
            || tracks[0].internal_index != 1
            || (tracks[0].base_size - ROOT_HEIGHT).abs() > EPSILON
            || tracks[0].growth_limit != Some(ROOT_HEIGHT)
            || tracks[0].infinitely_growable
        {
            return Err(failure(
                "explicit 60px row fixed track differs from the fixture",
            ));
        }
    }
    Ok(())
}
