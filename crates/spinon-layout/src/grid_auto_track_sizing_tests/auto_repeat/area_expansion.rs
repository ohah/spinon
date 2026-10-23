use super::super::{final_axis_track_sizes, grid_input, id};
use super::collapsed_track_count;
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridAutoRows, GridItemInput, GridPlacementInput,
    GridTemplateAreaInput, GridTemplateAreasInput, GridTrackSize, LayoutDimension, LayoutEngine,
    LayoutGap, LayoutGridTrackSizingAxis, LayoutLengthPercentage, TaffyLayoutEngine,
};

struct Case {
    label: &'static str,
    area_rows: u16,
    area_columns: u16,
    hero: Option<[u16; 4]>,
    columns_mode: GridAutoRepeatMode,
    rows_mode: GridAutoRepeatMode,
    expected_columns: &'static [f32],
    expected_rows: &'static [f32],
    collapsed_columns: usize,
    collapsed_rows: usize,
    expected_frame: Option<[f32; 4]>,
}

fn area_line(name: &str) -> GridPlacementInput {
    GridPlacementInput::NamedLine {
        name: name.to_owned(),
        occurrence: 1,
    }
}

fn input_for(case: &Case) -> crate::LayoutInput {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(20.0)],
        vec![GridTrackSize::LengthPx(20.0)],
        usize::from(case.hero.is_some()),
        LayoutGap {
            column: LayoutLengthPercentage::length(4.0),
            row: LayoutLengthPercentage::length(4.0),
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(52.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(52.0);
    input.viewport.width = 52.0;
    input.viewport.height = 52.0;
    if case.hero.is_some() {
        input.nodes[1].style.width = LayoutDimension::Auto;
        input.nodes[1].style.height = LayoutDimension::Auto;
        input.grid_items.insert(
            id(2),
            GridItemInput {
                row_start: area_line("hero-start"),
                row_end: area_line("hero-end"),
                column_start: area_line("hero-start"),
                column_end: area_line("hero-end"),
            },
        );
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = crate::GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(GridAutoRepeat {
        mode: case.columns_mode,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: case.rows_mode,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.auto_columns = crate::GridAutoColumns::LengthPx(16.0);
    container.auto_rows = GridAutoRows::LengthPx(12.0);
    container.template_areas = Some(GridTemplateAreasInput {
        rows: case.area_rows,
        columns: case.area_columns,
        areas: case
            .hero
            .map(|[row_start, row_end, column_start, column_end]| {
                vec![GridTemplateAreaInput {
                    name: "hero".to_owned(),
                    row_start,
                    row_end,
                    column_start,
                    column_end,
                }]
            })
            .unwrap_or_default(),
    });
    input
}

#[test]
fn area_expansion_tracks_keep_auto_fit_collapse_and_item_geometry() {
    let cases = [
        Case {
            label: "empty 3x3 area",
            area_rows: 3,
            area_columns: 3,
            hero: None,
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[16.0],
            expected_rows: &[12.0],
            collapsed_columns: 2,
            collapsed_rows: 2,
            expected_frame: None,
        },
        Case {
            label: "item only in the added 3x3 area cell",
            area_rows: 3,
            area_columns: 3,
            hero: Some([2, 3, 2, 3]),
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[16.0],
            expected_rows: &[12.0],
            collapsed_columns: 2,
            collapsed_rows: 2,
            expected_frame: Some([0.0, 0.0, 16.0, 12.0]),
        },
        Case {
            label: "item spans repeated and area-added tracks",
            area_rows: 3,
            area_columns: 3,
            hero: Some([0, 3, 0, 3]),
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[20.0, 20.0, 16.0],
            expected_rows: &[20.0, 20.0, 12.0],
            collapsed_columns: 0,
            collapsed_rows: 0,
            expected_frame: Some([0.0, 0.0, 64.0, 60.0]),
        },
        Case {
            label: "column auto-fit with row auto-fill",
            area_rows: 3,
            area_columns: 3,
            hero: Some([2, 3, 2, 3]),
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFill,
            expected_columns: &[16.0],
            expected_rows: &[20.0, 20.0, 12.0],
            collapsed_columns: 2,
            collapsed_rows: 0,
            expected_frame: Some([0.0, 48.0, 16.0, 12.0]),
        },
        Case {
            label: "column auto-fill with row auto-fit",
            area_rows: 3,
            area_columns: 3,
            hero: Some([2, 3, 2, 3]),
            columns_mode: GridAutoRepeatMode::AutoFill,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[20.0, 20.0, 16.0],
            expected_rows: &[12.0],
            collapsed_columns: 0,
            collapsed_rows: 2,
            expected_frame: Some([48.0, 0.0, 16.0, 12.0]),
        },
        Case {
            label: "area adds only a column",
            area_rows: 1,
            area_columns: 3,
            hero: Some([0, 1, 2, 3]),
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[16.0],
            expected_rows: &[20.0],
            collapsed_columns: 2,
            collapsed_rows: 1,
            expected_frame: Some([0.0, 0.0, 16.0, 20.0]),
        },
        Case {
            label: "area adds only a row",
            area_rows: 3,
            area_columns: 1,
            hero: Some([2, 3, 0, 1]),
            columns_mode: GridAutoRepeatMode::AutoFit,
            rows_mode: GridAutoRepeatMode::AutoFit,
            expected_columns: &[20.0],
            expected_rows: &[12.0],
            collapsed_columns: 1,
            collapsed_rows: 2,
            expected_frame: Some([0.0, 0.0, 20.0, 12.0]),
        },
    ];

    for case in &cases {
        let output = TaffyLayoutEngine.compute(&input_for(case)).unwrap();
        assert_eq!(
            final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
            case.expected_columns,
            "{} columns",
            case.label
        );
        assert_eq!(
            final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
            case.expected_rows,
            "{} rows",
            case.label
        );
        assert_eq!(
            collapsed_track_count(&output, LayoutGridTrackSizingAxis::Columns),
            case.collapsed_columns,
            "{} collapsed columns",
            case.label
        );
        assert_eq!(
            collapsed_track_count(&output, LayoutGridTrackSizingAxis::Rows),
            case.collapsed_rows,
            "{} collapsed rows",
            case.label
        );
        match case.expected_frame {
            Some(expected) => {
                let frame = output.frames[&id(2)];
                assert_eq!(
                    [frame.x, frame.y, frame.width, frame.height],
                    expected,
                    "{} item frame",
                    case.label
                );
            }
            None => assert_eq!(output.frames.len(), 1, "{} has no item", case.label),
        }
    }
}
