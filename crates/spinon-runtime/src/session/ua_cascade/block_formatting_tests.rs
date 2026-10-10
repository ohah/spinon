use super::*;
use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use std::collections::BTreeMap;

const HTML: &str = include_str!("../../../../../tests/fixtures/css/c09/block-formatting.html");
const INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c09/block-formatting-inventory.json");
const REFERENCE: &str =
    include_str!("../../../../../tests/fixtures/css/references/c09-block-formatting-v1.json");
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const RECT_TOLERANCE_CSS_PX: f32 = 0.5;

#[derive(Clone)]
struct FixtureNode {
    id: String,
    parent_id: Option<String>,
    class_name: String,
    inline_style: String,
}

#[test]
fn c091_block_flow_and_containing_block_match_pinned_chromium_for_all_cases() {
    let inventory: Value = serde_json::from_str(INVENTORY).unwrap();
    let reference: Value = serde_json::from_str(REFERENCE).unwrap();
    let inventory_cases = inventory["cases"].as_array().unwrap();
    let reference_cases = reference["observations"][0]["cases"].as_array().unwrap();
    let mut tested_case_count = 0;
    let mut tested_node_count = 0;

    for case in inventory_cases
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("c091-"))
    {
        let case_id = case["id"].as_str().unwrap();
        let expected = reference_cases
            .iter()
            .find(|candidate| candidate["id"] == case_id)
            .unwrap_or_else(|| panic!("Chromium reference에 case가 없습니다: {case_id}"));
        let fixtures = flatten_fixture_tree(&case["tree"], None);
        let expected_nodes = expected["nodes"].as_array().unwrap();
        assert_eq!(fixtures.len(), expected_nodes.len(), "{case_id} node 수");

        for (fixture, expected_node) in fixtures.iter().zip(expected_nodes) {
            assert_eq!(
                fixture.id,
                expected_node["id"].as_str().unwrap(),
                "{case_id} preorder"
            );
            assert_eq!(
                fixture.parent_id.as_deref(),
                expected_node["parentId"].as_str()
            );
        }

        let (request, handles) = make_request(&fixtures, 1.0);
        let first = compute_request_for_block_formatting(&request)
            .unwrap_or_else(|error| panic!("{case_id} cascade 실패: {error}"));
        assert_layout_matches_reference(case_id, &first, &handles, expected);
        assert_styles_match_reference(case_id, &first, &handles, expected);

        let (scaled_request, scaled_handles) = make_request(&fixtures, 2.0);
        let scaled = compute_request_for_block_formatting(&scaled_request)
            .unwrap_or_else(|error| panic!("{case_id} DPR 2 cascade 실패: {error}"));
        assert_layout_matches_reference(case_id, &scaled, &scaled_handles, expected);
        assert_styles_match_reference(case_id, &scaled, &scaled_handles, expected);
        assert_eq!(
            first.layout.as_ref().unwrap().frames,
            scaled.layout.as_ref().unwrap().frames
        );
        assert_eq!(
            first.roots[0].styles.elements, scaled.roots[0].styles.elements,
            "{case_id}: DPR 변경으로 computed style이 달라졌습니다"
        );

        tested_case_count += 1;
        tested_node_count += fixtures.len();
    }

    assert_eq!(tested_case_count, 10);
    assert_eq!(tested_node_count, 30);
}

#[test]
fn c091_root_auto_margins_are_resolved_inside_the_viewport_containing_block() {
    let fixture = FixtureNode {
        id: "c091-root-auto-margins".to_owned(),
        parent_id: None,
        class_name: "root".to_owned(),
        inline_style:
            "width:100px;height:10px;margin-left:auto;margin-right:auto;background-color:#3366ff"
                .to_owned(),
    };
    let (request, handles) = make_request(&[fixture], 1.0);
    let calculation = compute_request_for_block_formatting(&request).unwrap();
    let frame = calculation
        .layout
        .as_ref()
        .unwrap()
        .frames
        .iter()
        .find(|frame| frame.node_id == handles[0].id().get())
        .unwrap();

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (110.0, 0.0, 100.0, 10.0)
    );
}

