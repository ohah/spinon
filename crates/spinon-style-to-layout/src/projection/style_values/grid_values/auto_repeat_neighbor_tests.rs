use super::project_grid_layout;
use crate::StyleLayoutError;
use spinon_core::NodeId;
use spinon_layout::{GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedCssMath, ComputedStyleProfile,
};

fn repeated_track() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(40.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn min_content_with_fixed_maximum(maximum: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::MinContent,
        max: ComputedCssGridTrackBreadth::LengthPx(maximum),
    }
}

fn fixed_minimum_auto_maximum(minimum: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(minimum),
        max: ComputedCssGridTrackBreadth::Auto,
    }
}

fn grid_with_column_neighbors(
    prefix: ComputedCssGridTrackSize,
    suffix: ComputedCssGridTrackSize,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![prefix, repeated_track(), suffix],
        rows: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: Vec::new(),
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn grid_with_row_neighbors(
    prefix: ComputedCssGridTrackSize,
    suffix: ComputedCssGridTrackSize,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        rows: vec![prefix, repeated_track(), suffix],
        columns_auto_repeat: None,
        rows_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: Vec::new(),
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: Vec::new(),
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn calc_neighbor(length: ComputedCssMath, percentage: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
        length,
        ComputedCssMath::Percentage(percentage),
    ]))
}

#[test]
fn track_sizing_projection_accepts_the_supported_auto_repeat_neighbor_shapes() {
    let node = NodeId::new(11_131).unwrap();
    let supported = [
        ComputedCssGridTrackSize::LengthPx(8.0),
        ComputedCssGridTrackSize::Percentage(0.05),
        calc_neighbor(ComputedCssMath::LengthPx(10.0), 0.05),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(0.05),
            ComputedCssMath::LengthPx(10.0),
        ])),
        calc_neighbor(
            ComputedCssMath::Negate(Box::new(ComputedCssMath::LengthPx(2.0))),
            0.05,
        ),
        calc_neighbor(ComputedCssMath::LengthPx(20.0), -0.05),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::Negate(Box::new(ComputedCssMath::Percentage(0.05))),
            ComputedCssMath::LengthPx(20.0),
        ])),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::LengthPx(-2.0),
            ComputedCssMath::Negate(Box::new(ComputedCssMath::Percentage(0.01))),
        ])),
    ];
    for neighbor in supported {
        let grid = grid_with_column_neighbors(neighbor.clone(), neighbor);
        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_or_else(|error| panic!("supported auto-repeat neighbor: {error:?}"));
        assert_eq!(projected.columns.len(), 3);
    }
}

#[test]
fn track_sizing_projection_accepts_fixed_minimum_flex_maximum_neighbors_on_both_axes() {
    let node = NodeId::new(11_135).unwrap();
    let neighbor = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(12.0),
        max: ComputedCssGridTrackBreadth::Fraction(0.5),
    };
    let expected = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(12.0),
        max: GridTrackMaxSizingFunction::Fraction(0.5),
    };

    let mut column_grid = grid_with_column_neighbors(neighbor.clone(), neighbor.clone());
    column_grid
        .columns_auto_repeat
        .as_mut()
        .unwrap()
        .fixed_repeat_minmax_track_indices = vec![0, 2];
    let columns = project_grid_layout(
        node,
        &column_grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert_eq!(columns.columns[0], expected);
    assert_eq!(columns.columns[2], expected);

    let mut row_grid = grid_with_row_neighbors(neighbor.clone(), neighbor.clone());
    row_grid
        .rows_auto_repeat
        .as_mut()
        .unwrap()
        .fixed_repeat_minmax_track_indices = vec![0, 2];
    let rows = project_grid_layout(
        node,
        &row_grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert_eq!(rows.rows[0], expected);
    assert_eq!(rows.rows[2], expected);
}

#[test]
fn track_sizing_projection_accepts_only_the_single_trailing_column_intrinsic_neighbor() {
    let node = NodeId::new(11_137).unwrap();
    let neighbor = min_content_with_fixed_maximum(40.0);
    for mode in [
        ComputedCssGridAutoRepeatMode::AutoFill,
        ComputedCssGridAutoRepeatMode::AutoFit,
    ] {
        let mut grid =
            grid_with_column_neighbors(ComputedCssGridTrackSize::LengthPx(8.0), neighbor.clone());
        grid.columns_auto_repeat.as_mut().unwrap().mode = mode;
        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_or_else(|error| panic!("supported trailing {mode:?} neighbor: {error:?}"));
        assert_eq!(
            projected.columns[2],
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::MinContent,
                max: GridTrackMaxSizingFunction::LengthPx(40.0),
            }
        );
    }
}

