use serde_json::Value;
use std::sync::OnceLock;

const C11_3_2_GRID_NAMED_LINES_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/grid-named-lines-c1132-runtime-matrix-v1.json"
);
const C11_3_2_GRID_NAMED_LINES_SCENE_IDS: [&str; 2] =
    ["positive-occurrence", "missing-name-implicit-line"];

pub(crate) fn c11_3_2_grid_named_line_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_3_2_GRID_NAMED_LINES_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.3.2 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-3-2-grid-named-lines-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeBlockFlexGridTemplateV1")
            {
                return Err("C11.3.2 runtime matrix schema/profile이 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.3.2 runtime matrix 장면 목록이 없습니다".to_owned())?;
            if cases.len() != C11_3_2_GRID_NAMED_LINES_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(2)
                || matrix["summary"]["nodeCount"].as_u64() != Some(4)
                || matrix["summary"]["successCount"].as_u64() != Some(1)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(1)
            {
                return Err("C11.3.2 runtime matrix 요약이 고정 계약과 다릅니다".to_owned());
            }

            let mut success_count = 0;
            let mut rejection_count = 0;
            let mut node_count = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.3.2 scene {index} ID 누락"))?;
                if id != C11_3_2_GRID_NAMED_LINES_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.3.2 scene 순서 또는 ID 불일치: index={index}, id={id}"
                    ));
                }
                let expected = &case["expected"];
                match expected["outcome"].as_str() {
                    Some("success") => success_count += 1,
                    Some("expected-rejection") => {
                        rejection_count += 1;
                        if expected["code"].as_str() != Some("unsupported_named_grid_line")
                            || expected["property"].as_str().is_none()
                            || expected["nodeId"].as_str().is_none()
                        {
                            return Err(format!("C11.3.2 scene {id} 거부 계약이 불완전합니다"));
                        }
                    }
                    _ => return Err(format!("C11.3.2 scene {id} outcome이 잘못되었습니다")),
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.3.2 scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.3.2 scene {id} parentIds 누락"))?;
                if node_ids.len() != 2
                    || parent_ids.len() != node_ids.len()
                    || node_ids.iter().any(|node| node.as_str().is_none())
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != node_ids.len()
                    || case["rootId"].as_str() != node_ids.first().and_then(Value::as_str)
                    || !parent_ids[0].is_null()
                    || parent_ids[1].as_str() != case["rootId"].as_str()
                {
                    return Err(format!("C11.3.2 scene {id} node identity가 잘못되었습니다"));
                }
                if expected["outcome"].as_str() == Some("expected-rejection")
                    && expected["nodeId"].as_str() != node_ids.get(1).and_then(Value::as_str)
                {
                    return Err(format!("C11.3.2 scene {id} 거부 node ID가 다릅니다"));
                }
                node_count += node_ids.len();
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.3.2 scene {id} source 누락"))?;
                if source.is_empty()
                    || source.len() > 16 * 1024
                    || !source.contains("document.createElement")
                    || !source.contains("grid-template-columns:")
                    || !source.contains(&format!("sceneId:'{id}'"))
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .any(|node_id| !source.contains(node_id))
                {
                    return Err(format!("C11.3.2 scene {id} source가 고정 계약과 다릅니다"));
                }
            }
            if success_count != 1 || rejection_count != 1 || node_count != 4 {
                return Err("C11.3.2 runtime matrix 개별 장면과 요약이 다릅니다".to_owned());
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
        .ok_or_else(|| format!("알 수 없는 C11.3.2 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.3.2 scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{
        C11_3_2_GRID_NAMED_LINES_RUNTIME_MATRIX, C11_3_2_GRID_NAMED_LINES_SCENE_IDS,
        c11_3_2_grid_named_line_runtime_source,
    };

    #[test]
    fn fixed_runtime_matrix_selects_only_the_success_and_expected_rejection_scenes() {
        for id in C11_3_2_GRID_NAMED_LINES_SCENE_IDS {
            let source = c11_3_2_grid_named_line_runtime_source(id).unwrap();
            assert!(source.contains(&format!("sceneId:'{id}'")));
            assert!(source.contains("document.createElement"));
        }
        assert!(c11_3_2_grid_named_line_runtime_source("unknown;throw 1").is_err());
        assert!(C11_3_2_GRID_NAMED_LINES_RUNTIME_MATRIX.contains("\"nodeCount\": 4"));
    }
}
