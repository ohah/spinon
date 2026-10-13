use std::collections::BTreeMap;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutError;
use spinon_style::{ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_block_flex_grid_dense_auto_placement_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const CHROME_REFERENCE: &str = include_str!(
    "../../../tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json"
);

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
            DocumentChangeBatch::new(OwnerId::new(11_223).unwrap(), document.document_revision());
        append_element(
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
            append_element(
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

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
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
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
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

fn chrome_cases() -> Vec<Value> {
    let reference: Value = serde_json::from_str(CHROME_REFERENCE).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-css-c11-2-3-grid-auto-placement-precomparison/v7"
    );
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["deviceScaleFactor"] == 1)
        .unwrap()["cases"]
        .as_array()
        .unwrap()
        .clone()
}

#[test]
fn supported_dense_column_and_sparse_cases_match_pinned_chrome_154() {
    let cases = chrome_cases();
    let mut checked_scenes = 0;
    let mut checked_nodes = 0;

    for case in cases.iter().filter(|case| {
        case["scope"] == "implementation-candidate"
            && case["id"] != "auto-span-over-track-limit"
            && case["id"] != "row-dense-fixed-row"
    }) {
        let fixture = Fixture::from_case(case);
        let output = match fixture.compute() {
            Ok(output) => output,
            Err(error) => panic!("{}: 지원 scene 계산 실패: {error:?}", case["id"]),
        };
        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1,
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

    assert_eq!(checked_scenes, 19);
    assert_eq!(checked_nodes, 86);
}

#[test]
fn unsupported_oracle_cases_and_track_limit_fail_closed() {
    for case in chrome_cases()
        .iter()
        .filter(|case| case["scope"] == "chromium-oracle-only")
    {
        let fixture = Fixture::from_case(case);
        let error = fixture
            .compute()
            .expect_err("oracle-only scene은 거부되어야 합니다");
        match case["id"].as_str().unwrap() {
            "reject-repeated-auto-track" => assert!(matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: "grid-auto-rows",
                    ..
                }
            )),
            "reject-negative-implicit-line" => assert!(matches!(
                error,
                StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
                    property: "grid-column-start",
                    ..
                })
            )),
            "reject-nonzero-order" => assert!(matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: "order",
                    ..
                }
            )),
            "reject-indefinite-axis" => assert!(matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: "height",
                    ..
                }
            )),
            id => panic!("예상하지 못한 oracle-only scene: {id}"),
        }
    }

    let case = chrome_cases()
        .into_iter()
        .find(|case| case["id"] == "row-dense-fixed-row")
        .unwrap();
    assert!(matches!(
        Fixture::from_case(&case).compute(),
        Err(StyleLayoutError::Layout(
            LayoutError::UnsupportedGridAutoColumns { .. }
        ))
    ));

    let case = chrome_cases()
        .into_iter()
        .find(|case| case["id"] == "auto-span-over-track-limit")
        .unwrap();
    let fixture = Fixture::from_case(&case);
    let container = fixture.nodes[case["rootId"].as_str().unwrap()].id();
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::Layout(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-columns",
            count: 10_001,
            maximum: 10_000,
        })) if node == container
    ));
}
