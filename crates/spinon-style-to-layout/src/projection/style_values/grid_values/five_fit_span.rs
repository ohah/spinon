use super::*;

pub(super) fn supports_repeated_five_fit_row_span_track_list(
    tracks: &[ComputedCssGridTrackSize],
) -> bool {
    if tracks.len() != 6 {
        return false;
    }

    let mut caps_seen = [false; 5];
    let mut tail_count = 0;
    for track in tracks {
        let cap_index = match track {
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) if *value == 0.0 => 0,
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) if *value == 10.0 => 1,
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::Percentage(value),
            ) if *value == 0.2 => 2,
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) if *value == 30.0 => 3,
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) if *value == 40.0 => 4,
            ComputedCssGridTrackSize::FitContent(_) => return false,
            ComputedCssGridTrackSize::MinContent
            | ComputedCssGridTrackSize::MaxContent
            | ComputedCssGridTrackSize::Auto => {
                tail_count += 1;
                continue;
            }
            _ => return false,
        };
        if std::mem::replace(&mut caps_seen[cap_index], true) {
            return false;
        }
    }

    tail_count == 1 && caps_seen.into_iter().all(|seen| seen)
}
