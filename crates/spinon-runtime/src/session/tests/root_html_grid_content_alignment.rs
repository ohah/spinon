use super::*;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-content-alignment.html");

#[test]
fn c11_1_root_html_fixed_grid_content_alignment_reaches_default_runtime() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("parsed Root HTML fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
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
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={cascade:?}",
        layout.error
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("default Root HTML route should publish the Grid paint scene");

    let frame_for_color = |expected| {
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| {
                matches!(
                    render_box.paint(),
                    spinon_render::RuntimePaint::Opaque(color) if color == expected
                )
            })
            .unwrap_or_else(|| panic!("expected Root HTML paint color {expected:?}"));
        let frame = render_box.frame_css_px();
        [frame.x(), frame.y(), frame.width(), frame.height()]
    };

    assert_eq!(
        frame_for_color(spinon_render::OpaqueCssSrgb::new(244, 244, 244)),
        [0.0, 0.0, 240.0, 180.0],
        "Grid container frame"
    );
    for (color, expected) in [
        (
            spinon_render::OpaqueCssSrgb::new(229, 57, 53),
            [43.328125, 22.5],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(30, 136, 229),
            [136.65625, 22.5],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(67, 160, 71),
            [43.328125, 107.5],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(142, 36, 170),
            [136.65625, 107.5],
        ),
    ] {
        let actual = frame_for_color(color);
        for (component, (actual, expected)) in actual
            .into_iter()
            .zip([expected[0], expected[1], 10.0, 10.0])
            .enumerate()
        {
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "Grid item {color:?} frame component {component}: actual={actual}, expected={expected} ± 1/64 CSS px"
            );
        }
    }
}