#[test]
fn track_sizing_projection_rejects_intrinsic_neighbor_outside_the_wpt_column_auto_fill_suffix() {
    let node = NodeId::new(11_138).unwrap();
    let neighbor = min_content_with_fixed_maximum(40.0);
    let mut multiple_suffix_tracks =
        grid_with_column_neighbors(ComputedCssGridTrackSize::LengthPx(8.0), neighbor.clone());
    multiple_suffix_tracks
        .columns
        .push(ComputedCssGridTrackSize::LengthPx(4.0));

    let invalid_maximums = [
        min_content_with_fixed_maximum(-1.0),
        min_content_with_fixed_maximum(f32::INFINITY),
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MaxContent,
            max: ComputedCssGridTrackBreadth::LengthPx(40.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::LengthPx(40.0),
        },
    ];
    let mut cases = vec![
        (
            "intrinsic prefix",
            grid_with_column_neighbors(neighbor.clone(), ComputedCssGridTrackSize::LengthPx(8.0)),
            "grid-template-columns",
        ),
        (
            "multiple suffix tracks",
            multiple_suffix_tracks,
            "grid-template-columns",
        ),
        (
            "row-axis suffix",
            grid_with_row_neighbors(ComputedCssGridTrackSize::LengthPx(8.0), neighbor),
            "grid-template-rows",
        ),
    ];
    for invalid in invalid_maximums {
        cases.push((
            "invalid intrinsic minimum or maximum",
            grid_with_column_neighbors(ComputedCssGridTrackSize::LengthPx(8.0), invalid),
            "grid-template-columns",
        ));
    }

    for (case, grid, property) in cases {
        assert!(
            matches!(
                project_grid_layout(
                    node,
                    &grid,
                    ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
                ),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node: error_node,
                    property: error_property,
                    ..
                }) if error_node == node && error_property == property
            ),
            "{case} must retain node and property context"
        );
    }
}

#[test]
fn track_sizing_projection_rejects_minmax_authored_neighbors_without_fixed_repeat_provenance() {
    let node = NodeId::new(11_136).unwrap();
    let neighbor = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(12.0),
        max: ComputedCssGridTrackBreadth::Fraction(0.5),
    };
    for grid in [
        grid_with_column_neighbors(neighbor.clone(), ComputedCssGridTrackSize::LengthPx(4.0)),
        grid_with_row_neighbors(neighbor, ComputedCssGridTrackSize::LengthPx(4.0)),
    ] {
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_err();
        assert!(matches!(
            error,
            StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-template-columns" | "grid-template-rows",
                ..
            } if error_node == node
        ));
    }
}

