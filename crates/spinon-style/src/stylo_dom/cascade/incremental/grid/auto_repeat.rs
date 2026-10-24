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
    MAX_COMPUTED_GRID_TRACKS, add_line_name_budget, auto_repeat_track_has_fixed_minimum,
    copy_computed_line_name_groups, fail_integer_repeat_reservation_if_requested,
    line_name_groups_metadata_bytes, reserve_computed_tracks, reserve_integer_repeat_string,
    reserve_integer_repeat_vec, track_size, track_size_is_valid, unsupported_grid_value,
};

#[derive(Clone, Copy)]
struct FixedRepeatGroup {
    count: usize,
    expanded_track_count: usize,
}

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
    if list.values.len() > MAX_COMPUTED_GRID_TRACKS {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut auto_repeat_index = None;
    let mut fixed_repeat_group_count = 0_usize;
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
                if usize::try_from(count)
                    .ok()
                    .filter(|count| *count > 0)
                    .is_none()
                {
                    return Err(unsupported_grid_value(node, property, serialized));
                }
                fixed_repeat_group_count = fixed_repeat_group_count
                    .checked_add(1)
                    .filter(|count| *count <= MAX_COMPUTED_GRID_TRACKS)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
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

    let mut fixed_repeats = Vec::<Option<FixedRepeatGroup>>::new();
    if fixed_repeat_group_count > 0 {
        reserve_integer_repeat_vec(
            &mut fixed_repeats,
            list.values.len(),
            IntegerRepeatReservationSite::FixedRepeatGroups,
            node,
            property,
        )?;
        fixed_repeats.resize_with(list.values.len(), || None);
        for (index, value) in list.values.iter().enumerate() {
            let TrackListValue::TrackRepeat(repeat) = value else {
                continue;
            };
            let RepeatCount::Number(count) = repeat.count else {
                continue;
            };
            let count = usize::try_from(count)
                .ok()
                .filter(|count| *count > 0)
                .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
            if repeat.track_sizes.is_empty()
                || repeat.track_sizes.len() > 2
                || repeat.track_sizes.len().checked_add(1) != Some(repeat.line_names.len())
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
            let expanded_track_count = count
                .checked_mul(repeat.track_sizes.len())
                .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
            fixed_repeats[index] = Some(FixedRepeatGroup {
                count,
                expanded_track_count,
            });
        }
    }

    let mut total_track_count = 0_usize;
    for (index, value) in list.values.iter().enumerate() {
        let component_track_count = match value {
            TrackListValue::TrackSize(_) => 1,
            TrackListValue::TrackRepeat(repeat) if index == auto_repeat_index => {
                repeat.track_sizes.len()
            }
            TrackListValue::TrackRepeat(_) => fixed_repeats
                .get(index)
                .and_then(Option::as_ref)
                .map(|group| group.expanded_track_count)
                .ok_or_else(|| unsupported_grid_value(node, property, serialized))?,
        };
        total_track_count = total_track_count
            .checked_add(component_track_count)
            .filter(|count| *count <= MAX_COMPUTED_GRID_TRACKS)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    }

    let auto_repeat_inner_bytes =
        line_name_groups_metadata_bytes(auto_repeat.line_names.iter().map(|group| group.iter()))
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let outer_line_name_count = if fixed_repeat_group_count > 0 {
        total_track_count
            .checked_sub(auto_repeat.track_sizes.len())
            .and_then(|count| count.checked_add(2))
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?
    } else {
        list.line_names.len()
    };
    let outer_line_name_bytes = if fixed_repeat_group_count > 0 {
        expanded_fixed_line_name_metadata_bytes(
            list,
            &fixed_repeats,
            outer_line_name_count,
            node,
            property,
            serialized,
        )?
    } else {
        line_name_groups_metadata_bytes(list.line_names.iter().map(|group| group.iter()))
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?
    };
    outer_line_name_bytes
        .checked_add(auto_repeat_inner_bytes)
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
            TrackListValue::TrackRepeat(repeat) => {
                let Some(group) = fixed_repeats.get(index).and_then(Option::as_ref) else {
                    return Err(unsupported_grid_value(node, property, serialized));
                };
                for _ in 0..group.count {
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
        }
    }

    let (Some(start), Some(end)) = (auto_repeat_start, auto_repeat_end) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    let line_names = if fixed_repeat_group_count > 0 {
        expand_fixed_repeat_line_names(
            list,
            auto_repeat_index,
            &fixed_repeats,
            outer_line_name_count,
            node,
            property,
            serialized,
        )?
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
    let single_bounded_auto_repeat = list.values.len() == 1
        && fixed_repeat_group_count == 0
        && auto_repeat.track_sizes.len() == 1
        && auto_repeat.track_sizes.iter().all(|size| {
            matches!(
                track_size(size, allow_intrinsic_tracks),
                Some(ComputedCssGridTrackSize::LengthPx(value))
                    if value.is_finite() && value >= 0.0
            )
        });

    Ok(ComputedGridTracks {
        tracks,
        line_names,
        auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode,
            start,
            end,
            line_names: repeated_line_names,
        }),
        has_fixed_repeat: fixed_repeat_group_count > 0,
        fixed_repeat_group_count,
        single_bounded_fixed_repeat: false,
        single_bounded_auto_repeat,
    })
}

