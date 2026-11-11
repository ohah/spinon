use super::*;

fn cap(index: usize) -> GridTrackLengthPercentage {
    match index {
        0 => GridTrackLengthPercentage::LengthPx(0.0),
        1 => GridTrackLengthPercentage::LengthPx(10.0),
        2 => GridTrackLengthPercentage::Percentage(0.2),
        3 => GridTrackLengthPercentage::LengthPx(30.0),
        _ => unreachable!("four supported caps are registered"),
    }
}

fn cap_orders() -> Vec<[GridTrackLengthPercentage; 4]> {
    let mut orders = Vec::with_capacity(24);
    for first in 0..4 {
        for second in 0..4 {
            for third in 0..4 {
                for fourth in 0..4 {
                    if [first, second, third, fourth]
                        .into_iter()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        == 4
                    {
                        orders.push([cap(first), cap(second), cap(third), cap(fourth)]);
                    }
                }
            }
        }
    }
    orders
}

fn four_fit_tracks(
    positions: [usize; 4],
    caps: [GridTrackLengthPercentage; 4],
    tail: [GridTrackSize; 2],
) -> Vec<GridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut caps = caps.into_iter();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                GridTrackSize::FitContent(caps.next().expect("four fit caps"))
            } else {
                tail.next().expect("two intrinsic tail tracks")
            }
        })
        .collect()
}

#[test]
fn repeated_four_fit_row_span_accepts_all_fit_positions_and_tail_orders() {
    use GridTrackSize::{Auto, MinContent};

    let tails = [[MinContent, Auto], [Auto, MinContent]];
    let caps = cap_orders();
    assert_eq!(caps.len(), 24);
    let mut combinations = 0;
    for first in 1..=3 {
        for second in first + 1..=4 {
            for third in second + 1..=5 {
                for fourth in third + 1..=6 {
                    for cap_order in &caps {
                        for tail in &tails {
                            let tracks =
                                four_fit_tracks([first, second, third, fourth], *cap_order, *tail);
                            assert!(supports_repeated_min_max_content_auto_span_track_list(
                                &tracks
                            ));
                            assert!(supports_repeated_span_fit_positions(&tracks, 6));
                            assert!(!supports_repeated_span_fit_positions(&tracks, 3));
                            assert!(supports_intrinsic_auto_track_list(&tracks));
                            combinations += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(
        combinations, 720,
        "15 fit positions × 24 cap orders × 2 tail orders"
    );
}

#[test]
fn repeated_four_fit_row_span_rejects_wrong_tail_counts_caps_and_positions() {
    use GridTrackSize::{Auto, MaxContent, MinContent};

    let caps = [cap(0), cap(1), cap(2), cap(3)];
    let valid = four_fit_tracks([1, 2, 3, 4], caps, [MinContent, Auto]);
    for tail in [
        [MinContent, MinContent],
        [Auto, Auto],
        [MinContent, MaxContent],
    ] {
        let invalid = four_fit_tracks([1, 2, 3, 4], caps, tail);
        assert!(!supports_repeated_min_max_content_auto_span_track_list(
            &invalid
        ));
    }

    let mut invalid_cap = valid.clone();
    invalid_cap[1] =
        GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)));
    assert!(!supports_repeated_min_max_content_auto_span_track_list(
        &invalid_cap
    ));

    let mut wrong_fit_count = valid.clone();
    wrong_fit_count[5] = GridTrackSize::MinContent;
    assert!(!supports_repeated_min_max_content_auto_span_track_list(
        &wrong_fit_count
    ));

    let mut duplicate_cap = valid.clone();
    duplicate_cap[1] = duplicate_cap[0];
    assert!(!supports_repeated_min_max_content_auto_span_track_list(
        &duplicate_cap
    ));
}
