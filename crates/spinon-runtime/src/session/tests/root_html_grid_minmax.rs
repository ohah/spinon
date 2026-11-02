use super::RuntimeSession;
use std::time::Duration;

const MINIMUM_OVERFLOW_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-minmax-runtime.html");

fn source(width: &str, height: &str, columns: &str, rows: &str, gap: &str, items: &str) -> String {
    r#"<!doctype html>
<html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html, body { margin: 0; padding: 0; }
main { box-sizing: border-box; display: grid; width: @WIDTH@; height: @HEIGHT@;
  grid-template-columns: @COLUMNS@; grid-template-rows: @ROWS@; gap: @GAP@;
  margin: 0; padding: 0; background-color: #101820; }
.a { width: 8px; height: 8px; background-color: #e5484d; }
.b { width: 8px; height: 8px; background-color: #3b82f6; }
.c { width: 8px; height: 8px; background-color: #16a34a; }
</style></head><body><main>@ITEMS@</main></body></html>"#
        .replace("@WIDTH@", width)
        .replace("@HEIGHT@", height)
        .replace("@COLUMNS@", columns)
        .replace("@ROWS@", rows)
        .replace("@GAP@", gap)
        .replace("@ITEMS@", items)
}

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

fn frame_for_paint(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected: spinon_render::OpaqueCssSrgb,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color) if color == expected
            )
        })
        .unwrap_or_else(|| panic!("expected painted Root HTML box {expected:?}"));
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], scene: &str) {
    for (axis, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{scene} frame component {axis}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

fn assert_second_item_position(
    html: &str,
    color: spinon_render::OpaqueCssSrgb,
    expected_x: f32,
    expected_y: f32,
    scene_name: &str,
) {
    let (session, layout) = run(html);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{scene_name}: error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    let completed = layout
        .completed
        .expect("bounded minmax Grid should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish a scene");
    assert_frame(
        frame_for_paint(scene, color),
        [expected_x, expected_y, 8.0, 8.0],
        scene_name,
    );
    let cascade_completed = cascade
        .completed
        .expect("computed track values should remain observable");
    assert_eq!(completed.key, cascade_completed.key);
}

#[test]
fn c11_1_root_html_default_runtime_keeps_minmax_minimum_overflow_visible() {
    use spinon_render::OpaqueCssSrgb;

    let (session, layout) = run(MINIMUM_OVERFLOW_SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    let completed = layout
        .completed
        .expect("bounded minmax grid should complete through the default Root HTML route");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish its paint scene");

    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 30.0, 20.0],
        "minimum-overflow-root",
    );
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(229, 72, 77)),
        [0.0, 0.0, 8.0, 8.0],
        "minimum-overflow-first-item",
    );
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(59, 130, 246)),
        [40.0, 0.0, 8.0, 8.0],
        "minimum-overflow-visible-marker",
    );
}

