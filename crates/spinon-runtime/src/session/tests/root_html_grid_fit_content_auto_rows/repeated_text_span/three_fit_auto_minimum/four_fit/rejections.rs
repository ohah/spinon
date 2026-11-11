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
fn parsed_root_html_four_fit_row_span_rejects_out_of_profile_mutations_atomically() {
    let record = four_fit_candidate_record();
    let source = source_for(&record);
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let duplicate_cap = track_list.replacen("fit-content(20%)", "fit-content(10px)", 1);
    let five_fits = track_list.replacen("min-content", "fit-content(40px)", 1);
    let missing_auto = track_list.replacen(" auto", " max-content", 1);

    for (name, invalid) in [
        (
            "duplicate and missing cap",
            with_track_list(&record, &duplicate_cap),
        ),
        ("fifth fit track", with_track_list(&record, &five_fits)),
        ("missing auto tail", with_track_list(&record, &missing_auto)),
        (
            "unsupported fit cap",
            with_track_list(
                &record,
                &track_list.replacen("fit-content(0px)", "fit-content(31px)", 1),
            ),
        ),
        (
            "unresolved percentage calc cap",
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
    ] {
        assert_rejected_atomically(name, &invalid);
    }
}

#[test]
fn parsed_root_html_four_fit_auto_minimum_rejects_mixed_items_then_recovers() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let source = replace_once(
        &source_for(&four_fit_candidate_record()),
        "</style>",
        "@media (prefers-color-scheme: dark) { .item[data-item=\"B\"] { min-height:0; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "initial all-auto candidate is supported: {:?}",
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
        .expect("dark media query produces mixed item minimums");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none(), "rejection is atomic");

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light media query restores all automatic minimums");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery publishes a valid revision: {:?}",
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
        "recovered four-fit row-span item",
    );
}
