use super::*;

const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/expected-values.json"
);
const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);
const TOLERANCE: f32 = 1.0 / 64.0;

fn expected_y(expected: &Value, group: &str, alignment: &str) -> Vec<f32> {
    expected[group]["expectedYRelativeCssPx"][alignment]
        .as_array()
        .expect("expected group should list item y offsets")
        .iter()
        .map(|value| value.as_f64().expect("y offset should be numeric") as f32)
        .collect()
}

#[test]
fn parsed_root_html_safe_end_matches_chrome_and_independent_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("safe-end expected values should be valid JSON");
    for (group, alignment, row_gap) in [
        ("positiveSpace", "safe end", "0px"),
        ("positiveSpace", "unsafe end", "0px"),
        ("positiveSpace", "end", "0px"),
        ("overflow", "safe end", "calc(120% - 2px)"),
        ("overflow", "unsafe end", "calc(120% - 2px)"),
        ("overflow", "end", "calc(120% - 2px)"),
        ("overflow", "safe center", "calc(120% - 2px)"),
        ("overflow", "unsafe center", "calc(120% - 2px)"),
    ] {
        let source = SOURCE
            .replace(
                "align-content: normal;",
                &format!("align-content: {alignment};"),
            )
            .replace("row-gap: 0%;", &format!("row-gap: {row_gap};"));
        assert_ne!(
            source, SOURCE,
            "the selected scene should mutate the fixture"
        );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{group} / {alignment}: layout error={:?}, cascade error={:?}",
            layout.error,
            cascade.error
        );
        let completed = layout.completed.as_ref().expect("layout should complete");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete")
                .key,
            "style and layout revisions should match"
        );
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML runtime should publish paint boxes");
        let root = painted_frame(scene, OpaqueCssSrgb::new(233, 237, 242), "safe-end Grid");
        assert_frame_values(root, [root[0], root[1], 40.0, 40.0], "safe-end Grid");
        let expected_y = expected_y(&expected, group, alignment);
        for (color, expected_y, label) in [
            (
                OpaqueCssSrgb::new(216, 221, 227),
                expected_y[0],
                "fixed item",
            ),
            (OpaqueCssSrgb::new(91, 159, 245), expected_y[1], "text A"),
            (OpaqueCssSrgb::new(244, 162, 97), expected_y[2], "text B"),
        ] {
            let frame = painted_frame(scene, color, label);
            let relative_y = frame[1] - root[1];
            assert!(
                (relative_y - expected_y).abs() <= TOLERANCE,
                "{group} / {alignment} / {label}: expected relative y={expected_y}, got {relative_y}"
            );
            assert_eq!(
                frame[3], 10.0,
                "{label}: auto row sizing should remain 10px"
            );
        }
    }
}

#[test]
fn parsed_root_html_safe_end_rejects_other_safety_values_and_recovers() {
    let source = SOURCE.replace(
        "      #fixed {",
        "      @media (prefers-color-scheme: dark) {\n        #grid { align-content: safe flex-start; }\n      }\n\n      #fixed {",
    );
    assert_ne!(source, SOURCE);
    let source = source
        .replace("row-gap: 0%;", "row-gap: calc(120% - 2px);")
        .replace("align-content: normal;", "align-content: safe end;");
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
