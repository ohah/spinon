use super::*;

#[test]
fn parsed_root_html_later_fit_content_positions_two_and_three_reject_auto_placement_profile() {
    let (_, scenes) = scenes();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-before-span3")
        .expect("matrix contains a bounded span topology");
    let original_tracks = "fit-content(20px) min-content min-content max-content max-content auto";
    for fit_position in 2..=3 {
        let tracks = track_list_for_position(scene, fit_position);
        let source = replace_once(REPEATED_AUTO_PLACEMENT_SOURCE, original_tracks, &tracks);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "position {fit_position} must not broaden repeated auto-placement: {:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        assert_eq!(
            layout
                .error
                .as_ref()
                .and_then(|failure| failure.property.as_deref()),
            Some("grid-auto-rows"),
            "position {fit_position} reports the rejected property"
        );
        assert!(layout.completed.is_none(), "no partial output is published");
    }
}

#[test]
fn parsed_root_html_later_fit_content_positions_4_through_6_reject_atomically() {
    let (_, scenes) = scenes();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-before-span3")
        .expect("matrix contains a span-3 scene");
    for fit_position in 4..=6 {
        let tracks = track_list_for_position(scene, fit_position);
        let source = source_with_track_list(scene, &tracks, None);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "fit-content position {fit_position} must fail closed: layout={:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        assert_eq!(
            layout
                .error
                .as_ref()
                .and_then(|failure| failure.property.as_deref()),
            Some("grid-auto-rows"),
            "fit-content position {fit_position} reports the rejected property"
        );
        assert!(layout.completed.is_none(), "no partial output is published");
    }
}

#[test]
fn parsed_root_html_later_fit_content_duplicate_fit_track_rejects_atomically() {
    let (_, scenes) = scenes();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-before-span3")
        .expect("matrix contains a span topology");
    let tracks =
        "fit-content(20px) fit-content(20px) min-content max-content min-content max-content";
    let source = source_with_track_list(scene, tracks, None);
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "duplicate fit-content must not enter the bounded profile: {:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    assert_eq!(
        layout
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(layout.completed.is_none(), "no partial output is published");
}

#[test]
fn parsed_root_html_later_fit_content_rejection_recovers_in_the_same_session() {
    let (_, scenes) = scenes();
    let scene = &scenes[0];
    let invalid_tracks = track_list_for_position(scene, 4);
    let source = source_with_track_list(scene, &scene_track_list(scene), Some(&invalid_tracks));
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_key = initial
        .completed
        .as_ref()
        .expect("initial output exists")
        .key;

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark mode selects the unsupported position");
    let rejected = session.wait_for_layout_snapshot(std::time::Duration::from_secs(5));
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
        "rejection publishes no partial frames"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light mode restores the supported position");
    let recovered = session.wait_for_layout_snapshot(std::time::Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    let completed = recovered
        .completed
        .as_ref()
        .expect("recovery publishes layout");
    assert_ne!(
        completed.key, initial_key,
        "recovery publishes a new revision"
    );
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("recovery publishes render output");
    for item in scene["items"].as_array().expect("items are present") {
        let id = item["id"].as_str().unwrap();
        assert_frame(
            frame_for_paint(render, color_for_item(id)),
            frame(&item["frame"]),
            &format!("recovered item {id}"),
        );
    }
}
