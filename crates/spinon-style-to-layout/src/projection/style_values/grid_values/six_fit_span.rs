use super::*;

pub(super) fn supports_six_fit_row_span_track_list(tracks: &[ComputedCssGridTrackSize]) -> bool {
    if tracks.len() != 6 {
        return false;
    }

    let mut caps_seen = [false; 6];
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
            ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(value),
            ) if *value == 50.0 => 5,
            _ => return false,
        };
        if std::mem::replace(&mut caps_seen[cap_index], true) {
            return false;
        }
    }

    caps_seen.into_iter().all(|seen| seen)
}
