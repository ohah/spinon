use serde_json::Value;
use std::sync::OnceLock;

const C11_3_1_GRID_REPEAT_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/grid-template-repeat-c1131-runtime-matrix-v1.json"
);
const C11_3_1_GRID_REPEAT_SCENE_IDS: [&str; 1] = ["multi-track-repeat"];

pub(crate) fn c11_3_1_grid_repeat_runtime_source(scene_id: &str) -> Result<&'static str, String> {
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_3_1_GRID_REPEAT_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.3.1 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str() != Some("spinon-css-c11-3-1-grid-repeat-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeBlockFlexGridTemplateV1")
            {
                return Err("C11.3.1 runtime matrix schema/profile이 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.3.1 runtime matrix 장면 목록이 없습니다".to_owned())?;
            if cases.len() != C11_3_1_GRID_REPEAT_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(1)
                || matrix["summary"]["nodeCount"].as_u64() != Some(3)
                || matrix["summary"]["successCount"].as_u64() != Some(1)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(0)
            {
                return Err("C11.3.1 runtime matrix 요약이 고정 계약과 다릅니다".to_owned());
            }
            let case = &cases[0];
            let node_ids = case["nodeIds"]
                .as_array()
                .ok_or_else(|| "C11.3.1 runtime matrix nodeIds가 없습니다".to_owned())?;
            let parent_ids = case["parentIds"]
                .as_array()
                .ok_or_else(|| "C11.3.1 runtime matrix parentIds가 없습니다".to_owned())?;
            let source = case["source"]
                .as_str()
                .ok_or_else(|| "C11.3.1 runtime matrix source가 없습니다".to_owned())?;
            if case["index"].as_u64() != Some(0)
                || case["id"].as_str() != Some(C11_3_1_GRID_REPEAT_SCENE_IDS[0])
                || case["expected"]["outcome"].as_str() != Some("success")
                || case["rootId"].as_str() != node_ids.first().and_then(Value::as_str)
                || node_ids.len() != 3
                || parent_ids.len() != node_ids.len()
                || node_ids.iter().any(|value| value.as_str().is_none())
                || source.is_empty()
                || source.len() > 16 * 1024
                || !source.contains("repeat(2,40px 1fr)")
            {
                return Err("C11.3.1 runtime matrix 장면이 고정 계약과 다릅니다".to_owned());
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
        .ok_or_else(|| format!("알 수 없는 C11.3.1 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.3.1 scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{C11_3_1_GRID_REPEAT_RUNTIME_MATRIX, c11_3_1_grid_repeat_runtime_source};

    #[test]
    fn fixed_runtime_matrix_exposes_only_the_pinned_repeat_scene() {
        let source = c11_3_1_grid_repeat_runtime_source("multi-track-repeat").unwrap();
        assert!(source.contains("repeat(2,40px 1fr)"));
        assert!(source.contains("document.createElement"));
        assert!(c11_3_1_grid_repeat_runtime_source("unknown").is_err());
        assert!(C11_3_1_GRID_REPEAT_RUNTIME_MATRIX.contains("\"nodeCount\": 3"));
    }
}
