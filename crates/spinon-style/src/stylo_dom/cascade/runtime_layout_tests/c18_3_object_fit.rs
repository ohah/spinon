use super::super::{
    CssViewport, compute_runtime_block_image_cascade_with_stylesheets,
    first_unsupported_runtime_block_image_inline_property,
};
use super::view_for_inline_style;
use crate::{CssOrigin, StylesheetSource};

#[test]
fn runtime_block_image_profile_preserves_computed_image_object_values() {
    let (view, node) = view_for_inline_style(
        "display:block;width:24px;height:24px;object-fit:cover;object-position:-25% 150%;image-rendering:auto",
    );
    assert_eq!(
        first_unsupported_runtime_block_image_inline_property(&view),
        None
    );
    let snapshot = compute_runtime_block_image_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        snapshot.profile,
        super::super::ComputedStyleProfile::RuntimeBlockImageV1
    );
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.properties["object-fit"], "cover");
    assert_eq!(style.properties["object-position"], "-25% 150%");
    assert_eq!(style.properties["image-rendering"], "auto");
}

#[test]
fn runtime_block_image_profile_captures_css_initial_values() {
    let (view, node) = view_for_inline_style("display:block;width:24px;height:12px");
    let snapshot = compute_runtime_block_image_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.properties["object-fit"], "fill");
    assert_eq!(style.properties["object-position"], "50% 50%");
    assert_eq!(style.properties["image-rendering"], "auto");
}

#[test]
fn runtime_block_image_profile_rejects_non_percentage_object_position_values() {
    for value in [
        "left",
        "center center",
        "25% bottom",
        "25%",
        "calc(10% + 5%) 50%",
    ] {
        let (view, node) = view_for_inline_style(&format!(
            "display:block;width:24px;height:24px;object-position:{value}"
        ));
        assert_eq!(
            first_unsupported_runtime_block_image_inline_property(&view),
            Some((node, "object-position".to_owned())),
            "object-position value must fail closed: {value}"
        );
    }
}

#[test]
fn runtime_block_image_profile_rejects_stylesheet_object_position_keywords() {
    let (view, _) = view_for_inline_style("display:block;width:24px;height:24px");
    let stylesheet = StylesheetSource {
        id: "c18-3-unsupported-object-position".to_owned(),
        base_url: "https://spinon.invalid/c18-3.css".to_owned(),
        origin: CssOrigin::Author,
        css: "div { object-position: left; }".to_owned(),
    };
    let result = compute_runtime_block_image_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        Default::default(),
    );
    assert!(matches!(
        result,
        Err(crate::CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        }) if stylesheet_id == "c18-3-unsupported-object-position"
            && feature.contains("object-position")
    ));
}
