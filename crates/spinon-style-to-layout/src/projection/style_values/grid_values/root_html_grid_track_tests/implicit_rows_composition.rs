use super::*;

#[test]
fn root_html_profile_projects_percentage_tracks_with_fixed_implicit_rows() {
    let node = NodeId::new(11_188).unwrap();
    let mut grid = grid(
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Percentage(0.75),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    grid.auto_rows = ComputedCssGridAutoRows::LengthPx(15.0);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("direct percentage explicit tracks can use a fixed implicit row");

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::Percentage(0.25),
            GridTrackSize::Percentage(0.75)
        ]
    );
    assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(20.0)]);
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::LengthPx(15.0)
    );
}

#[test]
fn root_html_profile_projects_fraction_rows_with_fixed_implicit_rows() {
    let node = NodeId::new(11_190).unwrap();
    let mut grid = grid(
        vec![ComputedCssGridTrackSize::LengthPx(120.0)],
        vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ],
    );
    grid.auto_rows = ComputedCssGridAutoRows::LengthPx(15.0);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("direct fraction explicit tracks can use a fixed implicit row");

    assert_eq!(
        projected.rows,
        vec![GridTrackSize::Fraction(1.0), GridTrackSize::Fraction(2.0)]
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::LengthPx(15.0)
    );
}

#[test]
fn root_html_profile_still_rejects_intrinsic_tracks_with_fixed_implicit_rows() {
    let node = NodeId::new(11_191).unwrap();
    for (track, property) in [
        (ComputedCssGridTrackSize::Auto, "grid-auto-rows"),
        (
            ComputedCssGridTrackSize::MinMax {
                min: spinon_style::ComputedCssGridTrackBreadth::Auto,
                max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            "grid-template-columns",
        ),
    ] {
        let mut grid = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(20.0)]);
        grid.auto_rows = ComputedCssGridAutoRows::LengthPx(15.0);
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("intrinsic explicit tracks remain unsupported with fixed implicit rows");
        assert!(matches!(
            error,
            crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: error_property,
                ..
            } if error_node == node && error_property == property
        ));
    }
}
