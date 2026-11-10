use super::*;

fn four_fit_tracks(
    positions: [usize; 4],
    tail: [ComputedCssGridTrackSize; 2],
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                let value = position as f32;
                let cap = if position % 2 == 0 {
                    ComputedCssGridTrackLengthPercentage::Percentage(value)
                } else {
                    ComputedCssGridTrackLengthPercentage::LengthPx(value)
                };
                ComputedCssGridTrackSize::FitContent(cap)
            } else {
                tail.next().expect("two non-fit members")
            }
        })
        .collect()
}

fn tail_orders() -> [[ComputedCssGridTrackSize; 2]; 2] {
    use ComputedCssGridTrackSize::{MaxContent, MinContent};
    [[MinContent, MaxContent], [MaxContent, MinContent]]
}

#[test]
fn four_fit_auto_placement_accepts_all_positions_and_tail_orders() {
    let mut combinations = 0;
    for first in 1..=3 {
        for second in first + 1..=4 {
            for third in second + 1..=5 {
                for fourth in third + 1..=6 {
                    for tail in tail_orders() {
                        let tracks = four_fit_tracks([first, second, third, fourth], tail.clone());
                        assert!(
                            root_html_fit_content_repeated_four_fit_auto_placement_track_list(
                                &tracks
                            ),
                            "positions=({first}, {second}, {third}, {fourth}), tail={tail:?}"
                        );
                        assert!(
                            root_html_fit_content_repeated_min_max_content_auto_placement_track_list(
                                &tracks
                            )
                        );
                        combinations += 1;
                    }
                }
            }
        }
    }
    assert_eq!(combinations, 30, "15 position quadruples × 2 tail orders");
}

#[test]
fn four_fit_auto_placement_rejects_wrong_counts_tails_lengths_and_caps() {
    use ComputedCssGridTrackSize::{Auto, MaxContent, MinContent};

    let valid_tail = [MinContent, MaxContent];
    let valid = four_fit_tracks([1, 2, 3, 4], valid_tail);
    assert!(root_html_fit_content_repeated_four_fit_auto_placement_track_list(&valid));

    for fit_count in [0, 1, 2, 3, 5, 6] {
        let mut tracks = vec![MinContent; 6];
        for (position, track) in tracks.iter_mut().enumerate().take(fit_count) {
            *track = ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(position as f32),
            );
        }
        assert!(
            !root_html_fit_content_repeated_four_fit_auto_placement_track_list(&tracks),
            "fit count {fit_count} belongs to another profile"
        );
    }

    for invalid in [
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                1.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                2.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                3.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                4.0,
            )),
            MinContent,
            MinContent,
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                1.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                2.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                3.0,
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                4.0,
            )),
            MinContent,
            Auto,
        ],
    ] {
        assert!(!root_html_fit_content_repeated_four_fit_auto_placement_track_list(&invalid));
    }

    assert!(!root_html_fit_content_repeated_four_fit_auto_placement_track_list(&valid[..5]));
    let mut too_long = valid.clone();
    too_long.push(MinContent);
    assert!(!root_html_fit_content_repeated_four_fit_auto_placement_track_list(&too_long));

    for cap in [
        ComputedCssGridTrackLengthPercentage::LengthPx(-1.0),
        ComputedCssGridTrackLengthPercentage::LengthPx(f32::NAN),
        ComputedCssGridTrackLengthPercentage::LengthPx(f32::INFINITY),
        ComputedCssGridTrackLengthPercentage::Percentage(-0.1),
        ComputedCssGridTrackLengthPercentage::Percentage(f32::NAN),
        ComputedCssGridTrackLengthPercentage::Percentage(f32::INFINITY),
        ComputedCssGridTrackLengthPercentage::Calc(ComputedCssMath::LengthPx(1.0)),
    ] {
        let mut invalid = valid.clone();
        invalid[1] = ComputedCssGridTrackSize::FitContent(cap);
        assert!(!root_html_fit_content_repeated_four_fit_auto_placement_track_list(&invalid));
    }
}
