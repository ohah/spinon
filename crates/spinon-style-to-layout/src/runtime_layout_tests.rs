use std::collections::BTreeMap;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_layout_cascade, compute_runtime_flex_paint_cascade,
    first_unsupported_runtime_layout_inline_property,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/chromium-macos-arm64-macos-26.5.1-25f80-c04-runtime-layout-ebd9482a8d06-6849840baee0-6d02aa4972a1-ccffd5c5fe77/runtime-style-layout.json"
);
const RESIZE_REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c04-runtime-css-to-gpu-resize-v1.json"
);

fn runtime_fixture() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    runtime_fixture_with_flex_b_style(
        "display:flex;width:100px;height:80px;flex-direction:column;row-gap:4px;padding:2px",
    )
}

fn runtime_fixture_with_flex_b_style(
    flex_b_style: &str,
) -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(4901).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let flex_a = document.reserve_node_handle().unwrap();
    let flex_b = document.reserve_node_handle().unwrap();
    let block_child = document.reserve_node_handle().unwrap();
    let hidden = document.reserve_node_handle().unwrap();
    let nodes = BTreeMap::from([
        ("root".to_owned(), root),
        ("flex-a".to_owned(), flex_a),
        ("flex-b".to_owned(), flex_b),
        ("block-child".to_owned(), block_child),
        ("hidden".to_owned(), hidden),
    ]);
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (id, handle, parent, style) in [
        (
            "root",
            root,
            HostParent::Root,
            "display:flex;box-sizing:border-box;width:300px;height:140px;flex-direction:row;align-items:center;justify-content:flex-start;column-gap:7px;padding:5px",
        ),
        (
            "flex-a",
            flex_a,
            HostParent::Node(root),
            "display:block;width:40px;height:30px;margin:2px;padding:3px",
        ),
        ("flex-b", flex_b, HostParent::Node(root), flex_b_style),
        (
            "block-child",
            block_child,
            HostParent::Node(flex_b),
            "display:block;width:30px;height:11px;margin-top:3px",
        ),
        (
            "hidden",
            hidden,
            HostParent::Node(flex_b),
            "display:none;width:20px;height:20px",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, nodes)
}

fn runtime_resize_fixture() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(4902).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let opaque = document.reserve_node_handle().unwrap();
    let transparent = document.reserve_node_handle().unwrap();
    let hidden = document.reserve_node_handle().unwrap();
    let nodes = BTreeMap::from([
        ("root".to_owned(), root),
        ("opaque".to_owned(), opaque),
        ("transparent".to_owned(), transparent),
        ("hidden".to_owned(), hidden),
    ]);
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (id, handle, parent, style) in [
        (
            "root",
            root,
            spinon_core::HostParent::Root,
            "display:flex;box-sizing:border-box;width:100vw;height:100vh;flex-direction:row;align-items:flex-start;justify-content:flex-start;column-gap:11px;background-color:#123456",
        ),
        (
            "opaque",
            opaque,
            spinon_core::HostParent::Node(root),
            "display:block;box-sizing:border-box;width:51px;height:31px;background-color:#3366ff",
        ),
        (
            "transparent",
            transparent,
            spinon_core::HostParent::Node(root),
            "display:block;box-sizing:border-box;width:41px;height:31px",
        ),
        (
            "hidden",
            hidden,
            spinon_core::HostParent::Node(root),
            "display:none;width:23px;height:17px;background-color:#ff0000",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, nodes)
}

