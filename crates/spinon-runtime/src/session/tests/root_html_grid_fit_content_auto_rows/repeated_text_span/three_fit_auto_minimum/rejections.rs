use super::*;
use std::time::Duration;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture fragment missing: {from:?}");
    source.replacen(from, to, 1)
}

fn with_track_list(record: &Value, replacement: &str) -> String {
    replace_once(
        &source_for(record),
        record["scene"]["trackList"].as_str().unwrap(),
        replacement,
    )
}

fn assert_rejected_atomically(name: &str, source: &str) {
    let (session, layout) = run(source);
    let cascade_error = session.ua_cascade_snapshot().error;
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail closed: {:?}",
        layout.error
    );
    assert!(layout.completed.is_none(), "{name} must publish no layout");
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| error.node_id.is_some() && error.property.is_some()),
        "{name} should identify the unsupported node and property: layout={:?}; cascade={cascade_error:?}",
        layout.error,
    );
}

#[test]
fn parsed_root_html_three_fit_row_span_rejects_out_of_profile_mutations_atomically() {
    let record = candidate_record();
    let source = source_for(&record);
    let original_tracks = record["scene"]["trackList"].as_str().unwrap();
    let four_fits = original_tracks.replacen(" auto", " fit-content(30px)", 1);
    let wrong_tail = original_tracks.replacen(" auto", " min-content", 1);
    let wrong_length = original_tracks
        .strip_suffix(" auto")
        .expect("matrix ends with the auto member");

    for (name, invalid) in [
        ("four fit tracks", with_track_list(&record, &four_fits)),
        (
            "wrong tail multiplicity",
            with_track_list(&record, &wrong_tail),
        ),
        (
            "calculated zero fit cap",
            with_track_list(
                &record,
                &original_tracks.replacen("fit-content(0px)", "fit-content(calc(0px))", 1),
            ),
        ),
        (
            "percentage-bearing calculated fit cap",
            with_track_list(
                &record,
                &original_tracks.replacen("fit-content(0px)", "fit-content(calc(10px + 10%))", 1),
            ),
        ),
        ("short track list", with_track_list(&record, wrong_length)),
        (
            "three-fit auto-placement",
            replace_once(&source, "grid-auto-flow:row;", "grid-auto-flow:row dense;"),
        ),
        (
            "unsupported alignment",
            replace_once(&source, "align-content:start;", "align-content:end;"),
        ),
        (
            "indefinite percentage basis",
            replace_once(&source, "height:220px;", "height:auto;"),
        ),
        (
            "wrong column placement",
            replace_once(
                &source,
                "grid-row:2 / span 2\"><div class=\"payload\">",
                "grid-row:2 / span 2; grid-column:2 / 3\"><div class=\"payload\">",
            ),
        ),
        (
            "unsupported span topology",
            replace_once(
                &source,
                "grid-row:3 / span 2\"><div class=\"payload\">",
                "grid-row:3 / span 3\"><div class=\"payload\">",
            ),
        ),
        (
            "seven positive implicit rows",
            replace_once(
                &source,
                "grid-row:6 / span 2\"><div class=\"payload\">",
                "grid-row:6 / span 3\"><div class=\"payload\">",
            ),
        ),
        (
            "scrollable item overflow",
            replace_once(
                &source,
                "overflow:visible; grid-column:1 / 2;",
                "overflow:auto; grid-column:1 / 2;",
            ),
        ),
        (
            "unsupported text payload",
            replace_once(&source, "AAAA BBBB CCCC DDDD EEEE FFFF", "가나다"),
        ),
    ] {
        assert_rejected_atomically(name, &invalid);
    }
}

#[test]
fn parsed_root_html_three_fit_row_span_auto_minimum_rejects_mixed_items_and_recovers() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let record = candidate_record();
    let source = source_for(&record);
    let mixed = replace_once(
        &source,
        "</style>",
        "@media (prefers-color-scheme: dark) { .item[data-item=\"B\"] { min-height:0; } } </style>",
    );
    let (session, initial) = run(&mixed);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "initial light candidate is supported: {:?}",
        initial.error
    );
    let initial_completed = initial.completed.as_ref().unwrap();
    let initial_key = initial_completed.key;
    let initial_frame = frame_for_paint(
        initial_completed.render_snapshot.as_ref().unwrap(),
        color_for_item("A"),
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark media query applies the mixed item minimum");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none(), "rejected revision is atomic");

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light media query restores the supported profile");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery publishes a new revision: {:?}",
        recovered.error
    );
    let recovered_completed = recovered.completed.expect("recovered layout is published");
    assert_ne!(recovered_completed.key, initial_key);
    assert_frame(
        frame_for_paint(
            recovered_completed.render_snapshot.as_ref().unwrap(),
            color_for_item("A"),
        ),
        initial_frame,
        "recovered row-span item",
    );
}
