use super::*;

#[path = "normal_whitespace/rejects_invalid.rs"]
mod rejects_invalid;

const ROOT_HTML_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-combined-multiple-text-precomparison.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/expected.json"
);
const LONG_ITEM: &str = "<div id=\"long-text\" class=\"item wrapped-text\">A A A A A A</div>";
const SHORT_ITEM: &str = "<div id=\"short-text\" class=\"item wrapped-text\">A</div>";

struct TextVariant {
    name: &'static str,
    long_html: &'static str,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    long_dom: &'static str,
    short_html: &'static str,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    short_dom: &'static str,
}

const VARIANTS: [TextVariant; 6] = [
    TextVariant {
        name: "single-spaces",
        long_html: "A A A A A A",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: "A A A A A A",
        short_html: "A",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: "A",
    },
    TextVariant {
        name: "repeated-and-edge-spaces",
        long_html: "  A   A A  A A A  ",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: "  A   A A  A A A  ",
        short_html: "  A  ",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: "  A  ",
    },
    TextVariant {
        name: "tabs",
        long_html: "A\tA\tA\tA\tA\tA",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: "A\tA\tA\tA\tA\tA",
        short_html: "\tA\t",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: "\tA\t",
    },
    TextVariant {
        name: "segment-breaks",
        long_html: "A\nA\nA\nA\nA\nA",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: "A\nA\nA\nA\nA\nA",
        short_html: "\nA\n",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: "\nA\n",
    },
    TextVariant {
        name: "character-reference-carriage-returns",
        long_html: "A&#x0d;A&#x0d;A&#x0d;A&#x0d;A&#x0d;A",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: "A\rA\rA\rA\rA\rA",
        short_html: "&#x0d;A&#x0d;",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: "\rA\r",
    },
    TextVariant {
        name: "mixed-whitespace",
        long_html: " \tA\t A\nA \r\n A \tA\nA  ",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        long_dom: " \tA\t A\nA \n A \tA\nA  ",
        short_html: " \tA\r\n ",
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        short_dom: " \tA\n ",
    },
];

