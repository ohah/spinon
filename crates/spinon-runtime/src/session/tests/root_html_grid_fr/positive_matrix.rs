use super::*;

#[test]
fn c11_1_root_html_default_runtime_lays_out_fraction_tracks_from_author_css() {
    use spinon_render::OpaqueCssSrgb;

    let column_1_2_items = fixed_items(&[("a", "30", "40"), ("b", "30", "40")]);
    let column_2_1_items = fixed_items(&[("a", "30", "40"), ("b", "30", "40")]);
    let mixed_track_items = fixed_items(&[("a", "20", "40"), ("b", "20", "40"), ("c", "20", "40")]);
    let row_1_2_items = fixed_items(&[("a", "40", "10"), ("b", "40", "10")]);
    let subunit_sum_items = fixed_items(&[("a", "10", "40"), ("b", "10", "40")]);
    let zero_factor_items = fixed_items(&[("a", "0", "40"), ("b", "30", "40")]);
    let large_factor_items =
        fixed_items(&[("a", "20", "40"), ("b", "20", "40"), ("c", "20", "40")]);
    let overflow_factor_items = fixed_items(&[
        ("a", "20", "40"),
        ("b", "20", "40"),
        ("c", "20", "40"),
        ("d", "20", "40"),
    ]);
    let reversed_items =
        fixed_items(&[("semantic-blue", "30", "40"), ("semantic-red", "30", "40")]);
    let cases = [
        (
            "column-1-2",
            "130px",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "",
            column_1_2_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 30.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [50.0, 0.0, 30.0, 40.0]),
            ],
        ),
        (
            "column-2-1",
            "130px",
            "40px",
            "2fr 1fr",
            "40px",
            "0 10px",
            "",
            column_2_1_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 30.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [90.0, 0.0, 30.0, 40.0]),
            ],
        ),
        (
            "mixed-tracks",
            "160px",
            "40px",
            "30px 1fr 2fr",
            "40px",
            "0 10px",
            "",
            mixed_track_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [40.0, 0.0, 20.0, 40.0]),
                (
                    OpaqueCssSrgb::new(22, 163, 74),
                    [260.0 / 3.0, 0.0, 20.0, 40.0],
                ),
            ],
        ),
        (
            "row-1-2",
            "40px",
            "130px",
            "40px",
            "1fr 2fr",
            "10px 0",
            "",
            row_1_2_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 40.0, 10.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 50.0, 40.0, 10.0]),
            ],
        ),
        (
            "subunit-sum",
            "110px",
            "40px",
            "0.25fr 0.25fr",
            "40px",
            "0 10px",
            "",
            subunit_sum_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 10.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [35.0, 0.0, 10.0, 40.0]),
            ],
        ),
        (
            "zero-factor",
            "120px",
            "40px",
            "0fr 1fr",
            "40px",
            "0",
            "",
            zero_factor_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 0.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 0.0, 30.0, 40.0]),
            ],
        ),
        (
            "large-factor",
            "120px",
            "40px",
            "1e38fr 1e38fr 1e38fr",
            "40px",
            "0",
            "",
            large_factor_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [40.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(22, 163, 74), [80.0, 0.0, 20.0, 40.0]),
            ],
        ),
        (
            "large-factor-overflow-spec-result",
            "120px",
            "40px",
            "1e38fr 1e38fr 1e38fr 1e38fr",
            "40px",
            "0",
            "",
            overflow_factor_items,
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [30.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(22, 163, 74), [60.0, 0.0, 20.0, 40.0]),
                (OpaqueCssSrgb::new(245, 158, 11), [90.0, 0.0, 20.0, 40.0]),
            ],
        ),
        (
            "renamed-reversed",
            "130px",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "",
            reversed_items,
            vec![
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 0.0, 30.0, 40.0]),
                (OpaqueCssSrgb::new(229, 72, 77), [50.0, 0.0, 30.0, 40.0]),
            ],
        ),
    ];

    for (name, width, height, columns, rows, gap, extra, items, expected_items) in cases {
        let (session, layout) = run(&source(width, height, columns, rows, gap, extra, &items));
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{name}: error={:?}; cascade state={:?}",
            layout.error,
            session.ua_cascade_snapshot().state
        );
        let completed = layout
            .completed
            .expect("valid fraction Grid should complete through the default route");
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("valid fraction Grid should publish its render scene");
        assert_frame(
            frame_for_paint(scene, OpaqueCssSrgb::new(16, 24, 32)),
            [
                0.0,
                0.0,
                width.trim_end_matches("px").parse().unwrap(),
                height.trim_end_matches("px").parse().unwrap(),
            ],
            name,
        );
        let cascade = session.ua_cascade_snapshot();
        let cascade_completed = cascade
            .completed
            .expect("computed tracks should complete in the same style revision");
        assert_eq!(
            completed.key, cascade_completed.key,
            "{name} style revision"
        );
        for (color, expected) in expected_items {
            let frame = if expected[2] == 0.0 || expected[3] == 0.0 {
                let element = cascade_completed.roots[0]
                    .styles
                    .elements
                    .iter()
                    .find(|element| {
                        element.background_color.is_some_and(|background| {
                            background.red == color.red()
                                && background.green == color.green()
                                && background.blue == color.blue()
                        })
                    })
                    .expect("zero-area semantic Grid item should retain its computed paint");
                let frame = completed
                    .frames
                    .iter()
                    .find(|frame| frame.node_id == element.node_id.get())
                    .expect("zero-area semantic Grid item should retain its layout frame");
                [frame.x, frame.y, frame.width, frame.height]
            } else {
                frame_for_paint(scene, color)
            };
            assert_frame(frame, expected, name);
        }
        assert!(
            cascade_completed.roots[0]
                .styles
                .elements
                .iter()
                .any(|element| {
                    element.grid_layout.as_ref().is_some_and(|grid| {
                        grid.columns.iter().chain(&grid.rows).any(|track| {
                            matches!(track, spinon_style::ComputedCssGridTrackSize::Fraction(_))
                        })
                    })
                }),
            "{name} should keep the author's typed fraction tracks"
        );
    }
}

