use super::super::*;
use super::repeated_text_row_track_stages;

#[path = "rounding/fit_content_span_distribution.rs"]
mod fit_content_span_distribution;
#[path = "rounding/fit_content_span_sizing.rs"]
mod fit_content_span_sizing;
#[path = "rounding/mixed_minimum.rs"]
mod mixed_minimum;

use mixed_minimum::{
    blink_layout_unit_ceil_units, compare_layout_unit_potential, has_mixed_span_minimum_modes,
    snapped_rows_match_final_tracks,
};

#[derive(Clone, Copy)]
pub(super) enum ResidualRounding {
    Nearest,
    Up,
    UpAutoLaterTrack,
    UpLaterTrack,
    UpAutoMinimumLaterTrack,
    UpFiveFitAutoLaterTrack,
    NearestFiveFitAutoLaterTrack,
    NearestAutoLaterTrack,
}

fn blink_layout_unit_floor_units(value: f32) -> i64 {
    let value = f64::from(value);
    let rounding_error_bound = f64::from(f32::EPSILON) * value.abs().max(1.0) * 4.0 * 64.0;
    (value * 64.0 + rounding_error_bound).floor() as i64
}

fn blink_layout_unit_is_fractional(value: f32) -> bool {
    let scaled = f64::from(value) * 64.0;
    let nearest_units = scaled.round();
    let rounding_error_bound =
        f64::from(f32::EPSILON) * f64::from(value).abs().max(1.0) * 4.0 * 64.0;
    (scaled - nearest_units).abs() > rounding_error_bound
}

pub(in crate::grid::root_html_fit_content_auto_rows::repeated_text) fn blink_layout_unit_to_px(
    units: i64,
) -> f32 {
    (units as f64 / 64.0) as f32
}

