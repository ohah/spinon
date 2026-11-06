use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{ComputedCssGridAutoRepeatMode, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize};

#[test]
fn root_html_auto_repeat_preserves_non_negative_percentage_minimums_on_both_axes() {
    for (axis, mode, minimum, expected_minimum) in [
        ("columns", "auto-fill", "10%", 0.1),
        ("columns", "auto-fit", "0%", 0.0),
        ("rows", "auto-fill", "120%", 1.2),
        ("rows", "auto-fit", "10%", 0.1),
    ] {
        let other_axis = if axis == "columns" {
            "grid-template-rows:40px"
        } else {
            "grid-template-columns:40px"
        };
        let style = format!(
            "display:grid;box-sizing:border-box;width:200px;height:160px;grid-template-{axis}:repeat({mode},minmax({minimum},1fr));{other_axis};margin:0;padding:0"
        );
        let (view, root) = view_for_inline_style(&style);
        let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode} {minimum}: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == root)
            .and_then(|element| element.grid_layout.as_ref())
            .unwrap_or_else(|| panic!("{axis}/{mode} {minimum} should have typed Grid data"));
        let (tracks, repeat) = if axis == "columns" {
            (&grid.columns, &grid.columns_auto_repeat)
        } else {
            (&grid.rows, &grid.rows_auto_repeat)
        };

        assert_eq!(
            tracks,
            &[ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Percentage(expected_minimum),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            }],
            "{axis}/{mode} {minimum} should preserve the percentage minimum"
        );
        assert_eq!(
            repeat.as_ref().map(|repeat| repeat.mode),
            Some(if mode == "auto-fill" {
                ComputedCssGridAutoRepeatMode::AutoFill
            } else {
                ComputedCssGridAutoRepeatMode::AutoFit
            }),
            "{axis}/{mode} {minimum} should preserve the auto-repeat mode"
        );
    }
}
