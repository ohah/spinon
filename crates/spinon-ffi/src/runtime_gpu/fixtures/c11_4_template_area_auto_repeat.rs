const SCENES_JSON: &str = include_str!("c11_4/template_area_auto_repeat_scenes.json");

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
  `grid-auto-columns:${{scene.autoColumns ?? 'auto'}}`,
  `grid-auto-rows:${{scene.autoRows ?? 'auto'}}`,
  ...(scene.areas ? [`grid-template-areas:${{scene.areas}}`] : []),
  'background-color:#101820',
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  const placement = Object.entries(itemSpec.placement).map(([property, value]) => {{
    const name = property === 'gridArea'
      ? 'grid-area'
      : `grid-${{property.replace(/[A-Z]/g, (letter) => `-${{letter.toLowerCase()}}`)}}`;
    return `${{name}}:${{value}}`;
  }});
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block', 'width:auto', 'height:auto',
    'min-width:0', 'min-height:0', 'margin:0', 'padding:0', 'border:0',
    ...placement,
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
    fn all_thirteen_template_area_auto_repeat_scenes_have_fixed_runtime_frames() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(scenes.len(), 13);
        assert_eq!(ids.len(), scenes.len());
        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let expected = &scene["expect"];
            assert_eq!(expected["root"].as_array().unwrap().len(), 2, "{scene_id}");
            let items = scene["items"].as_array().unwrap();
            let expected_items = expected["items"].as_array().unwrap();
            assert_eq!(items.len(), expected_items.len(), "{scene_id}");
            assert!(
                items.len() >= 2,
                "{scene_id} must include occupied neighbors"
            );
            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id} source is too large");
            if scene.get("areas").is_some() {
                assert!(source.contains("grid-template-areas"), "{scene_id}");
            }
            assert!(source.contains(scene_id), "{scene_id}");
            assert!(
                source.contains("c114-${scene.id}-${itemSpec.name}"),
                "{scene_id} item IDs must include their names"
            );
            for item in items {
                let name = item["name"].as_str().unwrap();
                assert!(source.contains(name), "{scene_id}/{name}");
                assert!(
                    source.contains(item["color"].as_str().unwrap()),
                    "{scene_id}/{name}"
                );
            }
        }
        assert!(runtime_source("template-area-repeat-unlisted").is_none());
    }
}
