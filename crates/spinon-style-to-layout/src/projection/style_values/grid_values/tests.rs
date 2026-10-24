use super::{
    GridLineNameReservationSite, project_grid_layout, project_grid_layout_with_math,
    project_grid_layout_with_reservation, project_grid_layout_with_reservations,
};
use crate::projection::typed_math::CssMathProjector;
use spinon_core::NodeId;
use spinon_layout::{
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutError,
};
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTemplateAreas, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

#[test]
fn track_sizing_projection_preserves_fixed_minimum_fraction_maximum() {
    let node = NodeId::new(11_119).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(12.0),
            max: ComputedCssGridTrackBreadth::Fraction(0.5),
        }],
        rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        vec![GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(12.0),
            max: GridTrackMaxSizingFunction::Fraction(0.5),
        }]
    );
}

#[test]
fn projection_reservation_failure_keeps_node_and_axis_context() {
    let node = NodeId::new(11_101).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };
    let mut calls = 0;
    let error = project_grid_layout_with_reservation(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
        |_, _| {
            calls += 1;
            if calls == 2 { Err(()) } else { Ok(()) }
        },
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
            node: error_node,
            property: "grid-template-rows",
        }) if error_node == node
    ));
}

#[test]
fn template_profile_projects_n_plus_one_line_names_and_dense_grid_contract() {
    let node = NodeId::new(11_103).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(30.0),
        ],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![
            vec!["start".to_owned()],
            vec!["seam-a".to_owned(), "seam-b".to_owned()],
            vec!["end".to_owned()],
        ],
        rows_line_names: vec![Vec::new(), vec!["row-end".to_owned()]],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::ColumnDense,
        auto_rows: ComputedCssGridAutoRows::LengthPx(12.0),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::LengthPx(16.0),
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap();

    assert_eq!(projected.columns_line_names, grid.columns_line_names);
    assert_eq!(projected.rows_line_names, grid.rows_line_names);
    assert_eq!(
        projected.auto_flow,
        spinon_layout::GridAutoFlow::ColumnDense
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::LengthPx(12.0)
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::LengthPx(16.0)
    );
    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::DenseAutoPlacement
    );
}

