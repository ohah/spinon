use super::four_member_intrinsic::input_with_tracks;
use super::*;

const PAYLOAD_HEIGHTS: [f32; 8] = [1.0; 8];

fn repeated_tracks(
    cap: GridTrackLengthPercentage,
    tail: &[GridTrackSize],
    fit_position: usize,
) -> Vec<GridTrackSize> {
    let mut tracks = tail.to_vec();
    tracks.insert(fit_position - 1, GridTrackSize::FitContent(cap));
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

fn payload_heights(fit_position: usize) -> [f32; 8] {
    let mut heights = PAYLOAD_HEIGHTS;
    heights[fit_position - 1] = 20.0;
    if let Some(repeated_fit_position) = fit_position.checked_sub(1).and_then(|p| p.checked_add(6))
        && repeated_fit_position < heights.len()
    {
        heights[repeated_fit_position] = 60.0;
    }
    heights
}

fn expected_rows(
    tracks: &[GridTrackSize],
    payload_heights: &[f32; 8],
    cap: GridTrackLengthPercentage,
    align_content: LayoutAlignContent,
) -> Vec<f32> {
    let cap = match cap {
        GridTrackLengthPercentage::LengthPx(value) => value,
        GridTrackLengthPercentage::Percentage(value) => value * 100.0,
        GridTrackLengthPercentage::Calc(_) => unreachable!("the fixed matrix uses px/percentage"),
    };
    let base_heights = payload_heights
        .iter()
        .copied()
        .enumerate()
        .map(|(row, height)| match &tracks[row % tracks.len()] {
            GridTrackSize::FitContent(_) => height.min(cap),
            _ => height,
        })
        .collect::<Vec<_>>();
    let auto_rows = (0..payload_heights.len())
        .filter(|row| matches!(tracks[row % tracks.len()], GridTrackSize::Auto))
        .count();
    let remaining = (100.0 - 10.0 - base_heights.iter().sum::<f32>()).max(0.0);
    let auto_stretch = if align_content == LayoutAlignContent::Stretch {
        remaining / auto_rows as f32
    } else {
        0.0
    };
    let mut rows = vec![10.0];
    for (row, base_height) in base_heights.into_iter().enumerate() {
        let row_height = if matches!(tracks[row % tracks.len()], GridTrackSize::Auto) {
            base_height + auto_stretch
        } else {
            base_height
        };
        rows.push(row_height);
    }
    rows
}

fn caps() -> [(GridTrackLengthPercentage, f32); 4] {
    [
        (GridTrackLengthPercentage::LengthPx(20.0), 20.0),
        (GridTrackLengthPercentage::Percentage(0.2), 20.0),
        (GridTrackLengthPercentage::Percentage(0.0), 0.0),
        (GridTrackLengthPercentage::Percentage(1.2), 120.0),
    ]
}

#[test]
fn six_member_repeated_intrinsic_track_list_preserves_all_positions_orders_caps_and_alignment() {
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
    let mut scene_count = 0;
    let mut overflow_count = 0;
    for fit_position in 1..=6 {
        let payload_heights = payload_heights(fit_position);
        for tail in &orders {
            for (cap, _) in caps() {
                for align_content in [LayoutAlignContent::FlexStart, LayoutAlignContent::Stretch] {
                    let tracks = repeated_tracks(cap, tail, fit_position);
                    let input = input_with_tracks(tracks.clone(), &payload_heights, align_content);
                    let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                        panic!(
                            "fit position={fit_position} tail={tail:?} cap={cap:?} {align_content:?}: {error:?}"
                        )
                    });
                    let rows = expected_rows(&tracks, &payload_heights, cap, align_content);
                    assert_eq!(
                        final_rows(&output),
                        rows,
                        "fit_position={fit_position}, tail={tail:?}, cap={cap:?}, align={align_content:?}"
                    );

                    let mut y = rows[0];
                    for (index, payload_height) in payload_heights.iter().copied().enumerate() {
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

                    let repeated_fit_index = fit_position - 1 + 6;
                    if repeated_fit_index < payload_heights.len() {
                        let overflow_payload =
                            &output.frames[&id(4 + (repeated_fit_index * 2) as u64)];
                        let overflow_y: f32 = rows[..repeated_fit_index + 1].iter().sum();
                        close(
                            overflow_payload.y,
                            overflow_y,
                            "cycled fit-content payload y",
                        );
                        overflow_count +=
                            usize::from(overflow_payload.y + overflow_payload.height > 100.0);
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
                    }
                    close(
                        rows.iter().sum(),
                        if align_content == LayoutAlignContent::Stretch {
                            100.0
                        } else {
                            10.0 + rows[1..].iter().sum::<f32>()
                        },
                        "six-member total row size",
                    );
                    scene_count += 1;
                }
            }
        }
    }
    assert_eq!(scene_count, 1_440);
    assert!(
        overflow_count > 0,
        "matrix should include visible Grid overflow"
    );
}

#[test]
fn six_member_repeated_intrinsic_profile_rejects_wrong_fit_position_multiplicities_and_item_counts()
{
    let cap = GridTrackLengthPercentage::LengthPx(20.0);
    let allowed_payload_heights = PAYLOAD_HEIGHTS;
    let tail = [
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
    ];
    let tracks = repeated_tracks(cap, &tail, 1);

    let duplicate_fit = [
        GridTrackSize::FitContent(cap),
        GridTrackSize::FitContent(cap),
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
    ];
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                duplicate_fit.to_vec(),
                &allowed_payload_heights,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "a repeated list must contain exactly one fit-content member"
    );

    let mut wrong_duplicate = tracks.clone();
    wrong_duplicate[5] = GridTrackSize::MinContent;
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                wrong_duplicate,
                &allowed_payload_heights,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "a third min-content and missing repeated max-content member must fail closed"
    );

    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                tracks.clone(),
                &allowed_payload_heights[..7],
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

    let mut overflow_in_first_fit_position = payload_heights(1);
    overflow_in_first_fit_position[0] = 60.0;
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                repeated_tracks(cap, &tail, 1),
                &overflow_in_first_fit_position,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "the 60px overflow payload is reserved for the repeated fit-content position"
    );

    let mut mixed_numeric_profile = payload_heights(1);
    mixed_numeric_profile[1] = 10.0;
    assert!(
        TaffyLayoutEngine
            .compute(&input_with_tracks(
                repeated_tracks(cap, &tail, 1),
                &mixed_numeric_profile,
                LayoutAlignContent::FlexStart,
            ))
            .is_err(),
        "fixed-size fixture items must match one whole numeric payload profile"
    );
}
