const SCENES_JSON: &str = include_str!("multiple_fixed_repeat_groups_scenes.json");

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
  'box-sizing:border-box', 'display:grid', `width:${{scene.width}}`,
  `height:${{scene.height}}`, 'min-width:0', 'min-height:0',
  'margin:0', 'padding:0', 'border:0', 'align-content:start', 'justify-content:start',
  `column-gap:${{scene.columnGap}}`, `row-gap:${{scene.rowGap}}`,
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  ...(scene.autoFlow ? [`grid-auto-flow:${{scene.autoFlow}}`] : []),
  `background-color:${{scene.rootColor}}`,
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  const placement = itemSpec.placement ?? {{}};
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block',
    `width:${{itemSpec.width ?? 'auto'}}`, `height:${{itemSpec.height ?? 'auto'}}`,
    'min-width:0', 'min-height:0', 'margin:0', 'padding:0', 'border:0',
    ...(placement.gridColumn ? [`grid-column:${{placement.gridColumn}}`] : []),
    ...(placement.gridRow ? [`grid-row:${{placement.gridRow}}`] : []),
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
    use serde_json::Value;
    use std::collections::BTreeSet;

    #[test]
    fn multiple_fixed_repeat_group_matrix_has_twelve_bounded_unique_scenes() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let expected_ids = [
            "fixed-repeat-groups-prefix-fill",
            "fixed-repeat-groups-split-fill",
            "fixed-repeat-groups-suffix-fill",
            "fixed-repeat-groups-reversed-fill",
            "fixed-repeat-groups-multitrack-fill",
            "fixed-repeat-groups-prefix-fit-empty",
            "fixed-repeat-groups-prefix-fit-occupied",
            "fixed-repeat-groups-row-prefix-fill",
            "fixed-repeat-groups-row-split-fit-empty",
            "fixed-repeat-groups-row-prefix-fit-occupied",
            "fixed-repeat-groups-four-prefix-fill",
            "fixed-repeat-groups-four-row-split-fill",
        ];
        assert_eq!(scenes.len(), expected_ids.len());
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), scenes.len());
        assert_eq!(ids, expected_ids.into_iter().collect());

        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let expected = &scene["expect"];
            let items = scene["items"].as_array().unwrap();
            let expected_items = expected["items"].as_array().unwrap();
            assert_eq!(expected["root"].as_array().unwrap().len(), 2, "{scene_id}");
            assert_eq!(items.len(), expected_items.len(), "{scene_id}");
            for (item, expected_item) in items.iter().zip(expected_items) {
                assert_eq!(item["name"], expected_item["name"], "{scene_id}");
                assert_eq!(
                    expected_item["frame"].as_array().unwrap().len(),
                    4,
                    "{scene_id}"
                );
            }
            for axis in ["columns", "rows"] {
                let tracks = expected["tracks"][axis].as_array().unwrap();
                let collapsed = expected["collapsed"][axis].as_array().unwrap();
                assert_eq!(tracks.len(), collapsed.len(), "{scene_id}/{axis}");
                assert!(
                    tracks.iter().all(|track| track.as_f64().is_some()),
                    "{scene_id}/{axis}"
                );
            }
            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id}");
            assert!(source.contains(scene_id));
            assert!(source.contains("grid-template-columns:"), "{scene_id}");
            assert!(source.contains("grid-template-rows:"), "{scene_id}");
            assert!(source.contains("background-color:"), "{scene_id}");
            if !items.is_empty() {
                assert!(source.contains("grid-column:"), "{scene_id}");
                assert!(source.contains("grid-row:"), "{scene_id}");
            }
        }
        assert!(runtime_source("fixed-repeat-groups-unlisted").is_none());
    }
}
