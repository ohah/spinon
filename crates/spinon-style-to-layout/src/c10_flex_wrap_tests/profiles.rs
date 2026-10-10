use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_layout_cascade,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

use super::fixture::HTML;

#[test]
fn all_runtime_flex_profiles_project_wrap_into_taffy() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let first = document.reserve_node_handle().unwrap();
    let second = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_101).unwrap(), document.document_revision());
    for (node, id, style, parent) in [
        (
            root,
            "profile-wrap-root",
            "display:flex;box-sizing:border-box;width:35px;height:30px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;align-items:flex-start;justify-content:flex-start;row-gap:0;column-gap:0;margin:0;padding:0;border:0",
            HostParent::Root,
        ),
        (
            first,
            "profile-wrap-first",
            "display:block;box-sizing:border-box;width:30px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
            HostParent::Node(root),
        ),
        (
            second,
            "profile-wrap-second",
            "display:block;box-sizing:border-box;width:30px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
            HostParent::Node(root),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let viewport = CssViewport::C04_FIXTURE;
    let profiles = [
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_custom_properties_cascade(&view, viewport, StyleRevision::INITIAL)
            .unwrap(),
        compute_runtime_flex_custom_properties_paint_cascade(
            &view,
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
    ];
    let expected_profiles = [
        ComputedStyleProfile::RuntimeFlexLayoutV1,
        ComputedStyleProfile::RuntimeFlexPaintV1,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
    ];

    for (computed, expected_profile) in profiles.into_iter().zip(expected_profiles) {
        assert_eq!(computed.profile, expected_profile);
        let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
        let first_frame = output.layout.frames[&first.id()];
        let second_frame = output.layout.frames[&second.id()];
        assert!(
            second_frame.y > first_frame.y,
            "{expected_profile:?} did not project flex-wrap into Taffy: first={first_frame:?}, second={second_frame:?}"
        );
    }
}

#[test]
fn flex_wrap_reverse_fails_closed_at_the_node_and_property_boundary() {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(10_101).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "id").unwrap(),
        value: "unsupported-wrap-root".to_owned().into(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:flex;width:100px;height:20px;flex-wrap:wrap-reverse"
            .to_owned()
            .into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let viewport = CssViewport::C04_FIXTURE;
    let computed =
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, viewport),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "flex-wrap",
            value,
        }) if node == root.id() && value == "wrap-reverse"
    ));
}
