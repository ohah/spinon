use serde_json::Value;
use std::sync::OnceLock;

const C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json"
);
const C11_3_3_GRID_TEMPLATE_AREAS_REVISION_REUSE_SOURCE: &str = r#"(() => {
const key = '__spinonC1133RevisionReuseV1';
let state = globalThis[key];
const baseStyle = 'box-sizing:border-box;display:grid;width:160px;height:80px;margin:0;padding:0;border:0;gap:0;grid-template-columns:80px 80px;grid-template-rows:40px 40px;background-color:#101820;';
if (!state) {
  const root = document.createElement('div');
  root.setAttribute('id', 'revision-grid');
  root.setAttribute('data-node', '');
  root.setAttribute('style', baseStyle + "grid-template-areas:'item .' '. .'");
  document.appendChild(root);
  const item = document.createElement('div');
  item.setAttribute('id', 'revision-item');
  item.setAttribute('data-node', '');
  item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#3e63dd;grid-area:item');
  root.appendChild(item);
  state = { root, phase: 1 };
  globalThis[key] = state;
} else {
  state.root.setAttribute('style', baseStyle + "grid-template-areas:'. item' '. .'");
  state.phase += 1;
}
return { sceneId:'revision-reuse', rootId:'revision-grid', nodeIds:['revision-grid', 'revision-item'], phase:state.phase };
})()"#;
const C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS: [&str; 26] = [
    "basic-rectangles",
    "null-cell-dot-runs",
    "all-null-cells",
    "single-cell-area",
    "bare-area-edge-aliases",
    "grid-area-one-value",
    "grid-area-two-values",
    "grid-area-three-values",
    "grid-area-four-values",
    "row-column-longhands-area",
    "grid-template-shorthand",
    "grid-shorthand",
    "bare-area-vs-explicit-one-name",
    "explicit-line-before-area",
    "explicit-line-after-area",
    "area-names-ending-start-or-end",
    "implicit-area-from-explicit-lines",
    "escaped-and-unicode-identifiers",
    "invalid-l-shape-fallback",
    "invalid-disconnected-area-fallback",
    "invalid-row-width-fallback",
    "invalid-trash-token-fallback",
    "empty-string-fallback",
    "area-track-count-mismatch",
    "unknown-area-implicit-tracks",
    "nested-hidden-source-order",
];

