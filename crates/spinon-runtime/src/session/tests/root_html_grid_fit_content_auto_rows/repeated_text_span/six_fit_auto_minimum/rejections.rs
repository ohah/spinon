use super::*;
use std::time::Duration;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture fragment missing: {from:?}");
    source.replacen(from, to, 1)
}

fn with_track_list(record: &Value, replacement: &str) -> String {
    replace_once(
        &source_for_record(record),
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
    let has_layout_location = layout
        .error
        .as_ref()
        .is_some_and(|error| error.node_id.is_some() && error.property.is_some());
    assert!(
        has_layout_location || cascade_error.is_some(),
        "{name} should identify the rejected input in layout or cascade diagnostics: layout={:?}; cascade={cascade_error:?}",
        layout.error,
    );
}

#[test]
fn parsed_root_html_six_fit_row_span_rejects_out_of_profile_mutations_atomically() {
    let record = candidate_record();
    let source = source_for_record(&record);
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let two_fits = track_list
        .split_whitespace()
        .enumerate()
        .map(|(index, member)| if index < 2 { member } else { "auto" })
        .collect::<Vec<_>>()
        .join(" ");
    let no_fits = track_list
        .split_whitespace()
        .map(|_| "auto")
        .collect::<Vec<_>>()
        .join(" ");
    let duplicate_cap = track_list.replacen("fit-content(20%)", "fit-content(10px)", 1);
    let missing_item = replace_once(
        &source,
        "<div class=\"item\" data-item=\"E\" style=\"grid-row:6 / span 2\"><div class=\"payload\">AAA BBBB CCCCCC</div></div>",
        "",
    );
    let extra_item = replace_once(
        &source,
        "</div></body>",
        "<div class=\"item\" data-item=\"extra\" style=\"grid-row:8 / span 1\"></div></div></body>",
    );

    let invalid_inputs = [
        (
            "unsupported two fit tracks",
            with_track_list(&record, &two_fits),
        ),
        ("no fit tracks", with_track_list(&record, &no_fits)),
        (
            "seven authored tracks",
            with_track_list(&record, &format!("{track_list} auto")),
        ),
        (
            "duplicate and missing cap",
            with_track_list(&record, &duplicate_cap),
        ),
        (
            "unsupported cap",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(31px)", 1),
            ),
        ),
        (
            "unresolved calculation",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(calc(2px + 2%))", 1),
            ),
        ),
        (
            "negative cap",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(-1px)", 1),
            ),
        ),
        (
            "flexible track",
            with_track_list(&record, &track_list.replacen("fit-content(0px)", "1fr", 1)),
        ),
        ("missing active child", missing_item),
        ("extra positive implicit row", extra_item),
        (
            "wrong row span",
            replace_once(&source, "grid-row:6 / span 2", "grid-row:6 / span 3"),
        ),
        (
            "column flow",
            replace_once(&source, "grid-auto-flow:row;", "grid-auto-flow:column;"),
        ),
        (
            "dense row flow",
            replace_once(&source, "grid-auto-flow:row;", "grid-auto-flow:row dense;"),
        ),
        (
            "scrollable grid",
            replace_once(
                &source,
                "overflow:visible; margin:0;",
                "overflow:auto; margin:0;",
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
            "unexpected explicit column",
            replace_once(
                &source,
                "grid-template-columns:40px;",
                "grid-template-columns:40px 40px;",
            ),
        ),
        (
            "unexpected explicit row",
            replace_once(
                &source,
                "grid-template-rows:10px;",
                "grid-template-rows:10px 10px;",
            ),
        ),
        (
            "right-to-left direction",
            replace_once(&source, "direction:ltr;", "direction:rtl;"),
        ),
        (
            "nonzero row gap",
            replace_once(&source, "row-gap:0;", "row-gap:1px;"),
        ),
        (
            "unsupported font",
            replace_once(&source, "font-family:monospace;", "font-family:serif;"),
        ),
        (
            "unsupported line height",
            replace_once(&source, "line-height:10px;", "line-height:12px;"),
        ),
        (
            "unsupported text payload",
            replace_once(&source, "AAAA BBBB CCCC DDDD EEEE FFFF", "가나다"),
        ),
    ];
    for (name, invalid) in invalid_inputs {
        assert_rejected_atomically(name, &invalid);
    }
}

#[test]
fn parsed_root_html_six_fit_row_span_mixed_minimum_rejection_is_atomic_and_recovers() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let record = candidate_record();
    let source = replace_once(
        &source_for_record(&record),
        "</style>",
        "@media (prefers-color-scheme: dark) { #grid .item[data-item=\"B\"] { min-height:0; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "light baseline is valid: {:?}",
        initial.error,
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 320.0, 1.0, media)
        .expect("dark media selects the mixed automatic-minimum profile");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(
        rejected.completed.is_none(),
        "rejection publishes no partial layout"
    );

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 320.0, 1.0, media)
        .expect("light media restores the all-zero baseline");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery publishes a valid revision: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
