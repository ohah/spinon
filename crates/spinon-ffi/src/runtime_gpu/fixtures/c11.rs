use serde_json::Value;
use std::sync::OnceLock;

const C11_GRID_RUNTIME_MATRIX: &str =
    include_str!("../../../../../tests/fixtures/css/c11/runtime-grid-matrix.json");
const C11_2_1_GRID_PLACEMENT_RUNTIME_MATRIX: &str =
    include_str!("../../../../../tests/fixtures/css/c11/grid-placement-runtime-matrix.json");
const C11_GRID_SCENE_IDS: [&str; 10] = [
    "fixed-tracks-gap",
    "percentage-tracks",
    "fractional-tracks-gap",
    "mixed-fixed-percent-fr",
    "row-auto-placement",
    "empty-grid",
    "single-item-grid",
    "grid-under-block",
    "grid-under-flex",
    "flex-under-grid",
];
const C11_2_1_GRID_PLACEMENT_SCENE_IDS: [&str; 10] = [
    "numeric-lines",
    "positive-spans",
    "reversed-lines",
    "shorthand-cascade",
    "overlap-source-order",
    "nested-placement-modes",
    "deferred-implicit-track",
    "deferred-named-line",
    "deferred-mixed-auto-explicit",
    "deferred-equal-lines",
];

struct C11GridRuntimeCase {
    id: String,
    source: String,
    node_count: usize,
}

fn runtime_cases() -> Result<&'static [C11GridRuntimeCase], String> {
    static CASES: OnceLock<Result<Vec<C11GridRuntimeCase>, String>> = OnceLock::new();
    CASES
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_GRID_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.1 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str() != Some("spinon-css-c11-grid-runtime-matrix/v1") {
                return Err("C11.1 runtime matrix schema가 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.1 runtime matrix scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_GRID_SCENE_IDS.len() {
                return Err(format!(
                    "C11.1 runtime scene 수가 다릅니다: expected={}, actual={}",
                    C11_GRID_SCENE_IDS.len(),
                    cases.len()
                ));
            }
            let mut runtime_cases = Vec::with_capacity(cases.len());
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.1 scene {index}에 ID가 없습니다"))?;
                if id != C11_GRID_SCENE_IDS[index] || case["index"].as_u64() != Some(index as u64) {
                    return Err(format!(
                        "C11.1 scene index/ID가 기준과 다릅니다: index={index}, id={id}"
                    ));
                }
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.1 scene {id}에 JavaScript source가 없습니다"))?;
                if source.is_empty() || source.len() > 64 * 1024 {
                    return Err(format!("C11.1 scene {id} source 크기가 허용 범위 밖입니다"));
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.1 scene {id} NodeId 목록이 없습니다"))?;
                let node_count = node_ids.len();
                if !(1..=7).contains(&node_count)
                    || node_ids.iter().any(|node| node.as_str().is_none())
                {
                    return Err(format!("C11.1 scene {id}의 NodeId 목록이 잘못되었습니다"));
                }
                let unique_nodes = node_ids
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<std::collections::BTreeSet<_>>();
                if unique_nodes.len() != node_count {
                    return Err(format!("C11.1 scene {id}에 중복 NodeId가 있습니다"));
                }
                runtime_cases.push(C11GridRuntimeCase {
                    id: id.to_owned(),
                    source: source.to_owned(),
                    node_count,
                });
            }
            if matrix["summary"]["sceneCount"].as_u64() != Some(C11_GRID_SCENE_IDS.len() as u64)
                || runtime_cases
                    .iter()
                    .map(|case| case.node_count)
                    .sum::<usize>()
                    != 40
                || matrix["summary"]["nodeCount"].as_u64() != Some(40)
            {
                return Err("C11.1 runtime matrix summary가 scene 내용과 다릅니다".to_owned());
            }
            Ok(runtime_cases)
        })
        .as_deref()
        .map_err(Clone::clone)
}

pub(crate) fn c11_grid_runtime_scene(index: u32) -> Result<(String, String), String> {
    let cases = runtime_cases()?;
    let case = cases
        .get(index as usize)
        .ok_or_else(|| format!("C11.1 scene index 범위 초과: {index}"))?;
    Ok((case.id.clone(), case.source.clone()))
}

