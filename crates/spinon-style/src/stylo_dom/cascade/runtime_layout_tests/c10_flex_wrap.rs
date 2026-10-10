use super::super::{
    CssViewport, compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_paint_cascade, first_unsupported_runtime_flex_paint_inline_property,
    first_unsupported_runtime_layout_inline_property,
};
use super::view_for_inline_style;
use crate::{CssOrigin, StylesheetSource};

#[test]
fn c101_flex_wrap_is_limited_to_runtime_flex_layout_and_paint_profiles() {
    let (view, node) = view_for_inline_style(
        "display:flex;width:105px;height:20px;flex-flow:column wrap;flex-direction:row",
    );
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    assert_eq!(
        first_unsupported_runtime_flex_paint_inline_property(&view),
        None
    );

    let layout = super::super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        layout.profile,
        super::super::ComputedStyleProfile::RuntimeFlexLayoutV1
    );
    let layout_style = layout
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(layout_style.properties["flex-wrap"], "wrap");
    assert_eq!(layout_style.properties["flex-direction"], "row");

    let paint =
        compute_runtime_flex_paint_cascade(&view, CssViewport::C04_FIXTURE, Default::default())
            .unwrap();
    assert_eq!(
        paint.profile,
        super::super::ComputedStyleProfile::RuntimeFlexPaintV1
    );
    let paint_style = paint
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(paint_style.properties["flex-wrap"], "wrap");

    assert_eq!(
        super::super::first_unsupported_runtime_custom_properties_inline_property(&view),
        None
    );

    let custom_layout = super::super::compute_runtime_flex_custom_properties_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        custom_layout.profile,
        super::super::ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
    );
    assert_eq!(custom_layout.elements[0].properties["flex-wrap"], "wrap");

    let custom_paint = super::super::compute_runtime_flex_custom_properties_paint_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        custom_paint.profile,
        super::super::ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
    );
    assert_eq!(custom_paint.elements[0].properties["flex-wrap"], "wrap");

    let registered_layout =
        super::super::compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
    assert_eq!(
        registered_layout.profile,
        super::super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
    );
    assert_eq!(
        registered_layout.elements[0].properties["flex-wrap"],
        "wrap"
    );

    let registered_paint =
        super::super::compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
    assert_eq!(
        registered_paint.profile,
        super::super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    assert_eq!(registered_paint.elements[0].properties["flex-wrap"], "wrap");

    let block = super::super::compute_runtime_block_paint_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert!(!block.elements[0].properties.contains_key("flex-wrap"));
    assert_eq!(
        super::super::first_unsupported_runtime_block_paint_inline_property(&view),
        Some((node, "flex-wrap".to_owned()))
    );
}

#[test]
fn c101_author_stylesheet_wrap_reaches_custom_and_registered_paint_profiles() {
    let (view, node) = view_for_inline_style("display:flex;width:105px;height:20px");
    let stylesheet = StylesheetSource {
        id: "c101-flex-wrap-author".to_owned(),
        base_url: "https://spinon.invalid/c101.css".to_owned(),
        origin: CssOrigin::Author,
        css: "div { flex-flow: column wrap; flex-direction: row; }".to_owned(),
    };

    let custom = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view,
        std::slice::from_ref(&stylesheet),
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let custom_style = custom
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(custom_style.properties["flex-wrap"], "wrap");
    assert_eq!(custom_style.properties["flex-direction"], "row");

    let registered_stylesheet = StylesheetSource {
        id: "c101-flex-wrap-registered-author".to_owned(),
        css: "@property --c101-width { syntax: \"<length>\"; inherits: false; initial-value: 7px; } div { flex-flow: column wrap; flex-direction: row; }".to_owned(),
        ..stylesheet
    };
    let registered =
        super::super::compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[registered_stylesheet],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
    let registered_style = registered
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(registered_style.properties["flex-wrap"], "wrap");
    assert_eq!(registered_style.properties["flex-direction"], "row");
}
