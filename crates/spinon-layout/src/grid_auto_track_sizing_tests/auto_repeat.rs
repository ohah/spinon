use super::*;
use crate::{LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue};

fn two_axis_auto_repeat_grid(
    mode: crate::GridAutoRepeatMode,
    auto_rows: GridAutoRows,
    auto_columns: crate::GridAutoColumns,
) -> LayoutInput {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(40.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(10.0),
        ],
        vec![
            GridTrackSize::LengthPx(14.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(30.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(6.0),
        ],
        1,
        LayoutGap {
            column: crate::LayoutLengthPercentage::length(8.0),
            row: crate::LayoutLengthPercentage::length(6.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(180.0);
    input.viewport = Viewport {
        width: 200.0,
        height: 180.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(crate::GridAutoRepeat {
        mode,
        start: 1,
        end: 2,
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(crate::GridAutoRepeat {
        mode,
        start: 1,
        end: 2,
        line_names: Vec::new(),
    });
    container.auto_rows = auto_rows;
    container.auto_columns = auto_columns;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(-9),
            row_end: GridPlacementInput::Line(-7),
            column_start: GridPlacementInput::Line(-8),
            column_end: GridPlacementInput::Line(-6),
        },
    );
    input
}

fn set_two_axis_leading_span(input: &mut LayoutInput, columns: i32, rows: i32) {
    let (column_start, column_end) = negative_leading_span(5, columns);
    let (row_start, row_end) = negative_leading_span(6, rows);
    let item = input.grid_items.get_mut(&id(2)).unwrap();
    item.column_start = column_start;
    item.column_end = column_end;
    item.row_start = row_start;
    item.row_end = row_end;
}

fn negative_leading_span(
    explicit_tracks: i32,
    leading_tracks: i32,
) -> (GridPlacementInput, GridPlacementInput) {
    (
        GridPlacementInput::Line(-(explicit_tracks + leading_tracks + 1)),
        GridPlacementInput::Line(-(explicit_tracks + 1)),
    )
}

fn collapsed_track_count(output: &crate::LayoutOutput, axis: LayoutGridTrackSizingAxis) -> usize {
    output.grid_track_sizing_traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap()
        .stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && track.is_collapsed)
        .count()
}

#[test]
fn numeric_negative_line_uses_expanded_auto_repeat_track_count() {
    let mut input = grid_input(
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(40.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(10.0),
        ],
        vec![GridTrackSize::LengthPx(24.0)],
        1,
        LayoutGap {
            column: crate::LayoutLengthPercentage::length(8.0),
            row: crate::LayoutLengthPercentage::length(6.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(24.0);
    input.viewport = Viewport {
        width: 200.0,
        height: 24.0,
    };
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(crate::GridAutoRepeat {
        mode: crate::GridAutoRepeatMode::AutoFill,
        start: 1,
        end: 2,
        line_names: Vec::new(),
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(-7),
            column_end: GridPlacementInput::Line(-6),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        (output.frames[&id(2)].x, output.frames[&id(2)].width),
        (0.0, 20.0)
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns).len(),
        6,
        "one leading implicit track plus five expanded explicit tracks"
    );
}

#[test]
fn auto_repeat_leading_auto_track_lists_cycle_backwards_on_both_axes() {
    let cases: &[(i32, &[f32], &[f32])] = &[
        (1, &[47.0], &[29.0]),
        (2, &[31.0, 47.0], &[17.0, 29.0]),
        (3, &[47.0, 31.0, 47.0], &[29.0, 17.0, 29.0]),
    ];
    for &(leading_tracks, expected_columns, expected_rows) in cases {
        let mut input = two_axis_auto_repeat_grid(
            crate::GridAutoRepeatMode::AutoFill,
            GridAutoRows::TrackList(vec![
                GridTrackSize::LengthPx(17.0),
                GridTrackSize::LengthPx(29.0),
            ]),
            crate::GridAutoColumns::TrackList(vec![
                GridTrackSize::LengthPx(31.0),
                GridTrackSize::LengthPx(47.0),
            ]),
        );
        set_two_axis_leading_span(&mut input, leading_tracks, leading_tracks);

        let output = TaffyLayoutEngine.compute(&input).unwrap();

        assert_eq!(
            &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns)
                [..leading_tracks as usize],
            expected_columns,
            "columns leading cycle with {leading_tracks} implicit tracks"
        );
        assert_eq!(
            &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows)
                [..leading_tracks as usize],
            expected_rows,
            "rows leading cycle with {leading_tracks} implicit tracks"
        );
    }
}

#[test]
fn auto_repeat_scalar_auto_track_sizes_cover_leading_implicit_tracks() {
    let input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::LengthPx(19.0),
        crate::GridAutoColumns::LengthPx(23.0),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns)[..2],
        [23.0, 23.0]
    );
    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows)[..2],
        [19.0, 19.0]
    );
}

#[test]
fn zero_px_auto_track_size_remains_zero_on_leading_implicit_tracks() {
    let input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::LengthPx(0.0),
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(0.0)]),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns)[..2],
        [0.0, 0.0]
    );
    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows)[..2],
        [0.0, 0.0]
    );
}

