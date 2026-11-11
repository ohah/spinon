use super::*;
use std::time::Duration;

fn assert_rejected(label: &str, source: &str) {
    let (session, layout) = run(source);
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
fn parsed_root_html_six_fit_auto_placement_rejects_unlisted_and_out_of_profile_inputs() {
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
            "zero-fit list",
            replace_tracks("min-content max-content auto min-content max-content auto"),
        ),
        (
            "short five-member fit list",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px)",
            ),
        ),
        (
            "seven-member list",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(50px) auto",
            ),
        ),
        (
            "duplicate cap and missing cap",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(40px)",
            ),
        ),
        (
            "unlisted finite cap",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(60px)",
            ),
        ),
        (
            "unsupported percentage cap",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(10%) fit-content(30px) fit-content(40px) fit-content(50px)",
            ),
        ),
        (
            "negative cap",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(-1px) fit-content(40px) fit-content(50px)",
            ),
        ),
        (
            "unresolved percentage calculation",
            replace_tracks(
                "fit-content(0px) fit-content(10px) fit-content(calc(20% + 1px)) fit-content(30px) fit-content(40px) fit-content(50px)",
            ),
        ),
        (
            "indefinite percentage basis",
            super::super::replace_once(&source, "height: 220px;", "height: auto;"),
        ),
        (
            "mixed item minima",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: 0; }\n      #grid #item-1 { min-height: 0; }",
            ),
        ),
        (
            "automatic minimum on control item",
            super::super::replace_once(
                &source,
                "#grid #item-0 { min-height: 0; }",
                "#grid #item-0 { min-height: auto; }",
            ),
        ),
        (
            "nested payload element",
            super::super::replace_once(&source, payload, &format!("<span>{payload}</span>")),
        ),
        (
            "unsupported font size",
            super::super::replace_once(&source, "font-size: 10px;", "font-size: 11px;"),
        ),
        (
            "unsupported line height",
            super::super::replace_once(&source, "line-height: 10px;", "line-height: 11px;"),
        ),
        (
            "right-to-left writing direction",
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
            "row span",
            super::super::replace_once(
                &source,
                "#item-1 { background-color:",
                "#item-1 { grid-row: 2 / span 2; background-color:",
            ),
        ),
        (
            "scrollable overflow",
            super::super::replace_once(&source, "overflow: visible;", "overflow: auto;"),
        ),
        (
            "scrollable text item",
            super::super::replace_once(
                &source,
                "#item-1 { background-color:",
                "#item-1 { overflow: auto; background-color:",
            ),
        ),
        (
            "unsupported alignment",
            super::super::replace_once(&source, "align-content: start;", "align-content: center;"),
        ),
        (
            "wrong Grid width",
            super::super::replace_once(&source, "width: 40px;", "width: 41px;"),
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
            "second Grid owner",
            super::super::replace_once(
                &source,
                "  </body>",
                "    <div id=\"grid-extra\" style=\"display: grid; width: 1px; height: 1px; grid-template-columns: 1px; grid-auto-rows: fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) fit-content(5px) fit-content(6px)\"></div>\n  </body>",
            ),
        ),
    ];

    for (label, source) in cases {
        assert_rejected(label, &source);
    }
}

#[test]
fn parsed_root_html_six_fit_rejection_is_atomic_and_same_session_recovers() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let dark_override = "\n      @media (prefers-color-scheme: dark) { #grid { grid-auto-rows: fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(60px); } }\n";
    let source = super::super::replace_once(
        &source,
        "    </style>",
        &format!("{dark_override}    </style>"),
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "supported light scene: {:?}",
        initial.error
    );
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("supported six-fit scene publishes layout");
    let initial_key = initial_completed.key;
    let initial_frames = initial_completed.frames.clone();

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    let invalid_revision = session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark media environment applies");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        rejected.state,
        crate::session::RuntimeLayoutState::Failed,
        "invalid dark cap must fail closed: {:?}",
        rejected.error
    );
    assert!(
        rejected.completed.is_none(),
        "invalid revision publishes no stale layout"
    );
    let rejected_cascade = session.ua_cascade_snapshot();
    assert_eq!(
        rejected_cascade.state,
        crate::session::RuntimeUaCascadeState::Ready,
        "the supported CSS parser still computes the unlisted cap"
    );
    let rejected_styles = rejected_cascade
        .completed
        .as_ref()
        .expect("the rejected environment has revision-owned computed styles");
    assert_eq!(
        rejected_styles.key.environment_revision,
        invalid_revision.environment_revision.get()
    );
    assert_eq!(rejected_cascade.requested, Some(rejected_styles.key));
    let mut dark_styles = rejected_cascade
        .completed
        .as_ref()
        .expect("dark revision has computed styles")
        .roots[0]
        .styles
        .elements
        .iter();
    let dark_grid = dark_styles
        .find(|style| style.grid_layout.is_some())
        .expect("dark revision retains its Grid owner");
    assert!(
        dark_grid
            .properties
            .get("grid-auto-rows")
            .is_some_and(|value| value.contains("fit-content(60px)")),
        "dark revision computed the intentionally unlisted cap"
    );

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light recovery environment applies");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same RuntimeSession recovers: {:?}",
        recovered.error
    );
    let recovered_completed = recovered
        .completed
        .as_ref()
        .expect("valid light revision publishes a layout");
    assert_ne!(recovered_completed.key, initial_key);
    assert_eq!(recovered_completed.frames, initial_frames);
}