fn source_for(variant: &TextVariant, columns: &str, reverse: bool) -> String {
    let long_item = format!(
        "<div id=\"long-text\" class=\"item wrapped-text\">{}</div>",
        variant.long_html
    );
    let short_item = format!(
        "<div id=\"short-text\" class=\"item wrapped-text\">{}</div>",
        variant.short_html
    );
    let mut source = ROOT_HTML_SOURCE
        .replace(
            "grid-template-columns: min-content;",
            &format!("grid-template-columns: {columns};"),
        )
        .replacen(LONG_ITEM, &long_item, 1)
        .replacen(SHORT_ITEM, &short_item, 1);
    if reverse {
        let forward = format!("        {long_item}\n        {short_item}");
        let reversed = format!("        {short_item}\n        {long_item}");
        assert!(
            source.contains(&forward),
            "variant={} source order",
            variant.name
        );
        source = source.replacen(&forward, &reversed, 1);
    }
    source
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn expected_rows(expected: &Value, columns: &str, reverse: bool) -> String {
    let key = if reverse {
        "reverseRowsCssPx"
    } else {
        "forwardRowsCssPx"
    };
    expected["expected"][columns][key]
        .as_array()
        .expect("the independent oracle should list every used row")
        .iter()
        .map(|row| format!("{}px", row.as_u64().expect("row size should be integral")))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn parsed_root_html_normal_whitespace_matches_all_24_reference_scenes() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("normal whitespace expected values should be valid JSON");
    assert_eq!(expected["matrix"]["sceneCount"].as_u64(), Some(24));
    assert_eq!(
        expected["matrix"]["geometryMismatchesAgainstSingleSpaceBaseline"].as_u64(),
        Some(0)
    );

    for columns in ["min-content", "auto"] {
        let expected_profile = &expected["expected"][columns];
        let expected_column = expected_profile["usedColumnsCssPx"]
            .as_f64()
            .expect("expected column should be numeric") as f32;
        let long_lines = expected_profile["longTextLines"]
            .as_u64()
            .expect("long text line count should be numeric") as usize;
        let long_height = long_lines as f32 * 10.0;
        let column_range = if columns == "min-content" {
            (6.0, 7.0)
        } else {
            (39.5, 40.5)
        };

        for variant in &VARIANTS {
            for reverse in [false, true] {
                let source = source_for(variant, columns, reverse);
                let (session, layout) = run(&source);
                let cascade = session.ua_cascade_snapshot();
                assert_eq!(
                    cascade.state,
                    crate::session::RuntimeUaCascadeState::Ready,
                    "variant={}, columns={columns}, reverse={reverse}: cascade={:?}",
                    variant.name,
                    cascade.error
                );
                assert_eq!(
                    layout.state,
                    crate::session::RuntimeLayoutState::Ready,
                    "variant={}, columns={columns}, reverse={reverse}: layout={:?}; cascade={:?}",
                    variant.name,
                    layout.error,
                    cascade.error
                );
                let completed = layout.completed.as_ref().expect("layout should complete");
                let scene = completed
                    .render_snapshot
                    .as_ref()
                    .expect("default Root HTML route should publish the text render snapshot");
                assert_eq!(
                    painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "Grid"),
                    [0.0, 0.0, 40.0, 40.0]
                );

                let long = painted_frame(scene, OpaqueCssSrgb::new(91, 168, 245), "long text");
                let short = painted_frame(scene, OpaqueCssSrgb::new(255, 209, 102), "short text");
                let first = painted_frame(scene, OpaqueCssSrgb::new(242, 102, 94), "first control");
                let last =
                    painted_frame(scene, OpaqueCssSrgb::new(85, 199, 145), "overflow control");
                let long_y = if reverse { 20.0 } else { 10.0 };
                let short_y = if reverse { 10.0 } else { 10.0 + long_height };
                assert_eq!(first, [0.0, 0.0, 1.0, 10.0]);
                assert!((column_range.0..=column_range.1).contains(&long[2]));
                assert!((column_range.0..=column_range.1).contains(&short[2]));
                assert_eq!(long[0], 0.0);
                assert_eq!(long[1], long_y);
                assert_eq!(long[3], long_height);
                assert_eq!(short[0], 0.0);
                assert_eq!(short[1], short_y);
                assert_eq!(short[2], long[2]);
                assert_eq!(short[3], 10.0);
                assert_eq!(last[0], 0.0);
                assert_eq!(last[1], 20.0 + long_height);
                assert_eq!(last[2], 1.0);
                assert_eq!(last[3], 34.0);
                assert!(last[1] + last[3] > 40.0);

                let node_for_color = |color| {
                    scene
                        .boxes()
                        .iter()
                        .find(|render_box| {
                            matches!(render_box.paint(), RuntimePaint::Opaque(actual) if actual == color)
                        })
                        .expect("each fixed Grid item should retain its color and node")
                        .node_id()
                };
                let first_id = node_for_color(OpaqueCssSrgb::new(242, 102, 94));
                let long_id = scene
                    .boxes()
                    .iter()
                    .find(|render_box| {
                        matches!(render_box.paint(), RuntimePaint::Opaque(color) if color == OpaqueCssSrgb::new(91, 168, 245))
                    })
                    .expect("long text owner should retain its painted box")
                    .node_id();
                let short_id = scene
                    .boxes()
                    .iter()
                    .find(|render_box| {
                        matches!(render_box.paint(), RuntimePaint::Opaque(color) if color == OpaqueCssSrgb::new(255, 209, 102))
                    })
                    .expect("short text owner should retain its painted box")
                    .node_id();
                let last_id = node_for_color(OpaqueCssSrgb::new(85, 199, 145));
                let order = scene
                    .boxes()
                    .iter()
                    .filter(|render_box| {
                        [
                            OpaqueCssSrgb::new(242, 102, 94),
                            OpaqueCssSrgb::new(91, 168, 245),
                            OpaqueCssSrgb::new(255, 209, 102),
                            OpaqueCssSrgb::new(85, 199, 145),
                        ]
                        .iter()
                        .any(|color| matches!(render_box.paint(), RuntimePaint::Opaque(actual) if actual == *color))
                    })
                    .map(|render_box| render_box.node_id())
                    .collect::<Vec<_>>();
                let expected_order = if reverse {
                    vec![first_id, short_id, long_id, last_id]
                } else {
                    vec![first_id, long_id, short_id, last_id]
                };
                assert_eq!(order, expected_order);
                assert_eq!(scene.texts().len(), 2);
                let expected_text_order = if reverse {
                    [short_id, long_id]
                } else {
                    [long_id, short_id]
                };
                assert_eq!(
                    scene
                        .texts()
                        .iter()
                        .map(|text| text.node_id())
                        .collect::<Vec<_>>(),
                    expected_text_order
                );

                for (item_index, item_id) in order.iter().enumerate() {
                    if *item_id != long_id && *item_id != short_id {
                        continue;
                    }
                    let box_index = scene
                        .boxes()
                        .iter()
                        .position(|render_box| render_box.node_id() == *item_id)
                        .expect("text item background should be present");
                    let text_index = scene
                        .texts()
                        .iter()
                        .position(|text| text.node_id() == *item_id)
                        .expect("text item should retain its text paint");
                    let box_command = scene
                        .paint_commands()
                        .iter()
                        .position(|command| {
                            matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == box_index)
                        })
                        .expect("text item background draw command should be present");
                    let text_command = scene
                        .paint_commands()
                        .iter()
                        .position(|command| {
                            matches!(command, spinon_render::RuntimePaintCommand::DrawText { index } if *index == text_index)
                        })
                        .expect("text draw command should be present");
                    assert!(box_command < text_command);
                    if let Some(next_item) = order.get(item_index + 1) {
                        let next_box_index = scene
                            .boxes()
                            .iter()
                            .position(|render_box| render_box.node_id() == *next_item)
                            .expect("next Grid item box should be present");
                        let next_box_command = scene
                            .paint_commands()
                            .iter()
                            .position(|command| {
                                matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == next_box_index)
                            })
                            .expect("next Grid item draw command should be present");
                        assert!(text_command < next_box_command);
                    }
                }

                for (node, expected_lines, expected_y) in
                    [(long_id, long_lines, long_y), (short_id, 1, short_y)]
                {
                    let text = scene
                        .texts()
                        .iter()
                        .find(|text| text.node_id() == node)
                        .expect("each text paint should retain its owner NodeId");
                    let baselines = text
                        .glyphs()
                        .iter()
                        .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                        .collect::<std::collections::BTreeSet<_>>();
                    assert_eq!(baselines.len(), expected_lines);
                    assert_eq!(text.origin_y_css_px(), expected_y);
                    assert_eq!(text.source().generation().get(), completed.key.generation);
                    assert_eq!(
                        text.source().document_revision().get(),
                        completed.key.document_revision
                    );
                    assert_eq!(
                        text.source().render_tree_revision().get(),
                        completed.key.render_tree_revision
                    );
                    assert_eq!(text.source().style_revision(), completed.key.style_revision);
                }

                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                assert_raw_sources_and_trace(RawSourceReviewCase {
                    completed,
                    long_id,
                    short_id,
                    variant,
                    reverse,
                    expected_column,
                    expected_rows: expected_rows(&expected, columns, reverse),
                    long_height,
                    long_lines,
                });
                #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
                let _ = (expected_column,);
            }
        }
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
struct RawSourceReviewCase<'a> {
    completed: &'a crate::session::RuntimeLayoutCompleted,
    long_id: spinon_core::NodeId,
    short_id: spinon_core::NodeId,
    variant: &'a TextVariant,
    expected_column: f32,
    expected_rows: String,
    long_height: f32,
    long_lines: usize,
    reverse: bool,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_raw_sources_and_trace(case: RawSourceReviewCase<'_>) {
    let RawSourceReviewCase {
        completed,
        long_id,
        short_id,
        variant,
        expected_column,
        expected_rows,
        long_height,
        long_lines,
        reverse,
    } = case;
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("C11.4 fixture diagnostics should retain the Root HTML Grid trace");
    let long_layout = diagnostics
        .item_text_layouts
        .iter()
        .find(|layout| layout.input.node == long_id)
        .expect("long text's layout snapshot should be retained by its Grid owner");
    let short_layout = diagnostics
        .item_text_layouts
        .iter()
        .find(|layout| layout.input.node == short_id)
        .expect("short text's layout snapshot should be retained by its Grid owner");
    assert_eq!(long_layout.input.sources.len(), 1);
    assert_eq!(long_layout.input.sources[0].text, variant.long_dom);
    assert_ne!(long_layout.input.sources[0].node, long_id);
    assert_eq!(short_layout.input.sources.len(), 1);
    assert_eq!(short_layout.input.sources[0].text, variant.short_dom);
    assert_ne!(short_layout.input.sources[0].node, short_id);
    assert_eq!(long_layout.revision, diagnostics.revision);
    assert_eq!(short_layout.revision, diagnostics.revision);
    assert_eq!(long_layout.layout.lines.len(), long_lines);
    assert_eq!(long_layout.layout.height, long_height);
    assert_eq!(short_layout.layout.lines.len(), 1);

    let long_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == long_id)
        .expect("long owner should have one Grid intrinsic input");
    let short_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == short_id)
        .expect("short owner should have one Grid intrinsic input");
    assert_ne!(long_input.source_order, short_input.source_order);
    assert_eq!(long_input.source_order < short_input.source_order, !reverse);
    assert_eq!(long_input.grid_container, short_input.grid_container);

    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == long_input.grid_container)
        .expect("Root HTML Grid should retain its track-sizing trace");
    assert_eq!(trace.used_rows.used_track_list, expected_rows);
    let used_column = trace
        .used_columns
        .used_track_list
        .strip_suffix("px")
        .expect("used column should be serialized in CSS px")
        .parse::<f32>()
        .expect("used column should be numeric");
    assert!((used_column - expected_column).abs() <= 0.5);
    let row_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .expect("Grid trace should retain its row sizing pass");
    for (node, value) in [(long_id, long_height), (short_id, 10.0)] {
        let contribution = row_pass
            .contributions
            .iter()
            .find(|contribution| contribution.item == node)
            .expect("each text owner should retain its row contribution");
        assert_eq!(
            contribution.kind,
            spinon_layout::LayoutGridTrackSizingContributionKind::MinContent
        );
        assert_eq!(contribution.value, value);
    }
}
