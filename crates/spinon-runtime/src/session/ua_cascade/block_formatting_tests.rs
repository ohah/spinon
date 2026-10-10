use super::block_formatting_test_support::{
    FixtureNode, INVENTORY, REFERENCE, assert_layout_matches_reference,
    assert_styles_match_reference, flatten_fixture_tree, make_request,
};
use super::*;
use serde_json::Value;

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
fn c092_margin_collapse_matches_pinned_chromium_for_all_cases() {
    let inventory: Value = serde_json::from_str(INVENTORY).unwrap();
    let reference: Value = serde_json::from_str(REFERENCE).unwrap();
    let inventory_cases = inventory["cases"].as_array().unwrap();
    let reference_cases = reference["observations"][0]["cases"].as_array().unwrap();
    let mut tested_case_count = 0;
    let mut tested_node_count = 0;

    for case in inventory_cases
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("c092-"))
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

    assert_eq!(tested_case_count, 16);
    assert_eq!(tested_node_count, 57);
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
            "direction:rtl",
            "unsupported_inline_property",
            Some("direction"),
        ),
        (
            "position:absolute",
            "unsupported_computed_value",
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
