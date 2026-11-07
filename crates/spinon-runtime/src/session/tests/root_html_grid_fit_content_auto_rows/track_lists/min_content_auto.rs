use super::*;

const SOURCE: &str = include_str!(
    "../../../../../../../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-track-list-runtime.html"
);

fn source_with_scene(track_list: &str) -> String {
    let source = replace_once(SOURCE, "fit-content(120%) min-content auto", track_list);
    let source = replace_once(
        &source,
        "#payload-1 { height: 6px;",
        "#payload-1 { height: 34px;",
    );
    replace_once(
        &source,
        "#payload-4 { height: 50px;",
        "#payload-4 { height: 6px;",
    )
}

fn assert_scene(
    source: &str,
    track_list: &str,
    tracks: Vec<spinon_style::ComputedCssGridTrackSize>,
    row_heights: [f32; 5],
    payload_heights: [f32; 4],
) {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridAutoRows;

    let (session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{track_list} should publish a complete layout: layout error={:?}; cascade error={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("supported mixed intrinsic list should publish completed layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("supported mixed intrinsic list should publish a render snapshot");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the layout revision should have completed computed styles");
    assert_eq!(completed.key, cascade.key);
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("the fixture should have one Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(tracks),
        "computed Grid track members should keep their typed order"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some(track_list),
        "computed serialization should keep the full author track list"
    );

    let item_colors = [
        (OpaqueCssSrgb::new(229, 72, 77), "explicit item"),
        (OpaqueCssSrgb::new(34, 197, 94), "first implicit item"),
        (OpaqueCssSrgb::new(20, 184, 166), "second implicit item"),
        (OpaqueCssSrgb::new(249, 115, 22), "third implicit item"),
        (OpaqueCssSrgb::new(168, 85, 247), "fourth implicit item"),
    ];
    let mut y = 0.0;
    for (index, (color, label)) in item_colors.into_iter().enumerate() {
        assert_frame(
            frame_for_paint(scene, color),
            [0.0, y, 40.0, row_heights[index]],
            label,
        );
        y += row_heights[index];
    }

    let payload_colors = [
        OpaqueCssSrgb::new(59, 130, 246),
        OpaqueCssSrgb::new(6, 182, 212),
        OpaqueCssSrgb::new(132, 204, 22),
        OpaqueCssSrgb::new(236, 72, 153),
    ];
    let mut y = row_heights[0];
    for (index, color) in payload_colors.into_iter().enumerate() {
        assert_frame(
            frame_for_paint(scene, color),
            [0.0, y, 40.0, payload_heights[index]],
            "nested payload",
        );
        y += row_heights[index + 1];
    }
}

#[test]
fn parsed_root_html_fit_content_min_content_auto_lists_match_both_orders_and_overflow() {
    use spinon_style::{
        ComputedCssGridTrackLengthPercentage::Percentage,
        ComputedCssGridTrackSize::{Auto, FitContent, MinContent},
    };

    assert_scene(
        &source_with_scene("fit-content(20%) min-content auto"),
        "fit-content(20%) min-content auto",
        vec![FitContent(Percentage(0.2)), MinContent, Auto],
        [10.0, 20.0, 34.0, 30.0, 6.0],
        [34.0, 34.0, 10.0, 6.0],
    );
    assert_scene(
        &source_with_scene("fit-content(20px) min-content auto"),
        "fit-content(20px) min-content auto",
        vec![
            FitContent(spinon_style::ComputedCssGridTrackLengthPercentage::LengthPx(20.0)),
            MinContent,
            Auto,
        ],
        [10.0, 20.0, 34.0, 30.0, 6.0],
        [34.0, 34.0, 10.0, 6.0],
    );
    assert_scene(
        &source_with_scene("fit-content(20%) auto min-content"),
        "fit-content(20%) auto min-content",
        vec![FitContent(Percentage(0.2)), Auto, MinContent],
        [10.0, 20.0, 54.0, 10.0, 6.0],
        [34.0, 34.0, 10.0, 6.0],
    );
    assert_scene(
        SOURCE,
        "fit-content(120%) min-content auto",
        vec![FitContent(Percentage(1.2)), MinContent, Auto],
        [10.0, 6.0, 34.0, 10.0, 40.0],
        [6.0, 34.0, 10.0, 50.0],
    );
}

#[test]
fn parsed_root_html_fit_content_min_content_auto_rejects_nearby_profiles_atomically() {
    for (name, from, to) in [
        (
            "duplicate intrinsic member",
            "fit-content(120%) min-content auto",
            "fit-content(20%) min-content min-content",
        ),
        (
            "extra fixed member",
            "fit-content(120%) min-content auto",
            "fit-content(20%) auto min-content 8px",
        ),
        (
            "unmodeled max-content member",
            "fit-content(120%) min-content auto",
            "fit-content(20%) max-content auto",
        ),
        (
            "missing fit-content anchor",
            "fit-content(120%) min-content auto",
            "min-content auto",
        ),
        (
            "unsupported distribution",
            "align-content: stretch;",
            "align-content: center;",
        ),
    ] {
        let source = replace_once(SOURCE, from, to);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name} should fail closed: layout={:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        assert!(
            layout.completed.is_none(),
            "{name} must not publish partial layout or render output"
        );
    }
}

#[test]
fn parsed_root_html_fit_content_min_content_auto_recovers_in_the_same_session() {
    let source = replace_once(
        SOURCE,
        "</style>",
        "@media (prefers-color-scheme: dark) { #grid { grid-auto-rows: fit-content(20%) min-content min-content; } }\n</style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark media should activate the unsupported repeated intrinsic member");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light media should restore the supported exact pattern");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should publish a complete layout: {:?}",
        recovered.error
    );
    let recovered_scene = recovered
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("recovery should publish a complete render snapshot");
    assert_frame(
        frame_for_paint(
            recovered_scene,
            spinon_render::OpaqueCssSrgb::new(168, 85, 247),
        ),
        [0.0, 60.0, 40.0, 40.0],
        "recovered fourth implicit row",
    );
}
