use super::*;

#[test]
fn parsed_root_html_positions_one_through_three_reject_the_six_row_topology_atomically() {
    let (_, scenes) = capture();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-six-forward")
        .expect("capture contains a six-row topology");
    let tail = scene["tail"]
        .as_array()
        .unwrap()
        .iter()
        .map(|track| track.as_str().unwrap())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let cap = scene["cap"].as_str().unwrap();
    for position in 1..=3 {
        let mut tracks = tail.clone();
        tracks.insert(position - 1, format!("fit-content({cap})"));
        let source = source_for_scene(scene, Some(&tracks.join(" ")));
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "position {position} must not enter the six-row profile: {:?}; cascade={:?}",
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
        assert!(
            layout.completed.is_none(),
            "rejection publishes no partial output"
        );
    }
}

#[test]
fn parsed_root_html_six_row_rejection_recovers_in_the_same_session() {
    let (_, scenes) = capture();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-span3-six-forward" && scene["fitPosition"] == 4)
        .expect("capture contains a mixed six-row position-four scene");
    let valid_tracks = scene["trackList"]
        .as_array()
        .unwrap()
        .iter()
        .map(|track| track.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    let invalid_tracks = scene["tail"]
        .as_array()
        .unwrap()
        .iter()
        .map(|track| track.as_str().unwrap())
        .collect::<Vec<_>>();
    let mut invalid_tracks = invalid_tracks;
    invalid_tracks.insert(0, "fit-content(20px)");
    let invalid_tracks = invalid_tracks.join(" ");
    let source = source_for_scene(scene, Some(&valid_tracks));
    let source = source.replacen(
        "</style>",
        &format!("@media (prefers-color-scheme: dark) {{ #grid {{ grid-auto-rows: {invalid_tracks}; }} }}\n</style>"),
        1,
    );
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial_key = initial.completed.as_ref().unwrap().key;

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark scheme selects the cross-topology invalid list");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(rejected.completed.is_none(), "rejected revision is atomic");

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light scheme restores the valid six-row list");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "{:?}",
        recovered.error
    );
    assert_ne!(recovered.completed.as_ref().unwrap().key, initial_key);
}

#[test]
fn parsed_root_html_six_row_profile_rejects_non_ltr_or_non_horizontal_writing_contexts_atomically()
{
    let (_, scenes) = capture();
    let scene = scenes
        .iter()
        .find(|scene| scene["topology"] == "span2-six-forward" && scene["fitPosition"] == 4)
        .expect("capture contains the six-span position-four scene");
    let overrides = [
        (
            "#grid { direction: rtl; }\n.item, .payload { direction: ltr; }",
            "right-to-left Grid direction",
            Some("grid-auto-rows"),
        ),
        (
            ".item { direction: rtl; }\n.payload { direction: ltr; }",
            "right-to-left Grid item direction",
            Some("grid-auto-rows"),
        ),
        (
            "#grid { writing-mode: vertical-rl; }\n.item, .payload { writing-mode: horizontal-tb; }",
            "vertical Grid writing mode",
            None,
        ),
        (
            ".item { writing-mode: vertical-rl; }\n.payload { writing-mode: horizontal-tb; }",
            "vertical Grid item writing mode",
            None,
        ),
    ];
    for (override_css, case, expected_property) in overrides {
        let source = source_for_scene(scene, None).replacen(
            "</style>",
            &format!("{override_css}\n</style>"),
            1,
        );
        let (_, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{case} is outside the bounded profile"
        );
        if let Some(expected_property) = expected_property {
            assert_eq!(
                layout
                    .error
                    .as_ref()
                    .and_then(|failure| failure.property.as_deref()),
                Some(expected_property),
                "{case} fails the Root HTML style admission gate: {:?}",
                layout.error
            );
        } else {
            assert_eq!(
                layout.error.as_ref().map(|failure| failure.code),
                Some("cascade_failed"),
                "unsupported vertical writing mode must fail during cascade"
            );
        }
        assert!(layout.completed.is_none(), "{case} must reject atomically");
    }
}
