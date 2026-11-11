use super::*;

#[path = "track_trace/rounding.rs"]
mod rounding;
#[cfg(test)]
use rounding::{
    ResidualRounding, blink_layout_unit_distribute_rows_from_stages,
    blink_layout_unit_stretch_auto_rows,
};
pub(super) use rounding::{
    blink_layout_unit_distribute_rows, blink_layout_unit_to_px,
    repeated_text_blink_layout_unit_distribute_rows,
};

pub(super) fn repeated_text_track_trace_error(grid_id: NodeId) -> LayoutError {
    LayoutError::UnsupportedGridContainerStyle {
        node: grid_id,
        property: "C11.4 repeated text track trace",
    }
}

struct RepeatedTextRowTrackStages {
    intrinsic_tracks: Vec<crate::LayoutGridTrackSizingTrackState>,
    pre_alignment_tracks: Vec<crate::LayoutGridTrackSizingTrackState>,
    final_tracks: Vec<crate::LayoutGridTrackSizingTrackState>,
    has_span_auto_minimum: bool,
    has_spanning_item: bool,
    stretch_applied: bool,
}

pub(super) fn repeated_text_final_row_track_sizes(
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    expected_track_count: usize,
    grid_height: f32,
    container: &crate::GridContainerInput,
) -> Option<Vec<f32>> {
    let stages = repeated_text_row_track_stages(
        output,
        grid_id,
        expected_track_count,
        grid_height,
        container,
    )?;
    Some(
        stages
            .final_tracks
            .into_iter()
            .map(|track| track.base_size)
            .collect(),
    )
}

fn repeated_text_row_track_stages(
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    expected_track_count: usize,
    grid_height: f32,
    container: &crate::GridContainerInput,
) -> Option<RepeatedTextRowTrackStages> {
    let mut traces = output
        .grid_track_sizing_traces
        .iter()
        .filter(|trace| trace.node == grid_id);
    let trace = traces.next()?;
    let resolved_track_count = usize::from(trace.used_rows.explicit_track_count)
        + usize::from(trace.used_rows.negative_implicit_track_count)
        + usize::from(trace.used_rows.positive_implicit_track_count);
    if traces.next().is_some()
        || usize::from(trace.used_rows.negative_implicit_track_count) != 0
        || resolved_track_count != expected_track_count
        || trace.used_rows.used_track_list.split_whitespace().count() != expected_track_count
        || !trace.used_rows.collapsed_track_indices.is_empty()
    {
        return None;
    }
    let passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    let final_sequence = passes.iter().map(|pass| pass.sequence).max()?;
    let mut final_passes = passes
        .into_iter()
        .filter(|pass| pass.sequence == final_sequence);
    let pass = final_passes.next()?;
    if final_passes.next().is_some() {
        return None;
    }
    let mut intrinsic_stages = pass.stages.iter().filter(|stage| {
        stage.stage == crate::LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
    });
    let intrinsic_stage = intrinsic_stages.next()?;
    if intrinsic_stages.next().is_some() {
        return None;
    }
    let mut pre_alignment_stages = pass
        .stages
        .iter()
        .filter(|stage| stage.stage == crate::LayoutGridTrackSizingStageKind::MaximizeTracks);
    let pre_alignment_stage = pre_alignment_stages.next()?;
    if pre_alignment_stages.next().is_some() {
        return None;
    }
    let final_stage = pass.stages.last()?;
    let has_span_auto_minimum = pass.contributions.iter().any(|contribution| {
        contribution.kind == crate::LayoutGridTrackSizingContributionKind::Minimum
            && matches!(
                contribution.minimum_source,
                Some(
                    crate::LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                        spans_auto_min_track: true,
                        spans_multiple_tracks: true,
                        spans_flexible_max_track: false,
                        content_based: true,
                        ..
                    }
                )
            )
    });
    let has_spanning_item = pass
        .contributions
        .iter()
        .any(|contribution| contribution.span > 1);
    let stretched_auto_tracks = match final_stage.stage {
        crate::LayoutGridTrackSizingStageKind::StretchAutoTracks { applied } => applied,
        _ => return None,
    };
    let GridAutoRows::TrackList(auto_rows) = &container.auto_rows else {
        return None;
    };
    if auto_rows.is_empty() {
        return None;
    }
    let validate_tracks = |tracks: &[crate::LayoutGridTrackSizingTrackState],
                           allow_stretched_auto: bool| {
        let visible_tracks = tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .collect::<Vec<_>>();
        if visible_tracks.len() != expected_track_count
            || visible_tracks.iter().enumerate().any(|(index, track)| {
                usize::from(track.internal_index) != index * 2 + 1
                    || track.is_collapsed
                    || !track.base_size.is_finite()
                    || track.base_size < 0.0
                    || track.growth_limit.is_some_and(|limit| {
                        !limit.is_finite()
                            || limit < 0.0
                            || (limit + 0.01 < track.base_size
                                && !(allow_stretched_auto
                                    && index > 0
                                    && matches!(
                                        auto_rows[(index - 1) % auto_rows.len()],
                                        GridTrackSize::Auto
                                    )))
                    })
            })
        {
            return None;
        }
        Some(visible_tracks.into_iter().cloned().collect::<Vec<_>>())
    };
    let intrinsic_tracks = validate_tracks(&intrinsic_stage.tracks, false)?;
    let pre_alignment_tracks = validate_tracks(&pre_alignment_stage.tracks, false)?;
    let final_tracks = validate_tracks(&final_stage.tracks, stretched_auto_tracks)?;
    let resolved_row_sizes = trace
        .used_rows
        .used_track_list
        .split_whitespace()
        .map(|value| {
            value
                .strip_suffix("px")
                .unwrap_or(value)
                .parse::<f32>()
                .ok()
        })
        .collect::<Option<Vec<_>>>()?;
    if resolved_row_sizes.len() != final_tracks.len()
        || resolved_row_sizes
            .iter()
            .zip(&final_tracks)
            .any(|(resolved, track)| {
                !resolved.is_finite() || (resolved - track.base_size).abs() > 0.01
            })
    {
        return None;
    }
    if !validate_auto_track_alignment(
        &pre_alignment_tracks,
        &final_tracks,
        auto_rows,
        stretched_auto_tracks,
        grid_height,
    ) {
        return None;
    }
    Some(RepeatedTextRowTrackStages {
        intrinsic_tracks,
        pre_alignment_tracks,
        final_tracks,
        has_span_auto_minimum,
        has_spanning_item,
        stretch_applied: stretched_auto_tracks,
    })
}

