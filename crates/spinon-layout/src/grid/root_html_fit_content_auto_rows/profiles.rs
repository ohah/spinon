use super::*;

pub(super) fn fit_content_cap_is_supported(track: &GridTrackSize) -> bool {
    matches!(
        track,
        GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(value)
                | GridTrackLengthPercentage::Percentage(value)
        ) if value.is_finite() && *value >= 0.0
    )
}

pub(super) fn supports_min_content_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    matches!(
        tracks,
        [fit, GridTrackSize::MinContent, GridTrackSize::Auto]
            | [fit, GridTrackSize::Auto, GridTrackSize::MinContent]
            | [GridTrackSize::MinContent, fit, GridTrackSize::Auto]
            | [GridTrackSize::MinContent, GridTrackSize::Auto, fit]
            | [GridTrackSize::Auto, fit, GridTrackSize::MinContent]
            | [GridTrackSize::Auto, GridTrackSize::MinContent, fit]
            if fit_content_cap_is_supported(fit)
    )
}

pub(super) fn supports_min_max_content_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    if tracks.len() != 4 {
        return false;
    }

    let mut members = [false; 4];
    for track in tracks {
        let member = match track {
            track @ GridTrackSize::FitContent(_) if fit_content_cap_is_supported(track) => 0,
            GridTrackSize::MinContent => 1,
            GridTrackSize::MaxContent => 2,
            GridTrackSize::Auto => 3,
            _ => return false,
        };
        if std::mem::replace(&mut members[member], true) {
            return false;
        }
    }
    members.into_iter().all(|present| present)
}

pub(super) fn supports_repeated_min_max_content_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    if tracks.len() != 6 {
        return false;
    }

    let mut counts = [0usize; 3];
    let mut fit_content_seen = false;
    for track in tracks {
        if matches!(track, GridTrackSize::FitContent(_)) {
            if fit_content_seen || !fit_content_cap_is_supported(track) {
                return false;
            }
            fit_content_seen = true;
            continue;
        }
        let index = match track {
            GridTrackSize::MinContent => 0,
            GridTrackSize::MaxContent => 1,
            GridTrackSize::Auto => 2,
            _ => return false,
        };
        counts[index] += 1;
    }
    fit_content_seen && counts == [2, 2, 1]
}

pub(super) fn supports_repeated_min_max_content_auto_span_track_list(
    tracks: &[GridTrackSize],
) -> bool {
    if tracks.len() != 6 {
        return false;
    }

    let mut counts = [0usize; 3];
    let mut fit_content_seen = false;
    for (position, track) in tracks.iter().enumerate() {
        if matches!(track, GridTrackSize::FitContent(_)) {
            if position >= 3 || fit_content_seen || !fit_content_cap_is_supported(track) {
                return false;
            }
            fit_content_seen = true;
            continue;
        }
        let index = match track {
            GridTrackSize::MinContent => 0,
            GridTrackSize::MaxContent => 1,
            GridTrackSize::Auto => 2,
            _ => return false,
        };
        counts[index] += 1;
    }
    fit_content_seen && counts == [2, 2, 1]
}

pub(super) fn supports_intrinsic_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    supports_min_content_auto_track_list(tracks)
        || supports_min_max_content_auto_track_list(tracks)
        || supports_repeated_min_max_content_auto_track_list(tracks)
        || supports_repeated_min_max_content_auto_span_track_list(tracks)
}

pub(super) fn supports_track_list(tracks: &[GridTrackSize]) -> bool {
    !tracks.is_empty()
        && tracks.len() <= GRID_MAX_TRACKS
        && tracks
            .iter()
            .any(|track| matches!(track, GridTrackSize::FitContent(_)))
        && if tracks
            .iter()
            .any(|track| matches!(track, GridTrackSize::MinContent | GridTrackSize::MaxContent))
        {
            supports_intrinsic_auto_track_list(tracks)
        } else {
            tracks.iter().all(|track| match track {
                GridTrackSize::Auto => true,
                GridTrackSize::LengthPx(value) | GridTrackSize::Percentage(value) => {
                    value.is_finite() && *value >= 0.0
                }
                track @ GridTrackSize::FitContent(_) => fit_content_cap_is_supported(track),
                _ => false,
            })
        }
}
