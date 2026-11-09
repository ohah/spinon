use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
};

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-fit-content-positions-runtime.html"
);
const CHROME_MATRIX: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/chrome157-matrix-observation.ndjson"
);
const ITEM_COLORS: [OpaqueCssSrgb; 9] = [
    OpaqueCssSrgb::new(229, 72, 77),
    OpaqueCssSrgb::new(242, 102, 94),
    OpaqueCssSrgb::new(91, 168, 245),
    OpaqueCssSrgb::new(85, 199, 145),
    OpaqueCssSrgb::new(168, 85, 247),
    OpaqueCssSrgb::new(250, 204, 21),
    OpaqueCssSrgb::new(14, 165, 233),
    OpaqueCssSrgb::new(251, 146, 60),
    OpaqueCssSrgb::new(45, 212, 191),
];
const PAYLOAD_TEXT: [&str; 8] = [
    "A A A A A A",
    "AAAA AAAA A",
    "AA AA AA AA",
    "AAA AAA AAA",
    "AAA AAA",
    "A A A A A A",
    "A A A A A A A A",
    "AA",
];

fn expected_frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("expected frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| {
        values[index]
            .as_f64()
            .expect("frame component should be numeric") as f32
    })
}

fn computed_track(value: &str) -> ComputedCssGridTrackSize {
    match value {
        "min-content" => ComputedCssGridTrackSize::MinContent,
        "max-content" => ComputedCssGridTrackSize::MaxContent,
        "auto" => ComputedCssGridTrackSize::Auto,
        value if value.starts_with("fit-content(") && value.ends_with(')') => {
            let cap = &value[12..value.len() - 1];
            let percentage = cap.ends_with('%');
            let number = if percentage {
                cap.strip_suffix('%').unwrap()
            } else {
                cap.strip_suffix("px").unwrap()
            }
            .parse::<f32>()
            .expect("fit-content cap is numeric");
            ComputedCssGridTrackSize::FitContent(if percentage {
                ComputedCssGridTrackLengthPercentage::Percentage(number / 100.0)
            } else {
                ComputedCssGridTrackLengthPercentage::LengthPx(number)
            })
        }
        _ => panic!("unsupported matrix track {value:?}"),
    }
}

fn source_for_scene(scene: &Value) -> String {
    let authored = scene["authoredAutoRows"]
        .as_array()
        .expect("authored track list should be present")
        .iter()
        .map(|track| track.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    let source = replace_once(
        SOURCE,
        "--tracks: fit-content(20px) min-content min-content max-content max-content auto;",
        &format!("--tracks: {authored};"),
    );
    replace_once(
        &source,
        "--align: start;",
        &format!("--align: {};", scene["align"].as_str().unwrap()),
    )
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.01,
            "{label} component {component}: actual={actual:?}, expected={expected:?} ± 0.01 CSS px"
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    scene: &Value,
    label: &str,
) {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("track trace feature exposes Grid sizing diagnostics");
    assert_eq!(diagnostics.traces.len(), 1, "{label}: one Grid trace");
    let trace = &diagnostics.traces[0];
    let used_rows = usize::from(trace.used_rows.explicit_track_count)
        + usize::from(trace.used_rows.negative_implicit_track_count)
        + usize::from(trace.used_rows.positive_implicit_track_count);
    assert_eq!(used_rows, 9, "{label}: explicit plus auto-placed rows");
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0, "{label}");
    assert_eq!(
        trace.used_rows.used_track_list.split_whitespace().count(),
        9,
        "{label}: resolved track-list count"
    );
    assert!(
        trace.used_rows.collapsed_track_indices.is_empty(),
        "{label}"
    );
    let rows_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row track-sizing pass exists");
    let final_stage = rows_pass.stages.last().expect("final sizing stage exists");
    let actual = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    let expected = scene["rows"].as_array().expect("matrix rows are present");
    assert_eq!(
        actual.len(),
        expected.len(),
        "{label}: final trace row count"
    );
    for (row, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let expected = expected.as_f64().unwrap() as f32;
        assert!(
            actual.base_size.is_finite() && (actual.base_size - expected).abs() <= 0.01,
            "{label}: final trace row {row}: actual={}, expected={expected} ± 0.01px",
            actual.base_size
        );
    }
}

