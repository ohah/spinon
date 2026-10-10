use std::{collections::BTreeMap, fs, sync::Arc};

use crate::compute_runtime_style_layout;
use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const FIXTURE_HTML: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c06/font-relative-units.html"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-font-relative-units-v1.json"
);

struct DprBaseline {
    properties: BTreeMap<NodeId, BTreeMap<String, String>>,
    frames: BTreeMap<NodeId, LayoutFrame>,
}

const TREE: &[(&str, Option<&str>)] = &[
    ("mount", None),
    ("em-inherit", Some("mount")),
    ("em-parent", Some("mount")),
    ("em-child", Some("em-parent")),
    ("rem-target", Some("mount")),
    ("font-size-percent", Some("mount")),
    ("zero", Some("mount")),
    ("var-em", Some("mount")),
    ("var-rem", Some("mount")),
    ("spacing", Some("mount")),
    ("basis-a", Some("spacing")),
    ("basis-b", Some("spacing")),
];

fn document_fixture() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(6404).unwrap();
    let nodes = TREE
        .iter()
        .map(|(id, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
        .collect::<BTreeMap<_, _>>();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());

    for (id, parent_id) in TREE {
        let node = nodes[*id];
        batch
            .push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: (*id).to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: parent_id
                    .map_or(HostParent::Root, |parent| HostParent::Node(nodes[parent])),
                node,
                before: None,
            });
    }
    document.commit(batch).unwrap();
    let root = nodes["mount"];
    (document, root, nodes)
}

fn stylesheet_from_chromium_fixture() -> StylesheetSource {
    let html = fs::read_to_string(FIXTURE_HTML).unwrap();
    let (_, style_and_rest) = html
        .split_once("<style>")
        .expect("fixture must contain one inline style element");
    let (css, _) = style_and_rest
        .split_once("</style>")
        .expect("fixture style element must close");
    assert!(
        !css.contains("<style>"),
        "fixture must contain exactly one style block"
    );
    StylesheetSource {
        id: "c06-4-font-relative-units".to_owned(),
        base_url: "https://spinon.invalid/c06/font-relative-units.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn number(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
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

#[test]
fn runtime_em_rem_computed_values_and_taffy_frames_match_chromium() {
    let (document, root, nodes) = document_fixture();
    let snapshot = document.snapshot();
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(reference["observations"].as_array().unwrap().len(), 2);

    let stylesheet = stylesheet_from_chromium_fixture();
    let mut dpr_one: Option<DprBaseline> = None;
    for device_scale_factor in [1.0_f32, 2.0_f32] {
        let viewport = CssViewport {
            width_css_px: 320.0,
            height_css_px: 800.0,
            device_scale_factor,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root)
                .unwrap();
        let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            std::slice::from_ref(&stylesheet),
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap();
        let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
        let expected = reference["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|observation| {
                number(&observation["viewport"]["deviceScaleFactor"]) == device_scale_factor
            })
            .unwrap();

        assert_eq!(output.computed_styles.document_root_font_size_css_px, 20.0);
        assert_eq!(output.computed_styles.elements.len(), TREE.len());
        assert_eq!(output.layout.frames.len(), TREE.len());

        for expected_node in expected["nodes"].as_array().unwrap() {
            let id = expected_node["id"].as_str().unwrap();
            if id == "html" || id == "body" {
                continue;
            }
            let node_id = nodes[id].id();
            let actual_style = output
                .computed_styles
                .elements
                .iter()
                .find(|element| element.node_id == node_id)
                .unwrap_or_else(|| panic!("{id}의 computed style이 없습니다"));
            for (property, expected_value) in expected_node["properties"].as_object().unwrap() {
                if matches!(
                    (id, property.as_str()),
                    ("basis-a", "height") | ("basis-b", "height")
                ) {
                    // Chrome resolves a flex item's computed height to its post-flex used size.
                    // Stylo's pre-layout computed value remains the specified 1rem/1em length;
                    // compare that typed input below and compare the final size through rect.
                    let expected_typed_height = if id == "basis-a" { 20.0 } else { 12.0 };
                    assert_eq!(
                        actual_style.layout_dimensions.height,
                        spinon_style::ComputedCssDimension::LengthPx(expected_typed_height),
                        "DPR {device_scale_factor}, {id}.height pre-layout typed CSS value"
                    );
                    assert_eq!(
                        css_px(expected_value),
                        number(&expected_node["rect"]["height"]),
                        "Chrome's resolved flex-item height must equal its final rect"
                    );
                    continue;
                }
                assert_eq!(
                    actual_style.properties.get(property).map(String::as_str),
                    expected_value.as_str(),
                    "DPR {device_scale_factor}, {id}.{property}"
                );
            }

            let frame = output.layout.frames.get(&node_id).unwrap();
            for (field, actual, expected_value) in [
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
            ] {
                assert!(
                    (actual - expected_value).abs() <= 0.5,
                    "DPR {device_scale_factor}, {id}.{field}: Rust={actual}, Chrome={expected_value} CSS px"
                );
            }
        }

        if let Some(baseline) = dpr_one.take() {
            assert_eq!(
                output.computed_styles.elements.len(),
                baseline.properties.len(),
                "DPR change must preserve computed-style node count"
            );
            for actual in output.computed_styles.elements.iter() {
                assert_eq!(
                    actual.properties, baseline.properties[&actual.node_id],
                    "DPR change must not alter CSS computed values"
                );
            }
            assert_eq!(
                output.layout.frames, baseline.frames,
                "DPR must not scale CSS px layout"
            );
        } else {
            dpr_one = Some(DprBaseline {
                properties: output
                    .computed_styles
                    .elements
                    .iter()
                    .map(|style| (style.node_id, style.properties.clone()))
                    .collect::<BTreeMap<_, _>>(),
                frames: output.layout.frames.clone(),
            });
        }
    }
}
