use super::*;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

fn first_candidate() -> Value {
    records()
        .into_iter()
        .find(|record| {
            record["scene"]["minHeight"] == "auto"
                && record["scene"]["fitPosition"] == 1
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2-forward"
        })
        .expect("Chrome matrix should contain a supported auto-minimum candidate")
}

fn assert_rejected_atomically(name: &str, source: &str) {
    let (_session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail closed: {:?}",
        layout.error
    );
    assert!(layout.completed.is_none(), "{name} must not publish layout");
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| { error.node_id.is_some() && error.property.is_some() }),
        "{name} should identify the unsupported node and property"
    );
}

#[test]
fn parsed_root_html_one_fit_row_span_auto_minimum_rejects_unsupported_minimum_and_position_four() {
    let record = first_candidate();
    let source = source_for(&record);

    let nonzero_minimum = replace_once(
        &source,
        "data-item=\"B\" style=\"grid-row:3 / span 2\"",
        "data-item=\"B\" style=\"grid-row:3 / span 2; min-height:1px\"",
    );
    assert_rejected_atomically("nonzero item min-height", &nonzero_minimum);

    let intrinsic_minimum = replace_once(
        &source,
        "data-item=\"B\" style=\"grid-row:3 / span 2\"",
        "data-item=\"B\" style=\"grid-row:3 / span 2; min-height:min-content\"",
    );
    assert_rejected_atomically("intrinsic item min-height", &intrinsic_minimum);

    let automatic_control = replace_once(
        &source,
        ".empty { min-height:0; height: 10px;",
        ".empty { min-height:auto; height: 10px;",
    );
    assert_rejected_atomically("explicit-row control min-height:auto", &automatic_control);

    let mut position_four = record;
    position_four["scene"]["fitPosition"] = Value::from(4);
    assert_rejected_atomically(
        "one-fit position four in the three-row topology",
        &source_for(&position_four),
    );
}

#[test]
#[should_panic(expected = "fit position should be an unsigned integer")]
fn malformed_fit_position_in_fixture_fails_closed() {
    let mut record = first_candidate();
    record["scene"]["fitPosition"] = Value::String("1".to_owned());
    let _ = source_for(&record);
}

#[test]
fn parsed_root_html_one_fit_row_span_auto_minimum_rejects_atomically_and_recovers_same_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};
    use std::time::Duration;

    let source = source_for(&first_candidate());
    let source = replace_once(
        &source,
        "data-item=\"B\" style=\"grid-row:3 / span 2\"",
        "data-item=\"B\" id=\"span-b\" style=\"grid-row:3 / span 2\"",
    );
    let source = replace_once(
        &source,
        "</style>",
        "@media (prefers-color-scheme: dark) { #span-b { min-height:1px; } }\n</style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "light scheme supports all-auto row-span items: {:?}",
        initial.error
    );
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("candidate publishes layout");
    let initial_key = initial_completed.key;
    let initial_frame = frame_for_paint(
        initial_completed
            .render_snapshot
            .as_ref()
            .expect("candidate publishes render output"),
        A_ITEM,
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark scheme should apply the unsupported nonzero minimum");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none(), "rejected revision is atomic");
    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light scheme should restore the supported minimum profile");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    let recovered_completed = recovered.completed.expect("recovery publishes layout");
    assert_ne!(recovered_completed.key, initial_key);
    assert_frame(
        frame_for_paint(
            recovered_completed
                .render_snapshot
                .as_ref()
                .expect("recovery publishes render output"),
            A_ITEM,
        ),
        initial_frame,
        "recovered A item",
    );
}
