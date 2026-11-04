use super::*;

fn assert_frame_with_ratio_tolerance(
    output: &crate::StyleLayoutOutput,
    node: HostNodeHandle,
    expected: [f32; 4],
    label: &str,
) {
    let frame = output.layout.frames[&node.id()];
    let actual = [frame.x, frame.y, frame.width, frame.height];
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.01,
            "{label} component {index}: expected {expected}, got {actual}"
        );
    }
}

#[test]
fn parsed_root_html_direct_text_grid_accepts_finite_positive_ratio_values() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let cases = [
        (
            "width:60px;height:auto;",
            "aspect-ratio:1 / 2;",
            120.0,
            60.0,
            120.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:4 / 3;",
            45.0,
            60.0,
            45.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:1.5;",
            40.0,
            60.0,
            40.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:5 / 2;",
            24.0,
            60.0,
            24.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:3 / 2;",
            40.0,
            60.0,
            40.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:10 / 1;",
            10.0,
            60.0,
            10.0,
        ),
        ("width:60px;height:auto;", "", 10.0, 60.0, 10.0),
        (
            "width:auto;height:80px;",
            "aspect-ratio:1 / 2;",
            80.0,
            40.0,
            80.0,
        ),
        (
            "width:auto;height:30px;",
            "aspect-ratio:4 / 3;",
            30.0,
            40.0,
            30.0,
        ),
        (
            "width:auto;height:30px;",
            "aspect-ratio:3 / 2;",
            30.0,
            45.0,
            30.0,
        ),
        (
            "width:auto;height:30px;",
            "aspect-ratio:5 / 2;",
            30.0,
            75.0,
            30.0,
        ),
        (
            "width:60px;height:auto;",
            "aspect-ratio:2 / 1;",
            30.0,
            60.0,
            30.0,
        ),
        (
            "width:auto;height:30px;",
            "aspect-ratio:2 / 1;",
            30.0,
            60.0,
            30.0,
        ),
        ("width:auto;height:30px;", "", 30.0, 40.0, 30.0),
    ];
    let target_styles = cases
        .iter()
        .map(|(dimensions, ratio, _, _, _)| item_style(dimensions, ratio))
        .collect::<Vec<_>>();
    let specs = target_styles
        .iter()
        .map(|target| scene(&grid, &short, target))
        .collect::<Vec<_>>();
    let fixture = Fixture::new(&root_style(""), &specs);
    let output = fixture
        .compute()
        .expect("all finite representable ratio scenes should project");

    let mut y = 0.0;
    for (index, (scene, (_, _, row_height, target_width, target_height))) in
        fixture.scenes.iter().zip(cases).enumerate()
    {
        assert_frame_with_ratio_tolerance(
            &output,
            scene.grid,
            [0.0, y, 70.0, row_height],
            &format!("scene {index} Grid"),
        );
        assert_frame_with_ratio_tolerance(
            &output,
            scene.items[0],
            [0.0, y, 10.0, 10.0],
            &format!("scene {index} short item"),
        );
        assert_frame_with_ratio_tolerance(
            &output,
            scene.items[1],
            [10.0, y, target_width, target_height],
            &format!("scene {index} ratio target"),
        );
        y += row_height;
    }
}

