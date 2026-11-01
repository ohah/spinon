use super::*;

#[test]
fn c11_1_root_html_default_runtime_projects_author_grid_tracks_and_auto_places_items() {
    const SOURCE: &str = include_str!(
        "../../../../../tests/fixtures/css/c11/root-html-grid-author-precomparison.html"
    );
    const ALTERNATE: &str = r#"<!doctype html>
<html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html, body { margin: 0; }
#grid-node { box-sizing: border-box; display: grid; width: 120px; height: 70px; grid-template-columns: 40px 80px; grid-template-rows: 70px; background-color: #101820; }
#grid-node > div { box-sizing: border-box; }
[data-cell="left"] { width: 40px; height: 70px; background-color: #e5484d; }
[data-cell="right"] { width: 80px; height: 70px; background-color: #3a8f5b; }
</style></head><body><div style="display:none"></div><section id="grid-node"><div data-cell="left"></div><div data-cell="right"></div></section></body></html>"#;

    fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
        let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
            .expect("parsed root HTML should initialize the default runtime");
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(1280.0, 633.0, 1.0, media)
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
            .expect("semantic grid item should have its computed paint");
        let frame = render_box.frame_css_px();
        [frame.x(), frame.y(), frame.width(), frame.height()]
    }

    fn assert_geometry(
        scene: &spinon_render::RuntimeRenderSnapshot,
        left: [f32; 4],
        right: [f32; 4],
    ) {
        let red = frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(229, 72, 77));
        let green = frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(58, 143, 91));
        assert_eq!(red, left, "red grid item frame; green={green:?}");
        assert_eq!(green, right, "green grid item frame; red={red:?}");
    }

    let (session, base) = run(SOURCE);
    assert_eq!(
        base.state,
        crate::session::RuntimeLayoutState::Ready,
        "{base:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let base_completed = base.completed.expect("base Grid layout should complete");
    let base_scene = base_completed
        .render_snapshot
        .as_ref()
        .expect("default root HTML route should publish Grid paint");
    assert_eq!(base_scene.viewport_css_px().width(), 1280.0);
    assert_eq!(base_scene.viewport_css_px().height(), 633.0);
    assert_eq!(
        frame_for_paint(base_scene, spinon_render::OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 120.0, 70.0],
        "the Grid container background should remain in the paint scene"
    );
    assert_geometry(base_scene, [0.0, 0.0, 40.0, 70.0], [40.0, 0.0, 80.0, 70.0]);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("Grid cascade should complete");
    assert_eq!(
        base_completed.key, cascade_completed.key,
        "layout and typed Grid styles must come from the same document/style/environment revision"
    );
    let styles = &cascade_completed.roots[0].styles;
    let root = styles
        .elements
        .iter()
        .find(|style| {
            style
                .properties
                .get("display")
                .is_some_and(|value| value == "grid")
        })
        .expect("author display:grid should remain in the root HTML style snapshot");
    assert_eq!(
        root.properties
            .get("grid-template-columns")
            .map(String::as_str),
        Some("40px 80px")
    );
    assert_eq!(
        root.properties
            .get("grid-template-rows")
            .map(String::as_str),
        Some("70px")
    );
    assert_eq!(
        root.properties.get("grid-auto-flow").map(String::as_str),
        Some("row")
    );
    assert!(
        root.grid_layout.is_some(),
        "typed Grid tracks should reach the layout projector"
    );

    let (_, alternate) = run(ALTERNATE);
    assert_eq!(
        alternate.state,
        crate::session::RuntimeLayoutState::Ready,
        "{alternate:?}"
    );
    let alternate_scene = alternate
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect(
            "changed root tag, attributes, hidden prefix and HostNode IDs should preserve CSS Grid",
        );
    assert_geometry(
        &alternate_scene,
        [0.0, 0.0, 40.0, 70.0],
        [40.0, 0.0, 80.0, 70.0],
    );

    let blockified_inline = ALTERNATE.replace(".item-a {", ".item-a { display: inline;");
    let (_, blockified_inline) = run(&blockified_inline);
    assert_eq!(
        blockified_inline.state,
        crate::session::RuntimeLayoutState::Ready,
        "CSS Grid blockifies an inline-level item before layout: {blockified_inline:?}"
    );
    let blockified_inline_scene = blockified_inline
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("blockified inline Grid item should publish the expected scene");
    assert_geometry(
        &blockified_inline_scene,
        [0.0, 0.0, 40.0, 70.0],
        [40.0, 0.0, 80.0, 70.0],
    );

    let preserved_whitespace_source = SOURCE.replace(
        "grid-template-rows: 70px;",
        "grid-template-rows: 70px;\n        white-space: pre;",
    );
    let (_, preserved_whitespace) = run(&preserved_whitespace_source);
    assert_eq!(
        preserved_whitespace.state,
        crate::session::RuntimeLayoutState::Ready,
        "CSS Grid must ignore whitespace-only text sequences even with white-space:pre: {preserved_whitespace:?}"
    );
    let preserved_whitespace_scene = preserved_whitespace
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("preserved whitespace-only text should not become a grid item");
    assert_geometry(
        &preserved_whitespace_scene,
        [0.0, 0.0, 40.0, 70.0],
        [40.0, 0.0, 80.0, 70.0],
    );

    let reversed = ALTERNATE.replace(
        "<div data-cell=\"left\"></div><div data-cell=\"right\"></div>",
        "<div data-cell=\"right\"></div><div data-cell=\"left\"></div>",
    );
    let (_, reversed) = run(&reversed);
    assert_eq!(
        reversed.state,
        crate::session::RuntimeLayoutState::Ready,
        "{reversed:?}"
    );
    let reversed_scene = reversed
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("source-order control should publish a Grid scene");
    assert_geometry(
        &reversed_scene,
        [40.0, 0.0, 40.0, 70.0],
        [0.0, 0.0, 80.0, 70.0],
    );

    const OVERFLOW_SOURCE: &str =
        include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-visible-overflow.html");
    let (_, overflowing) = run(OVERFLOW_SOURCE);
    assert_eq!(
        overflowing.state,
        crate::session::RuntimeLayoutState::Ready,
        "visible Grid item overflow should retain its computed frame: {overflowing:?}"
    );
    let overflowing_scene = overflowing
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("visible Grid overflow should remain in the render scene");
    assert_eq!(
        frame_for_paint(
            &overflowing_scene,
            spinon_render::OpaqueCssSrgb::new(16, 24, 32)
        ),
        [0.0, 0.0, 120.0, 70.0],
        "the Grid container frame should not expand to fit overflowing items"
    );
    assert_eq!(
        frame_for_paint(
            &overflowing_scene,
            spinon_render::OpaqueCssSrgb::new(229, 72, 77)
        ),
        [0.0, 0.0, 41.0, 71.0],
        "the authored item frame should extend past its track and container edges"
    );
    assert_eq!(
        frame_for_paint(
            &overflowing_scene,
            spinon_render::OpaqueCssSrgb::new(58, 143, 91)
        ),
        [40.0, 0.0, 81.0, 70.0]
    );
}

