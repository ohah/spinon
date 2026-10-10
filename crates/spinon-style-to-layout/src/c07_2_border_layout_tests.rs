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

use crate::compute_runtime_style_layout;

const HTML_NS: &str = "http://www.w3.org/1999/xhtml";
const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c07/border-width-layout.html"
);
const INVENTORY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c07/border-width-layout-inventory.json"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c07-2-border-width-layout-v1.json"
);
const GEOMETRY_TOLERANCE: f32 = 0.5;

fn fixture_stylesheet() -> StylesheetSource {
    let html = fs::read_to_string(FIXTURE).unwrap();
    let (_, rest) = html.split_once("<style>").expect("CSS fixture exists");
    let (css, _) = rest.split_once("</style>").expect("CSS fixture closes");
    let outline_declaration_count = css.matches("outline: 20px solid red;").count();
    assert_eq!(
        outline_declaration_count, 1,
        "기준 전용 outline 규칙이 달라졌습니다"
    );
    let css = css.replace("outline: 20px solid red;", "");
    StylesheetSource {
        id: "c07-2-border-width-layout".to_owned(),
        base_url: "https://spinon.invalid/c07/border-width-layout.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn fixture_document() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let inventory: Value = serde_json::from_str(&fs::read_to_string(INVENTORY).unwrap()).unwrap();
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
        DocumentChangeBatch::new(OwnerId::new(7720).unwrap(), document.document_revision());
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

    for id in ids {
        let node = nodes[&id];
        batch
            .push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NS.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.clone().into(),
            });
        if id != "flex-parent" {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "class").unwrap(),
                value: "case".into(),
            });
        }
        if id == "default-none" {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: "width:50px".into(),
            });
        }
        let parent = match id.as_str() {
            "flex-shrink-border" | "flex-shrink-sibling" => HostParent::Node(nodes["flex-parent"]),
            _ => HostParent::Node(root),
        };
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, nodes)
}

fn css_px(value: &Value) -> f32 {
    value
        .as_str()
        .unwrap()
        .strip_suffix("px")
        .unwrap()
        .parse()
        .unwrap()
}

fn number(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

fn expected_observation(reference: &Value, dpr: f32) -> &Value {
    reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| number(&observation["viewport"]["deviceScaleFactor"]) == dpr)
        .unwrap_or_else(|| panic!("DPR {dpr} 기준 결과가 없습니다"))
}

fn compute_fixture_layout(
    document: &HostDocument,
    root: HostNodeHandle,
    stylesheet: StylesheetSource,
    style_revision: StyleRevision,
) -> crate::StyleLayoutOutput {
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport {
            width_css_px: 400.0,
            height_css_px: 1000.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        },
        style_revision,
    )
    .unwrap();
    compute_runtime_style_layout(
        &snapshot,
        root,
        computed,
        CssViewport {
            width_css_px: 400.0,
            height_css_px: 1000.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        },
    )
    .unwrap()
}

