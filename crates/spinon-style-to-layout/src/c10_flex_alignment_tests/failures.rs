use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView, compute_runtime_flex_layout_cascade,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn document() -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_134).unwrap(), document.document_revision());
    for (node, id, style, parent) in [
        (
            root,
            "c1033-invalid-adapter-root",
            "display:flex;width:100px;height:100px;flex-direction:row;align-items:center;align-content:center;justify-content:center",
            HostParent::Root,
        ),
        (
            child,
            "c1033-invalid-adapter-item",
            "display:block;width:20px;height:20px;align-self:auto",
            HostParent::Node(root),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", id), ("style", style)] {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, child)
}

#[test]
fn unsupported_adapter_values_fail_with_node_and_property_context() {
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    for (property, value, target) in [
        ("align-items", "baseline", "root"),
        ("align-self", "baseline", "child"),
        ("align-content", "left", "root"),
        ("justify-content", "safe space-between", "root"),
    ] {
        let (document, root, child) = document();
        let snapshot = document.snapshot();
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
                .unwrap();
        let mut computed =
            compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
        let target_node = if target == "root" {
            root.id()
        } else {
            child.id()
        };
        let mut elements = computed.elements.to_vec();
        let element = elements
            .iter_mut()
            .find(|element| element.node_id == target_node)
            .unwrap();
        element
            .properties
            .insert(property.to_owned(), value.to_owned());
        computed.elements = elements.into();

        assert!(matches!(
            compute_runtime_style_layout(&snapshot, root, computed, viewport),
            Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property: found_property,
                value: found_value,
            }) if node == target_node && found_property == property && found_value == value
        ));
    }
}
