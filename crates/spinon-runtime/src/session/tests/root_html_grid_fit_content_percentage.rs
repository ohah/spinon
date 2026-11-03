use super::RuntimeSession;
use std::time::Duration;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-fit-content-percentage-runtime.html"
);
const OVERFLOW_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-fit-content-percentage-overflow-runtime.html"
);
const EXISTING_SOURCE: &str =
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

#[test]
fn c11_1_root_html_fit_content_percentages_reach_default_runtime() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridTrackLengthPercentage::Percentage;
    use spinon_style::ComputedCssGridTrackSize::FitContent;

    let (session, layout) = run(SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("percentage fit-content scenes should complete on the default Root HTML route");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("percentage fit-content scenes should publish a render snapshot");

    for (color, frame, name) in [
        (
            OpaqueCssSrgb::new(59, 130, 246),
            [40.0, 0.0, 10.0, 20.0],
            "20% column cap marker",
        ),
        (
            OpaqueCssSrgb::new(6, 182, 212),
            [40.0, 30.0, 10.0, 20.0],
            "120% column cap marker",
        ),
        (
            OpaqueCssSrgb::new(34, 197, 94),
            [0.0, 100.0, 20.0, 10.0],
            "20% row cap marker",
        ),
        (
            OpaqueCssSrgb::new(249, 115, 22),
            [0.0, 160.0, 40.0, 20.0],
            "first fixed-repeat item",
        ),
        (
            OpaqueCssSrgb::new(236, 72, 153),
            [40.0, 160.0, 40.0, 20.0],
            "second fixed-repeat item",
        ),
        (
            OpaqueCssSrgb::new(168, 85, 247),
            [0.0, 180.0, 20.0, 20.0],
            "zero-basis column item",
        ),
    ] {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }

    let cascade = session.ua_cascade_snapshot();
    let cascade = cascade
        .completed
        .expect("computed percentage Grid values should be available");
    assert_eq!(completed.key, cascade.key);
    let grids = cascade.roots[0]
        .styles
        .elements
        .iter()
        .filter_map(|style| style.grid_layout.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(grids.len(), 5);
    assert_eq!(grids[0].columns[0], FitContent(Percentage(0.2)));
    assert_eq!(grids[1].columns[0], FitContent(Percentage(1.2)));
    assert_eq!(grids[2].rows[0], FitContent(Percentage(0.2)));
    assert_eq!(grids[3].columns.len(), 2);
    assert!(
        grids[3]
            .columns
            .iter()
            .all(|track| *track == FitContent(Percentage(0.2)))
    );
    assert_eq!(grids[4].columns[0], FitContent(Percentage(1.0)));
}

#[test]
fn c11_1_root_html_fit_content_percentage_overflows_visible_root_border() {
    use spinon_render::{OpaqueCssSrgb, RuntimePaint};

    let (session, layout) = run(OVERFLOW_SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("border-overflow Grid should publish a complete runtime layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("border-overflow Grid should publish a render snapshot");
    let root_color = OpaqueCssSrgb::new(38, 55, 70);
    let root = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                RuntimePaint::Opaque(color) if color == root_color
            )
        })
        .expect("root Grid background should be present in the render snapshot");
    assert_frame(
        frame_for_paint(scene, root_color),
        [20.0, 40.0, 34.0, 24.0],
        "root border box",
    );
    assert_eq!(root.border().right().width_css_px(), 2.0);
    assert_eq!(
        root.border().right().paint(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(48, 164, 108))
    );
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(229, 72, 77)),
        [22.0, 42.0, 60.0, 20.0],
        "fixed child overflow",
    );
}

#[test]
fn c11_1_root_html_fit_content_percentage_out_of_scope_inputs_fail_atomically() {
    let cases = [
        (
            "indefinite row basis",
            replace_once(
                &replace_once(
                    EXISTING_SOURCE,
                    "#row {\n        width: 20px;\n        height: 20px;",
                    "#row {\n        width: 20px;\n        height: auto;",
                ),
                "grid-template-rows: fit-content(20px);",
                "grid-template-rows: fit-content(20%);",
            ),
            "height",
        ),
        (
            "named-area percentage track",
            EXISTING_SOURCE.replace(
                "grid-template-columns: 20px fit-content(40px);",
                "grid-template-columns: 20px fit-content(20%);",
            ),
            "grid-template-areas",
        ),
        (
            "px fit-content border remains outside percentage profile",
            OVERFLOW_SOURCE.replace("fit-content(200%)", "fit-content(20px)"),
            "border-top-width",
        ),
    ];

    for (name, source, expected_property) in cases {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope case={name} must fail closed: {layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        assert!(
            layout.completed.is_none(),
            "{name} must not publish a partial scene"
        );
        let error = layout
            .error
            .as_ref()
            .expect("rejected input should report an error");
        assert!(
            error
                .property
                .as_deref()
                .is_some_and(|property| property.contains(expected_property)),
            "{name}: {error:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
    }
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}