fn frame_for_node(completed: &crate::session::RuntimeLayoutCompleted, node_id: u64) -> [f32; 4] {
    let frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .unwrap_or_else(|| panic!("expected RuntimeSession frame for node {node_id}"));
    [frame.x, frame.y, frame.width, frame.height]
}

fn assert_position_matrix_scene(index: usize, scene: &Value) -> bool {
    let source = source_for_scene(scene);
    let (session, layout) = run(&source);
    let cascade_snapshot = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "scene {index}: layout={:?}; cascade={:?}",
        layout.error,
        cascade_snapshot.error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("scene should publish layout");
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("scene should publish render snapshot");
    let cascade = cascade_snapshot
        .completed
        .expect("scene should publish computed styles");
    assert_eq!(completed.key, cascade.key);
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("fixture has a Grid owner");
    let expected_tracks = scene["authoredAutoRows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|track| computed_track(track.as_str().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(expected_tracks),
        "scene {index}: typed track order"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some(scene["computedAutoRows"].as_str().unwrap()),
        "scene {index}: computed track serialization"
    );
    assert_eq!(
        grid.properties.get("align-content").map(String::as_str),
        Some(scene["align"].as_str().unwrap()),
        "scene {index}: computed alignment"
    );

    let grid_frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == grid.node_id.get())
        .expect("Grid frame should be present");
    assert_frame(
        [
            grid_frame.x,
            grid_frame.y,
            grid_frame.width,
            grid_frame.height,
        ],
        [0.0, 0.0, 40.0, 220.0],
        &format!("scene {index} Grid"),
    );

    let items = scene["items"]
        .as_array()
        .expect("scene items should be an array");
    assert_eq!(items.len(), ITEM_COLORS.len());
    for (item_index, (item, color)) in items.iter().zip(ITEM_COLORS).enumerate() {
        let expected = expected_frame(&item["frame"]);
        if expected[3] > 0.0 {
            assert_frame(
                frame_for_paint(render, color),
                expected,
                &format!("scene {index} item {item_index}"),
            );
        } else {
            assert!(
                completed.frames.iter().any(|frame| {
                    [frame.x, frame.y, frame.width, frame.height]
                        .into_iter()
                        .zip(expected)
                        .all(|(actual, expected)| (actual - expected).abs() <= 0.01)
                }),
                "scene {index}: zero-height item {item_index} frame missing: {expected:?}"
            );
        }
    }

    let rows = scene["rows"]
        .as_array()
        .expect("used row sizes should be an array");
    assert_eq!(rows.len(), items.len());
    for (row, (item, expected_row)) in items.iter().zip(rows).enumerate() {
        let item_frame = expected_frame(&item["frame"]);
        let row_size = expected_row.as_f64().unwrap() as f32;
        assert_frame(
            [item_frame[3], 0.0, 0.0, 0.0],
            [row_size, 0.0, 0.0, 0.0],
            &format!("scene {index} row {row}"),
        );
    }

    let texts = render.texts().iter().collect::<Vec<_>>();
    assert_eq!(texts.len(), PAYLOAD_TEXT.len());
    let mut overflow = false;
    for (payload_index, text) in texts.iter().enumerate() {
        let item = &items[payload_index + 1];
        let expected = expected_frame(&item["payload"]);
        let actual = frame_for_node(completed, text.node_id().get());
        assert_frame(
            actual,
            expected,
            &format!("scene {index} payload {payload_index}"),
        );
        let expected_text = PAYLOAD_TEXT[payload_index];
        assert_eq!(
            text.glyphs().len(),
            expected_text.bytes().filter(u8::is_ascii_uppercase).count(),
            "scene {index} payload {payload_index} glyph count"
        );
        let line_count = item["lineCount"].as_u64().unwrap() as usize;
        let baselines = text
            .glyphs()
            .iter()
            .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            baselines.len(),
            line_count,
            "scene {index} payload {payload_index} line count"
        );
        overflow |= actual[1] + actual[3] > 220.0;
    }
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    assert_track_trace(completed, scene, &format!("scene {index}"));
    overflow
}