#[test]
fn template_area_profile_selects_template_grid_sizing_mode() {
    let node = NodeId::new(11_105).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: Some(ComputedCssGridTemplateAreas {
            rows: 1,
            columns: 1,
            areas: vec![ComputedCssGridArea {
                name: "area".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TemplateGrid
    );
}

#[test]
fn track_sizing_projects_areas_with_one_auto_repeat_axis_and_preserves_authored_size() {
    let node = NodeId::new(11_117).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 0,
            end: 1,
            line_names: vec![Vec::new(), Vec::new()],
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: Some(ComputedCssGridTemplateAreas {
            rows: 1,
            columns: 5,
            areas: vec![ComputedCssGridArea {
                name: "suffix".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 4,
                column_end: 5,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::LengthPx(20.0),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::LengthPx(20.0),
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TrackSizing
    );
    assert_eq!(
        projected.template_areas,
        Some(spinon_layout::GridTemplateAreasInput {
            rows: 1,
            columns: 5,
            areas: vec![spinon_layout::GridTemplateAreaInput {
                name: "suffix".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 4,
                column_end: 5,
            }],
        })
    );
}

#[test]
fn auto_repeat_template_areas_project_two_axes_and_reject_allocation_failures() {
    let node = NodeId::new(11_118).unwrap();
    let mut grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 0,
            end: 1,
            line_names: vec![Vec::new(), Vec::new()],
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: Some(ComputedCssGridTemplateAreas {
            rows: 1,
            columns: 1,
            areas: vec![ComputedCssGridArea {
                name: "area".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };
    grid.rows_auto_repeat = Some(ComputedCssGridAutoRepeat {
        mode: ComputedCssGridAutoRepeatMode::AutoFit,
        start: 0,
        end: 1,
        line_names: vec![Vec::new(), Vec::new()],
        fixed_repeat_minmax_track_indices: Vec::new(),
    });
    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert!(projected.columns_auto_repeat.is_some());
    assert!(projected.rows_auto_repeat.is_some());
    assert_eq!(
        projected.template_areas,
        Some(spinon_layout::GridTemplateAreasInput {
            rows: 1,
            columns: 1,
            areas: vec![spinon_layout::GridTemplateAreaInput {
                name: "area".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        })
    );
    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-columns",
            ..
        } if error_node == node
    ));

    grid.rows_auto_repeat = None;
    for site in [
        GridLineNameReservationSite::Areas,
        GridLineNameReservationSite::AreaName,
    ] {
        let mut math = CssMathProjector::default();
        let error = project_grid_layout_with_reservations(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            &mut math,
            |_, _| Ok(()),
            |requested| if requested == site { Err(()) } else { Ok(()) },
        )
        .unwrap_err();
        assert!(matches!(
            error,
            crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                node: error_node,
                property: "grid-template-areas",
            }) if error_node == node
        ));
    }
}

#[test]
fn paired_area_edge_aliases_on_both_axes_select_template_grid_sizing_mode() {
    let node = NodeId::new(11_106).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![vec!["box-start".to_owned()], vec!["box-end".to_owned()]],
        rows_line_names: vec![vec!["box-start".to_owned()], vec!["box-end".to_owned()]],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TemplateGrid
    );

    let mut one_axis_only = grid;
    one_axis_only.rows_line_names = vec![Vec::new(), Vec::new()];
    assert_eq!(
        project_grid_layout(
            node,
            &one_axis_only,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
        )
        .unwrap()
        .placement_mode,
        spinon_layout::GridPlacementMode::DenseAutoPlacement
    );
}

#[test]
fn template_profile_line_name_allocation_failures_keep_context_and_recover() {
    let node = NodeId::new(11_104).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![vec!["start".to_owned()], vec!["end".to_owned()]],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    for site in [
        GridLineNameReservationSite::Groups,
        GridLineNameReservationSite::Names,
        GridLineNameReservationSite::NameText,
    ] {
        let mut math = CssMathProjector::default();
        let error = project_grid_layout_with_reservations(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            &mut math,
            |_, _| Ok(()),
            |requested| if requested == site { Err(()) } else { Ok(()) },
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                    node: error_node,
                    property: "grid-template-columns",
                }) if error_node == node
            ),
            "예약 실패 지점: {site:?}, 실제 오류: {error:?}"
        );
        assert!(
            project_grid_layout(
                node,
                &grid,
                ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            )
            .is_ok()
        );
    }
}

#[test]
fn template_profile_rejects_line_name_groups_that_do_not_match_track_count() {
    let node = NodeId::new(11_104).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![vec!["only-one-boundary".to_owned()]],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-columns",
            ..
        }) if error_node == node
    ));
}

#[test]
fn track_sizing_profile_projects_typed_functions_and_axis_scoped_calc_ids() {
    let node = NodeId::new(11_107).unwrap();
    let expression = spinon_style::ComputedCssMath::Sum(vec![
        spinon_style::ComputedCssMath::LengthPx(12.0),
        spinon_style::ComputedCssMath::Percentage(0.25),
    ]);
    let grid = ComputedCssGridLayout {
        columns: vec![
            ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackSize::MinMax {
                min: spinon_style::ComputedCssGridTrackBreadth::MinContent,
                max: spinon_style::ComputedCssGridTrackBreadth::Fraction(2.0),
            },
            ComputedCssGridTrackSize::FitContent(
                spinon_style::ComputedCssGridTrackLengthPercentage::Calc(expression.clone()),
            ),
        ],
        rows: vec![ComputedCssGridTrackSize::Calc(expression)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(); 4],
        rows_line_names: vec![Vec::new(); 2],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };
    let mut math = CssMathProjector::default();

    let projected = project_grid_layout_with_math(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        &mut math,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        [
            spinon_layout::GridTrackSize::Auto,
            spinon_layout::GridTrackSize::MinMax {
                min: spinon_layout::GridTrackMinSizingFunction::MinContent,
                max: spinon_layout::GridTrackMaxSizingFunction::Fraction(2.0),
            },
            spinon_layout::GridTrackSize::FitContent(
                spinon_layout::GridTrackLengthPercentage::Calc(spinon_layout::LayoutCalcId(0))
            ),
        ]
    );
    assert_eq!(
        projected.rows,
        [spinon_layout::GridTrackSize::Calc(
            spinon_layout::LayoutCalcId(1)
        )]
    );
    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TrackSizing
    );
    let values = math.into_values();
    assert_eq!(values.len(), 2);
    assert_eq!(
        values
            .iter()
            .map(|value| value.property)
            .collect::<Vec<_>>(),
        [
            spinon_layout::LayoutCssMathProperty::GridTemplateColumns,
            spinon_layout::LayoutCssMathProperty::GridTemplateRows,
        ]
    );
}

