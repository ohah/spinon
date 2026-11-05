use super::*;

use serde_json::Value;
use spinon_render::{OpaqueCssSrgb, RuntimePaint};

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-precomparison.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-precomparison-2026-11-06/expected.json"
);
const TOLERANCE: f32 = 0.5;

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn painted_frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: OpaqueCssSrgb,
    label: &str,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
        })
        .unwrap_or_else(|| panic!("{label}: expected painted box is missing"));
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_match_one_and_two_implicit_row_geometry() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("intrinsic auto-row expected values should be valid JSON");
    let expected_items = &expected["items"];
    let one_row_source = SOURCE.replace("        <div id=\"third\" class=\"item\"></div>\n", "");
    assert_ne!(
        one_row_source, SOURCE,
        "one-row mutation should match the fixture"
    );

    for (label, source, item_ids) in [
        (
            "one-implicit-row",
            one_row_source.as_str(),
            ["first", "second", ""],
        ),
        ("two-implicit-rows", SOURCE, ["first", "second", "third"]),
    ] {
        let (session, layout) = run(source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: layout error={:?}; cascade state={:?}; cascade error={:?}",
            layout.error,
            session.ua_cascade_snapshot().state,
            session.ua_cascade_snapshot().error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported implicit rows should publish a complete layout");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete for the same source revision")
                .key,
            "{label}: style and layout revisions should match"
        );
        let render_scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML route should publish paint boxes");
        let grid_frame = painted_frame(
            render_scene,
            OpaqueCssSrgb::new(57, 67, 79),
            &format!("{label} Grid"),
        );
        assert_frame_values(grid_frame, [0.0, 0.0, 40.0, 40.0], &format!("{label} Grid"));
        let mut render_positions = Vec::new();
        for id in item_ids.into_iter().filter(|id| !id.is_empty()) {
            let expected_item = &expected_items[id];
            let expected_frame = [
                expected_item["x"].as_f64().unwrap() as f32,
                expected_item["y"].as_f64().unwrap() as f32,
                expected_item["width"].as_f64().unwrap() as f32,
                expected_item["height"].as_f64().unwrap() as f32,
            ];
            let color = match id {
                "first" => OpaqueCssSrgb::new(242, 102, 94),
                "second" => OpaqueCssSrgb::new(91, 168, 245),
                "third" => OpaqueCssSrgb::new(85, 199, 145),
                _ => unreachable!("fixed Grid item id"),
            };
            let item_frame = painted_frame(render_scene, color, &format!("{label} {id}"));
            assert_frame_values(item_frame, expected_frame, &format!("{label} {id}"));
            let render_position = render_scene
                .boxes()
                .iter()
                .position(|render_box| {
                    matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
                });
            let render_position = render_position.unwrap_or_else(|| {
                panic!("{label}: {id} should remain visible in the paint scene")
            });
            render_positions.push(render_position);
        }
        assert!(
            render_positions.windows(2).all(|pair| pair[0] < pair[1]),
            "{label}: paint boxes should preserve DOM source order"
        );

        if label == "two-implicit-rows" {
            let third = painted_frame(
                render_scene,
                OpaqueCssSrgb::new(85, 199, 145),
                "two implicit rows third item",
            );
            assert_eq!(third[1], 32.0);
            assert_eq!(third[1] + third[3], 66.0);
            assert!(
                third[1] + third[3] > 40.0,
                "visible overflow should be preserved"
            );
        }
    }
}

fn assert_frame_values(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= TOLERANCE,
            "{label} component {component}: actual={actual}, expected={expected} ±{TOLERANCE} CSS px"
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_reject_unsupported_values_atomically() {
    for (label, source, expected_code) in [
        (
            "max-content",
            SOURCE.replace(
                "grid-auto-rows: min-content;",
                "grid-auto-rows: max-content;",
            ),
            "unsupported_computed_value",
        ),
        (
            "multi-track-list",
            SOURCE.replace(
                "grid-auto-rows: min-content;",
                "grid-auto-rows: min-content 12px;",
            ),
            "unsupported_computed_value",
        ),
        (
            "nonzero-row-gap",
            SOURCE.replace("gap: 0;", "row-gap: 1px; column-gap: 0;"),
            "unsupported_grid_auto_rows",
        ),
        (
            "nonzero-column-gap",
            SOURCE.replace("gap: 0;", "row-gap: 0; column-gap: 1px;"),
            "unsupported_grid_auto_rows",
        ),
        (
            "auto-sized-item",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: auto; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "item-min-size",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: 40px; min-height: 0; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "explicit-item-placement",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: 40px; grid-row-start: 1; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "intrinsic-explicit-track-combination",
            SOURCE.replace(
                "grid-template-columns: 40px;",
                "grid-template-columns: min-content;",
            ),
            "unsupported_computed_value",
        ),
        (
            "non-div-item",
            SOURCE.replace(
                "<div id=\"third\" class=\"item\"></div>",
                "<span id=\"third\" class=\"item\"></span>",
            ),
            "unsupported_computed_value",
        ),
        (
            "anonymous-text-item",
            SOURCE.replace("<div id=\"third\" class=\"item\"></div>", "text"),
            "unsupported_root_html_grid_text_item",
        ),
        (
            "text-descendant",
            SOURCE.replace(
                "<div id=\"third\" class=\"item\"></div>",
                "<div id=\"third\" class=\"item\">text</div>",
            ),
            "unsupported_computed_value",
        ),
        (
            "nested-grid-item",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { display: grid; grid-template-columns: 40px; grid-template-rows: 10px; box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "multiple-visible-grid-owners",
            SOURCE.replace(
                "    </main>",
                "      <div class=\"grid\"></div>\n    </main>",
            ),
            "unsupported_computed_value",
        ),
    ] {
        assert_ne!(source, SOURCE, "{label}: source mutation should match");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid input should finish cascade before the bounded adapter rejects it; error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("{label}: expected a contextual rejection"));
        assert_eq!(failure.code, expected_code, "{label}");
        if label != "anonymous-text-item" {
            assert_eq!(
                failure.property.as_deref(),
                Some("grid-auto-rows"),
                "{label}: rejection should identify the unsupported Grid property"
            );
        }
        assert!(
            layout.completed.is_none(),
            "{label}: rejection must not publish partial frames or paint"
        );
    }
}
