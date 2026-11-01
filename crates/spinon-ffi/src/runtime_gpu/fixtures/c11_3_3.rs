use serde_json::Value;
use std::sync::OnceLock;

#[path = "c11_3_3_fit_content_area_cap.rs"]
mod fit_content_area_cap;

const C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v2.json"
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
const C11_3_3_UNKNOWN_AREA_FIXED_IMPLICIT_TRACKS_SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'fixed-implicit-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', "box-sizing:border-box;display:grid;width:160px;height:120px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:80px 80px;grid-template-rows:60px 60px;grid-template-areas:'known .' '. .';grid-auto-rows:5px;grid-auto-columns:15px;grid-auto-flow:row");
document.appendChild(root);
const known = document.createElement('div');
known.setAttribute('id', 'fixed-known-area');
known.setAttribute('data-node', '');
known.setAttribute('style', 'box-sizing:border-box;width:80px;height:60px;margin:0;padding:0;border:0;background-color:#30a46c;grid-area:known');
root.appendChild(known);
const item = document.createElement('div');
item.setAttribute('id', 'fixed-unknown-area');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:15px;height:5px;margin:0;padding:0;border:0;background-color:#e5484d;grid-area:not-authored');
root.appendChild(item);
return { sceneId:'unknown-area-fixed-implicit-tracks', rootId:'fixed-implicit-grid', nodeIds:['fixed-implicit-grid', 'fixed-known-area', 'fixed-unknown-area'] };
})()"#;
const C11_3_3_NAMED_LINE_AUTO_IMPLICIT_TRACKS_SOURCES: [(&str, &str); 3] = [
    (
        "named-line-auto-columns",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'auto-columns-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:80px;height:20px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;grid-auto-flow:row');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'auto-columns-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:30px;height:10px;margin:0;padding:0;border:0;background-color:#e5484d;grid-column-start:missing-column;grid-row:1');
root.appendChild(item);
return { sceneId:'named-line-auto-columns', rootId:'auto-columns-grid', nodeIds:['auto-columns-grid','auto-columns-item'] };
})()"#,
    ),
    (
        "named-line-fixed-columns-control",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'auto-columns-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:80px;height:20px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:30px;grid-auto-flow:row');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'auto-columns-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:30px;height:10px;margin:0;padding:0;border:0;background-color:#e5484d;grid-column-start:missing-column;grid-row:1');
root.appendChild(item);
return { sceneId:'named-line-fixed-columns-control', rootId:'auto-columns-grid', nodeIds:['auto-columns-grid','auto-columns-item'] };
})()"#,
    ),
    (
        "named-line-auto-rows",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'auto-rows-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:20px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;grid-auto-flow:row');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'auto-rows-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:20px;height:25px;margin:0;padding:0;border:0;background-color:#e5484d;grid-column:1;grid-row-start:missing-row');
root.appendChild(item);
return { sceneId:'named-line-auto-rows', rootId:'auto-rows-grid', nodeIds:['auto-rows-grid','auto-rows-item'] };
})()"#,
    ),
];
const C11_3_3_POSITIVE_NAMED_LINE_PAIR_SOURCES: [(&str, &str); 2] = [
    (
        "named-line-pair-columns-multi-track",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'named-pair-columns-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:80px;height:20px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;grid-auto-flow:row');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'named-pair-columns-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:30px;height:10px;margin:0;padding:0;border:0;background-color:#e5484d;grid-column:1 alpha / 3 omega;grid-row:1');
root.appendChild(item);
return { sceneId:'named-line-pair-columns-multi-track', rootId:'named-pair-columns-grid', nodeIds:['named-pair-columns-grid','named-pair-columns-item'] };
})()"#,
    ),
    (
        "named-line-pair-rows-multi-track",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'named-pair-rows-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:40px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:40px;grid-template-rows:20px 20px;grid-auto-rows:auto;grid-auto-columns:auto;grid-auto-flow:row');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'named-pair-rows-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:20px;height:30px;margin:0;padding:0;border:0;background-color:#e5484d;grid-column:1;grid-row:1 alpha / 3 omega');
