use super::*;
use serde_json::Value;

const LETTER_EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/expected.json"
);
const ALPHANUMERIC_EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/expected.json"
);
const HYPHENATED_EXPECTED: &str = include_str!(
    "../../../../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/expected.json"
);

#[test]
fn intrinsic_auto_rows_measure_ascii_letter_words_against_independent_scenes() {
    assert_scenes(LETTER_EXPECTED, 10);
}

#[test]
fn intrinsic_auto_rows_measure_ascii_alphanumeric_words_against_independent_scenes() {
    assert_scenes(ALPHANUMERIC_EXPECTED, 10);
}

#[test]
fn intrinsic_auto_rows_measure_ascii_hyphenated_words_against_independent_scenes() {
    assert_scenes(HYPHENATED_EXPECTED, 9);
}

fn assert_scenes(expected_json: &str, expected_count: usize) {
    let expected: Value = serde_json::from_str(expected_json)
        .expect("ASCII word independent expectations should be valid JSON");
    let cases = expected["cases"]
        .as_array()
        .expect("independent oracle should contain each scene");
    assert_eq!(cases.len(), expected_count);

    for case in cases {
        let id = case["id"].as_str().expect("scene id should be a string");
        let source = case["source"]
            .as_str()
            .expect("word source should be a string");
        let columns = case["columnsMode"]
            .as_str()
            .expect("column mode should be a string");
        let reverse = case["sourceOrder"].as_str() == Some("reverse");
        let expected_column = case["independentExpectedColumnsCssPx"]
            .as_f64()
            .expect("independent column expectation should be numeric")
            as f32;
        let expected_lines = case["independentExpectedLongTextLineCount"]
            .as_u64()
            .expect("independent line count should be numeric")
            as usize;
        let mut fixture = Fixture::with_tracks(columns, 0.0, "0");
        fixture.set_text_source(source);
        let short_before = if reverse {
            Some(fixture.wrapped_text)
        } else {
            Some(fixture.last)
        };
        let (short_item, short_source) = fixture.add_text_item_before("A", short_before);
        let output = fixture.compute().unwrap_or_else(|error| {
            panic!("{id}: style-to-layout rejected supported ASCII words: {error:?}")
        });
        let frame = |node: HostNodeHandle| {
            let frame = output.layout.frames[&node.id()];
            [frame.x, frame.y, frame.width, frame.height]
        };
        let long_height = expected_lines as f32 * 10.0;
        let long_y = if reverse { 20.0 } else { 10.0 };
        let short_y = if reverse { 10.0 } else { 10.0 + long_height };
        assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0], "{id}");
        assert_eq!(frame(fixture.first), [0.0, 0.0, 0.0, 10.0], "{id}");
        assert_frame_close(
            frame(fixture.wrapped_text),
            [0.0, long_y, expected_column, long_height],
            id,
        );
        assert_frame_close(frame(short_item), [0.0, short_y, expected_column, 10.0], id);
        assert_eq!(frame(fixture.last)[1], 20.0 + long_height, "{id}");

        let long_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
        let short_layout = &output.layout.text_layouts[&short_item.id()];
        assert_eq!(long_layout.input.sources.len(), 1, "{id}");
        assert_eq!(
            long_layout.input.sources[0].node,
            fixture.text_source.id(),
            "{id}"
        );
        assert_eq!(
            long_layout.input.sources[0].text, source,
            "{id}: source preservation"
        );
        assert_eq!(long_layout.layout.lines.len(), expected_lines, "{id}");
        assert_eq!(long_layout.layout.height, long_height, "{id}");
        assert_eq!(long_layout.revision, output.layout.revision, "{id}");
        assert_eq!(short_layout.input.sources.len(), 1, "{id}");
        assert_eq!(
            short_layout.input.sources[0].node,
            short_source.id(),
            "{id}"
        );
        assert_eq!(short_layout.input.sources[0].text, "A", "{id}");
        assert_eq!(short_layout.layout.lines.len(), 1, "{id}");

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.grid.id())
            .expect("Grid track trace should be retained");
        let expected_rows = case["independentExpectedRowsCssPx"]
            .as_array()
            .expect("independent oracle should list used rows")
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
        assert_eq!(trace.used_rows.used_track_list, expected_rows, "{id}");
        let used_column = trace
            .used_columns
            .used_track_list
            .strip_suffix("px")
            .expect("used column should be CSS px")
            .parse::<f32>()
            .expect("used column should be numeric");
        assert!((used_column - expected_column).abs() <= 0.5, "{id}");
        let row_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .expect("Grid trace should retain a row sizing pass");
        for (item, value) in [(fixture.wrapped_text, long_height), (short_item, 10.0)] {
            let contribution = row_pass
                .contributions
                .iter()
                .find(|contribution| contribution.item == item.id())
                .expect("each text item should retain its own row contribution");
            assert_eq!(
                contribution.kind,
                LayoutGridTrackSizingContributionKind::MinContent,
                "{id}"
            );
            assert_eq!(contribution.value, value, "{id}");
        }
    }
}

fn assert_frame_close(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.5,
            "{label}: frame component {component} expected {expected}, observed {actual}"
        );
    }
}
