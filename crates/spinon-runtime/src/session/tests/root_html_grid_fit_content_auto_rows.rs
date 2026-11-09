use super::RuntimeSession;
use std::time::Duration;

#[path = "root_html_grid_fit_content_auto_rows/fit_content_positions.rs"]
mod fit_content_positions;
#[path = "root_html_grid_fit_content_auto_rows/later_fit_content_row_span.rs"]
mod later_fit_content_row_span;
#[path = "root_html_grid_fit_content_auto_rows/repeated_text_span.rs"]
mod repeated_text_span;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-runtime.html"
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

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

#[test]
fn parsed_root_html_fit_content_auto_rows_match_chrome_scenes_and_px_normalizations() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::{
        ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage::LengthPx,
        ComputedCssGridTrackSize::FitContent,
    };

    for (name, source, cap, row_height, probe_y) in [
        (
            "zero cap",
            replace_once(SOURCE, "fit-content(24px)", "fit-content(0px)"),
            0.0,
            0.0,
            10.0,
        ),
        ("cap below max-content", SOURCE.to_owned(), 24.0, 24.0, 34.0),
        (
            "cap above max-content",
            replace_once(SOURCE, "fit-content(24px)", "fit-content(40px)"),
            40.0,
            34.0,
            44.0,
        ),
        (
            "minimum contribution above cap",
            replace_once(SOURCE, "min-height: 0;", "min-height: 28px;"),
            24.0,
            28.0,
            38.0,
        ),
        (
            "constant absolute calc normalizes to px",
            replace_once(
                SOURCE,
                "fit-content(24px)",
                "fit-content(calc(12px + 12px))",
            ),
            24.0,
            24.0,
            34.0,
        ),
        (
            "absolute unit normalizes to px",
            replace_once(SOURCE, "fit-content(24px)", "fit-content(0.25in)"),
            24.0,
            24.0,
            34.0,
        ),
    ] {
        let (session, layout) = run(&source);
        let cascade_error = session.ua_cascade_snapshot().error;
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{name}: layout error={:?}; cascade error={cascade_error:?}",
            layout.error,
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported fit-content rows should publish a complete layout");
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("supported fit-content rows should publish a render snapshot");
        for (color, frame, label) in [
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
                        .zip(frame)
                        .all(|(actual, expected)| (actual - expected).abs() <= 0.02)
                    }),
                    "{label}: zero-height Grid item should retain frame {frame:?}"
                );
            } else {
                assert_frame(frame_for_paint(scene, color), frame, label);
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
            ComputedCssGridAutoRows::TrackList(vec![FitContent(LengthPx(cap))]),
            "{name}: the computed px cap should reach the default runtime"
        );
        if name.contains("normalizes to px") {
            let serialized = cascade.roots[0]
                .styles
                .elements
                .iter()
                .find(|style| style.grid_layout.is_some())
                .and_then(|style| style.properties.get("grid-auto-rows"))
                .map(String::as_str);
            assert_eq!(
                serialized,
                Some("fit-content(24px)"),
                "{name}: computed serialization must expose the same px cap as the literal input"
            );
        }
    }
}

#[test]
fn parsed_root_html_fit_content_auto_rows_reject_atomically_and_recover_same_session() {
    use spinon_render::OpaqueCssSrgb;

    let (session, initial) = run(SOURCE);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());
    let expected_grid_node_id = session
        .ua_cascade_snapshot()
        .completed
        .expect("the supported initial revision should have computed styles")
        .roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("the fixture has one Grid owner")
        .node_id
        .get();

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
    assert_eq!(
        rejected.error.as_ref().and_then(|failure| failure.node_id),
        Some(expected_grid_node_id),
        "the unsupported value should identify the owning Grid node"
    );
    assert!(
        rejected.completed.is_none(),
        "a percentage-bearing calc cap must not publish a partial render scene"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment should restore the supported px cap");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should succeed: {:?}",
        recovered.error
    );
    let scene = recovered
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("recovery should publish the complete render scene");
    assert_frame(
        frame_for_paint(scene, OpaqueCssSrgb::new(249, 115, 22)),
        [0.0, 34.0, 40.0, 6.0],
        "recovered following probe item",
    );
    assert_fit_content_auto_rows_reject_mixed_math_flow_and_placement_shapes();
}

fn assert_fit_content_auto_rows_reject_mixed_math_flow_and_placement_shapes() {
    let cases = [
        (
            "mixed implicit row list",
            replace_once(
                SOURCE,
                "grid-auto-rows: fit-content(24px);",
                "grid-auto-rows: fit-content(24px) min-content;",
            ),
        ),
        (
            "calc cap with percentage term",
            replace_once(
                SOURCE,
                "grid-auto-rows: fit-content(24px);",
                "grid-auto-rows: fit-content(calc(12px + 12%));",
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
            "{name} must not publish a partial scene"
        );
    }
}

mod repeated_text;
mod track_lists;
