use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    CssCascadeError,
};

fn fit_content_px(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(value))
}

fn fit_content_percentage(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(value))
}

#[test]
fn root_html_fit_content_tracks_are_typed_on_both_explicit_axes() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:fit-content(0px) 10px;grid-template-rows:fit-content(20px) 20px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("finite non-negative px fit-content tracks should survive Root HTML cascade");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("explicit fit-content tracks should produce a typed Grid");

    assert_eq!(
        grid.columns,
        [
            fit_content_px(0.0),
            ComputedCssGridTrackSize::LengthPx(10.0)
        ]
    );
    assert_eq!(
        grid.rows,
        [
            fit_content_px(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0)
        ]
    );
}

#[test]
fn root_html_fit_content_fixed_integer_repeat_expands_typed_tracks() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:repeat(2,fit-content(40px));grid-template-rows:20px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("a fixed integer repeat should reuse Root HTML fit-content track admission");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("expanded repeat should produce a typed Grid");

    assert_eq!(grid.columns, [fit_content_px(40.0), fit_content_px(40.0)]);
    assert_eq!(grid.columns_fixed_repeat_group_count, 1);
    assert_eq!(
        grid.columns_line_names,
        [Vec::<String>::new(), Vec::new(), Vec::new()]
    );
}

#[test]
fn root_html_fit_content_percentages_are_preserved_on_both_axes_and_fixed_repeat() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:fit-content(20%);grid-template-rows:fit-content(25%);margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("finite non-negative percentage limits should remain typed through Root HTML cascade");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("percentage tracks should produce a typed Grid");
    assert_eq!(grid.columns, [fit_content_percentage(0.2)]);
    assert_eq!(grid.rows, [fit_content_percentage(0.25)]);

    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:repeat(2,fit-content(20%));grid-template-rows:20px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("fixed integer repeat should preserve percentage fit-content values");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("expanded percentage repeat should produce a typed Grid");
    assert_eq!(
        grid.columns,
        [fit_content_percentage(0.2), fit_content_percentage(0.2)]
    );
    assert_eq!(grid.columns_fixed_repeat_group_count, 1);
}

#[test]
fn root_html_fit_content_rejects_calc_limits_and_keeps_auto_tracks_out() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:fit-content(calc(10px + 5%));grid-template-rows:20px;margin:0;padding:0",
    );
    let error = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect_err("percentage and calc fit-content limits stay outside Root HTML");
    assert!(
        matches!(error, CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-columns",
            ..
        } if node == root),
        "calc limit: {error:?}"
    );

    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:20px;grid-template-rows:20px;grid-auto-rows:fit-content(20px);margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("implicit fit-content stays represented as unsupported computed data");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("explicit tracks should remain available for the auto-track rejection check");
    assert!(matches!(
        grid.auto_rows,
        ComputedCssGridAutoRows::Unsupported(_)
    ));
}
