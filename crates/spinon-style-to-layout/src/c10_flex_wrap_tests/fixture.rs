use std::collections::BTreeMap;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView, compute_runtime_flex_paint_cascade,
    first_unsupported_runtime_flex_paint_inline_property,
};

use crate::{StyleLayoutOutput, compute_runtime_style_layout};

pub(super) const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c10-flex-wrap-v1.json"
);

pub(super) struct FixtureNode<'a> {
    pub(super) id: &'a str,
    pub(super) parent: Option<&'a str>,
    pub(super) style: &'a str,
}

pub(super) fn compute_case(
    root_id: &str,
    root_style: &str,
    children: &[FixtureNode<'_>],
) -> (StyleLayoutOutput, BTreeMap<String, HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(10_100).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut nodes = BTreeMap::from([(root_id.to_owned(), root)]);
    for child in children {
        nodes.insert(child.id.to_owned(), document.reserve_node_handle().unwrap());
    }
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    let mut entries = Vec::with_capacity(children.len() + 1);
    entries.push((root_id, None, root_style));
    entries.extend(
        children
            .iter()
            .map(|node| (node.id, node.parent, node.style)),
    );
    for (id, parent, style) in entries {
        let node = nodes[id];
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
            parent: if id == root_id {
                HostParent::Root
            } else {
                HostParent::Node(nodes[parent.unwrap_or(root_id)])
            },
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
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    assert_eq!(
        first_unsupported_runtime_flex_paint_inline_property(&view),
        None
    );
    let computed =
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    (output, nodes)
}

pub(super) fn compare_reference_case(
    actual: &StyleLayoutOutput,
    nodes: &BTreeMap<String, HostNodeHandle>,
    case_id: &str,
) {
    let reference: Value = serde_json::from_slice(&std::fs::read(REFERENCE).unwrap()).unwrap();
    let expected_case = reference["observations"][0]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == case_id)
        .unwrap_or_else(|| panic!("Chromium reference case {case_id}가 없습니다"));
    for expected_node in expected_case["nodes"].as_array().unwrap() {
        let id = expected_node["id"].as_str().unwrap();
        let handle = nodes[id];
        let actual_style = actual
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == handle.id())
            .unwrap_or_else(|| panic!("{case_id}.{id} computed style 누락"));
        for (property, expected) in expected_node["properties"].as_object().unwrap() {
            assert_eq!(
                actual_style.properties.get(property).map(String::as_str),
                expected.as_str(),
                "{case_id}.{id}.{property}"
            );
        }

        let frame = actual
            .layout
            .frames
            .get(&handle.id())
            .unwrap_or_else(|| panic!("{case_id}.{id} layout frame 누락"));
        for (field, actual) in [
            ("x", frame.x),
            ("y", frame.y),
            ("width", frame.width),
            ("height", frame.height),
        ] {
            let expected = expected_node["rect"][field].as_f64().unwrap();
            assert!(
                (f64::from(actual) - expected).abs() <= 0.5,
                "{case_id}.{id}.{field}: 실제 {actual}, Chromium {expected}, 허용치 0.5 CSS px"
            );
        }
    }
}

pub(super) const FROZEN_ITEM: &str = "display:block;box-sizing:border-box;width:40px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0";
pub(super) const FROZEN_ITEM_50: &str = "display:block;box-sizing:border-box;width:50px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0";
