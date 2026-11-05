use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{
    GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize,
};
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridTemplateAreas, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedStyleProfile,
};

fn auto_minimum_fraction_track(factor: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::Fraction(factor),
    }
}

fn zero_minimum_fraction_track(factor: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(0.0),
        max: ComputedCssGridTrackBreadth::Fraction(factor),
    }
}

fn named_area_template() -> ComputedCssGridTemplateAreas {
    ComputedCssGridTemplateAreas {
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
    }
}

#[test]
fn root_html_named_area_profile_projects_only_the_bounded_auto_minimum_matrices() {
    let node = NodeId::new(11_280).unwrap();
    for (factor, expected_min) in [
        (1.0, GridTrackMinSizingFunction::Auto),
        (0.0, GridTrackMinSizingFunction::Auto),
    ] {
        let mut columns = grid(
            vec![
                auto_minimum_fraction_track(factor),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(20.0),
            ],
        );
        columns.template_areas = Some(named_area_template());
        let projected = project_grid_layout(
            node,
            &columns,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect("the exact columns auto-minimum named-area profile should project");
        assert_eq!(
            projected.columns[0],
            GridTrackSize::MinMax {
                min: expected_min,
                max: GridTrackMaxSizingFunction::Fraction(factor),
            }
        );
        assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(20.0); 2]);
        assert_eq!(projected.template_areas.as_ref().unwrap().areas.len(), 3);
    }

    let mut zero_minimum = grid(
        vec![
            zero_minimum_fraction_track(1.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
        ],
    );
    zero_minimum.template_areas = Some(named_area_template());
    let projected = project_grid_layout(
        node,
        &zero_minimum,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("the exact zero-minimum comparison control should project");
    assert_eq!(
        projected.columns[0],
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(0.0),
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        }
    );

    let mut rows = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![
            auto_minimum_fraction_track(1.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
    );
    rows.template_areas = Some(named_area_template());
    let projected = project_grid_layout(
        node,
        &rows,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("the exact rows auto-minimum named-area profile should project");
    assert_eq!(
        projected.rows[0],
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Auto,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        }
    );

    let mut zero_minimum_rows = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![
            zero_minimum_fraction_track(1.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
    );
    zero_minimum_rows.template_areas = Some(named_area_template());
    project_grid_layout(
        node,
        &zero_minimum_rows,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("the exact zero-minimum rows comparison control should project");
}

#[test]
fn root_html_named_area_profile_rejects_out_of_matrix_minmax_tracks() {
    let node = NodeId::new(11_281).unwrap();
    let invalid_grids = [
        grid(
            vec![
                auto_minimum_fraction_track(1.0),
                ComputedCssGridTrackSize::LengthPx(41.0),
            ],
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(20.0),
            ],
        ),
        grid(
            vec![
                auto_minimum_fraction_track(1.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(21.0),
            ],
        ),
        grid(
            vec![
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::LengthPx(1.0),
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(20.0),
            ],
        ),
    ];
    for mut invalid in invalid_grids {
        invalid.template_areas = Some(named_area_template());
        let error = project_grid_layout(
            node,
            &invalid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("non-matching named-area minmax matrices must remain unsupported");
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
fn root_html_profile_projects_auto_minimum_fraction_tracks_to_implicit_rows_mode() {
    let node = NodeId::new(11_252).unwrap();
    let input = grid(
        vec![
            auto_minimum_fraction_track(1.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
    );

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("auto-minimum flex maximum is part of the bounded Root HTML profile");

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::Auto,
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(40.0),
        ]
    );
    assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(40.0)]);
    assert_eq!(
        projected.placement_mode,
        GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(projected.auto_rows, spinon_layout::GridAutoRows::Auto);
}

#[test]
fn root_html_profile_preserves_zero_fraction_auto_minimum_shape() {
    let node = NodeId::new(11_253).unwrap();
    let input = grid(
        vec![
            auto_minimum_fraction_track(0.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
    );

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("zero is a valid non-negative flex factor");

    assert_eq!(
        projected.columns[0],
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Auto,
            max: GridTrackMaxSizingFunction::Fraction(0.0),
        }
    );
}

#[test]
fn root_html_profile_rejects_invalid_fraction_with_auto_minimum() {
    let node = NodeId::new(11_254).unwrap();
    for track in [
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(-1.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(f32::NAN),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(f32::INFINITY),
        },
    ] {
        let input = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(40.0)]);
        let error = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("fraction maximums with auto minimum must be finite and non-negative");
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
