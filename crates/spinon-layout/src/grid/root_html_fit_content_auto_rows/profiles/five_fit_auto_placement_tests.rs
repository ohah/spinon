use super::*;

fn five_fit_tracks(positions: [usize; 5], tail: GridTrackSize) -> Vec<GridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = Some(tail);
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
                tail.take().expect("one non-fit member")
            }
        })
        .collect()
}

fn tails() -> [GridTrackSize; 3] {
    [
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
    ]
}

#[test]
fn repeated_five_fit_auto_placement_accepts_every_position_and_tail() {
    let mut combinations = 0;
    for first in 1..=2 {
        for second in first + 1..=3 {
            for third in second + 1..=4 {
                for fourth in third + 1..=5 {
                    for fifth in fourth + 1..=6 {
                        for tail in tails() {
                            let tracks =
                                five_fit_tracks([first, second, third, fourth, fifth], tail);
                            assert!(
                                supports_repeated_five_fit_auto_placement_track_list(&tracks),
                                "positions=({first}, {second}, {third}, {fourth}, {fifth}), tail={tail:?}"
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
    }
    assert_eq!(combinations, 18, "six fit positions × three tail values");
}

#[test]
fn repeated_five_fit_auto_placement_rejects_wrong_counts_lengths_and_caps() {
    use GridTrackSize::{Auto, FitContent, MaxContent, MinContent};

    let valid = five_fit_tracks([1, 2, 3, 4, 5], GridTrackSize::MinContent);
    assert!(supports_repeated_five_fit_auto_placement_track_list(&valid));

    for fit_count in [0, 1, 2, 3, 4, 6] {
        let mut tracks = vec![MinContent; 6];
        for (position, track) in tracks.iter_mut().enumerate().take(fit_count) {
            *track = FitContent(GridTrackLengthPercentage::LengthPx(position as f32));
        }
        assert!(
            !supports_repeated_five_fit_auto_placement_track_list(&tracks),
            "fit count {fit_count} belongs to another profile"
        );
    }

    let wrong_tail = five_fit_tracks([1, 2, 3, 4, 5], MaxContent);
    let mut unsupported_tail = wrong_tail;
    unsupported_tail[5] = GridTrackSize::LengthPx(10.0);
    assert!(!supports_repeated_five_fit_auto_placement_track_list(
        &unsupported_tail
    ));

    assert!(!supports_repeated_five_fit_auto_placement_track_list(
        &valid[..5]
    ));
    let mut too_long = valid.clone();
    too_long.push(MinContent);
    assert!(!supports_repeated_five_fit_auto_placement_track_list(
        &too_long
    ));

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
        invalid[0] = FitContent(cap);
        assert!(!supports_repeated_five_fit_auto_placement_track_list(
            &invalid
        ));
    }

    let duplicate_tail = vec![
        FitContent(GridTrackLengthPercentage::LengthPx(1.0)),
        FitContent(GridTrackLengthPercentage::LengthPx(2.0)),
        FitContent(GridTrackLengthPercentage::LengthPx(3.0)),
        FitContent(GridTrackLengthPercentage::LengthPx(4.0)),
        MinContent,
        Auto,
    ];
    assert!(!supports_repeated_five_fit_auto_placement_track_list(
        &duplicate_tail
    ));
}
