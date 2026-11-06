use super::*;
use serde_json::Value;
use spinon_render::{OpaqueCssSrgb, RuntimePaint};

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-runtime.html"
);
const LETTER_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/expected.json"
);
const ALPHANUMERIC_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/expected.json"
);
const HYPHENATED_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/expected.json"
);
const LONG_TEXT: &str = "<div id=\"long-text\" class=\"item\">AAAAAAAA</div>";
const SHORT_TEXT: &str = "<div id=\"short-text\" class=\"item\">A</div>";

#[test]
fn parsed_root_html_ascii_letter_words_match_all_independent_intrinsic_scenes() {
    assert_scenes(LETTER_EXPECTED, 10);
}

#[test]
fn parsed_root_html_ascii_alphanumeric_words_match_all_independent_intrinsic_scenes() {
    assert_scenes(ALPHANUMERIC_EXPECTED, 10);
}

#[test]
fn parsed_root_html_ascii_hyphenated_words_match_all_independent_intrinsic_scenes() {
    assert_scenes(HYPHENATED_EXPECTED, 9);
}

fn assert_scenes(expected_json: &str, expected_count: usize) {
    let expected: Value = serde_json::from_str(expected_json)
        .expect("ASCII word independent expectations should be valid JSON");
    let cases = expected["cases"]
        .as_array()
        .expect("independent oracle should contain the full scene matrix");
    assert_eq!(
        cases.len(),
        expected_count,
        "all precomparison scenes should execute"
    );

    for case in cases {
        let id = case["id"].as_str().expect("scene id should be a string");
        let text = case["source"]
            .as_str()
            .expect("text source should be a string");
        let columns = case["columnsMode"]
            .as_str()
            .expect("column mode should be a string");
        let reverse = case["sourceOrder"].as_str() == Some("reverse");
        let expected_column = case["independentExpectedColumnsCssPx"]
            .as_f64()
            .expect("independent track expectation should be numeric")
            as f32;
        let expected_long_lines = case["independentExpectedLongTextLineCount"]
            .as_u64()
            .expect("independent line count should be numeric")
            as usize;

        let long_item = format!("<div id=\"long-text\" class=\"item\">{text}</div>");
        let mut source = SOURCE
            .replace(
                "grid-template-columns: auto;",
                &format!("grid-template-columns: {columns};"),
            )
            .replacen(LONG_TEXT, &long_item, 1);
        assert!(
            source.contains(&long_item),
            "{id}: long text should be inserted"
        );
        if columns != "auto" || text != "AAAAAAAA" {
            assert_ne!(source, SOURCE, "{id}: scene mutation should be applied");
        }
        if reverse {
            let forward = format!("{long_item}\n        {SHORT_TEXT}");
            let reversed = format!("{SHORT_TEXT}\n        {long_item}");
            let reordered = source.replacen(&forward, &reversed, 1);
            assert_ne!(
                reordered, source,
                "{id}: reverse-order mutation should match"
            );
            source = reordered;
        }

        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{id}: cascade error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{id}: layout error={:?}; cascade error={:?}",
            layout.error,
            cascade.error
        );
        let completed = layout.completed.as_ref().expect("layout should complete");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete")
                .key,
            "{id}: style and layout revisions should match"
        );
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML route should publish the paint snapshot");
        assert_frame_values(
            painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "Grid"),
            [0.0, 0.0, 40.0, 40.0],
            id,
        );

        let expected_long_height = expected_long_lines as f32 * 10.0;
        let long_y = if reverse { 20.0 } else { 10.0 };
        let short_y = if reverse {
            10.0
        } else {
            10.0 + expected_long_height
        };
        for (item_id, color, expected_frame) in [
            (
                "first",
                OpaqueCssSrgb::new(242, 102, 94),
                [0.0, 0.0, 1.0, 10.0],
            ),
            (
                "long-text",
                OpaqueCssSrgb::new(91, 168, 245),
                [0.0, long_y, expected_column, expected_long_height],
            ),
            (
                "short-text",
                OpaqueCssSrgb::new(255, 209, 102),
                [0.0, short_y, expected_column, 10.0],
            ),
            (
                "last",
                OpaqueCssSrgb::new(85, 199, 145),
                [0.0, 20.0 + expected_long_height, 1.0, 34.0],
            ),
        ] {
            assert_frame_values(painted_frame(scene, color, item_id), expected_frame, id);
        }
        assert!(
            20.0 + expected_long_height + 34.0 > 40.0,
            "{id}: fixed last item should retain visible overflow geometry"
        );

        let expected_text_nodes = if reverse {
            ["short-text", "long-text"]
        } else {
            ["long-text", "short-text"]
        };
        assert_eq!(
            scene.texts().len(),
            2,
            "{id}: both text paints should remain"
        );
        for (index, text_id) in expected_text_nodes.iter().enumerate() {
            let color = if *text_id == "long-text" {
                OpaqueCssSrgb::new(91, 168, 245)
            } else {
                OpaqueCssSrgb::new(255, 209, 102)
            };
            let owner = scene
                .boxes()
                .iter()
                .find(|render_box| {
                    matches!(render_box.paint(), RuntimePaint::Opaque(actual) if actual == color)
                })
                .expect("text item should retain its owner box")
                .node_id();
            let text_paint = &scene.texts()[index];
            assert_eq!(text_paint.node_id(), owner, "{id}: text owner identity");
            let expected_lines = if *text_id == "long-text" {
                expected_long_lines
            } else {
                1
            };
            let baselines = text_paint
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(baselines.len(), expected_lines, "{id}: {text_id} lines");
            let expected_y = if *text_id == "long-text" {
                long_y
            } else {
                short_y
            };
            assert_eq!(
                text_paint.origin_y_css_px(),
                expected_y,
                "{id}: {text_id} origin"
            );
            assert_eq!(
                text_paint.source().generation().get(),
                completed.key.generation
            );
            assert_eq!(
                text_paint.source().document_revision().get(),
                completed.key.document_revision
            );
            assert_eq!(
                text_paint.source().render_tree_revision().get(),
                completed.key.render_tree_revision
            );
            assert_eq!(
                text_paint.source().style_revision(),
                completed.key.style_revision
            );
        }

        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        assert_raw_source_and_track_trace(
            completed,
            scene,
            case,
            text,
            expected_column,
            expected_long_lines,
            reverse,
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_raw_source_and_track_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected: &Value,
    source: &str,
    expected_column: f32,
    expected_long_lines: usize,
    reverse: bool,
) {
    let long_owner = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(91, 168, 245))
        })
        .expect("long text owner should be visible")
        .node_id();
    let short_owner = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(255, 209, 102))
        })
        .expect("short text owner should be visible")
        .node_id();
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture-test route should retain the Grid track trace");
    let long_layout = diagnostics
        .item_text_layouts
        .iter()
        .find(|layout| layout.input.node == long_owner)
        .expect("long text layout should retain its owning item identity");
    let short_layout = diagnostics
        .item_text_layouts
        .iter()
        .find(|layout| layout.input.node == short_owner)
        .expect("short text layout should retain its owning item identity");
    assert_eq!(long_layout.input.sources.len(), 1);
    assert_eq!(long_layout.input.sources[0].text, source);
    assert_ne!(long_layout.input.sources[0].node, long_owner);
    assert_eq!(short_layout.input.sources.len(), 1);
    assert_eq!(short_layout.input.sources[0].text, "A");
    assert_ne!(short_layout.input.sources[0].node, short_owner);
    assert_eq!(long_layout.revision, diagnostics.revision);
    assert_eq!(short_layout.revision, diagnostics.revision);
    assert_eq!(long_layout.layout.lines.len(), expected_long_lines);
    assert_eq!(long_layout.layout.height, expected_long_lines as f32 * 10.0);
    assert_eq!(short_layout.layout.lines.len(), 1);

    let long_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == long_owner)
        .expect("long item should have one intrinsic input");
    let short_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == short_owner)
        .expect("short item should have one intrinsic input");
    assert_eq!(long_input.grid_container, short_input.grid_container);
    assert_ne!(long_input.source_order, short_input.source_order);
    assert_eq!(long_input.source_order < short_input.source_order, !reverse);

    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == long_input.grid_container)
        .expect("the owning Grid should retain its track-sizing trace");
    let used_column = trace
        .used_columns
        .used_track_list
        .strip_suffix("px")
        .expect("one used column should be reported in CSS px")
        .parse::<f32>()
        .expect("used column should be numeric");
    assert!((used_column - expected_column).abs() <= TOLERANCE);
    let expected_rows = expected["independentExpectedRowsCssPx"]
        .as_array()
        .expect("independent oracle should list each used row")
        .iter()
        .map(|row| format!("{}px", row.as_u64().expect("row size should be integral")))
        .collect::<Vec<_>>()
        .join(" ");
    let expected_rows = expected_rows
        .split_whitespace()
        .take(3)
        .chain(std::iter::once("34px"))
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(trace.used_rows.used_track_list, expected_rows);

    let row_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .expect("the Grid trace should retain a row sizing pass");
    for (node, height) in [
        (long_owner, expected_long_lines as f32 * 10.0),
        (short_owner, 10.0),
    ] {
        let contribution = row_pass
            .contributions
            .iter()
            .find(|contribution| contribution.item == node)
            .expect("each text owner should retain its min-content contribution");
        assert_eq!(
            contribution.kind,
            spinon_layout::LayoutGridTrackSizingContributionKind::MinContent
        );
        assert_eq!(contribution.value, height);
    }
}

fn assert_frame_values(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "{label}: frame component {component} expected {expected}, observed {actual}"
        );
    }
}
