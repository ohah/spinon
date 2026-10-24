use spinon_core::NodeId;
use style::{
    values::computed::TrackList,
    values::generics::grid::{RepeatCount, TrackListValue},
};

use super::super::super::{
    ComputedCssGridAutoRepeat, ComputedCssGridAutoRepeatMode, ComputedCssGridTrackSize,
    CssCascadeError,
};
use super::{
    ComputedGridTracks, IntegerRepeatReservationSite, MAX_COMPUTED_GRID_LINE_METADATA_BYTES,
    MAX_COMPUTED_GRID_TRACKS, auto_repeat_track_has_fixed_minimum, copy_computed_line_name_groups,
    fail_integer_repeat_reservation_if_requested, line_name_groups_metadata_bytes,
    reserve_computed_tracks, reserve_integer_repeat_vec, track_size, track_size_is_valid,
    unsupported_grid_value,
};

pub(super) fn computed_auto_repeat_tracks(
    list: &TrackList,
    node: NodeId,
    property: &'static str,
    serialized: &str,
    allow_intrinsic_tracks: bool,
    allow_auto_repeat: bool,
) -> Result<ComputedGridTracks, CssCascadeError> {
    let has_valid_outer_line_names =
        list.values.len().checked_add(1) == Some(list.line_names.len());
    if !allow_auto_repeat || !has_valid_outer_line_names {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut auto_repeat_index = None;
    let mut fixed_repeat = None;
    let mut auto_repeat_mode = None;
    for (index, value) in list.values.iter().enumerate() {
        let TrackListValue::TrackRepeat(repeat) = value else {
            continue;
        };
        match repeat.count {
            RepeatCount::AutoFill => {
                if auto_repeat_index.replace(index).is_some() {
                    return Err(unsupported_grid_value(node, property, serialized));
                }
                auto_repeat_mode = Some(ComputedCssGridAutoRepeatMode::AutoFill);
            }
            RepeatCount::AutoFit => {
                if auto_repeat_index.replace(index).is_some() {
                    return Err(unsupported_grid_value(node, property, serialized));
                }
                auto_repeat_mode = Some(ComputedCssGridAutoRepeatMode::AutoFit);
            }
            RepeatCount::Number(count) => {
                if fixed_repeat.is_some() {
                    return Err(unsupported_grid_value(node, property, serialized));
                }
                let count = usize::try_from(count)
                    .ok()
                    .filter(|count| *count > 0)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
                fixed_repeat = Some((index, count));
            }
        }
    }
    let (Some(auto_repeat_index), Some(mode)) = (auto_repeat_index, auto_repeat_mode) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    let Some(TrackListValue::TrackRepeat(auto_repeat)) = list.values.get(auto_repeat_index) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    if auto_repeat.track_sizes.is_empty()
        || auto_repeat.track_sizes.len() > MAX_COMPUTED_GRID_TRACKS
        || auto_repeat.track_sizes.len().checked_add(1) != Some(auto_repeat.line_names.len())
        || !allow_intrinsic_tracks
    {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let fixed_repeat_track_count = if let Some((fixed_index, count)) = fixed_repeat {
        let Some(TrackListValue::TrackRepeat(repeat)) = list.values.get(fixed_index) else {
            return Err(unsupported_grid_value(node, property, serialized));
        };
        if repeat.track_sizes.is_empty()
            || repeat.track_sizes.len() > 2
            || repeat.track_sizes.len().checked_add(1) != Some(repeat.line_names.len())
            || list.line_names.iter().any(|group| !group.is_empty())
            || repeat.line_names.iter().any(|group| !group.is_empty())
            || auto_repeat.line_names.iter().any(|group| !group.is_empty())
            || repeat.track_sizes.iter().any(|size| {
                !matches!(
                    track_size(size, allow_intrinsic_tracks),
                    Some(ComputedCssGridTrackSize::LengthPx(value))
                        if value.is_finite() && value >= 0.0
                )
            })
        {
            return Err(unsupported_grid_value(node, property, serialized));
        }
        count
            .checked_mul(repeat.track_sizes.len())
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?
    } else {
        0
    };

    let mut total_track_count = 0_usize;
    for (index, value) in list.values.iter().enumerate() {
        let component_track_count = match value {
            TrackListValue::TrackSize(_) => 1,
            TrackListValue::TrackRepeat(repeat) if index == auto_repeat_index => {
                repeat.track_sizes.len()
            }
            TrackListValue::TrackRepeat(_)
                if fixed_repeat.is_some_and(|(fixed, _)| fixed == index) =>
            {
                fixed_repeat_track_count
            }
            TrackListValue::TrackRepeat(_) => {
                return Err(unsupported_grid_value(node, property, serialized));
            }
        };
        total_track_count = total_track_count
            .checked_add(component_track_count)
            .filter(|count| *count <= MAX_COMPUTED_GRID_TRACKS)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    }

    let auto_repeat_inner_bytes =
        line_name_groups_metadata_bytes(auto_repeat.line_names.iter().map(|group| group.iter()))
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let outer_line_name_count = if fixed_repeat.is_some() {
        total_track_count
            .checked_sub(auto_repeat.track_sizes.len())
            .and_then(|count| count.checked_add(2))
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?
    } else {
        list.line_names.len()
    };
    let outer_line_name_bytes = if fixed_repeat.is_some() {
        outer_line_name_count.checked_mul(std::mem::size_of::<Vec<String>>())
    } else {
        line_name_groups_metadata_bytes(list.line_names.iter().map(|group| group.iter()))
    };
    outer_line_name_bytes
        .and_then(|bytes| bytes.checked_add(auto_repeat_inner_bytes))
        .filter(|bytes| *bytes <= MAX_COMPUTED_GRID_LINE_METADATA_BYTES)
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;

    let mut tracks = Vec::new();
    reserve_computed_tracks(
        &mut tracks,
        total_track_count,
        node,
        property,
        |tracks, count| {
            fail_integer_repeat_reservation_if_requested(IntegerRepeatReservationSite::Tracks)?;
            tracks.try_reserve_exact(count).map_err(|_| ())
        },
    )?;

    let mut auto_repeat_start = None;
    let mut auto_repeat_end = None;
    for (index, value) in list.values.iter().enumerate() {
        match value {
            TrackListValue::TrackSize(size) => {
                tracks.push(
                    track_size(size, allow_intrinsic_tracks)
                        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?,
                );
            }
            TrackListValue::TrackRepeat(repeat) if index == auto_repeat_index => {
                auto_repeat_start = Some(tracks.len());
                for size in repeat.track_sizes.iter() {
                    let track = track_size(size, allow_intrinsic_tracks)
                        .filter(track_size_is_valid)
                        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
                    if !auto_repeat_track_has_fixed_minimum(&track) {
                        return Err(unsupported_grid_value(node, property, serialized));
                    }
                    tracks.push(track);
                }
                auto_repeat_end = Some(tracks.len());
            }
            TrackListValue::TrackRepeat(repeat)
                if fixed_repeat.is_some_and(|(fixed_index, _)| fixed_index == index) =>
            {
                let Some((_, count)) =
                    fixed_repeat.filter(|(fixed_index, _)| *fixed_index == index)
                else {
                    return Err(unsupported_grid_value(node, property, serialized));
                };
                for _ in 0..count {
                    for size in repeat.track_sizes.iter() {
                        let track = track_size(size, allow_intrinsic_tracks)
                            .filter(|track| {
                                matches!(track, ComputedCssGridTrackSize::LengthPx(value)
                                    if value.is_finite() && *value >= 0.0)
                            })
                            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
                        tracks.push(track);
                    }
                }
            }
            TrackListValue::TrackRepeat(_) => {
                return Err(unsupported_grid_value(node, property, serialized));
            }
        }
    }

    let (Some(start), Some(end)) = (auto_repeat_start, auto_repeat_end) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    let line_names = if fixed_repeat.is_some() {
        let mut groups = Vec::new();
        reserve_integer_repeat_vec(
            &mut groups,
            outer_line_name_count,
            IntegerRepeatReservationSite::LineGroups,
            node,
            property,
        )?;
        groups.resize_with(outer_line_name_count, Vec::new);
        groups
    } else {
        copy_computed_line_name_groups(
            list.line_names.iter().map(|group| group.iter()),
            node,
            property,
        )?
    };
    let repeated_line_names = copy_computed_line_name_groups(
        auto_repeat.line_names.iter().map(|group| group.iter()),
        node,
        property,
    )?;

    Ok(ComputedGridTracks {
        tracks,
        line_names,
        auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode,
            start,
            end,
            line_names: repeated_line_names,
        }),
        has_fixed_repeat: fixed_repeat.is_some(),
    })
}
