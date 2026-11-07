use super::*;

const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-precomparison-2026-11-07/expected-values.json"
);
const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);
const TOLERANCE: f32 = 1.0 / 64.0;

#[test]
fn parsed_root_html_auto_row_baseline_uses_safe_start_fallback() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("baseline fallback expected values should be valid JSON");
    for (group, row_gap) in [("positiveSpace", "0px"), ("overflow", "calc(120% - 2px)")] {
        let source = SOURCE
            .replace("align-content: normal;", "align-content: baseline;")
            .replace("row-gap: 0%;", &format!("row-gap: {row_gap};"));
        assert_ne!(source, SOURCE, "{group} baseline scene must mutate source");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{group}: layout error={:?}; cascade error={:?}",
            layout.error,
            cascade.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported baseline fallback should publish a complete layout");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete for the same revision")
                .key
        );
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML route should publish paint boxes");
        let root = painted_frame(scene, OpaqueCssSrgb::new(233, 237, 242), "Grid");
        let expected_y = expected["alignmentExpectedYCssPx"]["baseline"][group]
            .as_array()
            .expect("expected group should contain baseline positions");
        for (color, expected_frame_y, label) in [
            (
                OpaqueCssSrgb::new(216, 221, 227),
                expected_y[0].as_f64().unwrap() as f32,
                "fixed item",
            ),
            (
                OpaqueCssSrgb::new(91, 159, 245),
                expected_y[1].as_f64().unwrap() as f32,
                "text A",
            ),
            (
                OpaqueCssSrgb::new(244, 162, 97),
                expected_y[2].as_f64().unwrap() as f32,
                "text B",
            ),
        ] {
            let frame = painted_frame(scene, color, label);
            assert!(
                (frame[1] - root[1] - expected_frame_y).abs() <= TOLERANCE,
                "{group} / {label}: expected relative y={expected_frame_y}, got {}",
                frame[1] - root[1]
            );
            assert_eq!(frame[2], 40.0, "{label}: width should stay fixed");
            assert_eq!(frame[3], 10.0, "{label}: row height should stay fixed");
        }
    }
}

#[test]
fn parsed_root_html_baseline_rejects_changed_item_shape_atomically_and_recovers() {
    let source = SOURCE.replace(
        "      #text-b {",
        "      @media (prefers-color-scheme: dark) { #text-b { display: none; } }\n\n      #text-b {",
    )
    .replace("align-content: normal;", "align-content: baseline;");
    assert_ne!(source, SOURCE);
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("Root HTML with a dark-mode shape control should initialize");
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
        .expect("dark environment should select the invalid two-item shape");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light environment should restore the valid three-item shape");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should publish a complete layout: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
