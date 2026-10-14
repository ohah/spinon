use super::{
    GridLineNameReservationSite, project_grid_layout, project_grid_layout_with_reservation,
    project_grid_layout_with_reservations,
};
use spinon_core::NodeId;
use spinon_layout::LayoutError;
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTemplateAreas, ComputedCssGridTrackSize, ComputedStyleProfile,
};

#[test]
fn projection_reservation_failure_keeps_node_and_axis_context() {
    let node = NodeId::new(11_101).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        columns_line_names: Vec::new(),
        rows_line_names: Vec::new(),
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
fn paired_area_edge_aliases_on_both_axes_select_template_grid_sizing_mode() {
    let node = NodeId::new(11_106).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(80.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
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
        let error = project_grid_layout_with_reservations(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
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
