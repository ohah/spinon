use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
};

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-text-precomparison.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-precomparison-2026-11-08/chrome157-expected.json"
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
const TEXT_LINE_COUNTS: [usize; 8] = [2, 2, 2, 1, 2, 1, 3, 1];
const TEXT_GLYPH_COUNTS: [usize; 8] = [6, 9, 8, 2, 6, 4, 8, 2];

fn expected_frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("expected frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| {
        values[index]
            .as_f64()
            .expect("frame component should be numeric") as f32
    })
}

fn assert_frame_001(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.01,
            "{label} component {component}: actual={actual:?}, expected={expected:?} ± 0.01 CSS px"
        );
    }
}

fn css_source(scene: &Value) -> String {
    let cap = scene["cap"]
        .as_str()
        .expect("scene cap should be a CSS string");
    let tail = scene["tail"]
        .as_str()
        .expect("scene tail should be a CSS string");
    let align = scene["align"]
        .as_str()
        .expect("scene alignment should be a CSS string");
    let grid_height = scene["gridHeight"]
        .as_u64()
        .expect("scene Grid height should be an integer");
    let source = replace_once(SOURCE, "--cap: 20px;", &format!("--cap: {cap};"));
    let source = replace_once(
        &source,
        "--tail: min-content min-content max-content max-content auto;",
        &format!("--tail: {tail};"),
    );
    let source = replace_once(
        &source,
        "--grid-height: 220px;",
        &format!("--grid-height: {grid_height}px;"),
    );
    replace_once(&source, "--align: start;", &format!("--align: {align};"))
}

fn computed_cap(css: &str) -> ComputedCssGridTrackLengthPercentage {
    if let Some(percent) = css.strip_suffix('%') {
        ComputedCssGridTrackLengthPercentage::Percentage(
            percent.parse::<f32>().expect("percentage cap should parse") / 100.0,
        )
    } else {
        ComputedCssGridTrackLengthPercentage::LengthPx(
            css.strip_suffix("px")
                .expect("length cap should use px")
                .parse::<f32>()
                .expect("px cap should parse"),
        )
    }
}

fn computed_tail(tail: &str) -> Vec<ComputedCssGridTrackSize> {
    tail.split_whitespace()
        .map(|track| match track {
            "min-content" => ComputedCssGridTrackSize::MinContent,
            "max-content" => ComputedCssGridTrackSize::MaxContent,
            "auto" => ComputedCssGridTrackSize::Auto,
            _ => panic!("unsupported expected repeated track {track:?}"),
        })
        .collect()
}

fn runtime_frame(frame: &crate::session::RuntimeLayoutFrame) -> [f32; 4] {
    [frame.x, frame.y, frame.width, frame.height]
}

#[test]
fn parsed_root_html_repeated_intrinsic_text_matches_all_270_chrome_scenes() {
    let expected: Value = serde_json::from_str(EXPECTED).expect("Chrome expected JSON is valid");
    let scenes = expected["scenes"]
        .as_array()
        .expect("Chrome expected scenes should be an array");
    assert_eq!(scenes.len(), 270);
    assert_eq!(expected["uniqueTailOrders"].as_u64(), Some(30));
    assert_eq!(
        expected["textDescriptors"]
            .as_array()
            .expect("text descriptors should be an array")
            .iter()
            .map(|descriptor| descriptor["lineCount"].as_u64().unwrap() as usize)
            .collect::<Vec<_>>(),
        TEXT_LINE_COUNTS
    );

    for (scene_index, expected_scene) in scenes.iter().enumerate() {
        let source = css_source(expected_scene);
        let (session, layout) = run(&source);
        let cascade_snapshot = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index}: layout={:?}; cascade state={:?}; cascade error={:?}",
            layout.error,
            cascade_snapshot.state,
            cascade_snapshot.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported scene should publish complete layout");
        let render_scene = completed
            .render_snapshot
            .as_ref()
            .expect("supported scene should publish render output");
        let cascade = cascade_snapshot
            .completed
            .expect("supported scene should publish computed styles");
        assert_eq!(completed.key, cascade.key);
        let grid_style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("fixture has one Grid");
        let tracks = expected_scene["tail"].as_str().unwrap();
        let mut expected_tracks = vec![ComputedCssGridTrackSize::FitContent(computed_cap(
            expected_scene["cap"].as_str().unwrap(),
        ))];
        expected_tracks.extend(computed_tail(tracks));
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "scene {scene_index}: typed cap and repeated track order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(String::as_str),
            Some(
                format!(
                    "fit-content({}) {tracks}",
                    expected_scene["cap"].as_str().unwrap()
                )
                .as_str()
            ),
            "scene {scene_index}: computed track serialization"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            expected_scene["align"].as_str(),
            "scene {scene_index}: computed alignment"
        );

        let expected_grid_frame = expected_frame(&expected_scene["gridFrame"]);
        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame should be present");
        assert_frame_001(
            runtime_frame(grid_frame),
            expected_grid_frame,
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["itemFrames"]
            .as_array()
            .expect("expected item frames should be an array");
        assert_eq!(expected_items.len(), ITEM_COLORS.len());
        for (item_index, (expected_item, color)) in
            expected_items.iter().zip(ITEM_COLORS).enumerate()
        {
            let expected = expected_frame(expected_item);
            if expected[3] > 0.0 {
                assert_frame_001(
                    frame_for_paint(render_scene, color),
                    expected,
                    &format!("scene {scene_index} item {item_index}"),
                );
            } else {
                assert!(
                    completed
                        .frames
                        .iter()
                        .any(|frame| frame_eq(runtime_frame(frame), expected)),
                    "scene {scene_index} zero-height item {item_index} frame missing: {expected:?}"
                );
            }
        }

        let expected_rows = expected_scene["gridRows"]
            .as_array()
            .expect("expected Grid rows should be an array");
        assert_eq!(expected_rows.len(), expected_items.len());
        for (row, expected_row) in expected_rows.iter().enumerate() {
            let row_size = expected_row.as_f64().unwrap() as f32;
            let item_height = expected_frame(&expected_items[row])[3];
            assert_frame_001(
                [item_height, 0.0, 0.0, 0.0],
                [row_size, 0.0, 0.0, 0.0],
                &format!("scene {scene_index} row {row}"),
            );
        }

        let expected_payloads = expected_scene["payloadFrames"]
            .as_array()
            .expect("expected payload frames should be an array");
        assert_eq!(expected_payloads.len(), TEXT_LINE_COUNTS.len());
        let mut texts = render_scene.texts().iter().collect::<Vec<_>>();
        texts.sort_unstable_by_key(|text| text.node_id().get());
        assert_eq!(texts.len(), TEXT_LINE_COUNTS.len());
        for (text_index, text) in texts.into_iter().enumerate() {
            let expected = expected_frame(&expected_payloads[text_index]);
            let payload_frame = completed
                .frames
                .iter()
                .find(|frame| frame.node_id == text.node_id().get())
                .unwrap_or_else(|| {
                    panic!("scene {scene_index} payload {text_index} frame missing")
                });
            assert_frame_001(
                runtime_frame(payload_frame),
                expected,
                &format!("scene {scene_index} payload {text_index}"),
            );
            assert_eq!(
                text.glyphs().len(),
                TEXT_GLYPH_COUNTS[text_index],
                "scene {scene_index} payload {text_index} glyph count"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                TEXT_LINE_COUNTS[text_index],
                "scene {scene_index} payload {text_index} line count"
            );
        }
    }
}

