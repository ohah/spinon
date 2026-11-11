use super::*;

fn six_fit_tracks(order: [usize; 6]) -> Vec<ComputedCssGridTrackSize> {
    order
        .into_iter()
        .map(|cap| {
            let cap = match cap {
                0 => ComputedCssGridTrackLengthPercentage::LengthPx(0.0),
                1 => ComputedCssGridTrackLengthPercentage::LengthPx(10.0),
                2 => ComputedCssGridTrackLengthPercentage::Percentage(0.2),
                3 => ComputedCssGridTrackLengthPercentage::LengthPx(30.0),
                4 => ComputedCssGridTrackLengthPercentage::LengthPx(40.0),
                5 => ComputedCssGridTrackLengthPercentage::LengthPx(50.0),
                _ => unreachable!("six-fit cap index is bounded"),
            };
            ComputedCssGridTrackSize::FitContent(cap)
        })
        .collect()
}

#[test]
fn six_fit_auto_placement_accepts_all_720_exact_cap_orders() {
    fn visit(order: &mut [usize; 6], index: usize, count: &mut usize) {
        if index == order.len() {
            let tracks = six_fit_tracks(*order);
            assert!(
                root_html_fit_content_repeated_six_fit_auto_placement_track_list(&tracks),
                "cap order={order:?}"
            );
            assert!(
                !root_html_fit_content_repeated_min_max_content_auto_placement_track_list(&tracks),
                "six-fit admission stays separate from the one-to-five-fit mixed profile"
            );
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
    use ComputedCssGridTrackLengthPercentage::{Calc, LengthPx, Percentage};
    use ComputedCssGridTrackSize::{Auto, FitContent};

    let valid = six_fit_tracks([0, 1, 2, 3, 4, 5]);
    assert!(root_html_fit_content_repeated_six_fit_auto_placement_track_list(&valid));

    let mut too_short = valid[..5].to_vec();
    assert!(!root_html_fit_content_repeated_six_fit_auto_placement_track_list(&too_short));
    too_short.push(valid[5].clone());
    too_short.push(valid[4].clone());
    assert!(!root_html_fit_content_repeated_six_fit_auto_placement_track_list(&too_short));

    let mut duplicate_px = valid.clone();
    duplicate_px[5] = duplicate_px[0].clone();
    assert!(!root_html_fit_content_repeated_six_fit_auto_placement_track_list(&duplicate_px));

    let mut duplicate_percentage = valid.clone();
    duplicate_percentage[5] = FitContent(Percentage(0.2));
    assert!(
        !root_html_fit_content_repeated_six_fit_auto_placement_track_list(&duplicate_percentage)
    );

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
        Calc(ComputedCssMath::LengthPx(30.0)),
    ] {
        let mut invalid = valid.clone();
        invalid[0] = FitContent(cap.clone());
        assert!(
            !root_html_fit_content_repeated_six_fit_auto_placement_track_list(&invalid),
            "unexpectedly accepted cap {cap:?}"
        );
    }

    let mut non_fit_member = valid;
    non_fit_member[2] = Auto;
    assert!(!root_html_fit_content_repeated_six_fit_auto_placement_track_list(&non_fit_member));
}
