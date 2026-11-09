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
    let mut fit_content_count = 0usize;
    let mut tail_order = Vec::with_capacity(4);
    for track in tracks {
        if matches!(track, GridTrackSize::FitContent(_)) {
            if !fit_content_cap_is_supported(track) {
                return false;
            }
            fit_content_count += 1;
            continue;
        }
        let index = match track {
            GridTrackSize::MinContent => {
                tail_order.push(0);
                0
            }
            GridTrackSize::MaxContent => {
                tail_order.push(1);
                1
            }
            GridTrackSize::Auto => {
                tail_order.push(2);
                2
            }
            _ => return false,
        };
        counts[index] += 1;
    }
    match fit_content_count {
        1 => counts == [2, 2, 1],
        2 => {
            counts == [2, 1, 1]
                && matches!(
                    tail_order.as_slice(),
                    [0, 0, 1, 2] | [0, 1, 0, 2] | [1, 2, 0, 0] | [2, 0, 1, 0]
                )
        }
        _ => false,
    }
}

pub(super) fn supports_repeated_span_fit_positions(
    tracks: &[GridTrackSize],
    positive_implicit_rows: usize,
) -> bool {
    if !supports_repeated_min_max_content_auto_span_track_list(tracks) {
        return false;
    }
    let positions = tracks
        .iter()
        .enumerate()
        .filter_map(|(index, track)| {
            matches!(track, GridTrackSize::FitContent(_)).then_some(index + 1)
        })
        .collect::<Vec<_>>();
    match positions.as_slice() {
        [position] => match positive_implicit_rows {
            3 => (1..=3).contains(position),
            6 => (4..=6).contains(position),
            _ => false,
        },
        [first, second] => {
            positive_implicit_rows == 6
                && (1..=6).contains(first)
                && (1..=6).contains(second)
                && first < second
        }
        _ => false,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn two_fit_tracks(first: usize, second: usize, tail: [GridTrackSize; 4]) -> Vec<GridTrackSize> {
        let mut tracks = Vec::with_capacity(6);
        let mut tail = tail.into_iter();
        for position in 1..=6 {
            if position == first || position == second {
                tracks.push(GridTrackSize::FitContent(
                    GridTrackLengthPercentage::LengthPx(position as f32),
                ));
            } else {
                tracks.push(tail.next().expect("four non-fit tracks"));
            }
        }
        tracks
    }

    #[test]
    fn repeated_span_accepts_all_two_fit_positions_and_only_the_fixed_tail_orders() {
        let tails = [
            [
                GridTrackSize::MinContent,
                GridTrackSize::MinContent,
                GridTrackSize::MaxContent,
                GridTrackSize::Auto,
            ],
            [
                GridTrackSize::MinContent,
                GridTrackSize::MaxContent,
                GridTrackSize::MinContent,
                GridTrackSize::Auto,
            ],
            [
                GridTrackSize::MaxContent,
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
                GridTrackSize::MinContent,
            ],
            [
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
                GridTrackSize::MaxContent,
                GridTrackSize::MinContent,
            ],
        ];
        for tail in tails {
            for first in 1..=6 {
                for second in (first + 1)..=6 {
                    let tracks = two_fit_tracks(first, second, tail);
                    assert!(supports_repeated_min_max_content_auto_span_track_list(
                        &tracks
                    ));
                    assert!(supports_repeated_span_fit_positions(&tracks, 6));
                    assert!(!supports_repeated_span_fit_positions(&tracks, 3));
                    assert!(!supports_repeated_min_max_content_auto_track_list(&tracks));
                }
            }
        }
    }

    #[test]
    fn repeated_span_keeps_single_fit_positions_and_rejects_wrong_fit_counts_or_tail_order() {
        let mut single = vec![GridTrackSize::MinContent; 0];
        for position in 1..=6 {
            single.push(if position == 4 {
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0))
            } else {
                match position {
                    1 | 2 => GridTrackSize::MinContent,
                    3 | 5 => GridTrackSize::MaxContent,
                    6 => GridTrackSize::Auto,
                    _ => unreachable!(),
                }
            });
        }
        assert!(supports_repeated_min_max_content_auto_span_track_list(
            &single
        ));
        assert!(supports_repeated_span_fit_positions(&single, 6));

        let mut bad_tail = two_fit_tracks(
            1,
            2,
            [
                GridTrackSize::MinContent,
                GridTrackSize::MaxContent,
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
            ],
        );
        assert!(!supports_repeated_min_max_content_auto_span_track_list(
            &bad_tail
        ));

        bad_tail[2] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(3.0));
        assert!(!supports_repeated_min_max_content_auto_span_track_list(
            &bad_tail
        ));
        assert!(!supports_repeated_span_fit_positions(&bad_tail, 6));
    }
}
