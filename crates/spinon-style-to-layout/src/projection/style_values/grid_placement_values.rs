use spinon_core::NodeId;
use spinon_layout::{GridContainerInput, GridItemInput};
use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridTemplateAreas};

use crate::StyleLayoutError;

#[path = "grid_placement_values/named_line.rs"]
mod named_line;

pub(super) fn project_grid_item_placement(
    node: NodeId,
    placement: &ComputedCssGridItemPlacement,
    container: &GridContainerInput,
    template_areas: Option<&ComputedCssGridTemplateAreas>,
    allow_implicit_named_lines: bool,
) -> Result<(Option<GridItemInput>, bool), StyleLayoutError> {
    let (row_start, row_end, row_used_implicit_named_line) =
        match container.rows_auto_repeat.as_ref() {
            Some(_) => named_line::project_axis_for_auto_repeat(
                node,
                "grid-row-start",
                "grid-row-end",
                &placement.row_start,
                &placement.row_end,
            )?,
            None => named_line::project_axis_with_areas_and_implicit_named_lines(
                node,
                "grid-row-start",
                "grid-row-end",
                &placement.row_start,
                &placement.row_end,
                named_line::LineMetadata {
                    names: &container.rows_line_names,
                    opposite_names: &container.columns_line_names,
                    areas: template_areas,
                    axis: named_line::GridAxis::Rows,
                    track_count: container.rows.len(),
                },
                allow_implicit_named_lines,
            )?,
        };
    let (column_start, column_end, column_used_implicit_named_line) =
        match container.columns_auto_repeat.as_ref() {
            Some(_) => named_line::project_axis_for_auto_repeat(
                node,
                "grid-column-start",
                "grid-column-end",
                &placement.column_start,
                &placement.column_end,
            )?,
            None => named_line::project_axis_with_areas_and_implicit_named_lines(
                node,
                "grid-column-start",
                "grid-column-end",
                &placement.column_start,
                &placement.column_end,
                named_line::LineMetadata {
                    names: &container.columns_line_names,
                    opposite_names: &container.rows_line_names,
                    areas: template_areas,
                    axis: named_line::GridAxis::Columns,
                    track_count: container.columns.len(),
                },
                allow_implicit_named_lines,
            )?,
        };
    let result = GridItemInput {
        row_start,
        row_end,
        column_start,
        column_end,
    };
    if matches!(&result.row_start, spinon_layout::GridPlacementInput::Auto)
        && matches!(&result.row_end, spinon_layout::GridPlacementInput::Auto)
        && matches!(
            &result.column_start,
            spinon_layout::GridPlacementInput::Auto
        )
        && matches!(&result.column_end, spinon_layout::GridPlacementInput::Auto)
    {
        Ok((
            None,
            row_used_implicit_named_line || column_used_implicit_named_line,
        ))
    } else {
        Ok((
            Some(result),
            row_used_implicit_named_line || column_used_implicit_named_line,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::project_grid_item_placement;
    use spinon_core::NodeId;
    use spinon_layout::{
        GridAutoRepeat, GridAutoRepeatMode, GridContainerInput, GridPlacementInput, GridTrackSize,
    };
    use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridPlacement};

    fn container() -> GridContainerInput {
        GridContainerInput {
            columns: vec![
                GridTrackSize::LengthPx(20.0),
                GridTrackSize::LengthPx(40.0),
                GridTrackSize::LengthPx(10.0),
            ],
            rows: vec![GridTrackSize::LengthPx(40.0)],
            columns_auto_repeat: Some(GridAutoRepeat {
                mode: GridAutoRepeatMode::AutoFill,
                start: 1,
                end: 2,
                line_names: vec![vec!["item".into()], vec!["item-end".into()]],
            }),
            columns_line_names: vec![
                vec!["container-start".into()],
                vec!["repeat-start".into()],
                vec!["repeat-end".into()],
                vec!["container-end".into()],
            ],
            ..GridContainerInput::default()
        }
    }

    #[test]
    fn auto_repeat_item_projection_preserves_named_placements_for_taffy() {
        let placement = ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Line(1),
            row_end: ComputedCssGridPlacement::Line(2),
            column_start: ComputedCssGridPlacement::NamedLine {
                name: "item".into(),
                occurrence: Some(-1),
            },
            column_end: ComputedCssGridPlacement::SpanNamed {
                name: "item-end".into(),
                count: 2,
            },
        };
        let (item, used_implicit) = project_grid_item_placement(
            NodeId::new(22).unwrap(),
            &placement,
            &container(),
            None,
            false,
        )
        .unwrap();
        let item = item.unwrap();

        assert_eq!(item.row_start, GridPlacementInput::Line(1));
        assert_eq!(item.row_end, GridPlacementInput::Line(2));
        assert_eq!(
            item.column_start,
            GridPlacementInput::NamedLine {
                name: "item".into(),
                occurrence: -1,
            }
        );
        assert_eq!(
            item.column_end,
            GridPlacementInput::SpanNamed {
                name: "item-end".into(),
                count: 2,
            }
        );
        assert!(!used_implicit);
    }

    #[test]
    fn auto_repeat_item_projection_preserves_an_unlisted_name_for_implicit_fallback() {
        let placement = ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Auto,
            row_end: ComputedCssGridPlacement::Auto,
            column_start: ComputedCssGridPlacement::NamedLine {
                name: "missing".into(),
                occurrence: None,
            },
            column_end: ComputedCssGridPlacement::Auto,
        };
        let (item, used_implicit) = project_grid_item_placement(
            NodeId::new(23).unwrap(),
            &placement,
            &container(),
            None,
            false,
        )
        .unwrap();
        let item = item.unwrap();
        assert_eq!(
            item.column_start,
            GridPlacementInput::NamedLine {
                name: "missing".into(),
                occurrence: 0,
            }
        );
        assert_eq!(item.column_end, GridPlacementInput::Auto);
        assert!(!used_implicit);
    }
}
