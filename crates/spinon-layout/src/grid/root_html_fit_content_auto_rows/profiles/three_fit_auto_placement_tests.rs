use super::*;

fn three_fit_tracks(positions: [usize; 3], tail: [GridTrackSize; 3]) -> Vec<GridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(position as f32))
            } else {
                tail.next().expect("three non-fit members")
            }
        })
        .collect()
}

fn tail_orders() -> [[GridTrackSize; 3]; 6] {
    [
        [
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
        [
            GridTrackSize::MinContent,
            GridTrackSize::Auto,
            GridTrackSize::MaxContent,
        ],
        [
            GridTrackSize::MaxContent,
            GridTrackSize::MinContent,
            GridTrackSize::Auto,
        ],
        [
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
            GridTrackSize::MinContent,
        ],
        [
            GridTrackSize::Auto,
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
        ],
        [
            GridTrackSize::Auto,
            GridTrackSize::MaxContent,
            GridTrackSize::MinContent,
        ],
    ]
}

#[test]
fn repeated_three_fit_auto_placement_accepts_all_positions_and_tail_orders() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for tail in tail_orders() {
                    let tracks = three_fit_tracks([first, second, third], tail);
                    assert!(
                        supports_repeated_three_fit_min_max_content_auto_placement_track_list(
                            &tracks
                        ),
                        "positions=({first}, {second}, {third}), tail={tail:?}"
                    );
                    assert!(supports_repeated_min_max_content_auto_placement_track_list(
                        &tracks
                    ));
                    assert!(supports_intrinsic_auto_track_list(&tracks));
                    assert!(supports_repeated_span_fit_positions(&tracks, 6));
                    combinations += 1;
                }
            }
        }
    }
    assert_eq!(combinations, 120, "20 position triples × 6 tail orders");
}

#[test]
fn repeated_three_fit_auto_placement_rejects_wrong_count_multiplicity_and_caps() {
    let valid_tail = [
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
    ];
    let valid = three_fit_tracks([1, 2, 3], valid_tail);
    assert!(supports_repeated_three_fit_min_max_content_auto_placement_track_list(&valid));

    for fit_count in [0, 1, 2, 4] {
        let mut tracks = vec![GridTrackSize::MinContent; 6];
        for (position, track) in tracks.iter_mut().enumerate().take(fit_count) {
            *track =
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(position as f32));
        }
        assert!(
            !supports_repeated_three_fit_min_max_content_auto_placement_track_list(&tracks),
            "fit count {fit_count} belongs to another profile"
        );
    }

    let mut wrong_multiplicity = valid.clone();
    wrong_multiplicity[5] = GridTrackSize::MinContent;
    assert!(
        !supports_repeated_three_fit_min_max_content_auto_placement_track_list(&wrong_multiplicity)
    );
    assert!(!supports_repeated_three_fit_min_max_content_auto_placement_track_list(&valid[..5]));
    let mut too_long = valid.clone();
    too_long.push(GridTrackSize::Auto);
    assert!(!supports_repeated_three_fit_min_max_content_auto_placement_track_list(&too_long));

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
        assert!(!supports_repeated_three_fit_min_max_content_auto_placement_track_list(&invalid));
    }
}
