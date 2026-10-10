use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedCssMath, CssViewport, compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use crate::{CssOrigin, StylesheetSource, StyloDocumentView};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn copies_supported_typed_calculations_into_owned_layout_values() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6501).unwrap(), document.document_revision());
    for (node, parent, id, style) in [
        (
            root,
            HostParent::Root,
            "mount",
            "display:flex;width:300px;height:200px;--box:calc(5px + 5%)",
        ),
        (child, HostParent::Node(root), "target", ""),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.into(),
        });
        if !style.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    let stylesheet = StylesheetSource {
        id: "c06-5-typed-math-test".to_owned(),
        base_url: "https://spinon.invalid/c06-5.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#target { width: calc(10px + 25%); height: min(50px, 10%); flex-basis: max(40px, 5%); margin-left: clamp(-10px, calc(0px - 2%), 0px); margin-right: var(--box); padding-left: clamp(1px, 2%, 8px); column-gap: max(1px, 2%); }".to_owned(),
    };
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let target = computed
        .elements
        .iter()
        .find(|element| element.node_id == child.id())
        .unwrap();

    assert!(matches!(
        target.layout_math_values.get("width"),
        Some(ComputedCssMath::Sum(_))
    ));
    assert!(matches!(
        target.layout_math_values.get("height"),
        Some(ComputedCssMath::Min(_))
    ));
    assert!(matches!(
        target.layout_math_values.get("flex-basis"),
        Some(ComputedCssMath::Max(_))
    ));
    assert!(matches!(
        target.layout_math_values.get("margin-left"),
        Some(ComputedCssMath::Clamp { .. })
    ));
    assert!(matches!(
        target.layout_math_values.get("margin-right"),
        Some(ComputedCssMath::Sum(_))
    ));
    assert!(matches!(
        target.layout_math_values.get("padding-left"),
        Some(ComputedCssMath::Clamp { .. })
    ));
    assert!(matches!(
        target.layout_math_values.get("column-gap"),
        Some(ComputedCssMath::Max(_))
    ));

    assert_eq!(target.properties["width"], "calc(25% + 10px)");
}
