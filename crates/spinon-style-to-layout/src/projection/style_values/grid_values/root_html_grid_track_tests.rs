use super::{project_grid_layout, project_grid_layout_with_math};
use crate::projection::typed_math::CssMathProjector;
use spinon_core::NodeId;
use spinon_layout::GridTrackSize;
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTemplateAreas, ComputedCssGridTrackSize, ComputedStyleProfile,
};

#[path = "root_html_grid_track_tests/implicit_rows_composition.rs"]
mod implicit_rows_composition;

fn grid(
    columns: Vec<ComputedCssGridTrackSize>,
    rows: Vec<ComputedCssGridTrackSize>,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns_line_names: vec![Vec::new(); columns.len() + 1],
        rows_line_names: vec![Vec::new(); rows.len() + 1],
        columns,
        rows,
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
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
fn root_html_profile_rejects_invalid_or_nondirect_fraction_tracks() {
    let node = NodeId::new(11_182).unwrap();
    for track in [
        ComputedCssGridTrackSize::Fraction(-0.25),
        ComputedCssGridTrackSize::Fraction(f32::NAN),
        ComputedCssGridTrackSize::Fraction(f32::INFINITY),
        ComputedCssGridTrackSize::Fraction(f32::NEG_INFINITY),
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::LengthPx(0.0),
            max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
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
            max: spinon_style::ComputedCssGridTrackBreadth::Percentage(0.5),
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
fn root_html_named_area_profile_still_rejects_percentage_tracks() {
    let node = NodeId::new(11_186).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Percentage(0.75),
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
    .expect_err("percentage tracks must not widen the fixed named-area subset");
    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            ..
        } if error_node == node
    ));
}
