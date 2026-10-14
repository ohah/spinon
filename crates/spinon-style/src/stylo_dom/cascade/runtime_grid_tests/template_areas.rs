use super::*;
use crate::stylo_dom::cascade::incremental::{
    GridTemplateAreasReservationSite, inject_grid_template_areas_reservation_failure,
};

#[test]
fn template_profile_copies_sparse_area_rectangles_and_dimensions() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;grid-template-areas:'hero hero .' 'nav main main';margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();

    assert_eq!(
        grid.template_areas,
        Some(crate::ComputedCssGridTemplateAreas {
            rows: 2,
            columns: 3,
            areas: vec![
                crate::ComputedCssGridArea {
                    name: "hero".to_owned(),
                    row_start: 0,
                    row_end: 1,
                    column_start: 0,
                    column_end: 2,
                },
                crate::ComputedCssGridArea {
                    name: "nav".to_owned(),
                    row_start: 1,
                    row_end: 2,
                    column_start: 0,
                    column_end: 1,
                },
                crate::ComputedCssGridArea {
                    name: "main".to_owned(),
                    row_start: 1,
                    row_end: 2,
                    column_start: 1,
                    column_end: 3,
                },
            ],
        })
    );
}

#[test]
fn all_null_template_areas_keep_dimensions_without_dense_cells() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;grid-template-areas:'. . .' '. . .';margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(
        grid.template_areas,
        Some(crate::ComputedCssGridTemplateAreas {
            rows: 2,
            columns: 3,
            areas: Vec::new(),
        })
    );
}

#[test]
fn invalid_later_area_declaration_keeps_the_earlier_valid_template() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:80px 80px;grid-template-rows:40px;margin:0;padding:0",
    );
    let stylesheet = crate::StylesheetSource {
        id: "c1133-invalid-area-fallback".to_owned(),
        base_url: "https://spinon.invalid/c11/areas-fallback.css".to_owned(),
        origin: crate::CssOrigin::Author,
        css: "#root { grid-template-areas: 'kept .'; grid-template-areas: 'broken broken' 'broken .'; }"
            .to_owned(),
    };
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let root = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();
    let grid = root.grid_layout.as_ref().unwrap();

    assert_eq!(root.properties["grid-template-areas"], "\"kept .\"");
    assert_eq!(grid.template_areas.as_ref().unwrap().areas[0].name, "kept");
}

#[test]
fn area_allocation_failures_keep_node_and_property_context_and_recover() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:80px 80px;grid-template-rows:40px;grid-template-areas:'area area';margin:0;padding:0",
    );

    for site in [
        GridTemplateAreasReservationSite::Areas,
        GridTemplateAreasReservationSite::AreaNameText,
    ] {
        let error = {
            let _guard = inject_grid_template_areas_reservation_failure(site);
            compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap_err()
        };
        assert!(
            matches!(
                error,
                crate::CssCascadeError::GridAllocationFailure {
                    node,
                    property: "grid-template-areas"
                } if node == fixture.root
            ),
            "예약 실패 지점: {site:?}, 실제 오류: {error:?}"
        );
        assert!(
            compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .is_ok()
        );
    }
}