fn expanded_fixed_line_name_metadata_bytes(
    list: &TrackList,
    fixed_repeats: &[Option<FixedRepeatGroup>],
    outer_line_name_count: usize,
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<usize, CssCascadeError> {
    let mut name_count = 0_usize;
    let mut name_bytes = 0_usize;
    for names in list.line_names.iter() {
        add_line_name_budget(
            &mut name_count,
            &mut name_bytes,
            names,
            1,
            node,
            property,
            serialized,
        )?;
    }
    for (index, value) in list.values.iter().enumerate() {
        let TrackListValue::TrackRepeat(repeat) = value else {
            continue;
        };
        let Some(group) = fixed_repeats.get(index).and_then(Option::as_ref) else {
            continue;
        };
        for names in repeat.line_names.iter() {
            add_line_name_budget(
                &mut name_count,
                &mut name_bytes,
                names,
                group.count,
                node,
                property,
                serialized,
            )?;
        }
    }
    outer_line_name_count
        .checked_mul(std::mem::size_of::<Vec<String>>())
        .and_then(|bytes| {
            name_count
                .checked_mul(std::mem::size_of::<String>())
                .and_then(|name_headers| bytes.checked_add(name_headers))
        })
        .and_then(|bytes| bytes.checked_add(name_bytes))
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))
}

fn expand_fixed_repeat_line_names(
    list: &TrackList,
    auto_repeat_index: usize,
    fixed_repeats: &[Option<FixedRepeatGroup>],
    outer_line_name_count: usize,
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<Vec<Vec<String>>, CssCascadeError> {
    let mut groups = Vec::new();
    reserve_integer_repeat_vec(
        &mut groups,
        outer_line_name_count,
        IntegerRepeatReservationSite::LineGroups,
        node,
        property,
    )?;
    groups.resize_with(outer_line_name_count, Vec::new);

    let mut group_index = 0_usize;
    for (index, value) in list.values.iter().enumerate() {
        let Some(outer_names) = list.line_names.get(index) else {
            return Err(unsupported_grid_value(node, property, serialized));
        };
        append_line_names_to_group(
            &mut groups,
            group_index,
            outer_names,
            node,
            property,
            serialized,
        )?;

        match value {
            TrackListValue::TrackSize(_) => {
                group_index = next_line_name_group_index(
                    group_index,
                    outer_line_name_count,
                    node,
                    property,
                    serialized,
                )?;
            }
            TrackListValue::TrackRepeat(_) if index == auto_repeat_index => {
                group_index = next_line_name_group_index(
                    group_index,
                    outer_line_name_count,
                    node,
                    property,
                    serialized,
                )?;
            }
            TrackListValue::TrackRepeat(repeat) => {
                let Some(group) = fixed_repeats.get(index).and_then(Option::as_ref) else {
                    return Err(unsupported_grid_value(node, property, serialized));
                };
                for _ in 0..group.count {
                    for track_index in 0..repeat.track_sizes.len() {
                        let Some(names) = repeat.line_names.get(track_index) else {
                            return Err(unsupported_grid_value(node, property, serialized));
                        };
                        append_line_names_to_group(
                            &mut groups,
                            group_index,
                            names,
                            node,
                            property,
                            serialized,
                        )?;
                        group_index = next_line_name_group_index(
                            group_index,
                            outer_line_name_count,
                            node,
                            property,
                            serialized,
                        )?;
                    }
                    let Some(names) = repeat.line_names.last() else {
                        return Err(unsupported_grid_value(node, property, serialized));
                    };
                    append_line_names_to_group(
                        &mut groups,
                        group_index,
                        names,
                        node,
                        property,
                        serialized,
                    )?;
                }
            }
        }
    }

    let Some(final_names) = list.line_names.last() else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    append_line_names_to_group(
        &mut groups,
        group_index,
        final_names,
        node,
        property,
        serialized,
    )?;
    if group_index.checked_add(1) != Some(outer_line_name_count) {
        return Err(unsupported_grid_value(node, property, serialized));
    }
    Ok(groups)
}

fn next_line_name_group_index(
    index: usize,
    group_count: usize,
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<usize, CssCascadeError> {
    index
        .checked_add(1)
        .filter(|next| *next < group_count)
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))
}

fn append_line_names_to_group(
    groups: &mut [Vec<String>],
    group_index: usize,
    names: &[style::values::CustomIdent],
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<(), CssCascadeError> {
    let Some(group) = groups.get_mut(group_index) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    reserve_integer_repeat_vec(
        group,
        names.len(),
        IntegerRepeatReservationSite::LineNames,
        node,
        property,
    )?;
    for name in names {
        let value = name.0.as_str();
        let mut owned = String::new();
        reserve_integer_repeat_string(&mut owned, value.len(), node, property)?;
        owned.push_str(value);
        group.push(owned);
    }
    Ok(())
}
