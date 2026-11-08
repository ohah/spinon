use super::*;

const ORDERS: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

const PAYLOAD_HEIGHTS: [f32; 5] = [20.0, 10.0, 12.0, 6.0, 20.0];

fn members(cap: GridTrackLengthPercentage) -> [GridTrackSize; 4] {
    [
        GridTrackSize::FitContent(cap),
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
    ]
}

fn ordered_tracks(order: [usize; 4], cap: GridTrackLengthPercentage) -> Vec<GridTrackSize> {
    let members = members(cap);
    order.map(|index| members[index]).to_vec()
}

fn input_with_tracks(
    tracks: Vec<GridTrackSize>,
    payload_heights: &[f32],
    align_content: LayoutAlignContent,
) -> LayoutInput {
    let root = id(1);
    let explicit = id(2);
    let row_items = (0..payload_heights.len())
        .map(|index| id(3 + (index * 2) as u64))
        .collect::<Vec<_>>();
    let payloads = (0..payload_heights.len())
        .map(|index| id(4 + (index * 2) as u64))
        .collect::<Vec<_>>();
    let mut children = vec![explicit];
    children.extend(row_items.iter().copied());
    let mut nodes = vec![LayoutNode {
        id: root,
        children,
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            box_sizing: LayoutBoxSizing::BorderBox,
            width: LayoutDimension::Fixed(40.0),
            height: LayoutDimension::Fixed(100.0),
            align_content: Some(align_content),
            justify_content: LayoutJustifyContent::FlexStart,
            gap: LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::ZERO,
            },
            ..LayoutStyle::default()
        },
    }];
    nodes.push(LayoutNode {
        id: explicit,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    });
    for ((item, payload), payload_height) in row_items
        .iter()
        .copied()
        .zip(payloads.iter().copied())
        .zip(payload_heights.iter().copied())
    {
        nodes.push(LayoutNode {
            id: item,
            children: vec![payload],
            style: LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Auto,
                height: LayoutDimension::Auto,
                min_height: LayoutDimension::Fixed(0.0),
                ..LayoutStyle::default()
            },
        });
        nodes.push(LayoutNode {
            id: payload,
            children: Vec::new(),
            style: LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Auto,
                height: LayoutDimension::Fixed(payload_height),
                ..LayoutStyle::default()
            },
        });
    }

    let mut grid_items = BTreeMap::from([(explicit, GridItemInput::default())]);
    grid_items.extend(
        row_items
            .into_iter()
            .map(|item| (item, GridItemInput::default())),
    );
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 40.0,
            height: 100.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(40.0)],
                rows: vec![GridTrackSize::LengthPx(10.0)],
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::TrackList(tracks),
                placement_mode: GridPlacementMode::RootHtmlFitContentAutoRowsV1,
                track_sizing_trace_enabled: true,
                ..GridContainerInput::default()
            },
        )]),
        grid_items,
        text_paragraphs: BTreeMap::new(),
    }
}

fn expected_rows(order: [usize; 4], align_content: LayoutAlignContent) -> Vec<f32> {
    let repeated_types = (0..PAYLOAD_HEIGHTS.len())
        .map(|index| order[index % order.len()])
        .collect::<Vec<_>>();
    let auto_tracks = repeated_types.iter().filter(|member| **member == 3).count();
    let stretch_per_auto = if align_content == LayoutAlignContent::Stretch {
        22.0 / auto_tracks as f32
    } else {
        0.0
    };
    let mut rows = vec![10.0];
    rows.extend(
        PAYLOAD_HEIGHTS
            .iter()
            .zip(repeated_types)
            .map(|(height, member)| height + if member == 3 { stretch_per_auto } else { 0.0 }),
    );
    rows
}

