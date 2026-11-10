use super::*;
use std::collections::BTreeMap;

#[test]
fn parsed_root_html_two_fit_content_rejects_four_fit_tracks_atomically_and_recovers_same_session() {
    let expected = observations()
        .into_iter()
        .next()
        .expect("precomparison includes a representative scene");
    let scene = &expected["scene"];
    let mut source = source_for(scene);
    let invalid_dark_rule = "@media (prefers-color-scheme: dark) { .grid { grid-auto-rows: fit-content(1px) fit-content(2px) fit-content(3px) fit-content(4px) min-content max-content !important; } }";
    assert!(source.contains("</style>"));
    source = source.replacen("</style>", &format!("{invalid_dark_rule}</style>"), 1);

    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("the two-fit control publishes a complete layout");
    let initial_frames = initial_completed
        .frames
        .iter()
        .map(|frame| (frame.node_id, [frame.x, frame.y, frame.width, frame.height]))
        .collect::<BTreeMap<_, _>>();

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment should select the invalid four-fit rule");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(
        rejected.completed.is_none(),
        "four-fit multiplicity must not publish a partial layout or render scene"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment should restore the supported two-fit input");
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
        .expect("recovery should publish the complete two-fit layout");
    let recovered_frames = recovered_completed
        .frames
        .iter()
        .map(|frame| (frame.node_id, [frame.x, frame.y, frame.width, frame.height]))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        initial_frames.keys().collect::<Vec<_>>(),
        recovered_frames.keys().collect::<Vec<_>>()
    );
    for (node_id, initial_frame) in initial_frames {
        assert_frame(
            recovered_frames[&node_id],
            initial_frame,
            &format!("same-session recovery node {node_id}"),
        );
    }
}
