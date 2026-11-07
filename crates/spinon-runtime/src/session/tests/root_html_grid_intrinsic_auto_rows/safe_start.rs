use super::*;

const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-safe-start-precomparison-2026-11-07/expected-values.json"
);
const FLEX_START_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/expected.json"
);
const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);
const TOLERANCE: f32 = 1.0 / 64.0;

#[test]
fn parsed_root_html_safe_start_matches_chrome_and_independent_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("safe-start expected values should be valid JSON");
    for scene in expected["scenes"].as_array().unwrap() {
        let alignment = scene["computed_align_content"].as_str().unwrap();
        let row_gap = scene["row_gap"].as_str().unwrap();
        let source = SOURCE
            .replace(
                "align-content: normal;",
                &format!("align-content: {alignment};"),
            )
            .replace("row-gap: 0%;", &format!("row-gap: {row_gap};"));
        assert_ne!(source, SOURCE, "{} should mutate the fixture", scene["id"]);
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{}: layout error={:?}, cascade error={:?}",
            scene["id"],
            layout.error,
            cascade.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported alignment should publish a complete snapshot");
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
        let expected_y = scene["item_y_px"].as_array().unwrap();
        for (color, expected_y, label) in [
            (
                OpaqueCssSrgb::new(216, 221, 227),
                expected_y[0].as_f64().unwrap(),
                "fixed item",
            ),
            (
                OpaqueCssSrgb::new(91, 159, 245),
                expected_y[1].as_f64().unwrap(),
                "text A",
            ),
            (
                OpaqueCssSrgb::new(244, 162, 97),
                expected_y[2].as_f64().unwrap(),
                "text B",
            ),
        ] {
            let frame = painted_frame(render_scene, color, label);
            let relative_y = frame[1] - root[1];
            assert!(
                (relative_y - expected_y as f32).abs() <= TOLERANCE,
                "{} / {label}: expected y={expected_y}, got {relative_y}",
                scene["id"]
            );
            assert_eq!(frame[2], 40.0, "{label}: width should remain fixed");
            assert_eq!(frame[3], 10.0, "{label}: height should remain fixed");
        }
    }
}

#[test]
fn parsed_root_html_safe_flex_start_matches_chrome_and_independent_matrix() {
    let expected: Value = serde_json::from_str(FLEX_START_EXPECTED)
        .expect("safe flex-start expected values should be valid JSON");
    let alignments = expected["alignmentValues"]
        .as_array()
        .expect("expected values should list alignments");
    let cases = [("positive", "0px"), ("overflow", "calc(120% - 2px)")];

    for (gap_case, row_gap) in cases {
        let expected_frames = expected["expectedFramesByGapCase"][gap_case]
            .as_array()
            .expect("expected values should list item frames");
        for alignment_value in alignments {
            let alignment_value = alignment_value
                .as_str()
                .expect("alignment should be a string");
            let source = SOURCE
                .replace(
                    "align-content: normal;",
                    &format!("align-content: {alignment_value};"),
                )
                .replace("row-gap: 0%;", &format!("row-gap: {row_gap};"));
            assert_ne!(
                source, SOURCE,
                "{gap_case} / {alignment_value} should mutate the fixture"
            );
            let (session, layout) = run(&source);
            let cascade = session.ua_cascade_snapshot();
            assert_eq!(
                layout.state,
                crate::session::RuntimeLayoutState::Ready,
                "{gap_case} / {alignment_value}: layout error={:?}, cascade error={:?}",
                layout.error,
                cascade.error
            );
            let completed = layout
                .completed
                .as_ref()
                .expect("supported flex-start alignment should publish a complete snapshot");
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
                let expected_x = expected_frame["x"].as_f64().unwrap() as f32;
                let expected_y = expected_frame["y"].as_f64().unwrap() as f32;
                assert!(
                    ((frame[0] - root[0]) - expected_x).abs() <= TOLERANCE,
                    "{gap_case} / {alignment_value} / {label}: expected x={expected_x}, got {}",
                    frame[0] - root[0]
                );
                assert!(
                    ((frame[1] - root[1]) - expected_y).abs() <= TOLERANCE,
                    "{gap_case} / {alignment_value} / {label}: expected y={expected_y}, got {}",
                    frame[1] - root[1]
                );
                assert_eq!(frame[2], expected_frame["width"].as_f64().unwrap() as f32);
                assert_eq!(frame[3], expected_frame["height"].as_f64().unwrap() as f32);
            }
        }
    }
}

#[test]
fn parsed_root_html_safe_start_rejects_atomically_and_recovers_same_session() {
    let source = SOURCE
        .replace(
            "      #fixed {",
            "      @media (prefers-color-scheme: dark) {\n        #grid { align-content: baseline; }\n      }\n\n      #fixed {",
        )
        .replace("align-content: normal;", "align-content: safe start;");
    assert_ne!(source, SOURCE);
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("Root HTML with media rule should initialize");
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
        .expect("light environment should restore supported alignment");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "recovery should publish complete layout: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
