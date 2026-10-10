use std::sync::Arc;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, OwnerId};

use super::{
    CssCascadeError, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_paint_cascade, first_unsupported_runtime_flex_paint_inline_property,
    first_unsupported_runtime_layout_inline_property,
};
use crate::{CssOrigin, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn view_for_inline_style(style: &str) -> (StyloDocumentView, spinon_core::NodeId) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(31).unwrap(), document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: spinon_core::HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    (view, root.id())
}

#[test]
fn runtime_inline_allowlist_accepts_supported_shorthand_expansions() {
    let (view, _) = view_for_inline_style(
        "display:flex;flex:0 1 auto;gap:2px;margin-inline:3px;padding-block:4px;border:2px solid red",
    );
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
}

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

    let layout = super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        layout.profile,
        super::ComputedStyleProfile::RuntimeFlexLayoutV1
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
        super::ComputedStyleProfile::RuntimeFlexPaintV1
    );
    let paint_style = paint
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(paint_style.properties["flex-wrap"], "wrap");

    assert_eq!(
        super::first_unsupported_runtime_custom_properties_inline_property(&view),
        None
    );

    let custom_layout = super::compute_runtime_flex_custom_properties_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        custom_layout.profile,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
    );
    assert_eq!(custom_layout.elements[0].properties["flex-wrap"], "wrap");

    let custom_paint = super::compute_runtime_flex_custom_properties_paint_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        custom_paint.profile,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
    );
    assert_eq!(custom_paint.elements[0].properties["flex-wrap"], "wrap");

    let registered_layout =
        super::compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
    assert_eq!(
        registered_layout.profile,
        super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
    );
    assert_eq!(
        registered_layout.elements[0].properties["flex-wrap"],
        "wrap"
    );

    let registered_paint =
        super::compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
    assert_eq!(
        registered_paint.profile,
        super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    assert_eq!(registered_paint.elements[0].properties["flex-wrap"], "wrap");

    let block = super::compute_runtime_block_paint_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert!(!block.elements[0].properties.contains_key("flex-wrap"));
    assert_eq!(
        super::first_unsupported_runtime_block_paint_inline_property(&view),
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
        super::compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
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

#[test]
fn runtime_inline_allowlist_rejects_noninitial_border_image_values() {
    let (view, node) = view_for_inline_style("border:2px solid red;border-image-repeat:round");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((node, "border-image-repeat".to_owned()))
    );
}

#[test]
fn runtime_inline_allowlist_accepts_physical_min_max_size_longhands() {
    let (view, node) = view_for_inline_style(
        "display:flex;min-width:11px;max-width:90%;min-height:calc(10px + 2px);max-height:none",
    );
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let snapshot = super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.properties["min-width"], "11px");
    assert_eq!(style.properties["max-width"], "90%");
    assert_eq!(style.properties["min-height"], "12px");
    assert!(style.layout_math_values.contains_key("min-height"));
    assert_eq!(style.properties["max-height"], "none");
}

#[test]
fn runtime_inline_allowlist_reports_valid_unsupported_properties_without_values() {
    let (view, root) = view_for_inline_style("display:block;color:rgb(1,2,3)");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((root, "color".to_owned()))
    );
}

#[test]
fn runtime_inline_allowlist_rejects_unknown_custom_properties() {
    let (view, root) = view_for_inline_style("display:block;--theme-color:red");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((root, "--theme-color".to_owned()))
    );
}

#[test]
fn runtime_inline_allowlist_ignores_invalid_declarations_after_css_recovery() {
    let (view, _) = view_for_inline_style("display:block;color:nope");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let snapshot = super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(snapshot.diagnostics.len(), 1);
}

#[test]
fn runtime_paint_profile_accepts_only_background_color_beyond_layout_properties() {
    let (view, _) = view_for_inline_style(
        "display:flex;width:51px;height:31px;gap:11px;background-color:#3366ff",
    );
    assert_eq!(
        first_unsupported_runtime_flex_paint_inline_property(&view),
        None
    );
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((view.root_handle().id(), "background-color".to_owned()))
    );
}

#[test]
fn runtime_paint_profile_preserves_opaque_and_transparent_computed_paint() {
    let (opaque_view, opaque_node) =
        view_for_inline_style("display:block;width:51px;height:31px;background-color:#3366ff");
    let opaque = compute_runtime_flex_paint_cascade(
        &opaque_view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let opaque_style = opaque
        .elements
        .iter()
        .find(|style| style.node_id == opaque_node)
        .unwrap();
    assert_eq!(
        opaque_style.properties["background-color"],
        "rgb(51, 102, 255)"
    );
    assert_eq!(
        opaque_style.background_paint,
        Some(crate::ComputedBackgroundPaint::Opaque(
            crate::OpaqueCssSrgb {
                red: 51,
                green: 102,
                blue: 255,
            }
        ))
    );

    let (transparent_view, transparent_node) =
        view_for_inline_style("display:block;width:41px;height:31px;background-color:transparent");
    let transparent = compute_runtime_flex_paint_cascade(
        &transparent_view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let transparent_style = transparent
        .elements
        .iter()
        .find(|style| style.node_id == transparent_node)
        .unwrap();
    assert_eq!(
        transparent_style.properties["background-color"],
        "rgba(0, 0, 0, 0)"
    );
    assert_eq!(
        transparent_style.background_paint,
        Some(crate::ComputedBackgroundPaint::Transparent)
    );
}

#[test]
fn runtime_paint_profile_rejects_partial_alpha_and_keeps_layout_profile_strict() {
    let (view, node) = view_for_inline_style(
        "display:block;width:51px;height:31px;background-color:rgba(1, 2, 3, 0.5)",
    );
    assert!(matches!(
        compute_runtime_flex_paint_cascade(&view, CssViewport::C04_FIXTURE, Default::default()),
        Err(CssCascadeError::UnsupportedComputedBackgroundColor { node: actual, .. })
            if actual == node
    ));
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((node, "background-color".to_owned()))
    );
}
