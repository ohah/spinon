use super::super::{
    CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets,
    first_unsupported_runtime_root_html_text_paint_inline_property,
};
use super::view_for_inline_style;

#[test]
fn root_html_text_content_alignment_is_cascaded_and_allowed_inline() {
    let (view, node) =
        view_for_inline_style("display:block;align-content:safe end;justify-content:space-evenly");
    assert_eq!(
        first_unsupported_runtime_root_html_text_paint_inline_property(&view),
        None
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let element = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == node)
        .unwrap();
    assert_eq!(element.properties["align-content"], "safe end");
    assert_eq!(element.properties["justify-content"], "space-evenly");
}
