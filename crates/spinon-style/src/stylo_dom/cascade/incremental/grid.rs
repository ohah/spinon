use std::collections::BTreeMap;

use spinon_core::NodeId;
use style::{
    properties::ComputedValues,
    typed_om::{ToTyped, TypedValue},
    values::computed::{
        GridTemplateComponent, TrackBreadth, TrackSize,
        length_percentage::Unpacked as LengthPercentageValue, position::GridAutoFlow,
    },
    values::generics::grid::{RepeatCount, TrackListValue},
};

use super::super::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridItemPlacement,
    ComputedCssGridLayout, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize, CssCascadeError,
};

const MAX_COMPUTED_GRID_TRACKS: usize = 10_000;
const MAX_COMPUTED_GRID_LINE_METADATA_BYTES: usize = 1024 * 1024;

struct ComputedGridTracks {
    tracks: Vec<ComputedCssGridTrackSize>,
    line_names: Vec<Vec<String>>,
    auto_repeat: Option<ComputedCssGridAutoRepeat>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::stylo_dom::cascade) enum IntegerRepeatReservationSite {
    Tracks,
    LineGroups,
    LineNames,
    LineNameText,
    AutoRows,
    AutoColumns,
}

#[cfg(test)]
thread_local! {
    static INTEGER_REPEAT_FAILURE_SITE: std::cell::Cell<Option<IntegerRepeatReservationSite>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
struct IntegerRepeatFailureGuard;

#[cfg(test)]
impl Drop for IntegerRepeatFailureGuard {
    fn drop(&mut self) {
        INTEGER_REPEAT_FAILURE_SITE.with(|site| site.set(None));
    }
}

#[cfg(test)]
pub(in crate::stylo_dom::cascade) fn inject_integer_repeat_reservation_failure(
    site: IntegerRepeatReservationSite,
) -> impl Drop {
    let was_armed =
        INTEGER_REPEAT_FAILURE_SITE.with(|current| current.replace(Some(site)).is_some());
    assert!(
        !was_armed,
        "repeat reservation failpoint은 중첩할 수 없습니다"
    );
    IntegerRepeatFailureGuard
}

pub(super) fn computed_grid_layout(
    computed: &ComputedValues,
    node: NodeId,
    properties: &BTreeMap<String, String>,
    allow_integer_repeat: bool,
    allow_intrinsic_tracks: bool,
    allow_auto_repeat: bool,
) -> Result<ComputedCssGridLayout, CssCascadeError> {
    let template_areas = if allow_integer_repeat {
        super::grid_areas::computed_grid_template_areas(
            computed.clone_grid_template_areas(),
            node,
            property_value(properties, "grid-template-areas"),
        )?
    } else {
        None
    };
    let columns_value = property_value(properties, "grid-template-columns");
    let rows_value = property_value(properties, "grid-template-rows");
    let ComputedGridTracks {
        tracks: columns,
        line_names: columns_line_names,
        auto_repeat: columns_auto_repeat,
    } = computed_grid_tracks(
        computed.clone_grid_template_columns(),
        node,
        "grid-template-columns",
        columns_value,
        allow_integer_repeat,
        allow_intrinsic_tracks,
        allow_auto_repeat,
    )?;
    let ComputedGridTracks {
        tracks: rows,
        line_names: rows_line_names,
        auto_repeat: rows_auto_repeat,
    } = computed_grid_tracks(
        computed.clone_grid_template_rows(),
        node,
        "grid-template-rows",
        rows_value,
        allow_integer_repeat,
        allow_intrinsic_tracks,
        allow_auto_repeat,
    )?;
    if template_areas.is_some() && (columns_auto_repeat.is_some() || rows_auto_repeat.is_some()) {
        return Err(unsupported_grid_value(
            node,
            if columns_auto_repeat.is_some() {
                "grid-template-columns"
            } else {
                "grid-template-rows"
            },
            if columns_auto_repeat.is_some() {
                columns_value
            } else {
                rows_value
            },
        ));
    }
    let auto_rows = computed_grid_auto_rows(
        computed.clone_grid_auto_rows(),
        node,
        property_value(properties, "grid-auto-rows"),
        allow_intrinsic_tracks,
    )?;
    let auto_columns = computed_grid_auto_columns(
        computed.clone_grid_auto_columns(),
        node,
        property_value(properties, "grid-auto-columns"),
        allow_intrinsic_tracks,
    )?;
    let flow = computed.clone_grid_auto_flow();
    let auto_flow = match (
        flow.contains(GridAutoFlow::COLUMN),
        flow.contains(GridAutoFlow::DENSE),
    ) {
        (false, false) => ComputedCssGridAutoFlow::Row,
        (true, false) => ComputedCssGridAutoFlow::Column,
        (false, true) => ComputedCssGridAutoFlow::RowDense,
        (true, true) => ComputedCssGridAutoFlow::ColumnDense,
    };

    Ok(ComputedCssGridLayout {
        columns,
        rows,
        columns_auto_repeat,
        rows_auto_repeat,
        columns_line_names,
        rows_line_names,
        template_areas,
        auto_flow,
        auto_rows,
        auto_columns,
    })
}

fn computed_grid_auto_columns(
    value: style::values::computed::ImplicitGridTracks,
    node: NodeId,
    serialized: &str,
    allow_intrinsic_tracks: bool,
) -> Result<ComputedCssGridAutoColumns, CssCascadeError> {
    Ok(
        match computed_grid_auto_track_list(
            value,
            node,
            "grid-auto-columns",
            serialized,
            allow_intrinsic_tracks,
            IntegerRepeatReservationSite::AutoColumns,
        )? {
            ComputedGridAutoTrackValue::Auto => ComputedCssGridAutoColumns::Auto,
            ComputedGridAutoTrackValue::LengthPx(value) => {
                ComputedCssGridAutoColumns::LengthPx(value)
            }
            ComputedGridAutoTrackValue::TrackList(tracks) => {
                ComputedCssGridAutoColumns::TrackList(tracks)
            }
            ComputedGridAutoTrackValue::Unsupported(value) => {
                ComputedCssGridAutoColumns::Unsupported(value)
            }
        },
    )
}

fn computed_grid_auto_rows(
    value: style::values::computed::ImplicitGridTracks,
    node: NodeId,
    serialized: &str,
    allow_intrinsic_tracks: bool,
) -> Result<ComputedCssGridAutoRows, CssCascadeError> {
    Ok(
        match computed_grid_auto_track_list(
            value,
            node,
            "grid-auto-rows",
            serialized,
            allow_intrinsic_tracks,
            IntegerRepeatReservationSite::AutoRows,
        )? {
            ComputedGridAutoTrackValue::Auto => ComputedCssGridAutoRows::Auto,
            ComputedGridAutoTrackValue::LengthPx(value) => ComputedCssGridAutoRows::LengthPx(value),
            ComputedGridAutoTrackValue::TrackList(tracks) => {
                ComputedCssGridAutoRows::TrackList(tracks)
            }
            ComputedGridAutoTrackValue::Unsupported(value) => {
                ComputedCssGridAutoRows::Unsupported(value)
            }
        },
    )
}

enum ComputedGridAutoTrackValue {
    Auto,
    LengthPx(f32),
    TrackList(Vec<ComputedCssGridTrackSize>),
    Unsupported(String),
}

fn computed_grid_auto_track_list(
    value: style::values::computed::ImplicitGridTracks,
    node: NodeId,
    property: &'static str,
    serialized: &str,
    allow_intrinsic_tracks: bool,
    reservation_site: IntegerRepeatReservationSite,
) -> Result<ComputedGridAutoTrackValue, CssCascadeError> {
    if value.is_initial() {
        return Ok(ComputedGridAutoTrackValue::Auto);
    }
    if value.0.is_empty() || value.0.len() > MAX_COMPUTED_GRID_TRACKS {
        return Ok(ComputedGridAutoTrackValue::Unsupported(
            serialized.to_owned(),
        ));
    }

    if !allow_intrinsic_tracks {
        if value.0.len() != 1 {
            return Ok(ComputedGridAutoTrackValue::Unsupported(
                serialized.to_owned(),
            ));
        }
        let TrackSize::Breadth(TrackBreadth::Breadth(value)) = &value.0[0] else {
            return Ok(ComputedGridAutoTrackValue::Unsupported(
                serialized.to_owned(),
            ));
        };
        let LengthPercentageValue::Length(length) = value.unpack() else {
            return Ok(ComputedGridAutoTrackValue::Unsupported(
                serialized.to_owned(),
            ));
        };
        let pixels = length.px();
        return Ok(if pixels.is_finite() && pixels >= 0.0 {
            ComputedGridAutoTrackValue::LengthPx(pixels)
        } else {
            ComputedGridAutoTrackValue::Unsupported(serialized.to_owned())
        });
    }

    let mut tracks = Vec::new();
    reserve_computed_tracks(
        &mut tracks,
        value.0.len(),
        node,
        property,
        |tracks, count| {
            fail_integer_repeat_reservation_if_requested(reservation_site)?;
            tracks.try_reserve_exact(count).map_err(|_| ())
        },
    )?;
    for size in value.0.iter() {
        let Some(track) = track_size(size, true) else {
            return Ok(ComputedGridAutoTrackValue::Unsupported(
                serialized.to_owned(),
            ));
        };
        tracks.push(track);
    }
    if let [ComputedCssGridTrackSize::LengthPx(value)] = tracks.as_slice() {
        Ok(ComputedGridAutoTrackValue::LengthPx(*value))
    } else {
        Ok(ComputedGridAutoTrackValue::TrackList(tracks))
    }
}

pub(super) fn computed_grid_item_placement(
    computed: &ComputedValues,
    properties: &BTreeMap<String, String>,
    allow_named_lines: bool,
) -> ComputedCssGridItemPlacement {
    let row_start = computed.clone_grid_row_start();
    let row_end = computed.clone_grid_row_end();
    let column_start = computed.clone_grid_column_start();
    let column_end = computed.clone_grid_column_end();
    ComputedCssGridItemPlacement {
        row_start: computed_grid_placement(
            row_start,
            property_value(properties, "grid-row-start"),
            allow_named_lines,
        ),
        row_end: computed_grid_placement(
            row_end,
            property_value(properties, "grid-row-end"),
            allow_named_lines,
        ),
        column_start: computed_grid_placement(
            column_start,
            property_value(properties, "grid-column-start"),
            allow_named_lines,
        ),
        column_end: computed_grid_placement(
            column_end,
            property_value(properties, "grid-column-end"),
            allow_named_lines,
        ),
    }
}

fn computed_grid_placement(
    value: style::values::computed::GridLine,
    serialized: &str,
    allow_named_lines: bool,
) -> ComputedCssGridPlacement {
    let name = value.ident.0.as_str();
    if !name.is_empty() {
        if !allow_named_lines {
            return ComputedCssGridPlacement::Unsupported(serialized.to_owned());
        }
        if value.is_span {
            let count = if value.line_num == 0 {
                1
            } else if let Ok(count) = u32::try_from(value.line_num) {
                count
            } else {
                return ComputedCssGridPlacement::Unsupported(serialized.to_owned());
            };
            return ComputedCssGridPlacement::SpanNamed {
                name: name.to_owned(),
                count,
            };
        }
        return ComputedCssGridPlacement::NamedLine {
            name: name.to_owned(),
            occurrence: (value.line_num != 0).then_some(value.line_num),
        };
    }
    if value.is_auto() {
        return ComputedCssGridPlacement::Auto;
    }
    if value.is_span {
        ComputedCssGridPlacement::Span(value.line_num)
    } else {
        ComputedCssGridPlacement::Line(value.line_num)
    }
}

fn computed_grid_tracks(
    component: GridTemplateComponent,
    node: NodeId,
    property: &'static str,
    serialized: &str,
    allow_integer_repeat: bool,
    allow_intrinsic_tracks: bool,
    allow_auto_repeat: bool,
) -> Result<ComputedGridTracks, CssCascadeError> {
    let GridTemplateComponent::TrackList(list) = component else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    if list.values.is_empty() {
        return Err(unsupported_grid_value(node, property, serialized));
    }
    if list.has_auto_repeat() {
        return computed_auto_repeat_tracks(
            &list,
            node,
            property,
            serialized,
            allow_intrinsic_tracks,
            allow_auto_repeat,
        );
    }

    if !allow_integer_repeat {
        if list.values.len() > MAX_COMPUTED_GRID_TRACKS
            || list.line_names.iter().any(|names| !names.is_empty())
            || list
                .values
                .iter()
                .any(|value| matches!(value, TrackListValue::TrackRepeat(_)))
        {
            return Err(unsupported_grid_value(node, property, serialized));
        }
        let mut tracks = Vec::new();
        reserve_computed_tracks(
            &mut tracks,
            list.values.len(),
            node,
            property,
            |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
        )?;
        for value in list.values.iter() {
            let TrackListValue::TrackSize(size) = value else {
                return Err(unsupported_grid_value(node, property, serialized));
            };
            tracks.push(
                track_size(size, allow_intrinsic_tracks)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?,
            );
        }
        return Ok(ComputedGridTracks {
            tracks,
            line_names: Vec::new(),
            auto_repeat: None,
        });
    }

    expand_integer_repeats(&list, node, property, serialized, allow_intrinsic_tracks).map(
        |(tracks, line_names)| ComputedGridTracks {
            tracks,
            line_names,
            auto_repeat: None,
        },
    )
}

fn computed_auto_repeat_tracks(
    list: &style::values::computed::TrackList,
    node: NodeId,
    property: &'static str,
    serialized: &str,
    allow_intrinsic_tracks: bool,
    allow_auto_repeat: bool,
) -> Result<ComputedGridTracks, CssCascadeError> {
    let has_valid_outer_line_names = list.values.len().checked_add(1)
        == Some(list.line_names.len())
        && list.line_names.iter().all(|names| names.is_empty());
    if !allow_auto_repeat || !has_valid_outer_line_names {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut repeat_index = None;
    let mut mode = None;
    for (index, value) in list.values.iter().enumerate() {
        let TrackListValue::TrackRepeat(repeat) = value else {
            continue;
        };
        let current_mode = match repeat.count {
            RepeatCount::AutoFill => ComputedCssGridAutoRepeatMode::AutoFill,
            RepeatCount::AutoFit => ComputedCssGridAutoRepeatMode::AutoFit,
            RepeatCount::Number(_) => {
                return Err(unsupported_grid_value(node, property, serialized));
            }
        };
        if repeat_index.replace(index).is_some() {
            return Err(unsupported_grid_value(node, property, serialized));
        }
        mode = Some(current_mode);
    }
    let (Some(repeat_index), Some(mode)) = (repeat_index, mode) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    let Some(TrackListValue::TrackRepeat(repeat)) = list.values.get(repeat_index) else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    if repeat.track_sizes.is_empty()
        || repeat.track_sizes.len() > MAX_COMPUTED_GRID_TRACKS
        || repeat.track_sizes.len().checked_add(1) != Some(repeat.line_names.len())
        || repeat.line_names.iter().any(|names| !names.is_empty())
        || !allow_intrinsic_tracks
    {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let prefix_count = repeat_index;
    let repeat_end = prefix_count
        .checked_add(repeat.track_sizes.len())
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let suffix_count = list.values.len() - repeat_index - 1;
    let total_track_count = repeat_end
        .checked_add(suffix_count)
        .filter(|count| *count <= MAX_COMPUTED_GRID_TRACKS)
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let mut tracks = Vec::new();
    reserve_computed_tracks(
        &mut tracks,
        total_track_count,
        node,
        property,
        |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
    )?;

    for value in list.values.iter().take(prefix_count) {
        let TrackListValue::TrackSize(size) = value else {
            return Err(unsupported_grid_value(node, property, serialized));
        };
        let track = track_size(size, allow_intrinsic_tracks)
            .filter(is_auto_repeat_neighbor_track)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
        tracks.push(track);
    }
    for size in repeat.track_sizes.iter() {
        let track = track_size(size, allow_intrinsic_tracks)
            .filter(track_size_is_valid)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
        if !auto_repeat_track_has_fixed_minimum(&track) {
            return Err(unsupported_grid_value(node, property, serialized));
        }
        tracks.push(track);
    }
    for value in list.values.iter().skip(repeat_index + 1) {
        let TrackListValue::TrackSize(size) = value else {
            return Err(unsupported_grid_value(node, property, serialized));
        };
        let track = track_size(size, allow_intrinsic_tracks)
            .filter(is_auto_repeat_neighbor_track)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
        tracks.push(track);
    }
    Ok(ComputedGridTracks {
        tracks,
        line_names: Vec::new(),
        auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode,
            start: prefix_count,
            end: repeat_end,
        }),
    })
}

fn is_auto_repeat_neighbor_track(track: &ComputedCssGridTrackSize) -> bool {
    matches!(track, ComputedCssGridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}

fn expand_integer_repeats(
    list: &style::values::computed::TrackList,
    node: NodeId,
    property: &'static str,
    serialized: &str,
    allow_intrinsic_tracks: bool,
) -> Result<(Vec<ComputedCssGridTrackSize>, Vec<Vec<String>>), CssCascadeError> {
    let outer_line_name_count = list
        .values
        .len()
        .checked_add(1)
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    if list.line_names.len() != outer_line_name_count {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut expanded_count = 0_usize;
    let mut expanded_name_count = 0_usize;
    let mut expanded_name_bytes = 0_usize;
    for names in list.line_names.iter() {
        add_line_name_budget(
            &mut expanded_name_count,
            &mut expanded_name_bytes,
            names,
            1,
            node,
            property,
            serialized,
        )?;
    }
    for value in list.values.iter() {
        let count = match value {
            TrackListValue::TrackSize(_) => 1,
            TrackListValue::TrackRepeat(repeat) => {
                let RepeatCount::Number(count) = repeat.count else {
                    return Err(unsupported_grid_value(node, property, serialized));
                };
                let count = usize::try_from(count)
                    .ok()
                    .filter(|count| *count > 0)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
                if repeat.track_sizes.is_empty()
                    || repeat.track_sizes.len().checked_add(1) != Some(repeat.line_names.len())
                    || repeat
                        .track_sizes
                        .iter()
                        .any(|size| track_size(size, allow_intrinsic_tracks).is_none())
                {
                    return Err(unsupported_grid_value(node, property, serialized));
                }
                for names in repeat.line_names.iter() {
                    add_line_name_budget(
                        &mut expanded_name_count,
                        &mut expanded_name_bytes,
                        names,
                        count,
                        node,
                        property,
                        serialized,
                    )?;
                }
                repeat
                    .track_sizes
                    .len()
                    .checked_mul(count)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?
            }
        };
        expanded_count = expanded_count
            .checked_add(count)
            .filter(|count| *count <= MAX_COMPUTED_GRID_TRACKS)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    }
    let line_count = expanded_count
        .checked_add(1)
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let estimated_metadata_bytes = line_count
        .checked_mul(std::mem::size_of::<Vec<String>>())
        .and_then(|bytes| {
            expanded_name_count
                .checked_mul(std::mem::size_of::<String>())
                .and_then(|name_headers| bytes.checked_add(name_headers))
        })
        .and_then(|bytes| bytes.checked_add(expanded_name_bytes))
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    if estimated_metadata_bytes > MAX_COMPUTED_GRID_LINE_METADATA_BYTES {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut tracks = Vec::new();
    reserve_integer_repeat_vec(
        &mut tracks,
        expanded_count,
        IntegerRepeatReservationSite::Tracks,
        node,
        property,
    )?;
    let mut line_names = Vec::new();
    reserve_integer_repeat_vec(
        &mut line_names,
        line_count,
        IntegerRepeatReservationSite::LineGroups,
        node,
        property,
    )?;
    line_names.push(Vec::new());

    for (index, value) in list.values.iter().enumerate() {
        append_line_names(&mut line_names, &list.line_names[index], node, property)?;
        match value {
            TrackListValue::TrackSize(size) => {
                tracks.push(
                    track_size(size, allow_intrinsic_tracks)
                        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?,
                );
                line_names.push(Vec::new());
            }
            TrackListValue::TrackRepeat(repeat) => {
                let RepeatCount::Number(count) = repeat.count else {
                    return Err(unsupported_grid_value(node, property, serialized));
                };
                let count = usize::try_from(count)
                    .ok()
                    .filter(|count| *count > 0)
                    .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
                for _ in 0..count {
                    for (track_index, size) in repeat.track_sizes.iter().enumerate() {
                        append_line_names(
                            &mut line_names,
                            &repeat.line_names[track_index],
                            node,
                            property,
                        )?;
                        tracks.push(
                            track_size(size, allow_intrinsic_tracks).ok_or_else(|| {
                                unsupported_grid_value(node, property, serialized)
                            })?,
                        );
                        line_names.push(Vec::new());
                    }
                    append_line_names(
                        &mut line_names,
                        repeat.line_names.last().expect("validated N+1 line names"),
                        node,
                        property,
                    )?;
                }
            }
        }
    }
    append_line_names(
        &mut line_names,
        list.line_names.last().expect("validated N+1 line names"),
        node,
        property,
    )?;
    debug_assert_eq!(tracks.len(), expanded_count);
    debug_assert_eq!(line_names.len(), line_count);
    Ok((tracks, line_names))
}

fn append_line_names(
    line_groups: &mut [Vec<String>],
    names: &[style::values::CustomIdent],
    node: NodeId,
    property: &'static str,
) -> Result<(), CssCascadeError> {
    let group = line_groups
        .last_mut()
        .expect("the initial grid line group is always present");
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

fn add_line_name_budget(
    name_count: &mut usize,
    name_bytes: &mut usize,
    names: &[style::values::CustomIdent],
    repeat_count: usize,
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<(), CssCascadeError> {
    let mut group_count = 0_usize;
    let mut group_bytes = 0_usize;
    for name in names {
        group_count = group_count
            .checked_add(1)
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
        group_bytes = group_bytes
            .checked_add(name.0.as_str().len())
            .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    }
    let repeated_count = group_count
        .checked_mul(repeat_count)
        .and_then(|count| name_count.checked_add(count))
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    let repeated_bytes = group_bytes
        .checked_mul(repeat_count)
        .and_then(|bytes| name_bytes.checked_add(bytes))
        .ok_or_else(|| unsupported_grid_value(node, property, serialized))?;
    *name_count = repeated_count;
    *name_bytes = repeated_bytes;
    Ok(())
}

fn reserve_integer_repeat_vec<T>(
    values: &mut Vec<T>,
    count: usize,
    site: IntegerRepeatReservationSite,
    node: NodeId,
    property: &'static str,
) -> Result<(), CssCascadeError> {
    fail_integer_repeat_reservation_if_requested(site)
        .map_err(|_| CssCascadeError::GridAllocationFailure { node, property })?;
    values
        .try_reserve_exact(count)
        .map_err(|_| CssCascadeError::GridAllocationFailure { node, property })
}

fn reserve_integer_repeat_string(
    value: &mut String,
    count: usize,
    node: NodeId,
    property: &'static str,
) -> Result<(), CssCascadeError> {
    fail_integer_repeat_reservation_if_requested(IntegerRepeatReservationSite::LineNameText)
        .map_err(|_| CssCascadeError::GridAllocationFailure { node, property })?;
    value
        .try_reserve(count)
        .map_err(|_| CssCascadeError::GridAllocationFailure { node, property })
}

#[cfg(test)]
fn fail_integer_repeat_reservation_if_requested(
    site: IntegerRepeatReservationSite,
) -> Result<(), ()> {
    let should_fail = INTEGER_REPEAT_FAILURE_SITE
        .try_with(|current| {
            if current.get() == Some(site) {
                current.set(None);
                true
            } else {
                false
            }
        })
        .unwrap_or(false);
    if should_fail { Err(()) } else { Ok(()) }
}

#[cfg(not(test))]
fn fail_integer_repeat_reservation_if_requested(
    _site: IntegerRepeatReservationSite,
) -> Result<(), ()> {
    Ok(())
}

fn reserve_computed_tracks(
    tracks: &mut Vec<ComputedCssGridTrackSize>,
    count: usize,
    node: NodeId,
    property: &'static str,
    reserve: impl FnOnce(&mut Vec<ComputedCssGridTrackSize>, usize) -> Result<(), ()>,
) -> Result<(), CssCascadeError> {
    reserve(tracks, count).map_err(|_| CssCascadeError::GridAllocationFailure { node, property })
}

fn track_size(size: &TrackSize, allow_intrinsic: bool) -> Option<ComputedCssGridTrackSize> {
    let track = match size {
        TrackSize::Breadth(breadth) => match track_breadth(breadth)? {
            ComputedCssGridTrackBreadth::LengthPx(value) => {
                ComputedCssGridTrackSize::LengthPx(value)
            }
            ComputedCssGridTrackBreadth::Percentage(value) => {
                ComputedCssGridTrackSize::Percentage(value)
            }
            ComputedCssGridTrackBreadth::Calc(value) => ComputedCssGridTrackSize::Calc(value),
            ComputedCssGridTrackBreadth::Fraction(value) => {
                ComputedCssGridTrackSize::Fraction(value)
            }
            ComputedCssGridTrackBreadth::Auto if allow_intrinsic => ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackBreadth::MinContent if allow_intrinsic => {
                ComputedCssGridTrackSize::MinContent
            }
            ComputedCssGridTrackBreadth::MaxContent if allow_intrinsic => {
                ComputedCssGridTrackSize::MaxContent
            }
            _ => return None,
        },
        TrackSize::Minmax(min, max) if allow_intrinsic => {
            let min = track_breadth(min)?;
            if matches!(min, ComputedCssGridTrackBreadth::Fraction(_)) {
                return None;
            }
            ComputedCssGridTrackSize::MinMax {
                min,
                max: track_breadth(max)?,
            }
        }
        TrackSize::FitContent(value) if allow_intrinsic => {
            let TrackBreadth::Breadth(value) = value else {
                return None;
            };
            ComputedCssGridTrackSize::FitContent(track_length_percentage(value)?)
        }
        TrackSize::Minmax(_, _) | TrackSize::FitContent(_) => return None,
    };
    track_size_is_valid(&track).then_some(track)
}

fn track_breadth(value: &TrackBreadth) -> Option<ComputedCssGridTrackBreadth> {
    match value {
        TrackBreadth::Breadth(value) => match value.unpack() {
            LengthPercentageValue::Length(length) => {
                Some(ComputedCssGridTrackBreadth::LengthPx(length.px()))
            }
            LengthPercentageValue::Percentage(percentage) => {
                Some(ComputedCssGridTrackBreadth::Percentage(percentage.0))
            }
            LengthPercentageValue::Calc(calc) => {
                Some(ComputedCssGridTrackBreadth::Calc(computed_grid_calc(calc)?))
            }
        },
        TrackBreadth::Flex(value) => Some(ComputedCssGridTrackBreadth::Fraction(value.0)),
        TrackBreadth::Auto => Some(ComputedCssGridTrackBreadth::Auto),
        TrackBreadth::MinContent => Some(ComputedCssGridTrackBreadth::MinContent),
        TrackBreadth::MaxContent => Some(ComputedCssGridTrackBreadth::MaxContent),
    }
}

fn track_length_percentage(
    value: &style::values::computed::LengthPercentage,
) -> Option<ComputedCssGridTrackLengthPercentage> {
    match value.unpack() {
        LengthPercentageValue::Length(length) => {
            Some(ComputedCssGridTrackLengthPercentage::LengthPx(length.px()))
        }
        LengthPercentageValue::Percentage(percentage) => Some(
            ComputedCssGridTrackLengthPercentage::Percentage(percentage.0),
        ),
        LengthPercentageValue::Calc(calc) => Some(ComputedCssGridTrackLengthPercentage::Calc(
            computed_grid_calc(calc)?,
        )),
    }
}

fn computed_grid_calc(calc: &impl ToTyped) -> Option<super::super::ComputedCssMath> {
    let TypedValue::Numeric(value) = calc.to_typed_value()? else {
        return None;
    };
    super::dimensions::computed_math(&value, 0, &mut 256)
}

fn track_size_is_valid(track: &ComputedCssGridTrackSize) -> bool {
    fn breadth_is_valid(value: &ComputedCssGridTrackBreadth) -> bool {
        match value {
            ComputedCssGridTrackBreadth::LengthPx(value)
            | ComputedCssGridTrackBreadth::Percentage(value) => value.is_finite() && *value >= 0.0,
            ComputedCssGridTrackBreadth::Fraction(value) => value.is_finite() && *value >= 0.0,
            ComputedCssGridTrackBreadth::Calc(_) => true,
            ComputedCssGridTrackBreadth::Auto
            | ComputedCssGridTrackBreadth::MinContent
            | ComputedCssGridTrackBreadth::MaxContent => true,
        }
    }
    match track {
        ComputedCssGridTrackSize::LengthPx(value) | ComputedCssGridTrackSize::Percentage(value) => {
            value.is_finite() && *value >= 0.0
        }
        ComputedCssGridTrackSize::Fraction(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridTrackSize::Calc(_) => true,
        ComputedCssGridTrackSize::Auto
        | ComputedCssGridTrackSize::MinContent
        | ComputedCssGridTrackSize::MaxContent => true,
        ComputedCssGridTrackSize::MinMax { min, max } => {
            breadth_is_valid(min) && breadth_is_valid(max)
        }
        ComputedCssGridTrackSize::FitContent(value) => match value {
            ComputedCssGridTrackLengthPercentage::LengthPx(value)
            | ComputedCssGridTrackLengthPercentage::Percentage(value) => {
                value.is_finite() && *value >= 0.0
            }
            ComputedCssGridTrackLengthPercentage::Calc(_) => true,
        },
    }
}

fn auto_repeat_track_has_fixed_minimum(track: &ComputedCssGridTrackSize) -> bool {
    match track {
        ComputedCssGridTrackSize::LengthPx(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(value),
            max: ComputedCssGridTrackBreadth::LengthPx(_) | ComputedCssGridTrackBreadth::Fraction(_),
        } => value.is_finite() && *value >= 0.0,
        _ => false,
    }
}

fn property_value<'a>(properties: &'a BTreeMap<String, String>, property: &str) -> &'a str {
    properties
        .get(property)
        .map(String::as_str)
        .unwrap_or("<계산값 없음>")
}

fn unsupported_grid_value(node: NodeId, property: &'static str, value: &str) -> CssCascadeError {
    CssCascadeError::UnsupportedComputedGridValue {
        node,
        property,
        value: value.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::reserve_computed_tracks;

    #[test]
    fn track_reservation_failure_keeps_node_and_property_context() {
        let mut tracks = Vec::new();
        let error = reserve_computed_tracks(
            &mut tracks,
            4,
            spinon_core::NodeId::new(6059).unwrap(),
            "grid-template-columns",
            |_, _| Err(()),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            crate::stylo_dom::cascade::CssCascadeError::GridAllocationFailure {
                node,
                property: "grid-template-columns"
            } if node == spinon_core::NodeId::new(6059).unwrap()
        ));
        assert!(tracks.is_empty());
    }
}