fn assert_percentage_scene(
    name: &str,
    source_html: &str,
    expected_root: [f32; 4],
    expected_columns: &[spinon_style::ComputedCssGridTrackSize],
    expected_rows: &[spinon_style::ComputedCssGridTrackSize],
    expected_items: &[(spinon_render::OpaqueCssSrgb, [f32; 4])],
) {
    let (session, layout) = run(source_html);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{name}: error={:?}; cascade state={:?}",
        layout.error,
        session.ua_cascade_snapshot().state
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("valid percentage Grid should publish a complete layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("valid percentage Grid should publish its paint scene");
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("percentage style snapshot should complete");
    assert_eq!(completed.key, cascade_completed.key, "{name} revision");

    let grid_element = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|element| element.grid_layout.is_some())
        .expect("the parsed Root HTML source should retain its Grid container");
    let grid = grid_element
        .grid_layout
        .as_ref()
        .expect("the Root HTML Grid should retain typed tracks");
    assert_eq!(grid.columns, expected_columns, "{name} computed columns");
    assert_eq!(grid.rows, expected_rows, "{name} computed rows");
    let root_frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == grid_element.node_id.get())
        .expect("the Grid root should retain its layout frame");
    assert_frame(
        [
            root_frame.x,
            root_frame.y,
            root_frame.width,
            root_frame.height,
        ],
        expected_root,
        name,
    );

    for (color, expected) in expected_items {
        let element = cascade_completed.roots[0]
            .styles
            .elements
            .iter()
            .find(|element| {
                element.background_color.is_some_and(|background| {
                    background.red == color.red()
                        && background.green == color.green()
                        && background.blue == color.blue()
                })
            })
            .unwrap_or_else(|| panic!("{name}: missing semantic item color {color:?}"));
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == element.node_id.get())
            .unwrap_or_else(|| panic!("{name}: missing semantic item frame {color:?}"));
        let actual = [frame.x, frame.y, frame.width, frame.height];
        assert_frame(actual, *expected, name);
        assert_frame(frame_for_paint(scene, *color), *expected, name);
    }
}

