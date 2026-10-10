use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssViewport, StyloDocumentView, compute_flex_alignment_cascade,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_layout_cascade,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

use super::fixture::HTML;

#[test]
fn all_runtime_flex_profiles_project_reverse_direction_and_wrap_into_taffy() {
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
            "display:flex;box-sizing:border-box;width:35px;height:30px;min-width:0;min-height:0;flex-flow:row-reverse wrap-reverse;align-items:flex-start;justify-content:flex-start;row-gap:0;column-gap:0;margin:0;padding:0;border:0",
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
        let root_style = output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == root.id())
            .unwrap();
        assert_eq!(
            root_style
                .properties
                .get("flex-direction")
                .map(String::as_str),
            Some("row-reverse"),
            "{expected_profile:?} computed flex-flow direction"
        );
        assert_eq!(
            root_style.properties.get("flex-wrap").map(String::as_str),
            Some("wrap-reverse"),
            "{expected_profile:?} computed flex-flow wrap"
        );
        let first_frame = output.layout.frames[&first.id()];
        let second_frame = output.layout.frames[&second.id()];
        assert!(
            first_frame.x == 5.0 && second_frame.x == 5.0 && first_frame.y > second_frame.y,
            "{expected_profile:?} did not project row-reverse and wrap-reverse into Taffy: first={first_frame:?}, second={second_frame:?}"
        );
    }
}

#[test]
fn reverse_direction_fails_closed_outside_runtime_flex_profiles() {
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
        value: "display:flex;width:100px;height:20px;flex-direction:row-reverse"
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
        compute_flex_alignment_cascade(&view, &[], viewport, StyleRevision::INITIAL).unwrap();
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, viewport),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "flex-direction",
            value,
        }) if node == root.id() && value == "row-reverse"
    ));
}

#[test]
fn reverse_direction_fails_closed_when_text_direction_is_rtl() {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(10_102).unwrap();
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
        value: "unsupported-rtl-reverse-root".to_owned().into(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:flex;width:100px;height:20px;flex-direction:row-reverse;direction:rtl"
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
    let computed =
        compute_runtime_flex_paint_cascade(&view, CssViewport::C04_FIXTURE, StyleRevision::INITIAL)
            .unwrap();
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, CssViewport::C04_FIXTURE),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "flex-direction",
            value,
        }) if node == root.id() && value == "row-reverse with direction:rtl"
    ));
}