#[test]
fn c091_profile_rejects_other_formatting_models_and_unimplemented_positioning() {
    for (style, expected_code, expected_property) in [
        ("display:flex", "unsupported_block_display", Some("flex")),
        ("display:grid", "unsupported_block_display", Some("grid")),
        (
            "display:inline",
            "unsupported_block_display",
            Some("inline"),
        ),
        ("display:table", "unsupported_block_display", Some("table")),
        (
            "display:flow-root",
            "unsupported_block_display",
            Some("flow-root"),
        ),
        (
            "direction:rtl",
            "unsupported_inline_property",
            Some("direction"),
        ),
        (
            "position:absolute",
            "unsupported_inline_property",
            Some("position"),
        ),
        ("float:left", "unsupported_inline_property", Some("float")),
        ("clear:both", "unsupported_inline_property", Some("clear")),
        (
            "overflow:hidden",
            "unsupported_inline_property",
            Some("overflow-x"),
        ),
        (
            "transform:translateX(1px)",
            "unsupported_inline_property",
            Some("transform"),
        ),
    ] {
        let root = FixtureNode {
            id: "c091-unsupported-root".to_owned(),
            parent_id: None,
            class_name: "root".to_owned(),
            inline_style: String::new(),
        };
        let child = FixtureNode {
            id: "c091-unsupported".to_owned(),
            parent_id: Some("c091-unsupported-root".to_owned()),
            class_name: "box".to_owned(),
            inline_style: style.to_owned(),
        };
        let (request, _) = make_request(&[root, child], 1.0);
        let calculation = compute_request_for_block_formatting(&request).unwrap();
        let computed_display = calculation.roots[0].styles.elements[1]
            .properties
            .get("display")
            .cloned();
        let failure = match calculation.layout {
            Err(failure) => failure,
            Ok(layout) => panic!(
                "{style}는 실패해야 하지만 성공했습니다: computed display={computed_display:?}, {layout:?}"
            ),
        };
        assert_eq!(failure.code, expected_code, "{style}");
        assert_eq!(failure.property.as_deref(), expected_property, "{style}");
    }

    let (request, _) = super::block_paint_tests::block_request_with_author_stylesheet(
        ".painted { display: grid; }",
    );
    let calculation = compute_request_for_block_formatting(&request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "unsupported_block_stylesheet");
    assert_eq!(failure.node_id, None);
    assert!(
        failure
            .property
            .as_deref()
            .is_some_and(|value| value.starts_with("host-style:") && value.ends_with(": grid"))
    );

    let fixture = FixtureNode {
        id: "c091-unsupported-writing-mode".to_owned(),
        parent_id: None,
        class_name: "root".to_owned(),
        inline_style: "writing-mode:vertical-rl".to_owned(),
    };
    let (request, handles) = make_request(&[fixture], 1.0);
    let calculation = compute_request_for_block_formatting(&request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "cascade_diagnostics");
    assert!(
        calculation.layout_diagnostics.iter().any(|diagnostic| {
            diagnostic.node_id == Some(handles[0].id())
                && diagnostic.diagnostic.message.contains("writing-mode")
        }),
        "Stylo 진단에서 원인 node와 속성 이름을 유지해야 합니다"
    );
}

fn flatten_fixture_tree(tree: &Value, parent_id: Option<&str>) -> Vec<FixtureNode> {
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

fn html_attributes_for_id(id: &str) -> (String, String) {
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

fn attribute(tag: &str, name: &str) -> Option<String> {
    let marker = format!("{name}=\"");
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')? + start;
    Some(tag[start..end].to_owned())
}

fn fixture_rule(selector: &str) -> String {
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

fn make_request(
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

fn assert_layout_matches_reference(
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

fn assert_styles_match_reference(
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
            "border-top-width",
            "border-right-width",
            "border-bottom-width",
            "border-left-width",
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
    }
}
