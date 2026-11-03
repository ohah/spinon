use super::RuntimeSession;
use std::time::Duration;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-fit-content-runtime.html");

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
        .unwrap_or_else(|| panic!("expected painted Root HTML box {expected:?}"));
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

fn replace_fixture_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

#[test]
fn c11_1_root_html_fit_content_tracks_reach_default_runtime_and_keep_overflow_visible() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::{
        ComputedCssGridTrackLengthPercentage::LengthPx as FitContentPx,
        ComputedCssGridTrackSize::{FitContent, LengthPx},
    };

    let (session, layout) = run(SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
    let completed = layout
        .completed
        .expect("fit-content scenes should complete through the default Root HTML route");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish the three scenes");
    for (color, frame, name) in [
        (
            OpaqueCssSrgb::new(16, 24, 32),
            [0.0, 0.0, 30.0, 20.0],
            "direct-track root",
        ),
        (
            OpaqueCssSrgb::new(31, 41, 55),
            [0.0, 20.0, 30.0, 20.0],
            "named-area root",
        ),
        (
            OpaqueCssSrgb::new(17, 24, 39),
            [0.0, 40.0, 20.0, 20.0],
            "row root",
        ),
    ] {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }
    for (color, frame, name) in [
        (
            OpaqueCssSrgb::new(229, 72, 77),
            [0.0, 0.0, 40.0, 20.0],
            "direct fit-content item",
        ),
        (
            OpaqueCssSrgb::new(59, 130, 246),
            [40.0, 0.0, 8.0, 20.0],
            "direct overflow marker",
        ),
        (
            OpaqueCssSrgb::new(22, 163, 74),
            [0.0, 20.0, 20.0, 20.0],
            "named-area fixed item",
        ),
        (
            OpaqueCssSrgb::new(48, 164, 108),
            [20.0, 20.0, 40.0, 20.0],
            "named-area fit-content item",
        ),
        (
            OpaqueCssSrgb::new(240, 180, 41),
            [0.0, 40.0, 20.0, 40.0],
            "fit-content row item",
        ),
    ] {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }

    let cascade_completed = cascade
        .completed
        .expect("computed Grid values should be available for the same scene");
    assert_eq!(completed.key, cascade_completed.key);
    let grids = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .filter_map(|style| style.grid_layout.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(
        grids.len(),
        3,
        "fixture should have three parsed Grid roots"
    );
    assert_eq!(
        grids[0].columns,
        [FitContent(FitContentPx(40.0)), LengthPx(10.0)]
    );
    assert_eq!(grids[0].rows, [LengthPx(20.0)]);
    assert_eq!(
        grids[1].columns,
        [LengthPx(20.0), FitContent(FitContentPx(40.0))]
    );
    assert_eq!(grids[1].rows, [LengthPx(20.0)]);
    assert_eq!(
        grids[1]
            .template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((1, 2))
    );
    assert_eq!(grids[2].columns, [LengthPx(20.0)]);
    assert_eq!(grids[2].rows, [FitContent(FitContentPx(20.0))]);
}

#[test]
fn c11_1_root_html_fit_content_named_areas_ignore_names_ids_and_source_order() {
    let source = SOURCE
        .replace("#named-area", "#renamed-area-grid")
        .replace("id=\"named-area\"", "id=\"renamed-area-grid\"")
        .replace("#area-fixed", "#renamed-leading")
        .replace("id=\"area-fixed\"", "id=\"renamed-leading\"")
        .replace("#area-fit", "#renamed-trailing")
        .replace("id=\"area-fit\"", "id=\"renamed-trailing\"")
        .replace(
            "grid-template-areas: \"fixed fit\"",
            "grid-template-areas: \"leading trailing\"",
        )
        .replace("grid-area: fixed;", "grid-area: leading;")
        .replace("grid-area: fit;", "grid-area: trailing;");
    let source = replace_fixture_once(
        &source,
        "<div id=\"renamed-leading\"></div>\n      <div id=\"renamed-trailing\"></div>",
        "<div id=\"renamed-trailing\"></div>\n      <div id=\"renamed-leading\"></div>",
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "renamed areas/selectors and reversed source order should remain admitted: {layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("renamed named areas should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("renamed named areas should publish a scene");
    assert_frame(
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(22, 163, 74)),
        [0.0, 20.0, 20.0, 20.0],
        "renamed leading area",
    );
    assert_frame(
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(48, 164, 108)),
        [20.0, 20.0, 40.0, 20.0],
        "renamed trailing area",
    );
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade.completed.unwrap();
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|style| {
            style
                .grid_layout
                .as_ref()
                .filter(|grid| grid.template_areas.is_some())
        })
        .expect("renamed fit-content area should retain typed tracks");
    let mut area_names = grid
        .template_areas
        .as_ref()
        .unwrap()
        .areas
        .iter()
        .map(|area| area.name.as_str())
        .collect::<Vec<_>>();
    area_names.sort_unstable();
    assert_eq!(area_names, ["leading", "trailing"]);
}

