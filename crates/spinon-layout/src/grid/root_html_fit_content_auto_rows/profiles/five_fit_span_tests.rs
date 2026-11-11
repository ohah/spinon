use super::*;

fn cap(index: usize) -> GridTrackLengthPercentage {
    match index {
        0 => GridTrackLengthPercentage::LengthPx(0.0),
        1 => GridTrackLengthPercentage::LengthPx(10.0),
        2 => GridTrackLengthPercentage::Percentage(0.2),
        3 => GridTrackLengthPercentage::LengthPx(30.0),
        4 => GridTrackLengthPercentage::LengthPx(40.0),
        _ => unreachable!("five exact caps are registered"),
    }
}

fn cap_orders() -> Vec<[usize; 5]> {
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

fn tracks(positions: [usize; 5], cap_order: [usize; 5], tail: GridTrackSize) -> Vec<GridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut caps = cap_order.into_iter();
    let mut tail = Some(tail);
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                GridTrackSize::FitContent(cap(caps.next().expect("five fit caps")))
            } else {
                tail.take().expect("one intrinsic tail")
            }
        })
        .collect()
}

#[test]
fn repeated_five_fit_row_span_accepts_all_positions_caps_and_tails() {
    use GridTrackSize::{Auto, MaxContent, MinContent};

    let tails = [MinContent, MaxContent, Auto];
    let cap_orders = cap_orders();
    assert_eq!(cap_orders.len(), 120);
    let mut combinations = 0;
    for missing in 1..=6 {
        let positions = (1..=6)
            .filter(|position| *position != missing)
            .collect::<Vec<_>>()
            .try_into()
            .expect("five positions remain");
        for cap_order in &cap_orders {
            for tail in tails {
                let list = tracks(positions, *cap_order, tail);
                assert!(five_fit_span::supports_five_fit_row_span_track_list(&list));
                assert!(supports_repeated_min_max_content_auto_span_track_list(
                    &list
                ));
                assert!(supports_repeated_span_fit_positions(&list, 6));
                assert!(!supports_repeated_span_fit_positions(&list, 3));
                assert!(supports_repeated_min_max_content_auto_placement_track_list(
                    &list
                ));
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
fn repeated_five_fit_row_span_rejects_unregistered_caps_duplicate_caps_and_bad_tails() {
    use GridTrackSize::{Auto, FitContent, MaxContent, MinContent};

    let valid = tracks([1, 2, 3, 4, 5], [0, 1, 2, 3, 4], MinContent);
    for invalid_cap in [
        GridTrackLengthPercentage::LengthPx(-1.0),
        GridTrackLengthPercentage::LengthPx(50.0),
        GridTrackLengthPercentage::LengthPx(f32::NAN),
        GridTrackLengthPercentage::Percentage(0.3),
        GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)),
    ] {
        let mut invalid = valid.clone();
        invalid[0] = FitContent(invalid_cap);
        assert!(!five_fit_span::supports_five_fit_row_span_track_list(
            &invalid
        ));
        assert!(!supports_repeated_span_fit_positions(&invalid, 6));
    }

    let mut auto_placement_cap = valid.clone();
    auto_placement_cap[0] = FitContent(GridTrackLengthPercentage::LengthPx(50.0));
    assert!(!five_fit_span::supports_five_fit_row_span_track_list(
        &auto_placement_cap
    ));
    assert!(supports_repeated_min_max_content_auto_placement_track_list(
        &auto_placement_cap
    ));

    let mut duplicate_cap = valid.clone();
    duplicate_cap[1] = duplicate_cap[0];
    assert!(!five_fit_span::supports_five_fit_row_span_track_list(
        &duplicate_cap
    ));
    assert!(supports_repeated_min_max_content_auto_placement_track_list(
        &duplicate_cap
    ));

    let mut extra_tail = valid.clone();
    extra_tail.push(Auto);
    assert!(!five_fit_span::supports_five_fit_row_span_track_list(
        &extra_tail
    ));
    let wrong_tail_kind = [
        FitContent(cap(0)),
        FitContent(cap(1)),
        FitContent(cap(2)),
        FitContent(cap(3)),
        FitContent(cap(4)),
        GridTrackSize::LengthPx(10.0),
    ];
    assert!(!five_fit_span::supports_five_fit_row_span_track_list(
        &wrong_tail_kind
    ));

    let four_fits = tracks([1, 2, 3, 4, 5], [0, 1, 2, 3, 4], MaxContent);
    let mut shortened = four_fits;
    shortened[0] = MinContent;
    shortened.truncate(5);
    assert!(!five_fit_span::supports_five_fit_row_span_track_list(
        &shortened
    ));
}