#[test]
fn track_sizing_profile_projects_calc_minimums_on_their_own_axes() {
    let node = NodeId::new(11_110).unwrap();
    let column_calc = spinon_style::ComputedCssMath::Sum(vec![
        spinon_style::ComputedCssMath::LengthPx(4.0),
        spinon_style::ComputedCssMath::Percentage(2.0),
    ]);
    let row_calc = spinon_style::ComputedCssMath::Sum(vec![
        spinon_style::ComputedCssMath::LengthPx(2.0),
        spinon_style::ComputedCssMath::Percentage(2.0),
    ]);
    let minmax = |expression| ComputedCssGridTrackSize::MinMax {
        min: spinon_style::ComputedCssGridTrackBreadth::Calc(expression),
        max: spinon_style::ComputedCssGridTrackBreadth::MaxContent,
    };
    let grid = ComputedCssGridLayout {
        columns: vec![minmax(column_calc)],
        rows: vec![minmax(row_calc)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(); 2],
        rows_line_names: vec![Vec::new(); 2],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };
    let mut math = CssMathProjector::default();

    let projected = project_grid_layout_with_math(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        &mut math,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        [GridTrackSize::MinMax {
            min: spinon_layout::GridTrackMinSizingFunction::Calc(spinon_layout::LayoutCalcId(0)),
            max: spinon_layout::GridTrackMaxSizingFunction::MaxContent,
        }]
    );
    assert_eq!(
        projected.rows,
        [GridTrackSize::MinMax {
            min: spinon_layout::GridTrackMinSizingFunction::Calc(spinon_layout::LayoutCalcId(1)),
            max: spinon_layout::GridTrackMaxSizingFunction::MaxContent,
        }]
    );
    assert_eq!(
        math.into_values()
            .iter()
            .map(|value| value.property)
            .collect::<Vec<_>>(),
        [
            spinon_layout::LayoutCssMathProperty::GridTemplateColumns,
            spinon_layout::LayoutCssMathProperty::GridTemplateRows,
        ]
    );
}

#[test]
fn track_sizing_profile_preserves_cyclic_fit_content_percentage_limit() {
    let node = NodeId::new(11_108).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::FitContent(
            spinon_style::ComputedCssGridTrackLengthPercentage::Percentage(0.5),
        )],
        rows: vec![ComputedCssGridTrackSize::Auto],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(); 2],
        rows_line_names: vec![Vec::new(); 2],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        [GridTrackSize::FitContent(
            spinon_layout::GridTrackLengthPercentage::Percentage(0.5)
        )]
    );
}

