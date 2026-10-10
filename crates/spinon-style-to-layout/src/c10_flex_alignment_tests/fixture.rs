use std::{collections::BTreeMap, sync::Arc};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_layout_cascade,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};

use crate::{StyleLayoutOutput, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str =
    include_str!("../../../../tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json");

pub(super) const PROFILES: [ComputedStyleProfile; 6] = [
    ComputedStyleProfile::RuntimeFlexLayoutV1,
    ComputedStyleProfile::RuntimeFlexPaintV1,
    ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
    ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
    ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1,
    ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
];

pub(super) fn reference() -> Value {
    serde_json::from_str(REFERENCE).expect("C10.3.3 Chromium reference를 읽어야 합니다")
}

pub(super) fn observations(reference: &Value) -> &[Value] {
    reference["observations"]
        .as_array()
        .expect("Chromium observation 배열이 있어야 합니다")
}

pub(super) fn cases(observation: &Value) -> &[Value] {
    observation["cases"]
        .as_array()
        .expect("Chromium case 배열이 있어야 합니다")
}

pub(super) fn compute_case(
    case: &Value,
    profile: ComputedStyleProfile,
) -> (StyleLayoutOutput, BTreeMap<String, HostNodeHandle>) {
    compute_case_with_device_scale_factor(case, profile, 1.0)
}

pub(super) fn compute_case_with_device_scale_factor(
    case: &Value,
    profile: ComputedStyleProfile,
    device_scale_factor: f32,
) -> (StyleLayoutOutput, BTreeMap<String, HostNodeHandle>) {
    let fixture_nodes = case["nodes"]
        .as_array()
        .expect("fixture case에 노드 배열이 있어야 합니다");
    let mut document = HostDocument::new().unwrap();
    let mut nodes = BTreeMap::new();
    for fixture_node in fixture_nodes {
        let id = fixture_node["id"].as_str().unwrap();
        assert!(
            nodes
                .insert(id.to_owned(), document.reserve_node_handle().unwrap())
                .is_none()
        );
    }

    let roots = fixture_nodes
        .iter()
        .filter(|node| node["input"]["parentId"].is_null())
        .collect::<Vec<_>>();
    assert_eq!(
        roots.len(),
        1,
        "{}에는 단일 layout root가 있어야 합니다",
        case["id"]
    );
    let root_id = roots[0]["id"].as_str().unwrap();
    let root = nodes[root_id];

    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_133).unwrap(), document.document_revision());
    for fixture_node in fixture_nodes {
        let id = fixture_node["id"].as_str().unwrap();
        let input = &fixture_node["input"];
        let tag = input["tag"].as_str().unwrap();
        let style = input["style"].as_str().unwrap();
        assert!(
            !style.is_empty(),
            "{id}의 authored inline style이 비어 있습니다"
        );
        let node = nodes[id];
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: tag.to_owned(),
        });
        for (name, value) in [("id", id), ("style", style)] {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        let parent = if id == root_id {
            HostParent::Root
        } else {
            let parent_id = input["parentId"]
                .as_str()
                .expect("root가 아닌 노드는 data-c1033-node 부모가 있어야 합니다");
            HostParent::Node(nodes[parent_id])
        };
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed = match profile {
        ComputedStyleProfile::RuntimeFlexLayoutV1 => {
            compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap()
        }
        ComputedStyleProfile::RuntimeFlexPaintV1 => {
            compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap()
        }
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1 => {
            compute_runtime_flex_custom_properties_cascade(&view, viewport, StyleRevision::INITIAL)
                .unwrap()
        }
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1 => {
            compute_runtime_flex_custom_properties_paint_cascade(
                &view,
                viewport,
                StyleRevision::INITIAL,
            )
            .unwrap()
        }
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1 => {
            compute_runtime_flex_registered_properties_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )
            .unwrap()
        }
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
            compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )
            .unwrap()
        }
        other => panic!("지원하지 않는 C10.3.3 fixture profile: {other:?}"),
    };
    assert_eq!(computed.profile, profile);
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    (output, nodes)
}
