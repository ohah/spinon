use super::*;
use serde_json::Value;

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-percentage-gap-precomparison-2026-11-07/expected-values.json"
);
const GRID_COLOR: OpaqueCssSrgb = OpaqueCssSrgb::new(233, 237, 242);
const FIXED_COLOR: OpaqueCssSrgb = OpaqueCssSrgb::new(216, 221, 227);
const TEXT_A_COLOR: OpaqueCssSrgb = OpaqueCssSrgb::new(91, 159, 245);
const TEXT_B_COLOR: OpaqueCssSrgb = OpaqueCssSrgb::new(244, 162, 97);
const TOLERANCE: f32 = 1.0 / 32.0;

fn frame_for(scene: &spinon_render::RuntimeRenderSnapshot, color: OpaqueCssSrgb) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
        })
        .expect("expected Grid or item background should be painted");
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn expected_frame(value: &Value) -> [f32; 4] {
    value
        .as_array()
        .expect("expected item rectangle should be an array")
        .iter()
        .map(|component| {
            component
                .as_f64()
                .expect("expected item coordinate should be numeric") as f32
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("expected item rectangle should have four components")
}

fn assert_rect(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (axis, actual, expected) in [
        ("x", actual[0], expected[0]),
        ("y", actual[1], expected[1]),
        ("width", actual[2], expected[2]),
        ("height", actual[3], expected[3]),
    ] {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "{label} {axis}: expected {expected}, got {actual}"
        );
    }
}

#[test]
fn parsed_root_html_percentage_row_gap_matches_the_70_scene_alignment_matrix() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("percentage row-gap oracle should be valid JSON");
    let cases = expected["expectedCases"]
        .as_array()
        .expect("oracle should list expected cases");
    assert_eq!(cases.len(), 70);

    for case in cases {
        let percentage = case["percentage"]
            .as_f64()
            .expect("percentage should be numeric");
        let alignment = case["alignment"]
            .as_str()
            .expect("alignment should be a string");
        let source = SOURCE
            .replace("row-gap: 0%;", &format!("row-gap: {percentage}%;"))
            .replace(
                "align-content: normal;",
                &format!("align-content: {alignment};"),
            );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{percentage}% / {alignment}: layout error={:?}, diagnostics={:?}, cascade={:?}",
            layout.error,
            layout.diagnostics,
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
            "{percentage}% / {alignment}: style and layout revisions should match"
        );
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("runtime should publish the painted Root HTML scene");
        let root = frame_for(scene, GRID_COLOR);
        assert_rect(root, [root[0], root[1], 40.0, 40.0], "Grid root");
        let expected_items = case["itemRects"]
            .as_array()
            .expect("case should contain three item rectangles");
        assert_eq!(expected_items.len(), 3);
        for (actual, expected_item) in [
            (frame_for(scene, FIXED_COLOR), &expected_items[0]),
            (frame_for(scene, TEXT_A_COLOR), &expected_items[1]),
            (frame_for(scene, TEXT_B_COLOR), &expected_items[2]),
        ] {
            let mut local = actual;
            local[0] -= root[0];
            local[1] -= root[1];
            assert_rect(
                local,
                expected_frame(expected_item),
                &format!("{percentage}% / {alignment}"),
            );
        }
    }
}

#[test]
fn parsed_root_html_normal_row_gap_uses_zero_grid_gap() {
    let source = SOURCE.replace("row-gap: 0%;", "row-gap: normal;");
    assert_ne!(source, SOURCE);
    let (_session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "Grid row-gap:normal should use a zero gutter"
    );
    let completed = layout
        .completed
        .expect("normal Grid gap should produce a complete layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("normal Grid gap should produce a complete scene");
    assert_rect(
        frame_for(scene, TEXT_A_COLOR),
        [0.0, 10.0, 40.0, 15.0],
        "normal first auto row",
    );
    assert_rect(
        frame_for(scene, TEXT_B_COLOR),
        [0.0, 25.0, 40.0, 15.0],
        "normal second auto row",
    );
}

#[test]
fn parsed_root_html_negative_row_gap_keeps_the_existing_fatal_cascade_diagnostic() {
    let source = SOURCE.replace("row-gap: 0%;", "row-gap: -5%;");
    assert_ne!(source, SOURCE);
    let (session, layout) = run(&source);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    assert!(layout.completed.is_none());
    assert!(cascade.error.as_deref().is_some_and(|error| {
        error.contains("Unsupported property declaration") && error.contains("row-gap: -5%")
    }));
}

#[test]
fn parsed_root_html_unsupported_math_row_gap_rejects_atomically_and_recovers() {
    let source = SOURCE.replace(
        "      #fixed {",
        "      @media (prefers-color-scheme: dark) {\n        #grid { row-gap: calc(5px); }\n      }\n\n      #fixed {",
    );
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
    assert!(
        format!("{:?}", rejected.error).contains("row-gap")
            || rejected
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.diagnostic.message.contains("row-gap")),
        "unsupported calc() should report row-gap, got error={:?}, diagnostics={:?}",
        rejected.error,
        rejected.diagnostics
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light environment should restore the supported candidate");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "recovery should publish a complete layout: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
