use super::*;

#[test]
fn intrinsic_columns_measure_multiple_text_items_as_independent_implicit_rows() {
    for (columns, column_range, long_lines, long_height, used_rows) in [
        (
            "min-content",
            (6.0, 7.0),
            6,
            60.0,
            ["10px 60px 10px 34px", "10px 10px 60px 34px"],
        ),
        (
            "max-content",
            (60.0, 70.0),
            1,
            10.0,
            ["10px 10px 10px 34px", "10px 10px 10px 34px"],
        ),
        (
            "auto",
            (40.0, 40.0),
            2,
            20.0,
            ["10px 20px 10px 34px", "10px 10px 20px 34px"],
        ),
        (
            "minmax(min-content,1fr)",
            (40.0, 40.0),
            2,
            20.0,
            ["10px 20px 10px 34px", "10px 10px 20px 34px"],
        ),
    ] {
        for reverse in [false, true] {
            let mut fixture = Fixture::with_tracks(columns, 0.0, "0");
            let before = if reverse {
                fixture.wrapped_text
            } else {
                fixture.last
            };
            let (short_text, short_source) = fixture.add_text_item_before("A", Some(before));
            let output = fixture
                .compute()
                .unwrap_or_else(|error| panic!("columns={columns}, reverse={reverse}: {error:?}"));
            let frame = |node: HostNodeHandle| {
                let frame = output.layout.frames[&node.id()];
                [frame.x, frame.y, frame.width, frame.height]
            };
            let long_frame = frame(fixture.wrapped_text);
            let short_frame = frame(short_text);
            let expected_long_y = if reverse { 20.0 } else { 10.0 };
            let expected_short_y = if reverse { 10.0 } else { 10.0 + long_height };
            let expected_last_y = 20.0 + long_height;
            assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0]);
            assert!(
                (column_range.0..=column_range.1).contains(&long_frame[2]),
                "columns={columns}: unexpected resolved column width {}",
                long_frame[2]
            );
            assert_eq!(long_frame[1], expected_long_y, "columns={columns}");
            assert_eq!(long_frame[3], long_height, "columns={columns}");
            assert_eq!(short_frame[1], expected_short_y, "columns={columns}");
            assert_eq!(short_frame[3], 10.0, "columns={columns}");
            assert_eq!(short_frame[2], long_frame[2], "columns={columns}");
            assert_eq!(frame(fixture.last)[1], expected_last_y, "columns={columns}");

            let long_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
            let short_layout = &output.layout.text_layouts[&short_text.id()];
            assert_eq!(long_layout.input.sources[0].node, fixture.text_source.id());
            assert_eq!(long_layout.input.sources[0].text, TEXT);
            assert_eq!(long_layout.layout.lines.len(), long_lines);
            assert_eq!(long_layout.layout.height, long_height);
            assert_eq!(long_layout.revision, output.layout.revision);
            assert_eq!(short_layout.input.sources[0].node, short_source.id());
            assert_eq!(short_layout.input.sources[0].text, "A");
            assert_eq!(short_layout.layout.lines.len(), 1);
            assert_eq!(short_layout.layout.height, 10.0);
            assert_eq!(short_layout.revision, output.layout.revision);

            let trace = output
                .layout
                .grid_track_sizing_traces
                .iter()
                .find(|trace| trace.node == fixture.grid.id())
                .expect("combined intrinsic columns and text rows should retain a trace");
            let order = usize::from(reverse);
            assert_eq!(trace.used_rows.used_track_list, used_rows[order]);
            let column_px = trace
                .used_columns
                .used_track_list
                .strip_suffix("px")
                .expect("the single used column should be recorded in CSS px")
                .parse::<f32>()
                .expect("the used column should be a finite number");
            assert!(
                (column_range.0..=column_range.1).contains(&column_px),
                "columns={columns}: trace column width {column_px}"
            );
            let row_pass = trace
                .passes
                .iter()
                .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
                .expect("combined route should record row contributions");
            for (item, contribution) in [(fixture.wrapped_text, long_height), (short_text, 10.0)] {
                let observed = row_pass
                    .contributions
                    .iter()
                    .find(|entry| entry.item == item.id())
                    .expect("each text item's row contribution should retain its identity");
                assert_eq!(
                    observed.kind,
                    LayoutGridTrackSizingContributionKind::MinContent
                );
                assert_eq!(observed.value, contribution);
            }
        }
    }
}

#[test]
fn intrinsic_columns_reject_an_invalid_second_text_item_before_layout_publication() {
    let mut fixture = Fixture::with_tracks("min-content", 0.0, "0");
    let (invalid_item, _) = fixture.add_text_item_before("B--1", Some(fixture.last));
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "grid-auto-rows",
            ..
        }) if node == invalid_item.id()
    ));
}

#[test]
fn combined_intrinsic_columns_keep_the_implicit_row_limit_atomic() {
    const MAX_TRACKS: usize = 10_000;

    let mut fixture = Fixture::with_tracks("min-content", 1.0, "0");
    fixture.add_text_item_before("A", Some(fixture.last));
    let final_item = fixture
        .add_empty_items(MAX_TRACKS - 4)
        .expect("a full combined track list should add trailing controls");
    let output = fixture
        .compute()
        .expect("combined intrinsic columns and text rows should accept 10,000 tracks");
    let final_frame = output.layout.frames[&final_item.id()];
    assert_eq!(
        [
            final_frame.x,
            final_frame.y,
            final_frame.width,
            final_frame.height
        ],
        [0.0, 10_109.0, 1.0, 1.0]
    );
    assert!(
        output.layout.grid_track_sizing_traces.is_empty(),
        "an over-budget optional trace must not discard combined frames"
    );

    let _ = fixture.add_empty_items(1);
    let error = match fixture.compute() {
        Err(error) => error,
        Ok(_) => panic!("the 10,001st combined implicit row should fail before layout"),
    };
    match error {
        StyleLayoutError::UnsupportedComputedValue {
            property, value, ..
        } => {
            assert_eq!(property, "grid-auto-rows");
            assert!(value.contains("exceed the 10,000-track limit"));
        }
        other => panic!("expected early combined Grid track limit rejection, got {other:?}"),
    }
}
