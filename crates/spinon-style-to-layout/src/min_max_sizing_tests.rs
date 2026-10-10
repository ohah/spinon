use std::{collections::BTreeMap, fs, sync::Arc};

use crate::compute_runtime_style_layout;
use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    ComputedCssDimension, ComputedCssMaxSize, CssMediaEnvironment, CssOrigin, CssViewport,
    StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const FIXTURE_HTML: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c07/min-max-sizing.html"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c07-min-max-sizing-v1.json"
);
const TREE: &[(&str, Option<&str>)] = &[
    ("root", None),
    ("block-parent", Some("root")),
    ("percentage-height-parent", Some("root")),
    ("min-width-raises", Some("block-parent")),
    ("max-width-limits", Some("block-parent")),
    ("min-height-raises", Some("block-parent")),
    ("max-height-limits", Some("block-parent")),
    ("min-over-max", Some("block-parent")),
    ("content-box-max", Some("block-parent")),
    ("border-box-max", Some("block-parent")),
    ("border-box-min", Some("block-parent")),
    ("border-box-padding-floor", Some("block-parent")),
    ("percentage-min", Some("block-parent")),
    ("percentage-max", Some("block-parent")),
    ("calc-min", Some("block-parent")),
    ("calc-max", Some("block-parent")),
    ("var-min", Some("block-parent")),
    ("zero-min", Some("block-parent")),
    ("zero-max", Some("block-parent")),
    ("invalid-min-fallback", Some("block-parent")),
    ("invalid-max-fallback", Some("block-parent")),
    ("default-auto-none", Some("block-parent")),
    ("flex-min-parent", Some("block-parent")),
    ("flex-min-a", Some("flex-min-parent")),
    ("flex-min-b", Some("flex-min-parent")),
    ("flex-zero-parent", Some("block-parent")),
    ("flex-zero-a", Some("flex-zero-parent")),
    ("flex-zero-b", Some("flex-zero-parent")),
    ("flex-max-parent", Some("block-parent")),
    ("flex-max-a", Some("flex-max-parent")),
    ("flex-max-b", Some("flex-max-parent")),
    ("flex-auto-parent", Some("block-parent")),
    ("flex-auto-a", Some("flex-auto-parent")),
    ("flex-auto-b", Some("flex-auto-parent")),
    ("percentage-height-max", Some("percentage-height-parent")),
];

struct Baseline {
    frames: BTreeMap<NodeId, LayoutFrame>,
    dimensions: BTreeMap<NodeId, spinon_style::ComputedLayoutDimensions>,
    max_sizes: BTreeMap<NodeId, (ComputedCssMaxSize, ComputedCssMaxSize)>,
    math: BTreeMap<NodeId, BTreeMap<String, spinon_style::ComputedCssMath>>,
}

