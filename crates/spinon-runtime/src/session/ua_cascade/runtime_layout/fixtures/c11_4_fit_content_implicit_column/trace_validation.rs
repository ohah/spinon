use spinon_core::NodeId;
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind,
};

use super::RuntimeLayoutFailure;

const COLUMNS_BASES: [[f32; 2]; 6] = [
    [60.0, 0.0],
    [60.0, 0.0],
    [60.0, 0.0],
    [60.0, 40.0],
    [60.0, 40.0],
    [60.0, 40.0],
];
const ROWS_BASES: [[f32; 1]; 6] = [[0.0], [10.0], [10.0], [10.0], [10.0], [10.0]];
const COLUMNS_STAGES: [LayoutGridTrackSizingStageKind; 6] = [
    LayoutGridTrackSizingStageKind::Initialize,
    LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
        span: 1,
        crosses_flexible_track: false,
    },
    LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
    LayoutGridTrackSizingStageKind::MaximizeTracks,
    LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
    LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: false },
];
const ROWS_STAGES: [LayoutGridTrackSizingStageKind; 6] = [
    LayoutGridTrackSizingStageKind::Initialize,
    LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
        span: 1,
        crosses_flexible_track: false,
    },
    LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
    LayoutGridTrackSizingStageKind::MaximizeTracks,
    LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
    LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
];

pub(super) fn validate_columns_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
) -> Result<(), RuntimeLayoutFailure> {
    validate_track_bases(pass, LayoutGridTrackSizingAxis::Columns, 0, &COLUMNS_BASES)?;
    if pass.contributions.len() != 3 {
        return Err(super::failure(
            "column contribution count differs from WPT input",
        ));
    }
    let expected = [
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
    if pass
        .contributions
        .iter()
        .zip(expected)
        .any(|(actual, (kind, value, minimum_source))| {
            actual.item != item
                || actual.kind != kind
                || actual.span != 1
                || actual.crosses_flexible_track
                || (actual.value - value).abs() > 0.000001
                || actual.minimum_source != minimum_source
        })
    {
        return Err(super::failure(
            "column minimum/min-content/max-content contributions differ",
        ));
    }
    Ok(())
}

pub(super) fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
) -> Result<(), RuntimeLayoutFailure> {
    validate_track_bases(pass, LayoutGridTrackSizingAxis::Rows, 1, &ROWS_BASES)?;
    if pass.contributions.len() != 3 {
        return Err(super::failure(
            "row contribution count differs from WPT input",
        ));
    }
    let expected = [
        (
            LayoutGridTrackSizingContributionKind::Minimum,
            10.0,
            Some(
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
        ),
        (
            LayoutGridTrackSizingContributionKind::MinContent,
            10.0,
            None,
        ),
        (
            LayoutGridTrackSizingContributionKind::MaxContent,
            10.0,
            None,
        ),
    ];
    if pass
        .contributions
        .iter()
        .zip(expected)
        .any(|(actual, (kind, value, minimum_source))| {
            actual.item != item
                || actual.kind != kind
                || actual.span != 1
                || actual.crosses_flexible_track
                || (actual.value - value).abs() > 0.000001
                || actual.minimum_source != minimum_source
        })
    {
        return Err(super::failure(
            "row automatic-minimum/min-content/max-content contributions differ",
        ));
    }
    Ok(())
}

fn validate_track_bases<const TRACKS: usize>(
    pass: &LayoutGridTrackSizingPassTrace,
    axis: LayoutGridTrackSizingAxis,
    sequence: u16,
    expected_bases: &[[f32; TRACKS]; 6],
) -> Result<(), RuntimeLayoutFailure> {
    let expected_stages = match axis {
        LayoutGridTrackSizingAxis::Columns => &COLUMNS_STAGES,
        LayoutGridTrackSizingAxis::Rows => &ROWS_STAGES,
    };
    let internal_track_count = TRACKS * 2 + 1;
    if pass.sequence != sequence
        || pass.axis != axis
        || pass.stages.len() != expected_stages.len()
        || pass.fixture_phase.is_some()
        || pass.fixture_auto_repeat_count.is_some()
        || pass.auto_repeat_count.is_some()
        || pass.fixture_percentage_basis.is_some()
    {
        return Err(super::failure(
            "track sizing pass identity or stage count differs",
        ));
    }
    for (stage_index, ((stage, expected_stage), expected_bases)) in pass
        .stages
        .iter()
        .zip(expected_stages.iter())
        .zip(expected_bases.iter())
        .enumerate()
    {
        if &stage.stage != expected_stage || stage.tracks.len() != internal_track_count {
            return Err(super::failure(&format!(
                "track sizing stage or structure differs at axis {axis:?} stage {stage_index}: expected_stage={expected_stage:?} actual_stage={:?} expected_tracks={internal_track_count} actual_tracks={}",
                stage.stage,
                stage.tracks.len(),
            )));
        }
        let mut visible_tracks = 0;
        for (index, track) in stage.tracks.iter().enumerate() {
            let is_gutter = index % 2 == 0;
            let is_collapsed_boundary_gutter =
                is_gutter && (index == 0 || index + 1 == internal_track_count);
            if track.internal_index as usize != index
                || track.is_gutter != is_gutter
                || track.is_collapsed != is_collapsed_boundary_gutter
                || track.infinitely_growable
                || (is_gutter && track.base_size.abs() > 0.000001)
            {
                return Err(super::failure(&format!(
                    "track sizing stage gutter or track identity differs at axis {axis:?} stage {stage_index} slot {index}: internal_index={} gutter={} collapsed={} infinitely_growable={} base_size={}",
                    track.internal_index,
                    track.is_gutter,
                    track.is_collapsed,
                    track.infinitely_growable,
                    track.base_size,
                )));
            }
            if !is_gutter {
                if (track.base_size - expected_bases[visible_tracks]).abs() > 0.000001 {
                    return Err(super::failure(&format!(
                        "track sizing base differs at axis {axis:?} stage {stage_index} track {visible_tracks}: expected={} actual={}",
                        expected_bases[visible_tracks], track.base_size,
                    )));
                }
                visible_tracks += 1;
            }
        }
        if visible_tracks != TRACKS {
            return Err(super::failure(
                "track sizing stage visible track count differs",
            ));
        }
    }
    Ok(())
}
