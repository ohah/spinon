const SCENES_JSON: &str = include_str!("auto_repeat_negative_line_scenes.json");

const SCENE_IDS: [&str; 32] = [
    "columns-fill-one-leading",
    "columns-fill-two-repetitions",
    "columns-fill-four-repetitions",
    "columns-fit-one-leading-empty-repeat",
    "columns-fill-two-leading-cycle",
    "columns-fill-leading-span-prefix",
    "columns-fit-negative-and-repeat-occupied",
    "columns-fit-third-repeat-only",
    "columns-fill-negative-positive-cycle",
    "columns-fill-reversed-negative-lines",
    "columns-fill-repeat-only-leading",
    "rows-fill-one-leading",
    "rows-fill-three-repetitions",
    "rows-fill-five-repetitions",
    "rows-fit-one-leading-empty-repeat",
    "rows-fill-two-leading-cycle",
    "both-axes-fit-negative-leading",
    "both-axes-fill-independent-cycles",
    "typed-fixed-cycle",
    "typed-fixed-cycle-three-leading-trailing",
    "typed-mixed-cycle",
    "typed-auto",
    "typed-min-content",
    "typed-max-content",
    "typed-minmax-max-content",
    "typed-fit-content",
    "typed-fraction",
    "typed-percentage",
    "typed-calc",
    "typed-auto-fit-occupied",
    "typed-auto-fit-empty",
    "typed-auto-fit-gutter",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENE_IDS.contains(&scene_id).then(|| {
        format!(
            r#"(() => {{
const scenes = JSON.parse({SCENES_JSON:?});
const scene = scenes.find(candidate => candidate.id === {scene_id:?});
if (!scene) throw new Error('unknown auto-repeat negative line scene');
const rootId = `c114-${{scene.id}}-grid`;
const root = document.createElement('div');
root.setAttribute('id', rootId);
root.setAttribute('data-node', '');
root.setAttribute('style', [
  'box-sizing:border-box', 'display:grid', `width:${{scene.width}}`,
  `height:${{scene.height}}`, 'margin:0', 'padding:0', 'border:0',
  'background-color:#e2e8f0', 'color:#152238', 'font-family:Ahem', 'font-size:12px',
  'font-weight:400', 'font-style:normal', 'line-height:12px',
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  `grid-auto-columns:${{scene.autoColumns}}`, `grid-auto-rows:${{scene.autoRows}}`,
  `column-gap:${{scene.columnGap}}`, `row-gap:${{scene.rowGap}}`,
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
    ...(placement.text ? [
      'font-family:Ahem', 'font-size:12px', 'font-weight:400', 'font-style:normal',
      'line-height:12px', 'white-space-collapse:collapse', 'text-wrap-mode:wrap',
      'word-break:normal', 'direction:ltr', 'line-break:auto', 'text-align:start',
      'overflow-wrap:normal', 'text-transform:none', 'letter-spacing:normal',
      'word-spacing:0px', 'text-indent:0px',
    ] : []),
    placement.minAuto ? 'min-width:auto' : '',
    placement.minAuto ? 'min-height:auto' : '',
    placement.minHeightZero ? 'min-height:0' : '',
    `background-color:${{itemColors[index % itemColors.length]}}`,
    placement.c ? `grid-column:${{placement.c}}` : '',
    placement.r ? `grid-row:${{placement.r}}` : '',
  ].filter(Boolean).join(';'));
  if (placement.text) item.appendChild(document.createTextNode(placement.text));
  root.appendChild(item);
  if (scene.captureAllItems) {{
    nodeIds.push(itemId);
  }} else if (!firstVisibleItemRecorded) {{
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
    fn exposes_all_auto_repeat_negative_line_and_typed_track_runtime_scenes() {
        let unique_ids: BTreeSet<_> = SCENE_IDS.into_iter().collect();
        assert_eq!(SCENE_IDS.len(), 32);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for scene_id in SCENE_IDS {
            assert!(SCENES_JSON.contains(&format!("\"id\": \"{scene_id}\"")));
            let source = runtime_source(scene_id).expect("listed scene must have a source");
            assert!(source.len() < 24 * 1024, "{scene_id} source is too large");
            assert!(source.contains(&format!("candidate.id === {scene_id:?}")));
            assert!(source.contains("grid-auto-columns"));
            assert!(source.contains("grid-auto-rows"));
        }
        assert!(runtime_source("unknown-auto-repeat-negative-line-scene").is_none());
    }
}
