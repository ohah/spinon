use std::collections::BTreeMap;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView};
use style::context::QuirksMode;

use crate::{
    StyleLayoutError, compute_runtime_block_flex_grid_auto_placement_style_layout,
    compute_runtime_block_flex_grid_dense_auto_placement_style_layout,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const CHROME_REFERENCE: &str = include_str!(
    "../../../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json"
);
const ORDER_RUNTIME_MATRIX: &str =
    include_str!("../../../tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json");

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl Fixture {
    fn from_case(case: &Value) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut nodes = BTreeMap::new();
        for node in case["nodes"].as_array().unwrap() {
            let id = node["id"].as_str().unwrap();
            nodes.insert(id.to_owned(), document.reserve_node_handle().unwrap());
        }

        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_222).unwrap(), document.document_revision());
        Self::append_element(
            &mut batch,
            root,
            HostParent::Root,
            "scene-root",
            "display:block;position:relative;box-sizing:border-box;width:360px;height:800px;margin:0;padding:0;border:0;direction:ltr",
        );
        for node in case["nodes"].as_array().unwrap() {
            let id = node["id"].as_str().unwrap();
            let parent = node["parentId"]
                .as_str()
                .and_then(|parent| nodes.get(parent).copied())
                .map(HostParent::Node)
                .unwrap_or(HostParent::Node(root));
            Self::append_element(
                &mut batch,
                nodes[id],
                parent,
                id,
                node["authoredStyle"].as_str().unwrap(),
            );
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn append_element(
        batch: &mut DocumentChangeBatch,
        node: HostNodeHandle,
        parent: HostParent,
        id: &str,
        style: &str,
    ) {
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

    fn compute(&self, dpr: f32) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        compute_runtime_block_flex_grid_auto_placement_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 360.0,
                height_css_px: 800.0,
                device_scale_factor: dpr,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
    }

    fn compute_dense(&self, dpr: f32) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        compute_runtime_block_flex_grid_dense_auto_placement_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 360.0,
                height_css_px: 800.0,
                device_scale_factor: dpr,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
    }
}

fn reference() -> Value {
    let reference: Value = serde_json::from_str(CHROME_REFERENCE).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-css-c11-2-2-grid-auto-placement-precomparison/v4"
    );
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    reference
}

#[test]
fn all_supported_scene_frames_match_pinned_chrome_154() {
    let reference = reference();
    let cases = reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["deviceScaleFactor"] == 1)
        .unwrap()["cases"]
        .as_array()
        .unwrap();
    let mut checked_scenes = 0;
    let mut checked_nodes = 0;

    for case in cases
        .iter()
        .filter(|case| case["scope"] == "implementation-candidate")
    {
        let fixture = Fixture::from_case(case);
        let output = fixture
            .compute(1.0)
            .unwrap_or_else(|error| panic!("{}: 지원 장면 계산 실패: {error:?}", case["id"]));
        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
            "{}: profile이 바뀌었습니다",
            case["id"]
        );

        for node in case["nodes"].as_array().unwrap() {
            let name = node["id"].as_str().unwrap();
            let handle = fixture.nodes[name];
            let actual = output.layout.frames[&handle.id()];
            let expected = &node["rect"];
            for (field, value) in [
                ("x", actual.x),
                ("y", actual.y),
                ("width", actual.width),
                ("height", actual.height),
            ] {
                let expected = expected[field].as_f64().unwrap() as f32;
                assert!(
                    (value - expected).abs() <= 0.5,
                    "{} {} {field}: Spinon={value}, Chrome={expected}",
                    case["id"],
                    name
                );
            }
            checked_nodes += 1;
        }
        checked_scenes += 1;
    }

    assert_eq!(checked_scenes, 11);
    assert_eq!(checked_nodes, 43);
}

#[test]
fn chromium_only_scenes_are_rejected_by_the_documented_subset() {
    let reference = reference();
    let cases = reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["deviceScaleFactor"] == 1)
        .unwrap()["cases"]
        .as_array()
        .unwrap();
    let mut rejected = 0;

    for case in cases
        .iter()
        .filter(|case| case["scope"] == "chromium-oracle-only")
    {
        let fixture = Fixture::from_case(case);
        assert!(
            fixture.compute(1.0).is_err(),
            "{}: subset 밖 장면이 성공했습니다",
            case["id"]
        );
        rejected += 1;
    }

    assert_eq!(rejected, 5);
}

#[test]
fn sparse_order_runtime_cases_match_chrome_and_keep_host_dom_order() {
    let matrix: Value = serde_json::from_str(ORDER_RUNTIME_MATRIX).unwrap();
    assert_eq!(
        matrix["schema"],
        "spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1"
    );
    assert_eq!(matrix["profileId"], "RuntimeBlockFlexGridAutoPlacementV1");
    assert_eq!(matrix["summary"]["sceneCount"], 2);

    for case in matrix["cases"].as_array().unwrap() {
        let scene_id = case["id"].as_str().unwrap();
        let fixture = Fixture::from_case(case);
        let snapshot = fixture.document.snapshot();
        let output = fixture
            .compute(1.0)
            .unwrap_or_else(|error| panic!("{scene_id}: ordered Grid 계산 실패: {error:?}"));
        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
            "{scene_id}: computed profile"
        );

        let root = fixture.nodes[case["rootId"].as_str().unwrap()];
        let expected_dom_children = case["domItemOrder"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| fixture.nodes[value.as_str().unwrap()])
            .collect::<Vec<_>>();
        assert_eq!(
            snapshot.children(root).unwrap().collect::<Vec<_>>(),
            expected_dom_children,
            "{scene_id}: order가 HostDocument DOM child 순서를 바꾸면 안 됩니다"
        );

        for (node_id, expected_order) in case["computedOrders"].as_object().unwrap() {
            let handle = fixture.nodes[node_id];
            let computed = output
                .computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == handle.id())
                .unwrap_or_else(|| panic!("{scene_id}/{node_id}: computed style 누락"));
            assert_eq!(
                computed.properties["order"],
                expected_order.as_str().unwrap()
            );
        }

        for expected in case["expectedRects"].as_array().unwrap() {
            let node_id = expected["id"].as_str().unwrap();
            let handle = fixture.nodes[node_id];
            let actual = output.layout.frames[&handle.id()];
            let expected = &expected["rect"];
            for (field, value) in [
                ("x", actual.x),
                ("y", actual.y),
                ("width", actual.width),
                ("height", actual.height),
            ] {
                let expected = expected[field].as_f64().unwrap() as f32;
                assert!(
                    (value - expected).abs() <= 0.5,
                    "{scene_id}/{node_id} {field}: Spinon={value}, Chrome={expected}"
                );
            }
        }
    }
}

#[test]
fn dense_grid_profile_still_rejects_nonzero_order() {
    let matrix: Value = serde_json::from_str(ORDER_RUNTIME_MATRIX).unwrap();
    let case = matrix["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "order-mixed")
        .unwrap();
    let fixture = Fixture::from_case(case);

    assert!(matches!(
        fixture.compute_dense(1.0),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "order",
            ..
        })
    ));
}
