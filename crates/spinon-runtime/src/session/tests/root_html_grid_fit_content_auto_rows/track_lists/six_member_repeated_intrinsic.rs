use super::*;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
};

const SOURCE: &str = include_str!(
    "../../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-runtime.html"
);
const BASE_TRACK_LIST: &str =
    "fit-content(20px) min-content min-content max-content max-content auto";

fn base_tail() -> [ComputedCssGridTrackSize; 5] {
    [
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
    ]
}

fn computed_tracks(cap: ComputedCssGridTrackLengthPercentage) -> Vec<ComputedCssGridTrackSize> {
    vec![
        ComputedCssGridTrackSize::FitContent(cap),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
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
            format!("fit-content({cap_css}) min-content min-content max-content max-content auto")
                .as_str()
        ),
        "computed serialization must retain the complete repeated sequence"
    );

    for (color, frame, name) in expected_frames_for_tail(align, &base_tail()) {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }
    let overflow = frame_for_paint(scene, OpaqueCssSrgb::new(251, 146, 60));
    assert!(
        overflow[1] + overflow[3] > 100.0,
        "the repeated fit-content payload should cross the Grid border"
    );
}

fn repeated_tail_orders() -> Vec<Vec<ComputedCssGridTrackSize>> {
    fn visit(
        remaining: &mut [usize; 3],
        current: &mut Vec<ComputedCssGridTrackSize>,
        orders: &mut Vec<Vec<ComputedCssGridTrackSize>>,
    ) {
        if current.len() == 5 {
            orders.push(current.clone());
            return;
        }

        for kind in 0..remaining.len() {
            if remaining[kind] == 0 {
                continue;
            }
            remaining[kind] -= 1;
            current.push(match kind {
                0 => ComputedCssGridTrackSize::MinContent,
                1 => ComputedCssGridTrackSize::MaxContent,
                _ => ComputedCssGridTrackSize::Auto,
            });
            visit(remaining, current, orders);
            current.pop();
            remaining[kind] += 1;
        }
    }

    let mut orders = Vec::new();
    visit(&mut [2, 2, 1], &mut Vec::new(), &mut orders);
    orders
}

