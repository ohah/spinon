use super::*;

const SOURCE: &str = include_str!(
    "../../../../../../../tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-runtime.html"
);
const BASE_TRACK_LIST: &str = "fit-content(20px) min-content max-content auto";
const PAYLOAD_HEIGHTS: [f32; 5] = [20.0, 10.0, 12.0, 6.0, 20.0];

fn baseline_source() -> String {
    let source = replace_once(SOURCE, "align-content: start;", "align-content: stretch;");
    let source = replace_once(
        &source,
        "#payload-1 { height: 60px;",
        "#payload-1 { height: 20px;",
    );
    replace_once(
        &source,
        "#payload-5 { height: 60px;",
        "#payload-5 { height: 20px;",
    )
}

fn permutations() -> Vec<[usize; 4]> {
    fn visit(prefix: &mut Vec<usize>, remaining: &mut Vec<usize>, output: &mut Vec<[usize; 4]>) {
        if remaining.is_empty() {
            output.push(
                prefix
                    .as_slice()
                    .try_into()
                    .expect("four track members should form a complete order"),
            );
            return;
        }

        for index in 0..remaining.len() {
            let member = remaining.remove(index);
            prefix.push(member);
            visit(prefix, remaining, output);
            prefix.pop();
            remaining.insert(index, member);
        }
    }

    let mut output = Vec::with_capacity(24);
    visit(&mut Vec::new(), &mut vec![0, 1, 2, 3], &mut output);
    output
}

fn member_names(cap: &str) -> [String; 4] {
    [
        format!("fit-content({cap})"),
        "min-content".to_owned(),
        "max-content".to_owned(),
        "auto".to_owned(),
    ]
}

