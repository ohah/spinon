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
    "/../../tests/fixtures/css/c06/viewport-units.html"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-viewport-units-v1.json"
);
const TREE: &[(&str, Option<&str>)] = &[
    ("root", None),
    ("default-units", Some("root")),
    ("small-units", Some("root")),
    ("large-units", Some("root")),
    ("dynamic-units", Some("root")),
    ("logical-units", Some("root")),
    ("minmax-units", Some("root")),
    ("math-units", Some("root")),
    ("var-units", Some("root")),
    ("spacing-units", Some("root")),
    ("gap-row", Some("root")),
    ("gap-a", Some("gap-row")),
    ("gap-b", Some("gap-row")),
    ("basis-row", Some("root")),
    ("basis-item", Some("basis-row")),
];

struct Baseline {
    properties: BTreeMap<NodeId, BTreeMap<String, String>>,
    frames: BTreeMap<NodeId, LayoutFrame>,
}

fn fixture_document() -> (
    HostDocument,
    HostNodeHandle,
    BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let nodes = TREE
        .iter()
        .map(|(id, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
        .collect::<BTreeMap<_, _>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6606).unwrap(), document.document_revision());
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
    let root = nodes["root"];
    (document, root, nodes)
}

fn stylesheet() -> StylesheetSource {
    let html = fs::read_to_string(FIXTURE_HTML).unwrap();
    let (_, remainder) = html.split_once("<style>").expect("fixture must have style");
    let (css, _) = remainder.split_once("</style>").expect("style must close");
    StylesheetSource {
        id: "c06-6-viewport-units".to_owned(),
        base_url: "https://spinon.invalid/c06/viewport-units.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn number(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

fn css_px(value: &Value) -> f32 {
    let value = value.as_str().unwrap();
    value
        .strip_suffix("px")
        .unwrap_or_else(|| panic!("expected CSS px computed value, got {value:?}"))
        .parse()
        .unwrap()
}

fn compute(width: f32, height: f32, dpr: f32) -> (Baseline, BTreeMap<String, HostNodeHandle>) {
    let (document, root, nodes) = fixture_document();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let viewport = CssViewport {
        width_css_px: width,
        height_css_px: height,
        device_scale_factor: dpr,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet()],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let result = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    let properties = result
        .computed_styles
        .elements
        .iter()
        .map(|element| (element.node_id, element.properties.clone()))
        .collect();
    (
        Baseline {
            properties,
            frames: result.layout.frames,
        },
        nodes,
    )
}

fn observation(reference: &Value, width: f32, height: f32, dpr: f32) -> &Value {
    reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| {
            number(&item["viewport"]["width"]) == width
                && number(&item["viewport"]["height"]) == height
                && number(&item["viewport"]["deviceScaleFactor"]) == dpr
        })
        .unwrap()
}

#[test]
fn viewport_units_match_chromium_after_cascade_projection_and_taffy_layout() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");

    for (width, height) in [(320.0, 800.0), (800.0, 320.0), (390.0, 844.0)] {
        let mut dpr_one: Option<Baseline> = None;
        for dpr in [1.0, 2.0] {
            let (actual, nodes) = compute(width, height, dpr);
            let expected = observation(&reference, width, height, dpr);
            for expected_node in expected["nodes"].as_array().unwrap() {
                let id = expected_node["id"].as_str().unwrap();
                let node_id = nodes[id].id();
                let actual_properties = &actual.properties[&node_id];
                for (property, expected_value) in expected_node["properties"].as_object().unwrap() {
                    if matches!(
                        (id, property.as_str()),
                        ("gap-a" | "gap-b" | "basis-item", "width")
                    ) {
                        // Chrome exposes post-flex used width while Stylo keeps auto width;
                        // compare the final layout frame and basis input instead.
                        continue;
                    }
                    if matches!((id, property.as_str()), ("basis-item", "height")) {
                        continue;
                    }
                    let actual_value = actual_properties
                        .get(property)
                        .unwrap_or_else(|| panic!("{id}.{property} computed value is missing"));
                    let actual_px = actual_value.strip_suffix("px").unwrap_or_else(|| {
                        panic!("DPR {dpr}, viewport {width}x{height}, {id}.{property}: unsupported actual `{actual_value}`")
                    });
                    let expected_value = expected_value.as_str().unwrap();
                    let expected_px = expected_value.strip_suffix("px").unwrap_or_else(|| {
                        panic!("DPR {dpr}, viewport {width}x{height}, {id}.{property}: unexpected reference `{expected_value}`")
                    });
                    assert!(
                        (actual_px.parse::<f32>().unwrap() - expected_px.parse::<f32>().unwrap())
                            .abs()
                            <= 0.02,
                        "DPR {dpr}, viewport {width}x{height}, {id}.{property}: {actual_value} != {expected_value}"
                    );
                }

                let frame = actual.frames.get(&node_id).unwrap();
                for (field, value, expected_value) in [
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
                        (value - expected_value).abs() <= 0.5,
                        "DPR {dpr}, viewport {width}x{height}, {id}.{field}: {value} != {expected_value}"
                    );
                }
            }

            if let Some(baseline) = &dpr_one {
                assert_eq!(
                    actual.properties, baseline.properties,
                    "CSS values must be DPR invariant"
                );
                assert_eq!(
                    actual.frames, baseline.frames,
                    "CSS frames must be DPR invariant"
                );
            } else {
                dpr_one = Some(actual);
            }
        }
    }
}

#[test]
fn viewport_environment_resize_changes_all_unit_families_without_dpr_scaling() {
    let (portrait, portrait_nodes) = compute(320.0, 800.0, 1.0);
    let (resized, resized_nodes) = compute(390.0, 844.0, 1.0);
    for id in [
        "root",
        "default-units",
        "small-units",
        "large-units",
        "dynamic-units",
        "logical-units",
    ] {
        let portrait_width = portrait.properties[&portrait_nodes[id].id()]["width"]
            .trim_end_matches("px")
            .parse::<f32>()
            .unwrap();
        let resized_width = resized.properties[&resized_nodes[id].id()]["width"]
            .trim_end_matches("px")
            .parse::<f32>()
            .unwrap();
        assert_ne!(
            portrait_width, resized_width,
            "{id} must follow resized CSS viewport"
        );
    }
    let portrait_vmax_height = portrait.properties[&portrait_nodes["minmax-units"].id()]["height"]
        .trim_end_matches("px")
        .parse::<f32>()
        .unwrap();
    let resized_vmax_height = resized.properties[&resized_nodes["minmax-units"].id()]["height"]
        .trim_end_matches("px")
        .parse::<f32>()
        .unwrap();
    assert_ne!(portrait_vmax_height, resized_vmax_height);
    let portrait_default = &portrait.properties[&portrait_nodes["default-units"].id()];
    assert!((css_px(&Value::String(portrait_default["width"].clone())) - 160.0).abs() <= 0.02);
}
