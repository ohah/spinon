use super::*;

const SOURCE_LIST: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-list-runtime.html"
);

#[test]
fn parsed_root_html_fit_content_auto_row_lists_preserve_order_and_render_repeated_tracks() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::{
        ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    };

    let (session, layout) = run(SOURCE_LIST);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "list layout error={:?}; cascade error={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("supported list should publish a complete layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("supported list should publish a render snapshot");

    for (color, frame, label) in [
        (
            OpaqueCssSrgb::new(229, 72, 77),
            [0.0, 0.0, 40.0, 10.0],
            "explicit item",
        ),
        (
            OpaqueCssSrgb::new(34, 197, 94),
            [0.0, 10.0, 40.0, 20.0],
            "first cyclic item",
        ),
        (
            OpaqueCssSrgb::new(20, 184, 166),
            [0.0, 30.0, 40.0, 12.0],
            "fixed-size track item",
        ),
        (
            OpaqueCssSrgb::new(249, 115, 22),
            [0.0, 42.0, 40.0, 10.0],
            "percentage track item",
        ),
        (
            OpaqueCssSrgb::new(168, 85, 247),
            [0.0, 52.0, 40.0, 20.0],
            "repeated first track item",
        ),
        (
            OpaqueCssSrgb::new(59, 130, 246),
            [0.0, 10.0, 40.0, 34.0],
            "first overflow payload",
        ),
        (
            OpaqueCssSrgb::new(6, 182, 212),
            [0.0, 30.0, 40.0, 10.0],
            "fixed payload",
        ),
        (
            OpaqueCssSrgb::new(132, 204, 22),
            [0.0, 42.0, 40.0, 8.0],
            "percentage payload",
        ),
        (
            OpaqueCssSrgb::new(236, 72, 153),
            [0.0, 52.0, 40.0, 34.0],
            "repeated overflow payload",
        ),
    ] {
        assert_frame(frame_for_paint(scene, color), frame, label);
    }

    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the same revision should publish computed styles");
    assert_eq!(
        completed.key, cascade.key,
        "layout and computed list must publish from the same revision"
    );
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("the fixture has one Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                0.2
            ),),
            ComputedCssGridTrackSize::LengthPx(12.0),
            ComputedCssGridTrackSize::Percentage(0.1),
        ]),
        "computed typed tracks must retain author order"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some("fit-content(20%) 12px 10%"),
        "computed serialization must retain the complete track list"
    );

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark environment should activate the unsupported list member");
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
        "an unsupported list member must not publish a partial layout or render snapshot"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light environment should restore the supported list");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session list recovery should succeed: {:?}",
        recovered.error
    );
    let recovered_scene = recovered
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("recovery should publish a complete render snapshot");
    assert_frame(
        frame_for_paint(recovered_scene, OpaqueCssSrgb::new(168, 85, 247)),
        [0.0, 52.0, 40.0, 20.0],
        "recovered repeated track item",
    );

    let floor_source = replace_once(
        &replace_once(
            &replace_once(
                &replace_once(
                    SOURCE_LIST,
                    "grid-auto-rows: fit-content(20%) 12px 10%;",
                    "grid-auto-rows: fit-content(20%) fit-content(40%);",
                ),
                ".item-a { background-color: #22c55e; }",
                ".item-a { min-height: 28px; background-color: #22c55e; }",
            ),
            "  <div id=\"item-b\" class=\"item item-b\"><div id=\"payload-b\" class=\"payload payload-b\"></div></div>\n",
            "",
        ),
        "  <div id=\"item-c\" class=\"item item-c\"><div id=\"payload-c\" class=\"payload payload-c\"></div></div>\n",
        "",
    );
    let (floor_session, floor_layout) = run(&floor_source);
    assert_eq!(
        floor_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "minimum floor list error={:?}; cascade error={:?}",
        floor_layout.error,
        floor_session.ua_cascade_snapshot().error
    );
    let floor_scene = floor_layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("minimum floor list should publish a render snapshot");
    for (color, frame, label) in [
        (
            OpaqueCssSrgb::new(34, 197, 94),
            [0.0, 10.0, 40.0, 28.0],
            "minimum floor item",
        ),
        (
            OpaqueCssSrgb::new(59, 130, 246),
            [0.0, 10.0, 40.0, 34.0],
            "minimum floor overflow payload",
        ),
        (
            OpaqueCssSrgb::new(168, 85, 247),
            [0.0, 38.0, 40.0, 34.0],
            "following fit-content row",
        ),
    ] {
        assert_frame(frame_for_paint(floor_scene, color), frame, label);
    }

    let zero_source = replace_once(
        &replace_once(
            &floor_source,
            "grid-auto-rows: fit-content(20%) fit-content(40%);",
            "grid-auto-rows: fit-content(0%) fit-content(120%);",
        ),
        ".item-a { min-height: 28px; background-color: #22c55e; }",
        ".item-a { background-color: #22c55e; }",
    );
    let (_, zero_layout) = run(&zero_source);
    assert_eq!(
        zero_layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "zero and over-100% caps should remain valid: {:?}",
        zero_layout.error
    );
    let zero_completed = zero_layout
        .completed
        .as_ref()
        .expect("zero and over-100% caps should publish a complete layout");
    assert!(
        zero_completed.frames.iter().any(|frame| {
            [frame.x, frame.y, frame.width, frame.height] == [0.0, 10.0, 40.0, 0.0]
        }),
        "zero cap should retain the first implicit Grid item with a zero-height frame"
    );
    let zero_scene = zero_completed
        .render_snapshot
        .as_ref()
        .expect("zero and over-100% caps should publish a render snapshot");
    assert_frame(
        frame_for_paint(zero_scene, OpaqueCssSrgb::new(59, 130, 246)),
        [0.0, 10.0, 40.0, 34.0],
        "zero-cap visible overflow payload",
    );
    assert_frame(
        frame_for_paint(zero_scene, OpaqueCssSrgb::new(168, 85, 247)),
        [0.0, 10.0, 40.0, 34.0],
        "over-100% cap above max-content",
    );
}
