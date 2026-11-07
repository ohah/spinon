use super::*;

fn input_with_track_list(tracks: Vec<GridTrackSize>, payload_heights: [f32; 4]) -> LayoutInput {
    let root = id(1);
    let explicit = id(2);
    let row_items = [id(3), id(5), id(7), id(9)];
    let payloads = [id(4), id(6), id(8), id(10)];
    let mut nodes = vec![LayoutNode {
        id: root,
        children: [
            explicit,
            row_items[0],
            row_items[1],
            row_items[2],
            row_items[3],
        ]
        .to_vec(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            box_sizing: LayoutBoxSizing::BorderBox,
            width: LayoutDimension::Fixed(40.0),
            height: LayoutDimension::Fixed(100.0),
            align_content: Some(LayoutAlignContent::FlexStart),
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
    for ((item, payload), payload_height) in
        row_items.into_iter().zip(payloads).zip(payload_heights)
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
        grid_items: BTreeMap::from(
            [
                explicit,
                row_items[0],
                row_items[1],
                row_items[2],
                row_items[3],
            ]
            .map(|item| (item, GridItemInput::default())),
        ),
        text_paragraphs: BTreeMap::new(),
    }
}

fn input_with_two_fit_content_rows_and_minimum_floor() -> LayoutInput {
    let mut input = input_with_cap(GridTrackLengthPercentage::Percentage(0.2), 28.0);
    input.grid_containers.get_mut(&id(1)).unwrap().auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.4)),
    ]);
    input.nodes[3].children = vec![id(6)];
    input.nodes[3].style.height = LayoutDimension::Auto;
    input.nodes[3].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes.push(LayoutNode {
        id: id(6),
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Fixed(34.0),
            ..LayoutStyle::default()
        },
    });
    input
}

#[test]
fn root_html_fit_content_auto_row_track_lists_repeat_in_order_for_implicit_rows() {
    for (tracks, payload_heights, expected_rows) in [
        (
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.4)),
            ],
            [34.0, 34.0, 50.0, 6.0],
            [10.0, 20.0, 34.0, 20.0, 6.0],
        ),
        (
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.1)),
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.7)),
            ],
            [34.0, 30.0, 50.0, 25.0],
            [10.0, 10.0, 20.0, 50.0, 10.0],
        ),
        (
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::LengthPx(12.0),
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.8)),
            ],
            [34.0, 10.0, 50.0, 6.0],
            [10.0, 20.0, 12.0, 50.0, 6.0],
        ),
        (
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::LengthPx(12.0),
                GridTrackSize::Percentage(0.1),
            ],
            [34.0, 10.0, 8.0, 34.0],
            [10.0, 20.0, 12.0, 10.0, 20.0],
        ),
    ] {
        let output = TaffyLayoutEngine
            .compute(&input_with_track_list(tracks, payload_heights))
            .expect("supported fit-content track list should size implicit rows in order");
        assert_eq!(final_rows(&output), expected_rows);

        let mut y = expected_rows[0];
        for index in 0..4 {
            let item = id(3 + (index * 2) as u64);
            let payload = id(4 + (index * 2) as u64);
            let item_frame = &output.frames[&item];
            close(item_frame.x, 0.0, "list item x");
            close(item_frame.y, y, "list item y");
            close(item_frame.width, 40.0, "list item width");
            close(
                item_frame.height,
                expected_rows[index + 1],
                "list item height",
            );

            let payload_frame = &output.frames[&payload];
            close(payload_frame.x, 0.0, "list payload x");
            close(payload_frame.y, y, "list payload y");
            close(payload_frame.width, 40.0, "list payload width");
            close(
                payload_frame.height,
                payload_heights[index],
                "list payload height",
            );
            y += expected_rows[index + 1];
        }
    }
}

#[test]
fn root_html_fit_content_auto_row_list_preserves_minimum_above_first_cap() {
    let output = TaffyLayoutEngine
        .compute(&input_with_two_fit_content_rows_and_minimum_floor())
        .expect("a row minimum may exceed the first repeated fit-content cap");
    assert_eq!(final_rows(&output), [10.0, 28.0, 34.0]);
    for (node, y, height) in [(id(2), 0.0, 10.0), (id(3), 10.0, 28.0), (id(4), 38.0, 34.0)] {
        let frame = &output.frames[&node];
        close(frame.x, 0.0, "minimum floor item x");
        close(frame.y, y, "minimum floor item y");
        close(frame.width, 40.0, "minimum floor item width");
        close(frame.height, height, "minimum floor item height");
    }
    for (node, y) in [(id(5), 10.0), (id(6), 38.0)] {
        let frame = &output.frames[&node];
        close(frame.y, y, "minimum floor payload y");
        close(frame.height, 34.0, "minimum floor payload height");
    }

    let mut zero_and_overflow = input_with_two_fit_content_rows_and_minimum_floor();
    zero_and_overflow.nodes[2].style.min_height = LayoutDimension::Fixed(0.0);
    zero_and_overflow
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.0)),
        GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(1.2)),
    ]);
    let output = TaffyLayoutEngine
        .compute(&zero_and_overflow)
        .expect("zero and over-100% list caps should retain their fit-content sizing functions");
    assert_eq!(final_rows(&output), [10.0, 0.0, 34.0]);
    close(output.frames[&id(3)].height, 0.0, "zero-cap first row");
    close(output.frames[&id(4)].y, 10.0, "following over-100% row y");
    close(
        output.frames[&id(4)].height,
        34.0,
        "over-100% cap above max-content",
    );
}

#[test]
fn root_html_fit_content_auto_row_lists_reject_unmodeled_minimum_floor_shape() {
    let mut input = input_with_track_list(
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.4)),
        ],
        [34.0, 34.0, 50.0, 6.0],
    );
    input.nodes[2].style.min_height = LayoutDimension::Fixed(28.0);
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));
}
