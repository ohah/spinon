use spinon_core::NodeId;
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTemplateAreas,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
};

use super::validate_root_html_intrinsic_named_area_template;

fn grid_with_tracks(rows: usize, columns: usize) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::MinContent; columns],
        rows: vec![ComputedCssGridTrackSize::MaxContent; rows],
        columns_fixed_repeat_group_count: 0,
        rows_fixed_repeat_group_count: 0,
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: Vec::new(),
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn area(
    name: &str,
    row_start: usize,
    row_end: usize,
    column_start: usize,
    column_end: usize,
) -> ComputedCssGridArea {
    ComputedCssGridArea {
        name: name.to_owned(),
        row_start,
        row_end,
        column_start,
        column_end,
    }
}

fn valid_areas() -> ComputedCssGridTemplateAreas {
    ComputedCssGridTemplateAreas {
        rows: 2,
        columns: 3,
        areas: vec![
            area("wide", 0, 1, 0, 2),
            area("tail", 0, 1, 2, 3),
            area("foot", 1, 2, 0, 3),
        ],
    }
}

#[test]
fn intrinsic_named_area_template_accepts_non_overlapping_rectangles_and_spans() {
    let node = NodeId::new(11_441).unwrap();
    assert!(
        validate_root_html_intrinsic_named_area_template(
            node,
            &grid_with_tracks(2, 3),
            &valid_areas()
        )
        .is_ok()
    );
}

#[test]
fn intrinsic_named_area_template_rejects_overlapping_rectangles() {
    let node = NodeId::new(11_442).unwrap();
    let mut areas = valid_areas();
    areas.areas[1].column_start = 1;
    assert!(matches!(
        validate_root_html_intrinsic_named_area_template(node, &grid_with_tracks(2, 3), &areas),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            ..
        }) if error_node == node
    ));
}

#[test]
fn intrinsic_named_area_template_rejects_duplicate_names_and_dimension_mismatch() {
    let node = NodeId::new(11_443).unwrap();
    let mut duplicate = valid_areas();
    duplicate.areas[1].name = "wide".to_owned();
    assert!(
        validate_root_html_intrinsic_named_area_template(node, &grid_with_tracks(2, 3), &duplicate)
            .is_err()
    );

    assert!(
        validate_root_html_intrinsic_named_area_template(
            node,
            &grid_with_tracks(2, 2),
            &valid_areas()
        )
        .is_err()
    );
}

#[test]
fn intrinsic_named_area_template_rejects_oversized_metadata_before_layout() {
    let node = NodeId::new(11_444).unwrap();
    let grid = grid_with_tracks(1, 1);
    let mut areas = ComputedCssGridTemplateAreas {
        rows: 1,
        columns: 1,
        areas: vec![area("x", 0, 1, 0, 1)],
    };
    areas.areas[0].name =
        "x".repeat(super::super::grid_values::MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES);
    assert!(matches!(
        validate_root_html_intrinsic_named_area_template(node, &grid, &areas),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-areas",
            ..
        }) if error_node == node
    ));
}

#[test]
fn intrinsic_named_area_template_rejects_unsupported_auto_minimum_fixed_maximum() {
    let node = NodeId::new(11_445).unwrap();
    let mut grid = grid_with_tracks(2, 3);
    grid.columns[0] = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::LengthPx(40.0),
    };
    assert!(matches!(
        validate_root_html_intrinsic_named_area_template(node, &grid, &valid_areas()),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-columns",
            ..
        }) if error_node == node
    ));
}
