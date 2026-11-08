use super::four_member_intrinsic::input_with_tracks;
use super::*;

const PAYLOAD_HEIGHTS: [f32; 8] = [20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 60.0, 6.0];

fn repeated_tracks(cap: GridTrackLengthPercentage) -> Vec<GridTrackSize> {
    vec![
        GridTrackSize::FitContent(cap),
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
    ]
}

fn expected_rows(align_content: LayoutAlignContent) -> Vec<f32> {
    let mut rows = vec![10.0, 20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 20.0, 6.0];
    if align_content == LayoutAlignContent::Stretch {
        rows[4] += 4.0;
    }
    rows
}

#[test]
fn six_member_repeated_intrinsic_track_list_cycles_and_stretches_only_auto() {
    for cap in [
        GridTrackLengthPercentage::LengthPx(20.0),
        GridTrackLengthPercentage::Percentage(0.2),
    ] {
        for align_content in [LayoutAlignContent::FlexStart, LayoutAlignContent::Stretch] {
            let input = input_with_tracks(repeated_tracks(cap), &PAYLOAD_HEIGHTS, align_content);
            let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                panic!("six-member list {cap:?} {align_content:?}: {error:?}")
            });
            let rows = expected_rows(align_content);
            assert_eq!(
                final_rows(&output),
                rows,
                "cap={cap:?}, align={align_content:?}"
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
            close(
                overflow_payload.y,
                if align_content == LayoutAlignContent::Stretch {
                    74.0
                } else {
                    70.0
                },
                "cycled fit-content payload y",
            );
            close(
                overflow_payload.height,
                60.0,
                "cycled fit-content payload height",
            );
            close(
                overflow_payload.y + overflow_payload.height - 100.0,
                if align_content == LayoutAlignContent::Stretch {
                    34.0
                } else {
                    30.0
                },
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

#[test]
fn six_member_repeated_intrinsic_profile_rejects_other_orders_and_item_counts() {
    let cap = GridTrackLengthPercentage::LengthPx(20.0);
    let payload_heights = PAYLOAD_HEIGHTS;
    let tracks = repeated_tracks(cap);

    let mut wrong_order = tracks.clone();
    wrong_order.swap(3, 4);
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                wrong_order,
                &payload_heights,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "moving the auto member must remain outside the exact supported order"
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
                repeated_tracks(cap),
                &overflow_in_first_fit_position,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "the 60px overflow payload is reserved for the repeated fit-content position"
    );
}
