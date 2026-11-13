use super::*;

fn tracks(
    positions: [usize; 3],
    tail: [ComputedCssGridTrackSize; 3],
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                ComputedCssGridTrackSize::FitContent(
                    ComputedCssGridTrackLengthPercentage::LengthPx(position as f32),
                )
            } else {
                tail.next().expect("three intrinsic tail tracks")
            }
        })
        .collect()
}

fn tail_orders() -> [[ComputedCssGridTrackSize; 3]; 6] {
    use ComputedCssGridTrackSize::{Auto, MaxContent, MinContent};
    [
        [MinContent, MaxContent, Auto],
        [MinContent, Auto, MaxContent],
        [MaxContent, MinContent, Auto],
        [MaxContent, Auto, MinContent],
        [Auto, MinContent, MaxContent],
        [Auto, MaxContent, MinContent],
    ]
}

fn three_fit_cap_orders() -> [[ComputedCssGridTrackSize; 3]; 6] {
    use ComputedCssGridTrackLengthPercentage::{LengthPx, Percentage};
    use ComputedCssGridTrackSize::FitContent;
    [
        [
            FitContent(LengthPx(0.0)),
            FitContent(LengthPx(10.0)),
            FitContent(Percentage(0.2)),
        ],
        [
            FitContent(LengthPx(0.0)),
            FitContent(Percentage(0.2)),
            FitContent(LengthPx(10.0)),
        ],
        [
            FitContent(LengthPx(10.0)),
            FitContent(LengthPx(0.0)),
            FitContent(Percentage(0.2)),
        ],
        [
            FitContent(LengthPx(10.0)),
            FitContent(Percentage(0.2)),
            FitContent(LengthPx(0.0)),
        ],
        [
            FitContent(Percentage(0.2)),
            FitContent(LengthPx(0.0)),
            FitContent(LengthPx(10.0)),
        ],
        [
            FitContent(Percentage(0.2)),
            FitContent(LengthPx(10.0)),
            FitContent(LengthPx(0.0)),
        ],
    ]
}

fn three_fit_mixed_tracks(
    positions: [usize; 3],
    caps: [ComputedCssGridTrackSize; 3],
    tail: [ComputedCssGridTrackSize; 3],
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut caps = caps.into_iter();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                caps.next().expect("one cap per fit position")
            } else {
                tail.next().expect("three intrinsic tail tracks")
            }
        })
        .collect()
}

#[test]
fn three_fit_row_span_and_auto_placement_accept_all_positions() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for tail in tail_orders() {
                    let tracks = tracks([first, second, third], tail);
                    assert!(
                        root_html_fit_content_repeated_min_max_content_auto_span_track_list(
                            &tracks
                        )
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
    assert_eq!(combinations, 120);
}

#[test]
fn three_fit_mixed_minimum_profile_accepts_exact_caps_tails_and_positions_only() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for caps in three_fit_cap_orders() {
                    for tail in tail_orders() {
                        let tracks =
                            three_fit_mixed_tracks([first, second, third], caps.clone(), tail);
                        assert!(three_fit_mixed_row_span_track_list(&tracks));
                        combinations += 1;
                    }
                }
            }
        }
    }
    assert_eq!(combinations, 720);

    let mut wrong_cap = three_fit_mixed_tracks(
        [1, 2, 3],
        three_fit_cap_orders()[0].clone(),
        tail_orders()[0].clone(),
    );
    wrong_cap[0] =
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(1.0));
    assert!(!three_fit_mixed_row_span_track_list(&wrong_cap));

    let mut duplicate_cap = three_fit_mixed_tracks(
        [1, 2, 3],
        three_fit_cap_orders()[0].clone(),
        tail_orders()[0].clone(),
    );
    duplicate_cap[1] =
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(0.0));
    assert!(!three_fit_mixed_row_span_track_list(&duplicate_cap));

    let mut wrong_tail = three_fit_mixed_tracks(
        [1, 2, 3],
        three_fit_cap_orders()[0].clone(),
        tail_orders()[0].clone(),
    );
    wrong_tail[5] = ComputedCssGridTrackSize::MaxContent;
    assert!(!three_fit_mixed_row_span_track_list(&wrong_tail));
}
