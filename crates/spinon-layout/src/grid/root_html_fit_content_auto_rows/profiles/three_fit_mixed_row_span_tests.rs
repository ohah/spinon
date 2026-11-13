use super::*;

fn cap(index: usize) -> GridTrackSize {
    GridTrackSize::FitContent(match index {
        0 => GridTrackLengthPercentage::LengthPx(0.0),
        1 => GridTrackLengthPercentage::LengthPx(10.0),
        2 => GridTrackLengthPercentage::Percentage(0.2),
        _ => unreachable!("three-fit cap index is bounded"),
    })
}

fn tail_orders() -> [[GridTrackSize; 3]; 6] {
    use GridTrackSize::{Auto, MaxContent, MinContent};
    [
        [MinContent, MaxContent, Auto],
        [MinContent, Auto, MaxContent],
        [MaxContent, MinContent, Auto],
        [MaxContent, Auto, MinContent],
        [Auto, MinContent, MaxContent],
        [Auto, MaxContent, MinContent],
    ]
}

fn tracks(
    positions: [usize; 3],
    cap_order: [usize; 3],
    tail: [GridTrackSize; 3],
) -> Vec<GridTrackSize> {
    let mut position_index = 0;
    let mut cap_index = 0;
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.get(position_index) == Some(&position) {
                position_index += 1;
                let value = cap(cap_order[cap_index]);
                cap_index += 1;
                value
            } else {
                tail.next().expect("three intrinsic tail tracks")
            }
        })
        .collect()
}

fn next_permutation(values: &mut [usize; 3]) -> bool {
    let Some(pivot) = (0..values.len() - 1)
        .rev()
        .find(|index| values[*index] < values[*index + 1])
    else {
        return false;
    };
    let successor = (pivot + 1..values.len())
        .rev()
        .find(|index| values[*index] > values[pivot])
        .expect("a pivot has a greater successor");
    values.swap(pivot, successor);
    values[pivot + 1..].reverse();
    true
}

#[test]
fn three_fit_mixed_row_span_accepts_exact_caps_tails_and_positions() {
    let mut position_sets = 0;
    let mut accepted = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                position_sets += 1;
                let mut cap_order = [0, 1, 2];
                loop {
                    for tail in tail_orders() {
                        let candidate = tracks([first, second, third], cap_order, tail);
                        assert!(
                            supports_three_fit_mixed_minimum_row_span_track_list(&candidate),
                            "{first}/{second}/{third} {cap_order:?} {tail:?}"
                        );
                        accepted += 1;
                    }
                    if !next_permutation(&mut cap_order) {
                        break;
                    }
                }
            }
        }
    }
    assert_eq!(position_sets, 20);
    assert_eq!(accepted, 720);
}

#[test]
fn three_fit_mixed_row_span_rejects_wrong_cap_types_duplicates_lengths_and_tails() {
    let valid = tracks([1, 2, 3], [0, 1, 2], tail_orders()[0]);
    assert!(supports_three_fit_mixed_minimum_row_span_track_list(&valid));
    for invalid in [
        valid[..5].to_vec(),
        [valid.clone(), vec![GridTrackSize::Auto]].concat(),
        {
            let mut tracks = valid.clone();
            tracks[0] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(1.0));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[1] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(0.0));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[2] = GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.3));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] = GridTrackSize::MinContent;
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] =
                GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)));
            tracks
        },
    ] {
        assert!(
            !supports_three_fit_mixed_minimum_row_span_track_list(&invalid),
            "{invalid:?}"
        );
    }
}
