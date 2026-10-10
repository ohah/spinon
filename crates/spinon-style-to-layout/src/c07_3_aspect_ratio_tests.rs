use std::{collections::BTreeMap, fs, sync::Arc};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssCascadeError, CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource,
    StyloDocumentView, compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML_NS: &str = "http://www.w3.org/1999/xhtml";
const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c07/aspect-ratio.html"
);
const INVENTORY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c07/aspect-ratio-inventory.json"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c07-3-aspect-ratio-v1.json"
);
const GEOMETRY_TOLERANCE: f32 = 0.5;

fn viewport(dpr: f32) -> CssViewport {
    CssViewport {
        width_css_px: 400.0,
        height_css_px: 1200.0,
        device_scale_factor: dpr,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn fixture_stylesheet() -> StylesheetSource {
    let html = fs::read_to_string(FIXTURE).unwrap();
    let (_, rest) = html.split_once("<style>").expect("CSS fixture exists");
    let (css, _) = rest.split_once("</style>").expect("CSS fixture closes");
    let mut css = css.to_owned();
    // Keep browser-reference vertical positions stable while these cases are
    // covered by separate fail-closed tests instead of the positive geometry run.
    css.push_str(
        "\n#ratio-min-height-transfer { width:100px; height:80px; aspect-ratio:auto; }\
         \n#ratio-max-height-transfer { width:160px; height:60px; aspect-ratio:auto; }\
         \n#ratio-min-width-transfer { width:100px; height:40px; aspect-ratio:auto; }\
         \n#ratio-max-width-transfer { width:120px; height:80px; aspect-ratio:auto; }\
         \n#ratio-flex-min-max > .child { width:120px; height:80px; aspect-ratio:auto; }\n",
    );
    StylesheetSource {
        id: "c07-3-aspect-ratio".to_owned(),
        base_url: "https://spinon.invalid/c07/aspect-ratio.css".to_owned(),
        origin: CssOrigin::Author,
        css,
    }
}

fn fixture_document() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let inventory: Value = serde_json::from_slice(&fs::read(INVENTORY).unwrap()).unwrap();
    let ids = inventory["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["id"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut nodes = ids
        .iter()
        .map(|id| (id.clone(), document.reserve_node_handle().unwrap()))
        .collect::<BTreeMap<_, _>>();
    nodes.insert("fixture".to_owned(), root);
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(7730).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id").unwrap(),
            value: "fixture".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });

    for entry in inventory["nodes"].as_array().unwrap() {
        let id = entry["id"].as_str().unwrap();
        let node = nodes[id];
        batch
            .push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NS.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.into(),
            });
        if let Some(class) = entry["class"].as_str() {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "class").unwrap(),
                value: class.into(),
            });
        }
        let parent = entry["parent"]
            .as_str()
            .map(|parent_id| HostParent::Node(nodes[parent_id]))
            .unwrap_or(HostParent::Node(root));
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, nodes)
}

fn compute_fixture(
    document: &HostDocument,
    root: HostNodeHandle,
    dpr: f32,
) -> crate::StyleLayoutOutput {
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[fixture_stylesheet()],
        viewport(dpr),
        StyleRevision::INITIAL,
    )
    .unwrap();
    compute_runtime_style_layout(&snapshot, root, computed, viewport(dpr)).unwrap()
}

fn compute_stylesheet_layout(
    document: &HostDocument,
    root: HostNodeHandle,
    css: &str,
    style_revision: StyleRevision,
) -> crate::StyleLayoutOutput {
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[StylesheetSource {
            id: "c07-3-ratio-revision".to_owned(),
            base_url: "https://spinon.invalid/c07/ratio-revision.css".to_owned(),
            origin: CssOrigin::Author,
            css: css.to_owned(),
        }],
        viewport(1.0),
        style_revision,
    )
    .unwrap();
    compute_runtime_style_layout(&snapshot, root, computed, viewport(1.0)).unwrap()
}