fn validate_auto_track_alignment(
    pre_alignment: &[crate::LayoutGridTrackSizingTrackState],
    final_tracks: &[crate::LayoutGridTrackSizingTrackState],
    auto_rows: &[GridTrackSize],
    stretch_applied: bool,
    grid_height: f32,
) -> bool {
    if pre_alignment.len() != final_tracks.len()
        || auto_rows.is_empty()
        || !grid_height.is_finite()
        || grid_height < 0.0
    {
        return false;
    }
    let auto_track_count = (1..pre_alignment.len())
        .filter(|row| matches!(auto_rows[(row - 1) % auto_rows.len()], GridTrackSize::Auto))
        .count();
    let expected_auto_stretch = if stretch_applied && auto_track_count > 0 {
        let used = pre_alignment
            .iter()
            .map(|track| f64::from(track.base_size))
            .sum::<f64>();
        ((f64::from(grid_height) - used).max(0.0) / auto_track_count as f64) as f32
    } else {
        0.0
    };
    for (row, (before, after)) in pre_alignment.iter().zip(final_tracks).enumerate() {
        let delta = after.base_size - before.base_size;
        let is_auto =
            row > 0 && matches!(auto_rows[(row - 1) % auto_rows.len()], GridTrackSize::Auto);
        if !delta.is_finite() || delta < -0.01 {
            return false;
        }
        if is_auto {
            if (delta - expected_auto_stretch).abs() > 0.01 {
                return false;
            }
        } else if delta.abs() > 0.01 {
            return false;
        }
    }
    // The stretch stage can run even when this finite used-row prefix contains no `auto`
    // track. In that case the loop above requires every row to remain unchanged.
    true
}

#[cfg(test)]
#[path = "track_trace/tests.rs"]
mod tests;