#[test]
fn four_member_intrinsic_track_lists_cover_all_orders_caps_and_auto_stretch() {
    assert_eq!(ORDERS.len(), 24);
    for order in ORDERS {
        for cap in [
            GridTrackLengthPercentage::LengthPx(20.0),
            GridTrackLengthPercentage::Percentage(0.2),
        ] {
            let tracks = ordered_tracks(order, cap);
            for align_content in [LayoutAlignContent::FlexStart, LayoutAlignContent::Stretch] {
                let input = input_with_tracks(tracks.clone(), &PAYLOAD_HEIGHTS, align_content);
                let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
                    panic!("supported order {order:?} {cap:?} {align_content:?}: {error:?}")
                });
                let rows = expected_rows(order, align_content);
                assert_eq!(
                    final_rows(&output),
                    rows,
                    "order={order:?}, cap={cap:?}, align={align_content:?}"
                );

                let mut y = rows[0];
                for (index, height) in PAYLOAD_HEIGHTS.iter().enumerate() {
                    let item = id(3 + (index * 2) as u64);
                    let payload = id(4 + (index * 2) as u64);
                    let item_frame = &output.frames[&item];
                    close(item_frame.x, 0.0, "four-member item x");
                    close(item_frame.y, y, "four-member item y");
                    close(item_frame.width, 40.0, "four-member item width");
                    close(
                        item_frame.height,
                        rows[index + 1],
                        "four-member item height",
                    );
                    let payload_frame = &output.frames[&payload];
                    close(payload_frame.x, 0.0, "four-member payload x");
                    close(payload_frame.y, y, "four-member payload y");
                    close(payload_frame.width, 40.0, "four-member payload width");
                    close(payload_frame.height, *height, "four-member payload height");
                    y += rows[index + 1];
                }
                let total: f32 = rows.iter().sum();
                close(
                    total,
                    if align_content == LayoutAlignContent::Stretch {
                        100.0
                    } else {
                        78.0
                    },
                    "four-member total row size",
                );
            }
        }
    }
}

#[test]
fn four_member_intrinsic_track_lists_accept_zero_and_over_100_percent_caps() {
    for order in ORDERS {
        for (cap, zero_cap) in [
            (GridTrackLengthPercentage::Percentage(0.0), true),
            (GridTrackLengthPercentage::Percentage(1.2), false),
        ] {
            let input = input_with_tracks(
                ordered_tracks(order, cap),
                &PAYLOAD_HEIGHTS,
                LayoutAlignContent::FlexStart,
            );
            let output = TaffyLayoutEngine
                .compute(&input)
                .unwrap_or_else(|error| panic!("order={order:?}, cap={cap:?}: {error:?}"));
            let mut rows = vec![10.0];
            rows.extend((0..PAYLOAD_HEIGHTS.len()).map(|index| {
                if zero_cap && order[index % order.len()] == 0 {
                    0.0
                } else {
                    PAYLOAD_HEIGHTS[index]
                }
            }));
            assert_eq!(
                final_rows(&output),
                rows,
                "zero and over-100% caps should retain fit-content sizing: order={order:?}, cap={cap:?}"
            );
        }
    }
}