fn number(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

fn expected_layout_ratio(node: &Value) -> Option<f32> {
    let value = node["typed"]["aspect-ratio"]["text"].as_str()?;
    if value == "auto" || value.is_empty() {
        return None;
    }
    let (numerator, denominator) = value
        .split_once('/')
        .unwrap_or_else(|| panic!("예상하지 못한 Chromium 종횡비 값: {value}"));
    let numerator = numerator.trim().parse::<f32>().unwrap();
    let denominator = denominator.trim().parse::<f32>().unwrap();
    let ratio = numerator / denominator;
    (numerator != 0.0 && denominator != 0.0 && ratio.is_finite() && ratio > 0.0).then_some(ratio)
}

fn compare_dpr(dpr: f32, reference: &Value) -> BTreeMap<String, [f32; 4]> {
    let (document, root, nodes) = fixture_document();
    let output = compute_fixture(&document, root, dpr);
    let inventory: Value = serde_json::from_slice(&fs::read(INVENTORY).unwrap()).unwrap();
    let supported_ids = inventory["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|node| node["layoutSupport"] != "unsupported-min-max-ratio")
        .map(|node| node["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let expected = reference["observations"][0]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|node| supported_ids.contains(node["id"].as_str().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(expected.len(), 30);
    let mut observed = BTreeMap::new();
    let mut geometry_mismatches = Vec::new();

    for expected_node in expected {
        let id = expected_node["id"].as_str().unwrap();
        let node_id = nodes[id].id();
        let style = output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == node_id)
            .unwrap_or_else(|| panic!("{dpr}x {id} 계산 스타일이 없습니다"));
        for property in ["aspect-ratio", "box-sizing"] {
            let Some(expected_value) = expected_node["typed"][property]["text"].as_str() else {
                continue;
            };
            assert_eq!(
                style.properties.get(property).map(String::as_str),
                Some(expected_value),
                "{dpr}x {id}.{property} computed style"
            );
        }
        let expected_ratio = expected_layout_ratio(expected_node);
        match (style.layout_aspect_ratio, expected_ratio) {
            (Some(actual), Some(expected)) => assert!(
                (actual - expected).abs() <= 0.00001,
                "{dpr}x {id}.aspect-ratio: Spinon {actual}, Chrome {expected}"
            ),
            (actual, expected) => assert_eq!(
                actual.is_some(),
                expected.is_some(),
                "{dpr}x {id}.aspect-ratio presence: Spinon {actual:?}, Chrome {expected:?}"
            ),
        }
        let expected_rect = &expected_node["rect"];
        let frame = output
            .layout
            .frames
            .get(&node_id)
            .unwrap_or_else(|| panic!("{dpr}x {id} 레이아웃 상자가 없습니다"));
        let actual = [frame.x, frame.y, frame.width, frame.height];
        for (field, actual, expected) in [
            ("x", frame.x, number(&expected_rect["x"])),
            ("y", frame.y, number(&expected_rect["y"])),
            ("width", frame.width, number(&expected_rect["width"])),
            ("height", frame.height, number(&expected_rect["height"])),
        ] {
            if (actual - expected).abs() > GEOMETRY_TOLERANCE {
                geometry_mismatches.push(format!(
                    "{id}.{field}: Spinon {actual}, Chrome {expected}, 허용치 {GEOMETRY_TOLERANCE} CSS px"
                ));
            }
        }
        observed.insert(id.to_owned(), actual);
    }
    assert!(
        geometry_mismatches.is_empty(),
        "{dpr}x geometry mismatches:\n{}",
        geometry_mismatches.join("\n")
    );
    observed
}

fn compute_inline_root_layout(style: &str) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(7732).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[],
        viewport(1.0),
        StyleRevision::INITIAL,
    )?;
    compute_runtime_style_layout(&snapshot, root, computed, viewport(1.0))
}

fn compute_inline_flex_child_layout(
    child_style: &str,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(7733).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "style").unwrap(),
            value: "display:flex;width:180px;height:100px".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::CreateElement {
            node: child,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: child,
            name: AttributeName::new(None, "style").unwrap(),
            value: child_style.to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: child,
            before: None,
        });
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[],
        viewport(1.0),
        StyleRevision::INITIAL,
    )?;
    compute_runtime_style_layout(&snapshot, root, computed, viewport(1.0))
}

mod comparison;
mod failures;
mod revision;
