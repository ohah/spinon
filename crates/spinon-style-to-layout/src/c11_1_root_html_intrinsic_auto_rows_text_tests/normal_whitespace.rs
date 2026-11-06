use super::*;

const INPUT_VARIANTS: [(&str, &str, &str); 6] = [
    ("single-spaces", TEXT, "A"),
    ("repeated-and-edge-spaces", "  A   A A  A A A  ", "  A  "),
    ("tabs", "A\tA\tA\tA\tA\tA", "\tA\t"),
    ("segment-breaks", "A\nA\nA\nA\nA\nA", "\nA\n"),
    (
        "character-reference-carriage-returns",
        "A\rA\rA\rA\rA\rA",
        "\rA\r",
    ),
    (
        "mixed-whitespace",
        " \tA\t A\nA \r\n A \tA\nA  ",
        " \tA\r\n ",
    ),
];

#[test]
fn intrinsic_auto_rows_preserve_normal_whitespace_sources_across_the_24_scene_matrix() {
    for columns in ["min-content", "auto"] {
        let (column_range, long_lines, long_height, rows_by_order) = match columns {
            "min-content" => (
                (6.0, 7.0),
                6,
                60.0,
                ["10px 60px 10px 34px", "10px 10px 60px 34px"],
            ),
            "auto" => (
                (39.5, 40.5),
                2,
                20.0,
                ["10px 20px 10px 34px", "10px 10px 20px 34px"],
            ),
            _ => unreachable!("fixed column test matrix"),
        };

        for (variant, long_source, short_source) in INPUT_VARIANTS {
            for reverse in [false, true] {
                let mut fixture = Fixture::with_tracks(columns, 0.0, "0");
                fixture.set_text_source(long_source);
                let before = if reverse {
                    fixture.wrapped_text
                } else {
                    fixture.last
                };
                let (short_item, short_text_source) =
                    fixture.add_text_item_before(short_source, Some(before));
                let output = fixture.compute().unwrap_or_else(|error| {
                    panic!("variant={variant}, columns={columns}, reverse={reverse}: {error:?}")
                });

                let frame = |node: HostNodeHandle| {
                    let frame = output.layout.frames[&node.id()];
                    [frame.x, frame.y, frame.width, frame.height]
                };
                let long_frame = frame(fixture.wrapped_text);
                let short_frame = frame(short_item);
                let long_y = if reverse { 20.0 } else { 10.0 };
                let short_y = if reverse { 10.0 } else { 10.0 + long_height };
                let source_order_index = usize::from(reverse);
                assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0]);
                assert_eq!(frame(fixture.first), [0.0, 0.0, 0.0, 10.0]);
                assert!((column_range.0..=column_range.1).contains(&long_frame[2]));
                assert_eq!(long_frame[0], 0.0);
                assert_eq!(long_frame[1], long_y);
                assert_eq!(long_frame[3], long_height);
                assert_eq!(short_frame[0], 0.0);
                assert_eq!(short_frame[1], short_y);
                assert_eq!(short_frame[2], long_frame[2]);
                assert_eq!(short_frame[3], 10.0);
                assert_eq!(frame(fixture.last)[1], 20.0 + long_height);
                assert!(frame(fixture.last)[1] + frame(fixture.last)[3] > frame(fixture.grid)[3]);

                let long_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
                let short_layout = &output.layout.text_layouts[&short_item.id()];
                assert_eq!(long_layout.input.sources.len(), 1);
                assert_eq!(long_layout.input.sources[0].node, fixture.text_source.id());
                assert_eq!(long_layout.input.sources[0].text, long_source);
                assert_eq!(long_layout.layout.lines.len(), long_lines);
                assert_eq!(long_layout.layout.height, long_height);
                assert_eq!(long_layout.revision, output.layout.revision);
                assert_eq!(short_layout.input.sources.len(), 1);
                assert_eq!(short_layout.input.sources[0].node, short_text_source.id());
                assert_eq!(short_layout.input.sources[0].text, short_source);
                assert_eq!(short_layout.layout.lines.len(), 1);
                assert_eq!(short_layout.layout.height, 10.0);
                assert_eq!(short_layout.revision, output.layout.revision);

                let trace = output
                    .layout
                    .grid_track_sizing_traces
                    .iter()
                    .find(|trace| trace.node == fixture.grid.id())
                    .expect("normal whitespace should retain the Grid track-sizing trace");
                assert_eq!(
                    trace.used_rows.used_track_list,
                    rows_by_order[source_order_index]
                );
                let used_column = trace
                    .used_columns
                    .used_track_list
                    .strip_suffix("px")
                    .expect("used column should be a single CSS-pixel track")
                    .parse::<f32>()
                    .expect("used column should be numeric");
                assert!((column_range.0..=column_range.1).contains(&used_column));
                let row_pass = trace
                    .passes
                    .iter()
                    .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
                    .expect("normal whitespace should retain row contributions");
                for (item, contribution) in
                    [(fixture.wrapped_text, long_height), (short_item, 10.0)]
                {
                    let observed = row_pass
                        .contributions
                        .iter()
                        .find(|entry| entry.item == item.id())
                        .expect("each direct text item should retain its row contribution");
                    assert_eq!(
                        observed.kind,
                        LayoutGridTrackSizingContributionKind::MinContent
                    );
                    assert_eq!(observed.value, contribution);
                }
            }
        }
    }
}

#[test]
fn intrinsic_auto_rows_atomically_reject_non_collapsible_or_invalid_text_sources() {
    for (label, source) in [
        ("empty", ""),
        ("whitespace-only", " \t\n\r"),
        ("form-feed", "A\u{000c}A"),
        ("vertical-tab", "A\u{000b}A"),
        ("nbsp", "A\u{00a0}A"),
        ("nel", "A\u{0085}A"),
        ("unicode-space", "A\u{2003}A"),
        ("punctuation-token", "A-B"),
        ("letter-digit-punctuation-token", "A1-B"),
    ] {
        let mut fixture = Fixture::new();
        let (invalid_item, _) = fixture.add_text_item_before(source, Some(fixture.last));
        let result = fixture.compute();
        assert!(
            matches!(
                result,
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-auto-rows",
                    ..
                }) if node == invalid_item.id()
            ),
            "case={label}, source={source:?}: unexpected result {result:?}"
        );
    }
}