#[test]
fn runtime_viewport_units_recompute_to_each_frozen_chromium_resize_size() {
    let (document, root, nodes) = runtime_resize_fixture();
    let snapshot = document.snapshot();
    let reference: Value =
        serde_json::from_slice(&std::fs::read(RESIZE_REFERENCE).unwrap()).unwrap();

    for (scenario, width, height) in [("baseline", 301.0, 100.0), ("expanded", 341.0, 128.0)] {
        let viewport = CssViewport {
            width_css_px: width,
            height_css_px: height,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view = StyloDocumentView::new_html_fragment_child_shared(
            std::sync::Arc::new(snapshot.clone()),
            root,
        )
        .unwrap();
        let computed =
            compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
        let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
        let expected_nodes = reference["observations"][scenario]["nodes"]
            .as_array()
            .unwrap();

        for expected_node in expected_nodes {
            let id = expected_node["id"].as_str().unwrap();
            let node_id = nodes[id].id();
            let style = output
                .computed_styles
                .elements
                .iter()
                .find(|element| element.node_id == node_id)
                .unwrap_or_else(|| panic!("{scenario}.{id}의 computed style이 없습니다"));
            for (property, expected) in expected_node["properties"].as_object().unwrap() {
                assert_eq!(
                    style.properties.get(property).map(String::as_str),
                    expected.as_str(),
                    "{scenario}.{id}.{property}"
                );
            }
            let frame = output.layout.frames.get(&node_id).unwrap();
            for (field, actual, expected) in [
                (
                    "x",
                    frame.x,
                    expected_node["rect"]["x"].as_f64().unwrap() as f32,
                ),
                (
                    "y",
                    frame.y,
                    expected_node["rect"]["y"].as_f64().unwrap() as f32,
                ),
                (
                    "width",
                    frame.width,
                    expected_node["rect"]["width"].as_f64().unwrap() as f32,
                ),
                (
                    "height",
                    frame.height,
                    expected_node["rect"]["height"].as_f64().unwrap() as f32,
                ),
            ] {
                assert!(
                    (actual - expected).abs() <= 0.5,
                    "{scenario}.{id}.{field}: 실제 {actual}, Chromium {expected}, 허용치 0.5 CSS px"
                );
            }
        }
    }
}

#[test]
fn runtime_profile_matches_frozen_chromium_styles_and_geometry() {
    let (document, root, nodes) = runtime_fixture();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let viewport = CssViewport {
        width_css_px: 800.0,
        height_css_px: 600.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    };
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let computed =
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    assert_eq!(computed.profile, ComputedStyleProfile::RuntimeFlexLayoutV1);
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();

    let reference: Value = serde_json::from_slice(&std::fs::read(REFERENCE).unwrap()).unwrap();
    for expected_node in reference["observations"]["nodes"].as_array().unwrap() {
        let fixture_id = expected_node["id"].as_str().unwrap();
        let node_id = nodes[fixture_id].id();
        let actual_style = output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == node_id)
            .unwrap_or_else(|| panic!("{fixture_id}의 computed style이 없습니다"));
        for (property, expected) in expected_node["properties"].as_object().unwrap() {
            assert_eq!(
                actual_style.properties.get(property).map(String::as_str),
                expected.as_str(),
                "{fixture_id}.{property}"
            );
        }

        let frame = output.layout.frames.get(&node_id).unwrap();
        for (field, actual, expected) in [
            (
                "x",
                frame.x,
                expected_node["rect"]["x"].as_f64().unwrap() as f32,
            ),
            (
                "y",
                frame.y,
                expected_node["rect"]["y"].as_f64().unwrap() as f32,
            ),
            (
                "width",
                frame.width,
                expected_node["rect"]["width"].as_f64().unwrap() as f32,
            ),
            (
                "height",
                frame.height,
                expected_node["rect"]["height"].as_f64().unwrap() as f32,
            ),
        ] {
            assert!(
                (actual - expected).abs() <= 0.5,
                "{fixture_id}.{field}: 실제 {actual}, Chromium {expected}, 허용치 0.5 CSS px"
            );
        }
    }
}

#[test]
fn runtime_flex_shorthand_matches_frozen_chromium_longhand_values_and_geometry() {
    let (document, root, nodes) = runtime_fixture_with_flex_b_style(
        "display:flex;width:100px;height:80px;flex-direction:column;row-gap:4px;padding:2px;flex:0 1 auto",
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let computed = compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let output =
        compute_runtime_style_layout(&snapshot, root, computed, CssViewport::C04_FIXTURE).unwrap();
    let reference: Value = serde_json::from_slice(&std::fs::read(REFERENCE).unwrap()).unwrap();
    let expected_node = reference["observations"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["id"] == "flex-b")
        .unwrap();
    let node_id = nodes["flex-b"].id();
    let actual_style = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == node_id)
        .unwrap();
    for (property, expected) in expected_node["properties"].as_object().unwrap() {
        assert_eq!(
            actual_style.properties.get(property).map(String::as_str),
            expected.as_str(),
            "flex-b.{property}"
        );
    }
    let frame = output.layout.frames.get(&node_id).unwrap();
    for (field, actual, expected) in [
        (
            "x",
            frame.x,
            expected_node["rect"]["x"].as_f64().unwrap() as f32,
        ),
        (
            "y",
            frame.y,
            expected_node["rect"]["y"].as_f64().unwrap() as f32,
        ),
        (
            "width",
            frame.width,
            expected_node["rect"]["width"].as_f64().unwrap() as f32,
        ),
        (
            "height",
            frame.height,
            expected_node["rect"]["height"].as_f64().unwrap() as f32,
        ),
    ] {
        assert!(
            (actual - expected).abs() <= 0.5,
            "flex-b.{field}: 실제 {actual}, Chromium {expected}, 허용치 0.5 CSS px"
        );
    }
}

#[test]
fn runtime_flex_shorthand_preserves_percentage_basis_for_taffy() {
    let (document, root, nodes) = runtime_fixture_with_flex_b_style(
        "display:flex;width:100px;height:80px;flex-direction:column;row-gap:4px;padding:2px;flex:0 0 50%",
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let computed = compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let output =
        compute_runtime_style_layout(&snapshot, root, computed, CssViewport::C04_FIXTURE).unwrap();
    let flex_basis = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == nodes["flex-b"].id())
        .unwrap()
        .layout_dimensions
        .flex_basis;
    assert_eq!(
        flex_basis,
        spinon_style::ComputedCssDimension::Percentage(0.5)
    );
}