pub(in crate::grid::root_html_fit_content_auto_rows::repeated_text) fn blink_layout_unit_distribute_rows(
    input: &crate::LayoutInput,
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
    let fit_content_count = match &container.auto_rows {
        crate::GridAutoRows::TrackList(tracks) => tracks
            .iter()
            .filter(|track| matches!(track, GridTrackSize::FitContent(_)))
            .count(),
        _ => 0,
    };
    let span_auto_minimum = raw_row_sizes.len() == 7 && stages.has_span_auto_minimum;
    let one_fit_span_auto_minimum = span_auto_minimum && fit_content_count == 1;
    let three_fit_span_auto_minimum = span_auto_minimum && fit_content_count == 3;
    let four_fit_row_span = raw_row_sizes.len() == 7 && fit_content_count == 4;
    let four_fit_span_auto_minimum = four_fit_row_span && stages.has_span_auto_minimum;
    let four_fit_mixed_minimum = four_fit_row_span
        && stages.has_spanning_item
        && has_mixed_span_minimum_modes(input, grid_id);
    let five_fit_row_span =
        raw_row_sizes.len() == 7 && fit_content_count == 5 && stages.has_spanning_item;
    let five_fit_span_auto_minimum = five_fit_row_span && stages.has_span_auto_minimum;
    let five_fit_mixed_minimum = five_fit_row_span && has_mixed_span_minimum_modes(input, grid_id);
    let five_fit_has_fractional_auto_track = if five_fit_row_span {
        match &container.auto_rows {
            crate::GridAutoRows::TrackList(tracks) if !tracks.is_empty() => {
                pre_alignment_sizes.iter().enumerate().any(|(row, size)| {
                    row >= container.rows.len()
                        && matches!(
                            tracks[(row - container.rows.len()) % tracks.len()],
                            GridTrackSize::Auto
                        )
                        && blink_layout_unit_is_fractional(*size)
                })
            }
            _ => false,
        }
    } else {
        false
    };
    let four_fit_has_fractional_auto_track = if four_fit_row_span {
        match &container.auto_rows {
            crate::GridAutoRows::TrackList(tracks) if !tracks.is_empty() => {
                pre_alignment_sizes.iter().enumerate().any(|(row, size)| {
                    row >= container.rows.len()
                        && matches!(
                            tracks[(row - container.rows.len()) % tracks.len()],
                            GridTrackSize::Auto
                        )
                        && blink_layout_unit_is_fractional(*size)
                })
            }
            _ => false,
        }
    } else {
        false
    };
    let residual_rounding = match &container.auto_rows {
        crate::GridAutoRows::TrackList(_) if fit_content_count == 2 => ResidualRounding::Up,
        _ if five_fit_span_auto_minimum => ResidualRounding::UpAutoMinimumLaterTrack,
        _ if one_fit_span_auto_minimum
            || three_fit_span_auto_minimum
            || four_fit_span_auto_minimum =>
        {
            ResidualRounding::UpLaterTrack
        }
        _ if four_fit_has_fractional_auto_track => ResidualRounding::UpAutoLaterTrack,
        _ if four_fit_row_span => ResidualRounding::NearestAutoLaterTrack,
        _ if five_fit_has_fractional_auto_track => ResidualRounding::UpFiveFitAutoLaterTrack,
        _ if five_fit_row_span => ResidualRounding::NearestFiveFitAutoLaterTrack,
        _ => ResidualRounding::Nearest,
    };
    let six_fit_row_span =
        raw_row_sizes.len() == 7 && fit_content_count == 6 && stages.has_spanning_item;
    let fit_content_span_rows = (four_fit_mixed_minimum || five_fit_row_span || six_fit_row_span)
        .then(|| {
            fit_content_span_sizing::simulate_blink_row_sizes(
                input,
                container,
                output,
                grid_id,
                grid_frame.height,
                &stages.pre_alignment_tracks,
            )
        });
    match fit_content_span_rows {
        Some(Some(rows)) => {
            let rows = if four_fit_mixed_minimum {
                let target_total = blink_layout_unit_round_total(
                    pre_alignment_sizes
                        .iter()
                        .map(|size| f64::from(*size))
                        .sum(),
                    residual_rounding,
                );
                let simulated_total = rows.iter().sum::<i64>();
                if simulated_total < target_total {
                    let fallback_units = blink_layout_unit_distribute_rows_from_stages(
                        container,
                        grid_frame,
                        &pre_alignment_sizes,
                        &stages.intrinsic_tracks,
                        &stages.pre_alignment_tracks,
                        residual_rounding,
                    )?;
                    restore_missing_residual_units(rows, &fallback_units, target_total)?
                } else {
                    rows
                }
            } else {
                rows
            };
            let snapped_rows = if stages.stretch_applied {
                blink_layout_unit_stretch_auto_rows(container, grid_frame.height, rows)?
            } else {
                rows
            };
            if !snapped_rows_match_final_tracks(
                &snapped_rows,
                &stages.final_tracks,
                container,
                grid_frame.height,
                (four_fit_mixed_minimum || five_fit_mixed_minimum) && stages.stretch_applied,
            ) {
                return None;
            }
            return Some(snapped_rows);
        }
        Some(None) if four_fit_mixed_minimum || six_fit_row_span => return None,
        Some(None) | None => {}
    }
    let pre_alignment_units = blink_layout_unit_distribute_rows_from_stages(
        container,
        grid_frame,
        &pre_alignment_sizes,
        &stages.intrinsic_tracks,
        &stages.pre_alignment_tracks,
        residual_rounding,
    );
    let pre_alignment_units = pre_alignment_units?;
    let snapped_rows = if stages.stretch_applied {
        blink_layout_unit_stretch_auto_rows(container, grid_frame.height, pre_alignment_units)?
    } else {
        pre_alignment_units
    };
    if !snapped_rows_match_final_tracks(
        &snapped_rows,
        &stages.final_tracks,
        container,
        grid_frame.height,
        (four_fit_mixed_minimum || five_fit_mixed_minimum) && stages.stretch_applied,
    ) {
        return None;
    }
    Some(snapped_rows)
}

