use serde_json::Value;
use std::sync::OnceLock;

const C11_1_ROOT_HTML_DENSE_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-dense-auto-placement-runtime-matrix-v1.json"
);
const C11_1_ROOT_HTML_DENSE_SCENE_IDS: [&str; 14] = [
    "row-dense-hole-fill",
    "row-dense-span-collision",
    "row-dense-fixed-row",
    "row-dense-fixed-column",
    "row-dense-positive-implicit-row",
    "column-dense-hole-fill",
    "column-dense-span-collision",
    "column-dense-positive-implicit-column",
    "definite-first-dense",
    "hidden-item-excluded",
    "nested-mixed-flow",
    "auto-track-no-implicit",
    "auto-span-at-track-limit",
    "auto-span-over-track-limit",
];

pub(crate) fn c11_1_root_html_dense_placement_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_1_ROOT_HTML_DENSE_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.1 Root HTML dense matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-root-html-dense-auto-placement-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeRootHtmlTextPaintV1")
            {
                return Err("C11.1 Root HTML dense matrix schema/profile이 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.1 Root HTML dense matrix scene 목록이 없습니다".to_owned())?;
            if cases.len() != C11_1_ROOT_HTML_DENSE_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(14)
                || matrix["summary"]["implementationCandidateCount"].as_u64() != Some(14)
                || matrix["summary"]["successCount"].as_u64() != Some(12)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(2)
                || matrix["summary"]["nodeCount"].as_u64() != Some(60)
            {
                return Err("C11.1 Root HTML dense matrix 요약이 고정 계약과 다릅니다".to_owned());
            }

            let mut total_nodes = 0;
            let mut successes = 0;
            let mut expected_rejections = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.1 Root HTML dense scene {index} ID 누락"))?;
                if id != C11_1_ROOT_HTML_DENSE_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.1 Root HTML dense scene 순서 또는 ID 불일치: index={index}, id={id}"
                    ));
                }
                match case["expected"]["outcome"].as_str() {
                    Some("success") => successes += 1,
                    Some("expected-rejection") => {
                        expected_rejections += 1;
                        if case["expected"]["code"].as_str().is_none()
                            || case["expected"]["property"].as_str().is_none()
                            || case["expected"]["nodeId"].as_str().is_none()
                        {
                            return Err(format!(
                                "C11.1 Root HTML dense scene {id} 거부 계약이 불완전합니다"
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "C11.1 Root HTML dense scene {id} outcome이 잘못되었습니다"
                        ));
                    }
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.1 Root HTML dense scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.1 Root HTML dense scene {id} parentIds 누락"))?;
                if node_ids.is_empty()
                    || node_ids.len() != parent_ids.len()
                    || case["rootId"].as_str() != node_ids.first().and_then(Value::as_str)
                {
                    return Err(format!(
                        "C11.1 Root HTML dense scene {id} node identity가 잘못되었습니다"
                    ));
                }
                total_nodes += node_ids.len();
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.1 Root HTML dense scene {id} source 누락"))?;
                if source.is_empty()
                    || source.len() > spinon_runtime::ROOT_HTML_SOURCE_MAX_BYTES
                    || !source.contains("<html")
                    || source.contains("<script")
                {
                    return Err(format!(
                        "C11.1 Root HTML dense scene {id} source가 허용 범위를 벗어났습니다"
                    ));
                }
            }
            if total_nodes != 60 || successes != 12 || expected_rejections != 2 {
                return Err("C11.1 Root HTML dense 개별 장면과 요약이 일치하지 않습니다".to_owned());
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
        .ok_or_else(|| format!("알 수 없는 C11.1 Root HTML dense scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.1 Root HTML dense scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{C11_1_ROOT_HTML_DENSE_SCENE_IDS, c11_1_root_html_dense_placement_source};

    #[test]
    fn fixed_matrix_selects_only_the_fourteen_pinned_dense_scenes() {
        for scene_id in C11_1_ROOT_HTML_DENSE_SCENE_IDS {
            let source = c11_1_root_html_dense_placement_source(scene_id).unwrap();
            assert!(source.contains(&format!("data-scene=\"{scene_id}\"")));
        }
        assert!(c11_1_root_html_dense_placement_source("row-sparse-baseline").is_err());
        assert!(c11_1_root_html_dense_placement_source("unknown;throw 1").is_err());
        assert!(c11_1_root_html_dense_placement_source("").is_err());
    }
}
