use super::*;

const HTML_TEMPLATE: &str = r#"<!doctype html>
<html lang="en"><head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    html, body { margin: 0; padding: 0; }
    main {
      box-sizing: border-box; display: grid;
      width: @WIDTH@; height: @HEIGHT@;
      grid-template-columns: @COLUMNS@; grid-template-rows: @ROWS@;
      gap: @GAP@; margin: 0; padding: 0;
      background-color: #101820;
      @EXTRA@
    }
    .a { background-color: #e5484d; }
    .b { background-color: #3b82f6; }
    .c { background-color: #16a34a; }
    .d { background-color: #f59e0b; }
    .e { background-color: #a855f7; }
    .f { background-color: #eab308; }
    .g { background-color: #14b8a6; }
    .h { background-color: #ec4899; }
    .i { background-color: #94a3b8; }
    .semantic-blue { background-color: #3b82f6; }
    .semantic-red { background-color: #e5484d; }
  </style>
</head><body>
  <main>@ITEMS@</main>
</body></html>"#;

fn source(
    width: &str,
    height: &str,
    columns: &str,
    rows: &str,
    gap: &str,
    extra: &str,
    items: &str,
) -> String {
    HTML_TEMPLATE
        .replace("@WIDTH@", width)
        .replace("@HEIGHT@", height)
        .replace("@COLUMNS@", columns)
        .replace("@ROWS@", rows)
        .replace("@GAP@", gap)
        .replace("@EXTRA@", extra)
        .replace("@ITEMS@", items)
}

fn fixed_items(items: &[(&str, &str, &str)]) -> String {
    items
        .iter()
        .map(|(class, width, height)| {
            format!(r#"<div class="{class}" style="width:{width}px;height:{height}px"></div>"#)
        })
        .collect()
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
        .unwrap_or_else(|| {
            panic!(
                "semantic Root HTML Grid box should keep background paint {expected:?}; observed {:?}",
                scene
                    .boxes()
                    .iter()
                    .map(|render_box| {
                        let frame = render_box.frame_css_px();
                        (
                            render_box.paint(),
                            [frame.x(), frame.y(), frame.width(), frame.height()],
                        )
                    })
                    .collect::<Vec<_>>()
            )
        });
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn assert_frame(actual_frame: [f32; 4], expected_frame: [f32; 4], scene: &str) {
    for (axis, (actual, expected)) in actual_frame.into_iter().zip(expected_frame).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{scene} frame component {axis}: actual frame={actual_frame:?}, expected frame={expected_frame:?} ± 0.02"
        );
    }
}

#[test]
fn c11_1_root_html_default_runtime_uses_the_shared_simulator_source() {
    use spinon_render::OpaqueCssSrgb;

    const SOURCE: &str =
        include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-fr-runtime.html");
    let (session, layout) = run(SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "error={:?}; cascade state={:?}",
        layout.error,
        session.ua_cascade_snapshot().state
    );
    let completed = layout
        .completed
        .expect("shared source Grid should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("shared source should publish the default Root HTML paint scene");
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 130.0, 40.0],
        "shared-source-root",
    );
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(229, 72, 77)),
        [0.0, 0.0, 30.0, 40.0],
        "shared-source-first-item",
    );
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(59, 130, 246)),
        [50.0, 0.0, 30.0, 40.0],
        "shared-source-second-item",
    );
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("shared source cascade should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|element| element.grid_layout.as_ref())
        .expect("shared source computed tracks should remain typed");
    assert_eq!(
        grid.columns,
        [
            spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
            spinon_style::ComputedCssGridTrackSize::Fraction(2.0),
        ]
    );
}

mod positive_matrix;
mod rejection_matrix;
