use super::*;
use std::time::Duration;

#[test]
fn parsed_root_html_three_fit_auto_placement_accepts_unlisted_typed_caps() {
    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let source = super::super::replace_once(
        &source,
        scene["scene"]["trackList"].as_str().unwrap(),
        "fit-content(17px) fit-content(31%) fit-content(37px) min-content max-content auto",
    );
    let (session, layout) = super::super::run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "unlisted finite direct caps should stay within the typed profile: {:?}",
        layout.error
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some("fit-content(17px) fit-content(31%) fit-content(37px) min-content max-content auto")
    );
}

#[test]
fn parsed_root_html_three_fit_auto_placement_reacts_to_color_scheme_in_one_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let (session, initial) = super::super::run(&source_for(&scene));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    let dark_revision = session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment applies")
        .environment_revision;
    let dark = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        dark.state,
        crate::session::RuntimeLayoutState::Ready,
        "{:?}",
        dark.error
    );
    let dark_completed = dark.completed.expect("dark Grid publishes layout");
    assert_eq!(dark_completed.key.environment_revision, dark_revision.get());
    let dark_cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("dark cascade belongs to the current revision");
    let dark_grid = dark_cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("dark Grid owner exists");
    assert_eq!(
        dark_grid
            .properties
            .get("grid-auto-rows")
            .map(String::as_str),
        Some(DARK_TRACK_LIST)
    );
    assert_eq!(
        dark_grid
            .properties
            .get("align-content")
            .map(String::as_str),
        Some("stretch")
    );

    media.color_scheme = CssColorScheme::Light;
    let light_revision = session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment restores the authored light profile")
        .environment_revision;
    let light = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        light.state,
        crate::session::RuntimeLayoutState::Ready,
        "{:?}",
        light.error
    );
    let light_completed = light.completed.expect("light Grid republishes layout");
    assert_eq!(
        light_completed.key.environment_revision,
        light_revision.get()
    );
    let light_cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("light cascade belongs to the current revision");
    let light_grid = light_cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("light Grid owner exists");
    assert_eq!(
        light_grid
            .properties
            .get("grid-auto-rows")
            .map(String::as_str),
        scene["scene"]["trackList"].as_str()
    );
}