pub(crate) fn c11_2_1_grid_placement_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    static CASES: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = CASES
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_2_1_GRID_PLACEMENT_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.2.1 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-2-1-grid-placement-runtime-matrix/v1")
            {
                return Err("C11.2.1 runtime matrix schema가 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.2.1 runtime matrix scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_2_1_GRID_PLACEMENT_SCENE_IDS.len() {
                return Err(format!(
                    "C11.2.1 runtime scene 수가 다릅니다: expected={}, actual={}",
                    C11_2_1_GRID_PLACEMENT_SCENE_IDS.len(),
                    cases.len()
                ));
            }
            let mut total_nodes = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.1 scene {index}에 ID가 없습니다"))?;
                if id != C11_2_1_GRID_PLACEMENT_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.2.1 scene index/ID가 기준과 다릅니다: index={index}, id={id}"
                    ));
                }
                let expected_scope = if index < 6 {
                    "implementation-candidate"
                } else {
                    "chromium-oracle-only"
                };
                if case["scope"].as_str() != Some(expected_scope) {
                    return Err(format!("C11.2.1 scene {id} scope가 고정값과 다릅니다"));
                }
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.2.1 scene {id}에 source가 없습니다"))?;
                if source.is_empty() || source.len() > 64 * 1024 {
                    return Err(format!(
                        "C11.2.1 scene {id} source 크기가 범위를 벗어났습니다"
                    ));
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.2.1 scene {id} nodeIds가 없습니다"))?;
                let root_id = case["rootId"].as_str();
                if !(1..=7).contains(&node_ids.len())
                    || node_ids.iter().any(|node| node.as_str().is_none())
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != node_ids.len()
                    || root_id != node_ids.first().and_then(Value::as_str)
                {
                    return Err(format!("C11.2.1 scene {id} node identity가 잘못되었습니다"));
                }
                total_nodes += node_ids.len();
                let expected = &case["expected"];
                if index < 6 {
                    if expected["outcome"].as_str() != Some("success") {
                        return Err(format!("C11.2.1 scene {id} success expectation이 다릅니다"));
                    }
                } else if expected["outcome"].as_str() != Some("expected-rejection")
                    || expected["code"].as_str().is_none()
                    || !(expected["nodeId"].is_null() || expected["nodeId"].as_str().is_some())
                    || !(expected["property"].is_null() || expected["property"].as_str().is_some())
                    || !(expected["runtimeNodeId"].is_null()
                        || expected["runtimeNodeId"].as_u64().is_some())
                    || !(expected["diagnosticIncludes"].is_null()
                        || expected["diagnosticIncludes"].as_str().is_some())
                {
                    return Err(format!(
                        "C11.2.1 scene {id} reject expectation이 불완전합니다"
                    ));
                }
            }
            if total_nodes != 30
                || matrix["summary"]["sceneCount"].as_u64() != Some(10)
                || matrix["summary"]["candidateCount"].as_u64() != Some(6)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(4)
                || matrix["summary"]["nodeCount"].as_u64() != Some(30)
            {
                return Err("C11.2.1 runtime matrix summary가 scene 내용과 다릅니다".to_owned());
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
        .ok_or_else(|| format!("알 수 없는 C11.2.1 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.2.1 scene {scene_id} source가 없습니다"))
}

#[cfg(test)]
mod tests {
    use super::{
        C11_2_1_GRID_PLACEMENT_SCENE_IDS, C11_GRID_SCENE_IDS,
        c11_2_1_grid_placement_runtime_source, c11_grid_runtime_scene,
    };

    #[test]
    fn matrix_cases_are_fixed_unique_and_bounded() {
        let mut total_nodes = 0;
        for (index, expected_id) in C11_GRID_SCENE_IDS.iter().enumerate() {
            let (id, source) = c11_grid_runtime_scene(index as u32).expect("scene source");
            assert_eq!(id, *expected_id);
            assert!(!source.is_empty());
            assert!(source.len() <= 64 * 1024);
            total_nodes += source
                .matches("setAttribute(\"data-c11-grid-node\"")
                .count();
        }
        assert_eq!(total_nodes, 40);
        assert!(c11_grid_runtime_scene(C11_GRID_SCENE_IDS.len() as u32).is_err());
        assert!(c11_grid_runtime_scene(u32::MAX).is_err());
    }

    #[test]
    fn placement_matrix_is_fixed_bounded_and_rejects_unlisted_scene_ids() {
        let mut total_nodes = 0;
        for scene_id in C11_2_1_GRID_PLACEMENT_SCENE_IDS {
            let source =
                c11_2_1_grid_placement_runtime_source(scene_id).expect("fixed scene source");
            assert!(!source.is_empty());
            assert!(source.len() <= 64 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            total_nodes += source
                .matches("setAttribute(\"data-c1121-grid-node\"")
                .count();
        }
        assert_eq!(total_nodes, 30);
        assert!(c11_2_1_grid_placement_runtime_source("numeric-lines;throw 1").is_err());
        assert!(c11_2_1_grid_placement_runtime_source("").is_err());
    }
}