fn overflow_source(order: [usize; 4], cap_css: &str, payload_index: usize) -> String {
    let source = baseline_source();
    let source = replace_once(&source, "align-content: stretch;", "align-content: start;");
    let names = member_names(cap_css);
    let track_list = order
        .iter()
        .map(|member| names[*member].as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let source = replace_once(&source, BASE_TRACK_LIST, &track_list);
    let height = PAYLOAD_HEIGHTS[payload_index] as u32;
    let old = format!("#payload-{} {{ height: {height}px;", payload_index + 1);
    let new = format!("#payload-{} {{ height: 60px;", payload_index + 1);
    replace_once(&source, &old, &new)
}

fn typed_members(
    cap: spinon_style::ComputedCssGridTrackLengthPercentage,
) -> [spinon_style::ComputedCssGridTrackSize; 4] {
    use spinon_style::ComputedCssGridTrackSize;

    [
        ComputedCssGridTrackSize::FitContent(cap),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
    ]
}

fn assert_scene(
    order: [usize; 4],
    cap_css: &str,
    cap: spinon_style::ComputedCssGridTrackLengthPercentage,
    align: &str,
) {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridAutoRows;

    let names = member_names(cap_css);
    let track_list = order
        .iter()
        .map(|member| names[*member].as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let base = baseline_source();
    let source = replace_once(&base, BASE_TRACK_LIST, &track_list);
    let source = replace_once(
        &source,
        "align-content: stretch;",
        &format!("align-content: {align};"),
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "order={order:?}, cap={cap_css}, align={align}: layout={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("supported four-member list should publish a complete layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("supported four-member list should publish a render snapshot");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the same revision should publish computed styles");
    assert_eq!(completed.key, cascade.key);
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("fixture should have one Grid owner");
    let members = typed_members(cap);
    let expected_tracks = order.map(|member| members[member].clone()).to_vec();
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(expected_tracks),
        "typed computed tracks should retain all four member identities and order"
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some(track_list.as_str()),
        "computed serialization should retain the authored member order"
    );

    let repeated_types = (0..PAYLOAD_HEIGHTS.len())
        .map(|index| order[index % order.len()])
        .collect::<Vec<_>>();
    let auto_tracks = repeated_types.iter().filter(|member| **member == 3).count();
    let stretch_per_auto = if align == "stretch" {
        22.0 / auto_tracks as f32
    } else {
        0.0
    };
    let row_heights = std::iter::once(10.0)
        .chain(PAYLOAD_HEIGHTS.iter().enumerate().map(|(index, height)| {
            height
                + if repeated_types[index] == 3 {
                    stretch_per_auto
                } else {
                    0.0
                }
        }))
        .collect::<Vec<_>>();
    let item_colors = [
        OpaqueCssSrgb::new(229, 72, 77),
        OpaqueCssSrgb::new(34, 197, 94),
        OpaqueCssSrgb::new(20, 184, 166),
        OpaqueCssSrgb::new(249, 115, 22),
        OpaqueCssSrgb::new(168, 85, 247),
        OpaqueCssSrgb::new(250, 204, 21),
    ];
    let mut y = 0.0;
    for (index, color) in item_colors.into_iter().enumerate() {
        let row_height = row_heights[index];
        assert_frame(
            frame_for_paint(scene, color),
            [0.0, y, 40.0, row_height],
            if index == 0 {
                "explicit item"
            } else {
                "implicit item"
            },
        );
        if index > 0 {
            let payload_color = [
                OpaqueCssSrgb::new(59, 130, 246),
                OpaqueCssSrgb::new(6, 182, 212),
                OpaqueCssSrgb::new(132, 204, 22),
                OpaqueCssSrgb::new(236, 72, 153),
                OpaqueCssSrgb::new(163, 230, 53),
            ][index - 1];
            assert_frame(
                frame_for_paint(scene, payload_color),
                [0.0, y, 40.0, PAYLOAD_HEIGHTS[index - 1]],
                "implicit payload",
            );
        }
        y += row_height;
    }
    assert!((y - if align == "stretch" { 100.0 } else { 78.0 }).abs() <= 0.02);
}

#[test]
fn parsed_root_html_four_member_intrinsic_lists_match_all_orders_caps_and_stretch_cases() {
    use spinon_style::ComputedCssGridTrackLengthPercentage::{LengthPx, Percentage};

    let orders = permutations();
    assert_eq!(orders.len(), 24);
    for order in orders {
        assert_scene(order, "20px", LengthPx(20.0), "start");
        assert_scene(order, "20px", LengthPx(20.0), "stretch");
        assert_scene(order, "20%", Percentage(0.2), "start");
        assert_scene(order, "20%", Percentage(0.2), "stretch");
    }
}

#[test]
fn parsed_root_html_four_member_fit_content_cap_keeps_track_size_and_visible_overflow() {
    use spinon_render::OpaqueCssSrgb;

    let orders = permutations();
    let item_colors = [
        OpaqueCssSrgb::new(34, 197, 94),
        OpaqueCssSrgb::new(20, 184, 166),
        OpaqueCssSrgb::new(249, 115, 22),
        OpaqueCssSrgb::new(168, 85, 247),
        OpaqueCssSrgb::new(250, 204, 21),
    ];
    let payload_colors = [
        OpaqueCssSrgb::new(59, 130, 246),
        OpaqueCssSrgb::new(6, 182, 212),
        OpaqueCssSrgb::new(132, 204, 22),
        OpaqueCssSrgb::new(236, 72, 153),
        OpaqueCssSrgb::new(163, 230, 53),
    ];

    for order in orders {
        let cap_position = order
            .iter()
            .position(|member| *member == 0)
            .expect("each order contains fit-content");
        let overflow_index = if cap_position == 0 { 4 } else { cap_position };

        for cap_css in ["20px", "20%"] {
            let source = overflow_source(order, cap_css, overflow_index);
            let (session, layout) = run(&source);
            assert_eq!(
                layout.state,
                crate::session::RuntimeLayoutState::Ready,
                "bounded fit-content overflow order={order:?}, cap={cap_css}: layout={:?}; cascade={:?}",
                layout.error,
                session.ua_cascade_snapshot().error
            );
            let scene = layout
                .completed
                .as_ref()
                .and_then(|completed| completed.render_snapshot.as_ref())
                .expect("cap overflow should publish a complete render snapshot");
            assert_frame(
                frame_for_paint(scene, OpaqueCssSrgb::new(23, 33, 43)),
                [0.0, 0.0, 40.0, 100.0],
                "Grid border box",
            );

            let mut rows = vec![10.0];
            rows.extend((0..PAYLOAD_HEIGHTS.len()).map(|index| {
                if order[index % order.len()] == 0 || index == overflow_index {
                    20.0
                } else {
                    PAYLOAD_HEIGHTS[index]
                }
            }));
            let item_y = rows[..=overflow_index].iter().sum::<f32>();
            assert_frame(
                frame_for_paint(scene, item_colors[overflow_index]),
                [0.0, item_y, 40.0, 20.0],
                "fit-content row remains capped",
            );
            let payload = frame_for_paint(scene, payload_colors[overflow_index]);
            assert_frame(
                payload,
                [0.0, item_y, 40.0, 60.0],
                "payload visibly overflows its capped row",
            );
            assert_eq!(
                payload[1] + payload[3] > 100.0,
                cap_position == 0 || cap_position == 3,
                "only the expected late cycle positions spill past the Grid: order={order:?}, cap={cap_css}"
            );
        }
    }
}

#[test]
fn parsed_root_html_four_member_fit_content_caps_keep_zero_and_over_100_percent_values() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::{
        ComputedCssGridAutoRows,
        ComputedCssGridTrackLengthPercentage::Percentage,
        ComputedCssGridTrackSize::{Auto, FitContent, MaxContent, MinContent},
    };

    let cases = [
        (
            "fit-content(0%) min-content max-content auto",
            Percentage(0.0),
            [10.0, 0.0, 10.0, 12.0, 6.0, 0.0],
        ),
        (
            "fit-content(120%) min-content max-content auto",
            Percentage(1.2),
            [10.0, 20.0, 10.0, 12.0, 6.0, 20.0],
        ),
    ];
    for (track_list, cap, rows) in cases {
        let baseline = baseline_source();
        let source = replace_once(
            &baseline,
            "align-content: stretch;",
            "align-content: start;",
        );
        let source = replace_once(&source, BASE_TRACK_LIST, track_list);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "finite non-negative boundary cap should be supported: {:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("a supported cap should publish a complete layout");
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("a supported cap should publish a render snapshot");
        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("a supported cap should publish computed styles");
        assert_eq!(completed.key, cascade.key);
        let grid = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("fixture should have one Grid owner");
        assert_eq!(
            grid.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(
                vec![FitContent(cap), MinContent, MaxContent, Auto,]
            )
        );
        assert_eq!(
            grid.properties.get("grid-auto-rows").map(String::as_str),
            Some(track_list)
        );

        let mut y = 0.0;
        for (index, row_height) in rows.into_iter().enumerate() {
            let color = [
                OpaqueCssSrgb::new(229, 72, 77),
                OpaqueCssSrgb::new(34, 197, 94),
                OpaqueCssSrgb::new(20, 184, 166),
                OpaqueCssSrgb::new(249, 115, 22),
                OpaqueCssSrgb::new(168, 85, 247),
                OpaqueCssSrgb::new(250, 204, 21),
            ][index];
            if row_height == 0.0 {
                assert!(
                    completed.frames.iter().any(|frame| {
                        [frame.x, frame.y, frame.width, frame.height] == [0.0, y, 40.0, row_height]
                    }),
                    "zero-height item should remain in the completed layout frames"
                );
            } else {
                assert_frame(
                    frame_for_paint(scene, color),
                    [0.0, y, 40.0, row_height],
                    "boundary cap Grid item",
                );
            }
            if index > 0 {
                let payload_color = [
                    OpaqueCssSrgb::new(59, 130, 246),
                    OpaqueCssSrgb::new(6, 182, 212),
                    OpaqueCssSrgb::new(132, 204, 22),
                    OpaqueCssSrgb::new(236, 72, 153),
                    OpaqueCssSrgb::new(163, 230, 53),
                ][index - 1];
                assert_frame(
                    frame_for_paint(scene, payload_color),
                    [0.0, y, 40.0, PAYLOAD_HEIGHTS[index - 1]],
                    "boundary cap payload overflow",
                );
            }
            y += row_height;
        }
    }
}

#[test]
fn parsed_root_html_four_member_overflow_payload_is_rejected_on_a_non_fit_content_track() {
    let source = replace_once(
        SOURCE,
        BASE_TRACK_LIST,
        "min-content fit-content(20px) max-content auto",
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
    assert!(layout.completed.is_none());
}

#[test]
fn parsed_root_html_four_member_list_accepts_an_absolute_calc_cap_after_px_normalization() {
    use spinon_style::{
        ComputedCssGridAutoRows,
        ComputedCssGridTrackLengthPercentage::LengthPx,
        ComputedCssGridTrackSize::{Auto, FitContent, MaxContent, MinContent},
    };

    let base = baseline_source();
    let source = replace_once(
        &base,
        BASE_TRACK_LIST,
        "fit-content(calc(12px + 8px)) min-content max-content auto",
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "absolute calc should normalize to a supported px cap: {:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("supported absolute calc should publish computed styles");
    assert_eq!(layout.completed.as_ref().unwrap().key, cascade.key);
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("fixture should have one Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            FitContent(LengthPx(20.0)),
            MinContent,
            MaxContent,
            Auto,
        ])
    );
    assert_eq!(
        grid.properties.get("grid-auto-rows").map(String::as_str),
        Some("fit-content(20px) min-content max-content auto")
    );
}

#[test]
fn parsed_root_html_four_member_intrinsic_lists_reject_nearby_profiles_atomically() {
    for (name, property, from, to) in [
        (
            "missing auto member",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "fit-content(20px) min-content max-content",
        ),
        (
            "duplicate intrinsic member",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "fit-content(20px) min-content min-content auto",
        ),
        (
            "duplicate auto member",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "fit-content(20px) min-content auto auto",
        ),
        (
            "extra fixed member",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "fit-content(20px) min-content max-content auto 8px",
        ),
        (
            "missing fit-content anchor",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "min-content max-content auto",
        ),
        (
            "unresolved mixed percentage calc cap",
            "grid-auto-rows",
            BASE_TRACK_LIST,
            "fit-content(calc(12px + 8%)) min-content max-content auto",
        ),
        (
            "indefinite percentage basis",
            "grid-auto-rows",
            "height: 100px;",
            "height: auto;",
        ),
        (
            "unsupported distribution",
            "align-content",
            "align-content: stretch;",
            "align-content: center;",
        ),
        (
            "extra visible direct item",
            "grid-auto-rows",
            "    </div>\n  </body>",
            "      <div class=\"item\" id=\"item-6\"></div>\n    </div>\n  </body>",
        ),
        (
            "payload height outside the exact fixture order",
            "height",
            "#payload-3 { height: 12px;",
            "#payload-3 { height: 10px;",
        ),
    ] {
        let base = baseline_source();
        let source = replace_once(&base, from, to);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name} should fail closed; layout={:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        assert_eq!(
            layout
                .error
                .as_ref()
                .and_then(|failure| failure.property.as_deref()),
            Some(property),
            "{name} should identify the rejected property"
        );
        assert!(
            layout.completed.is_none(),
            "{name} must not publish partial layout or render output"
        );
    }
}

#[test]
fn parsed_root_html_four_member_intrinsic_list_recovers_after_same_session_rejection() {
    let base = baseline_source();
    let source = replace_once(
        &base,
        BASE_TRACK_LIST,
        "auto fit-content(20%) min-content max-content",
    );
    let source = replace_once(
        &source,
        "</style>",
        "@media (prefers-color-scheme: dark) { #grid { grid-auto-rows: auto fit-content(20%) min-content min-content; } }\n</style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark media should activate the invalid duplicate member list");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light media should restore the valid four-member list");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery should publish the valid list: {:?}",
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
            spinon_render::OpaqueCssSrgb::new(250, 204, 21),
        ),
        [0.0, 69.0, 40.0, 31.0],
        "recovered fifth implicit item after repeated auto-row stretch",
    );
}
