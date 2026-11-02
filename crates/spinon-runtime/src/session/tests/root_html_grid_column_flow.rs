use super::*;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-precomparison.html"
);
const IMPLICIT_COLUMNS_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-implicit-columns-precomparison.html"
);
const PERCENTAGE_COLUMNS_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-percent-auto-columns-precomparison.html"
);
const ROOT: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(16, 24, 32);
const RED: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);
const GREEN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(58, 143, 91);
const BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(56, 112, 224);
const YELLOW: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(240, 204, 70);
const IMPLICIT_ITEM_COLORS: [spinon_render::OpaqueCssSrgb; 12] = [
    spinon_render::OpaqueCssSrgb::new(229, 72, 77),
    spinon_render::OpaqueCssSrgb::new(58, 143, 91),
    spinon_render::OpaqueCssSrgb::new(56, 112, 224),
    spinon_render::OpaqueCssSrgb::new(240, 204, 70),
    spinon_render::OpaqueCssSrgb::new(155, 89, 182),
    spinon_render::OpaqueCssSrgb::new(230, 126, 34),
    spinon_render::OpaqueCssSrgb::new(22, 160, 133),
    spinon_render::OpaqueCssSrgb::new(211, 84, 0),
    spinon_render::OpaqueCssSrgb::new(142, 68, 173),
    spinon_render::OpaqueCssSrgb::new(41, 128, 185),
    spinon_render::OpaqueCssSrgb::new(192, 57, 43),
    spinon_render::OpaqueCssSrgb::new(39, 174, 96),
];

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

