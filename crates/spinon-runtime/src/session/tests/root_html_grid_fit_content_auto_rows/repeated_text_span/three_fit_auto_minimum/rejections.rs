use super::*;
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