root.appendChild(item);
return { sceneId:'named-line-pair-rows-multi-track', rootId:'named-pair-rows-grid', nodeIds:['named-pair-rows-grid','named-pair-rows-item'] };
})()"#,
    ),
];
const C11_3_3_NEGATIVE_NAMED_LINE_PAIR_MATRIX_SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'negative-named-pair-matrix-grid');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:320px;height:240px;margin:0;padding:0;border:0;gap:0;grid-template-columns:160px 160px;grid-template-rows:60px 60px 60px 60px;grid-auto-flow:row');
document.appendChild(root);
const cases = [
  { id:'columns-negative-span', root:'negative-pair-span-grid', item:'negative-pair-span-item', width:160, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:17, itemHeight:10, column:'-4 ghost / -1 ghost', row:'1 / 2' },
  { id:'columns-negative-reversed', root:'negative-pair-reversed-grid', item:'negative-pair-reversed-item', width:160, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:17, itemHeight:10, column:'-1 ghost / -4 ghost', row:'1 / 2' },
  { id:'columns-negative-one', root:'negative-pair-one-grid', item:'negative-pair-one-item', width:160, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:17, itemHeight:10, column:'-2 ghost / -1 ghost', row:'1 / 2' },
  { id:'columns-negative-equal', root:'negative-pair-equal-grid', item:'negative-pair-equal-item', width:160, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:17, itemHeight:10, column:'-2 ghost / -2 ghost', row:'1 / 2' },
  { id:'columns-negative-offset', root:'negative-pair-offset-grid', item:'negative-pair-offset-item', width:160, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:17, itemHeight:10, column:'-5 ghost / -2 ghost', row:'1 / 2' },
  { id:'columns-negative-auto', root:'negative-pair-auto-columns-grid', item:'negative-pair-auto-columns-item', width:108, height:30, columns:'20px 30px 40px', rows:'10px 10px 10px', autoColumns:'auto', autoRows:'5px 7px', itemWidth:18, itemHeight:10, column:'-4 ghost / -1 ghost', row:'1 / 2' },
  { id:'rows-negative-span', root:'negative-pair-rows-grid', item:'negative-pair-rows-item', width:160, height:54, columns:'20px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'5px 7px', itemWidth:10, itemHeight:17, column:'1 / 2', row:'-4 ghost / -1 ghost' },
  { id:'rows-negative-auto', root:'negative-pair-auto-rows-grid', item:'negative-pair-auto-rows-item', width:160, height:48, columns:'20px', rows:'10px 10px 10px', autoColumns:'5px 7px', autoRows:'auto', itemWidth:10, itemHeight:18, column:'1 / 2', row:'-4 ghost / -1 ghost' },
];
const nodeIds = [];
for (const scene of cases) {
  const grid = document.createElement('div');
  grid.setAttribute('id', scene.root);
  grid.setAttribute('data-node', '');
  grid.setAttribute('style', `box-sizing:border-box;display:grid;width:${scene.width}px;height:${scene.height}px;margin:0;padding:0;border:0;gap:0;background-color:#101820;grid-template-columns:${scene.columns};grid-template-rows:${scene.rows};grid-auto-rows:${scene.autoRows};grid-auto-columns:${scene.autoColumns};grid-auto-flow:row`);
  root.appendChild(grid);
  const item = document.createElement('div');
  item.setAttribute('id', scene.item);
  item.setAttribute('data-node', '');
  item.setAttribute('style', `box-sizing:border-box;width:${scene.itemWidth}px;height:${scene.itemHeight}px;margin:0;padding:0;border:0;background-color:#2866f6;grid-column:${scene.column};grid-row:${scene.row}`);
  grid.appendChild(item);
  nodeIds.push(scene.root, scene.item);
}
return { sceneId:'negative-named-line-pair-matrix', rootId:cases[0].root, nodeIds };
})()"#;
const C11_3_3_LEADING_NAMED_IMPLICIT_LINE_SOURCES: [(&str, &str); 7] = [
    (
        "positive-missing-occurrences",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-positive-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:114px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:[known-start] 30px 40px [known-end];grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-positive-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:3 ghost / 4 ghost;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'positive-missing-occurrences', rootId:'leading-positive-grid', nodeIds:['leading-positive-grid','leading-positive-item'] };
})()"#,
    ),
    (
        "negative-missing-occurrences",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-negative-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:92px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:[known-start] 30px 40px [known-end];grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-negative-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:-1 ghost / -2 ghost;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'negative-missing-occurrences', rootId:'leading-negative-grid', nodeIds:['leading-negative-grid','leading-negative-item'] };
})()"#,
    ),
    (
        "backward-span-one",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-backward-one-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:129px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:100px;grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-backward-one-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:span ghost / 4;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'backward-span-one', rootId:'leading-backward-one-grid', nodeIds:['leading-backward-one-grid','leading-backward-one-item'] };
})()"#,
    ),
    (
        "backward-span-two",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-backward-two-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:144px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:100px;grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-backward-two-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:span 2 ghost / 4;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'backward-span-two', rootId:'leading-backward-two-grid', nodeIds:['leading-backward-two-grid','leading-backward-two-item'] };
})()"#,
    ),
    (
        "backward-span-partial-explicit",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-backward-partial-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:162px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:[ghost] 40px [other] 100px;grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-backward-partial-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:span 2 ghost / 4;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'backward-span-partial-explicit', rootId:'leading-backward-partial-grid', nodeIds:['leading-backward-partial-grid','leading-backward-partial-item'] };
})()"#,
    ),
    (
        "forward-span-two",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-forward-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:94px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:50px;grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-forward-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:4 / span 2 ghost;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'forward-span-two', rootId:'leading-forward-grid', nodeIds:['leading-forward-grid','leading-forward-item'] };
})()"#,
    ),
    (
        "backward-span-uses-authored-name",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'leading-authored-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:85px;height:20px;margin:0;padding:0;border:0;gap:0;grid-template-columns:30px [ghost] 40px;grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'leading-authored-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:span 1 ghost / 4;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(item);
return { sceneId:'backward-span-uses-authored-name', rootId:'leading-authored-grid', nodeIds:['leading-authored-grid','leading-authored-item'] };
})()"#,
    ),
];
const C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS: [&str; 39] = [
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
    "area-auto-tracks-none",
    "area-auto-tracks-short-row",
    "area-auto-tracks-short-col",
    "area-auto-tracks-auto",
    "area-auto-tracks-long",
    "both-axis-cycle",
    "row-cycle-from-first-implicit",
    "column-cycle-from-first-implicit",
    "authored-tracks-exceed-area",
    "row-cycle-continues-after-area",
    "column-cycle-continues-after-area",
    "typed-area-minmax-row",
    "typed-area-percentage-column",
];

