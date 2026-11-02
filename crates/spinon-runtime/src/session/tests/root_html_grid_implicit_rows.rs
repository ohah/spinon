use super::*;

#[path = "root_html_grid_implicit_rows/composition.rs"]
mod composition;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-implicit-rows-runtime.html");

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
        .expect("fixture node should contribute its unique paint color");
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn assert_frames(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected: &[(spinon_render::OpaqueCssSrgb, [f32; 4])],
) {
    for (color, frame) in expected {
        assert_eq!(frame_for_paint(scene, *color), *frame);
    }
}

const ROOT: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(16, 24, 32);
const RED: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);
const BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(59, 130, 246);
const GREEN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(58, 143, 91);
const ORANGE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(245, 158, 11);
const PURPLE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(168, 85, 247);
const CYAN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(6, 182, 212);

fn ready_scene(
    session: &RuntimeSession,
    layout: &crate::session::RuntimeLayoutSnapshot,
) -> spinon_render::RuntimeRenderSnapshot {
    let cascade = session.ua_cascade_snapshot();
    let computed_auto_rows = cascade
        .completed
        .as_ref()
        .into_iter()
        .flat_map(|completed| completed.roots.iter())
        .flat_map(|root| root.styles.elements.iter())
        .filter_map(|style| style.grid_layout.as_ref())
        .map(|grid| {
            (
                grid.auto_rows.clone(),
                grid.template_areas.clone(),
                grid.columns.clone(),
                grid.rows.clone(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; computed auto rows={computed_auto_rows:?}",
        layout.error
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    let scene = completed
        .render_snapshot
        .clone()
        .expect("default Root HTML runtime should publish a scene");
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("styles should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let projected_grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|style| style.grid_layout.as_ref())
        .expect("computed Root HTML grid styles should remain observable");
    assert_eq!(
        projected_grid.auto_rows,
        spinon_style::ComputedCssGridAutoRows::LengthPx(15.0)
    );
    scene
}

#[test]
fn c11_1_root_html_default_runtime_places_fixed_implicit_rows_and_preserves_hidden_items() {
    let (session, layout) = run(SOURCE);
    let scene = ready_scene(&session, &layout);
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 100.0, 40.0]),
            (RED, [0.0, 0.0, 40.0, 20.0]),
            (BLUE, [45.0, 0.0, 55.0, 20.0]),
            (GREEN, [0.0, 25.0, 40.0, 15.0]),
        ],
    );
    assert!(!scene.boxes().iter().any(|render_box| {
        matches!(render_box.paint(), spinon_render::RuntimePaint::Opaque(color) if color == ORANGE)
    }));

    let explicit_row = SOURCE.replace(
        "grid-auto-rows: 15px;",
        "grid-auto-flow: row;\n        grid-auto-rows: 15px;",
    );
    let (session, layout) = run(&explicit_row);
    let explicit_scene = ready_scene(&session, &layout);
    assert_frames(
        &explicit_scene,
        &[
            (ROOT, [0.0, 0.0, 100.0, 40.0]),
            (RED, [0.0, 0.0, 40.0, 20.0]),
            (BLUE, [45.0, 0.0, 55.0, 20.0]),
            (GREEN, [0.0, 25.0, 40.0, 15.0]),
        ],
    );

    let renamed = SOURCE
        .replace(".item", ".tile")
        .replace("class=\"item", "class=\"tile")
        .replace("#a {", "#first {\n      ")
        .replace("id=\"a\"", "id=\"first\"");
    let (session, layout) = run(&renamed);
    let renamed_scene = ready_scene(&session, &layout);
    assert_frames(
        &renamed_scene,
        &[
            (RED, [0.0, 0.0, 40.0, 20.0]),
            (BLUE, [45.0, 0.0, 55.0, 20.0]),
            (GREEN, [0.0, 25.0, 40.0, 15.0]),
        ],
    );

    let reordered = SOURCE.replace(
        "<div id=\"a\" class=\"item\"></div>\n      <div id=\"b\" class=\"item\"></div>",
        "<div id=\"b\" class=\"item\"></div>\n      <div id=\"a\" class=\"item\"></div>",
    );
    let (session, layout) = run(&reordered);
    let reordered_scene = ready_scene(&session, &layout);
    assert_frames(
        &reordered_scene,
        &[
            (RED, [45.0, 0.0, 40.0, 20.0]),
            (BLUE, [0.0, 0.0, 55.0, 20.0]),
            (GREEN, [0.0, 25.0, 40.0, 15.0]),
        ],
    );
}

#[test]
fn c11_1_root_html_default_runtime_handles_zero_and_repeated_fixed_implicit_rows() {
    let zero = SOURCE
        .replace("height: 40px;", "height: 25px;")
        .replace("grid-auto-rows: 15px;", "grid-auto-rows: 0px;")
        .replace(
            "#c { width: 40px; height: 15px;",
            "#c { width: 40px; height: 0px;",
        );
    let (_session, layout) = run(&zero);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{layout:?}"
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("zero-size implicit row layout should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("zero-size implicit row should retain a published scene");
    assert_frames(
        scene,
        &[
            (ROOT, [0.0, 0.0, 100.0, 25.0]),
            (RED, [0.0, 0.0, 40.0, 20.0]),
            (BLUE, [45.0, 0.0, 55.0, 20.0]),
        ],
    );
    assert!(
        completed.frames.iter().any(|frame| {
            [frame.x, frame.y, frame.width, frame.height] == [0.0, 25.0, 40.0, 0.0]
        })
    );

    let two_rows = SOURCE
        .replace("height: 40px;", "height: 60px;")
        .replace(".beyond { display: none; }", ".beyond { display: block; }");
    let (session, layout) = run(&two_rows);
    let scene = ready_scene(&session, &layout);
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 100.0, 60.0]),
            (RED, [0.0, 0.0, 40.0, 20.0]),
            (BLUE, [45.0, 0.0, 55.0, 20.0]),
            (GREEN, [0.0, 25.0, 40.0, 15.0]),
            (ORANGE, [45.0, 25.0, 55.0, 15.0]),
            (PURPLE, [0.0, 45.0, 40.0, 15.0]),
            (CYAN, [45.0, 45.0, 55.0, 15.0]),
        ],
    );
}

