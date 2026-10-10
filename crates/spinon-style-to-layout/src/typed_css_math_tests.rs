use std::{collections::BTreeMap, fs, sync::Arc};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView, compute_runtime_flex_layout_cascade,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-typed-css-math-v1.json"
);

#[test]
fn typed_css_math_matches_chromium_computed_geometry() {
    let (document, root, nodes) = fixture();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 800.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::INITIAL,
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed =
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");

    let expected_nodes = reference["observations"][0]["nodes"].as_array().unwrap();
    let mut compared = 0;
    let mut maximum_error = 0.0_f32;
    for expected in expected_nodes {
        let id = expected["id"].as_str().unwrap();
        if id == "round-function" {
            continue;
        }
        let node_id = nodes[id].id();
        let frame = output
            .layout
            .frames
            .get(&node_id)
            .unwrap_or_else(|| panic!("{id}의 layout frame이 없습니다"));
        maximum_error = maximum_error.max(assert_frame_matches(frame, &expected["rect"], id));
        compared += 1;
    }
    assert_eq!(
        compared, 25,
        "Chrome fixture의 지원 범위를 명시적으로 비교합니다"
    );
    assert!(
        maximum_error <= 0.5,
        "최대 frame 오차: {maximum_error} CSS px"
    );
}

fn assert_frame_matches(actual: &LayoutFrame, expected: &Value, id: &str) -> f32 {
    let mut maximum = 0.0_f32;
    for (field, value) in [
        ("x", actual.x),
        ("y", actual.y),
        ("width", actual.width),
        ("height", actual.height),
    ] {
        let reference = expected[field].as_f64().unwrap() as f32;
        let error = (value - reference).abs();
        maximum = maximum.max(error);
        assert!(
            error <= 0.5,
            "{id}.{field}: Spinon {value}, Chrome {reference}, 허용치 0.5 CSS px"
        );
    }
    maximum
}

fn fixture() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    fixture_math()
}

fn fixture_math() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let specifications = vec![
        (
            "root",
            None,
            "display:flex;flex-direction:column;box-sizing:border-box;width:300px;height:780px;margin:0;padding:0",
        ),
        (
            "calc-length",
            Some("root"),
            "width:calc(20px + 25px);height:10px",
        ),
        (
            "calc-mixed",
            Some("root"),
            "width:calc(10px + 25%);height:10px",
        ),
        (
            "calc-nested",
            Some("root"),
            "width:calc(10px + calc(20px + 10%));height:10px",
        ),
        (
            "calc-product",
            Some("root"),
            "width:calc(10px + 4 * 5px + 20px / 4);height:10px",
        ),
        (
            "min-value",
            Some("root"),
            "width:min(100px, 50%);height:10px",
        ),
        (
            "max-value",
            Some("root"),
            "width:max(100px, 50%);height:10px",
        ),
        (
            "min-mixed",
            Some("root"),
            "width:min(calc(50px + 10%), calc(80px + 5%));height:10px",
        ),
        (
            "max-nested",
            Some("root"),
            "width:max(20px, min(40%, 60px));height:10px",
        ),
        (
            "clamp-value",
            Some("root"),
            "width:clamp(20px, 50%, 100px);height:10px",
        ),
        (
            "clamp-negative",
            Some("root"),
            "width:clamp(20px, -10px, 100px);height:10px",
        ),
        (
            "clamp-inverted-bounds",
            Some("root"),
            "width:clamp(100px, 80px, 50px);height:10px",
        ),
        (
            "clamp-nan",
            Some("root"),
            "width:clamp(10px, calc(0px / 0), 20px);height:10px",
        ),
        (
            "var-math",
            Some("root"),
            "--box:calc(20px + 10%);width:var(--box);height:10px",
        ),
        (
            "negative-margin",
            Some("root"),
            "width:30px;height:10px;margin-left:calc(5px - 10px)",
        ),
        (
            "padding-math",
            Some("root"),
            "box-sizing:border-box;width:50px;height:10px;padding-left:calc(5px + 10px)",
        ),
        (
            "gap-container",
            Some("root"),
            "display:flex;flex-direction:row;box-sizing:border-box;width:300px;height:20px;column-gap:calc(5px + 5%)",
        ),
        (
            "gap-first",
            Some("gap-container"),
            "flex:0 0 30px;height:10px",
        ),
        (
            "gap-second",
            Some("gap-container"),
            "flex:0 0 30px;height:10px",
        ),
        (
            "basis-container",
            Some("root"),
            "display:flex;flex-direction:row;box-sizing:border-box;width:300px;height:20px",
        ),
        (
            "basis-math",
            Some("basis-container"),
            "flex:0 0 calc(40px + 10%);height:10px",
        ),
        (
            "divide-by-zero",
            Some("root"),
            "width:calc(10px / 0);height:10px",
        ),
        (
            "zero-divided-by-zero",
            Some("root"),
            "width:calc(0px / 0);height:10px",
        ),
        (
            "negative-size",
            Some("root"),
            "width:calc(20px - 50px);height:10px",
        ),
        (
            "invalid-dimension",
            Some("root"),
            "width:23px;width:calc(1px + 1s);height:10px",
        ),
    ];
    build_fixture(&specifications)
}

fn build_fixture(
    specifications: &[(&str, Option<&str>, &str)],
) -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(7605).unwrap();
    let mut nodes = BTreeMap::new();
    for (id, _, _) in specifications {
        nodes.insert((*id).to_owned(), document.reserve_node_handle().unwrap());
    }
    let root = nodes["root"];
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (id, parent, style) in specifications {
        let node = nodes[*id];
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: (*id).to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: (*style).to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: parent.map_or(HostParent::Root, |parent| HostParent::Node(nodes[parent])),
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, nodes)
}
