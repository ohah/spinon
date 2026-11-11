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
        "{label}: rejection must not publish layout or render output"
    );
}

#[test]
fn parsed_root_html_five_fit_auto_placement_rejects_out_of_profile_mutations() {
    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let tracks = scene["scene"]["trackList"].as_str().unwrap();
    let replace_tracks =
        |replacement: &str| super::super::replace_once(&source, tracks, replacement);
    let payload = super::super::two_fit_auto_placement::PAYLOAD_TEXTS[7];
    let cases = [
        (
            "zero-fit six-member list",
            replace_tracks("min-content max-content auto min-content max-content auto"),
        ),
        (
            "six fit tracks without a tail",
            replace_tracks(
                "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) fit-content(6px)",
            ),
        ),
        (
            "four fit tracks with two tails",
            replace_tracks(
                "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) min-content auto",
            ),
        ),
        (
            "five fit tracks in a five-member list",
            replace_tracks(
                "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px)",
            ),
        ),
        (
            "five fit tracks in a seven-member list",
            replace_tracks(
                "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) min-content max-content",
            ),
        ),
        (
            "unsupported negative cap",
            replace_tracks(
                "fit-content(-1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) min-content",
            ),
        ),
        (
            "unresolved percentage calculation in cap",
            replace_tracks(
                "fit-content(calc(1px + 2%)) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) min-content",
            ),
        ),
        (
            "unsupported fixed-size tail",
            replace_tracks(
                "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) 10px",
            ),
        ),
        (
            "percentage cap with indefinite Grid block size",
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
            "automatic minimum on the explicit-row control",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: auto; }",
            ),
        ),
        (
            "nonzero fixed minimum on a repeated item",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: 0; }\n      #grid #item-1 { min-height: 1px; }",
            ),
        ),
        (
            "nested payload element",
            super::super::replace_once(&source, payload, &format!("<span>{payload}</span>")),
        ),
        (
            "replaced payload element",
            super::super::replace_once(
                &source,
                &format!("<div id=\"text-8\" class=\"payload\">{payload}</div>"),
                "<img id=\"text-8\" class=\"payload\" src=\"data:,AA\">",
            ),
        ),
        (
            "inline payload box",
            super::super::replace_once(
                &source,
                "      .payload {\n        display: block;",
                "      .payload {\n        display: inline;",
            ),
        ),
        (
            "unsupported payload font size",
            super::super::replace_once(&source, "font-size: 10px;", "font-size: 11px;"),
        ),
        (
            "unsupported payload text",
            super::super::replace_once(&source, payload, "A1"),
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
            "explicit placement",
            super::super::replace_once(
                &source,
                "#item-1 { background-color:",
                "#item-1 { grid-row: 2; background-color:",
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
            "additional explicit column",
            super::super::replace_once(
                &source,
                "grid-template-columns: 40px;",
                "grid-template-columns: 20px 20px;",
            ),
        ),
        (
            "additional explicit row",
            super::super::replace_once(
                &source,
                "grid-template-rows: 10px;",
                "grid-template-rows: 10px 10px;",
            ),
        ),
        (
            "missing repeated item",
            super::super::replace_once(
                &source,
                &format!(
                    "      <div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">{payload}</div></div>"
                ),
                "",
            ),
        ),
        (
            "scrollable Grid overflow",
            super::super::replace_once(&source, "overflow: visible;", "overflow: auto;"),
        ),
        (
            "unsupported content alignment",
            super::super::replace_once(&source, "align-content: start;", "align-content: center;"),
        ),
        (
            "second Grid owner",
            super::super::replace_once(
                &source,
                "  </body>",
                "    <div id=\"grid-extra\" style=\"display: grid; width: 1px; height: 1px; grid-template-columns: 1px; grid-auto-rows: fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) auto\"></div>\n  </body>",
            ),
        ),
    ];

    for (label, source) in cases {
        assert_rejected(label, &source);
    }
}

#[test]
fn parsed_root_html_five_fit_auto_placement_rejects_atomically_and_recovers_same_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    const INVALID_DARK_TRACK_LIST: &str = "fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) fit-content(6px)";

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let source = super::super::replace_once(&source, DARK_TRACK_LIST, INVALID_DARK_TRACK_LIST);
    let (session, initial) = super::super::run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("supported five-fit scene publishes a complete layout");
    let initial_key = initial_completed.key;
    let initial_frames = initial_completed.frames.clone();

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    let invalid_revision = session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment applies");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        rejected.state,
        crate::session::RuntimeLayoutState::Failed,
        "invalid six-fit dark mutation fails closed: {:?}",
        rejected.error
    );
    assert!(
        rejected.completed.is_none(),
        "rejection must not publish stale layout frames"
    );
    let rejected_cascade = session.ua_cascade_snapshot();
    assert_eq!(
        rejected_cascade.state,
        crate::session::RuntimeUaCascadeState::Ready,
        "the CSS cascade should compute the syntactically valid but unsupported track list"
    );
    let rejected_styles = rejected_cascade
        .completed
        .as_ref()
        .expect("the rejected layout revision still has computed styles");
    assert_eq!(
        rejected_styles.key.environment_revision,
        invalid_revision.environment_revision.get()
    );
    assert_eq!(rejected_cascade.requested, Some(rejected_styles.key));
    let rejected_grid = rejected_styles.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("the invalid-revision Grid style is available");
    assert_eq!(
        rejected_grid
            .properties
            .get("grid-auto-rows")
            .map(String::as_str),
        Some(INVALID_DARK_TRACK_LIST),
        "cascade styles belong to the exact layout revision that was rejected"
    );

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment restores the supported five-fit list");
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
    assert_eq!(
        session
            .ua_cascade_snapshot()
            .completed
            .expect("recovered revision publishes computed styles")
            .key,
        recovered_completed.key
    );
}