#[test]
fn c11_1_root_html_runtime_resolves_percentage_fraction_and_repeat_minmax_tracks() {
    use spinon_render::OpaqueCssSrgb;

    let items = r#"<div class="a"></div><div class="b"></div>"#;
    for (name, html, expected_x) in [
        (
            "fixed-bounds-grow-to-maximum",
            source(
                "160px",
                "20px",
                "minmax(20px, 80px) 10px",
                "20px",
                "0px",
                items,
            ),
            80.0,
        ),
        (
            "fixed-bounds-stop-at-available-space",
            source(
                "60px",
                "20px",
                "minmax(20px, 80px) 10px",
                "20px",
                "0px",
                items,
            ),
            50.0,
        ),
        (
            "percentage-fixed-bounds",
            source(
                "200px",
                "20px",
                "minmax(40%, 40%) 10px",
                "20px",
                "0px",
                items,
            ),
            80.0,
        ),
        (
            "percentage-bounds-and-gap",
            source(
                "200px",
                "20px",
                "minmax(20%, 40%) 10px",
                "20px",
                "5%",
                items,
            ),
            90.0,
        ),
        (
            "percentage-max-below-percentage-min",
            source(
                "200px",
                "20px",
                "minmax(40%, 20%) 10px",
                "20px",
                "0px",
                items,
            ),
            80.0,
        ),
        (
            "percentage-max-below-pixel-min",
            source(
                "200px",
                "20px",
                "minmax(60px, 20%) 10px",
                "20px",
                "0px",
                items,
            ),
            60.0,
        ),
        (
            "percentage-min-flex-max",
            source(
                "200px",
                "20px",
                "minmax(50%, 1fr) 1fr",
                "20px",
                "0px",
                items,
            ),
            100.0,
        ),
        (
            "fraction-max-with-gap",
            source(
                "130px",
                "20px",
                "minmax(20px, 1fr) 1fr",
                "20px",
                "10px",
                items,
            ),
            70.0,
        ),
        (
            "zero-fr-max-keeps-minimum",
            source(
                "130px",
                "20px",
                "minmax(20px, 0fr) 1fr",
                "20px",
                "10px",
                items,
            ),
            30.0,
        ),
        (
            "max-under-min-floor",
            source(
                "30px",
                "20px",
                "minmax(40px, 20px) 10px",
                "20px",
                "0px",
                items,
            ),
            40.0,
        ),
        (
            "fixed-repeat-bounds",
            source(
                "100px",
                "20px",
                "repeat(2, minmax(20px, 40px))",
                "20px",
                "5px",
                items,
            ),
            45.0,
        ),
        (
            "fixed-repeat-percentage-bounds",
            source(
                "200px",
                "20px",
                "repeat(2, minmax(20%, 40%))",
                "20px",
                "5px",
                items,
            ),
            85.0,
        ),
        (
            "zero-percentage-basis",
            source("0px", "20px", "minmax(50%, 80%) 10px", "20px", "0px", items),
            0.0,
        ),
    ] {
        assert_second_item_position(
            &html,
            OpaqueCssSrgb::new(59, 130, 246),
            expected_x,
            0.0,
            name,
        );
    }
}

#[test]
fn c11_1_root_html_runtime_resolves_percentage_minmax_rows_on_height_axis() {
    use spinon_render::OpaqueCssSrgb;

    let html = source(
        "40px",
        "100px",
        "20px 20px",
        "minmax(20%, 40%) 10px",
        "0px",
        r#"<div class="a"></div><div class="b"></div><div class="c"></div>"#,
    );
    assert_second_item_position(
        &html,
        OpaqueCssSrgb::new(22, 163, 74),
        0.0,
        40.0,
        "row-percentage-bounds",
    );
}

#[test]
fn c11_1_root_html_runtime_grows_fixed_minmax_rows_to_their_maximum() {
    use spinon_render::OpaqueCssSrgb;

    let html = source(
        "40px",
        "100px",
        "20px 20px",
        "minmax(20px, 80px) 10px",
        "0px",
        r#"<div class="a"></div><div class="b"></div><div class="c"></div>"#,
    );
    assert_second_item_position(
        &html,
        OpaqueCssSrgb::new(22, 163, 74),
        0.0,
        80.0,
        "fixed-row-bounds-grow-to-maximum",
    );
}

#[test]
fn c11_1_root_html_minmax_keeps_default_auto_implicit_rows_out_of_scope() {
    let html = source(
        "60px",
        "20px",
        "minmax(20px, 40px) 20px",
        "20px",
        "0px",
        r#"<div class="a"></div><div class="b"></div><div class="c"></div>"#,
    );
    let (_, layout) = run(&html);

    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "minmax must not silently enable intrinsic auto-sized implicit rows: {layout:?}"
    );
    assert_eq!(
        layout
            .error
            .as_ref()
            .and_then(|error| error.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(layout.completed.is_none());
}

#[test]
fn c11_1_root_html_invalid_minmax_declarations_fall_back_in_the_css_cascade() {
    use spinon_render::OpaqueCssSrgb;

    for (name, invalid_declaration) in [
        ("negative-length-minimum", "minmax(-1px, 80px) 10px"),
        ("negative-fraction-maximum", "minmax(20px, -1fr) 10px"),
        ("fraction-minimum", "minmax(1fr, 80px) 10px"),
    ] {
        let html = source(
            "100px",
            "20px",
            &format!("40px 10px; grid-template-columns: {invalid_declaration}"),
            "20px",
            "0px",
            r#"<div class="a"></div><div class="b"></div>"#,
        );
        assert_second_item_position(&html, OpaqueCssSrgb::new(59, 130, 246), 40.0, 0.0, name);
    }
}
