use super::super::{CssViewport, compute_runtime_root_html_block_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::CssCascadeError;

#[test]
fn root_html_block_paint_rejects_partial_alpha_and_non_srgb_backgrounds() {
    for background in ["rgb(1 2 3 / 50%)", "color(display-p3 0.9 0.1 0.4)"] {
        let style = format!("display:block;width:1px;height:1px;background-color:{background}");
        let (view, _) = view_for_inline_style(&style);
        let error = compute_runtime_root_html_block_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .expect_err("the root HTML Block paint profile has a closed color boundary");
        assert!(matches!(
            error,
            CssCascadeError::UnsupportedComputedBackgroundColor { .. }
        ));
    }
}