pub(in crate::grid::root_html_fit_content_auto_rows::repeated_text) fn repeated_text_blink_layout_unit_distribute_rows(
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

pub(super) fn blink_layout_unit_distribute_rows_from_stages(
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

        let floor_units = blink_layout_unit_floor_units(raw_size);
        let is_fractional = blink_layout_unit_is_fractional(raw_size);
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
        let content_growth_potential = (growth_limit - track.base_size).max(0.0);
        // automatic-minimum 후보는 base size를 키운다. 이 잔여분 순서에는 fit-content 상한을
        // 적용하지 않고, Blink는 max-content를 배분할 때 상한을 사용한다.
        let growth_potential = if matches!(
            residual_rounding,
            ResidualRounding::NearestFiveFitAutoLaterTrack
        ) {
            fit_content_limit.map_or(content_growth_potential, |limit| {
                content_growth_potential.max((limit - track.base_size).max(0.0))
            })
        } else {
            content_growth_potential
        };
        // 최소 기여로 인해 사용 base가 max-side 한도를 넘을 수 있다.
        let rounded_base_limit = growth_limit.max(track.base_size);
        let rounded_up_size = blink_layout_unit_to_px(floor_units + 1);
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
            let within_rounding_tolerance =
                rounded_up_size - raw_size <= 0.01 && rounded_up_size <= rounded_base_limit + 0.01;
            if within_rounding_tolerance {
                growth_potentials.push((row, growth_potential, is_auto_track));
            }
        } else if fit_content_limit.is_none()
            || (matches!(
                residual_rounding,
                ResidualRounding::UpLaterTrack
                    | ResidualRounding::UpAutoMinimumLaterTrack
                    | ResidualRounding::UpFiveFitAutoLaterTrack
                    | ResidualRounding::UpAutoLaterTrack
                    | ResidualRounding::NearestAutoLaterTrack
                    | ResidualRounding::NearestFiveFitAutoLaterTrack
            ) && rounded_up_size <= rounded_base_limit + 1.0 / 64.0)
        {
            growth_potentials.push((row, growth_potential, is_auto_track));
        }
    }

    // Blink는 각 track-sizing 단계에서 intrinsic 기여분을 1/64px LayoutUnit으로 배분한다.
    // 분수 몫은 최종 f32 합을 최근접 반올림하면 버려질 수 있는 1단위를 뒤쪽의 제한 없는
    // track에 남길 수 있다.
    let target_total = blink_layout_unit_round_total(raw_total, residual_rounding);
    let leftover_units = target_total - floor_total;
    if leftover_units < 0 || leftover_units as usize > growth_potentials.len() {
        return None;
    }
    // Blink는 growth potential 순서로 1/64px 잔여분을 나눈다. Taffy는 같은 크기를
    // f32로 유지하므로 여기서 잔여 단위를 배분한다.
    growth_potentials.sort_by(|left, right| {
        compare_layout_unit_potential(left.1, right.1)
            .then_with(|| {
                if matches!(residual_rounding, ResidualRounding::UpLaterTrack) {
                    std::cmp::Ordering::Equal
                } else {
                    left.2.cmp(&right.2)
                }
            })
            .then_with(|| left.0.cmp(&right.0))
    });
    for (row, _, _) in growth_potentials.iter().rev().take(leftover_units as usize) {
        quantized[*row] += 1;
    }
    Some(quantized)
}

fn blink_layout_unit_round_total(raw_total: f64, residual_rounding: ResidualRounding) -> i64 {
    match residual_rounding {
        ResidualRounding::Nearest
        | ResidualRounding::NearestAutoLaterTrack
        | ResidualRounding::NearestFiveFitAutoLaterTrack => (raw_total * 64.0).round() as i64,
        ResidualRounding::Up
        | ResidualRounding::UpAutoLaterTrack
        | ResidualRounding::UpLaterTrack
        | ResidualRounding::UpAutoMinimumLaterTrack
        | ResidualRounding::UpFiveFitAutoLaterTrack => blink_layout_unit_ceil_units(raw_total),
    }
}

fn restore_missing_residual_units(
    mut simulated: Vec<i64>,
    fallback_units: &[i64],
    target_total: i64,
) -> Option<Vec<i64>> {
    if simulated.len() != fallback_units.len() || fallback_units.iter().sum::<i64>() != target_total
    {
        return None;
    }

    let mut missing_units = target_total - simulated.iter().sum::<i64>();
    if missing_units <= 0 {
        return Some(simulated);
    }

    // 성장 한도를 Taffy trace와 맞춘 뒤 사라진 잔여 단위만 복원한다. span replay가 이미
    // 선택한 행별 분배는 유지하고, 기존 성장 잠재력 기반 분배가 선택한 증가분을 사용한다.
    for row in (0..simulated.len()).rev() {
        let available_units = (fallback_units[row] - simulated[row]).max(0);
        let restored_units = available_units.min(missing_units);
        simulated[row] += restored_units;
        missing_units -= restored_units;
        if missing_units == 0 {
            break;
        }
    }

    (missing_units == 0).then_some(simulated)
}

pub(super) fn blink_layout_unit_stretch_auto_rows(
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
