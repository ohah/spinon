const SCENES_JSON: &str = include_str!("c11_4/auto_repeat_zero_fixed_maximum_scenes.json");

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scenes: serde_json::Value = serde_json::from_str(SCENES_JSON).ok()?;
    let scene = scenes.as_array()?.iter().find(|scene| {
        scene
            .get("id")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|id| id == scene_id)
    })?;
    let scene_json = serde_json::to_string(scene).ok()?;
    Some(format!(
        r#"(() => {{
const scene = JSON.parse({scene_json:?});
const rootId = `c114-${{scene.id}}-grid`;
const root = document.createElement('div');
root.setAttribute('id', rootId);
root.setAttribute('data-node', '');
root.setAttribute('style', [
  `box-sizing:${{scene.boxSizing}}`, 'display:grid', `width:${{scene.width}}`,
  `height:${{scene.height}}`, `min-width:${{scene.minWidth}}`,
  `min-height:${{scene.minHeight}}`, `max-width:${{scene.maxWidth}}`,
  `max-height:${{scene.maxHeight}}`, 'margin:0', 'padding:0', `border:${{scene.border}}`,
  'align-content:start', 'justify-content:start', 'align-items:start', 'justify-items:start',
  `column-gap:${{scene.columnGap}}`, `row-gap:${{scene.rowGap}}`,
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  `grid-auto-flow:${{scene.autoFlow}}`, `background-color:${{scene.rootColor}}`,
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block', `width:${{itemSpec.width}}`,
    `height:${{itemSpec.height}}`, `min-width:${{itemSpec.minWidth}}`,
    `min-height:${{itemSpec.minHeight}}`, 'margin:0', 'padding:0', 'border:0',
    `grid-column:${{itemSpec.placement.column}}`, `grid-row:${{itemSpec.placement.row}}`,
    `background-color:${{itemSpec.color}}`,
  ].join(';'));
  root.appendChild(item);
  return itemId;
}});
return {{ sceneId:scene.id, rootId, nodeIds:[rootId, ...itemIds] }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES_JSON, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;
    use serde_json::Value;
    use std::collections::BTreeSet;

    const CHROME_OBSERVATIONS: &str = include_str!(
        "../../../../../spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/chrome-156-observations.json"
    );

    fn track_sizes(serialized: &str) -> Vec<f32> {
        serialized
            .split_whitespace()
            .map(|value| value.strip_suffix("px").unwrap().parse().unwrap())
            .collect()
    }

    #[test]
    fn zero_fixed_maximum_auto_repeat_catalog_matches_chrome_and_routes_exactly() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        assert_eq!(scenes.len(), 24);
        let observations: Value = serde_json::from_str(CHROME_OBSERVATIONS).unwrap();
        let observations = observations["scenes"].as_array().unwrap();
        assert_eq!(observations.len(), 24);

        let scene_ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        let oracle_ids = observations
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(scene_ids.len(), 24);
        assert_eq!(scene_ids, oracle_ids);

        for (scene, oracle) in scenes.iter().zip(observations) {
            let scene_id = scene["id"].as_str().unwrap();
            assert_eq!(scene["axis"], oracle["axis"], "{scene_id}");
            assert_eq!(scene["mode"], oracle["mode"], "{scene_id}");
            assert_eq!(scene["minimum"], oracle["minimum"], "{scene_id}");
            assert_eq!(scene["maximum"], "0px", "{scene_id}");
            assert_eq!(scene["gap"], oracle["gap"], "{scene_id}");
            assert_eq!(scene["basis"], 16, "{scene_id}");

            let axis = scene["axis"].as_str().unwrap();
            let mode = scene["mode"].as_str().unwrap();
            let gap = scene["gap"].as_u64().unwrap() as usize;
            let basis = scene["basis"].as_u64().unwrap() as usize;
            let expected_count = (basis + gap) / (1 + gap);
            let oracle_count = oracle["repeatedTrackCount"].as_u64().unwrap() as usize;
            assert_eq!(oracle_count, expected_count, "{scene_id} UA floor count");

            let oracle_tracks = if axis == "columns" {
                oracle["gridTemplateColumns"].as_str().unwrap()
            } else {
                oracle["gridTemplateRows"].as_str().unwrap()
            };
            let expected_tracks = track_sizes(oracle_tracks);
            let runtime_tracks = scene["expect"]["tracks"][axis]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_f64().unwrap() as f32)
                .collect::<Vec<_>>();
            assert_eq!(runtime_tracks, expected_tracks, "{scene_id} tracks");
            assert_eq!(runtime_tracks.len(), expected_count, "{scene_id} count");

            let expected_collapsed = (0..expected_count)
                .map(|index| mode == "auto-fit" && index != 2)
                .collect::<Vec<_>>();
            let collapsed = scene["expect"]["collapsed"][axis]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(collapsed, expected_collapsed, "{scene_id} collapse");

            let item = &scene["items"][0];
            assert_eq!(item["width"], "12px", "{scene_id}");
            assert_eq!(item["height"], "12px", "{scene_id}");
            assert_eq!(item["minWidth"], "12px", "{scene_id}");
            assert_eq!(item["minHeight"], "12px", "{scene_id}");
            assert_eq!(
                item["placement"]["column"],
                if axis == "columns" { "3" } else { "1" }
            );
            assert_eq!(
                item["placement"]["row"],
                if axis == "rows" { "3" } else { "1" }
            );
            assert_eq!(
                scene["expect"]["items"][0]["frame"][0], oracle["itemOffset"]["x"],
                "{scene_id} item x"
            );
            assert_eq!(
                scene["expect"]["items"][0]["frame"][1], oracle["itemOffset"]["y"],
                "{scene_id} item y"
            );

            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id}");
            assert!(source.contains(scene_id));
            assert!(source.contains("grid-column:${itemSpec.placement.column}"));
            assert!(source.contains("grid-row:${itemSpec.placement.row}"));
            assert!(
                c11_4_grid_track_sizing_runtime_source(scene_id).is_ok(),
                "{scene_id}"
            );
        }

        assert!(runtime_source("zero-fixed-maximum-unlisted").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("zero-fixed-maximum-unlisted").is_err());
    }
}
