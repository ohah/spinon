use super::*;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-template-areas-precomparison.html"
);

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
        .expect("semantic area item should have its computed paint");
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn area_frames(scene: &spinon_render::RuntimeRenderSnapshot) -> [[f32; 4]; 3] {
    [
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(229, 72, 77)),
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(58, 143, 91)),
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(10, 132, 255)),
    ]
}

#[test]
fn c11_1_default_root_html_runtime_places_semantic_named_areas_independent_of_source_order() {
    let (session, baseline) = run(SOURCE);
    assert_eq!(
        baseline.state,
        crate::session::RuntimeLayoutState::Ready,
        "{baseline:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = baseline
        .completed
        .expect("named-area Grid layout should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish the named-area scene");
    assert_eq!(
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 120.0, 70.0]
    );
    let expected = [
        [0.0, 0.0, 120.0, 20.0],
        [0.0, 20.0, 40.0, 50.0],
        [40.0, 20.0, 80.0, 50.0],
    ];
    assert_eq!(area_frames(scene), expected);

    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade
        .completed
        .expect("named-area typed styles should complete");
    assert_eq!(
        completed.key, cascade_completed.key,
        "layout and typed areas must use one document/style/environment revision"
    );
    let styles = &cascade_completed.roots[0].styles;
    let grid = styles
        .elements
        .iter()
        .find_map(|style| style.grid_layout.as_ref())
        .expect("the typed root Grid should reach the layout projector");
    assert_eq!(
        grid.template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 2))
    );

    let reordered = SOURCE
        .replace("#layout", "#renamed-grid")
        .replace("id=\"layout\"", "id=\"renamed-grid\"")
        .replace("data-area", "data-zone")
        .replace(
            "<div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>\n      <div data-zone=\"side\"></div>",
            "<div data-zone=\"side\"></div>\n      <div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>",
        );
    let (_, reordered) = run(&reordered);
    assert_eq!(
        reordered.state,
        crate::session::RuntimeLayoutState::Ready,
        "renamed selectors and reordered items should preserve named placement: {reordered:?}"
    );
    let reordered_scene = reordered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("reordered named-area Grid should publish a scene");
    assert_eq!(area_frames(&reordered_scene), expected);
}

#[test]
fn c11_1_default_root_html_runtime_rejects_out_of_scope_named_area_candidates() {
    let cases = [
        (
            "wrong root width",
            SOURCE.replace("width: 120px;", "width: 121px;"),
        ),
        (
            "root max-width constraint",
            SOURCE.replace("width: 120px;", "width: 120px; max-width: 121px;"),
        ),
        (
            "wrong column tracks",
            SOURCE.replace(
                "grid-template-columns: 40px 80px;",
                "grid-template-columns: 41px 79px;",
            ),
        ),
        (
            "wrong row tracks",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 21px 49px;",
            ),
        ),
        (
            "nonzero gap",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; row-gap: 1px;",
            ),
        ),
        (
            "nonzero item edge",
            SOURCE.replace("grid-area: header;", "grid-area: header; margin-left: 1px;"),
        ),
        (
            "nonrectangular template",
            SOURCE.replace(
                "\"header header\" \"side content\"",
                "\"header side\" \"side content\"",
            ),
        ),
        (
            "template area and track dimension mismatch",
            SOURCE.replace(
                "grid-template-areas: \"header header\" \"side content\";",
                "grid-template-areas: \"header header\" \"side content\" \"extra extra\";",
            ),
        ),
        (
            "missing semantic item",
            SOURCE.replace("<div data-area=\"header\"></div>\n", ""),
        ),
        (
            "duplicate semantic area",
            SOURCE.replace("data-area=\"side\"", "data-area=\"header\""),
        ),
        (
            "unknown semantic area",
            SOURCE.replace("grid-area: side;", "grid-area: sidebar;"),
        ),
        (
            "extra visible item",
            SOURCE.replace("</main>", "<div data-area=\"extra\"></div>\n    </main>"),
        ),
        (
            "hidden extra item",
            SOURCE.replace("</main>", "<div style=\"display:none\"></div>\n    </main>"),
        ),
        (
            "image item",
            SOURCE.replace(
                "<div data-area=\"side\"></div>",
                "<img data-area=\"side\" />",
            ),
        ),
        (
            "anonymous text item",
            SOURCE.replace(
                "<main id=\"layout\">",
                "<main id=\"layout\">unexpected text",
            ),
        ),
        (
            "nonempty div item",
            SOURCE.replace(
                "<div data-area=\"side\"></div>",
                "<div data-area=\"side\"><span></span></div>",
            ),
        ),
        (
            "numeric item placement",
            SOURCE.replace(
                "grid-area: header;",
                "grid-area: header; grid-column-start: 1;",
            ),
        ),
        (
            "nondefault auto flow",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-flow: column;",
            ),
        ),
        (
            "implicit row sizing",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-rows: 10px;",
            ),
        ),
        (
            "implicit column sizing",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-columns: 10px;",
            ),
        ),
        (
            "nested grid item",
            SOURCE.replace(
                "[data-area=\"header\"] {",
                "[data-area=\"header\"] { display: grid;",
            ),
        ),
    ];

    for (name, source) in cases {
        let (_, result) = run(&source);
        assert_eq!(
            result.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope named-area case={name} should fail closed: {result:?}"
        );
        assert!(
            result.error.is_some(),
            "out-of-scope named-area case={name} should preserve a failure diagnostic"
        );
    }
}

#[test]
fn c11_1_failed_new_environment_does_not_retain_a_stale_named_area_scene() {
    let responsive = SOURCE.replace(
        "  </style>",
        "  @media (max-width: 500px) { [data-area=\"header\"] { grid-area: sidebar; } }\n  </style>",
    );
    let (session, initial) = run(&responsive);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "wide environment should produce the supported named-area scene: {initial:?}"
    );
    let initial_completed = initial.completed.expect("wide scene should complete");
    let initial_key = initial_completed.key;
    assert!(initial_completed.render_snapshot.is_some());

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    let narrow_environment = session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("narrow viewport should update the environment revision");
    let narrow = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        narrow.state,
        crate::session::RuntimeLayoutState::Failed,
        "the media-query placement is outside the named-area subset: {narrow:?}"
    );
    assert!(
        narrow.requested.is_some_and(|key| {
            key.environment_revision == narrow_environment.environment_revision.get()
                && key.environment_revision > initial_key.environment_revision
        }),
        "the failed result must identify the newer environment revision: {narrow:?}"
    );
    assert!(
        narrow.completed.is_none(),
        "a failed newer request must not expose the prior named-area scene: {narrow:?}"
    );
    assert!(narrow.error.is_some());
}

#[test]
fn c11_1_invalid_area_declaration_uses_css_cascade_fallback() {
    let invalid_without_items = SOURCE
        .replace("\"header header\" \"side content\"", "1px")
        .replace("grid-area: header;", "")
        .replace("grid-area: side;", "")
        .replace("grid-area: content;", "")
        .replace(
            "background-color: #e5484d;",
            "width: 120px; height: 20px; background-color: #e5484d;",
        )
        .replace(
            "background-color: #3a8f5b;",
            "width: 40px; height: 50px; background-color: #3a8f5b;",
        )
        .replace(
            "background-color: #0a84ff;",
            "width: 80px; height: 50px; background-color: #0a84ff;",
        );
    let (session, result) = run(&invalid_without_items);
    assert_eq!(
        result.state,
        crate::session::RuntimeLayoutState::Ready,
        "CSS-invalid grid-template-areas should drop to computed none when item placement also falls back: {result:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
}