#[test]
fn c11_1_root_html_fit_content_named_areas_reuse_definite_percentage_gap_validation() {
    let source = replace_fixture_once(
        SOURCE,
        "#named-area {\n        width: 30px;\n        height: 20px;",
        "#named-area {\n        width: 30px;\n        height: 20px;\n        column-gap: 5%;\n        row-gap: 5%;",
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "finite percentage gaps against definite named-area axes should pass the shared gap gate: {layout:?}; cascade_error={:?}",
        session.ua_cascade_snapshot().error
    );
}

#[test]
fn c11_1_root_html_fit_content_named_areas_reject_out_of_scope_shapes_atomically() {
    let fit_tracks = "grid-template-columns: 20px fit-content(40px);";
    let cases = [
        (
            "indefinite width",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;",
                "#named-area {\n        width: auto;",
            ),
            "width",
        ),
        (
            "zero width",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;",
                "#named-area {\n        width: 0px;",
            ),
            "width",
        ),
        (
            "indefinite height",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;\n        height: 20px;",
                "#named-area {\n        width: 30px;\n        height: auto;",
            ),
            "height",
        ),
        (
            "zero height",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;\n        height: 20px;",
                "#named-area {\n        width: 30px;\n        height: 0px;",
            ),
            "height",
        ),
        (
            "minimum width override",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;",
                "#named-area {\n        width: 30px;\n        min-width: 0;",
            ),
            "min-width",
        ),
        (
            "maximum width override",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;",
                "#named-area {\n        width: 30px;\n        max-width: 31px;",
            ),
            "max-width",
        ),
        (
            "minimum height override",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;\n        height: 20px;",
                "#named-area {\n        width: 30px;\n        height: 20px;\n        min-height: 0;",
            ),
            "min-height",
        ),
        (
            "maximum height override",
            replace_fixture_once(
                SOURCE,
                "#named-area {\n        width: 30px;\n        height: 20px;",
                "#named-area {\n        width: 30px;\n        height: 20px;\n        max-height: 21px;",
            ),
            "max-height",
        ),
        (
            "percentage fit-content cap",
            SOURCE.replace(fit_tracks, "grid-template-columns: 20px fit-content(40%);"),
            "grid-template-areas",
        ),
        (
            "dimension-mismatched area map",
            replace_fixture_once(
                SOURCE,
                "grid-template-rows: 20px;\n        grid-template-areas: \"fixed fit\";",
                "grid-template-rows: 20px 20px;\n        grid-template-areas: \"fixed fit\";",
            ),
            "grid-template-areas",
        ),
        (
            "unsupported minmax area track",
            SOURCE.replace(
                fit_tracks,
                "grid-template-columns: 20px minmax(10px, 40px);",
            ),
            "grid-template-areas",
        ),
        (
            "duplicate area placement",
            replace_fixture_once(SOURCE, "grid-area: fit;", "grid-area: fixed;"),
            "grid-area",
        ),
        (
            "missing area item",
            replace_fixture_once(SOURCE, "      <div id=\"area-fit\"></div>\n", ""),
            "grid-template-areas",
        ),
        (
            "non-empty area item",
            replace_fixture_once(
                SOURCE,
                "<div id=\"area-fit\"></div>",
                "<div id=\"area-fit\"><span></span></div>",
            ),
            "grid-item",
        ),
        (
            "anonymous text item",
            replace_fixture_once(
                SOURCE,
                "    <main id=\"named-area\">\n      <div id=\"area-fixed\"></div>",
                "    <main id=\"named-area\">\n      label\n      <div id=\"area-fixed\"></div>",
            ),
            "visible element siblings of anonymous Grid text are outside this slice",
        ),
        (
            "hidden area item",
            replace_fixture_once(
                SOURCE,
                "#area-fit {\n        grid-area: fit;",
                "#area-fit {\n        display: none;\n        grid-area: fit;",
            ),
            "display",
        ),
        (
            "non-div area item",
            replace_fixture_once(
                SOURCE,
                "<div id=\"area-fixed\"></div>",
                "<section id=\"area-fixed\"></section>",
            ),
            "grid-item",
        ),
    ];

    for (name, source, expected_property) in cases {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope named-area case={name} must fail closed: {layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        assert!(
            layout.completed.is_none(),
            "{name} must not publish a partial scene"
        );
        let error = layout
            .error
            .as_ref()
            .expect("failed named-area case should report an error");
        if let Some(property) = error.property.as_deref() {
            assert_eq!(
                property,
                expected_property,
                "{name}: {error:?}; cascade_error={:?}",
                session.ua_cascade_snapshot().error
            );
        } else {
            assert_eq!(
                error.code,
                "cascade_failed",
                "{name}: property-less failure must be a rejected cascade; cascade_error={:?}",
                session.ua_cascade_snapshot().error
            );
        }
    }
}