#[test]
fn track_sizing_projection_accepts_only_a_single_trailing_column_minmax_length_auto_neighbor() {
    let node = NodeId::new(11_139).unwrap();
    let neighbor = fixed_minimum_auto_maximum(60.0);
    let expected = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(60.0),
        max: GridTrackMaxSizingFunction::Auto,
    };

    for mode in [
        ComputedCssGridAutoRepeatMode::AutoFill,
        ComputedCssGridAutoRepeatMode::AutoFit,
    ] {
        let mut grid =
            grid_with_column_neighbors(ComputedCssGridTrackSize::LengthPx(10.0), neighbor.clone());
        grid.columns_auto_repeat.as_mut().unwrap().mode = mode;
        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_or_else(|error| panic!("supported trailing {mode:?} neighbor: {error:?}"));
        assert_eq!(projected.columns[2], expected);
    }

    let mut multiple_suffix_tracks =
        grid_with_column_neighbors(ComputedCssGridTrackSize::LengthPx(10.0), neighbor.clone());
    multiple_suffix_tracks
        .columns
        .push(ComputedCssGridTrackSize::LengthPx(4.0));

    let invalid = [
        (
            "prefix",
            grid_with_column_neighbors(neighbor.clone(), ComputedCssGridTrackSize::LengthPx(10.0)),
            "grid-template-columns",
        ),
        (
            "multiple suffix tracks",
            multiple_suffix_tracks,
            "grid-template-columns",
        ),
        (
            "row axis",
            grid_with_row_neighbors(ComputedCssGridTrackSize::LengthPx(10.0), neighbor.clone()),
            "grid-template-rows",
        ),
        (
            "negative minimum",
            grid_with_column_neighbors(
                ComputedCssGridTrackSize::LengthPx(10.0),
                fixed_minimum_auto_maximum(-1.0),
            ),
            "grid-template-columns",
        ),
        (
            "non-finite minimum",
            grid_with_column_neighbors(
                ComputedCssGridTrackSize::LengthPx(10.0),
                fixed_minimum_auto_maximum(f32::INFINITY),
            ),
            "grid-template-columns",
        ),
        (
            "NaN minimum",
            grid_with_column_neighbors(
                ComputedCssGridTrackSize::LengthPx(10.0),
                fixed_minimum_auto_maximum(f32::NAN),
            ),
            "grid-template-columns",
        ),
    ];
    for (case, grid, property) in invalid {
        assert!(
            matches!(
                project_grid_layout(
                    node,
                    &grid,
                    ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
                ),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node: error_node,
                    property: error_property,
                    ..
                }) if error_node == node && error_property == property
            ),
            "{case} must fail with node and property context"
        );
    }
}

#[test]
fn track_sizing_projection_preserves_css_property_context_for_unsupported_neighbors() {
    let node = NodeId::new(11_132).unwrap();
    let unsupported = [
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::LengthPx(10.0),
            ComputedCssMath::Percentage(0.05),
            ComputedCssMath::Clamp {
                min: Box::new(ComputedCssMath::LengthPx(0.0)),
                value: Box::new(ComputedCssMath::Percentage(0.0)),
                max: Box::new(ComputedCssMath::LengthPx(1.0)),
            },
        ])),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::LengthPx(10.0),
            ComputedCssMath::Negate(Box::new(ComputedCssMath::Negate(Box::new(
                ComputedCssMath::Percentage(0.05),
            )))),
        ])),
        calc_neighbor(ComputedCssMath::LengthPx(f32::NAN), 0.05),
        calc_neighbor(ComputedCssMath::LengthPx(10.0), f32::INFINITY),
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Fraction(1.0),
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(10.0),
            max: ComputedCssGridTrackBreadth::LengthPx(20.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Percentage(0.2),
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(-1.0),
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(10.0),
            max: ComputedCssGridTrackBreadth::Fraction(f32::INFINITY),
        },
    ];
    for neighbor in unsupported {
        let grid = grid_with_column_neighbors(neighbor, ComputedCssGridTrackSize::LengthPx(4.0));
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    node: error_node,
                    property: "grid-template-columns",
                    ..
                } if error_node == node
            ),
            "unsupported column neighbor must retain node/property: {error:?}"
        );
    }

    let grid = grid_with_row_neighbors(
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::LengthPx(10.0),
            ComputedCssMath::Product(vec![
                ComputedCssMath::Percentage(-0.05),
                ComputedCssMath::Number(1.0),
            ]),
        ])),
        ComputedCssGridTrackSize::LengthPx(4.0),
    );
    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        ),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-rows",
            ..
        }) if error_node == node
    ));
}

#[test]
fn track_sizing_projection_rejects_invalid_auto_repeat_ranges_before_slicing() {
    let node = NodeId::new(11_133).unwrap();
    let mut grid = grid_with_column_neighbors(
        ComputedCssGridTrackSize::LengthPx(4.0),
        ComputedCssGridTrackSize::LengthPx(4.0),
    );
    grid.columns_auto_repeat.as_mut().unwrap().end = usize::MAX;

    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        ),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-columns",
            ..
        }) if error_node == node
    ));
}

#[test]
fn supported_auto_repeat_neighbor_projection_keeps_calc_ids_on_their_css_axis() {
    let node = NodeId::new(11_134).unwrap();
    let neighbor = calc_neighbor(ComputedCssMath::LengthPx(10.0), 0.05);
    let grid = grid_with_column_neighbors(neighbor, ComputedCssGridTrackSize::LengthPx(4.0));
    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert!(matches!(&projected.columns[0], GridTrackSize::Calc(_)));
}
