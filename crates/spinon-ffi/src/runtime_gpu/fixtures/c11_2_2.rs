use serde_json::Value;
use std::sync::OnceLock;

const C11_2_2_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX: &str =
    include_str!("../../../../../tests/fixtures/css/c11/grid-auto-placement-runtime-matrix.json");
const C11_2_2_GRID_ITEM_ORDER_RUNTIME_MATRIX: &str =
    include_str!("../../../../../tests/fixtures/css/c11/grid-item-order-runtime-matrix-v1.json");
const C11_2_2_GRID_ITEM_ORDER_SCENE_IDS: [&str; 2] = ["order-mixed", "order-ties"];
const C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS: [&str; 16] = [
    "definite-first-mixed",
    "auto-only-fill",
    "positive-implicit-row",
    "row-locked-auto-column",
    "column-locked-auto-row",
    "auto-column-span",
    "sparse-hole-not-backfilled",
    "implicit-track-size-and-gap",
    "hidden-child-excluded",
    "nested-independent-placement",
    "reject-implicit-column",
    "reject-auto-sized-implicit-row",
    "reject-repeated-auto-row-pattern",
    "reject-dense-flow",
    "reject-column-flow",
    "column-cursor-wrap",
];

pub(crate) fn c11_2_2_grid_auto_placement_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    if C11_2_2_GRID_ITEM_ORDER_SCENE_IDS.contains(&scene_id) {
        return c11_2_2_grid_item_order_runtime_source(scene_id);
    }
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_2_2_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.2.2 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-2-2-grid-auto-placement-runtime-matrix/v1")
            {
                return Err("C11.2.2 runtime matrix schema가 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.2.2 runtime matrix scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS.len() {
                return Err(format!(
                    "C11.2.2 runtime scene 수가 다릅니다: expected={}, actual={}",
                    C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS.len(),
                    cases.len()
                ));
            }

            let mut total_nodes = 0;
            let mut candidates = 0;
            let mut expected_rejects = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.2 scene {index} ID 누락"))?;
                if id != C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.2.2 scene 순서 또는 ID 불일치: index={index}, id={id}"
                    ));
                }
                let scope = case["scope"].as_str();
                if scope == Some("implementation-candidate") {
                    candidates += 1;
                    if case["expected"]["outcome"].as_str() != Some("success") {
                        return Err(format!("C11.2.2 scene {id} success 계약 누락"));
                    }
                } else if scope == Some("chromium-oracle-only") {
                    expected_rejects += 1;
                    let expected = &case["expected"];
                    if expected["outcome"].as_str() != Some("expected-rejection")
                        || expected["code"].as_str().is_none()
                        || expected["property"].as_str().is_none()
                        || expected["runtimeNodeId"].as_u64().is_none()
                    {
                        return Err(format!("C11.2.2 scene {id} 거부 계약이 불완전합니다"));
                    }
                } else {
                    return Err(format!("C11.2.2 scene {id} scope가 잘못되었습니다"));
                }

                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.2 scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.2 scene {id} parentIds 누락"))?;
                let node_count = node_ids.len();
                if !(2..=9).contains(&node_count)
                    || node_count != parent_ids.len()
                    || node_ids.iter().any(|node| node.as_str().is_none())
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != node_count
                    || case["rootId"].as_str() != node_ids.first().and_then(Value::as_str)
                {
                    return Err(format!("C11.2.2 scene {id} node identity가 잘못되었습니다"));
                }
                total_nodes += node_count;
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.2 scene {id} source 누락"))?;
                if source.is_empty() || source.len() > 64 * 1024 {
                    return Err(format!(
                        "C11.2.2 scene {id} source 길이가 범위를 벗어났습니다"
                    ));
                }
            }
            if candidates != 11
                || expected_rejects != 5
                || total_nodes != 69
                || matrix["summary"]["sceneCount"].as_u64() != Some(16)
                || matrix["summary"]["candidateCount"].as_u64() != Some(11)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(5)
                || matrix["summary"]["nodeCount"].as_u64() != Some(69)
            {
                return Err("C11.2.2 runtime matrix summary가 scene 내용과 다릅니다".to_owned());
            }
            Ok(matrix)
        })
        .as_ref()
        .map_err(Clone::clone)?;
    let case = matrix["cases"]
        .as_array()
        .and_then(|cases| {
            cases
                .iter()
                .find(|case| case["id"].as_str() == Some(scene_id))
        })
        .ok_or_else(|| format!("알 수 없는 C11.2.2 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.2.2 scene {scene_id} source 누락"))
}

