use super::*;

fn six_fit_tracks(order: [usize; 6]) -> Vec<GridTrackSize> {
    order
        .into_iter()
        .map(|cap| {
            let cap = match cap {
                0 => GridTrackLengthPercentage::LengthPx(0.0),
                1 => GridTrackLengthPercentage::LengthPx(10.0),
                2 => GridTrackLengthPercentage::Percentage(0.2),
                3 => GridTrackLengthPercentage::LengthPx(30.0),
                4 => GridTrackLengthPercentage::LengthPx(40.0),
                5 => GridTrackLengthPercentage::LengthPx(50.0),
                _ => unreachable!("six-fit cap index is bounded"),
            };
            GridTrackSize::FitContent(cap)
        })
        .collect()
}

#[test]
fn six_fit_auto_placement_accepts_all_720_exact_cap_orders() {
    fn visit(order: &mut [usize; 6], index: usize, count: &mut usize) {
        if index == order.len() {
            let tracks = six_fit_tracks(*order);
            assert!(supports_six_fit_auto_placement_track_list(&tracks));
            assert!(supports_intrinsic_auto_track_list(&tracks));
            assert!(!supports_repeated_min_max_content_auto_placement_track_list(&tracks));
            *count += 1;
            return;
        }
        for next in index..order.len() {
            order.swap(index, next);
            visit(order, index + 1, count);
            order.swap(index, next);
        }
    }

    let mut order = [0, 1, 2, 3, 4, 5];
    let mut count = 0;
    visit(&mut order, 0, &mut count);
    assert_eq!(count, 720);
}

#[test]
fn six_fit_auto_placement_rejects_unlisted_caps_duplicates_and_wrong_lengths() {
    use GridTrackLengthPercentage::{Calc, LengthPx, Percentage};
    use GridTrackSize::{Auto, FitContent};

    let valid = six_fit_tracks([0, 1, 2, 3, 4, 5]);
    assert!(supports_six_fit_auto_placement_track_list(&valid));

    assert!(!supports_six_fit_auto_placement_track_list(&valid[..5]));
    let mut too_long = valid.clone();
    too_long.push(valid[5]);
    assert!(!supports_six_fit_auto_placement_track_list(&too_long));

    let mut duplicate_px = valid.clone();
    duplicate_px[5] = duplicate_px[0];
    assert!(!supports_six_fit_auto_placement_track_list(&duplicate_px));

    let mut duplicate_percentage = valid.clone();
    duplicate_percentage[5] = FitContent(Percentage(0.2));
    assert!(!supports_six_fit_auto_placement_track_list(
        &duplicate_percentage
    ));

    for cap in [
        LengthPx(5.0),
        LengthPx(60.0),
        LengthPx(-1.0),
        LengthPx(f32::NAN),
        LengthPx(f32::INFINITY),
        Percentage(0.1),
        Percentage(0.200_001),
        Percentage(-0.2),
        Percentage(f32::NAN),
        Percentage(f32::INFINITY),
        Calc(crate::LayoutCalcId(1)),
    ] {
        let mut invalid = valid.clone();
        invalid[0] = FitContent(cap);
        assert!(
            !supports_six_fit_auto_placement_track_list(&invalid),
            "unexpectedly accepted cap {cap:?}"
        );
    }

    let mut non_fit_member = valid;
    non_fit_member[2] = Auto;
    assert!(!supports_six_fit_auto_placement_track_list(&non_fit_member));
}