#[test]
fn c11_1_root_html_default_runtime_rejects_unsupported_implicit_row_combinations_atomically() {
    let cases = [
        (
            "column flow",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "grid-auto-flow: column;\n        grid-auto-rows: 15px;",
            ),
        ),
        (
            "dense flow",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "grid-auto-flow: row dense;\n        grid-auto-rows: 15px;",
            ),
        ),
        (
            "implicit columns",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "grid-auto-columns: 10px;\n        grid-auto-rows: 15px;",
            ),
        ),
        (
            "multiple implicit rows",
            SOURCE.replace("grid-auto-rows: 15px;", "grid-auto-rows: 15px 20px;"),
        ),
        (
            "percentage implicit row",
            SOURCE.replace("grid-auto-rows: 15px;", "grid-auto-rows: 50%;"),
        ),
        (
            "fraction implicit row",
            SOURCE.replace("grid-auto-rows: 15px;", "grid-auto-rows: 1fr;"),
        ),
        (
            "intrinsic implicit row",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "grid-auto-rows: minmax(10px, 20px);",
            ),
        ),
        (
            "intrinsic explicit track",
            SOURCE.replace(
                "grid-template-columns: 40px 55px;",
                "grid-template-columns: minmax(10px, 1fr) 55px;",
            ),
        ),
        (
            "auto-repeat explicit track",
            SOURCE.replace(
                "grid-template-columns: 40px 55px;",
                "grid-template-columns: repeat(auto-fill, 40px);",
            ),
        ),
        (
            "named areas",
            SOURCE.replace(
                "grid-template-rows: 20px;",
                "grid-template-rows: 20px;\n        grid-template-areas: \"left right\";",
            ),
        ),
        (
            "authored placement",
            SOURCE.replace("#c { width:", "#c { grid-row: 2; width:"),
        ),
        (
            "named authored placement",
            SOURCE.replace("#c { width:", "#c { grid-row: named; width:"),
        ),
        (
            "nonzero order",
            SOURCE.replace("#c { width:", "#c { order: 1; width:"),
        ),
        (
            "non-default alignment",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "align-items: start;\n        grid-auto-rows: 15px;",
            ),
        ),
        (
            "clipped overflow",
            SOURCE.replace(
                "grid-auto-rows: 15px;",
                "overflow: hidden;\n        grid-auto-rows: 15px;",
            ),
        ),
        (
            "visible text item",
            SOURCE.replace(
                "<div id=\"c\" class=\"item\"></div>",
                "<div id=\"c\" class=\"item\">text</div>",
            ),
        ),
        (
            "nested item",
            SOURCE.replace(
                "<div id=\"c\" class=\"item\"></div>",
                "<div id=\"c\" class=\"item\"><div></div></div>",
            ),
        ),
    ];
    for (name, source) in cases {
        let (_session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "case={name}; result={layout:?}"
        );
        assert!(
            layout.error.is_some(),
            "case={name} should retain its error"
        );
        let expected_code = match name {
            "visible text item" | "nested item" => "layout_failed",
            "nonzero order"
            | "non-default alignment"
            | "intrinsic explicit track"
            | "auto-repeat explicit track" => "cascade_failed",
            _ => "unsupported_computed_value",
        };
        assert_eq!(
            layout.error.as_ref().map(|error| error.code),
            Some(expected_code),
            "case={name} should fail at its expected projection or layout boundary"
        );
        assert!(
            layout.completed.is_none(),
            "case={name} must not publish a partial scene"
        );
    }
}
