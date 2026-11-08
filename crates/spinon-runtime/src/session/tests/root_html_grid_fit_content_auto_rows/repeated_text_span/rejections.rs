use super::*;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

fn first_scene_source() -> String {
    let scene = serde_json::from_str::<Value>(
        MATRIX
            .lines()
            .nth(1)
            .expect("matrix should contain a first scene"),
    )
    .expect("first matrix scene should be valid JSON");
    source_for(&scene)
}

fn assert_rejected(name: &str, source: &str) {
    let (session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail closed: layout={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let failure = layout
        .error
        .as_ref()
        .unwrap_or_else(|| panic!("{name} should identify the failure"));
    assert!(
        failure.node_id.is_some(),
        "{name}: node id should be reported: {failure:?}"
    );
    assert!(
        failure.property.is_some(),
        "{name}: property should be reported"
    );
    assert!(
        layout.completed.is_none(),
        "{name} must not publish partial frames or paint"
    );
}

fn assert_cascade_failed_atomically(name: &str, source: &str) {
    let (_, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail before layout publication: {:?}",
        layout.error
    );
    assert_eq!(
        layout.error.as_ref().map(|failure| failure.code),
        Some("cascade_failed"),
        "{name} should be rejected by the cascade boundary"
    );
    assert!(
        layout.completed.is_none(),
        "{name} must not publish frames or paint"
    );
}

#[test]
fn parsed_root_html_repeated_text_row_span_nearby_shapes_reject_atomically() {
    let source = first_scene_source();
    let b_item = "<div class=\"item\" data-item=\"B\" style=\"grid-row:3 / span 2\"><div class=\"payload\">A A A A A A A A</div></div>";
    let b_with_duplicate = format!(
        "{b_item}<div class=\"item\" data-item=\"D\" style=\"grid-row:2 / span 2\"><div class=\"payload\">A</div></div>"
    );
    let cases = [
        (
            "wrong row start",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:4 / span 2"),
        ),
        (
            "implicit row start line",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:auto / span 2"),
        ),
        (
            "negative row line",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:-1 / span 2"),
        ),
        (
            "line end instead of span end",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:2 / 4"),
        ),
        (
            "reversed row line pair",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:span 2 / 2"),
        ),
        (
            "span one",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:2 / span 1"),
        ),
        (
            "span four",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:2 / span 4"),
        ),
        (
            "span five",
            replace_once(&source, "grid-row:2 / span 2", "grid-row:2 / span 5"),
        ),
        (
            "span count differs from topology",
            replace_once(&source, "grid-row:3 / span 2", "grid-row:3 / span 3"),
        ),
        (
            "implicit column from auto placement",
            replace_once(&source, "grid-column: 1 / 2;", "grid-column: auto;"),
        ),
        (
            "wrong explicit column end",
            replace_once(&source, "grid-column: 1 / 2;", "grid-column: 1 / 3;"),
        ),
        (
            "negative column line",
            replace_once(&source, "grid-column: 1 / 2;", "grid-column: -1 / 2;"),
        ),
        (
            "duplicate span start",
            replace_once(&source, "grid-row:3 / span 2", "grid-row:2 / span 2"),
        ),
        ("missing direct item", replace_once(&source, b_item, "")),
        (
            "duplicate direct item",
            replace_once(&source, b_item, &b_with_duplicate),
        ),
        (
            "missing text payload",
            replace_once(&source, "<div class=\"payload\">A A A A A A</div>", ""),
        ),
        (
            "nested payload wrapper",
            replace_once(
                &source,
                "<div class=\"payload\">A A A A A A</div>",
                "<span><div class=\"payload\">A A A A A A</div></span>",
            ),
        ),
        (
            "inline markup inside text payload",
            replace_once(
                &source,
                "<div class=\"payload\">A A A A A A</div>",
                "<div class=\"payload\"><span>A A A A A A</span></div>",
            ),
        ),
        (
            "lowercase text",
            replace_once(&source, "A A A A A A</div>", "a A A A A A</div>"),
        ),
        (
            "unsupported font weight",
            replace_once(&source, "font-weight: 400;", "font-weight: 500;"),
        ),
        (
            "wrong track multiplicity",
            replace_once(
                &source,
                "fit-content(20px) min-content min-content max-content max-content auto",
                "fit-content(20px) min-content min-content min-content max-content auto",
            ),
        ),
        (
            "extra explicit row",
            replace_once(
                &source,
                "grid-template-rows: 10px;",
                "grid-template-rows: 10px 10px;",
            ),
        ),
        (
            "extra explicit column",
            replace_once(
                &source,
                "grid-template-columns: 40px;",
                "grid-template-columns: 40px 1px;",
            ),
        ),
        (
            "percentage-bearing calc cap",
            replace_once(
                &source,
                "fit-content(20px)",
                "fit-content(calc(12px + 12%))",
            ),
        ),
        (
            "unsupported content alignment",
            replace_once(&source, "align-content: start;", "align-content: center;"),
        ),
        (
            "column auto flow",
            replace_once(&source, "grid-auto-flow: row;", "grid-auto-flow: column;"),
        ),
        (
            "dense row flow",
            replace_once(
                &source,
                "grid-auto-flow: row;",
                "grid-auto-flow: row dense;",
            ),
        ),
        (
            "nonzero row gap",
            replace_once(&source, "row-gap: 0;", "row-gap: 1px;"),
        ),
        (
            "clipped grid overflow",
            replace_once(&source, "overflow: visible;", "overflow: hidden;"),
        ),
        (
            "hidden spanning child",
            replace_once(
                &source,
                "</style>",
                ".item[data-item=\"B\"] { display: none; }\n</style>",
            ),
        ),
        (
            "second visible grid owner",
            replace_once(&source, "</body>", "<div class=\"grid\"></div></body>"),
        ),
        (
            "changed font size",
            replace_once(&source, "font-size: 10px;", "font-size: 11px;"),
        ),
        (
            "nowrap text",
            replace_once(&source, "white-space: normal;", "white-space: nowrap;"),
        ),
        (
            "nonzero letter spacing",
            replace_once(&source, "letter-spacing: 0;", "letter-spacing: 1px;"),
        ),
        (
            "right-to-left text direction",
            replace_once(&source, "direction: ltr;", "direction: rtl;"),
        ),
    ];

    assert_eq!(cases.len(), 35);
    for (name, source) in cases {
        assert_rejected(name, &source);
    }
}

#[test]
fn parsed_root_html_repeated_text_vertical_writing_mode_fails_before_layout_publication() {
    let source = replace_once(
        &first_scene_source(),
        "font-family: monospace;",
        "font-family: monospace; writing-mode: vertical-rl;",
    );
    assert_cascade_failed_atomically("vertical writing mode", &source);
}

#[test]
fn parsed_root_html_repeated_text_row_span_recovers_same_session_after_rejection() {
    let source = replace_once(
        &first_scene_source(),
        "data-item=\"A\" style=\"grid-row:2 / span 2\"",
        "data-item=\"A\" id=\"span-a\"",
    );
    let source = replace_once(
        &source,
        "</style>",
        "#span-a { grid-row: 2 / span 2; }\n@media (prefers-color-scheme: dark) { #span-a { grid-row: 4 / span 2; } }\n</style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "the light scheme should retain the supported span topology: {:?}",
        initial.error
    );
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("initial scene should publish");
    let initial_key = initial_completed.key;
    let initial_render = initial_completed
        .render_snapshot
        .as_ref()
        .expect("initial scene should publish paint");
    let initial_a = frame_for_paint(initial_render, A_ITEM);

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark scheme should apply the unsupported row placement");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(
        rejected.completed.is_none(),
        "rejected revision must be atomic"
    );
    assert!(
        rejected
            .error
            .as_ref()
            .is_some_and(|failure| { failure.node_id.is_some() && failure.property.is_some() })
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light scheme should restore the supported span topology");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should succeed: {:?}",
        recovered.error
    );
    let recovered_completed = recovered
        .completed
        .as_ref()
        .expect("recovery should publish a complete scene");
    assert_ne!(recovered_completed.key, initial_key);
    let recovered_render = recovered_completed
        .render_snapshot
        .as_ref()
        .expect("recovery should publish paint");
    assert_frame(
        frame_for_paint(recovered_render, A_ITEM),
        initial_a,
        "recovered A item",
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision should have computed styles");
    assert_eq!(recovered_completed.key, cascade.key);
}
