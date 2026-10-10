use std::{collections::BTreeMap, fs};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    ComputedCssDimension, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_layout_cascade,
};
use style::context::QuirksMode;

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-percentage-dimensions-v1.json"
);

#[test]
fn typed_percentages_match_chromium_geometry_for_supported_runtime_nodes() {
    let fixture = RuntimePercentageFixture::new();
    let viewport = fixture.viewport();
    let document_snapshot = fixture.document.snapshot();
    let view = fixture.view();
    let computed =
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::default()).unwrap();
    let output =
        compute_runtime_style_layout(&document_snapshot, fixture.root, computed, viewport).unwrap();
    let reference = load_reference();
    let reference_nodes = reference["observation"]["nodes"].as_array().unwrap();
    let supported_ids = [
        "root",
        "root-half",
        "content-parent",
        "content-50",
        "content-125",
        "content-zero",
        "content-fraction",
        "content-negative",
        "border-parent",
        "border-50",
        "row-flex",
        "row-50",
        "row-25",
        "column-flex",
        "column-50",
        "column-25",
        "auto-parent",
        "auto-height-child",
    ];

    for id in supported_ids {
        let expected = reference_nodes
            .iter()
            .find(|node| node["id"] == id)
            .unwrap_or_else(|| panic!("Chromium 기준 노드 {id}가 없습니다"));
        let node_id = fixture.nodes[id].id();
        let actual_style = output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == node_id)
            .unwrap_or_else(|| panic!("계산 스타일 {id}가 없습니다"));
        for property in expected["properties"].as_object().unwrap().keys() {
            if matches!(property.as_str(), "width" | "height" | "flex-basis") {
                continue;
            }
            assert_eq!(
                actual_style.properties.get(property).map(String::as_str),
                expected["properties"][property].as_str(),
                "{id}.{property} computed-style 직렬화"
            );
        }
        assert_frame_matches(&output.layout.frames[&node_id], &expected["rect"], id);
    }

    let actual_dimensions = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.nodes["content-125"].id())
        .unwrap()
        .layout_dimensions;
    assert_eq!(
        actual_dimensions.width,
        ComputedCssDimension::Percentage(1.25)
    );
    for id in [
        "root",
        "root-half",
        "content-50",
        "content-zero",
        "border-50",
    ] {
        let dimensions = fixture.computed_dimensions_for(&output, id);
        let expected = if id == "root" {
            1.0
        } else if id == "content-zero" {
            0.0
        } else {
            0.5
        };
        assert_eq!(
            dimensions.width,
            ComputedCssDimension::Percentage(expected),
            "{id}.width"
        );
        assert_eq!(
            dimensions.height,
            ComputedCssDimension::Percentage(expected),
            "{id}.height"
        );
    }
    assert_eq!(
        fixture
            .computed_dimensions_for(&output, "row-50")
            .flex_basis,
        ComputedCssDimension::Percentage(0.5)
    );
    assert_eq!(
        fixture
            .computed_dimensions_for(&output, "column-25")
            .flex_basis,
        ComputedCssDimension::Percentage(0.25)
    );
    assert_eq!(
        fixture
            .computed_dimensions_for(&output, "content-negative")
            .width,
        ComputedCssDimension::Auto
    );
    assert_eq!(
        fixture
            .computed_dimensions_for(&output, "auto-height-child")
            .height,
        ComputedCssDimension::Percentage(0.5)
    );

    assert_eq!(
        output.layout.revision.source(),
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: document_snapshot.generation(),
            document: document_snapshot.document_revision(),
            render_tree: document_snapshot.render_tree_revision(),
        }
    );
    assert_eq!(output.layout.revision.style(), StyleRevision::default());
    assert_eq!(
        output.layout.revision.environment(),
        EnvironmentRevision::default()
    );
}

#[test]
fn calc_dimension_keeps_typed_basis_until_taffy_resolution() {
    let mut fixture = RuntimePercentageFixture::new();
    fixture.set_style("content-50", "width:calc(50% + 10px);height:50%");
    let output = fixture.compute().unwrap();
    let node = fixture.nodes["content-50"];
    let node_id = node.id();
    let frame = &output.layout.frames[&node_id];
    assert_eq!(frame.width, 60.0);
    assert_eq!(frame.height, 40.0);
}

#[test]
fn percentage_margin_padding_and_gap_are_preserved_when_basis_is_definite() {
    for (id, declaration) in [
        ("content-50", "width:10px;height:10px;margin-left:10%"),
        ("content-50", "width:10px;height:10px;padding-left:10%"),
        (
            "content-50",
            "display:flex;width:100px;height:50px;column-gap:10%",
        ),
    ] {
        let mut fixture = RuntimePercentageFixture::new();
        fixture.set_style(id, declaration);
        let result = fixture.compute();
        assert!(
            result.is_ok(),
            "definite percentage spacing rejected: {result:?}"
        );
    }
}

