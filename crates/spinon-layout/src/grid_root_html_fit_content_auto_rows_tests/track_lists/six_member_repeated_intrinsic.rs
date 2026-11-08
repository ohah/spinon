use super::four_member_intrinsic::input_with_tracks;
use super::*;

const PAYLOAD_HEIGHTS: [f32; 8] = [20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 60.0, 6.0];

fn repeated_tracks(cap: GridTrackLengthPercentage, tail: &[GridTrackSize]) -> Vec<GridTrackSize> {
    let mut tracks = vec![GridTrackSize::FitContent(cap)];
    tracks.extend(tail.iter().cloned());
    tracks
}

fn repeated_tail_orders() -> Vec<Vec<GridTrackSize>> {
    fn visit(
        remaining: &mut [usize; 3],
        current: &mut Vec<GridTrackSize>,
        orders: &mut Vec<Vec<GridTrackSize>>,
    ) {
        if current.len() == 5 {
            orders.push(current.clone());
            return;
        }

        for kind in 0..remaining.len() {
            if remaining[kind] == 0 {
                continue;
            }
            remaining[kind] -= 1;
            current.push(match kind {
                0 => GridTrackSize::MinContent,
                1 => GridTrackSize::MaxContent,
                _ => GridTrackSize::Auto,
            });
            visit(remaining, current, orders);
            current.pop();
            remaining[kind] += 1;
        }
    }

    let mut orders = Vec::new();
    visit(&mut [2, 2, 1], &mut Vec::new(), &mut orders);
    orders
}

fn expected_rows(tail: &[GridTrackSize], align_content: LayoutAlignContent) -> Vec<f32> {
    let auto_rows = (0..PAYLOAD_HEIGHTS.len())
        .filter(|row| *row % 6 != 0 && tail[*row % 6 - 1] == GridTrackSize::Auto)
        .count();
    let auto_stretch = if align_content == LayoutAlignContent::Stretch {
        4.0 / auto_rows as f32
    } else {
        0.0
    };
    let mut rows = vec![10.0];
    for (row, payload_height) in PAYLOAD_HEIGHTS.iter().copied().enumerate() {
        let track = if row % 6 == 0 {
            None
        } else {
            Some(&tail[row % 6 - 1])
        };
        let mut row_height = if track.is_none() {
            payload_height.min(20.0)
        } else {
            payload_height
        };
        if matches!(track, Some(GridTrackSize::Auto)) {
            row_height += auto_stretch;
        }
        rows.push(row_height);
    }
    rows
}

#[test]
fn six_member_repeated_intrinsic_track_list_preserves_all_30_orders_and_stretches_only_auto() {
    let orders = repeated_tail_orders();
    assert_eq!(orders.len(), 30);
    assert_eq!(
        orders
            .iter()
            .map(|tail| format!("{tail:?}"))
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        30,
        "the repeated-tail generator must produce 30 unique orders"
    );
    for tail in orders {
        for cap in [
            GridTrackLengthPercentage::LengthPx(20.0),
            GridTrackLengthPercentage::Percentage(0.2),
        ] {
            for align_content in [LayoutAlignContent::FlexStart, LayoutAlignContent::Stretch] {
                let input =
                    input_with_tracks(repeated_tracks(cap, &tail), &PAYLOAD_HEIGHTS, align_content);
                let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                    panic!("six-member tail {tail:?} {cap:?} {align_content:?}: {error:?}")
                });
                let rows = expected_rows(&tail, align_content);
                assert_eq!(
                    final_rows(&output),
                    rows,
                    "tail={tail:?}, cap={cap:?}, align={align_content:?}"
                );

                let mut y = rows[0];
                for (index, payload_height) in PAYLOAD_HEIGHTS.iter().copied().enumerate() {
                    let item = id(3 + (index * 2) as u64);
                    let payload = id(4 + (index * 2) as u64);
                    let item_frame = &output.frames[&item];
                    close(item_frame.x, 0.0, "six-member item x");
                    close(item_frame.y, y, "six-member item y");
                    close(item_frame.width, 40.0, "six-member item width");
                    close(item_frame.height, rows[index + 1], "six-member item height");

                    let payload_frame = &output.frames[&payload];
                    close(payload_frame.x, 0.0, "six-member payload x");
                    close(payload_frame.y, y, "six-member payload y");
                    close(payload_frame.width, 40.0, "six-member payload width");
                    close(
                        payload_frame.height,
                        payload_height,
                        "six-member payload height",
                    );
                    y += rows[index + 1];
                }

                let overflow_payload = &output.frames[&id(16)];
                let overflow_y: f32 = rows[..7].iter().sum();
                close(
                    overflow_payload.y,
                    overflow_y,
                    "cycled fit-content payload y",
                );
                close(
                    overflow_payload.height,
                    60.0,
                    "cycled fit-content payload height",
                );
                close(
                    overflow_payload.y + overflow_payload.height - 100.0,
                    overflow_y + 60.0 - 100.0,
                    "cycled fit-content overflow past Grid border",
                );
                close(
                    rows.iter().sum(),
                    if align_content == LayoutAlignContent::Stretch {
                        100.0
                    } else {
                        96.0
                    },
                    "six-member total row size",
                );
            }
        }
    }
}

#[test]
fn six_member_repeated_intrinsic_profile_rejects_wrong_fit_position_multiplicities_and_item_counts()
{
    let cap = GridTrackLengthPercentage::LengthPx(20.0);
    let payload_heights = PAYLOAD_HEIGHTS;
    let tail = [
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
    ];
    let tracks = repeated_tracks(cap, &tail);

    let mut fit_not_first = tracks.clone();
    fit_not_first.swap(0, 1);
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                fit_not_first,
                &payload_heights,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "moving fit-content away from the first slot must remain unsupported"
    );

    let mut wrong_duplicate = tracks.clone();
    wrong_duplicate[5] = GridTrackSize::MinContent;
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                wrong_duplicate,
                &payload_heights,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "a third min-content and missing repeated max-content member must fail closed"
    );

    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                tracks.clone(),
                &payload_heights[..7],
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "seven implicit items do not cover the required second cycle profile"
    );

    let too_many_payloads = [20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 60.0, 6.0, 10.0];
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                tracks,
                &too_many_payloads,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "a ninth implicit item must not extend the accepted item topology"
    );

    let mut overflow_in_first_fit_position = PAYLOAD_HEIGHTS;
    overflow_in_first_fit_position[0] = 60.0;
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                repeated_tracks(cap, &tail),
                &overflow_in_first_fit_position,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "the 60px overflow payload is reserved for the repeated fit-content position"
    );
}
