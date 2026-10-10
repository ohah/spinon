use std::{collections::BTreeMap, fs};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-spacing-percentages-v1.json"
);
const HTML_FIXTURE: &str =
    include_str!("../../../../tests/fixtures/css/c06/spacing-percentages.html");

pub(super) fn fixture_reference() -> Value {
    let value: Value = serde_json::from_str(&fs::read_to_string(REFERENCE).unwrap()).unwrap();
    assert_eq!(
        value["schema"],
        "spinon-css-c06-spacing-percentages-reference/v1"
    );
    assert_eq!(value["fixture"]["id"], "C06.2-spacing-percentages-v1");
    assert_eq!(value["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(value["observation"]["nodes"].as_array().unwrap().len(), 78);
    value
}

pub(super) fn reference_node(reference: &Value, id: &str) -> Value {
    reference["observation"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["id"] == id)
        .unwrap_or_else(|| panic!("Chromium 기준 노드 {id}가 없습니다"))
        .clone()
}

pub(super) fn assert_relative_frame(
    actual: &spinon_layout::LayoutFrame,
    actual_parent: &spinon_layout::LayoutFrame,
    expected: &Value,
    expected_parent: &Value,
    id: &str,
) {
    let comparisons = [
        (
            "x",
            actual.x - actual_parent.x,
            expected["x"].as_f64().unwrap() as f32 - expected_parent["x"].as_f64().unwrap() as f32,
        ),
        (
            "y",
            actual.y - actual_parent.y,
            expected["y"].as_f64().unwrap() as f32 - expected_parent["y"].as_f64().unwrap() as f32,
        ),
        (
            "width",
            actual.width,
            expected["width"].as_f64().unwrap() as f32,
        ),
        (
            "height",
            actual.height,
            expected["height"].as_f64().unwrap() as f32,
        ),
    ];
    for (field, actual, expected) in comparisons {
        assert!(
            (actual - expected).abs() <= 0.5,
            "{id}.{field}: Spinon {actual}, Chromium {expected}, 허용치 0.5 CSS px"
        );
    }
}

pub(super) fn assert_absolute_frame(
    actual: &spinon_layout::LayoutFrame,
    expected: &Value,
    id: &str,
) -> f32 {
    let mut maximum_error = 0.0_f32;
    for (field, value) in [
        ("x", actual.x),
        ("y", actual.y),
        ("width", actual.width),
        ("height", actual.height),
    ] {
        let reference = expected[field].as_f64().unwrap() as f32;
        let error = (value - reference).abs();
        assert!(
            error <= 0.5,
            "{id}.{field}: 실제 {value}, Chromium {reference}, 허용치 0.5 CSS px"
        );
        maximum_error = maximum_error.max(error);
    }
    maximum_error
}

pub(super) struct RuntimeSpacingFixture {
    document: HostDocument,
    root: HostNodeHandle,
    pub(super) nodes: BTreeMap<String, HostNodeHandle>,
}

pub(super) struct FullChromiumSpacingFixture {
    document: HostDocument,
    root: HostNodeHandle,
    pub(super) nodes: BTreeMap<String, HostNodeHandle>,
    stylesheet: StylesheetSource,
}

impl FullChromiumSpacingFixture {
    pub(super) fn new() -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(7622).unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut nodes = BTreeMap::from([("root".to_owned(), root)]);
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        let fixture_root = HTML_FIXTURE.find("<div id=\"root\"").unwrap();
        let markup = &HTML_FIXTURE[fixture_root..];
        let mut stack = Vec::<Option<HostNodeHandle>>::new();
        let mut cursor = 0;
        let excluded = ["m-invalid-auto", "g-auto-row-inline"];

        while let Some(relative_start) = markup[cursor..].find('<') {
            let start = cursor + relative_start;
            let Some(relative_end) = markup[start..].find('>') else {
                break;
            };
            let end = start + relative_end;
            let tag = markup[start..=end].trim();
            cursor = end + 1;
            if tag.starts_with("</div") {
                stack.pop();
                if stack.is_empty() {
                    break;
                }
                continue;
            }
            if !tag.starts_with("<div ") && !tag.starts_with("<div>") {
                continue;
            }

            let id = html_attribute(tag, "id").expect("fixture div에는 id가 있어야 합니다");
            let parent_is_included = stack.last().is_none_or(Option::is_some);
            if !parent_is_included || excluded.contains(&id.as_str()) {
                stack.push(None);
                continue;
            }
            let node = if id == "root" {
                root
            } else {
                document.reserve_node_handle().unwrap()
            };
            if id != "root" {
                nodes.insert(id.clone(), node);
            }
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.clone().into(),
            });
            for attribute in ["class", "style"] {
                if let Some(value) = html_attribute(tag, attribute) {
                    batch.push(DocumentOperation::SetAttribute {
                        node,
                        name: AttributeName::new(None, attribute).unwrap(),
                        value: value.into(),
                    });
                }
            }
            if id == "g-auto-column" {
                // Keep later root-level oracle coordinates stable; its original cyclic gap is
                // tested as a separate expected-error input below.
                batch.push(DocumentOperation::SetAttribute {
                    node,
                    name: AttributeName::new(None, "style").unwrap(),
                    value: "height:20px;row-gap:0px".to_owned().into(),
                });
            }
            let parent = stack.last().and_then(|parent| *parent);
            batch.push(DocumentOperation::InsertBefore {
                parent: parent.map_or(HostParent::Root, HostParent::Node),
                node,
                before: None,
            });
            stack.push(Some(node));
        }
        document.commit(batch).unwrap();

        let style_start = HTML_FIXTURE.find("<style>").unwrap() + "<style>".len();
        let style_end = style_start + HTML_FIXTURE[style_start..].find("</style>").unwrap();
        let stylesheet = StylesheetSource {
            id: "c06-2-spacing-percentages-oracle".to_owned(),
            base_url: "https://spinon.invalid/c06/spacing-percentages.css".to_owned(),
            origin: CssOrigin::Author,
            css: HTML_FIXTURE[style_start..style_end].to_owned(),
        };
        Self {
            document,
            root,
            nodes,
            stylesheet,
        }
    }

    pub(super) fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let viewport = CssViewport {
            width_css_px: 320.0,
            height_css_px: 800.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new_with_base_url(
            snapshot.clone(),
            self.root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c06/spacing-percentages.html",
        )
        .unwrap();
        let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            std::slice::from_ref(&self.stylesheet),
            viewport,
            StyleRevision::default(),
        )?;
        compute_runtime_style_layout(&snapshot, self.root, computed, viewport)
    }
}

