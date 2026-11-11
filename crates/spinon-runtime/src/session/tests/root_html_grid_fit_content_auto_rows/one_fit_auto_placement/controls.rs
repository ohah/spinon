use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage as TrackLength,
    ComputedCssGridTrackSize as TrackSize,
};
use std::time::Duration;

#[test]
fn parsed_root_html_one_fit_auto_minimum_accepts_unlisted_px_and_percentage_caps() {
    for (label, track_list, fit_item_index, floor, expected_cap) in [
        (
            "direct 7px cap",
            "fit-content(7px) min-content max-content max-content auto min-content",
            1usize,
            20.0f32,
            TrackLength::LengthPx(7.0),
        ),
        (
            "13% cap below a three-line floor",
            "min-content max-content min-content fit-content(13%) max-content auto",
            4usize,
            30.0f32,
            TrackLength::Percentage(0.13),
        ),
    ] {
        let source = source_for_track_list(track_list);
        let (session, layout) = super::super::run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: {:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        let completed = layout.completed.as_ref().expect("control layout publishes");
        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("control publishes computed styles");
        let grid = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("one Grid owner exists");
        let expected_tracks = track_list
            .split_whitespace()
            .map(parse_track)
            .collect::<Vec<_>>();
        assert_eq!(
            grid.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "{label}: typed track order and cap"
        );
        let fit_position = track_list
            .split_whitespace()
            .position(|track| track.starts_with("fit-content("))
            .unwrap();
        assert_eq!((fit_item_index - 1) % 6, fit_position);
        let ComputedCssGridAutoRows::TrackList(actual_tracks) =
            &grid.grid_layout.as_ref().unwrap().auto_rows
        else {
            panic!("{label}: typed track list");
        };
        assert!(matches!(
            actual_tracks.get(fit_position),
            Some(TrackSize::FitContent(actual)) if actual == &expected_cap
        ));
        let color = super::super::two_fit_auto_placement::ITEM_COLORS[fit_item_index];
        let item_style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| {
                style.background_color.is_some_and(|actual| {
                    (actual.red, actual.green, actual.blue)
                        == (color.red(), color.green(), color.blue())
                })
            })
            .expect("fit item has computed style");
        assert_eq!(
            item_style.layout_dimensions.min_height,
            spinon_style::ComputedCssDimension::Auto,
            "{label}: auto minimum reaches projection"
        );
        let frame = super::super::two_fit_auto_placement::frame_for_background(
            completed,
            &cascade.roots[0].styles.elements,
            color,
            label,
        );
        assert!(
            frame[3] + 0.01 >= floor,
            "{label}: {frame:?} below {floor}px floor"
        );
    }
}

#[test]
fn parsed_root_html_one_fit_auto_minimum_rejects_then_recovers_same_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    const DARK_THREE_FIT_TRACK_LIST: &str =
        "fit-content(10px) fit-content(20%) fit-content(30px) min-content max-content auto";
    const DARK_UNLISTED_SIX_FIT_TRACK_LIST: &str = "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(60px)";

    let scene = super::auto_minimum_scenes()
        .into_iter()
        .find(|scene| {
            scene["fitPosition"].as_u64() == Some(1)
                && scene["cap"].as_str() == Some("0%")
                && scene["align"].as_str() == Some("start")
        })
        .expect("one-fit zero cap start scene exists");
    let source = super::source_for(&scene, true);
    let source = super::super::replace_once(
        &source,
        DARK_THREE_FIT_TRACK_LIST,
        DARK_UNLISTED_SIX_FIT_TRACK_LIST,
    );
    let (session, initial) = super::super::run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    let initial = initial.completed.expect("valid profile publishes layout");
    let initial_key = initial.key;
    let initial_frames = initial.frames.clone();

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment applies");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        rejected.state,
        crate::session::RuntimeLayoutState::Failed,
        "the dark unlisted six-fit list fails closed: {:?}",
        rejected.error
    );
    assert!(
        rejected.completed.is_none(),
        "invalid revision publishes no previous frames or render output"
    );

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment restores one-fit list");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    let recovered = recovered
        .completed
        .expect("recovery publishes complete layout");
    assert_ne!(recovered.key, initial_key, "recovery has a new revision");
    assert_eq!(
        recovered.frames, initial_frames,
        "recovery restores every frame"
    );
    assert!(recovered.render_snapshot.is_some());
}

fn source_for_track_list(track_list: &str) -> String {
    let scene = super::auto_minimum_scenes()
        .into_iter()
        .find(|scene| scene["fitPosition"].as_u64() == Some(1))
        .expect("one-fit source scene exists");
    let source = super::source_for(&scene, true);
    super::super::replace_once(
        &source,
        scene["computedAutoRows"].as_str().unwrap(),
        track_list,
    )
}

fn parse_track(track: &str) -> TrackSize {
    match track {
        "min-content" => TrackSize::MinContent,
        "max-content" => TrackSize::MaxContent,
        "auto" => TrackSize::Auto,
        value if value.starts_with("fit-content(") && value.ends_with(')') => {
            let cap = &value[12..value.len() - 1];
            let cap = if let Some(percent) = cap.strip_suffix('%') {
                TrackLength::Percentage(percent.parse::<f32>().unwrap() / 100.0)
            } else {
                TrackLength::LengthPx(cap.strip_suffix("px").unwrap().parse::<f32>().unwrap())
            };
            TrackSize::FitContent(cap)
        }
        other => panic!("unexpected track member: {other:?}"),
    }
}
