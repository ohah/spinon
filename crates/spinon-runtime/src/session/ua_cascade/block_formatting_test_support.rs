use super::{WorkRequest, calculation::RuntimeCalculation, key_for};
use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId,
};
use spinon_style::{CssMediaEnvironment, CssViewport};
use std::collections::BTreeMap;
use std::sync::Arc;

pub(super) const HTML: &str =
    include_str!("../../../../../tests/fixtures/css/c09/block-formatting.html");
pub(super) const INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c09/block-formatting-inventory.json");
pub(super) const REFERENCE: &str =
    include_str!("../../../../../tests/fixtures/css/references/c09-block-formatting-v2.json");
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub(super) const RECT_TOLERANCE_CSS_PX: f32 = 0.5;

#[derive(Clone)]
pub(super) struct FixtureNode {
    pub(super) id: String,
    pub(super) parent_id: Option<String>,
    pub(super) class_name: String,
    pub(super) inline_style: String,
}

pub(super) fn flatten_fixture_tree(tree: &Value, parent_id: Option<&str>) -> Vec<FixtureNode> {
    let id = tree["id"].as_str().unwrap();
    let (class_name, inline_style) = html_attributes_for_id(id);
    let mut nodes = vec![FixtureNode {
        id: id.to_owned(),
        parent_id: parent_id.map(str::to_owned),
        class_name,
        inline_style,
    }];
    for child in tree["children"].as_array().into_iter().flatten() {
        nodes.extend(flatten_fixture_tree(child, Some(id)));
    }
    nodes
}

pub(super) fn html_attributes_for_id(id: &str) -> (String, String) {
    let marker = format!("id=\"{id}\"");
    let marker_offset = HTML
        .find(&marker)
        .unwrap_or_else(|| panic!("HTML fixture node 없음: {id}"));
    let tag_start = HTML[..marker_offset].rfind("<div").unwrap();
    let tag_end = HTML[marker_offset..]
        .find('>')
        .map(|offset| marker_offset + offset)
        .unwrap();
    let opening_tag = &HTML[tag_start..=tag_end];
    (
        attribute(opening_tag, "class").unwrap_or_default(),
        attribute(opening_tag, "style").unwrap_or_default(),
    )
}

pub(super) fn attribute(tag: &str, name: &str) -> Option<String> {
    let marker = format!("{name}=\"");
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')? + start;
    Some(tag[start..end].to_owned())
}

pub(super) fn fixture_rule(selector: &str) -> String {
    let stylesheet = HTML
        .split_once("<style>")
        .and_then(|(_, tail)| tail.split_once("</style>"))
        .map(|(stylesheet, _)| stylesheet)
        .expect("C09 HTML fixture stylesheet 없음");
    let marker = format!("{selector} {{");
    let start = stylesheet
        .find(&marker)
        .unwrap_or_else(|| panic!("C09 HTML fixture selector 없음: {selector}"))
        + marker.len();
    let end = stylesheet[start..]
        .find('}')
        .map(|offset| start + offset)
        .expect("C09 HTML fixture CSS rule 닫힘 괄호 없음");
    stylesheet[start..end].trim().to_owned()
}

pub(super) fn make_request(
    fixtures: &[FixtureNode],
    device_scale_factor: f32,
) -> (WorkRequest, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(90 + device_scale_factor as u64).unwrap();
    let handles = (0..fixtures.len())
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let indexes = fixtures
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    let element_defaults = format!(
        "display:block;{};{}",
        fixture_rule("div"),
        fixture_rule("*, *::before, *::after")
    );

    for (fixture, handle) in fixtures.iter().zip(&handles) {
        let mut declarations = element_defaults.clone();
        for name in fixture.class_name.split_ascii_whitespace() {
            declarations.push(';');
            declarations.push_str(&fixture_rule(&format!(".{name}")));
        }
        if !fixture.inline_style.is_empty() {
            declarations.push(';');
            declarations.push_str(&fixture.inline_style);
        }

        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: fixture.id.clone().into(),
        });
        if !fixture.class_name.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node: *handle,
                name: AttributeName::new(None, "class").unwrap(),
                value: fixture.class_name.clone().into(),
            });
        }
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: declarations.into(),
        });
        let parent = fixture
            .parent_id
            .as_deref()
            .map_or(HostParent::Root, |parent_id| {
                HostParent::Node(handles[indexes[parent_id]])
            });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: *handle,
            before: None,
        });
    }

    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    let environment_revision = EnvironmentRevision::INITIAL;
    let key = key_for(&snapshot, environment_revision);
    (
        WorkRequest {
            key,
            snapshot,
            viewport: CssViewport {
                width_css_px: 320.0,
                height_css_px: 240.0,
                device_scale_factor,
                environment_revision,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        },
        handles,
    )
}

