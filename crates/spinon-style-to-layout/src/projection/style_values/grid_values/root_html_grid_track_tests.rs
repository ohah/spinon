use super::{project_grid_layout, project_grid_layout_with_math};
use crate::projection::typed_math::CssMathProjector;
use spinon_core::NodeId;
use spinon_layout::GridTrackSize;
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTemplateAreas,
    ComputedCssGridTrackSize, ComputedStyleProfile,
};

#[path = "root_html_grid_track_tests/fit_content.rs"]
mod fit_content;
#[path = "root_html_grid_track_tests/fr_auto_columns.rs"]
mod fr_auto_columns;
#[path = "root_html_grid_track_tests/implicit_rows_composition.rs"]
mod implicit_rows_composition;
#[path = "root_html_grid_track_tests/minmax.rs"]
mod minmax;

fn grid(
    columns: Vec<ComputedCssGridTrackSize>,
    rows: Vec<ComputedCssGridTrackSize>,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns_line_names: vec![Vec::new(); columns.len() + 1],
        rows_line_names: vec![Vec::new(); rows.len() + 1],
        columns,
        rows,
        columns_fixed_repeat_group_count: 0,
        rows_fixed_repeat_group_count: 0,
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    }
}

#[test]
fn root_html_column_flow_selects_its_restricted_layout_mode() {
    let node = NodeId::new(11_240).unwrap();
    let mut input = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(30.0),
            ComputedCssGridTrackSize::LengthPx(70.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
    );
    input.auto_flow = ComputedCssGridAutoFlow::Column;

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlColumnFlowV1
    );
    assert_eq!(projected.auto_flow, spinon_layout::GridAutoFlow::Column);
    assert_eq!(projected.auto_rows, spinon_layout::GridAutoRows::Auto);
    assert_eq!(projected.auto_columns, spinon_layout::GridAutoColumns::Auto);
}

#[test]
fn root_html_column_flow_projects_direct_fixed_px_auto_columns() {
    let node = NodeId::new(11_243).unwrap();
    for (auto_columns, expected) in [
        (
            ComputedCssGridAutoColumns::LengthPx(15.0),
            spinon_layout::GridAutoColumns::LengthPx(15.0),
        ),
        (
            ComputedCssGridAutoColumns::TrackList(vec![
                ComputedCssGridTrackSize::LengthPx(15.0),
                ComputedCssGridTrackSize::LengthPx(25.0),
            ]),
            spinon_layout::GridAutoColumns::TrackList(vec![
                GridTrackSize::LengthPx(15.0),
                GridTrackSize::LengthPx(25.0),
            ]),
        ),
    ] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(30.0)],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        );
        input.auto_flow = ComputedCssGridAutoFlow::Column;
        input.auto_columns = auto_columns;

        let projected = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .unwrap();

        assert_eq!(projected.auto_columns, expected);
        assert_eq!(
            projected.placement_mode,
            spinon_layout::GridPlacementMode::RootHtmlColumnFlowV1
        );
    }
}

#[test]
fn root_html_column_flow_projects_direct_percentage_auto_column_lists() {
    let node = NodeId::new(11_246).unwrap();
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(30.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    input.auto_flow = ComputedCssGridAutoFlow::Column;
    input.auto_columns = ComputedCssGridAutoColumns::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(10.0),
        ComputedCssGridTrackSize::Percentage(0.15),
    ]);

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("direct px/percentage auto-column pattern should project in column-flow mode");
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(10.0),
            GridTrackSize::Percentage(0.15),
        ])
    );
}

#[test]
fn root_html_column_flow_accepts_auto_column_list_at_track_limit() {
    let node = NodeId::new(11_245).unwrap();
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(30.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    input.auto_flow = ComputedCssGridAutoFlow::Column;
    input.auto_columns = ComputedCssGridAutoColumns::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(1.0);
        spinon_layout::GRID_MAX_TRACKS
    ]);

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("an auto-column pattern at the shared track limit should be accepted");
    assert!(matches!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(tracks)
            if tracks.len() == spinon_layout::GRID_MAX_TRACKS
    ));
}

