use super::*;
use std::time::Duration;

fn assert_rejected(label: &str, source: &str) {
    let (session, layout) = super::super::run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{label}: layout={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    assert!(
        layout.completed.is_none(),
        "{label}: failure must not publish layout or render output"
    );
}

#[test]
fn parsed_root_html_three_fit_auto_placement_rejects_out_of_profile_mutations() {
    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let tracks = scene["scene"]["trackList"].as_str().unwrap();
    let replace_tracks =
        |replacement: &str| super::super::replace_once(&source, tracks, replacement);
    let cases = [
        ("four fit tracks", replace_tracks(DARK_FOUR_FIT_TRACK_LIST)),
        (
            "wrong three-fit tail multiplicity",
            replace_tracks(
                "fit-content(10px) fit-content(20px) fit-content(30px) min-content min-content max-content",
            ),
        ),
        (
            "five-member track list",
            replace_tracks(
                "fit-content(10px) fit-content(20px) fit-content(30px) min-content auto",
            ),
        ),
        (
            "seven-member track list",
            replace_tracks(
                "fit-content(10px) fit-content(20px) fit-content(30px) min-content max-content auto auto",
            ),
        ),
        (
            "second visible intrinsic Grid owner",
            super::super::replace_once(
                &source,
                "  </body>",
                "    <div id=\"grid-extra\" style=\"display: grid; width: 1px; height: 1px; grid-template-columns: 1px; grid-auto-rows: fit-content(1px) min-content max-content auto\"></div>\n  </body>",
            ),
        ),
        (
            "negative fit cap",
            replace_tracks(
                "fit-content(-1px) fit-content(20px) fit-content(30px) min-content max-content auto",
            ),
        ),
        (
            "calculated fit cap",
            replace_tracks(
                "fit-content(calc(10px + 10%)) fit-content(20px) fit-content(30px) min-content max-content auto",
            ),
        ),
        (
            "indefinite percentage basis",
            super::super::replace_once(&source, "height: 220px;", "height: auto;"),
        ),
        (
            "mixed zero and automatic item minima",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: 0; }\n      #grid #item-1 { min-height: 0; }",
            ),
        ),
        (
            "automatic minimum on explicit-row control",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: auto; }",
            ),
        ),
        (
            "unsupported fixed text-item minimum",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: 0; }\n      #grid #item-1 { min-height: 1px; }",
            ),
        ),
        (
            "nested payload element",
            super::super::replace_once(
                &source,
                "<div id=\"text-8\" class=\"payload\">AA</div>",
                "<div id=\"text-8\" class=\"payload\">AA<span>BB</span></div>",
            ),
        ),
        (
            "unsupported payload font size",
            super::super::replace_once(&source, "font-size: 10px;", "font-size: 11px;"),
        ),
        (
            "unsupported payload text",
            super::super::replace_once(
                &source,
                "<div id=\"text-8\" class=\"payload\">AA</div>",
                "<div id=\"text-8\" class=\"payload\">A1</div>",
            ),
        ),
        (
            "right-to-left Grid direction",
            super::super::replace_once(&source, "direction: ltr;", "direction: rtl;"),
        ),
        (
            "column auto-flow",
            super::super::replace_once(&source, "grid-auto-flow: row;", "grid-auto-flow: column;"),
        ),
        (
            "dense auto-flow",
            super::super::replace_once(
                &source,
                "grid-auto-flow: row;",
                "grid-auto-flow: row dense;",
            ),
        ),
        (
            "row-span placement",
            super::super::replace_once(
                &source,
                "#item-1 { background-color:",
                "#item-1 { grid-row: 2 / span 2; background-color:",
            ),
        ),
        (
            "missing repeated item",
            super::super::replace_once(
                &source,
                "      <div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA</div></div>",
                "",
            ),
        ),
        (
            "scrollable Grid overflow",
            super::super::replace_once(&source, "overflow: visible;", "overflow: auto;"),
        ),
        (
            "unsupported alignment",
            super::super::replace_once(&source, "align-content: start;", "align-content: center;"),
        ),
    ];

    for (label, source) in cases {
        assert_rejected(label, &source);
    }
}

#[test]
fn parsed_root_html_three_fit_auto_placement_rejects_atomically_and_recovers_same_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let source = super::super::replace_once(&source, DARK_TRACK_LIST, DARK_FOUR_FIT_TRACK_LIST);
    let (session, initial) = super::super::run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("supported initial scene publishes a complete layout");
    let initial_key = initial_completed.key;
    let initial_frames = initial_completed.frames.clone();

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment applies");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        rejected.state,
        crate::session::RuntimeLayoutState::Failed,
        "four-fit dark mutation fails closed: {:?}",
        rejected.error
    );
    assert!(
        rejected.completed.is_none(),
        "rejection must not publish stale layout frames"
    );

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment restores the supported list");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    let recovered_completed = recovered
        .completed
        .as_ref()
        .expect("recovery publishes a complete layout");
    assert_ne!(
        recovered_completed.key, initial_key,
        "recovery uses a new revision"
    );
    assert_eq!(recovered_completed.frames, initial_frames);
    assert!(recovered_completed.render_snapshot.is_some());
}
