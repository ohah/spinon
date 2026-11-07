use super::RuntimeSession;
use std::time::Duration;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-percentage-runtime.html"
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

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
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

#[test]
fn parsed_root_html_fit_content_percentage_auto_rows_match_definite_content_box_basis() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::{
        ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage::Percentage,
        ComputedCssGridTrackSize::FitContent,
    };

    for (name, authored_percent, row_height, probe_y, minimum_floor) in [
        ("zero cap", 0.0, 0.0, 10.0, false),
        ("twenty percent", 20.0, 20.0, 30.0, false),
        ("twenty-four percent", 24.0, 24.0, 34.0, false),
        ("max-content boundary", 34.0, 34.0, 44.0, false),
        ("above max-content", 40.0, 34.0, 44.0, false),
        ("above one hundred percent", 120.0, 34.0, 44.0, false),
        ("minimum contribution floor", 20.0, 28.0, 38.0, true),
    ] {
        let cap = format!("grid-auto-rows: fit-content({authored_percent}%);");
        let mut source = replace_once(SOURCE, "grid-auto-rows: fit-content(24%);", &cap);
        if minimum_floor {
            source = replace_once(&source, "min-height: 0;", "min-height: 28px;");
        }
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{name}: layout error={:?}; cascade error={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported percentage rows should publish a complete layout");
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("supported percentage rows should publish a render snapshot");
        for (color, expected, label) in [
            (
                OpaqueCssSrgb::new(16, 32, 48),
                [0.0, 0.0, 40.0, 100.0],
                "grid",
            ),
            (
                OpaqueCssSrgb::new(229, 72, 77),
                [0.0, 0.0, 40.0, 10.0],
                "explicit row item",
            ),
            (
                OpaqueCssSrgb::new(34, 197, 94),
                [0.0, 10.0, 40.0, row_height],
                "fit-content implicit row item",
            ),
            (
                OpaqueCssSrgb::new(59, 130, 246),
                [0.0, 10.0, 40.0, 34.0],
                "visible nested overflow",
            ),
            (
                OpaqueCssSrgb::new(249, 115, 22),
                [0.0, probe_y, 40.0, 6.0],
                "following probe item",
            ),
        ] {
            if label == "fit-content implicit row item" && row_height == 0.0 {
                assert!(
                    completed.frames.iter().any(|item_frame| {
                        [
                            item_frame.x,
                            item_frame.y,
                            item_frame.width,
                            item_frame.height,
                        ]
                        .into_iter()
                        .zip(expected)
                        .all(|(actual, expected)| (actual - expected).abs() <= 0.02)
                    }),
                    "{label}: zero-height Grid item should retain frame {expected:?}"
                );
            } else {
                assert_frame(frame_for_paint(scene, color), expected, label);
            }
        }

        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("the same runtime revision should publish computed style");
        assert_eq!(completed.key, cascade.key);
        let grids = cascade.roots[0]
            .styles
            .elements
            .iter()
            .filter_map(|style| style.grid_layout.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(grids.len(), 1, "{name}: only the target Grid is visible");
        assert_eq!(
            grids[0].auto_rows,
            ComputedCssGridAutoRows::TrackList(vec![FitContent(Percentage(
                authored_percent / 100.0,
            ))]),
            "{name}: the computed percentage cap should reach the default runtime"
        );
    }
}

#[test]
fn parsed_root_html_fit_content_percentage_auto_rows_reject_unsupported_shapes_atomically() {
    let cases = [
        (
            "mixed implicit row list",
            replace_once(
                SOURCE,
                "grid-auto-rows: fit-content(24%);",
                "grid-auto-rows: fit-content(24%) 10px;",
            ),
        ),
        (
            "percentage-bearing calc",
            replace_once(
                SOURCE,
                "grid-auto-rows: fit-content(24%);",
                "grid-auto-rows: fit-content(calc(12px + 12%));",
            ),
        ),
        (
            "indefinite grid block size",
            replace_once(SOURCE, "height: 100px;", "height: auto;"),
        ),
        (
            "nonzero row gap",
            replace_once(SOURCE, "row-gap: 0;", "row-gap: 1px;"),
        ),
        (
            "intrinsic explicit column",
            replace_once(
                SOURCE,
                "grid-template-columns: 40px;",
                "grid-template-columns: min-content;",
            ),
        ),
        (
            "template areas",
            replace_once(
                SOURCE,
                "grid-template-rows: 10px;",
                "grid-template-rows: 10px;\n    grid-template-areas: 'main';",
            ),
        ),
        (
            "column auto-flow",
            replace_once(SOURCE, "grid-auto-flow: row;", "grid-auto-flow: column;"),
        ),
        (
            "explicitly placed probe",
            replace_once(
                SOURCE,
                ".probe {\n    display: block;",
                ".probe {\n    grid-row-start: 3;\n    display: block;",
            ),
        ),
    ];

    for (name, source) in cases {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name} must fail closed: layout error={:?}; cascade error={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        assert!(
            layout.completed.is_none(),
            "{name} must not publish a partial layout or render scene"
        );
    }
}

#[test]
fn parsed_root_html_fit_content_percentage_auto_rows_recover_after_dark_calc_rejection() {
    let (session, initial) = run(SOURCE);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment should be applied");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(
        rejected.completed.is_none(),
        "a percentage-bearing calc cap must not publish partial output"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment should restore the supported percentage cap");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should succeed: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
