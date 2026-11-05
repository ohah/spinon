use super::*;
use crate::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};

fn root_html_intrinsic_gap_input(gap: LayoutGap) -> LayoutInput {
    let flex_track = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(0.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let mut input = super::grid_input(
        vec![GridTrackSize::MinContent, flex_track],
        vec![GridTrackSize::MinContent, flex_track],
        3,
        gap,
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(100.0);
    input.nodes[1].style.width = LayoutDimension::Fixed(40.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
    input.nodes[2].style.width = LayoutDimension::Fixed(20.0);
    input.nodes[2].style.height = LayoutDimension::Fixed(20.0);
    input.nodes[3].style.width = LayoutDimension::Fixed(10.0);
    input.nodes[3].style.height = LayoutDimension::Fixed(10.0);
    input.viewport = crate::Viewport {
        width: 100.0,
        height: 100.0,
    };
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .track_sizing_trace_enabled = true;
    input.grid_items.insert(
        id(2),
        crate::grid_placement::GridItemInput {
            row_start: GridPlacementInput::Line(1),
            ..Default::default()
        },
    );
    input.grid_items.insert(
        id(3),
        crate::grid_placement::GridItemInput {
            row_start: GridPlacementInput::Line(1),
            ..Default::default()
        },
    );
    input.grid_items.insert(
        id(4),
        crate::grid_placement::GridItemInput {
            row_start: GridPlacementInput::Line(2),
            ..Default::default()
        },
    );
    input
}

fn root_html_intrinsic_input(columns: Vec<GridTrackSize>, width: f32) -> LayoutInput {
    let mut input = super::grid_input(
        columns,
        vec![GridTrackSize::LengthPx(30.0)],
        1,
        LayoutGap::default(),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.viewport.width = width;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    container.track_sizing_trace_enabled = true;
    input
}

fn assert_column_tracks(columns: Vec<GridTrackSize>, width: f32, expected: &[f32], case: &str) {
    let input = root_html_intrinsic_input(columns, width);
    let output = TaffyLayoutEngine
        .compute(&input)
        .unwrap_or_else(|error| panic!("{case}: {error:?}"));
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap_or_else(|| panic!("{case}: missing Grid track sizing trace"));
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{case}: missing columns sizing pass"));
    assert!(pass.stages.iter().any(|stage| {
        matches!(
            stage.stage,
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
        )
    }));
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final columns stage"));
    let actual = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len(), "{case}: track count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.02,
            "{case}: columns[{index}] expected {expected}, got {actual}"
        );
    }
}

#[test]
fn root_html_intrinsic_tracks_match_fixed_input_expectations() {
    assert_column_tracks(
        vec![GridTrackSize::MinContent],
        320.0,
        &[20.0],
        "min-content",
    );
    assert_column_tracks(
        vec![GridTrackSize::MaxContent],
        320.0,
        &[20.0],
        "max-content",
    );
    assert_column_tracks(vec![GridTrackSize::Auto], 320.0, &[320.0], "auto stretch");
    assert_column_tracks(
        vec![GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::MinContent,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        }],
        160.0,
        &[160.0],
        "intrinsic minimum with flex maximum",
    );
    assert_column_tracks(
        vec![
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::Percentage(0.5),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(100.0),
        ],
        160.0,
        &[80.0, 100.0],
        "percentage minimum with flex maximum",
    );
}

#[test]
fn root_html_intrinsic_tracks_reject_mixed_auto_and_one_axis_placement() {
    let mut input = super::grid_input(
        vec![GridTrackSize::MinContent, GridTrackSize::MaxContent],
        vec![GridTrackSize::LengthPx(30.0)],
        2,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::RootHtmlIntrinsicTrackSizingV1;
    input.grid_items.insert(
        id(3),
        GridItemInput {
            column_start: GridPlacementInput::Line(2),
            ..GridItemInput::default()
        },
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement { .. })
    ));
}

