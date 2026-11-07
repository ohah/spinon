use super::super::root_html_fit_content_implicit_rows_candidate;
use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackLengthPercentage, GridTrackSize};
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridTemplateAreas, ComputedCssGridTrackLengthPercentage,
    ComputedCssGridTrackSize, ComputedStyleProfile,
};

fn fit_content(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(value))
}

fn fit_content_percentage(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(value))
}

fn one_row_two_area_grid(
    columns: Vec<ComputedCssGridTrackSize>,
    rows: Vec<ComputedCssGridTrackSize>,
    names: [&str; 2],
) -> spinon_style::ComputedCssGridLayout {
    let mut input = grid(columns, rows);
    input.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 1,
        columns: 2,
        areas: vec![
            ComputedCssGridArea {
                name: names[0].to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            },
            ComputedCssGridArea {
                name: names[1].to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 1,
                column_end: 2,
            },
        ],
    });
    input
}

#[test]
fn root_html_explicit_fit_content_px_tracks_project_for_columns_rows_and_fixed_repeat() {
    let node = NodeId::new(11_280).unwrap();
    let column_grid = grid(
        vec![fit_content(40.0), ComputedCssGridTrackSize::LengthPx(10.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    let projected = project_grid_layout(
        node,
        &column_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("finite non-negative px fit-content columns should project");
    assert_eq!(
        projected.columns,
        [
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
            GridTrackSize::LengthPx(10.0),
        ]
    );
    let zero_limit_grid = grid(
        vec![fit_content(0.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    let projected = project_grid_layout(
        node,
        &zero_limit_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("zero is a valid non-negative fit-content px limit");
    assert_eq!(
        projected.columns,
        [GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(0.0)
        )]
    );

    let row_grid = grid(
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        vec![fit_content(20.0)],
    );
    let projected = project_grid_layout(
        node,
        &row_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("finite non-negative px fit-content rows should project");
    assert_eq!(
        projected.rows,
        [GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(20.0)
        )]
    );

    let mut repeated_grid = grid(
        vec![fit_content(40.0), fit_content(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(60.0)],
    );
    repeated_grid.columns_fixed_repeat_group_count = 1;
    let projected = project_grid_layout(
        node,
        &repeated_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("expanded fixed repeat fit-content tracks should use the same admission");
    assert_eq!(
        projected.columns,
        [
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
        ]
    );
}

#[test]
fn root_html_explicit_fit_content_percentage_tracks_project_for_columns_rows_and_fixed_repeat() {
    let node = NodeId::new(11_284).unwrap();
    let column_grid = grid(
        vec![
            fit_content_percentage(0.2),
            ComputedCssGridTrackSize::LengthPx(10.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    let projected = project_grid_layout(
        node,
        &column_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("finite non-negative percentage fit-content columns should project");
    assert_eq!(
        projected.columns,
        [
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::LengthPx(10.0),
        ]
    );

    let row_grid = grid(
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
        vec![fit_content_percentage(0.0), fit_content_percentage(1.2)],
    );
    let projected = project_grid_layout(
        node,
        &row_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("zero and over-100% fit-content rows should project as typed percentages");
    assert_eq!(
        projected.rows,
        [
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.0)),
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(1.2)),
        ]
    );

    let mut repeated_grid = grid(
        vec![fit_content_percentage(0.2), fit_content_percentage(0.2)],
        vec![ComputedCssGridTrackSize::LengthPx(60.0)],
    );
    repeated_grid.columns_fixed_repeat_group_count = 1;
    let projected = project_grid_layout(
        node,
        &repeated_grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("expanded fixed-repeat percentage fit-content tracks should project");
    assert_eq!(
        projected.columns,
        [
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
        ]
    );
}

#[test]
fn root_html_fit_content_percentage_auto_rows_require_the_bounded_definite_grid_shape() {
    use spinon_style::ComputedCssGridAutoRows;

    let node = NodeId::new(11_285).unwrap();
    for ratio in [0.0, 0.2, 0.34, 0.4, 1.2] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            vec![ComputedCssGridTrackSize::LengthPx(10.0)],
        );
        input.auto_rows = ComputedCssGridAutoRows::TrackList(vec![fit_content_percentage(ratio)]);

        let projected = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect("a direct finite percentage cap should select the bounded implicit-row route");

        assert_eq!(
            projected.placement_mode,
            spinon_layout::GridPlacementMode::RootHtmlFitContentAutoRowsV1
        );
        assert_eq!(
            projected.auto_rows,
            spinon_layout::GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::Percentage(ratio)
            )])
        );
    }

    for ratio in [-0.01, f32::NAN, f32::INFINITY] {
        let mut input = grid(
            vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            vec![ComputedCssGridTrackSize::LengthPx(10.0)],
        );
        input.auto_rows = ComputedCssGridAutoRows::TrackList(vec![fit_content_percentage(ratio)]);
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }
}

#[test]
fn root_html_fit_content_auto_row_lists_preserve_supported_track_kinds_and_order() {
    use spinon_style::ComputedCssGridAutoRows;

    let node = NodeId::new(11_286).unwrap();
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    input.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        fit_content_percentage(0.2),
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::LengthPx(12.0),
        ComputedCssGridTrackSize::Percentage(0.1),
    ]);

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("auto, finite px, and percentage entries should select the bounded row route");
    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlFitContentAutoRowsV1
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::Auto,
            GridTrackSize::LengthPx(12.0),
            GridTrackSize::Percentage(0.1),
        ])
    );

    let mut empty = input.clone();
    empty.auto_rows = ComputedCssGridAutoRows::TrackList(Vec::new());
    assert!(matches!(
        project_grid_layout(node, &empty, ComputedStyleProfile::RuntimeRootHtmlTextPaintV1),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));
    let mut without_fit_content = input.clone();
    without_fit_content.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::LengthPx(12.0),
        ComputedCssGridTrackSize::Percentage(0.1),
    ]);
    assert!(!root_html_fit_content_implicit_rows_candidate(
        &without_fit_content
    ));
    without_fit_content.auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::LengthPx(12.0),
    ]);
    assert!(matches!(
        project_grid_layout(
            node,
            &without_fit_content,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));

    for unsupported in [
        ComputedCssGridTrackSize::Fraction(1.0),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::LengthPx(-1.0),
        ComputedCssGridTrackSize::LengthPx(f32::NAN),
        ComputedCssGridTrackSize::Percentage(f32::INFINITY),
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(0.0),
            max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
        },
        ComputedCssGridTrackSize::Percentage(-0.1),
    ] {
        let mut invalid = input.clone();
        invalid.auto_rows =
            ComputedCssGridAutoRows::TrackList(vec![fit_content_percentage(0.2), unsupported]);
        assert!(matches!(
            project_grid_layout(
                node,
                &invalid,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }

    let mut oversized = input;
    let mut tracks =
        vec![ComputedCssGridTrackSize::LengthPx(1.0); spinon_layout::GRID_MAX_TRACKS + 1];
    tracks[0] = fit_content_percentage(0.2);
    oversized.auto_rows = ComputedCssGridAutoRows::TrackList(tracks);
    assert!(matches!(
        project_grid_layout(
            node,
            &oversized,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));
}

#[test]
fn root_html_dimension_matched_fit_content_named_areas_do_not_depend_on_area_names() {
    let node = NodeId::new(11_281).unwrap();
    for names in [["fixed", "fit"], ["leading", "trailing"]] {
        let input = one_row_two_area_grid(
            vec![ComputedCssGridTrackSize::LengthPx(20.0), fit_content(40.0)],
            vec![ComputedCssGridTrackSize::LengthPx(20.0)],
            names,
        );
        let projected = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect("dimension-matched px/fit-content named areas should project by shape");
        assert_eq!(
            projected.columns,
            [
                GridTrackSize::LengthPx(20.0),
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
            ]
        );
        assert_eq!(
            projected
                .template_areas
                .as_ref()
                .unwrap()
                .areas
                .iter()
                .map(|area| area.name.as_str())
                .collect::<Vec<_>>(),
            names
        );
    }
}

#[test]
fn root_html_fit_content_rejects_calc_or_invalid_limits() {
    let node = NodeId::new(11_282).unwrap();
    for track in [
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Calc(
            spinon_style::ComputedCssMath::LengthPx(20.0),
        )),
        fit_content_percentage(-0.5),
        fit_content_percentage(f32::NAN),
        fit_content_percentage(f32::INFINITY),
        fit_content_percentage(f32::NEG_INFINITY),
        fit_content(-1.0),
        fit_content(f32::NAN),
        fit_content(f32::INFINITY),
        fit_content(f32::NEG_INFINITY),
    ] {
        let input = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(20.0)]);
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                property: "grid-template-columns",
                ..
            })
        ));
    }
}

