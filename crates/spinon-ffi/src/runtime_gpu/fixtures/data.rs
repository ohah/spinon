pub(crate) const RUNTIME_GPU_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c04/runtime-css-to-gpu-resize.js");
pub(crate) const RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c04/runtime-author-stylesheets.js");
pub(crate) const RUNTIME_CSS_CUSTOM_PROPERTIES_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c05/runtime-custom-properties-app.js");
pub(crate) const RUNTIME_CSS_REGISTERED_PROPERTIES_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c05/runtime-registered-properties.js");
pub(crate) const RUNTIME_CSS_RESULT_CACHE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c05/runtime-result-cache.js");
pub(crate) const RUNTIME_CSS_INCREMENTAL_RESTYLE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c05/runtime-incremental-restyle-runtime.js");
pub(crate) const RUNTIME_CSS_PERCENTAGE_DIMENSIONS_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-percentage-dimensions.js");
pub(crate) const RUNTIME_CSS_SPACING_PERCENTAGES_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-spacing-percentages.js");
pub(crate) const RUNTIME_CSS_ABSOLUTE_LENGTHS_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-absolute-lengths.js");
pub(crate) const RUNTIME_CSS_FONT_RELATIVE_UNITS_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-font-relative-units.js");
pub(crate) const RUNTIME_CSS_TYPED_MATH_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-typed-css-math.js");
pub(crate) const RUNTIME_CSS_VIEWPORT_UNITS_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c06/runtime-viewport-units.js");
pub(crate) const RUNTIME_CSS_MIN_MAX_SIZING_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c07/runtime-min-max-sizing.js");
pub(crate) const RUNTIME_CSS_BORDER_WIDTH_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c07/runtime-border-width.js");
pub(crate) const RUNTIME_CSS_ASPECT_RATIO_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c07/runtime-aspect-ratio.js");
pub(crate) const RUNTIME_CSS_BLOCK_PAINT_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c08/runtime-block-paint.js");
pub(crate) const RUNTIME_CSS_BLOCK_FORMATTING_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c09/runtime-block-formatting.js");
pub(crate) const RUNTIME_CSS_C12_1_POSITION_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c12/runtime-position-static-relative.js");
pub(crate) const RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c12/runtime-position-absolute-block.js");
pub(crate) const RUNTIME_CSS_C12_3_FIXED_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c12/runtime-position-fixed.js");
pub(crate) const RUNTIME_CSS_C12_4_STACKING_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c12/runtime-stacking-context.js");
pub(crate) const C12_3_FIXED_INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c12/position-fixed-inventory.json");
pub(crate) const C12_3_FIXED_SUPPORTED_CASES: &[&str] = &[
    "viewport-insets",
    "unpositioned-ancestor",
    "positioned-ancestors",
    "fixed-nesting",
    "hidden-subtree",
];
pub(crate) const RUNTIME_CSS_MARGIN_COLLAPSE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c09/runtime-margin-collapse.js");
pub(crate) const RUNTIME_CSS_FLOW_ROOT_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c09/runtime-flow-root.js");
pub(crate) const RUNTIME_CSS_FLEX_WRAP_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-wrap.js");
pub(crate) const RUNTIME_CSS_FLEX_REVERSE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-reverse.js");
pub(crate) const RUNTIME_CSS_FLEX_ORDER_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-order-alignment.js");
pub(crate) const RUNTIME_CSS_FLEX_DISTRIBUTION_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-distribution.js");
pub(crate) const RUNTIME_CSS_FLEX_ALIGNMENT_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-alignment.js");
pub(crate) const RUNTIME_CSS_FLEX_BASELINE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-flex-baseline.js");
pub(crate) const RUNTIME_CSS_C10_3_5_POSITIONED_FLEX_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c10/runtime-positioned-flex.js");
pub(crate) const C10_3_5_POSITIONED_FLEX_INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c10/positioned-flex-inventory.json");
pub(crate) const C10_3_5_POSITIONED_FLEX_RUNTIME_INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c10/positioned-flex-runtime-inventory.json");

