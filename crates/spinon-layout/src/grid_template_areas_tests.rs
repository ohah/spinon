use super::{
    GridAutoColumns, GridAutoFlow, GridAutoRepeat, GridAutoRepeatMode, GridAutoRows,
    GridContainerInput, GridPlacementMode, GridTemplateAreaInput, GridTemplateAreasInput,
    GridTrackSize, MAX_GRID_LINE_METADATA_BYTES, validate_template_areas,
};
use crate::{LayoutError, NodeId};

fn valid_container() -> GridContainerInput {
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(20.0)],
        rows: vec![GridTrackSize::LengthPx(20.0)],
        columns_auto_repeat: Some(GridAutoRepeat {
            mode: GridAutoRepeatMode::AutoFill,
            start: 0,
            end: 1,
            line_names: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: Vec::new(),
        template_areas: Some(GridTemplateAreasInput {
            rows: 1,
            columns: 1,
            areas: vec![GridTemplateAreaInput {
                name: "area".to_owned(),
                row_start: 0,
                row_end: 1,
                column_start: 0,
                column_end: 1,
            }],
        }),
        justify_items: Default::default(),
        auto_flow: GridAutoFlow::Row,
        auto_rows: GridAutoRows::Auto,
        auto_columns: GridAutoColumns::Auto,
        placement_mode: GridPlacementMode::TrackSizing,
    }
}

fn areas(container: &mut GridContainerInput) -> &mut GridTemplateAreasInput {
    container.template_areas.as_mut().unwrap()
}

fn assert_invalid(node: NodeId, label: &str, container: &GridContainerInput) {
    assert!(
        matches!(
            validate_template_areas(node, container),
            Err(LayoutError::InvalidGridTrackList {
                node: error_node,
                property: "grid-template-areas",
            }) if error_node == node
        ),
        "{label} must fail closed"
    );
}

#[test]
fn auto_repeat_template_areas_accept_empty_and_bounded_metadata() {
    let node = NodeId::new(11_119).unwrap();
    assert!(validate_template_areas(node, &valid_container()).is_ok());

    let mut empty = valid_container();
    areas(&mut empty).areas.clear();
    assert!(validate_template_areas(node, &empty).is_ok());
}

#[test]
fn template_grid_areas_accept_missing_and_extra_authored_tracks() {
    let node = NodeId::new(11_136).unwrap();
    let mut container = valid_container();
    container.columns_auto_repeat = None;
    container.placement_mode = GridPlacementMode::TemplateGrid;
    areas(&mut container).rows = 2;
    areas(&mut container).columns = 3;
    areas(&mut container).areas[0].row_start = 1;
    areas(&mut container).areas[0].row_end = 2;
    areas(&mut container).areas[0].column_start = 2;
    areas(&mut container).areas[0].column_end = 3;
    assert!(validate_template_areas(node, &container).is_ok());

    container.rows.clear();
    container.columns.clear();
    assert!(validate_template_areas(node, &container).is_ok());

    container.rows = vec![GridTrackSize::LengthPx(10.0), GridTrackSize::LengthPx(11.0)];
    container.columns = vec![
        GridTrackSize::LengthPx(12.0),
        GridTrackSize::LengthPx(13.0),
        GridTrackSize::LengthPx(14.0),
    ];
    areas(&mut container).rows = 1;
    areas(&mut container).columns = 1;
    areas(&mut container).areas[0] = GridTemplateAreaInput {
        name: "area".to_owned(),
        row_start: 0,
        row_end: 1,
        column_start: 0,
        column_end: 1,
    };
    assert!(validate_template_areas(node, &container).is_ok());
}

#[test]
fn template_grid_areas_remain_valid_after_implicit_placement_mode_selection() {
    let node = NodeId::new(11_137).unwrap();
    for mode in [
        GridPlacementMode::TemplateGridImplicitTracks,
        GridPlacementMode::TemplateGridSignedImplicitTracks,
    ] {
        let mut container = valid_container();
        container.columns_auto_repeat = None;
        container.placement_mode = mode;
        areas(&mut container).rows = 2;
        areas(&mut container).columns = 3;
        areas(&mut container).areas[0].row_start = 1;
        areas(&mut container).areas[0].row_end = 2;
        areas(&mut container).areas[0].column_start = 2;
        areas(&mut container).areas[0].column_end = 3;
        assert!(validate_template_areas(node, &container).is_ok());
    }
}

#[test]
fn auto_repeat_template_areas_accept_two_auto_repeat_axes_and_area_extension() {
    let node = NodeId::new(11_121).unwrap();
    let mut container = valid_container();
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFit,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    let areas = areas(&mut container);
    areas.rows = 3;
    areas.columns = 3;
    areas.areas[0].row_start = 2;
    areas.areas[0].row_end = 3;
    areas.areas[0].column_start = 2;
    areas.areas[0].column_end = 3;

    assert!(validate_template_areas(node, &container).is_ok());
}

#[test]
fn auto_repeat_template_areas_reject_invalid_shape_names_bounds_and_metadata() {
    let node = NodeId::new(11_120).unwrap();
    let mut invalid = Vec::new();

    let mut no_auto_repeat = valid_container();
    no_auto_repeat.columns_auto_repeat = None;
    invalid.push(("no auto-repeat axis", no_auto_repeat));

    let mut zero_rows = valid_container();
    areas(&mut zero_rows).rows = 0;
    invalid.push(("zero area rows", zero_rows));

    let mut too_many_columns = valid_container();
    areas(&mut too_many_columns).columns = (super::GRID_MAX_TRACKS + 1) as u16;
    invalid.push(("area dimensions over the grid limit", too_many_columns));

    let mut fixed_axis_mismatch = valid_container();
    areas(&mut fixed_axis_mismatch).rows = 2;
    invalid.push(("area dimension on fixed axis", fixed_axis_mismatch));

    let mut empty_name = valid_container();
    areas(&mut empty_name).areas[0].name.clear();
    invalid.push(("empty area name", empty_name));

    let mut nul_name = valid_container();
    areas(&mut nul_name).areas[0].name = "area\0name".to_owned();
    invalid.push(("NUL in area name", nul_name));

    let mut empty_row_range = valid_container();
    areas(&mut empty_row_range).areas[0].row_end = 0;
    invalid.push(("empty row range", empty_row_range));

    let mut row_out_of_bounds = valid_container();
    areas(&mut row_out_of_bounds).areas[0].row_end = 2;
    invalid.push(("row range outside template", row_out_of_bounds));

    let mut empty_column_range = valid_container();
    areas(&mut empty_column_range).areas[0].column_end = 0;
    invalid.push(("empty column range", empty_column_range));

    let mut column_out_of_bounds = valid_container();
    areas(&mut column_out_of_bounds).areas[0].column_end = 2;
    invalid.push(("column range outside template", column_out_of_bounds));

    let mut duplicate_name = valid_container();
    areas(&mut duplicate_name).columns = 2;
    areas(&mut duplicate_name)
        .areas
        .push(GridTemplateAreaInput {
            name: "area".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 1,
            column_end: 2,
        });
    invalid.push(("duplicate area name", duplicate_name));

    let mut over_budget_name = valid_container();
    areas(&mut over_budget_name).areas[0].name = "x".repeat(MAX_GRID_LINE_METADATA_BYTES + 1);
    invalid.push(("area metadata over one MiB", over_budget_name));

    for (label, container) in invalid {
        assert_invalid(node, label, &container);
    }
}
