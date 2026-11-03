use super::*;
use crate::session::RuntimeLayoutState;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-explicit-placement-runtime.html"
);

#[test]
fn c11_1_root_html_numeric_grid_placement_matches_expected_runtime_frames() {
    let (session, report) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("explicit-placement Root HTML should initialize");
    assert!(report.contains("session=ready"), "{report}");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(980.0, 735.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");

    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        cascade.state,
        RuntimeUaCascadeState::Ready,
        "cascade error={:?}",
        cascade.error
    );
    assert_eq!(
        layout.state,
        RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={cascade:?}",
        layout.error
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("accepted explicit scenes should publish one render snapshot");

    for (color, expected) in [
        ([229, 72, 77], [40.0, 0.0, 100.0, 30.0]),
        ([58, 143, 91], [0.0, 20.0, 40.0, 20.0]),
        ([53, 120, 199], [0.0, 0.0, 40.0, 20.0]),
        ([249, 115, 22], [40.0, 20.0, 80.0, 20.0]),
        ([143, 63, 113], [40.0, 60.0, 80.0, 20.0]),
        ([91, 111, 143], [0.0, 40.0, 40.0, 20.0]),
        ([190, 75, 219], [40.0, 80.0, 80.0, 20.0]),
        ([47, 158, 68], [0.0, 100.0, 40.0, 20.0]),
        ([132, 94, 247], [40.0, 80.0, 80.0, 40.0]),
        ([201, 42, 42], [0.0, 120.0, 40.0, 20.0]),
        ([8, 127, 91], [0.0, 120.0, 40.0, 20.0]),
        ([240, 140, 0], [40.0, 160.0, 80.0, 20.0]),
        ([103, 65, 217], [24.0, 210.0, 16.0, 10.0]),
    ] {
        assert_frame_for_color(&scene, color, expected);
    }

    let first_overlap = index_for_color(&scene, [201, 42, 42]);
    let last_overlap = index_for_color(&scene, [8, 127, 91]);
    assert!(
        first_overlap < last_overlap,
        "overlap paint order must follow source order: {first_overlap} before {last_overlap}"
    );
    assert_eq!(scene.viewport_css_px().width(), 980.0);
    assert_eq!(scene.viewport_css_px().height(), 735.0);
}

#[test]
fn c11_1_root_html_numeric_grid_placement_rejects_unsupported_scenes_atomically() {
    fn assert_rejected(source: &str, label: &str) {
        let (session, report) = RuntimeSession::new_runtime_gpu_with_root_html(source)
            .unwrap_or_else(|error| panic!("{label}: HTML import failed unexpectedly: {error}"));
        assert!(report.contains("session=ready"), "{label}: {report}");
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(980.0, 735.0, 1.0, media)
            .expect("Root HTML metadata should configure the CSS environment");
        let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
        assert_eq!(
            layout.state,
            RuntimeLayoutState::Failed,
            "{label}: unsupported input must fail the full scene"
        );
        assert!(
            layout.error.is_some(),
            "{label}: rejection needs a diagnostic"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: a partial or stale render snapshot must not be published"
        );
    }

    let prefix = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"></head><body style="margin:0;padding:0"><div style="display:grid;width:120px;height:50px;grid-template-columns:40px 80px;grid-template-rows:20px 30px;gap:0;margin:0;padding:0;"#;
    let suffix = "</div></body></html>";
    let grid_source = |items: &str, extra: &str| format!("{prefix}{extra}\">{items}{suffix}");

    assert_rejected(
        &grid_source(
            "<div style=\"grid-row:1 / 2;grid-column:2 / 3\"></div><div></div>",
            "",
        ),
        "mixed explicit and auto items",
    );
    assert_rejected(
        &grid_source(
            "<div style=\"grid-column:1 / 2\"></div><div style=\"grid-row:2 / 3;grid-column:2 / 3\"></div>",
            "",
        ),
        "partial auto axis",
    );
    assert_rejected(
        &grid_source(
            "<div style=\"grid-row:3 / 4;grid-column:1 / 2\"></div>",
            "grid-auto-rows:8px;",
        ),
        "implicit row outside explicit tracks",
    );
    assert_rejected(
        &grid_source(
            "<div style=\"grid-row:1 / 2;grid-column:1 / 2\"></div>TEXT<div style=\"grid-row:2 / 3;grid-column:2 / 3\"></div>",
            "",
        ),
        "anonymous direct text auto item",
    );
    assert_rejected(
        &grid_source(
            "<div style=\"display:grid;grid-row:1 / 2;grid-column:1 / 2\"></div>",
            "",
        ),
        "nested Grid item",
    );
    assert_rejected(
        &grid_source(
            "<img style=\"display:block;grid-row:1 / 2;grid-column:1 / 2\" />",
            "",
        ),
        "replaced image item",
    );
    assert_rejected(
        &grid_source(
            "<div style=\"grid-row:1 / 2;grid-column:alpha / beta\"></div>",
            "",
        ),
        "named grid lines",
    );
    assert_rejected(
        &grid_source("<div style=\"grid-row:0 / 1;grid-column:1 / 2\"></div>", ""),
        "zero grid line",
    );
}

#[test]
fn c11_1_root_html_numeric_grid_placement_recovers_in_the_same_session() {
    let source = r#"<!doctype html>
<html><head>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    #item { display:block; width:40px; height:30px; grid-row:2 / 3; grid-column:1 / 2; background-color:#e5484d; }
    @media (prefers-color-scheme: dark) { #item { grid-column:alpha / beta; } }
  </style>
</head>
<body style="margin:0;padding:0">
  <div style="display:grid;width:120px;height:50px;grid-template-columns:40px 80px;grid-template-rows:20px 30px;gap:0;margin:0;padding:0"><div id="item"></div></div>
</body></html>"#;
    let (session, report) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("media-query placement Root HTML should initialize");
    assert!(report.contains("session=ready"), "{report}");

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(980.0, 735.0, 1.0, media)
        .expect("light Root HTML metadata should configure the CSS environment");
    let initial = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(initial.state, RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());

    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(980.0, 735.0, 1.0, media)
        .expect("dark Root HTML metadata should reconfigure the CSS environment");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, RuntimeLayoutState::Failed);
    assert!(rejected.error.is_some());
    assert!(rejected.completed.is_none());

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(980.0, 735.0, 1.0, media)
        .expect("light Root HTML metadata should recover the CSS environment");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={:?}",
        recovered.error,
        session.ua_cascade_snapshot()
    );
    let scene = recovered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("supported media-query placement should publish a render snapshot");
    assert_frame_for_color(&scene, [229, 72, 77], [0.0, 20.0, 40.0, 30.0]);
}

fn assert_frame_for_color(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected_color: [u8; 3],
    expected_frame: [f32; 4],
) {
    let color =
        spinon_render::OpaqueCssSrgb::new(expected_color[0], expected_color[1], expected_color[2]);
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(actual) if actual == color
            )
        })
        .unwrap_or_else(|| panic!("expected Grid item color {expected_color:?}"));
    let frame = render_box.frame_css_px();
    let actual = [frame.x(), frame.y(), frame.width(), frame.height()];
    for (actual, expected) in actual.into_iter().zip(expected_frame) {
        assert!(
            (actual - expected).abs() <= 0.5,
            "color={expected_color:?}: expected {expected}px ±0.5, got {actual}px"
        );
    }
}

fn index_for_color(scene: &spinon_render::RuntimeRenderSnapshot, expected_color: [u8; 3]) -> usize {
    let color =
        spinon_render::OpaqueCssSrgb::new(expected_color[0], expected_color[1], expected_color[2]);
    scene
        .boxes()
        .iter()
        .position(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(actual) if actual == color
            )
        })
        .unwrap_or_else(|| panic!("expected Grid item color {expected_color:?}"))
}