#[test]
fn fit_content_cap_keeps_track_bounded_while_payload_overflows_the_grid() {
    for order in ORDERS {
        let cap_position = order
            .iter()
            .position(|member| *member == 0)
            .expect("each order contains fit-content");
        let overflow_index = if cap_position == 0 { 4 } else { cap_position };

        for cap in [
            GridTrackLengthPercentage::LengthPx(20.0),
            GridTrackLengthPercentage::Percentage(0.2),
        ] {
            let mut payload_heights = PAYLOAD_HEIGHTS;
            payload_heights[overflow_index] = 60.0;
            let input = input_with_tracks(
                ordered_tracks(order, cap),
                &payload_heights,
                LayoutAlignContent::FlexStart,
            );
            let output = TaffyLayoutEngine
                .compute(&input)
                .unwrap_or_else(|error| panic!("order={order:?}, cap={cap:?}: {error:?}"));

            let mut rows = vec![10.0];
            rows.extend((0..payload_heights.len()).map(|index| {
                if order[index % order.len()] == 0 {
                    20.0
                } else {
                    payload_heights[index]
                }
            }));
            assert_eq!(
                final_rows(&output),
                rows,
                "fit-content cycle position must keep its used track capped: order={order:?}, cap={cap:?}"
            );
            close(
                output.frames[&id(1)].height,
                100.0,
                "Grid border-box height",
            );

            let item = id(3 + (overflow_index * 2) as u64);
            let payload = id(4 + (overflow_index * 2) as u64);
            let item_y = rows[..=overflow_index].iter().sum::<f32>();
            close(output.frames[&item].y, item_y, "fit-content item y");
            close(output.frames[&item].height, 20.0, "fit-content row cap");
            close(output.frames[&payload].y, item_y, "overflow payload y");
            close(
                output.frames[&payload].height,
                60.0,
                "visible payload height",
            );
            assert!(
                output.frames[&payload].y + output.frames[&payload].height
                    > output.frames[&item].y + output.frames[&item].height,
                "payload must visibly overflow its capped item: order={order:?}, cap={cap:?}"
            );
            assert_eq!(
                output.frames[&payload].y + output.frames[&payload].height > 100.0,
                cap_position == 0 || cap_position == 3,
                "only the expected late cycle positions spill past the Grid: order={order:?}, cap={cap:?}"
            );
        }
    }
}

#[test]
fn four_member_intrinsic_track_lists_reject_nearby_types_counts_and_other_profiles() {
    let cap = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0));
    for tracks in [
        vec![cap, GridTrackSize::MinContent, GridTrackSize::MaxContent],
        vec![
            cap,
            GridTrackSize::MinContent,
            GridTrackSize::Auto,
            GridTrackSize::Auto,
        ],
        vec![
            cap,
            GridTrackSize::MinContent,
            GridTrackSize::MinContent,
            GridTrackSize::Auto,
        ],
        vec![
            cap,
            GridTrackSize::MaxContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
        vec![
            cap,
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
            GridTrackSize::LengthPx(8.0),
        ],
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(-1.0)),
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(f32::INFINITY)),
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(f32::NAN)),
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ],
    ] {
        let input = input_with_tracks(tracks, &PAYLOAD_HEIGHTS, LayoutAlignContent::FlexStart);
        assert!(TaffyLayoutEngine.compute(&input).is_err());
    }

    let valid_tracks = ordered_tracks([0, 1, 2, 3], GridTrackLengthPercentage::LengthPx(20.0));
    let unsupported_alignment = input_with_tracks(
        valid_tracks.clone(),
        &PAYLOAD_HEIGHTS,
        LayoutAlignContent::Center,
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&unsupported_alignment),
        Err(crate::LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "align-content",
        }) if node == id(1)
    ));

    let too_few_rows = input_with_tracks(
        valid_tracks.clone(),
        &PAYLOAD_HEIGHTS[..4],
        LayoutAlignContent::FlexStart,
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&too_few_rows),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));

    let too_many_rows = input_with_tracks(
        valid_tracks.clone(),
        &[20.0, 10.0, 12.0, 6.0, 20.0, 10.0],
        LayoutAlignContent::FlexStart,
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&too_many_rows),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));

    let legacy_three_member_with_six_items = input_with_tracks(
        vec![cap, GridTrackSize::MinContent, GridTrackSize::Auto],
        &PAYLOAD_HEIGHTS,
        LayoutAlignContent::FlexStart,
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&legacy_three_member_with_six_items),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));

    let mut unrelated_profile = input_with_tracks(
        valid_tracks,
        &PAYLOAD_HEIGHTS,
        LayoutAlignContent::FlexStart,
    );
    unrelated_profile
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    assert!(TaffyLayoutEngine.compute(&unrelated_profile).is_err());
}