pub(crate) fn c11_3_3_grid_template_areas_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    if scene_id == "negative-named-line-pair-matrix" {
        return Ok(C11_3_3_NEGATIVE_NAMED_LINE_PAIR_MATRIX_SOURCE);
    }
    if scene_id == "fit-content-area-cap" {
        return Ok(fit_content_area_cap::SOURCE);
    }
    if scene_id == "revision-reuse" {
        return Ok(C11_3_3_GRID_TEMPLATE_AREAS_REVISION_REUSE_SOURCE);
    }
    if scene_id == "unknown-area-fixed-implicit-tracks" {
        return Ok(C11_3_3_UNKNOWN_AREA_FIXED_IMPLICIT_TRACKS_SOURCE);
    }
    if let Some((_, source)) = C11_3_3_POSITIVE_NAMED_LINE_PAIR_SOURCES
        .iter()
        .find(|(id, _)| *id == scene_id)
    {
        return Ok(source);
    }
    if let Some((_, source)) = C11_3_3_NAMED_LINE_AUTO_IMPLICIT_TRACKS_SOURCES
        .iter()
        .find(|(id, _)| *id == scene_id)
    {
        return Ok(source);
    }
    if let Some((_, source)) = C11_3_3_LEADING_NAMED_IMPLICIT_LINE_SOURCES
        .iter()
        .find(|(id, _)| *id == scene_id)
    {
        return Ok(source);
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
                || matrix["summary"]["sceneCount"].as_u64() != Some(39)
                || matrix["summary"]["nodeCount"].as_u64() != Some(126)
                || matrix["summary"]["successCount"].as_u64() != Some(34)
                || matrix["summary"]["cascadeFallbackCount"].as_u64() != Some(5)
                || matrix["summary"]["expectedRejectCount"].as_u64() != Some(0)
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
                        let node_id = expected["nodeId"].as_str();
                        let node_ids = case["nodeIds"].as_array();
                        if expected["code"].as_str().is_none()
                            || expected["property"].as_str().is_none()
                            || node_id.is_none()
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
                    || (!(source.contains("grid-template-columns:")
                        || source.contains("grid-template:")
                        || source.contains("grid:"))
                        && !(source.contains("grid-template-areas:")
                            && (source.contains("grid-auto-columns:")
                                || source.contains("grid-auto-rows:"))))
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
            if success_count != 34
                || cascade_fallback_count != 5
                || rejection_count != 0
                || node_count != 126
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
        assert!(C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX.contains("\"sceneCount\": 39"));
        assert!(C11_3_3_GRID_TEMPLATE_AREAS_RUNTIME_MATRIX.contains("\"nodeCount\": 126"));
        assert!(c11_3_3_grid_template_areas_runtime_source("unknown-area-implicit-tracks").is_ok());
    }

    #[test]
    fn area_fit_content_cap_uses_a_separate_fixed_source() {
        let source = c11_3_3_grid_template_areas_runtime_source("fit-content-area-cap")
            .expect("isolated area fit-content cap scene");
        for expected in [
            "c1133-fit-content-area-cap-grid",
            "grid-template-areas:\"fixed fit\"",
            "grid-auto-columns:fit-content(40px)",
            "document.createTextNode('AA AA AA AA')",
        ] {
            assert!(
                source.contains(expected),
                "missing fixed source token: {expected}"
            );
        }
        assert!(!C11_3_3_GRID_TEMPLATE_AREAS_SCENE_IDS.contains(&"fit-content-area-cap"));
        assert!(
            c11_3_3_grid_template_areas_runtime_source("fit-content-area-cap;throw 1").is_err()
        );
    }

    #[test]
    fn revision_reuse_source_mutates_the_area_on_a_reused_runtime_host() {
        let source = c11_3_3_grid_template_areas_runtime_source("revision-reuse").unwrap();
        assert!(source.contains("globalThis[key]"));
        assert!(source.contains("grid-template-areas:'item .' '. .'"));
        assert!(source.contains("grid-template-areas:'. item' '. .'"));
        assert!(source.contains("revision-item"));
    }

    #[test]
    fn fixed_implicit_area_scene_keeps_area_sibling_and_overflow_target() {
        let source =
            c11_3_3_grid_template_areas_runtime_source("unknown-area-fixed-implicit-tracks")
                .unwrap();
        assert!(source.contains("grid-auto-rows:5px"));
        assert!(source.contains("grid-auto-columns:15px"));
        assert!(source.contains("grid-area:not-authored"));
        assert!(source.contains("fixed-known-area"));
        assert!(source.contains("fixed-unknown-area"));
    }

    #[test]
    fn positive_named_line_auto_scenes_have_separate_sources_and_fixed_control() {
        let expected = [
            (
                "named-line-auto-columns",
                "auto-columns-grid",
                "auto-columns-item",
                "grid-auto-columns:auto",
            ),
            (
                "named-line-fixed-columns-control",
                "auto-columns-grid",
                "auto-columns-item",
                "grid-auto-columns:30px",
            ),
            (
                "named-line-auto-rows",
                "auto-rows-grid",
                "auto-rows-item",
                "grid-auto-rows:auto",
            ),
        ];

        for (scene_id, root_id, item_id, track_style) in expected {
            let source = c11_3_3_grid_template_areas_runtime_source(scene_id).unwrap();
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains(&format!("rootId:'{root_id}'")));
            assert!(source.contains(&format!("'{root_id}','{item_id}'")));
            assert!(source.contains(track_style));
            assert!(
                source.contains("grid-column-start:missing-column")
                    || source.contains("grid-row-start:missing-row")
            );
            assert!(source.contains("background-color:#e5484d"));
        }
        assert!(
            c11_3_3_grid_template_areas_runtime_source("named-line-auto-columns;throw 1").is_err()
        );
    }

    #[test]
    fn positive_named_line_pair_scenes_are_allowlisted_for_both_axes() {
        let expected = [
            (
                "named-line-pair-columns-multi-track",
                "named-pair-columns-grid",
                "named-pair-columns-item",
                "grid-column:1 alpha / 3 omega",
            ),
            (
                "named-line-pair-rows-multi-track",
                "named-pair-rows-grid",
                "named-pair-rows-item",
                "grid-row:1 alpha / 3 omega",
            ),
        ];

        for (scene_id, root_id, item_id, placement) in expected {
            let source = c11_3_3_grid_template_areas_runtime_source(scene_id).unwrap();
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains(&format!("rootId:'{root_id}'")));
            assert!(source.contains(&format!("'{root_id}','{item_id}'")));
            assert!(source.contains(placement));
            assert!(source.contains("grid-auto-rows:auto;grid-auto-columns:auto"));
            assert!(source.contains("background-color:#e5484d"));
        }
        assert!(
            c11_3_3_grid_template_areas_runtime_source(
                "named-line-pair-columns-multi-track;throw 1"
            )
            .is_err()
        );
    }

    #[test]
    fn negative_named_line_pair_matrix_has_all_fixed_and_auto_leading_cases() {
        let source =
            c11_3_3_grid_template_areas_runtime_source("negative-named-line-pair-matrix").unwrap();
        for expected in [
            "columns-negative-span",
            "columns-negative-reversed",
            "columns-negative-one",
            "columns-negative-equal",
            "columns-negative-offset",
            "columns-negative-auto",
            "rows-negative-span",
            "rows-negative-auto",
            "autoColumns:'auto'",
            "autoRows:'auto'",
            "column:'-4 ghost / -1 ghost'",
            "row:'-4 ghost / -1 ghost'",
            "background-color:#2866f6",
        ] {
            assert!(
                source.contains(expected),
                "missing runtime token: {expected}"
            );
        }
        assert!(
            c11_3_3_grid_template_areas_runtime_source("negative-named-line-pair-matrix;throw 1")
                .is_err()
        );
    }

    #[test]
    fn leading_named_implicit_scenes_are_allowlisted_and_match_pixel_fixture_contract() {
        let expected = [
            (
                "positive-missing-occurrences",
                "leading-positive-grid",
                "leading-positive-item",
                114,
            ),
            (
                "negative-missing-occurrences",
                "leading-negative-grid",
                "leading-negative-item",
                92,
            ),
            (
                "backward-span-one",
                "leading-backward-one-grid",
                "leading-backward-one-item",
                129,
            ),
            (
                "backward-span-two",
                "leading-backward-two-grid",
                "leading-backward-two-item",
                144,
            ),
            (
                "backward-span-partial-explicit",
                "leading-backward-partial-grid",
                "leading-backward-partial-item",
                162,
            ),
            (
                "forward-span-two",
                "leading-forward-grid",
                "leading-forward-item",
                94,
            ),
            (
                "backward-span-uses-authored-name",
                "leading-authored-grid",
                "leading-authored-item",
                85,
            ),
        ];

        for (scene_id, root_id, item_id, root_width) in expected {
            let source = c11_3_3_grid_template_areas_runtime_source(scene_id).unwrap();
            assert!(
                source.contains(&format!("sceneId:'{scene_id}'")),
                "{scene_id}"
            );
            assert!(
                source.contains(&format!("rootId:'{root_id}'")),
                "{scene_id}"
            );
            assert!(
                source.contains(&format!("'{root_id}','{item_id}'")),
                "{scene_id}"
            );
            assert!(
                source.contains(&format!("width:{root_width}px")),
                "{scene_id}"
            );
            assert!(source.contains("background-color:#2866f6"), "{scene_id}");
            assert!(source.contains("grid-auto-columns:15px 7px"), "{scene_id}");
        }

        assert!(
            c11_3_3_grid_template_areas_runtime_source("positive-missing-occurrences;throw 1")
                .is_err()
        );
    }
}
