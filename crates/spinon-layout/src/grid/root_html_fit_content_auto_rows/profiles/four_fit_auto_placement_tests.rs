use super::*;

fn four_fit_tracks(positions: [usize; 4], tail: [GridTrackSize; 2]) -> Vec<GridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                let value = position as f32;
                let cap = if position % 2 == 0 {
                    GridTrackLengthPercentage::Percentage(value)
                } else {
                    GridTrackLengthPercentage::LengthPx(value)
                };
                GridTrackSize::FitContent(cap)
            } else {
                tail.next().expect("two non-fit members")
            }
        })
        .collect()
}

fn tail_orders() -> [[GridTrackSize; 2]; 2] {
    [
        [GridTrackSize::MinContent, GridTrackSize::MaxContent],
        [GridTrackSize::MaxContent, GridTrackSize::MinContent],
    ]
}

#[test]
fn repeated_four_fit_auto_placement_accepts_all_positions_and_tail_orders() {
    let mut combinations = 0;
    for first in 1..=3 {
        for second in first + 1..=4 {
            for third in second + 1..=5 {
                for fourth in third + 1..=6 {
                    for tail in tail_orders() {
                        let tracks = four_fit_tracks([first, second, third, fourth], tail);
                        assert!(
                            supports_repeated_four_fit_min_max_content_auto_placement_track_list(
                                &tracks
                            ),
                            "positions=({first}, {second}, {third}, {fourth}), tail={tail:?}"
                        );
                        assert!(supports_repeated_min_max_content_auto_placement_track_list(
                            &tracks
                        ));
                        assert!(supports_intrinsic_auto_track_list(&tracks));
                        assert!(!supports_repeated_min_max_content_auto_span_track_list(
                            &tracks
                        ));
                        combinations += 1;
                    }
                }
            }
        }
    }
    assert_eq!(combinations, 30, "15 position quadruples × 2 tail orders");
}

#[test]
fn repeated_four_fit_auto_placement_rejects_wrong_counts_tails_lengths_and_caps() {
    use GridTrackSize::{Auto, MaxContent, MinContent};

    let valid = four_fit_tracks([1, 2, 3, 4], [MinContent, MaxContent]);
    assert!(supports_repeated_four_fit_min_max_content_auto_placement_track_list(&valid));

    for fit_count in [0, 1, 2, 3, 5, 6] {
        let mut tracks = vec![MinContent; 6];
        for (position, track) in tracks.iter_mut().enumerate().take(fit_count) {
            *track =
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(position as f32));
        }
        assert!(
            !supports_repeated_four_fit_min_max_content_auto_placement_track_list(&tracks),
            "fit count {fit_count} belongs to another profile"
        );
    }

    let wrong_tail = four_fit_tracks([1, 2, 3, 4], [MinContent, MinContent]);
    assert!(!supports_repeated_four_fit_min_max_content_auto_placement_track_list(&wrong_tail));
    let wrong_tail_kind = four_fit_tracks([1, 2, 3, 4], [MinContent, Auto]);
    assert!(
        !supports_repeated_four_fit_min_max_content_auto_placement_track_list(&wrong_tail_kind)
    );
    assert!(!supports_repeated_four_fit_min_max_content_auto_placement_track_list(&valid[..5]));
    let mut too_long = valid.clone();
    too_long.push(MinContent);
    assert!(!supports_repeated_four_fit_min_max_content_auto_placement_track_list(&too_long));

    for cap in [
        GridTrackLengthPercentage::LengthPx(-1.0),
        GridTrackLengthPercentage::LengthPx(f32::NAN),
        GridTrackLengthPercentage::LengthPx(f32::INFINITY),
        GridTrackLengthPercentage::Percentage(-0.1),
        GridTrackLengthPercentage::Percentage(f32::NAN),
        GridTrackLengthPercentage::Percentage(f32::INFINITY),
        GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)),
    ] {
        let mut invalid = valid.clone();
        invalid[1] = GridTrackSize::FitContent(cap);
        assert!(!supports_repeated_four_fit_min_max_content_auto_placement_track_list(&invalid));
    }
}