#[path = "auto_repeat/area_expansion.rs"]
mod area_expansion;

#[test]
fn auto_fit_keeps_repeat_count_and_collapses_empty_repeats_after_leading_placement() {
    let input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFit,
        GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(17.0),
            GridTrackSize::LengthPx(29.0),
        ]),
        crate::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(31.0),
            GridTrackSize::LengthPx(47.0),
        ]),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        collapsed_track_count(&output, LayoutGridTrackSizingAxis::Columns),
        3
    );
    assert_eq!(
        collapsed_track_count(&output, LayoutGridTrackSizingAxis::Rows),
        4
    );
    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns)[..2],
        [31.0, 47.0]
    );
    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows)[..2],
        [17.0, 29.0]
    );
}

#[test]
fn secondary_positive_line_uses_expanded_repeat_grid() {
    let mut input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::Auto,
    );
    let item = input.grid_items.get_mut(&id(2)).unwrap();
    item.row_start = GridPlacementInput::Line(4);
    item.row_end = GridPlacementInput::Line(5);

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let rows = final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows);
    let frame = output.frames[&id(2)];

    assert_eq!(
        rows.len(),
        6,
        "one authored row before, four repeats, one after"
    );
    assert_eq!(frame.y, 14.0 + 6.0 + rows[1] + 6.0 + rows[2] + 6.0);
    assert_eq!(frame.height, 10.0);
}

#[test]
fn secondary_reversed_and_equal_lines_follow_taffy_grid_placement() {
    let mut reversed = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::Auto,
    );
    let item = reversed.grid_items.get_mut(&id(2)).unwrap();
    item.row_start = GridPlacementInput::Line(5);
    item.row_end = GridPlacementInput::Line(4);
    let reversed_output = TaffyLayoutEngine.compute(&reversed).unwrap();
    let rows = final_axis_track_sizes(&reversed_output, LayoutGridTrackSizingAxis::Rows);
    let reversed_frame = reversed_output.frames[&id(2)];
    assert_eq!(reversed_frame.y, 14.0 + 6.0 + rows[1] + 6.0 + rows[2] + 6.0);
    assert_eq!(reversed_frame.height, 10.0);

    let mut equal = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::Auto,
    );
    let item = equal.grid_items.get_mut(&id(2)).unwrap();
    item.row_start = GridPlacementInput::Line(4);
    item.row_end = GridPlacementInput::Line(4);
    let equal_output = TaffyLayoutEngine.compute(&equal).unwrap();
    let rows = final_axis_track_sizes(&equal_output, LayoutGridTrackSizingAxis::Rows);
    let equal_frame = equal_output.frames[&id(2)];
    assert_eq!(equal_frame.y, 14.0 + 6.0 + rows[1] + 6.0 + rows[2] + 6.0);
    assert_eq!(equal_frame.height, 10.0);
}

#[test]
fn zero_secondary_line_fails_before_layout() {
    let mut input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::Auto,
    );
    input.grid_items.get_mut(&id(2)).unwrap().row_start = GridPlacementInput::Line(0);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridPlacement {
            property: "grid-row-start",
            ..
        })
    ));
}

