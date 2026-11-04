use super::super::{
    CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets,
    first_unsupported_runtime_root_html_block_paint_inline_property,
    first_unsupported_runtime_root_html_text_paint_inline_property,
};
use super::view_for_inline_style;
use crate::{ComputedAspectRatioSizingBox, ComputedStyleProfile, CssCascadeError};

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
fn positive_numeric_aspect_ratio_values_keep_their_typed_ratio_in_the_default_profile() {
    let cases = [
        ("1 / 2", "1 / 2", 0.5),
        ("4 / 3", "4 / 3", 4.0 / 3.0),
        ("1.5", "1.5 / 1", 1.5),
        ("3 / 2", "3 / 2", 1.5),
        ("5 / 2", "5 / 2", 2.5),
        ("10 / 1", "10 / 1", 10.0),
    ];

    for (css_ratio, serialized, expected_ratio) in cases {
        let inline_style = format!("display:block;width:60px;height:auto;aspect-ratio:{css_ratio}");
        let (view, node) = view_for_inline_style(&inline_style);
        assert_eq!(
            first_unsupported_runtime_root_html_text_paint_inline_property(&view),
            None,
            "default text profile should admit {css_ratio}"
        );
        assert_eq!(
            first_unsupported_runtime_root_html_block_paint_inline_property(&view),
            Some((node, "aspect-ratio".to_owned())),
            "ratio admission must stay scoped to the default text profile"
        );
        let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("numeric ratio {css_ratio} should cascade: {error}"));
        let element = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == node)
            .expect("the styled node should remain in the computed snapshot");
        assert_eq!(
            element.properties.get("aspect-ratio").map(String::as_str),
            Some(serialized),
            "computed serialization for {css_ratio}"
        );
        let ratio = element
            .layout_aspect_ratio
            .expect("numeric preferred ratios should have typed layout values");
        assert_eq!(ratio.sizing_box, ComputedAspectRatioSizingBox::BoxSizing);
        assert!(
            (ratio.ratio - expected_ratio).abs() <= f32::EPSILON * expected_ratio.max(1.0),
            "typed ratio for {css_ratio}: expected {expected_ratio}, got {}",
            ratio.ratio
        );
    }
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