pub(crate) fn c10_3_5_positioned_flex_fixture_source() -> String {
    let mut source = String::with_capacity(
        C10_3_5_POSITIONED_FLEX_INVENTORY.len()
            + C10_3_5_POSITIONED_FLEX_RUNTIME_INVENTORY.len()
            + RUNTIME_CSS_C10_3_5_POSITIONED_FLEX_FIXTURE_SOURCE.len()
            + 128,
    );
    source.push_str("globalThis.__spinonC1035FixtureInventory = ");
    source.push_str(C10_3_5_POSITIONED_FLEX_INVENTORY);
    source.push_str(";\nglobalThis.__spinonC1035RuntimeInventory = ");
    source.push_str(C10_3_5_POSITIONED_FLEX_RUNTIME_INVENTORY);
    source.push_str(";\n");
    source.push_str(RUNTIME_CSS_C10_3_5_POSITIONED_FLEX_FIXTURE_SOURCE);
    source
}

pub(crate) fn c12_3_fixed_supported_nodes() -> Result<Vec<serde_json::Value>, String> {
    let inventory: serde_json::Value = serde_json::from_str(C12_3_FIXED_INVENTORY)
        .map_err(|error| format!("C12.3 inventory를 읽지 못했습니다: {error}"))?;
    if inventory["schema"].as_str() != Some("spinon-css-c12-3-position-fixed-inventory/v1") {
        return Err("C12.3 inventory schema가 다릅니다".to_owned());
    }
    let nodes = inventory["nodes"]
        .as_array()
        .ok_or_else(|| "C12.3 runtime matrix node 목록이 없습니다".to_owned())?;
    let supported_cases = C12_3_FIXED_SUPPORTED_CASES
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let selected = nodes
        .iter()
        .filter(|node| {
            node["id"].as_str() == Some("c12-root")
                || node["caseId"]
                    .as_str()
                    .is_some_and(|case| supported_cases.contains(case))
        })
        .cloned()
        .collect::<Vec<_>>();
    if selected.first().and_then(|node| node["id"].as_str()) != Some("c12-root")
        || selected
            .first()
            .is_none_or(|node| !node["parentId"].is_null())
    {
        return Err("C12.3 runtime subset의 root가 inventory 첫 node가 아닙니다".to_owned());
    }
    let mut seen = std::collections::BTreeSet::new();
    for node in &selected {
        let Some(id) = node["id"].as_str() else {
            return Err("C12.3 runtime node에 id가 없습니다".to_owned());
        };
        if !seen.insert(id) {
            return Err(format!("C12.3 runtime node id가 중복되었습니다: {id}"));
        }
        if let Some(parent_id) = node["parentId"].as_str()
            && !seen.contains(parent_id)
        {
            return Err(format!(
                "C12.3 runtime subset에서 parent가 자식보다 뒤에 있거나 제외되었습니다: {id}/{parent_id}"
            ));
        }
        if node["style"].as_str().is_none()
            || node["caseId"].as_str().is_none()
            || node["expectedPosition"].as_str().is_none()
        {
            return Err(format!("C12.3 runtime node 계약 필드가 없습니다: {id}"));
        }
    }
    Ok(selected)
}

pub(crate) fn c12_3_fixed_runtime_script() -> Result<String, String> {
    let nodes = c12_3_fixed_supported_nodes()?;
    let encoded_nodes = serde_json::to_string(&nodes)
        .map_err(|error| format!("C12.3 runtime node 목록 인코딩 실패: {error}"))?;
    Ok(format!(
        "globalThis.__spinonC123FixtureNodes={encoded_nodes};\n{RUNTIME_CSS_C12_3_FIXED_FIXTURE_SOURCE}"
    ))
}