#[test]
fn auto_repeat_implicit_lists_convert_supported_track_sizing_functions() {
    let tracks = [
        GridTrackSize::Percentage(0.25),
        GridTrackSize::Fraction(0.5),
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::MinMax {
            min: crate::GridTrackMinSizingFunction::LengthPx(8.0),
            max: crate::GridTrackMaxSizingFunction::Fraction(1.0),
        },
        GridTrackSize::FitContent(crate::GridTrackLengthPercentage::LengthPx(24.0)),
    ];
    for track in tracks {
        let mut input = two_axis_auto_repeat_grid(
            crate::GridAutoRepeatMode::AutoFill,
            GridAutoRows::TrackList(vec![track]),
            crate::GridAutoColumns::TrackList(vec![track]),
        );
        set_two_axis_leading_span(&mut input, 1, 1);

        let output = TaffyLayoutEngine.compute(&input).unwrap_or_else(|error| {
            panic!("typed auto track {track:?} must convert on both axes: {error:?}")
        });

        for axis in [
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingAxis::Rows,
        ] {
            let sizes = final_axis_track_sizes(&output, axis);
            assert!(!sizes.is_empty(), "{track:?} produced no {axis:?} tracks");
            assert!(
                sizes.iter().all(|size| size.is_finite()),
                "{track:?} produced a non-finite {axis:?} track: {sizes:?}"
            );
        }
    }
}

#[test]
fn auto_repeat_calc_implicit_tracks_use_axis_math_bindings() {
    let mut input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::TrackList(vec![GridTrackSize::Calc(LayoutCalcId(7_002))]),
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::Calc(LayoutCalcId(7_001))]),
    );
    set_two_axis_leading_span(&mut input, 1, 1);
    input = input.with_css_math(vec![
        LayoutCssMathValue {
            id: LayoutCalcId(7_001),
            node_id: id(1),
            property: LayoutCssMathProperty::GridTemplateColumns,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(5.0),
                LayoutCssMath::Percentage(0.1),
            ]),
        },
        LayoutCssMathValue {
            id: LayoutCalcId(7_002),
            node_id: id(1),
            property: LayoutCssMathProperty::GridTemplateRows,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(5.0),
                LayoutCssMath::Percentage(0.1),
            ]),
        },
    ]);

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns)[..1],
        [25.0]
    );
    assert_eq!(
        &final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows)[..1],
        [23.0]
    );
}

#[test]
fn auto_repeat_nested_calc_tracks_validate_axis_and_node_binding() {
    let mut input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            crate::GridTrackLengthPercentage::Calc(LayoutCalcId(7_003)),
        )]),
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::MinMax {
            min: crate::GridTrackMinSizingFunction::Calc(LayoutCalcId(7_001)),
            max: crate::GridTrackMaxSizingFunction::MaxContent,
        }]),
    );
    set_two_axis_leading_span(&mut input, 1, 1);
    input = input.with_css_math(vec![
        LayoutCssMathValue {
            id: LayoutCalcId(7_001),
            node_id: id(1),
            property: LayoutCssMathProperty::GridTemplateColumns,
            expression: LayoutCssMath::LengthPx(8.0),
        },
        LayoutCssMathValue {
            id: LayoutCalcId(7_003),
            node_id: id(1),
            property: LayoutCssMathProperty::GridTemplateRows,
            expression: LayoutCssMath::LengthPx(12.0),
        },
    ]);

    TaffyLayoutEngine.compute(&input).unwrap();

    let mut mismatched = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::Calc(LayoutCalcId(7_004))]),
    );
    mismatched = mismatched.with_css_math(vec![LayoutCssMathValue {
        id: LayoutCalcId(7_004),
        node_id: id(2),
        property: LayoutCssMathProperty::GridTemplateColumns,
        expression: LayoutCssMath::LengthPx(8.0),
    }]);

    assert_eq!(
        TaffyLayoutEngine.compute(&mismatched),
        Err(LayoutError::CssMathBindingMismatch {
            node: id(1),
            property: "grid-auto-columns",
            id: LayoutCalcId(7_004),
        })
    );
}

#[test]
fn auto_repeat_calc_implicit_track_rejects_missing_axis_binding() {
    let input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::Auto,
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::Calc(LayoutCalcId(7_005))]),
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::MissingCssMath {
            node: id(1),
            property: "grid-auto-columns",
            id: LayoutCalcId(7_005),
        })
    );
}

#[test]
fn typed_auto_track_lists_remain_rejected_outside_track_sizing_profile() {
    let mut input = two_axis_auto_repeat_grid(
        crate::GridAutoRepeatMode::AutoFill,
        GridAutoRows::TrackList(vec![GridTrackSize::MinContent]),
        crate::GridAutoColumns::Auto,
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::DenseAutoPlacement;
    container.columns_auto_repeat = None;
    container.rows_auto_repeat = None;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![GridTrackSize::MaxContent]);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoColumns { node, .. }) if node == id(1)
    ));
}