fn compare_dpr(dpr: f32, reference: &Value) -> BTreeMap<String, [f32; 8]> {
    let (document, root, nodes) = fixture_document();
    let snapshot = document.snapshot();
    let viewport = CssViewport {
        width_css_px: 400.0,
        height_css_px: 1000.0,
        device_scale_factor: dpr,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[fixture_stylesheet()],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    let expected = expected_observation(reference, dpr)["nodes"]
        .as_array()
        .unwrap();
    assert_eq!(expected.len(), 50, "고정 기준 노드 수가 달라졌습니다");
    let mut observed = BTreeMap::new();

    for expected_node in expected {
        let id = expected_node["id"].as_str().unwrap();
        let node_id = nodes[id].id();
        let style = output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == node_id)
            .unwrap_or_else(|| panic!("{dpr}x {id} 계산 스타일이 없습니다"));
        for side in ["top", "right", "bottom", "left"] {
            let property = format!("border-{side}-style");
            assert_eq!(
                style.properties.get(&property).map(String::as_str),
                expected_node["properties"][&property].as_str(),
                "{dpr}x {id}.{property}"
            );
        }
        let frame = output
            .layout
            .frames
            .get(&node_id)
            .unwrap_or_else(|| panic!("{dpr}x {id} 레이아웃 상자가 없습니다"));
        let mut record = [0.0; 8];
        for (index, (field, actual, expected_value)) in [
            ("x", frame.x, number(&expected_node["rect"]["x"])),
            ("y", frame.y, number(&expected_node["rect"]["y"])),
            (
                "width",
                frame.width,
                number(&expected_node["rect"]["width"]),
            ),
            (
                "height",
                frame.height,
                number(&expected_node["rect"]["height"]),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            assert!(
                (actual - expected_value).abs() <= GEOMETRY_TOLERANCE,
                "{dpr}x {id}.{field}: 실제 {actual}, Chrome {expected_value}, 허용치 {GEOMETRY_TOLERANCE} CSS px"
            );
            record[index] = actual;
        }

        for (side_index, (side, actual)) in [
            ("top", style.layout_border.top),
            ("right", style.layout_border.right),
            ("bottom", style.layout_border.bottom),
            ("left", style.layout_border.left),
        ]
        .into_iter()
        .enumerate()
        {
            let property = format!("border-{side}-width");
            let expected_width = css_px(&expected_node["properties"][&property]);
            assert_eq!(
                style.properties.get(&property).map(String::as_str),
                expected_node["properties"][&property].as_str(),
                "{dpr}x {id}.{property} computed style"
            );
            assert!(
                (actual - expected_width).abs() <= 0.01,
                "{dpr}x {id}.{side} used width {actual}, Chrome {expected_width}"
            );
            record[4 + side_index] = actual;
        }
        observed.insert(id.to_owned(), record);
    }
    observed
}

#[test]
fn border_used_widths_and_frames_match_pinned_chromium_at_both_device_scales() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(reference["fixture"]["id"], "C07.2-border-width-v1");

    let dpr_one = compare_dpr(1.0, &reference);
    let dpr_two = compare_dpr(2.0, &reference);
    assert_eq!(dpr_one, dpr_two, "CSS px 결과가 DPR에 따라 달라졌습니다");
}

#[test]
fn authored_noninitial_border_image_is_rejected_by_the_layout_profile() {
    let (document, root, _) = fixture_document();
    let snapshot = document.snapshot();
    let viewport = CssViewport {
        width_css_px: 400.0,
        height_css_px: 1000.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view = StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot), root).unwrap();
    let stylesheet = StylesheetSource {
        id: "unsupported-border-image".to_owned(),
        base_url: "https://spinon.invalid/c07/border-image.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#fixture { border-image-repeat: round; }".to_owned(),
    };
    assert!(matches!(
        compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            &[stylesheet],
            viewport,
            StyleRevision::INITIAL,
        ),
        Err(CssCascadeError::UnsupportedAuthorCss { feature, .. })
            if feature.contains("border-image-repeat")
    ));
}

#[test]
fn changing_only_border_color_keeps_used_width_and_layout_frame() {
    let (baseline_document, baseline_root, baseline_nodes) = fixture_document();
    let baseline = compute_fixture_layout(
        &baseline_document,
        baseline_root,
        fixture_stylesheet(),
        StyleRevision::INITIAL,
    );

    let (changed_document, changed_root, changed_nodes) = fixture_document();
    let mut changed_stylesheet = fixture_stylesheet();
    changed_stylesheet
        .css
        .push_str("\n#explicit-one-value { border-color: transparent; }\n");
    let changed = compute_fixture_layout(
        &changed_document,
        changed_root,
        changed_stylesheet,
        StyleRevision::INITIAL.checked_next().unwrap(),
    );

    let baseline_id = baseline_nodes["explicit-one-value"].id();
    let changed_id = changed_nodes["explicit-one-value"].id();
    let baseline_style = baseline
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == baseline_id)
        .unwrap();
    let changed_style = changed
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == changed_id)
        .unwrap();

    assert_eq!(baseline_style.layout_border, changed_style.layout_border);
    assert_eq!(
        baseline.layout.frames[&baseline_id],
        changed.layout.frames[&changed_id]
    );
}