#[test]
fn parsed_root_html_repeated_text_auto_placement_matches_all_fit_content_positions() {
    let mut lines = CHROME_MATRIX.lines();
    let metadata: Value = serde_json::from_str(lines.next().expect("matrix metadata"))
        .expect("matrix metadata should be valid JSON");
    assert_eq!(metadata["record"].as_str(), Some("metadata"));
    assert_eq!(metadata["caseCount"].as_u64(), Some(1440));
    assert_eq!(metadata["uniqueTailOrders"].as_u64(), Some(30));
    assert!(
        metadata["userAgent"]
            .as_str()
            .unwrap()
            .contains("HeadlessChrome/157.0.0.0"),
        "matrix must remain pinned to the recorded Chrome 157 observation"
    );
    assert_eq!(metadata["platform"].as_str(), Some("MacIntel"));
    assert_eq!(metadata["dpr"].as_f64(), Some(1.0));
    assert_eq!(
        metadata["fitContentPositions"],
        serde_json::json!([1, 2, 3, 4, 5, 6])
    );
    assert_eq!(metadata["payloads"], serde_json::json!(PAYLOAD_TEXT));
    assert_eq!(
        metadata["payloadLineCounts"],
        serde_json::json!([2, 2, 2, 3, 2, 2, 3, 1])
    );
    let scenes = lines
        .map(|line| serde_json::from_str::<Value>(line).expect("matrix scene should be valid JSON"))
        .collect::<Vec<_>>();
    assert_eq!(scenes.len(), 1440);

    let mut position_counts = [0usize; 6];
    let mut scene_keys = std::collections::BTreeSet::new();
    let mut overflow_count = 0usize;
    for (index, scene) in scenes.iter().enumerate() {
        assert_eq!(scene["record"].as_str(), Some("scene"));
        let position = scene["fitPosition"].as_u64().unwrap() as usize;
        assert!((1..=6).contains(&position));
        position_counts[position - 1] += 1;
        let tail = scene["tail"]
            .as_array()
            .unwrap()
            .iter()
            .map(|track| track.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(tail.len(), 5);
        let mut tail_counts = std::collections::BTreeMap::new();
        for track in &tail {
            *tail_counts.entry(*track).or_insert(0usize) += 1;
        }
        assert_eq!(
            tail_counts,
            std::collections::BTreeMap::from(
                [("auto", 1), ("max-content", 2), ("min-content", 2),]
            ),
            "scene {index}: repeated-tail multiplicity"
        );
        let authored = scene["authoredAutoRows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|track| track.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(authored.len(), 6);
        assert_eq!(
            authored[position - 1],
            format!("fit-content({})", scene["cap"].as_str().unwrap())
        );
        assert_eq!(
            authored
                .iter()
                .enumerate()
                .filter(|(member, _)| *member != position - 1)
                .map(|(_, track)| *track)
                .collect::<Vec<_>>(),
            tail,
            "scene {index}: authored list must insert fit-content without replacing tail members"
        );
        assert_eq!(
            scene["computedAutoRows"].as_str(),
            Some(authored.join(" ").as_str()),
            "scene {index}: computed order"
        );
        let cap = scene["cap"].as_str().unwrap();
        assert!(["20px", "20%", "0%", "120%"].contains(&cap));
        let align = scene["align"].as_str().unwrap();
        assert!(["start", "stretch"].contains(&align));
        assert!(scene_keys.insert(format!("{position}|{}|{cap}|{align}", tail.join(" "))));
        assert_eq!(scene["items"].as_array().unwrap().len(), 9);
        overflow_count += usize::from(assert_position_matrix_scene(index, scene));
    }
    assert_eq!(position_counts, [240; 6]);
    assert_eq!(
        scene_keys.len(),
        1440,
        "matrix cross-product keys must be unique"
    );
    assert_eq!(overflow_count, 54);
}
