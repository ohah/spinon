use super::*;

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
    stretch_applied: bool,
}

#[derive(Clone, Copy)]
enum ResidualRounding {
    Nearest,
    Up,
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
    let stages = repeated_text_row_track_stages(
        output,
        grid_id,
        raw_row_sizes.len(),
        grid_frame.height,
        container,
    )?;
    if stages.final_tracks.len() != raw_row_sizes.len()
        || stages.pre_alignment_tracks.len() != raw_row_sizes.len()
    {
        return None;
    }
    if stages
        .final_tracks
        .iter()
        .zip(raw_row_sizes)
        .any(|(track, size)| (track.base_size - size).abs() > 0.01)
    {
        return None;
    }

    let pre_alignment_sizes = stages
        .pre_alignment_tracks
        .iter()
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    let pre_alignment_units = blink_layout_unit_distribute_rows_from_stages(
        container,
        grid_frame,
        &pre_alignment_sizes,
        &stages.intrinsic_tracks,
        &stages.pre_alignment_tracks,
        match &container.auto_rows {
            crate::GridAutoRows::TrackList(tracks)
                if tracks
                    .iter()
                    .filter(|track| matches!(track, GridTrackSize::FitContent(_)))
                    .count()
                    == 2 =>
            {
                ResidualRounding::Up
            }
            _ => ResidualRounding::Nearest,
        },
    )?;
    let snapped_rows = if stages.stretch_applied {
        blink_layout_unit_stretch_auto_rows(container, grid_frame.height, pre_alignment_units)?
    } else {
        pre_alignment_units
    };
    if snapped_rows
        .iter()
        .zip(&stages.final_tracks)
        .any(|(units, track)| (blink_layout_unit_to_px(*units) - track.base_size).abs() > 0.02)
    {
        return None;
    }
    Some(snapped_rows)
}

pub(super) fn repeated_text_blink_layout_unit_distribute_rows(
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
) -> Option<Vec<i64>> {
    let stages = repeated_text_row_track_stages(
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
        &stages.intrinsic_tracks,
        &stages.final_tracks,
        ResidualRounding::Nearest,
    )
}

fn blink_layout_unit_distribute_rows_from_stages(
    container: &crate::GridContainerInput,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
    intrinsic_tracks: &[crate::LayoutGridTrackSizingTrackState],
    final_tracks: &[crate::LayoutGridTrackSizingTrackState],
    residual_rounding: ResidualRounding,
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
        let is_auto_track = if row < container.rows.len() {
            false
        } else {
            let crate::GridAutoRows::TrackList(tracks) = &container.auto_rows else {
                return None;
            };
            !tracks.is_empty()
                && matches!(
                    tracks[(row - container.rows.len()) % tracks.len()],
                    GridTrackSize::Auto
                )
        };
        if growth_potential >= 1.0 / 64.0 {
            growth_potentials.push((row, growth_potential, is_auto_track));
        } else if matches!(residual_rounding, ResidualRounding::Nearest) {
            return None;
        } else if fit_content_limit.is_none() {
            growth_potentials.push((row, growth_potential, is_auto_track));
        }
    }

    // Blink distributes intrinsic contributions in 1/64px LayoutUnits at each track-sizing
    // step. A fractional share can leave one unit on a later unconstrained track even when
    // rounding the final f32 sum to nearest would discard it.
    let target_total = match residual_rounding {
        ResidualRounding::Nearest => (raw_total * 64.0).round() as i64,
        ResidualRounding::Up => (raw_total * 64.0).ceil() as i64,
    };
    let leftover_units = target_total - floor_total;
    if leftover_units < 0 || leftover_units as usize > growth_potentials.len() {
        return None;
    }
    // Blink는 growth potential 순서로 1/64px 잔여분을 나눈다. Taffy는 같은 크기를
    // f32로 유지하므로 여기서 잔여 단위를 배분한다.
    growth_potentials.sort_by(|left, right| {
        left.1
            .total_cmp(&right.1)
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.0.cmp(&right.0))
    });
    for (row, _, _) in growth_potentials.iter().rev().take(leftover_units as usize) {
        quantized[*row] += 1;
    }
    Some(quantized)
}

fn blink_layout_unit_stretch_auto_rows(
    container: &crate::GridContainerInput,
    grid_height: f32,
    mut row_size_units: Vec<i64>,
) -> Option<Vec<i64>> {
    let crate::GridAutoRows::TrackList(auto_rows) = &container.auto_rows else {
        return None;
    };
    if auto_rows.is_empty() || !grid_height.is_finite() || grid_height < 0.0 {
        return None;
    }
    let auto_rows = (container.rows.len()..row_size_units.len())
        .filter(|row| {
            matches!(
                auto_rows[(*row - container.rows.len()) % auto_rows.len()],
                GridTrackSize::Auto
            )
        })
        .collect::<Vec<_>>();
    if auto_rows.is_empty() {
        return Some(row_size_units);
    }
    let grid_height_units = blink_layout_unit_floor_units(grid_height);
    let used_units = row_size_units.iter().sum::<i64>();
    let free_units = (grid_height_units - used_units).max(0);
    let share = free_units / auto_rows.len() as i64;
    let remainder = (free_units % auto_rows.len() as i64) as usize;
    for row in &auto_rows {
        row_size_units[*row] += share;
    }
    for row in auto_rows.iter().rev().take(remainder) {
        row_size_units[*row] += 1;
    }
    Some(row_size_units)
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

#[cfg(test)]
#[path = "track_trace/tests.rs"]
mod tests;