fn c11_2_2_grid_item_order_runtime_source(scene_id: &str) -> Result<&'static str, String> {
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_2_2_GRID_ITEM_ORDER_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.2.2 order runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeBlockFlexGridAutoPlacementV1")
            {
                return Err("C11.2.2 order runtime matrix schema가 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.2.2 order runtime scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_2_2_GRID_ITEM_ORDER_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(2)
                || matrix["summary"]["nodeCount"].as_u64() != Some(9)
                || matrix["summary"]["itemCount"].as_u64() != Some(7)
            {
                return Err("C11.2.2 order runtime matrix summary가 다릅니다".to_owned());
            }
            let mut node_count = 0usize;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.2 order scene {index} ID 누락"))?;
                if id != C11_2_2_GRID_ITEM_ORDER_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                    || case["expected"]["outcome"].as_str() != Some("success")
                {
                    return Err(format!("C11.2.2 order scene 순서 또는 계약 불일치: {id}"));
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.2 order scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.2 order scene {id} parentIds 누락"))?;
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.2 order scene {id} source 누락"))?;
                if !(4..=5).contains(&node_ids.len())
                    || node_ids.len() != parent_ids.len()
                    || node_ids.iter().any(|node| node.as_str().is_none())
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != node_ids.len()
                    || case["rootId"].as_str() != node_ids.first().and_then(Value::as_str)
                    || source.len() > 64 * 1024
                    || !source.contains(&format!("sceneId:{id:?}"))
                    || !source.contains("document.createElement")
                {
                    return Err(format!(
                        "C11.2.2 order scene {id} identity/source가 잘못되었습니다"
                    ));
                }
                let expected_rects = case["expectedRects"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.2 order scene {id} expectedRects 누락"))?;
                if expected_rects.len() != node_ids.len()
                    || expected_rects.iter().any(|expected| {
                        expected["id"].as_str().is_none()
                            || ["x", "y", "width", "height"]
                                .into_iter()
                                .any(|field| expected["rect"][field].as_f64().is_none())
                    })
                {
                    return Err(format!(
                        "C11.2.2 order scene {id} frame 계약이 잘못되었습니다"
                    ));
                }
                node_count += node_ids.len();
            }
            if node_count != 9 {
                return Err(format!(
                    "C11.2.2 order runtime node 수가 다릅니다: expected=9, actual={node_count}"
                ));
            }
            Ok(matrix)
        })
        .as_ref()
        .map_err(Clone::clone)?;
    let case = matrix["cases"]
        .as_array()
        .and_then(|cases| {
            cases
                .iter()
                .find(|case| case["id"].as_str() == Some(scene_id))
        })
        .ok_or_else(|| format!("알 수 없는 C11.2.2 order scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.2.2 order scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{
        C11_2_2_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX, C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS,
        C11_2_2_GRID_ITEM_ORDER_RUNTIME_MATRIX, C11_2_2_GRID_ITEM_ORDER_SCENE_IDS,
        c11_2_2_grid_auto_placement_runtime_source,
    };

    #[test]
    fn fixed_runtime_matrix_has_all_scenes_and_source_is_only_selected_by_id() {
        for id in C11_2_2_GRID_AUTO_PLACEMENT_SCENE_IDS {
            let source = c11_2_2_grid_auto_placement_runtime_source(id).unwrap();
            assert!(source.contains(&format!("sceneId:{id:?}")));
            assert!(source.contains("document.createElement"));
        }
        assert!(c11_2_2_grid_auto_placement_runtime_source("unknown;throw 1").is_err());
        assert!(c11_2_2_grid_auto_placement_runtime_source("").is_err());
        assert!(C11_2_2_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX.contains("\"nodeCount\": 69"));
    }

    #[test]
    fn order_runtime_matrix_is_separate_and_only_selected_by_fixed_scene_id() {
        for id in C11_2_2_GRID_ITEM_ORDER_SCENE_IDS {
            let source = c11_2_2_grid_auto_placement_runtime_source(id).unwrap();
            assert!(source.contains(&format!("sceneId:{id:?}")));
            assert!(source.contains("document.createElement"));
        }
        assert!(
            C11_2_2_GRID_ITEM_ORDER_RUNTIME_MATRIX
                .contains("spinon-css-c11-2-2-grid-item-order-runtime-matrix/v1")
        );
        assert!(c11_2_2_grid_auto_placement_runtime_source("order-ties;throw 1").is_err());
    }
}
