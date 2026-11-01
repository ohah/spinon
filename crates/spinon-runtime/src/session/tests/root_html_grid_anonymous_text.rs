use super::*;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-anonymous-text-precomparison.html"
);

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn frame_matches(frame: &crate::session::RuntimeLayoutFrame, expected: [f32; 4]) -> bool {
    [frame.x, frame.y, frame.width, frame.height] == expected
}

#[test]
fn c11_1_root_html_default_runtime_renders_one_anonymous_grid_text_item() {
    let (session, layout) = run(SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("Root HTML Grid text should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish text paint");
    assert_eq!(scene.texts().len(), 1);
    let text = &scene.texts()[0];
    assert_eq!(text.origin_x_css_px(), 0.0);
    assert_eq!(text.origin_y_css_px(), 0.0);
    assert_eq!(
        text.color(),
        spinon_render::RuntimeSrgbColor::new(229.0 / 255.0, 72.0 / 255.0, 77.0 / 255.0, 1.0,)
            .unwrap()
    );
    assert_eq!(text.glyphs().len(), 2, "AB should rasterize as two glyphs");
    let baselines = text
        .glyphs()
        .iter()
        .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(baselines.len(), 1, "AB should occupy one text line");
    assert!(
        completed
            .frames
            .iter()
            .any(|frame| frame_matches(frame, [0.0, 0.0, 40.0, 70.0])),
        "the anonymous Grid item should stretch to the first 40×70 cell: {:?}",
        completed.frames
    );
    let grid = scene
        .boxes()
        .iter()
        .find(|render_box| {
            render_box.frame_css_px().width() == 120.0
                && render_box.frame_css_px().height() == 70.0
                && matches!(
                    render_box.paint(),
                    spinon_render::RuntimePaint::Opaque(color)
                        if color == spinon_render::OpaqueCssSrgb::new(16, 24, 32)
                )
        })
        .expect("Grid container background should remain in the scene");
    let grid_frame = grid.frame_css_px();
    assert!(frame_matches(
        &crate::session::RuntimeLayoutFrame {
            node_id: grid.node_id().get(),
            x: grid_frame.x(),
            y: grid_frame.y(),
            width: grid_frame.width(),
            height: grid_frame.height(),
        },
        [0.0, 0.0, 120.0, 70.0]
    ));
    assert!(
        scene
            .paint_commands()
            .iter()
            .any(|command| matches!(command, spinon_render::RuntimePaintCommand::DrawText { .. })),
        "the runtime scene should contain a text paint command"
    );
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade = cascade.completed.expect("Root HTML styles should be ready");
    assert_eq!(completed.key, cascade.key);
}

#[test]
fn c11_1_root_html_default_runtime_keeps_adjacent_text_sources_in_one_grid_item() {
    let split_source = SOURCE.replace("AB</main>", "A<!-- source boundary -->B</main>");
    let (session, layout) = run(&split_source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "adjacent text nodes should retain one anonymous item: {layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("adjacent text sources should publish one scene");
    assert_eq!(scene.texts().len(), 1);
    assert_eq!(scene.texts()[0].origin_x_css_px(), 0.0);
    assert_eq!(scene.texts()[0].origin_y_css_px(), 0.0);
    assert_eq!(scene.texts()[0].glyphs().len(), 2);
}

#[test]
fn c11_1_root_html_default_runtime_omits_whitespace_only_grid_text() {
    let whitespace = SOURCE.replace(
        "<main id=\"layout\">AB</main>",
        "<main id=\"layout\"> \n  </main>",
    );
    let (session, layout) = run(&whitespace);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("whitespace-only Grid text should keep a ready scene");
    assert!(scene.texts().is_empty());
    assert!(scene.boxes().iter().any(|render_box| {
        render_box.frame_css_px().width() == 120.0
            && render_box.frame_css_px().height() == 70.0
            && matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color)
                    if color == spinon_render::OpaqueCssSrgb::new(16, 24, 32)
            )
    }));
}

#[test]
fn c11_1_root_html_default_runtime_keeps_mixed_and_out_of_scope_text_fail_closed() {
    let mixed = SOURCE.replace(
        "<main id=\"layout\">AB</main>",
        "<main id=\"layout\">AB<div></div></main>",
    );
    let cases = [
        (
            "mixed visible child",
            "unsupported_root_html_grid_text_item",
            mixed,
        ),
        (
            "non-baseline text value",
            "unsupported_root_html_grid_text_item",
            SOURCE.replace("AB</main>", "ABC</main>"),
        ),
        (
            "preserved whitespace style",
            "unsupported_root_html_default_render_style",
            SOURCE.replace("white-space: normal;", "white-space: pre;"),
        ),
        (
            "unsupported text color",
            "unsupported_root_html_default_render_style",
            SOURCE.replace("color: #e5484d;", "color: #ff0000;"),
        ),
    ];
    for (name, code, source) in cases {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope case={name} should fail closed; result={layout:?}"
        );
        assert_eq!(layout.error.as_ref().map(|error| error.code), Some(code));
        assert!(layout.completed.is_none());
        assert!(
            session.ua_cascade_snapshot().completed.is_some(),
            "the source style snapshot should remain available after rejected layout"
        );
    }
}