#[test]
fn root_html_column_flow_rejects_unsupported_auto_columns_and_row_flow_sizing() {
    let node = NodeId::new(11_244).unwrap();
    for auto_columns in [
        ComputedCssGridAutoColumns::LengthPx(-1.0),
        ComputedCssGridAutoColumns::LengthPx(f32::NAN),
        ComputedCssGridAutoColumns::LengthPx(f32::INFINITY),
        ComputedCssGridAutoColumns::TrackList(Vec::new()),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Percentage(-0.5)]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Percentage(f32::NAN)]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Percentage(
            f32::INFINITY,
        )]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Fraction(-0.5)]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Fraction(f32::NAN)]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Fraction(
            f32::INFINITY,
        )]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Calc(
            spinon_style::ComputedCssMath::LengthPx(15.0),
        )]),
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::LengthPx(-1.0)]),
        ComputedCssGridAutoColumns::Unsupported("15%".to_owned()),
    ] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(30.0)],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        );
        input.auto_flow = ComputedCssGridAutoFlow::Column;
        input.auto_columns = auto_columns;
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                property: "grid-auto-columns",
                ..
            })
        ));
    }

    let mut row_flow = grid(
        vec![ComputedCssGridTrackSize::LengthPx(30.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    row_flow.auto_columns = ComputedCssGridAutoColumns::LengthPx(15.0);
    assert!(matches!(
        project_grid_layout(
            node,
            &row_flow,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            property: "grid-auto-columns",
            ..
        })
    ));
}

#[test]
fn root_html_column_flow_rejects_dense_implicit_and_non_px_combinations() {
    let node = NodeId::new(11_241).unwrap();
    for flow in [
        ComputedCssGridAutoFlow::RowDense,
        ComputedCssGridAutoFlow::ColumnDense,
    ] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(30.0)],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        );
        input.auto_flow = flow;
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                property: "grid-auto-flow",
                ..
            })
        ));
    }

    let mut percentage = grid(
        vec![ComputedCssGridTrackSize::Percentage(0.5)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    percentage.auto_flow = ComputedCssGridAutoFlow::Column;
    assert!(matches!(
        project_grid_layout(
            node,
            &percentage,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            property: "grid-template-columns",
            ..
        })
    ));

    let mut implicit = grid(
        vec![ComputedCssGridTrackSize::LengthPx(30.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    implicit.auto_flow = ComputedCssGridAutoFlow::Column;
    implicit.auto_rows = ComputedCssGridAutoRows::LengthPx(12.0);
    assert!(matches!(
        project_grid_layout(
            node,
            &implicit,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            property: "grid-auto-flow",
            ..
        })
    ));
}

#[test]
fn root_html_column_flow_rejects_fixed_repeat_groups_on_both_axes() {
    let node = NodeId::new(11_242).unwrap();
    for (property, columns, rows) in [
        ("grid-template-columns", 1, 0),
        ("grid-template-rows", 0, 1),
    ] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(30.0)],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        );
        input.auto_flow = ComputedCssGridAutoFlow::Column;
        input.columns_fixed_repeat_group_count = columns;
        input.rows_fixed_repeat_group_count = rows;
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                property: actual,
                ..
            }) if actual == property
        ));
    }
}

#[test]
fn root_html_profile_projects_direct_fraction_tracks_on_both_axes() {
    let node = NodeId::new(11_181).unwrap();
    let grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(30.0),
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ],
        vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ],
    );
    let mut math = CssMathProjector::default();

    let projected = project_grid_layout_with_math(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        &mut math,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::Fraction(1.0),
            GridTrackSize::Fraction(2.0),
        ]
    );
    assert_eq!(
        projected.rows,
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)]
    );
}

#[test]
fn root_html_profile_projects_direct_percentage_tracks_on_both_axes() {
    let node = NodeId::new(11_184).unwrap();
    let grid = grid(
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        vec![
            ComputedCssGridTrackSize::Percentage(0.5),
            ComputedCssGridTrackSize::Percentage(0.0),
        ],
    );
    let mut math = CssMathProjector::default();

    let projected = project_grid_layout_with_math(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        &mut math,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::Fraction(1.0),
        ]
    );
    assert_eq!(
        projected.rows,
        vec![
            GridTrackSize::Percentage(0.5),
            GridTrackSize::Percentage(0.0)
        ]
    );
}

#[test]
fn root_html_profile_projects_one_fixed_implicit_row_size_in_its_own_mode() {
    let node = NodeId::new(11_187).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(55.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    grid.auto_rows = ComputedCssGridAutoRows::LengthPx(15.0);
    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::LengthPx(15.0)
    );
    assert_eq!(
        projected.columns,
        vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(55.0)]
    );
    assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(20.0)]);
}

#[test]
fn root_html_profile_projects_fixed_px_implicit_row_track_lists_in_its_own_mode() {
    let node = NodeId::new(11_190).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(55.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    grid.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(15.0),
        ComputedCssGridTrackSize::LengthPx(25.0),
    ]);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(25.0),
        ])
    );
    assert_eq!(projected.columns.len(), 2);
    assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(20.0)]);
}

#[test]
fn root_html_profile_projects_mixed_px_percentage_fractional_implicit_row_track_lists() {
    let node = NodeId::new(11_195).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(55.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    grid.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(15.0),
        ComputedCssGridTrackSize::Percentage(0.25),
        ComputedCssGridTrackSize::Fraction(1.0),
    ]);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0),
        ])
    );
}

