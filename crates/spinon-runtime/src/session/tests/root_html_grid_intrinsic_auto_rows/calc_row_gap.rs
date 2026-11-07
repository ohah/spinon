use super::*;

const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/expected-values.json"
);
const CALC_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-runtime.html"
);

fn expected_rect(value: &Value) -> [f32; 4] {
    value
        .as_array()
        .expect("expected item rectangle should be an array")
        .iter()
        .map(|component| {
            component
                .as_f64()
                .expect("expected rectangle component should be numeric") as f32
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("expected rectangle should have four components")
}

fn expected_case<'a>(
    expected: &'a Value,
    expression: &str,
    item_count: usize,
    alignment: &str,
) -> &'a Value {
    expected["groups"]
        .as_array()
        .expect("expected matrix should list expression groups")
        .iter()
        .find(|group| group["expression"] == expression)
        .and_then(|group| group["itemCountCases"].as_array())
        .and_then(|item_counts| {
            item_counts
                .iter()
                .find(|candidate| candidate["itemCount"] == item_count)
        })
        .and_then(|item_count| item_count["cases"].as_array())
        .and_then(|cases| {
            cases
                .iter()
                .find(|candidate| candidate["alignContent"] == alignment)
        })
        .expect("selected runtime scene should exist in the independent matrix")
}

#[test]
fn parsed_root_html_calc_row_gap_matches_clamp_positive_space_and_overflow_cases() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("calc row-gap expected values should be valid JSON");
    for (expression, alignment, item_count) in [
        ("calc(6.25% + 2px)", "normal", 3),
        ("calc(1px - 10%)", "end", 3),
        ("calc(120% - 2px)", "center", 3),
    ] {
        let case = expected_case(&expected, expression, item_count, alignment);
        let source = CALC_SOURCE
            .replace("row-gap: 0%;", &format!("row-gap: {expression};"))
            .replace(
                "align-content: normal;",
                &format!("align-content: {alignment};"),
            );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{expression} / {alignment} / {item_count} items: layout error={:?}, cascade error={:?}",
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
            .expect("default Root HTML route should publish the scene");
        let root = painted_frame(scene, OpaqueCssSrgb::new(233, 237, 242), "calc Grid");
        assert_frame_values(root, [root[0], root[1], 40.0, 40.0], "calc Grid");

        let expected_items = case["itemRects"]
            .as_array()
            .expect("expected scene should contain item rectangles");
        assert_eq!(expected_items.len(), item_count);
        for (color, expected_item, label) in [
            (
                OpaqueCssSrgb::new(216, 221, 227),
                &expected_items[0],
                "fixed item",
            ),
            (
                OpaqueCssSrgb::new(91, 159, 245),
                &expected_items[1],
                "first auto item",
            ),
        ]
        .into_iter()
        .chain(
            expected_items
                .get(2)
                .map(|value| (OpaqueCssSrgb::new(244, 162, 97), value, "second auto item")),
        ) {
            let mut actual = painted_frame(scene, color, label);
            actual[0] -= root[0];
            actual[1] -= root[1];
            assert_frame_values(
                actual,
                expected_rect(expected_item),
                &format!("{expression} / {alignment} / {label}"),
            );
        }
    }
}

#[test]
fn parsed_root_html_calc_row_gap_failure_is_atomic_and_recovers() {
    let source = CALC_SOURCE.replace(
        "      #fixed {",
        "      @media (prefers-color-scheme: dark) {\n        #grid { row-gap: calc(5px); }\n      }\n\n      #fixed {",
    );
    assert_ne!(source, CALC_SOURCE);
    let source = source.replace("row-gap: 0%;", "row-gap: calc(6.25% + 2px);");
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
