use super::*;
use std::time::Duration;

#[test]
fn parsed_root_html_five_fit_auto_placement_accepts_a_candidate_scene() {
    let scene = scenes()
        .into_iter()
        .find(|scene| {
            is_automatic_minimum(scene)
                && scene["scene"]["fitPositions"] == serde_json::json!([1, 2, 3, 4, 5])
                && scene["scene"]["tail"] == serde_json::json!(["min-content"])
                && scene["scene"]["alignContent"] == "start"
                && scene["scene"]["domOrder"] == "forward"
        })
        .expect("Chrome matrix has the first five-fit auto-minimum scene");
    let (session, layout) = super::super::run(&source_for(&scene));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "five-fit candidate should be admitted: {:?}",
        layout.error
    );
    let completed = layout.completed.expect("candidate scene publishes layout");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("candidate scene publishes its computed styles");
    assert_eq!(completed.key, cascade.key);
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one parsed Root HTML Grid owner exists");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        spinon_style::ComputedCssGridAutoRows::TrackList(computed_tracks(&scene))
    );
}

#[test]
fn parsed_root_html_five_fit_auto_placement_normalizes_absolute_calc_caps_to_px() {
    use spinon_style::{
        ComputedCssGridTrackLengthPercentage::LengthPx, ComputedCssGridTrackSize::FitContent,
    };

    let scene = scenes()
        .into_iter()
        .find(|scene| {
            is_automatic_minimum(scene)
                && scene["scene"]["trackList"]
                    .as_str()
                    .is_some_and(|tracks| tracks.contains("fit-content(0px)"))
        })
        .expect("Chrome matrix has a five-fit scene with a zero-pixel cap");
    let track_list = scene["scene"]["trackList"].as_str().unwrap();
    let cap_position = track_list
        .split_whitespace()
        .position(|member| member == "fit-content(0px)")
        .expect("the selected track list has one zero-pixel cap");
    let source = super::super::replace_once(
        &source_for(&scene),
        "fit-content(0px)",
        "fit-content(calc(1px + 2px))",
    );
    let (session, layout) = super::super::run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "absolute CSS calculation resolves to an admitted px cap: {:?}",
        layout.error
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed style");
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    let mut expected = computed_tracks(&scene);
    expected[cap_position] = FitContent(LengthPx(3.0));
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        spinon_style::ComputedCssGridAutoRows::TrackList(expected)
    );
}

#[test]
fn parsed_root_html_five_fit_auto_placement_reacts_to_color_scheme_in_one_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a candidate scene");
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
        "dark five-fit scene remains supported: {:?}",
        dark.error
    );
    let dark_completed = dark.completed.expect("dark scene publishes layout");
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
        .expect("light environment restores the observed profile")
        .environment_revision;
    let light = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(light.state, crate::session::RuntimeLayoutState::Ready);
    let light_completed = light.completed.expect("light scene republishes layout");
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
