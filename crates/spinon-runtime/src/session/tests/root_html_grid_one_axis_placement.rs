use super::*;
use crate::session::RuntimeLayoutState;
use std::time::Duration;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-one-axis-placement-runtime.html"
);

#[test]
fn c11_1_root_html_one_axis_placement_matches_fixed_track_frames() {
    let (session, report) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("one-axis-placement Root HTML should initialize");
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
        .expect("supported one-axis scenes should publish a render snapshot");

    for (color, expected) in [
        ([229, 72, 77], [40.0, 0.0, 40.0, 20.0]),
        ([53, 120, 199], [40.0, 50.0, 40.0, 40.0]),
        ([249, 115, 22], [0.0, 90.0, 40.0, 20.0]),
        ([143, 63, 113], [40.0, 90.0, 80.0, 20.0]),
    ] {
        assert_frame_for_color(&scene, color, expected);
    }
}

#[test]
fn c11_1_root_html_mixed_cross_axis_runtime_placement_matches_fixed_track_frames() {
    let source = "<!doctype html><html><head><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body style=\"margin:0;padding:0\"><div style=\"display:grid;width:120px;height:50px;grid-template-columns:40px 80px;grid-template-rows:20px 30px;grid-auto-flow:row;gap:0;margin:0;padding:0\"><div style=\"width:40px;height:20px;grid-row:auto;grid-column:1;background-color:#3578c7\"></div><div style=\"width:40px;height:20px;grid-row:1;grid-column:auto;background-color:#f97316\"></div></div></body></html>";
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("row-flow mixed Root HTML should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(980.0, 735.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        layout.state,
        RuntimeLayoutState::Ready,
        "layout error={:?}; cascade error={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("supported row-flow mixed scene should publish a render snapshot");
    assert_frame_for_color(&scene, [53, 120, 199], [0.0, 20.0, 40.0, 20.0]);
    assert_frame_for_color(&scene, [249, 115, 22], [0.0, 0.0, 40.0, 20.0]);
}

#[test]
fn c11_1_root_html_one_axis_placement_rejects_unsupported_shapes_atomically() {
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

    let document = |grid_style: &str, items: &str| {
        format!(
            "<!doctype html><html><head><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body style=\"margin:0;padding:0\"><div style=\"{grid_style}\">{items}</div></body></html>"
        )
    };
    let base = "display:grid;width:120px;height:50px;grid-template-columns:40px 80px;grid-template-rows:20px 30px;gap:0;margin:0;padding:0";

    assert_rejected(
        &document(
            "display:grid;width:120px;height:40px;grid-template-columns:40px 40px 40px;grid-template-rows:20px 20px;grid-auto-rows:10px;gap:0;margin:0;padding:0",
            "<div style=\"grid-column:1 / 2\"></div><div style=\"grid-column:1 / 2\"></div><div style=\"grid-column:1 / 2\"></div>",
        ),
        "implicit row required",
    );
    assert_rejected(
        &document(
            "display:grid;width:120px;height:20px;grid-template-columns:40px 80px;grid-template-rows:20px;grid-auto-columns:10px;gap:0;margin:0;padding:0",
            "<div style=\"grid-row:1 / 2\"></div><div style=\"grid-row:1 / 2\"></div><div style=\"grid-row:1 / 2\"></div>",
        ),
        "implicit column required",
    );
    assert_rejected(
        &document(
            base,
            "<div style=\"display:grid;width:40px;height:20px;grid-template-columns:40px;grid-template-rows:20px;grid-column:1 / 2\"></div>",
        ),
        "nested Grid item",
    );
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
