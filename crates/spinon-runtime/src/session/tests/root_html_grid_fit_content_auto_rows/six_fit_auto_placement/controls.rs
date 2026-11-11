use super::*;
use spinon_style::{ComputedCssGridAutoRows, CssColorScheme, CssMediaEnvironment};
use std::time::Duration;

#[test]
fn parsed_root_html_six_fit_auto_minimum_stretch_remains_supported_in_dark_scheme() {
    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let (session, light) = run(&source);
    assert_eq!(
        light.state,
        crate::session::RuntimeLayoutState::Ready,
        "light candidate layout: {:?}",
        light.error
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark media environment applies");
    let dark = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        dark.state,
        crate::session::RuntimeLayoutState::Ready,
        "dark candidate layout: {:?}",
        dark.error
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("dark revision has computed styles");
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(&scene))
    );
    assert_eq!(
        grid.properties.get("align-content").map(String::as_str),
        Some("stretch")
    );
}

#[test]
fn parsed_root_html_six_fit_auto_minimum_accepts_normalized_absolute_calc_cap() {
    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("Chrome matrix has a forward auto-minimum scene");
    let source = source_for(&scene);
    let source = super::super::replace_once(
        &source,
        "fit-content(30px)",
        "fit-content(calc(10px + 20px))",
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "absolute calc cap normalizes to the admitted 30px cap: {:?}",
        layout.error
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("computed absolute cap belongs to the completed revision");
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(&scene)),
        "computed track order contains the normalized direct 30px cap"
    );
}