fn document_fixture() -> (
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
        DocumentChangeBatch::new(OwnerId::new(7707).unwrap(), document.document_revision());
    for (id, parent_id) in TREE {
        let node = nodes[*id];
        batch
            .push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: if *id == "root" {
                    "main"
                } else if id.ends_with("parent") {
                    "section"
                } else {
                    "div"
                }
                .to_owned(),
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
    let (_, rest) = html
        .split_once("<style>")
        .expect("fixture must have one style block");
    let (css, _) = rest
        .split_once("</style>")
        .expect("fixture style block must close");
    StylesheetSource {
        id: "c07-1-min-max-sizing".to_owned(),
        base_url: "https://spinon.invalid/c07/min-max-sizing.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn number(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

fn compute(dpr: f32) -> (Baseline, BTreeMap<String, HostNodeHandle>) {
    let (document, root, nodes) = document_fixture();
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
        &[stylesheet()],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    let dimensions = output
        .computed_styles
        .elements
        .iter()
        .map(|style| (style.node_id, style.layout_dimensions))
        .collect();
    let max_sizes = output
        .computed_styles
        .elements
        .iter()
        .map(|style| {
            (
                style.node_id,
                (
                    style.layout_dimensions.max_width,
                    style.layout_dimensions.max_height,
                ),
            )
        })
        .collect();
    let math = output
        .computed_styles
        .elements
        .iter()
        .map(|style| (style.node_id, style.layout_math_values.clone()))
        .collect();
    (
        Baseline {
            frames: output.layout.frames,
            dimensions,
            max_sizes,
            math,
        },
        nodes,
    )
}

#[test]
fn min_max_values_and_layout_frames_match_pinned_chromium_for_each_node_and_dpr() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(reference["fixture"]["id"], "C07.1-min-max-size-v1");
    assert_eq!(TREE.len(), 35);

    let mut dpr_one: Option<Baseline> = None;
    for dpr in [1.0, 2.0] {
        let (actual, nodes) = compute(dpr);
        let expected = reference["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|observation| number(&observation["viewport"]["deviceScaleFactor"]) == dpr)
            .unwrap();
        let mut max_geometry_error = 0.0_f32;
        let mut max_geometry_case = String::new();
        for expected_node in expected["nodes"].as_array().unwrap() {
            let id = expected_node["id"].as_str().unwrap();
            let node = nodes[id].id();
            let frame = actual
                .frames
                .get(&node)
                .unwrap_or_else(|| panic!("{id} frame missing"));
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
                let error = (value - expected_value).abs();
                if error > max_geometry_error {
                    max_geometry_error = error;
                    max_geometry_case = format!("{id}.{field}");
                }
                assert!(
                    error <= 0.5,
                    "DPR {dpr}, {id}.{field}: {value} != {expected_value}"
                );
            }
        }
        println!(
            "DPR {dpr}: 35개 노드 최대 절대 frame 오차 {max_geometry_error} CSS px ({max_geometry_case})"
        );

        for (id, axis, expected_size) in [
            ("min-width-raises", "width", 90.0),
            ("max-width-limits", "width", 90.0),
            ("min-over-max", "width", 80.0),
            ("content-box-max", "width", 120.0),
            ("border-box-max", "width", 100.0),
            ("border-box-min", "width", 100.0),
            ("border-box-padding-floor", "width", 24.0),
            ("percentage-min", "width", 100.0),
            ("percentage-max", "width", 150.0),
            ("calc-min", "width", 50.0),
            ("calc-max", "width", 80.0),
            ("var-min", "width", 60.0),
            ("flex-min-a", "width", 80.0),
            ("flex-zero-a", "width", 75.0),
            ("flex-max-a", "width", 60.0),
            ("flex-auto-a", "width", 50.0),
            ("percentage-height-max", "height", 60.0),
        ] {
            let frame = actual.frames[&nodes[id].id()];
            let size = match axis {
                "width" => frame.width,
                "height" => frame.height,
                _ => unreachable!("known axis"),
            };
            assert!(
                (size - expected_size).abs() <= 0.5,
                "DPR {dpr}, {id}.{axis}: {size}"
            );
        }
        assert_eq!(
            actual.dimensions[&nodes["default-auto-none"].id()].min_width,
            ComputedCssDimension::Auto
        );
        assert_eq!(
            actual.max_sizes[&nodes["default-auto-none"].id()].0,
            ComputedCssMaxSize::None
        );
        assert_eq!(
            actual.max_sizes[&nodes["default-auto-none"].id()].1,
            ComputedCssMaxSize::None
        );
        assert_eq!(
            actual.dimensions[&nodes["flex-auto-a"].id()].min_width,
            ComputedCssDimension::Auto
        );
        assert_eq!(
            actual.dimensions[&nodes["min-width-raises"].id()].min_width,
            ComputedCssDimension::LengthPx(90.0)
        );
        assert_eq!(
            actual.max_sizes[&nodes["max-width-limits"].id()].0,
            ComputedCssMaxSize::LengthPx(90.0)
        );
        assert_eq!(
            actual.dimensions[&nodes["min-height-raises"].id()].min_height,
            ComputedCssDimension::LengthPx(55.0)
        );
        assert_eq!(
            actual.max_sizes[&nodes["max-height-limits"].id()].1,
            ComputedCssMaxSize::LengthPx(55.0)
        );
        assert_eq!(
            actual.dimensions[&nodes["percentage-min"].id()].min_width,
            ComputedCssDimension::Percentage(0.5)
        );
        assert_eq!(
            actual.max_sizes[&nodes["percentage-max"].id()].0,
            ComputedCssMaxSize::Percentage(0.75)
        );
        assert!(actual.math[&nodes["calc-min"].id()].contains_key("min-width"));
        assert!(actual.math[&nodes["calc-max"].id()].contains_key("max-width"));
        assert!(actual.math[&nodes["var-min"].id()].contains_key("min-width"));

        if let Some(first) = &dpr_one {
            assert_eq!(actual.frames, first.frames, "DPR must not alter CSS frames");
            assert_eq!(
                actual.dimensions, first.dimensions,
                "DPR must not alter typed sizes"
            );
            assert_eq!(
                actual.max_sizes, first.max_sizes,
                "DPR must not alter max sizes"
            );
            assert_eq!(
                actual.math, first.math,
                "DPR must not alter calc expressions"
            );
        } else {
            dpr_one = Some(actual);
        }
    }
}

#[test]
fn unsupported_intrinsic_min_max_keywords_fail_closed_with_property_context() {
    for (property, keyword) in [
        ("min-width", "min-content"),
        ("max-width", "max-content"),
        ("min-height", "fit-content"),
    ] {
        let (document, root, nodes) = document_fixture();
        let snapshot = document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root)
                .unwrap();
        let viewport = CssViewport {
            width_css_px: 400.0,
            height_css_px: 1000.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let mut override_sheet = stylesheet();
        override_sheet.id = format!("c07-1-unsupported-{property}");
        override_sheet.css = format!("#default-auto-none {{ {property}: {keyword}; }}");
        let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            &[stylesheet(), override_sheet],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap();
        assert!(matches!(
            compute_runtime_style_layout(&snapshot, root, computed, viewport),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node,
                property: actual_property,
                ..
            }) if node == nodes["default-auto-none"].id() && actual_property == property
        ));
    }
}
