use super::*;

fn track_for_cap(index: usize) -> GridTrackSize {
    GridTrackSize::FitContent(match index {
        0 => GridTrackLengthPercentage::LengthPx(0.0),
        1 => GridTrackLengthPercentage::LengthPx(10.0),
        2 => GridTrackLengthPercentage::Percentage(0.2),
        3 => GridTrackLengthPercentage::LengthPx(30.0),
        4 => GridTrackLengthPercentage::LengthPx(40.0),
        5 => GridTrackLengthPercentage::LengthPx(50.0),
        _ => unreachable!("six-fit cap index is bounded"),
    })
}

fn next_permutation(values: &mut [usize; 6]) -> bool {
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
fn six_fit_row_span_accepts_all_720_exact_typed_cap_orders() {
    let mut order = [0, 1, 2, 3, 4, 5];
    let mut accepted = 0;
    loop {
        let tracks = order.map(track_for_cap).to_vec();
        assert!(
            six_fit_span::supports_six_fit_row_span_track_list(&tracks),
            "{order:?}"
        );
        assert!(supports_repeated_min_max_content_auto_span_track_list(
            &tracks
        ));
        assert!(supports_repeated_span_fit_positions(&tracks, 6));
        assert!(!supports_repeated_span_fit_positions(&tracks, 3));
        accepted += 1;
        if !next_permutation(&mut order) {
            break;
        }
    }
    assert_eq!(accepted, 720);
}

#[test]
fn six_fit_row_span_rejects_wrong_lengths_caps_types_duplicates_and_tails() {
    let valid = [0, 1, 2, 3, 4, 5].map(track_for_cap).to_vec();
    assert!(six_fit_span::supports_six_fit_row_span_track_list(&valid));

    for invalid in [
        valid[..5].to_vec(),
        [valid.clone(), vec![track_for_cap(0)]].concat(),
        {
            let mut tracks = valid.clone();
            tracks[5] = track_for_cap(4);
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[2] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[2] = GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.3));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(-1.0));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] =
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(f32::INFINITY));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] = GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(f32::NAN));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] =
                GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(crate::LayoutCalcId(1)));
            tracks
        },
        {
            let mut tracks = valid.clone();
            tracks[5] = GridTrackSize::Auto;
            tracks
        },
    ] {
        assert!(
            !six_fit_span::supports_six_fit_row_span_track_list(&invalid),
            "{invalid:?}"
        );
    }
}