pub(super) fn assert_layout_matches_reference(
    case_id: &str,
    calculation: &RuntimeCalculation,
    handles: &[HostNodeHandle],
    expected_case: &Value,
) {
    let actual_frames = &calculation
        .layout
        .as_ref()
        .unwrap_or_else(|failure| panic!("{case_id} runtime layout 실패: {failure:?}"))
        .frames;
    let expected_nodes = expected_case["nodes"].as_array().unwrap();
    assert_eq!(
        actual_frames.len(),
        expected_nodes.len(),
        "{case_id} frame 수"
    );
    for (handle, expected) in handles.iter().zip(expected_nodes) {
        let node_id = expected["id"].as_str().unwrap();
        let actual = actual_frames
            .iter()
            .find(|frame| frame.node_id == handle.id().get())
            .unwrap_or_else(|| panic!("{case_id}/{node_id} runtime frame 없음"));
        for field in ["x", "y", "width", "height"] {
            let actual_value = match field {
                "x" => actual.x,
                "y" => actual.y,
                "width" => actual.width,
                "height" => actual.height,
                _ => unreachable!(),
            };
            let expected_value = expected["rect"][field].as_f64().unwrap() as f32;
            assert!(
                (actual_value - expected_value).abs() <= RECT_TOLERANCE_CSS_PX,
                "{case_id}/{node_id}.{field}: runtime={actual_value}, Chrome={expected_value}"
            );
        }
    }
}

pub(super) fn assert_styles_match_reference(
    case_id: &str,
    calculation: &RuntimeCalculation,
    handles: &[HostNodeHandle],
    expected_case: &Value,
) {
    let expected_nodes = expected_case["nodes"].as_array().unwrap();
    let styles = &calculation.roots[0].styles.elements;
    assert_eq!(
        styles.len(),
        expected_nodes.len(),
        "{case_id} computed style 수"
    );
    for (handle, expected) in handles.iter().zip(expected_nodes) {
        let node_id = expected["id"].as_str().unwrap();
        let actual = styles
            .iter()
            .find(|style| style.node_id == handle.id())
            .unwrap_or_else(|| panic!("{case_id}/{node_id} computed style 없음"));
        for property in [
            "display",
            "direction",
            "box-sizing",
            "border-top-style",
            "border-right-style",
            "border-bottom-style",
            "border-left-style",
        ] {
            let expected_value = expected["properties"][property]
                .as_str()
                .unwrap_or_else(|| panic!("{case_id}/{node_id}.{property} Chromium 값 없음"));
            assert_eq!(
                actual.properties.get(property).map(String::as_str),
                Some(expected_value),
                "{case_id}/{node_id}.{property} computed value"
            );
        }
        for property in [
            "width",
            "height",
            "margin-top",
            "margin-right",
            "margin-bottom",
            "margin-left",
            "padding-top",
            "padding-right",
            "padding-bottom",
            "padding-left",
        ] {
            let Some(expected_value) = expected["typed"][property]["text"].as_str() else {
                // Chrome의 getComputedStyle은 auto min/max를 used value로 보여 주지만
                // Typed OM은 값을 제공하지 않습니다. 이 경우 used geometry가 판정값입니다.
                continue;
            };
            assert_eq!(
                actual.properties.get(property).map(String::as_str),
                Some(expected_value),
                "{case_id}/{node_id}.{property} typed value"
            );
        }
        for property in [
            "border-top-width",
            "border-right-width",
            "border-bottom-width",
            "border-left-width",
        ] {
            let expected_value = expected["properties"][property]
                .as_str()
                .unwrap_or_else(|| panic!("{case_id}/{node_id}.{property} used value 없음"));
            assert_eq!(
                actual.properties.get(property).map(String::as_str),
                Some(expected_value),
                "{case_id}/{node_id}.{property} used value"
            );
        }
    }
}
