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
