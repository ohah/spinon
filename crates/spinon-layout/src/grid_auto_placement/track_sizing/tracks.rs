use crate::{GridAutoTrackSize, GridTrackSize, LayoutError, grid::GRID_MAX_TRACKS};

use super::super::reservation::{GridReservationSite, reserve_exact};

pub(super) fn expand_tracks(
    node: spinon_core::NodeId,
    property: &'static str,
    explicit: &[GridTrackSize],
    leading_count: usize,
    trailing_count: usize,
    auto_size: &GridAutoTrackSize,
    rows: bool,
) -> Result<Option<Vec<GridTrackSize>>, LayoutError> {
    let count = leading_count
        .checked_add(explicit.len())
        .and_then(|count| count.checked_add(trailing_count))
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    validate_count(node, property, count)?;
    if leading_count == 0 && trailing_count == 0 {
        return Ok(None);
    }

    let pattern = match auto_size {
        GridAutoTrackSize::Auto => TrackPattern::Single(GridTrackSize::Auto),
        GridAutoTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0 => {
            TrackPattern::Single(GridTrackSize::LengthPx(*value))
        }
        GridAutoTrackSize::LengthPx(_) => {
            return Err(LayoutError::InvalidGridTrack { node, property });
        }
        GridAutoTrackSize::TrackList(tracks) if !tracks.is_empty() => TrackPattern::List(tracks),
        GridAutoTrackSize::TrackList(_) => {
            return if rows {
                Err(LayoutError::UnsupportedGridAutoRows {
                    node,
                    reason: "grid-auto-rows track-size 목록은 비어 있을 수 없습니다",
                })
            } else {
                Err(LayoutError::UnsupportedGridAutoColumns {
                    node,
                    reason: "grid-auto-columns track-size 목록은 비어 있을 수 없습니다",
                })
            };
        }
    };

    let mut tracks = Vec::new();
    reserve_exact(
        &mut tracks,
        count,
        node,
        property,
        if rows {
            GridReservationSite::ExpandedRows
        } else {
            GridReservationSite::ExpandedColumns
        },
    )?;
    for index in 0..leading_count {
        let track = match pattern {
            TrackPattern::Single(track) => track,
            TrackPattern::List(pattern) => {
                let distance_from_explicit = leading_count - index;
                let remainder = distance_from_explicit % pattern.len();
                pattern[(pattern.len() - remainder) % pattern.len()]
            }
        };
        tracks.push(track);
    }
    tracks.extend_from_slice(explicit);
    for index in 0..trailing_count {
        let track = match pattern {
            TrackPattern::Single(track) => track,
            TrackPattern::List(pattern) => pattern[index % pattern.len()],
        };
        tracks.push(track);
    }
    Ok(Some(tracks))
}

fn validate_count(
    node: spinon_core::NodeId,
    property: &'static str,
    count: usize,
) -> Result<(), LayoutError> {
    if count > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count,
            maximum: GRID_MAX_TRACKS,
        });
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum TrackPattern<'a> {
    Single(GridTrackSize),
    List(&'a [GridTrackSize]),
}