fn frame_eq(actual: [f32; 4], expected: [f32; 4]) -> bool {
    actual
        .into_iter()
        .zip(expected)
        .all(|(actual, expected)| actual.is_finite() && (actual - expected).abs() <= 0.01)
}

#[test]
fn parsed_root_html_repeated_intrinsic_text_rejects_mutations_without_partial_output() {
    let cases = [
        (
            "wrong text",
            replace_once(SOURCE, "A A A A A A</div>", "A A A A A</div>"),
        ),
        (
            "wrong font family",
            replace_once(
                SOURCE,
                "font-family: monospace;",
                "font-family: sans-serif;",
            ),
        ),
        (
            "wrong font size",
            replace_once(SOURCE, "font-size: 10px;", "font-size: 11px;"),
        ),
        (
            "wrong line height",
            replace_once(SOURCE, "line-height: 10px;", "line-height: 11px;"),
        ),
        (
            "nowrap",
            replace_once(SOURCE, "white-space: normal;", "white-space: nowrap;"),
        ),
        (
            "nested inline",
            replace_once(
                SOURCE,
                "<div id=\"text-1\" class=\"payload\">A A A A A A</div>",
                "<div id=\"text-1\" class=\"payload\"><span>A A A A A A</span></div>",
            ),
        ),
        (
            "fixed text height",
            replace_once(
                SOURCE,
                "height: auto;\n        margin: 0;\n        padding: 0;\n        font-family: monospace;",
                "height: 20px;\n        margin: 0;\n        padding: 0;\n        font-family: monospace;",
            ),
        ),
        (
            "nonzero item minimum height",
            replace_once(SOURCE, "min-height: 0;", "min-height: 1px;"),
        ),
    ];

    for (name, source) in cases {
        assert_ne!(source, SOURCE, "{name}: mutation should match the fixture");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name}: out-of-profile mutation must fail; cascade={:?}",
            cascade.error
        );
        let failure = layout
            .error
            .as_ref()
            .expect("mutation should report failure");
        assert!(
            failure.property.as_deref().is_some_and(|property| {
                matches!(property, "height" | "grid-auto-rows" | "font-size")
            }),
            "{name}: failure should identify the bounded property: {failure:?}; cascade={:?}",
            cascade.error
        );
        assert!(
            failure.node_id.is_some(),
            "{name}: failure should identify the affected node"
        );
        assert!(
            layout.completed.is_none(),
            "{name}: no partial output may publish"
        );
    }
}

#[test]
fn parsed_root_html_repeated_intrinsic_text_recovers_same_session_after_rejection() {
    let source = replace_once(
        SOURCE,
        "    </style>",
        "      @media (prefers-color-scheme: dark) { #text-3 { font-weight: 500; } }\n    </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "initial text profile should be supported: layout={:?}; cascade={:?}",
        initial.error,
        session.ua_cascade_snapshot().error
    );
    assert!(initial.completed.is_some());

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark scheme should apply the unsupported font weight");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());
    assert!(
        rejected
            .error
            .as_ref()
            .is_some_and(|failure| failure.node_id.is_some() && failure.property.is_some()),
        "rejection should identify a node and property: {:?}",
        rejected.error
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light scheme should restore supported text styles");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should succeed: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
