use super::*;
use std::time::Duration;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture fragment missing: {from:?}");
    source.replacen(from, to, 1)
}

fn with_track_list(record: &Value, replacement: &str) -> String {
    replace_once(
        &super::super::source_for(record),
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
fn parsed_root_html_five_fit_row_span_rejects_out_of_profile_mutations_atomically() {
    let record = candidate_record();
    let source = super::super::source_for(&record);
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let four_fits = track_list.replacen("fit-content(40px)", "min-content", 1);
    let six_fits = track_list
        .replacen("min-content", "fit-content(50px)", 1)
        .replacen("fit-content(50px)", "fit-content(51px)", 1);
    let duplicate_cap = track_list.replacen("fit-content(20%)", "fit-content(10px)", 1);
    let missing_tail = track_list
        .split_whitespace()
        .filter(|member| *member != "min-content")
        .collect::<Vec<_>>()
        .join(" ");

    for (name, invalid) in [
        ("four fit tracks", with_track_list(&record, &four_fits)),
        (
            "six fit tracks with an unsupported cap",
            with_track_list(&record, &six_fits),
        ),
        (
            "duplicate and missing cap",
            with_track_list(&record, &duplicate_cap),
        ),
        (
            "unsupported fit cap",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(31px)", 1),
            ),
        ),
        ("missing tail", with_track_list(&record, &missing_tail)),
        (
            "unresolved percentage calculation",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(calc(2px + 2%))", 1),
            ),
        ),
        (
            "extra positive implicit row",
            replace_once(&source, "grid-row:6 / span 2", "grid-row:6 / span 3"),
        ),
        (
            "dense row flow",
            replace_once(&source, "grid-auto-flow:row;", "grid-auto-flow:row dense;"),
        ),
        (
            "scrollable spanning item",
            replace_once(
                &source,
                "overflow:visible; grid-column:1 / 2;",
                "overflow:auto; grid-column:1 / 2;",
            ),
        ),
        (
            "unsupported alignment",
            replace_once(&source, "align-content:start;", "align-content:center;"),
        ),
        (
            "indefinite percentage basis",
            replace_once(&source, "height:220px;", "height:auto;"),
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
fn parsed_root_html_five_fit_row_span_mixed_minimum_is_accepted_and_recovers_atomically() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let record = candidate_record();
    let source = replace_once(
        &source_for_track_list(record["scene"]["trackList"].as_str().unwrap()),
        "</style>",
        "@media (prefers-color-scheme: dark) { #grid .item[data-item=\"B\"] { min-height:0; } } @media (min-height:10000px) { #grid .item[data-item=\"C\"] { min-height:1px; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "light baseline is valid: {:?}",
        initial.error
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 320.0, 1.0, media)
        .expect("dark media selects the mixed automatic-minimum profile");
    let mixed = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        mixed.state,
        crate::session::RuntimeLayoutState::Ready,
        "five-fit mixed zero/auto values are supported: {:?}",
        mixed.error
    );
    assert!(mixed.completed.is_some());

    session
        .set_root_html_environment(1280.0, 10_000.0, 1.0, media)
        .expect("height media activates an unsupported fixed minimum");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(
        rejected.completed.is_none(),
        "rejection publishes no partial layout"
    );

    session
        .set_root_html_environment(1280.0, 320.0, 1.0, media)
        .expect("same-session valid mixed profile restores after rejection");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery publishes a valid revision: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