#[test]
fn c11_1_root_html_default_runtime_lays_out_percentage_tracks_from_author_css() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridTrackSize::{Fraction, LengthPx, Percentage};

    let both_axes_items = fixed_items(&[
        ("a", "8", "6"),
        ("b", "8", "6"),
        ("c", "8", "6"),
        ("d", "8", "6"),
        ("e", "8", "6"),
        ("f", "8", "6"),
        ("g", "8", "6"),
        ("h", "8", "6"),
        ("i", "8", "6"),
    ]);
    let marker_pair = fixed_items(&[("a", "8", "6"), ("b", "8", "6")]);
    let cases = [
        (
            "both-axes-percent-px",
            source(
                "240px",
                "120px",
                "25% 50% 20px",
                "25% 50% 10px",
                "5px 10px",
                "",
                &both_axes_items,
            ),
            [0.0, 0.0, 240.0, 120.0],
            vec![Percentage(0.25), Percentage(0.5), LengthPx(20.0)],
            vec![Percentage(0.25), Percentage(0.5), LengthPx(10.0)],
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [70.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(22, 163, 74), [200.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(245, 158, 11), [0.0, 35.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(168, 85, 247), [70.0, 35.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(234, 179, 8), [200.0, 35.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(20, 184, 166), [0.0, 100.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(236, 72, 153), [70.0, 100.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(148, 163, 184), [200.0, 100.0, 8.0, 6.0]),
            ],
        ),
        (
            "percentage-plus-fr-and-gap",
            source(
                "240px",
                "40px",
                "25% 1fr",
                "100%",
                "0 10px",
                "",
                &marker_pair,
            ),
            [0.0, 0.0, 240.0, 40.0],
            vec![Percentage(0.25), Fraction(1.0)],
            vec![Percentage(1.0)],
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [70.0, 0.0, 8.0, 6.0]),
            ],
        ),
        (
            "overfull-visible-overflow",
            source(
                "200px",
                "40px",
                "120% 10%",
                "100%",
                "0 8px",
                "",
                &marker_pair,
            ),
            [0.0, 0.0, 200.0, 40.0],
            vec![Percentage(1.2), Percentage(0.1)],
            vec![Percentage(1.0)],
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [248.0, 0.0, 8.0, 6.0]),
            ],
        ),
        (
            "zero-and-full-percentage",
            source("200px", "40px", "0% 100%", "100%", "0", "", &marker_pair),
            [0.0, 0.0, 200.0, 40.0],
            vec![Percentage(0.0), Percentage(1.0)],
            vec![Percentage(1.0)],
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 0.0, 8.0, 6.0]),
            ],
        ),
        (
            "zero-definite-basis",
            source("0px", "40px", "50% 50%", "100%", "0", "", &marker_pair),
            [0.0, 0.0, 0.0, 40.0],
            vec![Percentage(0.5), Percentage(0.5)],
            vec![Percentage(1.0)],
            vec![
                (OpaqueCssSrgb::new(229, 72, 77), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 0.0, 8.0, 6.0]),
            ],
        ),
        (
            "selector-rewrite-reversed-source-order",
            source(
                "240px",
                "40px",
                "25% 75%",
                "100%",
                "0",
                "",
                &fixed_items(&[("semantic-blue", "8", "6"), ("semantic-red", "8", "6")]),
            ),
            [0.0, 0.0, 240.0, 40.0],
            vec![Percentage(0.25), Percentage(0.75)],
            vec![Percentage(1.0)],
            vec![
                (OpaqueCssSrgb::new(59, 130, 246), [0.0, 0.0, 8.0, 6.0]),
                (OpaqueCssSrgb::new(229, 72, 77), [60.0, 0.0, 8.0, 6.0]),
            ],
        ),
    ];

    for (name, source_html, root, columns, rows, items) in cases {
        assert_percentage_scene(name, &source_html, root, &columns, &rows, &items);
    }
}

#[test]
fn c11_1_root_html_default_runtime_uses_percentage_simulator_source_and_keeps_visible_overflow() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridTrackSize::Percentage;

    const SOURCE: &str = include_str!(
        "../../../../../../tests/fixtures/css/c11/root-html-grid-percentage-runtime.html"
    );
    let (session, layout) = run(SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("the exact simulator HTML should complete through default Root HTML");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the simulator HTML should publish a normal paint scene");
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("the same simulator HTML should publish computed styles");
    assert_eq!(
        completed.key, cascade_completed.key,
        "style/layout revision"
    );

    let elements = &cascade_completed.roots[0].styles.elements;
    let overflow_grid = elements
        .iter()
        .find(|element| {
            element.grid_layout.as_ref().is_some_and(|grid| {
                grid.columns == [Percentage(1.2), Percentage(0.1)] && grid.rows == [Percentage(1.0)]
            })
        })
        .expect("the overflow Grid should preserve its typed percentage tracks");
    let grid = overflow_grid
        .grid_layout
        .as_ref()
        .expect("typed overflow Grid tracks should be retained");
    assert_eq!(grid.columns, [Percentage(1.2), Percentage(0.1)]);
    assert_eq!(grid.rows, [Percentage(1.0)]);
    let overflow_root_frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == overflow_grid.node_id.get())
        .expect("the overflow Grid should have a root frame");
    assert_frame(
        [
            overflow_root_frame.x,
            overflow_root_frame.y,
            overflow_root_frame.width,
            overflow_root_frame.height,
        ],
        [0.0, 120.0, 200.0, 40.0],
        "simulator-overflow-root",
    );

    let cases = [
        (OpaqueCssSrgb::new(245, 158, 11), [0.0, 120.0, 8.0, 6.0]),
        (OpaqueCssSrgb::new(168, 85, 247), [248.0, 120.0, 8.0, 6.0]),
    ];
    for (color, expected) in cases {
        let element = elements
            .iter()
            .find(|element| {
                element.background_color.is_some_and(|background| {
                    background.red == color.red()
                        && background.green == color.green()
                        && background.blue == color.blue()
                })
            })
            .expect("semantic overflow markers should keep distinct paint colors");
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == element.node_id.get())
            .expect("semantic overflow markers should keep layout frames");
        assert_frame(
            [frame.x, frame.y, frame.width, frame.height],
            expected,
            "simulator-overflow-item",
        );
        assert_frame(
            frame_for_paint(scene, color),
            expected,
            "simulator-overflow-paint",
        );
    }

    assert!(
        248.0 + 8.0 > overflow_root_frame.x + overflow_root_frame.width,
        "the second item must paint to the right of its Grid root"
    );
    let overflow_paint = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), spinon_render::RuntimePaint::Opaque(color) if color == OpaqueCssSrgb::new(168, 85, 247))
        })
        .expect("the outside marker should remain in the paint snapshot");
    assert_eq!(
        overflow_paint.clip_css_px(),
        spinon_render::CssClip::unconstrained(),
        "visible overflow must not be clipped at the Grid root edge"
    );
}