fn frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: spinon_render::OpaqueCssSrgb,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(actual) if actual == color
            )
        })
        .unwrap_or_else(|| panic!("missing semantic paint {color:?}"));
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn ready_scene(
    session: &RuntimeSession,
    layout: &crate::session::RuntimeLayoutSnapshot,
    expected_auto_flow: spinon_style::ComputedCssGridAutoFlow,
) -> spinon_render::RuntimeRenderSnapshot {
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("Grid styles should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let grid_style = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|element| element.grid_layout.is_some())
        .expect("computed Grid style should be retained");
    let grid = grid_style.grid_layout.as_ref().unwrap();
    assert_eq!(
        grid.auto_flow,
        expected_auto_flow,
        "grid-auto-flow property={:?}; full computed Grid={grid:?}",
        grid_style.properties.get("grid-auto-flow")
    );
    assert_eq!(
        grid.columns,
        vec![
            spinon_style::ComputedCssGridTrackSize::LengthPx(30.0),
            spinon_style::ComputedCssGridTrackSize::LengthPx(70.0),
        ],
        "typed explicit columns"
    );
    assert_eq!(
        grid.rows,
        vec![
            spinon_style::ComputedCssGridTrackSize::LengthPx(20.0),
            spinon_style::ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        "typed explicit rows"
    );
    completed
        .render_snapshot
        .clone()
        .expect("default Root HTML route should publish a scene")
}

fn ready_implicit_column_scene(
    session: &RuntimeSession,
    layout: &crate::session::RuntimeLayoutSnapshot,
) -> spinon_render::RuntimeRenderSnapshot {
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("Grid styles should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|element| element.grid_layout.is_some())
        .and_then(|element| element.grid_layout.as_ref())
        .expect("computed Grid style should be retained");
    assert_eq!(
        grid.auto_flow,
        spinon_style::ComputedCssGridAutoFlow::Column
    );
    assert_eq!(
        grid.columns,
        vec![
            spinon_style::ComputedCssGridTrackSize::LengthPx(30.0),
            spinon_style::ComputedCssGridTrackSize::LengthPx(40.0),
        ]
    );
    assert_eq!(
        grid.rows,
        vec![
            spinon_style::ComputedCssGridTrackSize::LengthPx(20.0),
            spinon_style::ComputedCssGridTrackSize::LengthPx(40.0),
        ]
    );
    assert_eq!(grid.auto_rows, spinon_style::ComputedCssGridAutoRows::Auto);
    assert_eq!(
        grid.auto_columns,
        spinon_style::ComputedCssGridAutoColumns::TrackList(vec![
            spinon_style::ComputedCssGridTrackSize::LengthPx(15.0),
            spinon_style::ComputedCssGridTrackSize::LengthPx(25.0),
        ])
    );
    completed
        .render_snapshot
        .clone()
        .expect("the default Root HTML route should publish the complete implicit-column scene")
}

#[test]
fn c11_1_root_html_sparse_column_flow_matches_chrome_frames_and_row_control() {
    let (session, layout) = run(SOURCE);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, ROOT), [0.0, 0.0, 100.0, 60.0]);
    assert_eq!(frame(&scene, RED), [0.0, 0.0, 30.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [0.0, 20.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [30.0, 0.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [30.0, 20.0, 70.0, 40.0]);

    let with_gaps = SOURCE.replace(
        "grid-auto-flow: column;",
        "grid-auto-flow: column;\n        row-gap: 3px;\n        column-gap: 5px;",
    );
    let (session, layout) = run(&with_gaps);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, RED), [0.0, 0.0, 30.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [0.0, 23.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [35.0, 0.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [35.0, 23.0, 70.0, 40.0]);

    let absolute_calc = SOURCE.replace(
        "grid-template-columns: 30px 70px;",
        "grid-template-columns: calc(10px + 20px) 70px;",
    );
    let (session, layout) = run(&absolute_calc);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, RED), [0.0, 0.0, 30.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [0.0, 20.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [30.0, 0.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [30.0, 20.0, 70.0, 40.0]);

    let row_flow = SOURCE.replace("grid-auto-flow: column;", "grid-auto-flow: row;");
    let (session, layout) = run(&row_flow);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Row,
    );
    assert_eq!(frame(&scene, RED), [0.0, 0.0, 30.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [30.0, 0.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [0.0, 20.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [30.0, 20.0, 70.0, 40.0]);
}

#[test]
fn c11_1_root_html_column_flow_repeats_fixed_px_implicit_columns_and_gaps() {
    let (session, layout) = run(IMPLICIT_COLUMNS_SOURCE);
    let scene = ready_implicit_column_scene(&session, &layout);
    assert_eq!(frame(&scene, ROOT), [0.0, 0.0, 175.0, 63.0]);

    for (color, expected) in IMPLICIT_ITEM_COLORS.into_iter().zip([
        [0.0, 0.0, 30.0, 20.0],
        [0.0, 23.0, 30.0, 40.0],
        [35.0, 0.0, 40.0, 20.0],
        [35.0, 23.0, 40.0, 40.0],
        [80.0, 0.0, 15.0, 20.0],
        [80.0, 23.0, 15.0, 40.0],
        [100.0, 0.0, 25.0, 20.0],
        [100.0, 23.0, 25.0, 40.0],
        [130.0, 0.0, 15.0, 20.0],
        [130.0, 23.0, 15.0, 40.0],
        [150.0, 0.0, 25.0, 20.0],
        [150.0, 23.0, 25.0, 40.0],
    ]) {
        assert_eq!(frame(&scene, color), expected, "item color {color:?}");
    }
}

#[test]
fn c11_1_root_html_column_flow_repeats_percentage_implicit_columns_in_the_default_route() {
    let (session, layout) = run(PERCENTAGE_COLUMNS_SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("Grid styles should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|element| element.grid_layout.as_ref())
        .expect("percentage Grid style should be retained");
    assert_eq!(
        grid.auto_columns,
        spinon_style::ComputedCssGridAutoColumns::TrackList(vec![
            spinon_style::ComputedCssGridTrackSize::Percentage(0.1),
            spinon_style::ComputedCssGridTrackSize::Percentage(0.15),
        ])
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish the complete scene");
    assert_eq!(frame(scene, ROOT), [0.0, 0.0, 240.0, 63.0]);
    for (color, expected) in IMPLICIT_ITEM_COLORS.into_iter().zip([
        [0.0, 0.0, 10.0, 10.0],
        [0.0, 23.0, 10.0, 10.0],
        [35.0, 0.0, 10.0, 10.0],
        [35.0, 23.0, 10.0, 10.0],
        [80.0, 0.0, 10.0, 10.0],
        [80.0, 23.0, 10.0, 10.0],
        [109.0, 0.0, 10.0, 10.0],
        [109.0, 23.0, 10.0, 10.0],
        [150.0, 0.0, 10.0, 10.0],
        [150.0, 23.0, 10.0, 10.0],
        [179.0, 0.0, 10.0, 10.0],
        [179.0, 23.0, 10.0, 10.0],
    ]) {
        assert_eq!(frame(scene, color), expected, "item color {color:?}");
    }
}

#[test]
fn c11_1_root_html_sparse_column_flow_uses_document_order_and_tag_independent_identity() {
    let ordered_items = "<div class=\"item-a\"></div>\n      <div class=\"item-b\"></div>\n      <div class=\"item-c\"></div>\n      <div class=\"item-d\"></div>";
    let reversed_items = "<div class=\"item-d\"></div>\n      <div class=\"item-c\"></div>\n      <div class=\"item-b\"></div>\n      <div class=\"item-a\"></div>";
    let reversed = SOURCE.replace(ordered_items, reversed_items);
    let (session, layout) = run(&reversed);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, YELLOW), [0.0, 0.0, 70.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [0.0, 20.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [30.0, 0.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, RED), [30.0, 20.0, 30.0, 20.0]);

    let alternate_root = SOURCE
        .replace("main {", "#grid-node {")
        .replace("main > div {", "#grid-node > div {")
        .replace("<main>", "<div id=\"grid-node\">")
        .replace("</main>", "</div>")
        .replace("<body>", "<body>\n    <div style=\"display:none\"></div>");
    let (session, layout) = run(&alternate_root);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, RED), [0.0, 0.0, 30.0, 20.0]);
    assert_eq!(frame(&scene, GREEN), [0.0, 20.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [30.0, 0.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [30.0, 20.0, 70.0, 40.0]);
}

#[test]
fn c11_1_root_html_sparse_column_flow_ignores_hidden_items_without_using_extra_cells() {
    let source = SOURCE.replace(".item-a {", ".item-a { display: none;");
    let (session, layout) = run(&source);
    let scene = ready_scene(
        &session,
        &layout,
        spinon_style::ComputedCssGridAutoFlow::Column,
    );
    assert_eq!(frame(&scene, GREEN), [0.0, 0.0, 30.0, 40.0]);
    assert_eq!(frame(&scene, BLUE), [0.0, 20.0, 70.0, 20.0]);
    assert_eq!(frame(&scene, YELLOW), [30.0, 0.0, 70.0, 40.0]);
    assert_eq!(
        scene
            .boxes()
            .iter()
            .filter(|box_| box_.paint() == spinon_render::RuntimePaint::Opaque(RED))
            .count(),
        0
    );
}

#[test]
fn c11_1_root_html_sparse_column_flow_rejects_unsupported_shapes_without_a_scene() {
    let overflow = SOURCE.replace(
        "<div class=\"item-d\"></div>",
        "<div class=\"item-d\"></div>\n      <div class=\"item-e\"></div>",
    );
    let cases = [
        (
            "dense flow",
            SOURCE.replace("grid-auto-flow: column;", "grid-auto-flow: column dense;"),
        ),
        (
            "row dense flow",
            SOURCE.replace("grid-auto-flow: column;", "grid-auto-flow: row dense;"),
        ),
        ("implicit column", overflow),
        (
            "numeric authored placement",
            SOURCE.replace(".item-a {", ".item-a { grid-column-start: 1;"),
        ),
        (
            "named authored placement",
            SOURCE.replace(".item-a {", ".item-a { grid-column-start: named-line;"),
        ),
        (
            "span authored placement",
            SOURCE.replace(".item-a {", ".item-a { grid-column: span 2;"),
        ),
        (
            "non-auto implicit row sizing",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; grid-auto-rows: 10px;",
            ),
        ),
        (
            "percentage implicit columns with indefinite width",
            SOURCE
                .replace("grid-auto-flow: column;", "grid-auto-flow: column; grid-auto-columns: 25%;")
                .replace("width: 100px;", "width: auto;"),
        ),
        (
            "percentage implicit columns with min-width constraint",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; grid-auto-columns: 25%; min-width: 1px;",
            ),
        ),
        (
            "percentage implicit columns with max-width constraint",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; grid-auto-columns: 25%; max-width: 100px;",
            ),
        ),
        (
            "percentage implicit columns with aspect-ratio width constraint",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; grid-auto-columns: 25%; aspect-ratio: 5 / 3;",
            ),
        ),
        (
            "percentage implicit columns with math-backed width",
            SOURCE
                .replace("grid-auto-flow: column;", "grid-auto-flow: column; grid-auto-columns: 25%;")
                .replace("width: 100px;", "width: calc(100px + 0px);"),
        ),
        (
            "percentage explicit track",
            SOURCE.replace(
                "grid-template-columns: 30px 70px;",
                "grid-template-columns: 30px 70%;",
            ),
        ),
        (
            "fraction explicit track",
            SOURCE.replace(
                "grid-template-columns: 30px 70px;",
                "grid-template-columns: 30px 1fr;",
            ),
        ),
        (
            "empty explicit columns",
            SOURCE.replace("grid-template-columns: 30px 70px;", "grid-template-columns: none;"),
        ),
        (
            "empty explicit rows",
            SOURCE.replace("grid-template-rows: 20px 40px;", "grid-template-rows: none;"),
        ),
        (
            "named areas",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; grid-template-areas: \"a b\" \"c d\";",
            ),
        ),
        (
            "fixed column repeat",
            SOURCE.replace(
                "grid-template-columns: 30px 70px;",
                "grid-template-columns: repeat(1, 30px) 70px;",
            ),
        ),
        (
            "fixed row repeat",
            SOURCE.replace(
                "grid-template-rows: 20px 40px;",
                "grid-template-rows: 20px repeat(1, 40px);",
            ),
        ),
        (
            "named explicit line",
            SOURCE.replace(
                "grid-template-columns: 30px 70px;",
                "grid-template-columns: [left] 30px 70px;",
            ),
        ),
        (
            "right-to-left direction",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; direction: rtl;",
            ),
        ),
        (
            "vertical writing mode",
            SOURCE.replace(
                "grid-auto-flow: column;",
                "grid-auto-flow: column; writing-mode: vertical-rl;",
            ),
        ),
        (
            "order-modified sequence",
            SOURCE.replace(".item-a {", ".item-a { order: 1;"),
        ),
        (
            "direct text item",
            SOURCE.replace(
                "<div class=\"item-b\"></div>",
                "text<div class=\"item-b\"></div>",
            ),
        ),
        (
            "nested item content",
            SOURCE.replace(
                "<div class=\"item-a\"></div>",
                "<div class=\"item-a\"><span></span></div>",
            ),
        ),
        (
            "nested Grid",
            SOURCE
                .replace(".item-a {", ".item-a > div { display:grid; grid-template-columns:30px; grid-template-rows:20px; }\n      .item-a {")
                .replace(
                    "<div class=\"item-a\"></div>",
                    "<div class=\"item-a\"><div></div></div>",
                ),
        ),
        (
            "item minimum size",
            SOURCE.replace(".item-a {", ".item-a { min-width: 0px;"),
        ),
        (
            "item aspect ratio",
            SOURCE.replace(".item-a {", ".item-a { aspect-ratio: 3 / 2;"),
        ),
        (
            "intrinsic image item",
            SOURCE.replace(
                "<div class=\"item-a\"></div>",
                "<img class=\"item-a\" src=\"missing.png\">",
            ),
        ),
    ];

    for (name, source) in cases {
        let (_session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "case={name}; error={:?}; diagnostics={:?}",
            layout.error,
            layout.diagnostics
        );
        assert!(
            layout.error.is_some(),
            "case={name} should retain its error"
        );
        assert!(
            layout.completed.is_none(),
            "case={name} must not publish a partial layout or render scene"
        );
    }
}
