const SCENES_JSON: &str =
    include_str!("c11_4/auto_repeat_negative_percentage_neighbor_scenes.json");

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
  `max-height:${{scene.maxHeight}}`, 'margin:0', `padding:${{scene.padding}}`,
  `border:${{scene.border}}`, 'align-content:start', 'justify-content:start', 'gap:0',
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
    `height:${{itemSpec.height}}`, 'min-width:0', 'min-height:0',
    'margin:0', 'padding:0', 'border:0',
    ...(itemSpec.placement.gridColumn ? [`grid-column:${{itemSpec.placement.gridColumn}}`] : []),
    ...(itemSpec.placement.gridRow ? [`grid-row:${{itemSpec.placement.gridRow}}`] : []),
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

    const EXPECTED_SCENE_IDS: [&str; 9] = [
        "negative-percentage-col-prefix-positive",
        "negative-percentage-col-suffix-positive",
        "negative-percentage-row-prefix-positive",
        "negative-percentage-row-suffix-positive",
        "negative-percentage-col-prefix-negative-final",
        "negative-percentage-row-suffix-negative-final",
        "negative-percentage-col-border-box",
        "negative-percentage-col-fit-prefix-item",
        "negative-percentage-col-both-negative-final",
    ];

    #[test]
    fn negative_percentage_neighbor_catalog_is_exact_and_routed() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        assert_eq!(scenes.len(), EXPECTED_SCENE_IDS.len());
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), scenes.len());
        assert_eq!(ids, EXPECTED_SCENE_IDS.into_iter().collect());

        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id}");
            assert!(source.contains(scene_id));
            assert!(source.contains("scene.columns"), "{scene_id}");
            assert!(source.contains("scene.rows"), "{scene_id}");
            assert!(source.contains("scene.padding"), "{scene_id}");
            assert!(scene.get("expectedRejection").is_none(), "{scene_id}");
            assert_eq!(scene["items"].as_array().unwrap().len(), 1, "{scene_id}");
            for axis in ["columns", "rows"] {
                let tracks = scene["expect"]["tracks"][axis].as_array().unwrap();
                let collapsed = scene["expect"]["collapsed"][axis].as_array().unwrap();
                assert_eq!(tracks.len(), collapsed.len(), "{scene_id}/{axis}");
            }
            assert!(
                c11_4_grid_track_sizing_runtime_source(scene_id).is_ok(),
                "{scene_id} must use the runtime fixture dispatcher"
            );
        }

        assert!(runtime_source("negative-percentage-unlisted").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("negative-percentage-unlisted").is_err());
    }
}