#[test]
fn root_html_intrinsic_tracks_apply_fixed_gutters_before_fractional_expansion() {
    let cases = [
        (
            LayoutGap::default(),
            [40.0, 60.0],
            [20.0, 80.0],
            [
                [0.0, 0.0, 40.0, 10.0],
                [40.0, 0.0, 20.0, 20.0],
                [0.0, 20.0, 10.0, 10.0],
            ],
            "zero gap control",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::length(10.0),
            },
            [40.0, 50.0],
            [20.0, 80.0],
            [
                [0.0, 0.0, 40.0, 10.0],
                [50.0, 0.0, 20.0, 20.0],
                [0.0, 20.0, 10.0, 10.0],
            ],
            "column gap",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::length(10.0),
                column: LayoutLengthPercentage::ZERO,
            },
            [40.0, 60.0],
            [20.0, 70.0],
            [
                [0.0, 0.0, 40.0, 10.0],
                [40.0, 0.0, 20.0, 20.0],
                [0.0, 30.0, 10.0, 10.0],
            ],
            "row gap",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::length(10.0),
                column: LayoutLengthPercentage::length(10.0),
            },
            [40.0, 50.0],
            [20.0, 70.0],
            [
                [0.0, 0.0, 40.0, 10.0],
                [50.0, 0.0, 20.0, 20.0],
                [0.0, 30.0, 10.0, 10.0],
            ],
            "both gaps",
        ),
    ];

    for (gap, expected_columns, expected_rows, expected_frames, case) in cases {
        let input = root_html_intrinsic_gap_input(gap);
        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{case}: {error:?}"));
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .unwrap_or_else(|| panic!("{case}: missing Grid track sizing trace"));

        for (axis, expected) in [
            (LayoutGridTrackSizingAxis::Columns, expected_columns),
            (LayoutGridTrackSizingAxis::Rows, expected_rows),
        ] {
            let pass = trace
                .passes
                .iter()
                .find(|pass| pass.axis == axis)
                .unwrap_or_else(|| panic!("{case}: missing {axis:?} sizing pass"));
            let final_stage = pass
                .stages
                .last()
                .unwrap_or_else(|| panic!("{case}: missing {axis:?} final stage"));
            let actual = final_stage
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            assert_eq!(actual.len(), 2, "{case}: {axis:?} track count");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert!(
                    (actual - expected).abs() <= 0.02,
                    "{case}: {axis:?}[{index}] expected {expected}, got {actual}"
                );
            }
        }

        for (node, expected) in [
            (id(2), expected_frames[0]),
            (id(3), expected_frames[1]),
            (id(4), expected_frames[2]),
        ] {
            let frame = output.frames[&node];
            assert_eq!(
                [frame.x, frame.y, frame.width, frame.height],
                expected,
                "{case}: item {node:?} frame"
            );
        }
    }
}

#[test]
fn root_html_intrinsic_tracks_reject_unsupported_and_invalid_gutters() {
    for (gap, property) in [
        (
            LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::percent(0.1),
            },
            "column-gap",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::Calc(crate::LayoutCalcId(1)),
                column: LayoutLengthPercentage::ZERO,
            },
            "row-gap",
        ),
    ] {
        let mut input = root_html_intrinsic_gap_input(gap);
        if matches!(gap.row, LayoutLengthPercentage::Calc(_)) {
            input.css_math.push(crate::LayoutCssMathValue {
                id: crate::LayoutCalcId(1),
                node_id: id(1),
                property: crate::LayoutCssMathProperty::RowGap,
                expression: crate::LayoutCssMath::LengthPx(10.0),
            });
        }
        match TaffyLayoutEngine.compute(&input) {
            Err(LayoutError::UnsupportedGridContainerStyle {
                node,
                property: actual,
            }) if node == id(1) && actual == property => {}
            result => panic!("{property}: expected an unsupported gap diagnostic, got {result:?}"),
        }
    }

    for (gap, field) in [
        (
            LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::length(-1.0),
            },
            "gap.column",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::length(f32::NAN),
                column: LayoutLengthPercentage::ZERO,
            },
            "gap.row",
        ),
        (
            LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::length(f32::INFINITY),
            },
            "gap.column",
        ),
    ] {
        let input = root_html_intrinsic_gap_input(gap);
        assert!(
            matches!(
                TaffyLayoutEngine.compute(&input),
                Err(LayoutError::InvalidStyle { node, field: actual })
                    if node == id(1) && actual == field
            ),
            "{field}: invalid typed gap should fail style validation"
        );
    }
}

#[test]
fn root_html_intrinsic_tracks_reject_gutter_extent_overflow() {
    let mut input = root_html_intrinsic_gap_input(LayoutGap {
        row: LayoutLengthPercentage::ZERO,
        column: LayoutLengthPercentage::length(f32::MAX),
    });
    input.grid_containers.get_mut(&id(1)).unwrap().columns = vec![
        GridTrackSize::MinContent,
        GridTrackSize::MinContent,
        GridTrackSize::MinContent,
    ];

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridContainerStyle { node, property })
            if node == id(1) && property == "column-gap"
    ));
}

#[test]
fn root_html_intrinsic_tracks_do_not_publish_unrepresentable_track_plus_gutter_extent() {
    let large = f32::MAX * 0.75;
    for (axis, property) in [
        (LayoutGridTrackSizingAxis::Columns, "column-gap"),
        (LayoutGridTrackSizingAxis::Rows, "row-gap"),
    ] {
        let gap = match axis {
            LayoutGridTrackSizingAxis::Columns => LayoutGap {
                row: LayoutLengthPercentage::ZERO,
                column: LayoutLengthPercentage::length(large),
            },
            LayoutGridTrackSizingAxis::Rows => LayoutGap {
                row: LayoutLengthPercentage::length(large),
                column: LayoutLengthPercentage::ZERO,
            },
        };
        let mut input = root_html_intrinsic_gap_input(gap);
        let container = input.grid_containers.get_mut(&id(1)).unwrap();
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                container.columns = vec![GridTrackSize::MinContent, GridTrackSize::MinContent];
                input.nodes[1].style.width = LayoutDimension::Fixed(large);
            }
            LayoutGridTrackSizingAxis::Rows => {
                container.rows = vec![GridTrackSize::MinContent, GridTrackSize::MinContent];
                input.nodes[1].style.height = LayoutDimension::Fixed(large);
            }
        }
        input.nodes[2].style.display = LayoutDisplay::None;
        input.nodes[3].style.display = LayoutDisplay::None;

        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::UnsupportedGridContainerStyle { node, property: actual })
                if node == id(1) && actual == property
        ));
    }
}
