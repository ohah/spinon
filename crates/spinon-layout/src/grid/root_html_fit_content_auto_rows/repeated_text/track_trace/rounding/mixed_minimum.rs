use super::{blink_layout_unit_floor_units, blink_layout_unit_to_px};

pub(super) fn blink_layout_unit_ceil_units(value: f64) -> i64 {
    let rounding_error_bound = f64::from(f32::EPSILON) * value.abs().max(1.0) * 4.0 * 64.0;
    (value * 64.0 - rounding_error_bound).ceil() as i64
}

pub(super) fn compare_layout_unit_potential(left: f32, right: f32) -> std::cmp::Ordering {
    match (left.is_finite(), right.is_finite()) {
        (true, true) => {
            let left_units = (f64::from(left) * 64.0).round() as i64;
            let right_units = (f64::from(right) * 64.0).round() as i64;
            left_units.cmp(&right_units)
        }
        (false, false) => std::cmp::Ordering::Equal,
        (false, true) => std::cmp::Ordering::Greater,
        (true, false) => std::cmp::Ordering::Less,
    }
}

pub(super) fn has_mixed_span_minimum_modes(
    input: &crate::LayoutInput,
    grid_id: crate::NodeId,
) -> bool {
    let Some(grid) = input.nodes().iter().find(|node| node.id == grid_id) else {
        return false;
    };
    let mut has_zero_minimum = false;
    let mut has_auto_minimum = false;
    for item_id in grid.children.iter().copied().skip(1) {
        let Some(item) = input.nodes().iter().find(|node| node.id == item_id) else {
            return false;
        };
        match item.style.min_height {
            crate::LayoutDimension::Auto => has_auto_minimum = true,
            crate::LayoutDimension::Fixed(0.0) => has_zero_minimum = true,
            _ => {}
        }
    }
    has_zero_minimum && has_auto_minimum
}

fn singleton_stretched_auto_row_index(
    container: &crate::GridContainerInput,
    row_count: usize,
) -> Option<usize> {
    let crate::GridAutoRows::TrackList(auto_rows) = &container.auto_rows else {
        return None;
    };
    if auto_rows.is_empty() {
        return None;
    }
    let stretched_rows = (container.rows.len()..row_count)
        .filter(|row| {
            matches!(
                auto_rows[(*row - container.rows.len()) % auto_rows.len()],
                crate::GridTrackSize::Auto
            )
        })
        .collect::<Vec<_>>();
    match stretched_rows.as_slice() {
        [row] => Some(*row),
        _ => None,
    }
}

fn mixed_five_fit_stretch_rows_match_trace(
    row_size_units: &[i64],
    final_tracks: &[crate::LayoutGridTrackSizingTrackState],
    container: &crate::GridContainerInput,
    grid_height: f32,
) -> bool {
    if row_size_units.len() != final_tracks.len() {
        return false;
    }
    let Some(stretched_row) = singleton_stretched_auto_row_index(container, row_size_units.len())
    else {
        return false;
    };
    if row_size_units.iter().sum::<i64>() != blink_layout_unit_floor_units(grid_height) {
        return false;
    }

    let mut non_stretched_delta = 0.0_f64;
    for (row, (units, track)) in row_size_units.iter().zip(final_tracks).enumerate() {
        if row == stretched_row {
            continue;
        }
        let delta = f64::from(blink_layout_unit_to_px(*units)) - f64::from(track.base_size);
        if delta.abs() > 0.02 {
            return false;
        }
        non_stretched_delta += delta;
    }

    // Blink rounds intrinsic rows to 1/64px before it assigns the remaining block size to
    // the singleton auto tail. That tail absorbs the other rows' rounding residual.
    let normalized_total = row_size_units.iter().sum::<i64>() as f64 / 64.0;
    let traced_total = final_tracks
        .iter()
        .map(|track| f64::from(track.base_size))
        .sum::<f64>();
    let expected_tail_delta = normalized_total - traced_total - non_stretched_delta;
    let actual_tail_delta = f64::from(blink_layout_unit_to_px(row_size_units[stretched_row]))
        - f64::from(final_tracks[stretched_row].base_size);
    (actual_tail_delta - expected_tail_delta).abs() <= 0.001
}

pub(super) fn snapped_rows_match_final_tracks(
    row_size_units: &[i64],
    final_tracks: &[crate::LayoutGridTrackSizingTrackState],
    container: &crate::GridContainerInput,
    grid_height: f32,
    allow_mixed_auto_tail_residual: bool,
) -> bool {
    if row_size_units.len() != final_tracks.len() {
        return false;
    }
    if allow_mixed_auto_tail_residual
        && singleton_stretched_auto_row_index(container, row_size_units.len()).is_some()
    {
        return mixed_five_fit_stretch_rows_match_trace(
            row_size_units,
            final_tracks,
            container,
            grid_height,
        );
    }
    row_size_units
        .iter()
        .zip(final_tracks)
        .all(|(units, track)| (blink_layout_unit_to_px(*units) - track.base_size).abs() <= 0.02)
}