#[test]
fn root_html_named_area_fit_content_requires_matched_dimensions_and_supported_tracks() {
    let node = NodeId::new(11_283).unwrap();
    let accepted_rows = vec![ComputedCssGridTrackSize::LengthPx(20.0)];
    let cases = [
        one_row_two_area_grid(
            vec![ComputedCssGridTrackSize::LengthPx(20.0), fit_content(40.0)],
            accepted_rows.clone(),
            ["one", "two"],
        ),
        one_row_two_area_grid(
            vec![ComputedCssGridTrackSize::LengthPx(20.0), fit_content(40.0)],
            accepted_rows.clone(),
            ["wide", "narrow"],
        ),
    ];
    for input in cases {
        assert!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            )
            .is_ok()
        );
    }

    let mut mismatched = one_row_two_area_grid(
        vec![ComputedCssGridTrackSize::LengthPx(20.0), fit_content(40.0)],
        accepted_rows.clone(),
        ["left", "right"],
    );
    mismatched.template_areas.as_mut().unwrap().columns = 3;
    assert!(matches!(
        project_grid_layout(
            node,
            &mismatched,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            property: "grid-template-areas",
            ..
        })
    ));

    for (columns, expected_property) in [
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            "grid-template-areas",
        ),
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::Percentage(0.5),
            ],
            "grid-template-areas",
        ),
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            "grid-template-areas",
        ),
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                fit_content_percentage(0.5),
            ],
            "grid-template-areas",
        ),
        (
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::MinMax {
                    min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(1.0),
                    max: spinon_style::ComputedCssGridTrackBreadth::LengthPx(40.0),
                },
            ],
            "grid-template-columns",
        ),
    ] {
        let input = one_row_two_area_grid(
            columns,
            accepted_rows.clone(),
            ["other-left", "other-right"],
        );
        assert!(matches!(
            project_grid_layout(
                node,
                &input,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                property,
                ..
            }) if property == expected_property
        ));
    }
}
