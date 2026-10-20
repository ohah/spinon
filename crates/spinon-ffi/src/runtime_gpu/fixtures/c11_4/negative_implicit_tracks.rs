const SCENES_JSON: &str = include_str!("negative_implicit_track_scenes.json");

const SCENE_IDS: [&str; 33] = [
    "columns-one-negative",
    "columns-two-negative",
    "columns-three-negative",
    "columns-four-negative",
    "columns-negative-and-positive-cycle",
    "columns-negative-fixed-single",
    "columns-zero-track-cycle",
    "columns-explicit-boundary-span",
    "columns-reversed-lines-normalize",
    "columns-negative-line-span",
    "columns-no-explicit-tracks",
    "columns-negative-minmax-track",
    "columns-negative-min-content-track",
    "columns-negative-max-content-track",
    "rows-one-negative",
    "rows-two-negative",
    "rows-three-negative",
    "rows-negative-and-positive-cycle",
    "both-axes-negative-independent-patterns",
    "hidden-negative-does-not-expand",
    "negative-column-auto-row-placement",
    "columns-negative-grid-auto-items",
    "rows-negative-grid-auto-items",
    "columns-negative-row-dense-auto-items",
    "columns-negative-row-sparse-cursor",
    "columns-negative-row-dense-cursor",
    "columns-negative-column-flow-auto-items",
    "columns-negative-column-dense-auto-items",
    "rows-negative-row-dense-auto-items",
    "rows-negative-column-flow-auto-items",
    "rows-negative-column-dense-auto-items",
    "columns-negative-fixed-root-overflow",
    "rows-negative-fixed-root-overflow",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENE_IDS.contains(&scene_id).then(|| {
        format!(
            r#"(() => {{
const scenes = JSON.parse({SCENES_JSON:?});
const scene = scenes.find(candidate => candidate.id === {scene_id:?});
if (!scene) throw new Error('unknown negative implicit track scene');
const rootId = `c114-${{scene.id}}-grid`;
const root = document.createElement('div');
root.setAttribute('id', rootId);
root.setAttribute('data-node', '');
root.setAttribute('style', [
  'box-sizing:border-box', 'display:grid', `width:${{scene.width || 'max-content'}}`,
  `height:${{scene.height || 'max-content'}}`, 'margin:0', 'padding:0',
  'border:1px solid #111827', 'background-color:#e2e8f0', 'color:#152238',
  'font-family:Ahem', 'font-size:12px', 'font-weight:400',
  'font-style:normal', 'line-height:12px',
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  `grid-auto-columns:${{scene.autoColumns}}`, `grid-auto-rows:${{scene.autoRows}}`,
  `column-gap:${{scene.columnGap || '3px'}}`, `row-gap:${{scene.rowGap || '3px'}}`,
  `grid-auto-flow:${{scene.autoFlow || 'row'}}`,
].join(';'));
document.appendChild(root);
const nodeIds = [rootId];
const itemColors = ['#e63946', '#2a9d8f', '#457b9d', '#f4a261', '#8d5fd3', '#e76f51'];
let firstVisibleItemRecorded = false;
for (const [index, placement] of scene.items.entries()) {{
  const itemId = `c114-${{scene.id}}-item-${{index + 1}}`;
  const item = document.createElement('div');
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block', 'width:auto', 'height:auto',
    'min-width:0', 'min-height:0', 'margin:0', 'padding:0', 'border:0',
    `background-color:${{itemColors[index % itemColors.length]}}`,
    placement.c ? `grid-column:${{placement.c}}` : '',
    placement.r ? `grid-row:${{placement.r}}` : '',
    placement.hidden ? 'display:none' : '',
  ].filter(Boolean).join(';'));
  if (placement.text) item.appendChild(document.createTextNode(placement.text));
  root.appendChild(item);
  if (!placement.hidden && !firstVisibleItemRecorded) {{
    nodeIds.push(itemId);
    firstVisibleItemRecorded = true;
  }}
}}
return {{ sceneId:scene.id, rootId, nodeIds }};
}})()"#
        )
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{SCENE_IDS, SCENES_JSON, runtime_source};

    #[test]
    fn exposes_all_negative_implicit_track_runtime_scenes() {
        let unique_ids: BTreeSet<_> = SCENE_IDS.into_iter().collect();
        assert_eq!(SCENE_IDS.len(), 33);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for scene_id in SCENE_IDS {
            assert!(SCENES_JSON.contains(&format!("\"id\": \"{scene_id}\"")));
            let source = runtime_source(scene_id).expect("listed scene must have a source");
            assert!(source.len() < 24 * 1024, "{scene_id} source is too large");
            assert!(source.contains(&format!("candidate.id === {scene_id:?}")));
            assert!(source.contains("grid-auto-columns"));
            assert!(source.contains("grid-auto-rows"));
        }
        assert!(runtime_source("unknown-negative-scene").is_none());
    }
}
