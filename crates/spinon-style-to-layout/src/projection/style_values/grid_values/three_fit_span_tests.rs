use super::*;

fn tracks(
    positions: [usize; 3],
    tail: [ComputedCssGridTrackSize; 3],
) -> Vec<ComputedCssGridTrackSize> {
    let mut positions = positions.into_iter().peekable();
    let mut tail = tail.into_iter();
    (1..=6)
        .map(|position| {
            if positions.peek() == Some(&position) {
                positions.next();
                ComputedCssGridTrackSize::FitContent(
                    ComputedCssGridTrackLengthPercentage::LengthPx(position as f32),
                )
            } else {
                tail.next().expect("three intrinsic tail tracks")
            }
        })
        .collect()
}

fn tail_orders() -> [[ComputedCssGridTrackSize; 3]; 6] {
    use ComputedCssGridTrackSize::{Auto, MaxContent, MinContent};
    [
        [MinContent, MaxContent, Auto],
        [MinContent, Auto, MaxContent],
        [MaxContent, MinContent, Auto],
        [MaxContent, Auto, MinContent],
        [Auto, MinContent, MaxContent],
        [Auto, MaxContent, MinContent],
    ]
}

#[test]
fn three_fit_row_span_accepts_all_positions_and_keeps_auto_placement_closed() {
    let mut combinations = 0;
    for first in 1..=4 {
        for second in first + 1..=5 {
            for third in second + 1..=6 {
                for tail in tail_orders() {
                    let tracks = tracks([first, second, third], tail);
                    assert!(
                        root_html_fit_content_repeated_min_max_content_auto_span_track_list(
                            &tracks
                        )
                    );
                    assert!(
                        !root_html_fit_content_repeated_min_max_content_auto_placement_track_list(
                            &tracks
                        ),
                        "three-fit admission must stay limited to explicit row-span profiles"
                    );
                    combinations += 1;
                }
            }
        }
    }
    assert_eq!(combinations, 120);
}
