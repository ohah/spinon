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
fn repeated_span_and_auto_placement_accept_all_three_fit_positions_and_tail_permutations() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for tail in tail_orders() {
                    let tracks = three_fit_tracks([first, second, third], tail);
                    assert!(supports_repeated_min_max_content_auto_span_track_list(
                        &tracks
                    ));
                    assert!(supports_repeated_span_fit_positions(&tracks, 6));
                    assert!(supports_intrinsic_auto_track_list(&tracks));
                    assert!(!supports_repeated_span_fit_positions(&tracks, 3));
                    assert!(supports_repeated_min_max_content_auto_placement_track_list(
                        &tracks
                    ));
                    combinations += 1;
                }
            }
        }
    }
    assert_eq!(combinations, 120, "20 position triples × 6 tail orders");
}

#[test]
fn repeated_span_three_fit_rejects_wrong_multiplicity_and_caps() {
    let mut four_fit = three_fit_tracks(
        [1, 2, 3],
        [
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
    );
    four_fit[4] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(4.0));
    assert!(!supports_repeated_min_max_content_auto_span_track_list(
        &four_fit
    ));
    assert!(!supports_repeated_span_fit_positions(&four_fit, 6));

    let mut wrong_tail = three_fit_tracks(
        [1, 2, 3],
        [
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
    );
    wrong_tail[5] = GridTrackSize::MinContent;
    assert!(!supports_repeated_min_max_content_auto_span_track_list(
        &wrong_tail
    ));

    for cap in [
        GridTrackLengthPercentage::LengthPx(-1.0),
        GridTrackLengthPercentage::LengthPx(f32::INFINITY),
        GridTrackLengthPercentage::Percentage(f32::NAN),
        GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)),
    ] {
        let mut invalid = three_fit_tracks(
            [1, 2, 3],
            [
                GridTrackSize::MinContent,
                GridTrackSize::MaxContent,
                GridTrackSize::Auto,
            ],
        );
        invalid[1] = GridTrackSize::FitContent(cap);
        assert!(!supports_repeated_min_max_content_auto_span_track_list(
            &invalid
        ));
    }
}
