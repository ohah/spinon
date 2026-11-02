use super::*;

fn fraction_grid(
    width: f32,
    item_count: usize,
    columns: Vec<GridTrackSize>,
    pattern: Vec<GridTrackSize>,
) -> LayoutInput {
    let mut input =
        root_html_column_flow_with_auto_columns(item_count, GridAutoColumns::TrackList(pattern));
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(20.0);
    input.nodes[0].style.gap.row = LayoutLengthPercentage::length(0.0);
    input.viewport.width = width;
    input.viewport.height = 20.0;
    for node in input.nodes.iter_mut().skip(1) {
        node.style.width = LayoutDimension::Fixed(1.0);
        node.style.height = LayoutDimension::Fixed(1.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = columns;
    container.rows = vec![GridTrackSize::LengthPx(20.0)];
    container.track_sizing_trace_enabled = true;
    input
}

fn assert_column_starts(input: &LayoutInput, expected: &[f32]) {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    let actual = expected
        .iter()
        .enumerate()
        .map(|(index, _)| output.frames[&id(index as u64 + 2)].x)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.02,
            "column {index}: expected {expected}, got {actual}; starts={actual:?}"
        );
    }
}

#[test]
fn root_html_column_flow_fraction_lists_match_fixed_precomparison_scenes() {
    let cases = [
        (
            "1fr 2fr cycling",
            fraction_grid(
                240.0,
                6,
                vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)],
                vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)],
            ),
            vec![0.0, 35.0, 80.0, 109.166_67, 162.5, 191.666_67],
        ),
        (
            "scalar 1fr",
            fraction_grid(
                240.0,
                6,
                vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)],
                vec![GridTrackSize::Fraction(1.0)],
            ),
            vec![0.0, 35.0, 80.0, 121.25, 162.5, 203.75],
        ),
        (
            "fixed and fraction mixed",
            fraction_grid(
                240.0,
                6,
                vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)],
                vec![GridTrackSize::LengthPx(10.0), GridTrackSize::Fraction(1.0)],
            ),
            vec![0.0, 35.0, 80.0, 95.0, 162.5, 177.5],
        ),
        (
            "fixed percentage and fraction mixed",
            fraction_grid(
                300.0,
                8,
                vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)],
                vec![
                    GridTrackSize::LengthPx(10.0),
                    GridTrackSize::Percentage(0.1),
                    GridTrackSize::Fraction(1.0),
                ],
            ),
            vec![0.0, 35.0, 80.0, 95.0, 130.0, 192.5, 207.5, 242.5],
        ),
        (
            "subunit factor preserves trailing free space",
            fraction_grid(
                120.0,
                3,
                vec![GridTrackSize::LengthPx(30.0)],
                vec![GridTrackSize::Fraction(0.25)],
            ),
            vec![0.0, 35.0, 60.0],
        ),
        (
            "large finite factors accumulate without f32 overflow",
            fraction_grid(
                215.0,
                6,
                vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(40.0)],
                vec![GridTrackSize::Fraction(1.0e38)],
            ),
            vec![0.0, 35.0, 80.0, 115.0, 150.0, 185.0],
        ),
    ];
    for (label, input, expected) in cases {
        assert_column_starts(&input, &expected);
        let output = TaffyLayoutEngine.compute(&input).unwrap();
        assert_eq!(output.grid_track_sizing_traces.len(), 1, "{label}");
    }
}

#[test]
fn root_html_column_flow_zero_fraction_keeps_the_fixed_item_auto_minimum() {
    let mut input = fraction_grid(
        120.0,
        3,
        vec![GridTrackSize::LengthPx(30.0)],
        vec![GridTrackSize::Fraction(0.0), GridTrackSize::Fraction(1.0)],
    );
    input.nodes[2].style.width = LayoutDimension::Fixed(10.0);
    assert_column_starts(&input, &[0.0, 35.0, 50.0]);
}

#[test]
fn root_html_column_flow_fraction_patterns_reject_invalid_factors_and_width_basis() {
    for factor in [-1.0, f32::NAN, f32::INFINITY] {
        let input = fraction_grid(
            120.0,
            3,
            vec![GridTrackSize::LengthPx(30.0)],
            vec![GridTrackSize::Fraction(factor)],
        );
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrack {
                property: "grid-auto-columns",
                ..
            })
        ));
    }

    let mut indefinite = fraction_grid(
        120.0,
        3,
        vec![GridTrackSize::LengthPx(30.0)],
        vec![GridTrackSize::Fraction(1.0)],
    );
    indefinite.nodes[0].style.width = LayoutDimension::Auto;
    let grid = indefinite.nodes.remove(0);
    indefinite.root = id(100);
    indefinite.nodes.insert(
        0,
        LayoutNode {
            id: id(100),
            children: vec![id(1)],
            style: LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Fixed(175.0),
                height: LayoutDimension::Fixed(63.0),
                ..LayoutStyle::default()
            },
        },
    );
    indefinite.nodes.insert(1, grid);
    indefinite.viewport.width = 175.0;
    indefinite.viewport.height = 63.0;
    let result = TaffyLayoutEngine.compute(&indefinite);
    assert!(
        matches!(
            &result,
            Err(crate::LayoutError::UnsupportedGridContainerStyle {
                property: "width",
                ..
            })
        ),
        "unexpected width-basis result: {result:?}"
    );

    for (input, property) in [
        (
            {
                let mut input = fraction_grid(
                    120.0,
                    3,
                    vec![GridTrackSize::LengthPx(30.0)],
                    vec![GridTrackSize::Fraction(1.0)],
                );
                input.nodes[0].style.min_width = LayoutDimension::Fixed(1.0);
                input
            },
            "min-width",
        ),
        (
            {
                let mut input = fraction_grid(
                    120.0,
                    3,
                    vec![GridTrackSize::LengthPx(30.0)],
                    vec![GridTrackSize::Fraction(1.0)],
                );
                input.nodes[0].style.max_width = LayoutDimension::Fixed(200.0);
                input
            },
            "max-width",
        ),
        (
            {
                let mut input = fraction_grid(
                    120.0,
                    3,
                    vec![GridTrackSize::LengthPx(30.0)],
                    vec![GridTrackSize::Fraction(1.0)],
                );
                input.nodes[0].style.aspect_ratio = Some(1.0);
                input
            },
            "aspect-ratio",
        ),
    ] {
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::UnsupportedGridContainerStyle {
                property: actual,
                ..
            }) if actual == property
        ));
    }
}
