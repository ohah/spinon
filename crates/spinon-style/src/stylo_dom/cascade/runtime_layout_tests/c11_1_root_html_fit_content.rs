use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    CssCascadeError,
};

fn fit_content_px(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(value))
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
fn root_html_fit_content_rejects_percentage_and_calc_limits_and_keeps_auto_tracks_out() {
    for limit in ["20%", "calc(10px + 5%)"] {
        let style = format!(
            "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:fit-content({limit});grid-template-rows:20px;margin:0;padding:0"
        );
        let (view, root) = view_for_inline_style(&style);
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
            "limit {limit}: {error:?}"
        );
    }

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