#[test]
fn parsed_root_html_aspect_ratio_fails_closed_outside_the_fixed_owner_slice() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let width_control = item_style("width:60px;height:auto;", "");
    let zero_ratio = item_style("width:60px;height:auto;", "aspect-ratio:0 / 1;");
    let zero_ratio_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &zero_ratio)]);
    assert!(matches!(
        zero_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    for (dimensions, ratio) in [
        ("width:60px;height:auto;", "1e-38 / 1"),
        ("width:auto;height:30px;", "1e38 / 1"),
        ("width:auto;height:80px;", "3 / 2"),
        ("width:auto;height:40px;", "1 / 2"),
    ] {
        let long = item_style(dimensions, &format!("aspect-ratio:{ratio};"));
        let fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &long)]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: "aspect-ratio",
                ..
            })
        ));
    }

    let ratio = item_style("width:60px;height:auto;", "aspect-ratio:2 / 1;");
    let constrained = item_style(
        "width:60px;height:auto;max-height:40px;",
        "aspect-ratio:2 / 1;",
    );
    let constrained_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &constrained)]);
    assert!(matches!(
        constrained_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let short_ratio = item_style("width:auto;height:auto;", "aspect-ratio:2 / 1;");
    let multiple_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short_ratio, &ratio)]);
    assert!(matches!(
        multiple_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let named_area_grid = grid_style("grid-template-areas:'a b';");
    let named_area_fixture =
        Fixture::new(&root_style(""), &[scene(&named_area_grid, &short, &ratio)]);
    assert!(matches!(
        named_area_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let grid_ratio = grid_style("aspect-ratio:2 / 1;");
    let grid_ratio_fixture = Fixture::new(&root_style(""), &[scene(&grid_ratio, &short, &ratio)]);
    assert!(matches!(
        grid_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let block_root_ratio = root_style("aspect-ratio:2 / 1;");
    let no_ratio = item_style("width:60px;height:auto;", "");
    let root_ratio_fixture = Fixture::new(&block_root_ratio, &[scene(&grid, &short, &no_ratio)]);
    assert!(matches!(
        root_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let non_candidate_auto_row_grid =
        grid_style("width:71px;align-items:normal;justify-items:normal;");
    let non_candidate_fixture = Fixture::new(
        &root_style(""),
        &[scene(&non_candidate_auto_row_grid, &short, &width_control)],
    );
    assert!(matches!(
        non_candidate_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "grid-template-rows",
            ..
        })
    ));

    for alignment in ["align-content:start;", "justify-content:start;"] {
        let non_default_grid = grid_style(alignment);
        let fixture = Fixture::new(&root_style(""), &[scene(&non_default_grid, &short, &ratio)]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: "aspect-ratio",
                ..
            })
        ));
    }

    for alignment in ["align-self:start;", "justify-self:start;"] {
        let aligned_short = item_style(&format!("width:auto;height:auto;{alignment}"), "");
        let aligned_long = item_style(
            &format!("width:60px;height:auto;{alignment}"),
            "aspect-ratio:2 / 1;",
        );
        for (short_style, long_style) in [(&aligned_short, &ratio), (&short, &aligned_long)] {
            let fixture = Fixture::new(&root_style(""), &[scene(&grid, short_style, long_style)]);
            assert!(matches!(
                fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: "aspect-ratio",
                    ..
                })
            ));
        }
    }
}

#[test]
fn multiple_finite_ratio_transfers_with_non_finite_absolute_edges_are_rejected() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let huge = item_style("width:60px;height:auto;", "aspect-ratio:2e-37 / 1;");
    let specs = [scene(&grid, &short, &huge), scene(&grid, &short, &huge)];
    let fixture = Fixture::new(&root_style(""), &specs);

    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::NonFiniteFrame(_)
        ))
    ));
}

#[test]
fn finite_ratio_transfers_below_text_minima_keep_the_automatic_minimum() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let height_locked = item_style("width:auto;height:30px;", "aspect-ratio:1 / 100;");
    let width_locked = item_style("width:60px;height:auto;", "aspect-ratio:100 / 1;");
    let specs = [
        scene(&grid, &short, &width_locked),
        scene(&grid, &short, &height_locked),
    ];
    let fixture = Fixture::new(&root_style(""), &specs);
    let output = fixture
        .compute()
        .expect("finite ratios with text automatic minima should remain supported");

    assert_frame(
        &output,
        fixture.scenes[0].grid,
        [0.0, 0.0, 70.0, 10.0],
        "100:1 width-locked auto row keeps the one-line text minimum",
    );
    assert_frame(
        &output,
        fixture.scenes[0].items[1],
        [10.0, 0.0, 60.0, 10.0],
        "100:1 width-locked target keeps its one-line text minimum",
    );
    assert_frame(
        &output,
        fixture.scenes[1].grid,
        [0.0, 10.0, 70.0, 30.0],
        "1:100 height-locked scene keeps fixed row height",
    );
    assert_frame(
        &output,
        fixture.scenes[1].items[1],
        [10.0, 10.0, 40.0, 30.0],
        "1:100 width transfer remains clamped by the Ahem text minimum",
    );
}
