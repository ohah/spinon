use super::*;
use serde_json::Value;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-precomparison.html"
);
const ZERO_MIN_RUNTIME_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-zero-min-runtime.html"
);
const NATIVE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/html/c11-root-html-grid-auto-repeat-native.html");
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/normative-expected.json"
);
const CHROME_OBSERVATION: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/chromium-observation.json"
);
const ROOT: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(16, 24, 32);
const RED: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);
const BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(59, 130, 246);

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    session
        .set_root_html_environment(
            1280.0,
            1200.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn has_frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: spinon_render::OpaqueCssSrgb,
    expected: [f32; 4],
) -> bool {
    scene.boxes().iter().any(|render_box| {
        if !matches!(
            render_box.paint(),
            spinon_render::RuntimePaint::Opaque(actual) if actual == color
        ) {
            return false;
        }
        let frame = render_box.frame_css_px();
        [frame.x(), frame.y(), frame.width(), frame.height()]
            .into_iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() <= 0.02)
    })
}

fn one_column_source(grid_overrides: &str, children: &str) -> String {
    format!(
        r#"<!doctype html>
<html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html, body {{ margin: 0; padding: 0; }}
#grid {{ box-sizing: border-box; display: grid; width: 170px; height: 20px;
  gap: 10px; grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  grid-template-rows: 20px; margin: 0; padding: 0;
  border-top-width: 0; border-right-width: 0; border-bottom-width: 0; border-left-width: 0;
  background-color: #101820; {grid_overrides} }}
#grid > div {{ box-sizing: border-box; min-width: 0; min-height: 0; }}
.red {{ background-color: #e5484d; }} .blue {{ background-color: #3b82f6; }}
</style></head><body><section id="grid">{children}</section></body></html>"#
    )
}

fn red_frames(source: &str, expected_count: usize) -> Vec<[f32; 4]> {
    let (session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "zero-minimum Root HTML auto-repeat should be supported; error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("zero-minimum auto-repeat should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("zero-minimum auto-repeat should publish a scene");
    let mut frames = scene
        .boxes()
        .iter()
        .filter_map(|render_box| match render_box.paint() {
            spinon_render::RuntimePaint::Opaque(color) if color == RED => {
                let frame = render_box.frame_css_px();
                Some([frame.x(), frame.y(), frame.width(), frame.height()])
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    frames.sort_by(|left, right| {
        left[1]
            .total_cmp(&right[1])
            .then(left[0].total_cmp(&right[0]))
    });
    assert_eq!(frames.len(), expected_count, "red item frames: {frames:?}");
    frames
}

#[test]
fn root_html_auto_repeat_matches_chrome_track_count_and_sizing_scenes() {
    // The browser fixture uses a semantically equivalent `background` shorthand on body, but
    // Root HTML currently admits only `background-color`; keep the Grid author input unchanged.
    let runtime_source = SOURCE
        .replace(
            "body { background: #ffffff; }",
            "body { background-color: #ffffff; }",
        )
        .replace(
            "border: 0;",
            "border-top-width: 0; border-right-width: 0; border-bottom-width: 0; border-left-width: 0;",
        );
    let (session, layout) = run(&runtime_source);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={:?}; cascade_state={:?}; cascade_error={:?}; grid={:?}",
        layout.error,
        cascade.state,
        cascade.error,
        cascade
            .completed
            .as_ref()
            .and_then(|completed| completed.roots.first())
            .and_then(|root| root
                .styles
                .elements
                .iter()
                .find(|style| style.node_id.get() == 17))
            .and_then(|style| style.grid_layout.as_ref())
    );
    let completed = layout
        .completed
        .expect("auto-repeat fixture should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish the scene");

    let expected: Value = serde_json::from_str(EXPECTED).expect("normative expectations parse");
    let chrome: Value =
        serde_json::from_str(CHROME_OBSERVATION).expect("Chrome observation parses");
    let observations = chrome["results"]
        .as_array()
        .expect("Chrome results should be an array");
    let cases = expected["cases"]
        .as_array()
        .expect("normative cases should be an array");
    assert_eq!(cases.len(), 13);
    for case in cases {
        let case_id = case["id"].as_str().expect("case id should be text");
        let observed = observations
            .iter()
            .find(|result| result["id"] == case["id"])
            .unwrap_or_else(|| panic!("Chrome observation is missing {case_id}"));
        let root_frame = observed["frame"]
            .as_array()
            .expect("Chrome root frame should be an array");
        let root_origin = [
            root_frame[0].as_f64().unwrap() as f32,
            root_frame[1].as_f64().unwrap() as f32,
        ];
        let root_size = case["rootPx"]
            .as_array()
            .expect("expected root size should be an array");
        assert!(
            has_frame(
                scene,
                ROOT,
                [
                    root_origin[0],
                    root_origin[1],
                    root_size[0].as_f64().unwrap() as f32,
                    root_size[1].as_f64().unwrap() as f32,
                ]
            ),
            "{case_id}: Root HTML grid frame should match the Chrome observation"
        );

        let items = case["itemsRelativeToGrid"]
            .as_array()
            .expect("expected item frames should be an array");
        for (index, item) in items.iter().enumerate() {
            let Some(relative_frame) = item.as_array() else {
                // `null` is the display:none child; it has no layout/paint frame.
                continue;
            };
            let color = if index % 2 == 0 { RED } else { BLUE };
            let expected_frame = [
                root_origin[0] + relative_frame[0].as_f64().unwrap() as f32,
                root_origin[1] + relative_frame[1].as_f64().unwrap() as f32,
                relative_frame[2].as_f64().unwrap() as f32,
                relative_frame[3].as_f64().unwrap() as f32,
            ];
            assert!(
                has_frame(scene, color, expected_frame),
                "{case_id} item {index}: expected {expected_frame:?} for {color:?}"
            );
        }
    }

    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let completed_styles = cascade
        .completed
        .expect("typed Grid styles should complete");
    let grids = completed_styles.roots[0]
        .styles
        .elements
        .iter()
        .filter_map(|element| element.grid_layout.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(grids.len(), 13);
    assert_eq!(
        grids
            .iter()
            .filter(|grid| grid.columns_auto_repeat.is_some())
            .count(),
        8
    );
    assert_eq!(
        grids
            .iter()
            .filter(|grid| grid.rows_auto_repeat.is_some())
            .count(),
        5
    );
}

#[test]
fn root_html_one_track_repeat_pattern_expands_for_multiple_visible_items() {
    let source = one_column_source("", "<div class=\"red\"></div><div class=\"blue\"></div>");
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "one-track auto-repeat should use its expanded track capacity; error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("one-track auto-repeat should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("one-track auto-repeat should publish a scene");
    assert!(has_frame(scene, RED, [0.0, 0.0, 80.0, 20.0]));
    assert!(has_frame(scene, BLUE, [90.0, 0.0, 80.0, 20.0]));
}

#[test]
fn parsed_root_html_zero_minimum_auto_repeat_matches_independent_count_and_sizing() {
    let fill_gap_items = "<div class=\"red\"></div>".repeat(6);
    assert_eq!(
        red_frames(
            &one_column_source(
                "width: 11px; height: 2px; gap: 1px; grid-template-columns: repeat(auto-fill, minmax(0px, 1fr)); grid-template-rows: 2px;",
                &fill_gap_items,
            ),
            6,
        ),
        [
            [0.0, 0.0, 1.0, 2.0],
            [2.0, 0.0, 1.0, 2.0],
            [4.0, 0.0, 1.0, 2.0],
            [6.0, 0.0, 1.0, 2.0],
            [8.0, 0.0, 1.0, 2.0],
            [10.0, 0.0, 1.0, 2.0],
        ]
    );

    assert_eq!(
        red_frames(
            &one_column_source(
                "width: 11px; height: 2px; gap: 0; grid-template-columns: repeat(auto-fit, minmax(0px, 1fr)); grid-template-rows: 2px;",
                "<div class=\"red\"></div>",
            ),
            1,
        ),
        [[0.0, 0.0, 11.0, 2.0]]
    );

    let mixed_pattern_items = "<div class=\"red\"></div>".repeat(8);
    assert_eq!(
        red_frames(
            &one_column_source(
                "width: 19px; height: 2px; gap: 1px; grid-template-columns: repeat(auto-fill, minmax(0px, 1fr) minmax(2px, 1fr)); grid-template-rows: 2px;",
                &mixed_pattern_items,
            ),
            8,
        ),
        [
            [0.0, 0.0, 1.0, 2.0],
            [2.0, 0.0, 2.0, 2.0],
            [5.0, 0.0, 1.0, 2.0],
            [7.0, 0.0, 2.0, 2.0],
            [10.0, 0.0, 1.0, 2.0],
            [12.0, 0.0, 2.0, 2.0],
            [15.0, 0.0, 1.0, 2.0],
            [17.0, 0.0, 2.0, 2.0],
        ]
    );

    let row_items = "<div class=\"red\"></div>".repeat(4);
    assert_eq!(
        red_frames(
            &one_column_source(
                "width: 3px; height: 7px; row-gap: 1px; column-gap: 0; grid-template-columns: 3px; grid-template-rows: repeat(auto-fill, minmax(0px, 1fr));",
                &row_items,
            ),
            4,
        ),
        [
            [0.0, 0.0, 3.0, 1.0],
            [0.0, 2.0, 3.0, 1.0],
            [0.0, 4.0, 3.0, 1.0],
            [0.0, 6.0, 3.0, 1.0],
        ]
    );
}

#[test]
fn fixed_zero_minimum_root_html_fixture_runs_all_ten_cases_in_the_default_runtime() {
    assert_eq!(
        red_frames(ZERO_MIN_RUNTIME_SOURCE, 9),
        [
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 2.0, 11.0, 2.0],
            [0.0, 4.0, 1.0, 2.0],
            [0.0, 6.0, 1.125, 2.0],
            [0.0, 8.0, 1.0, 2.0],
            [0.0, 10.0, 1.0, 2.0],
            [0.0, 12.0, 19.0, 2.0],
            [0.0, 16.0, 3.0, 1.0],
            [0.0, 23.0, 3.0, 1.0],
        ]
    );
}

#[test]
fn native_bootstrap_root_html_auto_repeat_fixture_has_expected_frames() {
    let (session, layout) = run(NATIVE_SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "native Root HTML fixture should pass the default runtime route; error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("native Root HTML fixture should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("native Root HTML fixture should publish a scene");
    let mut actual = scene
        .boxes()
        .iter()
        .filter_map(|render_box| {
            let color = match render_box.paint() {
                spinon_render::RuntimePaint::Opaque(color) if color == RED => 0,
                spinon_render::RuntimePaint::Opaque(color) if color == BLUE => 1,
                _ => return None,
            };
            let frame = render_box.frame_css_px();
            Some((color, [frame.x(), frame.y(), frame.width(), frame.height()]))
        })
        .collect::<Vec<_>>();
    actual.sort_by(|(_, left), (_, right)| {
        left[1]
            .total_cmp(&right[1])
            .then_with(|| left[0].total_cmp(&right[0]))
    });
    let expected = [
        (0, [0.0, 0.0, 80.0, 20.0]),
        (1, [90.0, 0.0, 80.0, 20.0]),
        (0, [0.0, 20.0, 40.0, 20.0]),
        (1, [50.0, 20.0, 30.0, 20.0]),
        (0, [90.0, 20.0, 40.0, 20.0]),
        (1, [140.0, 20.0, 30.0, 20.0]),
        (0, [0.0, 40.0, 60.0, 80.0]),
        (1, [0.0, 130.0, 60.0, 80.0]),
    ];
    assert_eq!(
        actual.len(),
        expected.len(),
        "visible item frames: {actual:?}"
    );
    for ((actual_color, actual_frame), (expected_color, expected_frame)) in
        actual.iter().zip(expected)
    {
        assert_eq!(*actual_color, expected_color);
        for (index, (actual, expected)) in actual_frame.iter().zip(expected_frame).enumerate() {
            let axis = ["x", "y", "width", "height"][index];
            assert!(
                (*actual - expected).abs() <= 0.02,
                "{axis}: actual={actual} expected={expected}; frames={actual:?}"
            );
        }
    }
}

#[test]
fn unsupported_auto_repeat_inputs_fail_without_publishing_partial_scenes() {
    let candidates = [
        (
            "percentage gap",
            one_column_source("column-gap: 10%;", "<div class=\"red\"></div>"),
        ),
        (
            "indefinite repeated axis",
            one_column_source("width: auto;", "<div class=\"red\"></div>"),
        ),
        (
            "integer placement",
            one_column_source("", "<div class=\"red\" style=\"grid-column: 1\"></div>"),
        ),
        (
            "implicit row sizing",
            one_column_source("grid-auto-rows: 10px;", "<div class=\"red\"></div>"),
        ),
        (
            "item capacity overflow",
            one_column_source(
                "width: 40px; gap: 0; grid-template-columns: repeat(auto-fill, minmax(40px, 1fr));",
                "<div class=\"red\"></div><div class=\"blue\"></div>",
            ),
        ),
        (
            "zero-minimum repeat count over the track limit",
            one_column_source(
                "width: 10001px; gap: 0; grid-template-columns: repeat(auto-fill, minmax(0px, 1fr));",
                "",
            ),
        ),
        (
            "repeat neighbor",
            one_column_source(
                "grid-template-columns: 10px repeat(auto-fill, minmax(80px, 1fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "repeat on both axes",
            one_column_source(
                "grid-template-rows: repeat(auto-fill, minmax(10px, 1fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "authored line name",
            one_column_source(
                "grid-template-columns: [start] repeat(auto-fill, minmax(80px, 1fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "repeat line name",
            one_column_source(
                "grid-template-columns: repeat(auto-fill, [cell] minmax(80px, 1fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "three-track repeat pattern",
            one_column_source(
                "grid-template-columns: repeat(auto-fill, minmax(40px, 1fr) 10px minmax(40px, 1fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "unsupported fractional maximum",
            one_column_source(
                "grid-template-columns: repeat(auto-fill, minmax(80px, 2fr));",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "repeated-axis minimum size",
            one_column_source("min-width: 0;", "<div class=\"red\"></div>"),
        ),
        (
            "repeated-axis maximum size",
            one_column_source("max-width: 300px;", "<div class=\"red\"></div>"),
        ),
        (
            "nonzero border",
            one_column_source(
                "border-top-width: 1px; border-top-style: solid;",
                "<div class=\"red\"></div>",
            ),
        ),
        (
            "nonzero margin",
            one_column_source("margin-left: 1px;", "<div class=\"red\"></div>"),
        ),
        (
            "nonzero padding",
            one_column_source("padding-left: 1px;", "<div class=\"red\"></div>"),
        ),
        (
            "content alignment",
            one_column_source("align-content: center;", "<div class=\"red\"></div>"),
        ),
        (
            "justify alignment",
            one_column_source("justify-content: center;", "<div class=\"red\"></div>"),
        ),
        (
            "non-visible overflow",
            one_column_source("overflow: hidden;", "<div class=\"red\"></div>"),
        ),
        (
            "right-to-left direction",
            one_column_source("direction: rtl;", "<div class=\"red\"></div>"),
        ),
        (
            "dense auto-flow",
            one_column_source("grid-auto-flow: row dense;", "<div class=\"red\"></div>"),
        ),
        (
            "fixed implicit row track",
            one_column_source("grid-auto-rows: 20px;", "<div class=\"red\"></div>"),
        ),
        (
            "ordered item",
            one_column_source("", "<div class=\"red\" style=\"order: 1\"></div>"),
        ),
        (
            "positioned item",
            one_column_source("", "<div class=\"red\" style=\"position: absolute\"></div>"),
        ),
        (
            "item aspect ratio",
            one_column_source("", "<div class=\"red\" style=\"aspect-ratio: 1\"></div>"),
        ),
        (
            "item self-alignment",
            one_column_source("", "<div class=\"red\" style=\"align-self: center\"></div>"),
        ),
    ];

    for (label, source) in candidates {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label} should fail closed; error={:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot()
        );
        assert!(
            layout
                .completed
                .as_ref()
                .is_none_or(|completed| completed.render_snapshot.is_none()),
            "{label} must not publish a partial scene"
        );
    }
}
