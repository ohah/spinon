use super::*;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
};

const SOURCE: &str = include_str!(
    "../../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-runtime.html"
);

fn computed_tracks(cap: ComputedCssGridTrackLengthPercentage) -> Vec<ComputedCssGridTrackSize> {
    vec![
        ComputedCssGridTrackSize::FitContent(cap),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ]
}

fn expected_frames(align: &str) -> Vec<(OpaqueCssSrgb, [f32; 4], &'static str)> {
    let auto_height = if align == "stretch" { 10.0 } else { 6.0 };
    let repeated_start = if align == "stretch" { 74.0 } else { 70.0 };
    let last_start = if align == "stretch" { 94.0 } else { 90.0 };
    vec![
        (
            OpaqueCssSrgb::new(23, 33, 43),
            [0.0, 0.0, 40.0, 100.0],
            "Grid",
        ),
        (
            OpaqueCssSrgb::new(229, 72, 77),
            [0.0, 0.0, 40.0, 10.0],
            "explicit row item",
        ),
        (
            OpaqueCssSrgb::new(34, 197, 94),
            [0.0, 10.0, 40.0, 20.0],
            "first fit-content item",
        ),
        (
            OpaqueCssSrgb::new(59, 130, 246),
            [0.0, 10.0, 40.0, 20.0],
            "first fit-content payload",
        ),
        (
            OpaqueCssSrgb::new(20, 184, 166),
            [0.0, 30.0, 40.0, 10.0],
            "first min-content item",
        ),
        (
            OpaqueCssSrgb::new(6, 182, 212),
            [0.0, 30.0, 40.0, 10.0],
            "first min-content payload",
        ),
        (
            OpaqueCssSrgb::new(249, 115, 22),
            [0.0, 40.0, 40.0, 12.0],
            "first max-content item",
        ),
        (
            OpaqueCssSrgb::new(132, 204, 22),
            [0.0, 40.0, 40.0, 12.0],
            "first max-content payload",
        ),
        (
            OpaqueCssSrgb::new(168, 85, 247),
            [0.0, 52.0, 40.0, auto_height],
            "auto item",
        ),
        (
            OpaqueCssSrgb::new(236, 72, 153),
            [0.0, 52.0, 40.0, 6.0],
            "auto payload",
        ),
        (
            OpaqueCssSrgb::new(250, 204, 21),
            [0.0, 52.0 + auto_height, 40.0, 6.0],
            "repeated min-content item",
        ),
        (
            OpaqueCssSrgb::new(163, 230, 53),
            [0.0, 52.0 + auto_height, 40.0, 6.0],
            "repeated min-content payload",
        ),
        (
            OpaqueCssSrgb::new(139, 92, 246),
            [0.0, 58.0 + auto_height, 40.0, 6.0],
            "repeated max-content item",
        ),
        (
            OpaqueCssSrgb::new(244, 114, 182),
            [0.0, 58.0 + auto_height, 40.0, 6.0],
            "repeated max-content payload",
        ),
        (
            OpaqueCssSrgb::new(14, 165, 233),
            [0.0, repeated_start, 40.0, 20.0],
            "cycled fit-content item",
        ),
        (
            OpaqueCssSrgb::new(251, 146, 60),
            [0.0, repeated_start, 40.0, 60.0],
            "cycled fit-content payload",
        ),
        (
            OpaqueCssSrgb::new(239, 68, 68),
            [0.0, last_start, 40.0, 6.0],
            "cycled min-content item",
        ),
        (
            OpaqueCssSrgb::new(45, 212, 191),
            [0.0, last_start, 40.0, 6.0],
            "cycled min-content payload",
        ),
    ]
}

fn assert_scene(cap_css: &str, cap: ComputedCssGridTrackLengthPercentage, align: &str) {
    let source = replace_once(
        SOURCE,
        "fit-content(20px)",
        &format!("fit-content({cap_css})"),
    );
    let source = replace_once(
        &source,
        "align-content: start;",
        &format!("align-content: {align};"),
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "cap={cap_css}, align={align}: layout={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("supported list should publish layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("supported list should publish render scene");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("computed styles should publish");
    assert_eq!(completed.key, cascade.key);

    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("fixture should have one Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(cap)),
        "computed track identity/order must preserve both repeated members"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some(
            format!("fit-content({cap_css}) min-content max-content auto min-content max-content")
                .as_str()
        ),
        "computed serialization must retain the complete repeated sequence"
    );

    for (color, frame, name) in expected_frames(align) {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }
    let overflow = frame_for_paint(scene, OpaqueCssSrgb::new(251, 146, 60));
    assert!(
        overflow[1] + overflow[3] > 100.0,
        "the repeated fit-content payload should cross the Grid border"
    );
}

#[test]
fn parsed_root_html_repeated_intrinsic_list_preserves_both_cycles_and_auto_stretch() {
    for (cap_css, cap) in [
        ("20px", ComputedCssGridTrackLengthPercentage::LengthPx(20.0)),
        ("20%", ComputedCssGridTrackLengthPercentage::Percentage(0.2)),
    ] {
        assert_scene(cap_css, cap.clone(), "start");
        assert_scene(cap_css, cap, "stretch");
    }
}

#[test]
fn parsed_root_html_repeated_intrinsic_list_rejects_atomically_and_recovers_same_session() {
    let (session, initial) = run(SOURCE);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());
    let grid_node_id = session
        .ua_cascade_snapshot()
        .completed
        .expect("supported initial revision should publish computed styles")
        .roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("fixture should have one Grid owner")
        .node_id
        .get();

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark mode should apply the unsupported six-member order");
    let rejected = session.wait_for_layout_snapshot(std::time::Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert_eq!(
        rejected.error.as_ref().and_then(|failure| failure.node_id),
        Some(grid_node_id)
    );
    assert!(
        rejected.completed.is_none(),
        "unsupported order must not publish partial output"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light mode should restore the supported exact order");
    let recovered = session.wait_for_layout_snapshot(std::time::Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should succeed: {:?}",
        recovered.error
    );
    let recovered_scene = recovered
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("recovery should publish a complete render scene");
    assert_frame(
        frame_for_paint(recovered_scene, OpaqueCssSrgb::new(251, 146, 60)),
        [0.0, 70.0, 40.0, 60.0],
        "recovered repeated fit-content overflow payload",
    );
}

#[test]
fn parsed_root_html_repeated_intrinsic_list_rejects_overflow_payload_at_first_fit_position() {
    let source = replace_once(
        SOURCE,
        "#payload-1 { height: 20px;",
        "#payload-1 { height: 60px;",
    );
    let (_, layout) = run(&source);
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        layout
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("height")
    );
    assert!(
        layout.completed.is_none(),
        "the first fit-content item with the repeated-cycle overflow height must fail atomically"
    );
}
