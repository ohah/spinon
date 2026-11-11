use super::*;

fn tracks(
    positions: [usize; 4],
    caps: [ComputedCssGridTrackLengthPercentage; 4],
    tail: [ComputedCssGridTrackSize; 2],
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut caps = caps.into_iter();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                ComputedCssGridTrackSize::FitContent(caps.next().expect("four fit caps"))
            } else {
                tail.next().expect("two intrinsic tail tracks")
            }
        })
        .collect()
}

fn cap(index: usize) -> ComputedCssGridTrackLengthPercentage {
    match index {
        0 => ComputedCssGridTrackLengthPercentage::LengthPx(0.0),
        1 => ComputedCssGridTrackLengthPercentage::LengthPx(10.0),
        2 => ComputedCssGridTrackLengthPercentage::Percentage(0.2),
        3 => ComputedCssGridTrackLengthPercentage::LengthPx(30.0),
        _ => unreachable!("four supported caps are registered"),
    }
}

fn cap_orders() -> Vec<[ComputedCssGridTrackLengthPercentage; 4]> {
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

#[test]
fn four_fit_row_span_accepts_all_fit_positions_and_tail_orders() {
    use ComputedCssGridTrackSize::{Auto, MinContent};

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
                            let tracks = tracks(
                                [first, second, third, fourth],
                                cap_order.clone(),
                                tail.clone(),
                            );
                            assert!(
                                root_html_fit_content_repeated_min_max_content_auto_span_track_list(
                                    &tracks
                                )
                            );
                            assert!(!root_html_fit_content_repeated_min_max_content_auto_placement_track_list(
                                &tracks
                            ));
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
fn four_fit_row_span_rejects_wrong_tail_counts_lengths_and_caps() {
    use ComputedCssGridTrackSize::{Auto, MaxContent, MinContent};

    for tail in [
        [MinContent, MinContent],
        [Auto, Auto],
        [MinContent, MaxContent],
    ] {
        assert!(
            !root_html_fit_content_repeated_min_max_content_auto_span_track_list(&tracks(
                [1, 2, 3, 4],
                [cap(0), cap(1), cap(2), cap(3)],
                tail
            ))
        );
    }

    let mut invalid_cap = tracks(
        [1, 2, 3, 4],
        [cap(0), cap(1), cap(2), cap(3)],
        [MinContent, Auto],
    );
    invalid_cap[0] = ComputedCssGridTrackSize::FitContent(
        ComputedCssGridTrackLengthPercentage::Percentage(f32::NAN),
    );
    assert!(!root_html_fit_content_repeated_min_max_content_auto_span_track_list(&invalid_cap));

    let mut duplicate_cap = tracks(
        [1, 2, 3, 4],
        [cap(0), cap(1), cap(2), cap(3)],
        [MinContent, Auto],
    );
    duplicate_cap[1] = duplicate_cap[0].clone();
    assert!(!root_html_fit_content_repeated_min_max_content_auto_span_track_list(&duplicate_cap));

    let wrong_length = tracks(
        [1, 2, 3, 4],
        [cap(0), cap(1), cap(2), cap(3)],
        [MinContent, Auto],
    );
    assert!(
        !root_html_fit_content_repeated_min_max_content_auto_span_track_list(&wrong_length[..5])
    );
}