#[test]
fn unsupported_spacing_still_fails_closed() {
    let mut fixture = RuntimePercentageFixture::new();
    fixture.set_style("content-50", "width:10px;height:10px;margin-left:auto");
    let result = fixture.compute();
    assert!(
        matches!(
            result,
            Err(crate::StyleLayoutError::UnsupportedComputedValue { .. })
        ),
        "margin-left:auto must fail closed: {result:?}"
    );
}

#[test]
fn mixed_unit_spacing_resolves_after_definite_basis_is_known() {
    let mut fixture = RuntimePercentageFixture::new();
    fixture.set_style(
        "content-50",
        "width:10px;height:10px;margin-left:calc(10% + 2px)",
    );
    let output = fixture.compute().unwrap();
    let node = fixture.nodes["content-50"];
    let node_id = node.id();
    assert_eq!(output.layout.frames[&node_id].x, 22.0);
}

fn assert_frame_matches(actual: &LayoutFrame, expected: &Value, id: &str) {
    for (field, value) in [
        ("x", actual.x),
        ("y", actual.y),
        ("width", actual.width),
        ("height", actual.height),
    ] {
        let reference = expected[field].as_f64().unwrap() as f32;
        assert!(
            (value - reference).abs() <= 0.5,
            "{id}.{field}: 실제 {value}, Chromium {reference}, 허용치 0.5 CSS px"
        );
    }
}

fn load_reference() -> Value {
    let reference: Value = serde_json::from_str(&fs::read_to_string(REFERENCE).unwrap()).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-css-c06-percentage-dimensions-reference/v1"
    );
    assert_eq!(reference["fixture"]["id"], "C06.1-percentage-dimensions-v1");
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(
        reference["observation"]["nodes"].as_array().unwrap().len(),
        22
    );
    reference
}

struct RuntimePercentageFixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl RuntimePercentageFixture {
    fn new() -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(7601).unwrap();
        let specifications = [
            (
                "root",
                None,
                "display:block;box-sizing:border-box;width:100%;height:100%",
            ),
            ("root-half", Some("root"), "width:50%;height:50%"),
            (
                "content-parent",
                Some("root"),
                "display:block;box-sizing:content-box;width:100px;height:80px;padding:10px",
            ),
            ("content-50", Some("content-parent"), "width:50%;height:50%"),
            (
                "content-125",
                Some("content-parent"),
                "width:125%;height:125%",
            ),
            ("content-zero", Some("content-parent"), "width:0%;height:0%"),
            (
                "content-fraction",
                Some("content-parent"),
                "width:33.333%;height:33.333%",
            ),
            (
                "content-negative",
                Some("content-parent"),
                "width:-10%;height:-10%",
            ),
            (
                "border-parent",
                Some("root"),
                "display:block;box-sizing:border-box;width:124px;height:104px;padding:10px",
            ),
            ("border-50", Some("border-parent"), "width:50%;height:50%"),
            (
                "row-flex",
                Some("root"),
                "display:flex;width:120px;height:40px;flex-direction:row",
            ),
            ("row-50", Some("row-flex"), "flex:0 0 50%;width:20px"),
            ("row-25", Some("row-flex"), "flex:0 0 25%;width:20px"),
            (
                "column-flex",
                Some("root"),
                "display:flex;flex-direction:column;width:80px;height:60px",
            ),
            ("column-50", Some("column-flex"), "flex:0 0 50%;height:10px"),
            ("column-25", Some("column-flex"), "flex:0 0 25%;height:10px"),
            ("auto-parent", Some("root"), "display:block;width:100px"),
            (
                "auto-height-child",
                Some("auto-parent"),
                "width:50%;height:50%",
            ),
        ];
        let root = document.reserve_node_handle().unwrap();
        let mut nodes = BTreeMap::from([("root".to_owned(), root)]);
        for (id, _, _) in specifications.iter().skip(1) {
            nodes.insert(id.to_string(), document.reserve_node_handle().unwrap());
        }
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        for (id, parent, style) in specifications {
            let node = nodes[id];
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
                parent: parent.map_or(HostParent::Root, |parent| HostParent::Node(nodes[parent])),
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn viewport(&self) -> CssViewport {
        CssViewport {
            width_css_px: 320.0,
            height_css_px: 800.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        }
    }

    fn view(&self) -> StyloDocumentView {
        StyloDocumentView::new_with_base_url(
            self.document.snapshot(),
            self.root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c06/percentage-dimensions.html",
        )
        .unwrap()
    }

    fn computed_dimensions_for(
        &self,
        output: &crate::StyleLayoutOutput,
        id: &str,
    ) -> spinon_style::ComputedLayoutDimensions {
        output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == self.nodes[id].id())
            .unwrap_or_else(|| panic!("계산 스타일 {id}가 없습니다"))
            .layout_dimensions
    }

    fn set_style(&mut self, id: &str, style: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(7601).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: self.nodes[id],
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        self.document.commit(batch).unwrap();
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, crate::StyleLayoutError> {
        let viewport = self.viewport();
        let snapshot = self.document.snapshot();
        let computed =
            compute_runtime_flex_layout_cascade(&self.view(), viewport, StyleRevision::default())?;
        compute_runtime_style_layout(&snapshot, self.root, computed, viewport)
    }
}