#[test]
fn c11_1_root_html_default_runtime_rejects_out_of_scope_author_grid_inputs() {
    const SOURCE: &str = include_str!(
        "../../../../../tests/fixtures/css/c11/root-html-grid-author-precomparison.html"
    );

    fn run(source: &str) -> crate::session::RuntimeLayoutSnapshot {
        let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
            .expect("parsed root HTML should initialize the default runtime");
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(1280.0, 633.0, 1.0, media)
            .expect("root metadata should configure the CSS environment");
        session.wait_for_layout_snapshot(Duration::from_secs(5))
    }

    let cases = [
        (
            "nonzero gap",
            "cascade_failed",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        column-gap: 1px;",
            ),
        ),
        (
            "nonzero grid edge",
            "unsupported_computed_value",
            SOURCE.replace(
                "display: grid;",
                "display: grid;\n        padding-left: 1px;",
            ),
        ),
        (
            "nonzero grid margin",
            "unsupported_computed_value",
            SOURCE.replace("display: grid;", "display: grid;\n        margin-top: 1px;"),
        ),
        (
            "nonzero grid border",
            "unsupported_computed_value",
            SOURCE.replace(
                "display: grid;",
                "display: grid;\n        border-left-width: 1px;\n        border-left-style: solid;",
            ),
        ),
        (
            "positioned Grid container",
            "cascade_failed",
            SOURCE.replace("display: grid;", "display: grid;\n        position: relative;"),
        ),
        (
            "positioned Grid item",
            "cascade_failed",
            SOURCE.replace(".item-a {", ".item-a { position: relative;"),
        ),
        (
            "clipping Grid container overflow",
            "unsupported_computed_value",
            SOURCE.replace("display: grid;", "display: grid;\n        overflow: hidden;"),
        ),
        (
            "clipping Grid item overflow",
            "unsupported_computed_value",
            SOURCE.replace(".item-a {", ".item-a { overflow: clip;"),
        ),
        (
            "explicit item placement",
            "cascade_failed",
            SOURCE.replace(".item-a {", ".item-a { grid-column: 2;"),
        ),
        (
            "non-default auto flow",
            "cascade_failed",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        grid-auto-flow: column;",
            ),
        ),
        (
            "repeat track function",
            "cascade_failed",
            SOURCE.replace("40px 80px", "repeat(2, 40px)"),
        ),
        (
            "fractional track",
            "unsupported_computed_value",
            SOURCE.replace("40px 80px", "40px 1fr"),
        ),
        (
            "percentage track",
            "unsupported_computed_value",
            SOURCE.replace("40px 80px", "40px 50%"),
        ),
        (
            "named grid area",
            "cascade_failed",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        grid-template-areas: \"a b\";",
            ),
        ),
        (
            "explicit implicit row sizing",
            "cascade_failed",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        grid-auto-rows: 20px;",
            ),
        ),
        (
            "named explicit line",
            "cascade_failed",
            SOURCE.replace("40px 80px", "[left] 40px 80px"),
        ),
        (
            "implicit row item",
            "unsupported_computed_value",
            SOURCE.replace("</main>", "<div class=\"item-a\"></div>\n    </main>"),
        ),
        (
            "grid item min-width",
            "layout_failed",
            SOURCE.replace(".item-a {", ".item-a { min-width: 0;"),
        ),
        (
            "flex Grid item display is out of scope",
            "unsupported_root_html_default_render_style",
            SOURCE.replace(".item-a {", ".item-a { display: flex;"),
        ),
        (
            "nested Grid item is out of scope",
            "cascade_failed",
            SOURCE.replace(".item-a {", ".item-a { display: grid;"),
        ),
        (
            "non-default item order",
            "cascade_failed",
            SOURCE.replace(".item-a {", ".item-a { order: 1;"),
        ),
        (
            "vertical writing mode",
            "cascade_failed",
            SOURCE.replace(
                "display: grid;",
                "display: grid;\n        writing-mode: vertical-rl;",
            ),
        ),
        (
            "right-to-left Grid direction",
            "unsupported_computed_value",
            SOURCE.replace("display: grid;", "display: grid;\n        direction: rtl;"),
        ),
        (
            "direct Grid text",
            "unsupported_root_html_grid_text_item",
            SOURCE.replace(
                "<main>\n      <div class=\"item-a\"></div>",
                "<main>\n      text\n      <div class=\"item-a\"></div>",
            ),
        ),
        (
            "preserved direct Grid text",
            "unsupported_root_html_grid_text_item",
            SOURCE
                .replace(
                    "grid-template-rows: 70px;",
                    "grid-template-rows: 70px;\n        white-space: pre;",
                )
                .replace(
                    "<main>\n      <div class=\"item-a\"></div>",
                    "<main>\n      text\n      <div class=\"item-a\"></div>",
                ),
        ),
    ];

    for (name, expected_code, source) in cases {
        let result = run(&source);
        assert_eq!(
            result.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope case={name} should fail closed; result={result:?}"
        );
        assert!(
            result.error.is_some(),
            "out-of-scope case={name} should keep a diagnostic"
        );
        assert_eq!(
            result.error.as_ref().map(|error| error.code),
            Some(expected_code),
            "out-of-scope case={name} should keep the expected rejection code"
        );
        assert!(
            result.completed.is_none(),
            "out-of-scope case={name} must not publish a partial scene"
        );
    }
}