#[test]
fn root_html_percentage_auto_row_list_composes_with_explicit_percentage_and_fraction_tracks() {
    let node = NodeId::new(11_197).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
    );
    grid.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(5.0),
        ComputedCssGridTrackSize::Percentage(0.1),
    ]);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("percentage implicit rows should compose with existing direct percentage/fr tracks");

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0)
        ]
    );
    assert_eq!(
        projected.rows,
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Fraction(1.0)
        ]
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(5.0),
            GridTrackSize::Percentage(0.1),
        ])
    );
}

#[test]
fn root_html_profile_rejects_empty_invalid_or_unsupported_implicit_row_lists() {
    let node = NodeId::new(11_191).unwrap();
    for auto_rows in [
        ComputedCssGridAutoRows::TrackList(Vec::new()),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::LengthPx(-1.0)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::LengthPx(f32::NAN)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::LengthPx(f32::INFINITY)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Percentage(-0.5)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Percentage(f32::NAN)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Percentage(
            f32::INFINITY,
        )]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Fraction(-1.0)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Fraction(f32::NAN)]),
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Fraction(f32::INFINITY)]),
    ] {
        let mut grid = grid(
            vec![
                ComputedCssGridTrackSize::LengthPx(40.0),
                ComputedCssGridTrackSize::LengthPx(55.0),
            ],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        );
        grid.auto_rows = auto_rows;
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("unsupported Root HTML implicit-row lists must fail closed");
        assert!(matches!(
            error,
            crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            } if error_node == node
        ));
    }
}

#[test]
fn root_html_profile_rejects_oversized_implicit_row_lists() {
    let node = NodeId::new(11_193).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(55.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    grid.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(1.0);
            spinon_layout::GRID_MAX_TRACKS + 1
        ]);

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("auto-row lists above the shared Grid track limit must fail closed");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        } if error_node == node
    ));
}

#[test]
fn root_html_named_area_profile_rejects_fixed_implicit_rows() {
    let node = NodeId::new(11_189).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(80.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(50.0),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![ComputedCssGridArea {
            name: "header".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 2,
        }],
    });
    grid.auto_rows = ComputedCssGridAutoRows::LengthPx(15.0);

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("fixed implicit rows must not widen the named-area profile");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        } if error_node == node
    ));
}

#[test]
fn root_html_named_area_profile_rejects_fixed_implicit_row_track_lists() {
    let node = NodeId::new(11_192).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(80.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(50.0),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![ComputedCssGridArea {
            name: "header".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 2,
        }],
    });
    grid.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(15.0),
        ComputedCssGridTrackSize::LengthPx(25.0),
    ]);

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("fixed implicit-row lists must not widen the named-area profile");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        } if error_node == node
    ));
}

#[test]
fn root_html_named_area_profile_rejects_percentage_implicit_row_track_lists() {
    let node = NodeId::new(11_196).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(80.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(50.0),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![ComputedCssGridArea {
            name: "header".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 2,
        }],
    });
    grid.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(15.0),
        ComputedCssGridTrackSize::Percentage(0.25),
    ]);

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("percentage implicit rows must not widen the named-area profile");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        } if error_node == node
    ));
}

#[test]
fn root_html_profile_rejects_invalid_or_nondirect_fraction_tracks() {
    let node = NodeId::new(11_182).unwrap();
    for track in [
        ComputedCssGridTrackSize::Fraction(-0.25),
        ComputedCssGridTrackSize::Fraction(f32::NAN),
        ComputedCssGridTrackSize::Fraction(f32::INFINITY),
        ComputedCssGridTrackSize::Fraction(f32::NEG_INFINITY),
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
            max: spinon_style::ComputedCssGridTrackBreadth::LengthPx(40.0),
        },
    ] {
        let grid = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(40.0)]);
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("out-of-scope fraction tracks must fail closed");
        assert!(matches!(
            error,
            crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-template-columns",
                ..
            } if error_node == node
        ));
    }
}

#[test]
fn root_html_profile_rejects_invalid_or_nondirect_percentage_tracks() {
    let node = NodeId::new(11_185).unwrap();
    for track in [
        ComputedCssGridTrackSize::Percentage(-0.25),
        ComputedCssGridTrackSize::Percentage(f32::NAN),
        ComputedCssGridTrackSize::Percentage(f32::INFINITY),
        ComputedCssGridTrackSize::Percentage(f32::NEG_INFINITY),
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(0.0),
            max: spinon_style::ComputedCssGridTrackBreadth::Auto,
        },
    ] {
        let grid = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(40.0)]);
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("out-of-scope percentage tracks must fail closed");
        assert!(matches!(
            error,
            crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-template-columns",
                ..
            } if error_node == node
        ));
    }
}