fn repeated_tail_css(tail: &[ComputedCssGridTrackSize]) -> String {
    tail.iter()
        .map(|track| match track {
            ComputedCssGridTrackSize::MinContent => "min-content",
            ComputedCssGridTrackSize::MaxContent => "max-content",
            ComputedCssGridTrackSize::Auto => "auto",
            _ => panic!("repeated intrinsic tail contains only min/max/auto tracks"),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn computed_tracks_for_tail(
    cap: ComputedCssGridTrackLengthPercentage,
    tail: &[ComputedCssGridTrackSize],
) -> Vec<ComputedCssGridTrackSize> {
    let mut tracks = vec![ComputedCssGridTrackSize::FitContent(cap)];
    tracks.extend(tail.iter().cloned());
    tracks
}

fn expected_frames_for_tail(
    align: &str,
    tail: &[ComputedCssGridTrackSize],
) -> Vec<(OpaqueCssSrgb, [f32; 4], &'static str)> {
    let payload_heights = [20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 60.0, 6.0];
    let item_colors = [
        OpaqueCssSrgb::new(34, 197, 94),
        OpaqueCssSrgb::new(20, 184, 166),
        OpaqueCssSrgb::new(249, 115, 22),
        OpaqueCssSrgb::new(168, 85, 247),
        OpaqueCssSrgb::new(250, 204, 21),
        OpaqueCssSrgb::new(139, 92, 246),
        OpaqueCssSrgb::new(14, 165, 233),
        OpaqueCssSrgb::new(239, 68, 68),
    ];
    let payload_colors = [
        OpaqueCssSrgb::new(59, 130, 246),
        OpaqueCssSrgb::new(6, 182, 212),
        OpaqueCssSrgb::new(132, 204, 22),
        OpaqueCssSrgb::new(236, 72, 153),
        OpaqueCssSrgb::new(163, 230, 53),
        OpaqueCssSrgb::new(244, 114, 182),
        OpaqueCssSrgb::new(251, 146, 60),
        OpaqueCssSrgb::new(45, 212, 191),
    ];
    let auto_rows = (0..payload_heights.len())
        .filter(|row| *row % 6 != 0 && tail[*row % 6 - 1] == ComputedCssGridTrackSize::Auto)
        .count();
    let auto_stretch = if align == "stretch" {
        4.0 / auto_rows as f32
    } else {
        0.0
    };
    let mut frames = vec![
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
    ];
    let mut y = 10.0;
    for row in 0..payload_heights.len() {
        let payload_height: f32 = payload_heights[row];
        let track = if row % 6 == 0 {
            None
        } else {
            Some(&tail[row % 6 - 1])
        };
        let mut row_height = if track.is_none() {
            payload_height.min(20.0)
        } else {
            payload_height
        };
        if matches!(track, Some(ComputedCssGridTrackSize::Auto)) {
            row_height += auto_stretch;
        }
        frames.push((
            item_colors[row],
            [0.0, y, 40.0, row_height],
            "implicit row item",
        ));
        frames.push((
            payload_colors[row],
            [0.0, y, 40.0, payload_height],
            "implicit row payload",
        ));
        y += row_height;
    }
    frames
}

fn assert_scene_for_tail(
    cap_css: &str,
    cap: ComputedCssGridTrackLengthPercentage,
    align: &str,
    tail: &[ComputedCssGridTrackSize],
) {
    let tail_css = repeated_tail_css(tail);
    let track_list = format!("fit-content({cap_css}) {tail_css}");
    let source = replace_once(SOURCE, BASE_TRACK_LIST, &track_list);
    let source = replace_once(
        &source,
        "align-content: start;",
        &format!("align-content: {align};"),
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "tracks={track_list}, align={align}: layout={:?}; cascade={:?}",
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
        ComputedCssGridAutoRows::TrackList(computed_tracks_for_tail(cap, tail)),
        "computed member identity/order must preserve {track_list}"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some(track_list.as_str()),
        "computed serialization must retain {track_list}"
    );

    for (color, frame, name) in expected_frames_for_tail(align, tail) {
        assert_frame(frame_for_paint(scene, color), frame, name);
    }
    let overflow = frame_for_paint(scene, OpaqueCssSrgb::new(251, 146, 60));
    assert!(
        overflow[1] + overflow[3] > 100.0,
        "the repeated fit-content payload should cross the Grid border: {track_list}"
    );
}

#[test]
fn parsed_root_html_repeated_intrinsic_tail_all_30_orders_matches_independent_frames() {
    let orders = repeated_tail_orders();
    assert_eq!(orders.len(), 30);
    assert_eq!(
        orders
            .iter()
            .map(|tail| format!("{tail:?}"))
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        30,
        "the repeated-tail generator must produce 30 unique orders"
    );
    for tail in orders {
        for (cap_css, cap) in [
            ("20px", ComputedCssGridTrackLengthPercentage::LengthPx(20.0)),
            ("20%", ComputedCssGridTrackLengthPercentage::Percentage(0.2)),
        ] {
            for align in ["start", "stretch"] {
                assert_scene_for_tail(cap_css, cap.clone(), align, &tail);
            }
        }
    }
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
    let unsupported_dark_source = replace_once(
        SOURCE,
        "min-content max-content fit-content(20px) auto min-content max-content",
        "min-content max-content fit-content(20px) auto auto max-content",
    );
    let (session, initial) = run(&unsupported_dark_source);
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
        expected_frames_for_tail("start", &base_tail())
            .into_iter()
            .find_map(|(color, frame, _)| {
                (color == OpaqueCssSrgb::new(251, 146, 60)).then_some(frame)
            })
            .expect("independent model includes the repeated fit-content payload"),
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

#[test]
fn parsed_root_html_repeated_intrinsic_list_rejects_mixed_numeric_payload_profiles() {
    let source = replace_once(
        SOURCE,
        "#payload-3 { height: 12px;",
        "#payload-3 { height: 1px;",
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
        "payload sizes from different fixture profiles must not be combined"
    );
}

#[test]
fn parsed_root_html_later_fit_content_position_requires_the_repeated_text_profile() {
    let source = replace_once(
        SOURCE,
        "fit-content(20px) min-content min-content max-content max-content auto",
        "min-content fit-content(20px) min-content max-content max-content auto",
    );
    let (_, layout) = run(&source);
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        layout
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(
        layout.completed.is_none(),
        "later fit-content positions require the bounded direct-text auto-placement profile"
    );
}
