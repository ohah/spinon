use spinon_core::NodeId;
use spinon_layout::{
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
};

use super::RuntimeLayoutFailure;

pub(super) fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
    expected_row_size: f32,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_stages = [
        (LayoutGridTrackSizingStageKind::Initialize, 0.0),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            0.0,
        ),
        (LayoutGridTrackSizingStageKind::IntrinsicSizingComplete, 0.0),
        (
            LayoutGridTrackSizingStageKind::MaximizeTracks,
            expected_row_size,
        ),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            expected_row_size,
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            expected_row_size,
        ),
    ];
    if pass.sequence != 1 || pass.stages.len() != expected_stages.len() {
        return Err(super::failure(
            "row sizing stage count or sequence differs from the WPT slice",
        ));
    }
    let mut track_state_mismatches = Vec::new();
    for (stage, (expected_kind, expected_base)) in pass.stages.iter().zip(expected_stages) {
        if stage.stage != expected_kind || stage.tracks.len() != 3 {
            return Err(super::failure(
                "row sizing stage order or track structure differs",
            ));
        }
        let expected_growth_limits = if expected_kind == LayoutGridTrackSizingStageKind::Initialize
        {
            [Some(0.0), None, Some(0.0)]
        } else {
            [Some(0.0), Some(400.0), Some(0.0)]
        };
        for (index, ((track, expected_track_base), expected_growth_limit)) in stage
            .tracks
            .iter()
            .zip([0.0, expected_base, 0.0])
            .zip(expected_growth_limits)
            .enumerate()
        {
            let is_gutter = index != 1;
            if track.internal_index as usize != index
                || track.is_gutter != is_gutter
                || track.is_collapsed != is_gutter
                || (track.base_size - expected_track_base).abs() > 0.000001
                || track.growth_limit != expected_growth_limit
                || track.infinitely_growable
            {
                track_state_mismatches.push(format!(
                    "row sizing stage {:?} track {index}: actual={track:?}, expected_gutter={is_gutter}, expected_base={expected_track_base}, expected_growth_limit={expected_growth_limit:?}, expected_infinitely_growable=false",
                    stage.stage
                ));
            }
        }
    }
    if !track_state_mismatches.is_empty() {
        return Err(super::failure(&track_state_mismatches.join("; ")));
    }

    let expected_contributions = [
        (
            LayoutGridTrackSizingContributionKind::Minimum,
            0.0,
            Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum),
        ),
        (
            LayoutGridTrackSizingContributionKind::MinContent,
            400.0,
            None,
        ),
        (
            LayoutGridTrackSizingContributionKind::MaxContent,
            400.0,
            None,
        ),
    ];
    if pass.contributions.len() != expected_contributions.len()
        || pass.contributions.iter().zip(expected_contributions).any(
            |(actual, (kind, value, minimum_source))| {
                actual.item != item
                    || actual.span != 1
                    || actual.crosses_flexible_track
                    || actual.kind != kind
                    || (actual.value - value).abs() > 0.000001
                    || actual.minimum_source != minimum_source
            },
        )
    {
        return Err(super::failure(
            "row minimum/min-content/max-content contributions differ",
        ));
    }
    Ok(())
}
