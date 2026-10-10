use super::{
    block_formatting_test_support::{
        FixtureNode, INVENTORY, REFERENCE, assert_layout_matches_reference,
        assert_styles_match_reference, flatten_fixture_tree, make_request,
    },
    compute_request_for_block_formatting,
};
use serde_json::Value;

#[test]
fn c093_flow_root_matches_pinned_chromium_for_all_cases_and_dpr_values() {
    let inventory: Value = serde_json::from_str(INVENTORY).unwrap();
    let reference: Value = serde_json::from_str(REFERENCE).unwrap();
    let inventory_cases = inventory["cases"].as_array().unwrap();
    let reference_cases = reference["observations"][0]["cases"].as_array().unwrap();
    let cases = inventory_cases
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("c093-"))
        .collect::<Vec<_>>();
    let mut tested_nodes = 0;

    assert_eq!(cases.len(), 4);
    for case in &cases {
        let case_id = case["id"].as_str().unwrap();
        let expected = reference_cases
            .iter()
            .find(|candidate| candidate["id"] == case_id)
            .unwrap_or_else(|| panic!("Chromium reference에 case가 없습니다: {case_id}"));
        let fixtures = flatten_fixture_tree(&case["tree"], None);
        let expected_nodes = expected["nodes"].as_array().unwrap();
        assert_eq!(fixtures.len(), expected_nodes.len(), "{case_id} node 수");
        tested_nodes += fixtures.len();

        for (fixture, expected_node) in fixtures.iter().zip(expected_nodes) {
            assert_eq!(
                fixture.id,
                expected_node["id"].as_str().unwrap(),
                "{case_id} preorder"
            );
            assert_eq!(
                fixture.parent_id.as_deref(),
                expected_node["parentId"].as_str(),
                "{case_id}/{} parent",
                fixture.id
            );
        }

        for dpr in [1.0, 2.0] {
            let (request, handles) = make_request(&fixtures, dpr);
            let calculation = compute_request_for_block_formatting(&request)
                .unwrap_or_else(|error| panic!("{case_id} runtime 계산 실패: {error}"));
            assert_layout_matches_reference(case_id, &calculation, &handles, expected);
            assert_styles_match_reference(case_id, &calculation, &handles, expected);
        }
    }

    assert_eq!(tested_nodes, 16);
}

#[test]
fn c093_flow_root_is_accepted_from_author_stylesheet() {
    let (request, child) = super::block_paint_tests::block_request_with_author_stylesheet(
        ".painted { display: flow-root; height: 12px; margin-top: 8px; }",
    );
    let calculation = compute_request_for_block_formatting(&request).unwrap();
    let layout = calculation
        .layout
        .unwrap_or_else(|failure| panic!("author flow-root layout 실패: {failure:?}"));
    let computed = calculation.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.node_id == child.id())
        .unwrap();

    assert_eq!(
        computed.properties.get("display").map(String::as_str),
        Some("flow-root")
    );
    assert_eq!(
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == child.id().get())
            .map(|frame| (frame.y, frame.height)),
        Some((8.0, 12.0))
    );
}

#[test]
fn flow_root_still_rejects_unimplemented_display_values() {
    for value in ["grid", "flex", "inline", "table", "inline flow-root"] {
        let fixture = FixtureNode {
            id: format!("c093-unsupported-{value}"),
            parent_id: None,
            class_name: String::new(),
            inline_style: format!("display:{value}"),
        };
        let (request, _) = make_request(&[fixture], 1.0);
        let calculation = compute_request_for_block_formatting(&request).unwrap();
        let failure = match calculation.layout {
            Err(failure) => failure,
            Ok(layout) => panic!("미지원 display가 성공했습니다: {value}: {layout:?}"),
        };
        assert_eq!(failure.code, "unsupported_block_display", "{value}");
    }
}
