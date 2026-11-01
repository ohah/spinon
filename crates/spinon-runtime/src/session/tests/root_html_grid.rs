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
fn c11_1_root_html_default_runtime_applies_fixed_px_grid_gaps() {
    const SOURCE: &str = include_str!(
        "../../../../../tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html"
    );

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
            .expect("semantic Grid box should have its computed paint");
        let frame = render_box.frame_css_px();
        [frame.x(), frame.y(), frame.width(), frame.height()]
    }

    fn assert_gap_scene(scene: &spinon_render::RuntimeRenderSnapshot) {
        assert_eq!(
            frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(16, 24, 32)),
            [0.0, 0.0, 130.0, 65.0],
            "gap space belongs between tracks and does not expand the root"
        );
        for (color, expected, name) in [
            (
                spinon_render::OpaqueCssSrgb::new(229, 72, 77),
                [0.0, 0.0, 40.0, 30.0],
                "first cell",
            ),
            (
                spinon_render::OpaqueCssSrgb::new(58, 143, 91),
                [50.0, 0.0, 80.0, 30.0],
                "second cell",
            ),
            (
                spinon_render::OpaqueCssSrgb::new(10, 132, 255),
                [0.0, 35.0, 40.0, 30.0],
                "third cell",
            ),
            (
                spinon_render::OpaqueCssSrgb::new(242, 201, 76),
                [50.0, 35.0, 80.0, 30.0],
                "fourth cell",
            ),
        ] {
            assert_eq!(frame_for_paint(scene, color), expected, "{name}");
        }
    }

    let (session, layout) = run(SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "fixed px gaps should publish through the default Root HTML runtime: {layout:?}; cascade: {:?}",
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .expect("fixed px gap layout should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("fixed px gap layout should publish paint");
    assert_gap_scene(scene);
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("Root HTML gap cascade should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| {
            style
                .properties
                .get("display")
                .is_some_and(|value| value == "grid")
        })
        .expect("the parsed author Grid remains in the computed snapshot");
    assert_eq!(
        grid.properties.get("row-gap").map(String::as_str),
        Some("5px")
    );
    assert_eq!(
        grid.properties.get("column-gap").map(String::as_str),
        Some("10px")
    );
    assert_eq!(
        grid.layout_spacing.row_gap,
        spinon_style::ComputedCssSpacingValue::LengthPx(5.0)
    );
    assert_eq!(
        grid.layout_spacing.column_gap,
        spinon_style::ComputedCssSpacingValue::LengthPx(10.0)
    );

    let longhands = SOURCE.replace("gap: 5px 10px;", "row-gap: 5px;\n        column-gap: 10px;");
    let (_, longhand_layout) = run(&longhands);
    assert_eq!(
        longhand_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "equivalent row/column longhands should reach the same profile"
    );
    assert_gap_scene(
        longhand_layout
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.as_ref())
            .expect("equivalent longhands should publish a scene"),
    );

    let renamed = SOURCE
        .replace("#gap-grid", ".renamed-grid")
        .replace("id=\"gap-grid\"", "class=\"renamed-grid\"")
        .replace("<body>", "<body><div style=\"display:none\"></div>");
    let (_, renamed_layout) = run(&renamed);
    assert_eq!(
        renamed_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "changing Grid selector/identity and inserting a hidden prefix must preserve layout"
    );
    assert_gap_scene(
        renamed_layout
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.as_ref())
            .expect("renamed Grid should publish a scene"),
    );

    let zero_gap_source = SOURCE.replace("gap: 5px 10px;", "gap: 0px;");
    let (_, zero_gap_layout) = run(&zero_gap_source);
    assert_eq!(
        zero_gap_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "the preexisting zero-gap profile should remain available"
    );
    let zero_gap_scene = zero_gap_layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("zero-gap Grid should publish a scene");
    for (color, expected) in [
        (
            spinon_render::OpaqueCssSrgb::new(229, 72, 77),
            [0.0, 0.0, 40.0, 30.0],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(58, 143, 91),
            [40.0, 0.0, 80.0, 30.0],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(10, 132, 255),
            [0.0, 30.0, 40.0, 30.0],
        ),
        (
            spinon_render::OpaqueCssSrgb::new(242, 201, 76),
            [40.0, 30.0, 80.0, 30.0],
        ),
    ] {
        assert_eq!(frame_for_paint(zero_gap_scene, color), expected);
    }

    let negative_gap_source = SOURCE.replace("gap: 5px 10px;", "gap: -1px 10px;");
    let (negative_gap_session, negative_gap_layout) = run(&negative_gap_source);
    assert_eq!(
        negative_gap_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "negative gap should fail closed unless Stylo discards the declaration: {negative_gap_layout:?}; cascade={:?}",
        negative_gap_session.ua_cascade_snapshot()
    );
    let negative_gap_scene = negative_gap_layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("discarded negative gap declaration should keep a zero-gap scene");
    assert_eq!(
        frame_for_paint(
            negative_gap_scene,
            spinon_render::OpaqueCssSrgb::new(58, 143, 91)
        ),
        [40.0, 0.0, 80.0, 30.0],
        "invalid negative shorthand must not leak its valid second component"
    );

    for (name, declaration) in [
        ("ASCII-case variants", "GAP: -1PX 10px !IMPORTANT;"),
        ("comment-separated values", "gap: -1px/**/ 10px;"),
    ] {
        let source = SOURCE.replace("gap: 5px 10px;", declaration);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "CSS-invalid negative gap ({name}) should be discarded: {layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        let scene = layout
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.as_ref())
            .expect("discarded CSS-invalid gap should publish the scene without that declaration");
        assert_eq!(
            frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(58, 143, 91)),
            [40.0, 0.0, 80.0, 30.0],
            "invalid negative shorthand ({name}) must not publish its column gutter"
        );
    }

    let inline_negative_gap_source = SOURCE.replace(
        "<section id=\"gap-grid\">",
        "<section id=\"gap-grid\" style=\"gap: -1px 10px;\">",
    );
    let (inline_negative_session, inline_negative_layout) = run(&inline_negative_gap_source);
    assert_eq!(
        inline_negative_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "inline negative gap declaration should be discarded without masking stylesheet CSS"
    );
    assert_gap_scene(
        inline_negative_layout
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.as_ref())
            .expect("valid stylesheet gap should survive invalid inline declaration"),
    );
    assert!(
        inline_negative_session
            .ua_cascade_snapshot()
            .completed
            .expect("inline negative gap cascade should complete")
            .roots[0]
            .styles
            .diagnostics
            .is_empty()
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
            "percentage gap",
            "unsupported_computed_value",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        column-gap: 10%;",
            ),
        ),
        (
            "calc gap",
            "unsupported_computed_value",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        row-gap: calc(1px + 1%);",
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
            "unsupported_computed_value",
            SOURCE.replace(".item-a {", ".item-a { grid-column: 2;"),
        ),
        (
            "non-default auto flow",
            "unsupported_computed_value",
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
            "unsupported_computed_value",
            SOURCE.replace(
                "grid-template-rows: 70px;",
                "grid-template-rows: 70px;\n        grid-template-areas: \"a b\";",
            ),
        ),
        (
            "explicit implicit row sizing",
            "unsupported_computed_value",
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
        let expected_gap_property = match name {
            "percentage gap" => Some("column-gap"),
            "calc gap" => Some("row-gap"),
            _ => None,
        };
        if let Some(property) = expected_gap_property {
            assert_eq!(
                result
                    .error
                    .as_ref()
                    .and_then(|error| error.property.as_deref()),
                Some(property),
                "out-of-scope case={name} should identify the rejected gap longhand"
            );
        }
        assert!(
            result.completed.is_none(),
            "out-of-scope case={name} must not publish a partial scene"
        );
    }
}
