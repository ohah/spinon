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
fn root_html_fit_content_rejects_non_px_or_invalid_limits() {
    let node = NodeId::new(11_282).unwrap();
    for track in [
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(0.5)),
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Calc(
            spinon_style::ComputedCssMath::LengthPx(20.0),
        )),
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