fn html_attribute(tag: &str, attribute: &str) -> Option<String> {
    let needle = format!("{attribute}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = start + tag[start..].find('"')?;
    Some(tag[start..end].to_owned())
}

impl RuntimeSpacingFixture {
    pub(super) fn new() -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(7621).unwrap();
        let specifications = [
            (
                "root",
                None,
                "display:block;box-sizing:border-box;width:100%;height:100%",
            ),
            (
                "margin-cb",
                Some("root"),
                "display:flex;flex-direction:column;width:200px;height:120px",
            ),
            (
                "m-physical",
                Some("margin-cb"),
                "flex:0 0 auto;width:20px;height:20px;margin:10%",
            ),
            (
                "m-negative",
                Some("margin-cb"),
                "flex:0 0 auto;width:20px;height:20px;margin:-5%",
            ),
            (
                "padding-cb",
                Some("root"),
                "display:flex;flex-direction:column;width:200px;height:120px",
            ),
            (
                "p-physical",
                Some("padding-cb"),
                "flex:0 0 auto;box-sizing:content-box;width:20px;height:10px;padding:10%",
            ),
            (
                "g-row",
                Some("root"),
                "display:flex;box-sizing:border-box;width:260px;height:180px;padding:30px;column-gap:10%;row-gap:25%",
            ),
            ("g-row-a", Some("g-row"), "flex:0 0 20px;height:10px"),
            ("g-row-b", Some("g-row"), "flex:0 0 20px;height:10px"),
            (
                "g-column",
                Some("root"),
                "display:flex;flex-direction:column;box-sizing:border-box;width:260px;height:180px;padding:30px;column-gap:10%;row-gap:25%",
            ),
            ("g-column-a", Some("g-column"), "flex:0 0 20px;height:10px"),
            ("g-column-b", Some("g-column"), "flex:0 0 20px;height:10px"),
            (
                "g-auto-width-row",
                Some("root"),
                "display:flex;height:40px;column-gap:10%",
            ),
            (
                "g-auto-width-row-a",
                Some("g-auto-width-row"),
                "flex:0 0 20px;height:10px",
            ),
            (
                "g-auto-width-row-b",
                Some("g-auto-width-row"),
                "flex:0 0 20px;height:10px",
            ),
            (
                "g-stretch-parent",
                Some("root"),
                "display:flex;width:200px;height:120px",
            ),
            (
                "g-stretched-column",
                Some("g-stretch-parent"),
                "display:flex;flex-direction:column;width:100px;height:auto;row-gap:10%",
            ),
            (
                "g-stretched-column-a",
                Some("g-stretched-column"),
                "flex:0 0 10px",
            ),
            (
                "g-stretched-column-b",
                Some("g-stretched-column"),
                "flex:0 0 10px",
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

    pub(super) fn set_style(&mut self, id: &str, style: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(7621).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: self.nodes[id],
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        self.document.commit(batch).unwrap();
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
            "https://spinon.invalid/c06/spacing-percentages.html",
        )
        .unwrap()
    }

    pub(super) fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let viewport = self.viewport();
        let snapshot = self.document.snapshot();
        let computed =
            compute_runtime_flex_layout_cascade(&self.view(), viewport, StyleRevision::default())?;
        compute_runtime_style_layout(&snapshot, self.root, computed, viewport)
    }

    pub(super) fn compute_custom_properties(
        &self,
    ) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let viewport = self.viewport();
        let snapshot = self.document.snapshot();
        let computed = spinon_style::compute_runtime_flex_custom_properties_cascade(
            &self.view(),
            viewport,
            StyleRevision::default(),
        )?;
        compute_runtime_style_layout(&snapshot, self.root, computed, viewport)
    }

    pub(super) fn computed_style<'a>(
        &self,
        output: &'a crate::StyleLayoutOutput,
        id: &str,
    ) -> &'a spinon_style::ComputedElementStyle {
        output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == self.nodes[id].id())
            .unwrap_or_else(|| panic!("계산 스타일 {id}가 없습니다"))
    }
}
