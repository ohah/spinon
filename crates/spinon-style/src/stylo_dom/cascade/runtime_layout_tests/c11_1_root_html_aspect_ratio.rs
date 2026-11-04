use super::super::{
    CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets,
    first_unsupported_runtime_root_html_block_paint_inline_property,
    first_unsupported_runtime_root_html_text_paint_inline_property,
};
use super::view_for_inline_style;
use crate::{ComputedStyleProfile, CssCascadeError};

#[test]
fn aspect_ratio_is_typed_only_for_the_root_html_text_profile() {
    let (view, node) =
        view_for_inline_style("display:block;width:60px;height:auto;aspect-ratio:2 / 1");

    assert_eq!(
        first_unsupported_runtime_root_html_text_paint_inline_property(&view),
        None
    );
    assert_eq!(
        first_unsupported_runtime_root_html_block_paint_inline_property(&view),
        Some((node, "aspect-ratio".to_owned()))
    );

    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("the parsed Root HTML text profile should retain numeric aspect-ratio");
    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
    );
    let element = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == node)
        .expect("the styled node should remain in the computed snapshot");
    assert_eq!(
        element.properties.get("aspect-ratio").map(String::as_str),
        Some("2 / 1")
    );
    assert_eq!(
        element.layout_aspect_ratio.map(|ratio| ratio.ratio),
        Some(2.0)
    );
}

#[test]
fn auto_numeric_aspect_ratio_stays_rejected_for_the_default_profile() {
    let (view, _) = view_for_inline_style("display:block;aspect-ratio:auto 2 / 1");
    let error = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect_err("auto <ratio> is outside this fixed numeric slice");

    assert!(matches!(
        error,
        CssCascadeError::UnsupportedComputedAspectRatio { .. }
    ));
}