#[test]
fn track_sizing_profile_projects_independent_auto_repeat_modes_on_both_axes() {
    let node = NodeId::new(11_108).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::MinMax {
                min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(24.0),
                max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            ComputedCssGridTrackSize::LengthPx(10.0),
        ],
        rows: vec![
            ComputedCssGridTrackSize::LengthPx(12.0),
            ComputedCssGridTrackSize::MinMax {
                min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(32.0),
                max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            ComputedCssGridTrackSize::LengthPx(8.0),
        ],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: Vec::new(),
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: Vec::new(),
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        columns_line_names: Vec::new(),
        rows_line_names: Vec::new(),
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns_auto_repeat,
        Some(spinon_layout::GridAutoRepeat {
            mode: spinon_layout::GridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: Vec::new(),
        })
    );
    assert_eq!(
        projected.rows_auto_repeat,
        Some(spinon_layout::GridAutoRepeat {
            mode: spinon_layout::GridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: Vec::new(),
        })
    );
    assert!(projected.columns_line_names.is_empty());
    assert_eq!(projected.columns.len(), 3);
    assert_eq!(projected.columns[0], GridTrackSize::LengthPx(20.0));
    assert_eq!(projected.columns[2], GridTrackSize::LengthPx(10.0));
}

#[test]
fn track_sizing_profile_projects_auto_repeat_inner_and_component_names() {
    let node = NodeId::new(11_110).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
        ],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: vec![vec!["item-start".into()], vec!["item-end".into()]],
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: vec![
            vec!["container-start".into()],
            vec!["repeat-start".into()],
            vec!["repeat-end".into()],
            vec!["container-end".into()],
        ],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(projected.columns_line_names, grid.columns_line_names);
    assert_eq!(
        projected.columns_auto_repeat.unwrap().line_names,
        vec![
            vec![String::from("item-start")],
            vec![String::from("item-end")],
        ]
    );
}

#[test]
fn track_sizing_profile_keeps_single_axis_auto_repeat_projection() {
    let node = NodeId::new(11_109).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::MinMax {
                min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(24.0),
                max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            ComputedCssGridTrackSize::LengthPx(10.0),
        ],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
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
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns_auto_repeat,
        Some(spinon_layout::GridAutoRepeat {
            mode: spinon_layout::GridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: Vec::new(),
        })
    );
    assert_eq!(projected.rows_auto_repeat, None);
    assert_eq!(projected.columns.len(), 3);
    assert_eq!(projected.columns[0], GridTrackSize::LengthPx(20.0));
    assert_eq!(projected.columns[2], GridTrackSize::LengthPx(10.0));
    assert_eq!(projected.rows, [GridTrackSize::LengthPx(40.0)]);
    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TrackSizing
    );
}

#[test]
fn auto_fit_projection_preserves_area_dimensions_beyond_both_repeat_axes() {
    let node = NodeId::new(11_122).unwrap();
    let auto_fit = || {
        Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 0,
            end: 1,
            line_names: vec![Vec::new(), Vec::new()],
            fixed_repeat_minmax_track_indices: Vec::new(),
        })
    };
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        columns_auto_repeat: auto_fit(),
        rows_auto_repeat: auto_fit(),
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: Some(ComputedCssGridTemplateAreas {
            rows: 3,
            columns: 3,
            areas: vec![ComputedCssGridArea {
                name: "hero".to_owned(),
                row_start: 2,
                row_end: 3,
                column_start: 2,
                column_end: 3,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::LengthPx(12.0),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::LengthPx(16.0),
    };

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected
            .columns_auto_repeat
            .as_ref()
            .map(|repeat| repeat.mode),
        Some(spinon_layout::GridAutoRepeatMode::AutoFit)
    );
    assert_eq!(
        projected
            .rows_auto_repeat
            .as_ref()
            .map(|repeat| repeat.mode),
        Some(spinon_layout::GridAutoRepeatMode::AutoFit)
    );
    assert_eq!(
        projected
            .template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((3, 3))
    );
    assert_eq!(
        projected.template_areas,
        Some(spinon_layout::GridTemplateAreasInput {
            rows: 3,
            columns: 3,
            areas: vec![spinon_layout::GridTemplateAreaInput {
                name: "hero".to_owned(),
                row_start: 2,
                row_end: 3,
                column_start: 2,
                column_end: 3,
            }],
        })
    );
    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::TrackSizing
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::LengthPx(16.0)
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::LengthPx(12.0)
    );
}
