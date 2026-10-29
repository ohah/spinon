use super::*;
use spinon_style::{ComputedCssGridArea, ComputedCssGridTemplateAreas};

const COMBINED_METADATA_LIMIT: usize = 1024 * 1024;
const AREA_NAME: &str = "hero";

fn projection_grid(line_name_bytes: usize, include_area: bool) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
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
        columns_line_names: vec![vec!["x".repeat(line_name_bytes)], Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: include_area.then_some(ComputedCssGridTemplateAreas {
            rows: 1,
            columns: 2,
            areas: vec![ComputedCssGridArea {
                name: AREA_NAME.to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn template_projection_grid(line_name_bytes: usize) -> ComputedCssGridLayout {
    let mut grid = projection_grid(line_name_bytes, true);
    grid.columns_auto_repeat = None;
    grid
}

fn line_and_area_fixed_bytes(auto_repeat_axes: usize) -> usize {
    let outer_line_name_groups = 2 + 2;
    let auto_repeat_line_name_groups = auto_repeat_axes * 2;
    let projected_wrappers = 2 * std::mem::size_of::<Vec<Vec<String>>>()
        + auto_repeat_axes * std::mem::size_of::<Option<spinon_layout::GridAutoRepeat>>()
        + std::mem::size_of::<Option<spinon_layout::GridTemplateAreasInput>>();
    projected_wrappers
        + (outer_line_name_groups + auto_repeat_line_name_groups)
            * std::mem::size_of::<Vec<String>>()
        + std::mem::size_of::<String>()
        + std::mem::size_of::<spinon_layout::GridTemplateAreaInput>()
        + AREA_NAME.len()
}

#[test]
fn combined_line_name_and_area_metadata_accepts_exactly_one_mib() {
    let node = NodeId::new(11_130).unwrap();
    let line_name_bytes = COMBINED_METADATA_LIMIT - line_and_area_fixed_bytes(1);
    let grid = projection_grid(line_name_bytes, true);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.template_areas,
        Some(spinon_layout::GridTemplateAreasInput {
            rows: 1,
            columns: 2,
            areas: vec![spinon_layout::GridTemplateAreaInput {
                name: AREA_NAME.to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        })
    );
    assert_eq!(projected.columns_line_names[0][0].len(), line_name_bytes);
}

#[test]
fn combined_line_name_and_area_metadata_rejects_one_byte_over_one_mib() {
    let node = NodeId::new(11_131).unwrap();
    let line_name_bytes = COMBINED_METADATA_LIMIT - line_and_area_fixed_bytes(1) + 1;
    let grid = projection_grid(line_name_bytes, true);

    let error = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            value,
        } if error_node == node && value.contains("1 MiB")
    ));
}

#[test]
fn line_name_only_projection_is_not_subject_to_combined_budget() {
    let node = NodeId::new(11_132).unwrap();
    let grid = projection_grid(COMBINED_METADATA_LIMIT + 1, false);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns_line_names[0][0].len(),
        COMBINED_METADATA_LIMIT + 1
    );
    assert_eq!(projected.template_areas, None);
}

#[test]
fn combined_metadata_budget_counts_both_auto_repeat_axes() {
    let node = NodeId::new(11_134).unwrap();
    let auto_repeat = || ComputedCssGridAutoRepeat {
        mode: ComputedCssGridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: vec![Vec::new(), Vec::new()],
        fixed_repeat_minmax_track_indices: Vec::new(),
    };
    let line_name_bytes = COMBINED_METADATA_LIMIT - line_and_area_fixed_bytes(2);
    let mut grid = projection_grid(line_name_bytes, true);
    grid.rows_auto_repeat = Some(auto_repeat());

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert!(projected.columns_auto_repeat.is_some());
    assert!(projected.rows_auto_repeat.is_some());

    let line_name_bytes = COMBINED_METADATA_LIMIT - line_and_area_fixed_bytes(2) + 1;
    let mut over_budget = projection_grid(line_name_bytes, true);
    over_budget.rows_auto_repeat = Some(auto_repeat());
    assert!(matches!(
        project_grid_layout(
            node,
            &over_budget,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            value,
        }) if error_node == node && value.contains("1 MiB")
    ));
}

#[test]
fn combined_metadata_reservation_failures_keep_property_context_and_recover() {
    let node = NodeId::new(11_133).unwrap();
    let grid = projection_grid(4, true);
    for (site, property) in [
        (
            super::GridLineNameReservationSite::Groups,
            "grid-template-columns",
        ),
        (
            super::GridLineNameReservationSite::Names,
            "grid-template-columns",
        ),
        (
            super::GridLineNameReservationSite::NameText,
            "grid-template-columns",
        ),
        (
            super::GridLineNameReservationSite::Areas,
            "grid-template-areas",
        ),
        (
            super::GridLineNameReservationSite::AreaName,
            "grid-template-areas",
        ),
    ] {
        let mut math = super::CssMathProjector::default();
        let error = super::project_grid_layout_with_reservations(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            &mut math,
            |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
            |requested| if requested == site { Err(()) } else { Ok(()) },
        )
        .unwrap_err();

        assert!(
            matches!(
                error,
                crate::StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
                    node: error_node,
                    property: error_property,
                }) if error_node == node && error_property == property
            ),
            "reservation site {site:?} returned {error:?}"
        );
        assert!(
            project_grid_layout(
                node,
                &grid,
                ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            )
            .is_ok()
        );
    }
}

#[test]
fn template_grid_combined_line_name_and_area_metadata_obeys_one_mib_budget() {
    let node = NodeId::new(11_135).unwrap();
    let line_name_bytes =
        COMBINED_METADATA_LIMIT - line_and_area_fixed_bytes(0) - std::mem::size_of::<Vec<String>>();
    let grid = template_projection_grid(line_name_bytes);

    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap();
    assert!(projected.template_areas.is_some());
    assert_eq!(projected.columns_line_names[0][0].len(), line_name_bytes);

    let over_budget = template_projection_grid(line_name_bytes + 1);
    assert!(matches!(
        project_grid_layout(
            node,
            &over_budget,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            value,
        }) if error_node == node && value.contains("1 MiB")
    ));
}
