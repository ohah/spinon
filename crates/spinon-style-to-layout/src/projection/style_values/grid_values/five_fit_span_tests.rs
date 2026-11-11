use super::*;

fn cap(index: usize) -> ComputedCssGridTrackSize {
    match index {
        0 => ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
            0.0,
        )),
        1 => ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
            10.0,
        )),
        2 => ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::Percentage(0.2),
        ),
        3 => ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
            30.0,
        )),
        4 => ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
            40.0,
        )),
        _ => unreachable!("five exact caps are registered"),
    }
}

fn track_list(
    positions: [usize; 5],
    cap_order: [usize; 5],
    tail: ComputedCssGridTrackSize,
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut caps = cap_order.into_iter();
    let mut tail = Some(tail);
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                cap(caps.next().expect("five fit caps"))
            } else {
                tail.take().expect("one intrinsic tail")
            }
        })
        .collect()
}

fn permutations() -> Vec<[usize; 5]> {
    fn append(prefix: &mut Vec<usize>, rest: &mut Vec<usize>, result: &mut Vec<[usize; 5]>) {
        if rest.is_empty() {
            result.push(prefix.as_slice().try_into().expect("five cap indexes"));
            return;
        }
        for index in 0..rest.len() {
            let value = rest.remove(index);
            prefix.push(value);
            append(prefix, rest, result);
            prefix.pop();
            rest.insert(index, value);
        }
    }

    let mut result = Vec::with_capacity(120);
    append(&mut Vec::new(), &mut vec![0, 1, 2, 3, 4], &mut result);
    result
}

#[test]
fn five_fit_row_span_accepts_all_positions_cap_orders_and_singleton_tails() {
    use ComputedCssGridTrackSize::{Auto, MaxContent, MinContent};

    let tails = [MinContent, MaxContent, Auto];
    let cap_orders = permutations();
    assert_eq!(cap_orders.len(), 120);
    let mut combinations = 0;
    for missing in 1..=6 {
        let positions = (1..=6)
            .filter(|position| *position != missing)
            .collect::<Vec<_>>()
            .try_into()
            .expect("five positions remain");
        for cap_order in &cap_orders {
            for tail in &tails {
                let tracks = track_list(positions, *cap_order, tail.clone());
                assert!(
                    five_fit_span::supports_repeated_five_fit_row_span_track_list(&tracks),
                    "missing={missing}, cap order={cap_order:?}, tail={tail:?}"
                );
                assert!(
                    root_html_fit_content_repeated_min_max_content_auto_span_track_list(&tracks)
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
    assert_eq!(
        combinations, 2_160,
        "6 positions × 120 cap orders × 3 tails"
    );
}

#[test]
fn five_fit_row_span_rejects_unregistered_caps_duplicate_caps_and_extra_tails() {
    use ComputedCssGridTrackSize::{Auto, FitContent, MinContent};

    let valid = track_list([1, 2, 3, 4, 5], [0, 1, 2, 3, 4], MinContent);
    for invalid_cap in [
        ComputedCssGridTrackLengthPercentage::LengthPx(-1.0),
        ComputedCssGridTrackLengthPercentage::LengthPx(20.0),
        ComputedCssGridTrackLengthPercentage::LengthPx(f32::NAN),
        ComputedCssGridTrackLengthPercentage::Percentage(0.3),
        ComputedCssGridTrackLengthPercentage::Calc(ComputedCssMath::LengthPx(1.0)),
    ] {
        let mut invalid = valid.clone();
        invalid[0] = FitContent(invalid_cap);
        assert!(!five_fit_span::supports_repeated_five_fit_row_span_track_list(&invalid));
        assert!(!root_html_fit_content_repeated_min_max_content_auto_span_track_list(&invalid));
    }

    let mut duplicate_cap = valid.clone();
    duplicate_cap[1] = duplicate_cap[0].clone();
    assert!(!five_fit_span::supports_repeated_five_fit_row_span_track_list(&duplicate_cap));
    assert!(
        root_html_fit_content_repeated_min_max_content_auto_placement_track_list(&duplicate_cap)
    );

    let mut extra_tail = valid.clone();
    extra_tail.push(Auto);
    assert!(!five_fit_span::supports_repeated_five_fit_row_span_track_list(&extra_tail));
    assert!(!root_html_fit_content_repeated_min_max_content_auto_span_track_list(&valid[..5]));

    let wrong_tail_kind = [
        FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(0.0)),
        FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(10.0)),
        FitContent(ComputedCssGridTrackLengthPercentage::Percentage(0.2)),
        FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(30.0)),
        FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(40.0)),
        ComputedCssGridTrackSize::LengthPx(10.0),
    ];
    assert!(!five_fit_span::supports_repeated_five_fit_row_span_track_list(&wrong_tail_kind));
    let mut nonregistered_cap = valid.clone();
    nonregistered_cap[0] = FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(50.0));
    assert!(
        root_html_fit_content_repeated_min_max_content_auto_placement_track_list(
            &nonregistered_cap
        )
    );
    assert!(!five_fit_span::supports_repeated_five_fit_row_span_track_list(&nonregistered_cap));
}