#[test]
fn root_html_named_area_profile_projects_the_bounded_two_axis_fraction_tracks() {
    let node = NodeId::new(11_182).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ],
        vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![
            ComputedCssGridArea {
                name: "header".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 2,
            },
            ComputedCssGridArea {
                name: "side".to_owned(),
                row_start: 1,
                row_end: 2,
                column_start: 0,
                column_end: 1,
            },
            ComputedCssGridArea {
                name: "content".to_owned(),
                row_start: 1,
                row_end: 2,
                column_start: 1,
                column_end: 2,
            },
        ],
    });

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("the exact two-axis fraction/area composition should project");

    assert_eq!(
        projected.columns,
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)]
    );
    assert_eq!(
        projected.rows,
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)]
    );
    assert_eq!(
        projected
            .template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 2))
    );
}

#[test]
fn root_html_named_area_profile_still_rejects_mixed_fraction_and_fixed_tracks() {
    let node = NodeId::new(11_183).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::LengthPx(80.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(50.0),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![ComputedCssGridArea {
            name: "header".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 2,
        }],
    });

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("mixed fraction and fixed tracks must not widen the named-area subset");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            ..
        } if error_node == node
    ));
}

#[test]
fn root_html_named_area_profile_projects_direct_px_percentage_track_pairs() {
    let node = NodeId::new(11_190).unwrap();
    let cases = [
        (
            vec![
                ComputedCssGridTrackSize::Percentage(0.25),
                ComputedCssGridTrackSize::Percentage(0.75),
            ],
            vec![
                ComputedCssGridTrackSize::Percentage(0.2),
                ComputedCssGridTrackSize::Percentage(0.8),
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(40.0),
                ComputedCssGridTrackSize::Percentage(0.5),
            ],
            vec![
                ComputedCssGridTrackSize::Percentage(0.2),
                ComputedCssGridTrackSize::LengthPx(50.0),
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::Percentage(1.2),
                ComputedCssGridTrackSize::Percentage(0.1),
            ],
            vec![
                ComputedCssGridTrackSize::LengthPx(14.0),
                ComputedCssGridTrackSize::LengthPx(56.0),
            ],
        ),
    ];

    let to_layout_track = |track| match track {
        ComputedCssGridTrackSize::LengthPx(value) => GridTrackSize::LengthPx(value),
        ComputedCssGridTrackSize::Percentage(value) => GridTrackSize::Percentage(value),
        _ => unreachable!("the positive matrix contains only direct px/percentage tracks"),
    };
    for (columns, rows) in cases {
        let mut grid = grid(columns.clone(), rows.clone());
        grid.template_areas = Some(ComputedCssGridTemplateAreas {
            rows: 2,
            columns: 2,
            areas: vec![
                ComputedCssGridArea {
                    name: "header".to_owned(),
                    row_start: 0,
                    row_end: 1,
                    column_start: 0,
                    column_end: 2,
                },
                ComputedCssGridArea {
                    name: "side".to_owned(),
                    row_start: 1,
                    row_end: 2,
                    column_start: 0,
                    column_end: 1,
                },
                ComputedCssGridArea {
                    name: "content".to_owned(),
                    row_start: 1,
                    row_end: 2,
                    column_start: 1,
                    column_end: 2,
                },
            ],
        });

        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect("the bounded direct px/percentage area composition should project");
        assert_eq!(
            projected.columns,
            columns.into_iter().map(to_layout_track).collect::<Vec<_>>()
        );
        assert_eq!(
            projected.rows,
            rows.into_iter().map(to_layout_track).collect::<Vec<_>>()
        );
        assert_eq!(
            projected
                .template_areas
                .as_ref()
                .map(|areas| (areas.rows, areas.columns)),
            Some((2, 2))
        );
    }
}

#[test]
fn root_html_named_area_profile_still_rejects_percentage_fraction_composition() {
    let node = NodeId::new(11_186).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Percentage(0.75),
        ],
    );
    grid.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 2,
        areas: vec![
            ComputedCssGridArea {
                name: "header".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 2,
            },
            ComputedCssGridArea {
                name: "side".to_owned(),
                row_start: 1,
                row_end: 2,
                column_start: 0,
                column_end: 1,
            },
            ComputedCssGridArea {
                name: "content".to_owned(),
                row_start: 1,
                row_end: 2,
                column_start: 1,
                column_end: 2,
            },
        ],
    });

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect_err("percentage and fr tracks must remain disjoint in the named-area subset");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            ..
        } if error_node == node
    ));
}
