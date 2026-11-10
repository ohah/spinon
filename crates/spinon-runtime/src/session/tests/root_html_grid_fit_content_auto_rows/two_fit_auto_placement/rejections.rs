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
fn parsed_root_html_two_fit_auto_placement_rejects_unsupported_mutations() {
    let three_fit =
        "fit-content(10px) fit-content(20%) fit-content(30px) min-content max-content auto";
    let wrong_multiplicity =
        "fit-content(10px) fit-content(20%) min-content max-content max-content auto";
    let mixed_minimums = replace_once(
        &replace_once(
            &replace_once(SOURCE, "min-height: 0;", "min-height: auto;"),
            "#item-0 { height: 10px;",
            "#item-0 { min-height: 0; height: 10px;",
        ),
        "#item-1 { background-color: #f2665e; }",
        "#item-1 { min-height: 0; background-color: #f2665e; }",
    );
    let zero_cap_scene = auto_minimum_scenes()
        .into_iter()
        .find(|scene| scene["scene"]["index"] == 0)
        .expect("Chrome auto-minimum matrix has a zero-cap first scene");
    let automatic_control_minimum = replace_once(
        &source_for_auto_minimum(&zero_cap_scene),
        "#item-0 { min-height: 0; height: 10px;",
        "#item-0 { min-height: auto; height: 10px;",
    );
    let cases = [
        (
            "three fit tracks",
            replace_once(SOURCE, BASE_TRACK_LIST, three_fit),
        ),
        (
            "wrong remaining multiplicity",
            replace_once(SOURCE, BASE_TRACK_LIST, wrong_multiplicity),
        ),
        (
            "negative first cap",
            replace_once(
                SOURCE,
                BASE_TRACK_LIST,
                "fit-content(-1px) fit-content(20%) min-content min-content max-content auto",
            ),
        ),
        (
            "calculated cap",
            replace_once(
                SOURCE,
                BASE_TRACK_LIST,
                "fit-content(calc(10px + 10%)) fit-content(20%) min-content min-content max-content auto",
            ),
        ),
        (
            "column auto-flow",
            replace_once(SOURCE, "grid-auto-flow: row;", "grid-auto-flow: column;"),
        ),
        (
            "dense auto-flow",
            replace_once(SOURCE, "grid-auto-flow: row;", "grid-auto-flow: row dense;"),
        ),
        (
            "implicit second column",
            replace_once(
                SOURCE,
                "grid-template-columns: 40px;",
                "grid-template-columns: 40px 40px;",
            ),
        ),
        (
            "indefinite grid block size",
            replace_once(SOURCE, "height: 220px;", "height: auto;"),
        ),
        (
            "unsupported alignment",
            replace_once(SOURCE, "align-content: start;", "align-content: center;"),
        ),
        (
            "seven auto-placement text items",
            replace_once(
                SOURCE,
                "      <div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA</div></div>\n",
                "",
            ),
        ),
        (
            "nested payload child",
            replace_once(
                SOURCE,
                "<div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA</div></div>",
                "<div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA<span>BB</span></div></div>",
            ),
        ),
        (
            "invalid item minimum",
            replace_once(SOURCE, "min-height: 0;", "min-height: 1px;"),
        ),
        ("mixed zero and automatic item minimums", mixed_minimums),
        (
            "automatic minimum on the explicit-row control",
            automatic_control_minimum,
        ),
        (
            "mixed auto-placement and row-span topology",
            replace_once(
                &replace_once(
                    SOURCE,
                    "#item-1 { background-color: #f2665e; }",
                    "#item-1 { grid-row: 2 / span 2; background-color: #f2665e; }",
                ),
                "min-height: 0;",
                "min-height: auto;",
            ),
        ),
    ];

    for (label, source) in cases {
        assert_rejected(label, &source);
    }
}

#[test]
fn parsed_root_html_two_fit_auto_placement_rejects_atomically_and_recovers_same_session() {
    let scene = auto_minimum_scenes()
        .into_iter()
        .find(|scene| scene["scene"]["index"] == 0)
        .expect("the Chrome matrix has its first scene");
    let source = source_for_auto_minimum(&scene);
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("supported initial input publishes a complete layout");
    let initial_key = initial_completed.key;
    let initial_frames = initial_completed.frames.clone();

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment should be applied");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        rejected.state,
        crate::session::RuntimeLayoutState::Failed,
        "the dark three-fit control should fail closed: {:?}",
        rejected.error
    );
    assert!(
        rejected.completed.is_none(),
        "failure must not retain or publish the preceding revision's completed frames"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment should restore the supported track list");
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
        .expect("recovery publishes a complete layout");
    assert_ne!(
        recovered_completed.key, initial_key,
        "recovery uses a new revision"
    );
    assert_eq!(
        recovered_completed.frames, initial_frames,
        "the recovered scene restores all initial item and text frames"
    );
    assert!(recovered_completed.render_snapshot.is_some());
}
