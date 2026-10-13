use serde_json::Value;
use std::sync::OnceLock;

const C11_2_3_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/grid-auto-placement-c1123-runtime-matrix-v1.json"
);
const C11_2_3_GRID_AUTO_PLACEMENT_SCENE_IDS: [&str; 25] = [
    "row-sparse-baseline",
    "row-dense-hole-fill",
    "row-dense-span-collision",
    "row-sparse-hole-control",
    "row-dense-fixed-row",
    "row-dense-fixed-column",
    "row-dense-positive-implicit-row",
    "column-sparse-explicit-fill",
    "column-sparse-positive-implicit-column",
    "column-sparse-fixed-row",
    "column-sparse-fixed-column",
    "column-dense-hole-fill",
    "column-sparse-hole-control",
    "column-dense-span-collision",
    "column-dense-positive-implicit-column",
    "definite-first-dense",
    "hidden-item-excluded",
    "nested-mixed-flow",
    "auto-track-no-implicit",
    "reject-repeated-auto-track",
    "reject-negative-implicit-line",
    "reject-nonzero-order",
    "reject-indefinite-axis",
    "auto-span-at-track-limit",
    "auto-span-over-track-limit",
];

pub(crate) fn c11_2_3_grid_auto_placement_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_2_3_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.2.3 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-2-3-grid-auto-placement-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeBlockFlexGridAutoPlacementDenseV1")
            {
                return Err("C11.2.3 runtime matrix schema/profile이 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.2.3 runtime matrix scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_2_3_GRID_AUTO_PLACEMENT_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(25)
                || matrix["summary"]["implementationCandidateCount"].as_u64() != Some(21)
                || matrix["summary"]["oracleOnlyCount"].as_u64() != Some(4)
                || matrix["summary"]["successCount"].as_u64() != Some(19)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(6)
                || matrix["summary"]["nodeCount"].as_u64() != Some(107)
            {
                return Err("C11.2.3 runtime matrix summary가 고정 계약과 다릅니다".to_owned());
            }

            let mut total_nodes = 0;
            let mut successes = 0;
            let mut expected_rejections = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.3 scene {index} ID 누락"))?;
                if id != C11_2_3_GRID_AUTO_PLACEMENT_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.2.3 scene 순서 또는 ID 불일치: index={index}, id={id}"
                    ));
                }
                let expected = &case["expected"];
                match expected["outcome"].as_str() {
                    Some("success") => successes += 1,
                    Some("expected-rejection") => {
                        expected_rejections += 1;
                        if expected["code"].as_str().is_none()
                            || expected["property"].as_str().is_none()
                            || expected["nodeId"].as_str().is_none()
                            || expected["runtimeNodeId"].as_u64().is_none()
                        {
                            return Err(format!("C11.2.3 scene {id} 거부 계약이 불완전합니다"));
                        }
                    }
                    _ => return Err(format!("C11.2.3 scene {id} outcome이 잘못되었습니다")),
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.3 scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.3 scene {id} parentIds 누락"))?;
                let node_count = node_ids.len();
                if !(2..=10).contains(&node_count)
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
                    return Err(format!("C11.2.3 scene {id} node identity가 잘못되었습니다"));
                }
                total_nodes += node_count;
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.3 scene {id} source 누락"))?;
                if source.is_empty() || source.len() > 64 * 1024 {
                    return Err(format!(
                        "C11.2.3 scene {id} source 길이가 범위를 벗어났습니다"
                    ));
                }
            }
            if total_nodes != 107 || successes != 19 || expected_rejections != 6 {
                return Err(
                    "C11.2.3 runtime matrix scene summary가 개별 장면과 다릅니다".to_owned(),
                );
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
        .ok_or_else(|| format!("알 수 없는 C11.2.3 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.2.3 scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{
        C11_2_3_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX, C11_2_3_GRID_AUTO_PLACEMENT_SCENE_IDS,
        c11_2_3_grid_auto_placement_runtime_source,
    };

    #[test]
    fn fixed_runtime_matrix_has_all_scenes_and_only_selects_static_source_by_id() {
        for id in C11_2_3_GRID_AUTO_PLACEMENT_SCENE_IDS {
            let source = c11_2_3_grid_auto_placement_runtime_source(id).unwrap();
            assert!(source.contains(&format!("sceneId:{id:?}")));
            assert!(source.contains("document.createElement"));
        }
        assert!(c11_2_3_grid_auto_placement_runtime_source("unknown;throw 1").is_err());
        assert!(c11_2_3_grid_auto_placement_runtime_source("").is_err());
        assert!(C11_2_3_GRID_AUTO_PLACEMENT_RUNTIME_MATRIX.contains("\"nodeCount\": 107"));
    }
}
