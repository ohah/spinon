use super::*;

pub(super) fn repeated_text_track_trace_error(grid_id: NodeId) -> LayoutError {
    LayoutError::UnsupportedGridContainerStyle {
        node: grid_id,
        property: "C11.4 repeated text track trace",
    }
}

pub(super) fn repeated_text_final_row_track_sizes(
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    expected_track_count: usize,
    grid_height: f32,
    container: &crate::GridContainerInput,
) -> Option<Vec<f32>> {
    let (_, final_tracks) = repeated_text_row_track_stages(
        output,
        grid_id,
        expected_track_count,
        grid_height,
        container,
    )?;
    Some(
        final_tracks
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
) -> Option<(
    Vec<crate::LayoutGridTrackSizingTrackState>,
    Vec<crate::LayoutGridTrackSizingTrackState>,
)> {
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
    Some((intrinsic_tracks, final_tracks))
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
    !stretch_applied || auto_track_count > 0
}

fn blink_layout_unit_floor_units(value: f32) -> i64 {
    let value = f64::from(value);
    let rounding_error_bound = f64::from(f32::EPSILON) * value.abs().max(1.0) * 4.0 * 64.0;
    (value * 64.0 + rounding_error_bound).floor() as i64
}

pub(super) fn blink_layout_unit_to_px(units: i64) -> f32 {
    (units as f64 / 64.0) as f32
}

pub(super) fn blink_layout_unit_distribute_rows(
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
) -> Option<Vec<i64>> {
    // direct-text auto-placement 정규화 이전의 row-span trace 계약을 유지한다.
    // row-span의 Taffy pass는 별도의 stage 구성을 사용한다.
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid_id)?;
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)?;
    let intrinsic_stage = pass.stages.iter().find(|stage| {
        stage.stage == crate::LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
    })?;
    let final_stage = pass.stages.last()?;
    let intrinsic_tracks = intrinsic_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .cloned()
        .collect::<Vec<_>>();
    let final_tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .cloned()
        .collect::<Vec<_>>();
    if intrinsic_tracks.len() != raw_row_sizes.len() || final_tracks.len() != raw_row_sizes.len() {
        return None;
    }
    blink_layout_unit_distribute_rows_from_stages(
        container,
        grid_frame,
        raw_row_sizes,
        &intrinsic_tracks,
        &final_tracks,
    )
}

pub(super) fn repeated_text_blink_layout_unit_distribute_rows(
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
) -> Option<Vec<i64>> {
    let (intrinsic_tracks, final_tracks) = repeated_text_row_track_stages(
        output,
        grid_id,
        raw_row_sizes.len(),
        grid_frame.height,
        container,
    )?;
    blink_layout_unit_distribute_rows_from_stages(
        container,
        grid_frame,
        raw_row_sizes,
        &intrinsic_tracks,
        &final_tracks,
    )
}

fn blink_layout_unit_distribute_rows_from_stages(
    container: &crate::GridContainerInput,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
    intrinsic_tracks: &[crate::LayoutGridTrackSizingTrackState],
    final_tracks: &[crate::LayoutGridTrackSizingTrackState],
) -> Option<Vec<i64>> {
    let mut quantized = Vec::with_capacity(raw_row_sizes.len());
    let mut growth_potentials = Vec::new();
    let mut raw_total = 0.0_f64;
    let mut floor_total = 0_i64;
    for (row, &raw_size) in raw_row_sizes.iter().enumerate() {
        if !raw_size.is_finite() || raw_size < 0.0 {
            return None;
        }
        if (final_tracks[row].base_size - raw_size).abs() > 0.01 {
            return None;
        }

        let scaled = f64::from(raw_size) * 64.0;
        let floor_units = blink_layout_unit_floor_units(raw_size);
        let nearest_units = scaled.round();
        let rounding_error_bound =
            f64::from(f32::EPSILON) * f64::from(raw_size).abs().max(1.0) * 4.0 * 64.0;
        let is_fractional = (scaled - nearest_units).abs() > rounding_error_bound;
        quantized.push(floor_units);
        floor_total += floor_units;
        raw_total += f64::from(raw_size);

        if !is_fractional {
            continue;
        }
        let track = &intrinsic_tracks[row];
        let growth_limit = track.growth_limit.unwrap_or(f32::INFINITY);
        let fit_content_limit = repeated_row_fit_content_limit(container, row, grid_frame.height)?;
        let growth_limit = fit_content_limit.map_or(growth_limit, |limit| growth_limit.min(limit));
        let growth_potential = (growth_limit - track.base_size).max(0.0);
        if growth_potential < 1.0 / 64.0 {
            return None;
        }
        growth_potentials.push((row, growth_potential));
    }

    let target_total = (raw_total * 64.0).round() as i64;
    let leftover_units = target_total - floor_total;
    if leftover_units < 0 || leftover_units as usize > growth_potentials.len() {
        return None;
    }
    // Blink는 growth potential 순서로 1/64px 잔여분을 나눈다. Taffy는 같은 크기를
    // f32로 유지하므로 여기서 잔여 단위를 배분한다.
    growth_potentials.sort_by(|left, right| left.1.total_cmp(&right.1));
    for (row, _) in growth_potentials.iter().rev().take(leftover_units as usize) {
        quantized[*row] += 1;
    }
    Some(quantized)
}

fn repeated_row_fit_content_limit(
    container: &crate::GridContainerInput,
    row: usize,
    grid_height: f32,
) -> Option<Option<f32>> {
    let track = if row < container.rows.len() {
        container.rows.get(row)?
    } else {
        let crate::GridAutoRows::TrackList(tracks) = &container.auto_rows else {
            return None;
        };
        if tracks.is_empty() {
            return None;
        }
        &tracks[(row - container.rows.len()) % tracks.len()]
    };
    let crate::GridTrackSize::FitContent(limit) = track else {
        return Some(None);
    };
    let limit = match limit {
        crate::GridTrackLengthPercentage::LengthPx(value) => *value,
        crate::GridTrackLengthPercentage::Percentage(value) => *value * grid_height,
        crate::GridTrackLengthPercentage::Calc(_) => return None,
    };
    Some(Some(limit))
}
