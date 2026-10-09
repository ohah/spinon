use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedStyleProfile, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_incremental_cascade_with_stylesheets,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn create_element(batch: &mut DocumentChangeBatch, node: spinon_core::HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.to_owned().into(),
    });
}

fn node_style(snapshot: &super::ComputedStyleSnapshot, id: spinon_core::NodeId) -> &str {
    snapshot
        .elements
        .iter()
        .find(|element| element.node_id == id)
        .unwrap()
        .properties
        .get("width")
        .unwrap()
}

#[test]
fn inline_style_subtree_reuse_matches_full_cascade_and_keeps_sibling_output() {
    let mut document = HostDocument::new().unwrap();
    let handles = (0..5)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut initial =
        DocumentChangeBatch::new(OwnerId::new(5401).unwrap(), document.document_revision());
    create_element(
        &mut initial,
        handles[0],
        "display:flex;width:320px;height:180px;--tile-size:41px",
    );
    create_element(
        &mut initial,
        handles[1],
        "display:flex;width:140px;height:80px;--tile-size:32px",
    );
    create_element(
        &mut initial,
        handles[2],
        "width:var(--tile-size);height:10px",
    );
    create_element(
        &mut initial,
        handles[3],
        "display:flex;width:140px;height:80px;--tile-size:41px",
    );
    create_element(
        &mut initial,
        handles[4],
        "width:var(--tile-size);height:10px",
    );
    for (node, parent) in [
        (handles[0], HostParent::Root),
        (handles[1], HostParent::Node(handles[0])),
        (handles[2], HostParent::Node(handles[1])),
        (handles[3], HostParent::Node(handles[0])),
        (handles[4], HostParent::Node(handles[3])),
    ] {
        initial.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(initial).unwrap();
    let previous_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: super::CssMediaEnvironment::MOBILE,
    };
    let previous = compute_runtime_flex_custom_properties_cascade(
        &previous_view,
        viewport,
        Default::default(),
    )
    .unwrap();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(5401).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: handles[1],
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:flex;width:140px;height:80px;--tile-size:46px"
            .to_owned()
            .into(),
    });
    document.commit(change).unwrap();
    let current_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let full =
        compute_runtime_flex_custom_properties_cascade(&current_view, viewport, Default::default())
            .unwrap();
    let (partial, stats) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        Default::default(),
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
        &previous,
        &[handles[1].id()],
    )
    .unwrap()
    .expect("안전한 inline style 변경은 부분 cascade 경로를 허용해야 합니다");

    assert_eq!(partial.elements, full.elements);
    assert_eq!(partial.diagnostics, full.diagnostics);
    assert_eq!(node_style(&partial, handles[2].id()), "46px");
    assert_eq!(node_style(&partial, handles[4].id()), "41px");
    assert_eq!(stats.recomputed_style_elements, 2);
    assert_eq!(stats.reused_style_elements, 3);
    assert_eq!(stats.context_style_elements, 1);
}
