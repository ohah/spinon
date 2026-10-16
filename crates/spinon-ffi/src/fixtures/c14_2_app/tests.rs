use super::oracle::{oracle_cases, verify_png};
use super::{
    BLOCK_HTML, BLOCK_INVENTORY, BLOCK_ORACLE, CASES, FLEX_HTML, FLEX_INVENTORY, FLEX_ORACLE,
};

#[test]
fn live_case_mapping_matches_the_21_plus_8_oracle_order() {
    let fixture: serde_json::Value = serde_json::from_str(CASES).expect("case JSON");
    let block: serde_json::Value = serde_json::from_str(BLOCK_INVENTORY).expect("Block inventory");
    let flex: serde_json::Value = serde_json::from_str(FLEX_INVENTORY).expect("Flex inventory");
    let block_mapping = fixture["blockCases"].as_array().expect("Block mapping");
    let flex_mapping = fixture["flexCases"].as_array().expect("Flex mapping");
    assert_eq!(block_mapping.len(), 21);
    assert_eq!(flex_mapping.len(), 8);
    let positive_block = block["cases"]
        .as_array()
        .expect("Block cases")
        .iter()
        .filter(|case| case.get("layoutStage").is_none())
        .map(|case| case["id"].as_str().expect("Block ID"))
        .collect::<Vec<_>>();
    let mapped_block = block_mapping
        .iter()
        .map(|case| case["id"].as_str().expect("mapped Block ID"))
        .collect::<Vec<_>>();
    assert_eq!(mapped_block, positive_block);
    let oracle_flex = flex["cases"]
        .as_array()
        .expect("Flex cases")
        .iter()
        .map(|case| case["id"].as_str().expect("Flex ID"))
        .collect::<Vec<_>>();
    let mapped_flex = flex_mapping
        .iter()
        .map(|case| case["id"].as_str().expect("mapped Flex ID"))
        .collect::<Vec<_>>();
    assert_eq!(mapped_flex, oracle_flex);
}

#[test]
fn live_case_styles_match_the_original_html_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(CASES).expect("case JSON");
    for case in fixture["blockCases"].as_array().expect("Block mapping") {
        let id = case["id"].as_str().expect("Block ID");
        let style = case["style"].as_str().expect("Block style");
        let expected = if style.is_empty() {
            format!(r#"<img id="{id}" src="../../resources/c14-1-png-basic.png">"#)
        } else {
            format!(r#"<img id="{id}" src="../../resources/c14-1-png-basic.png" style="{style}">"#)
        };
        assert!(
            BLOCK_HTML.contains(&expected),
            "Block source differs for {id}"
        );
    }
    for case in fixture["flexCases"].as_array().expect("Flex mapping") {
        let id = case["id"].as_str().expect("Flex ID");
        let style = case["style"].as_str().expect("Flex style");
        let parent_class = case["parentClass"].as_str().expect("parent class");
        let parent_style = case["parentStyle"].as_str().expect("parent style");
        let parent_style = if parent_style.is_empty() {
            String::new()
        } else {
            format!(r#" style="{parent_style}""#)
        };
        let expected_parent = format!(r#"<div class="{parent_class}" id="{id}"{parent_style}>"#);
        let expected_image =
            format!(r#"<img src="../../resources/c14-1-png-basic.png" style="{style}">"#);
        let start = FLEX_HTML
            .find(&expected_parent)
            .expect("Flex parent source");
        let end = FLEX_HTML[start..].find("</div>").expect("Flex parent end") + start;
        assert!(
            FLEX_HTML[start..end].contains(&expected_image),
            "Flex source differs for {id}"
        );
    }
}

#[test]
fn embedded_png_matches_both_inventory_hashes_and_dimensions() {
    let block: serde_json::Value = serde_json::from_str(BLOCK_INVENTORY).expect("Block inventory");
    let flex: serde_json::Value = serde_json::from_str(FLEX_INVENTORY).expect("Flex inventory");
    assert_eq!(
        verify_png(&block, &flex).expect("fixed PNG should match"),
        "5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2"
    );
}

#[test]
fn chrome_oracles_parse_png_natural_size_and_all_case_geometry() {
    let fixture: serde_json::Value = serde_json::from_str(CASES).expect("case JSON");
    let block: serde_json::Value = serde_json::from_str(BLOCK_INVENTORY).expect("Block inventory");
    let flex: serde_json::Value = serde_json::from_str(FLEX_INVENTORY).expect("Flex inventory");
    let block_oracle: serde_json::Value = serde_json::from_str(BLOCK_ORACLE).expect("Block oracle");
    let flex_oracle: serde_json::Value = serde_json::from_str(FLEX_ORACLE).expect("Flex oracle");
    let block_cases = oracle_cases(
        fixture["blockCases"].as_array().expect("Block mapping"),
        &block,
        &block_oracle,
        false,
    )
    .expect("Block Chrome oracle should parse");
    let flex_cases = oracle_cases(
        fixture["flexCases"].as_array().expect("Flex mapping"),
        &flex,
        &flex_oracle,
        true,
    )
    .expect("Flex Chrome oracle should parse");

    assert_eq!(block_cases.len(), 21);
    assert_eq!(flex_cases.len(), 8);
    assert_eq!(block_cases[0].expected_size, [24.0, 16.0]);
}