pub(crate) fn c11_3_3_grid_template_areas_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    if scene_id == "revision-reuse" {
        return Ok(C11_3_3_GRID_TEMPLATE_AREAS_REVISION_REUSE_SOURCE);
    }
    static MATRIX: OnceLock<Result<Value, String>> = OnceLock::new();
    let matrix = MATRIX
        .get_or_init(|| {
            let matrix: Value = serde_json::from_str(C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX)
                .map_err(|error| format!("C11.3.3 runtime matrix JSON 오류: {error}"))?;
            if matrix["schema"].as_str()
                != Some("spinon-css-c11-3-3-grid-template-areas-runtime-matrix/v1")
                || matrix["profileId"].as_str() != Some("RuntimeBlockFlexGridTemplateV1")
            {
                return Err("C11.3.3 runtime matrix schema/profile이 다릅니다".to_owned());
            }
            let cases = matrix["cases"]
                .as_array()
                .ok_or_else(|| "C11.3.3 runtime matrix 장면 목록이 없습니다".to_owned())?;
            if cases.len() != C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS.len()
                || matrix["summary"]["sceneCount"].as_u64() != Some(26)
                || matrix["summary"]["nodeCount"].as_u64() != Some(65)
                || matrix["summary"]["successCount"].as_u64() != Some(19)
                || matrix["summary"]["cascadeFallbackCount"].as_u64() != Some(5)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(2)
            {
                return Err("C11.3.3 runtime matrix 요약이 고정 계약과 다릅니다".to_owned());
            }

            let mut success_count = 0;
            let mut cascade_fallback_count = 0;
            let mut rejection_count = 0;
            let mut node_count = 0;
            for (index, case) in cases.iter().enumerate() {
                let id = case["id"]
                    .as_str()
                    .ok_or_else(|| format!("C11.3.3 scene {index} ID 누락"))?;
                if id != C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS[index]
                    || case["index"].as_u64() != Some(index as u64)
                {
                    return Err(format!(
                        "C11.3.3 scene 순서 또는 ID 불일치: index={index}, id={id}"
                    ));
                }
                let expected = &case["expected"];
                match expected["outcome"].as_str() {
                    Some("success") => success_count += 1,
                    Some("cascade-fallback") => {
                        cascade_fallback_count += 1;
                        if expected["runtimeOutcome"].as_str() != Some("success") {
                            return Err(format!(
                                "C11.3.3 scene {id} cascade fallback runtime 결과가 잘못되었습니다"
                            ));
                        }
                    }
                    Some("expected-rejection") => {
                        rejection_count += 1;
                        let contract = match id {
                            "area-track-count-mismatch" => {
                                Some(("unsupported_computed_value", "grid-template-areas"))
                            }
                            "unknown-area-implicit-tracks" => {
                                Some(("unsupported_named_grid_line", "grid-row-start"))
                            }
                            _ => None,
                        };
                        let node_id = expected["nodeId"].as_str();
                        let node_ids = case["nodeIds"].as_array();
                        if contract.is_none_or(|(code, property)| {
                            expected["code"].as_str() != Some(code)
                                || expected["property"].as_str() != Some(property)
                        }) || node_id.is_none()
                            || !node_ids
                                .is_some_and(|ids| ids.iter().any(|node| node.as_str() == node_id))
                        {
                            return Err(format!("C11.3.3 scene {id} 거부 계약이 불완전합니다"));
                        }
                    }
                    _ => return Err(format!("C11.3.3 scene {id} outcome이 잘못되었습니다")),
                }
                let node_ids = case["nodeIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.3.3 scene {id} nodeIds 누락"))?;
                let parent_ids = case["parentIds"]
                    .as_array()
                    .ok_or_else(|| format!("C11.3.3 scene {id} parentIds 누락"))?;
                if node_ids.is_empty()
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
                    || parent_ids
                        .iter()
                        .enumerate()
                        .skip(1)
                        .any(|(index, parent)| {
                            parent.as_str().is_none_or(|parent_id| {
                                !node_ids[..index]
                                    .iter()
                                    .any(|node| node.as_str() == Some(parent_id))
                            })
                        })
                {
                    return Err(format!("C11.3.3 scene {id} node identity가 잘못되었습니다"));
                }
                node_count += node_ids.len();
                let source = case["source"]
                    .as_str()
                    .ok_or_else(|| format!("C11.3.3 scene {id} source 누락"))?;
                if source.is_empty()
                    || source.len() > 16 * 1024
                    || !source.contains("document.createElement")
                    || !(source.contains("grid-template-columns:")
                        || source.contains("grid-template:")
                        || source.contains("grid:"))
                    || !(source.contains("grid-template-areas:")
                        || source.contains("grid-template:")
                        || source.contains("grid:"))
                    || !source.contains(&format!("sceneId:'{id}'"))
                    || node_ids
                        .iter()
                        .filter_map(Value::as_str)
                        .any(|node_id| !source.contains(node_id))
                {
                    return Err(format!("C11.3.3 scene {id} source가 고정 계약과 다릅니다"));
                }
            }
            if success_count != 19
                || cascade_fallback_count != 5
                || rejection_count != 2
                || node_count != 65
            {
                return Err("C11.3.3 runtime matrix 개별 장면과 요약이 다릅니다".to_owned());
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
        .ok_or_else(|| format!("알 수 없는 C11.3.3 scene ID: {scene_id}"))?;
    case["source"]
        .as_str()
        .ok_or_else(|| format!("C11.3.3 scene {scene_id} source 누락"))
}

#[cfg(test)]
mod tests {
    use super::{
        C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX, C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS,
        c11_3_3_grid_template_areas_runtime_source,
    };

    #[test]
    fn fixed_runtime_matrix_selects_every_scene_and_rejection_boundary() {
        for id in C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS {
            let source = c11_3_3_grid_template_areas_runtime_source(id).unwrap();
            assert!(source.contains(&format!("sceneId:'{id}'")));
            assert!(source.contains("document.createElement"));
        }
        assert!(c11_3_3_grid_template_areas_runtime_source("unknown;throw 1").is_err());
        assert!(C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX.contains("\"sceneCount\": 26"));
        assert!(C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX.contains("\"nodeCount\": 65"));
        assert!(c11_3_3_grid_template_areas_runtime_source("unknown-area-implicit-tracks").is_ok());
    }

    #[test]
    fn revision_reuse_source_mutates_the_area_on_a_reused_runtime_host() {
        let source = c11_3_3_grid_template_areas_runtime_source("revision-reuse").unwrap();
        assert!(source.contains("globalThis[key]"));
        assert!(source.contains("grid-template-areas:'item .' '. .'"));
        assert!(source.contains("grid-template-areas:'. item' '. .'"));
        assert!(source.contains("revision-item"));
    }
}
