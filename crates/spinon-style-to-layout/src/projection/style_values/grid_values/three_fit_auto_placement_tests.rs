use super::*;

fn three_fit_tracks(
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
                tail.next().expect("three non-fit members")
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

#[test]
fn three_fit_auto_placement_accepts_all_positions_and_tail_orders() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for tail in tail_orders() {
                    let tracks = three_fit_tracks([first, second, third], tail.clone());
                    assert!(
                        root_html_fit_content_repeated_three_fit_auto_placement_track_list(&tracks),
                        "positions=({first}, {second}, {third}), tail={tail:?}"
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
    assert_eq!(combinations, 120, "20 position triples × 6 tail orders");
}

#[test]
fn three_fit_auto_placement_rejects_wrong_count_multiplicity_length_and_caps() {
    let valid_tail = [
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
    ];
    let valid = three_fit_tracks([1, 2, 3], valid_tail);
    assert!(root_html_fit_content_repeated_three_fit_auto_placement_track_list(&valid));

    for fit_count in [0, 1, 2, 4] {
        let mut tracks = vec![ComputedCssGridTrackSize::MinContent; 6];
        for (position, track) in tracks.iter_mut().enumerate().take(fit_count) {
            *track = ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::LengthPx(position as f32),
            );
        }
        assert!(
            !root_html_fit_content_repeated_three_fit_auto_placement_track_list(&tracks),
            "fit count {fit_count} belongs to another profile"
        );
    }

    let mut wrong_multiplicity = valid.clone();
    wrong_multiplicity[5] = ComputedCssGridTrackSize::MinContent;
    assert!(
        !root_html_fit_content_repeated_three_fit_auto_placement_track_list(&wrong_multiplicity)
    );
    assert!(!root_html_fit_content_repeated_three_fit_auto_placement_track_list(&valid[..5]));
    let mut too_long = valid.clone();
    too_long.push(ComputedCssGridTrackSize::Auto);
    assert!(!root_html_fit_content_repeated_three_fit_auto_placement_track_list(&too_long));

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
        assert!(!root_html_fit_content_repeated_three_fit_auto_placement_track_list(&invalid));
    }
}
