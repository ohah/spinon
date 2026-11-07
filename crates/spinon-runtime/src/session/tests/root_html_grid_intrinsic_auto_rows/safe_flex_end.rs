use super::*;

const FLEX_END_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/expected.json"
);
const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);
const TOLERANCE: f32 = 1.0 / 64.0;

#[test]
fn parsed_root_html_safe_flex_end_matches_chrome_and_independent_matrix() {
    let expected: Value = serde_json::from_str(FLEX_END_EXPECTED)
        .expect("safe flex-end expected values should be valid JSON");
    let alignments = expected["alignmentValues"]
        .as_array()
        .expect("expected values should list alignments");

    for (gap_case, row_gap) in [("positive", "0px"), ("overflow", "calc(120% - 2px)")] {
        for alignment in alignments {
            let alignment = alignment.as_str().expect("alignment should be a string");
            let source = SOURCE
                .replace(
                    "align-content: normal;",
                    &format!("align-content: {alignment};"),
                )
                .replace("row-gap: 0%;", &format!("row-gap: {row_gap};"));
            assert_ne!(
                source, SOURCE,
                "{gap_case} / {alignment} should mutate the fixture"
            );
            let (session, layout) = run(&source);
            let cascade = session.ua_cascade_snapshot();
            assert_eq!(
                layout.state,
                crate::session::RuntimeLayoutState::Ready,
                "{gap_case} / {alignment}: layout error={:?}, cascade error={:?}",
                layout.error,
                cascade.error
            );
            let completed = layout
                .completed
                .as_ref()
                .expect("supported flex-end should publish a complete snapshot");
            assert_eq!(
                completed.key,
                cascade
                    .completed
                    .as_ref()
                    .expect("cascade should complete for the same revision")
                    .key
            );
            let render_scene = completed
                .render_snapshot
                .as_ref()
                .expect("default Root HTML route should publish paint boxes");
            let root = painted_frame(render_scene, OpaqueCssSrgb::new(233, 237, 242), "Grid");
            let expected_frames =
                &expected["expectedFramesByAlignmentAndGapCase"][alignment][gap_case];
            for (color, expected_frame, label) in [
                (
                    OpaqueCssSrgb::new(216, 221, 227),
                    &expected_frames[0],
                    "fixed item",
                ),
                (
                    OpaqueCssSrgb::new(91, 159, 245),
                    &expected_frames[1],
                    "text A",
                ),
                (
                    OpaqueCssSrgb::new(244, 162, 97),
                    &expected_frames[2],
                    "text B",
                ),
            ] {
                let frame = painted_frame(render_scene, color, label);
                for (axis, actual, expected_value) in [
                    (
                        "x",
                        frame[0] - root[0],
                        expected_frame["x"].as_f64().unwrap() as f32,
                    ),
                    (
                        "y",
                        frame[1] - root[1],
                        expected_frame["y"].as_f64().unwrap() as f32,
                    ),
                    (
                        "width",
                        frame[2],
                        expected_frame["width"].as_f64().unwrap() as f32,
                    ),
                    (
                        "height",
                        frame[3],
                        expected_frame["height"].as_f64().unwrap() as f32,
                    ),
                ] {
                    assert!(
                        (actual - expected_value).abs() <= TOLERANCE,
                        "{gap_case} / {alignment} / {label} {axis}: expected {expected_value}, got {actual}"
                    );
                }
            }
        }
    }
}

#[test]
fn parsed_root_html_safe_flex_end_rejects_atomically_and_recovers_same_session() {
    let source = SOURCE
        .replace(
            "      #fixed {",
            "      @media (prefers-color-scheme: dark) {\n        #grid { align-content: baseline; }\n      }\n\n      #fixed {",
        )
        .replace("row-gap: 0%;", "row-gap: calc(120% - 2px);")
        .replace("align-content: normal;", "align-content: safe end;");
    assert_ne!(source, SOURCE);
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("Root HTML with a media rule should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light environment should be set");
    let initial = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());

    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("dark environment should be set");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("align-content")
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light environment should restore the supported alignment");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "recovery should publish a complete layout: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
