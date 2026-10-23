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
  `background-color:${{scene.rootColor ?? '#101820'}}`,
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  const placement = Object.entries(itemSpec.placement).map(([property, value]) => {{
    const kebabProperty = property.replace(/[A-Z]/g, (letter) => `-${{letter.toLowerCase()}}`);
    const name = property === 'gridArea'
      ? 'grid-area'
      : property.startsWith('grid') ? kebabProperty : `grid-${{kebabProperty}}`;
    return `${{name}}:${{value}}`;
  }});
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block',
    `width:${{itemSpec.width ?? 'auto'}}`, `height:${{itemSpec.height ?? 'auto'}}`,
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
    fn template_area_auto_repeat_scenes_have_fixed_runtime_frames() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        const TWO_AXIS_SCENES: [&str; 9] = [
            "template-area-repeat-two-axis-area-short",
            "template-area-repeat-two-axis-area-added-columns",
            "template-area-repeat-two-axis-area-added-rows",
            "template-area-repeat-two-axis-area-added-both",
            "template-area-repeat-two-axis-fill-fit",
            "template-area-repeat-two-axis-fit-fill",
            "template-area-repeat-two-axis-fit-fit-empty",
            "template-area-repeat-two-axis-fit-fit-span",
            "template-area-repeat-two-axis-area-longhand",
        ];
        assert_eq!(scenes.len(), 22);
        assert_eq!(ids.len(), scenes.len());
        for scene_id in TWO_AXIS_SCENES {
            assert!(ids.contains(scene_id), "missing {scene_id}");
        }
        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let expected = &scene["expect"];
            assert_eq!(expected["root"].as_array().unwrap().len(), 2, "{scene_id}");
            let items = scene["items"].as_array().unwrap();
            let expected_items = expected["items"].as_array().unwrap();
            assert_eq!(items.len(), expected_items.len(), "{scene_id}");
            if scene_id.starts_with("template-area-repeat-two-axis-") {
                let expected_item_count =
                    if scene_id == "template-area-repeat-two-axis-fit-fit-empty" {
                        0
                    } else {
                        1
                    };
                assert_eq!(items.len(), expected_item_count, "{scene_id}");
                let tracks = &expected["tracks"];
                assert!(tracks["columns"].as_array().is_some(), "{scene_id} columns");
                assert!(tracks["rows"].as_array().is_some(), "{scene_id} rows");
            } else {
                assert!(
                    items.len() >= 2,
                    "{scene_id} must include occupied neighbors"
                );
                assert!(expected["tracks"].as_array().is_some(), "{scene_id} tracks");
            }
            let source = runtime_source(scene_id).unwrap();
            if scene_id == "template-area-repeat-two-axis-area-longhand" {
                assert!(source.contains("property.startsWith('grid')"));
                assert!(source.contains("property.replace(/[A-Z]/g,"));
                assert!(!source.contains("`grid-${property.replace"));
            }
            assert!(source.len() < 8 * 1024, "{scene_id} source is too large");
            if scene.get("areas").is_some() {
                assert!(source.contains("grid-template-areas"), "{scene_id}");
            }
            if let Some(root_color) = scene.get("rootColor").and_then(Value::as_str) {
                assert!(source.contains(root_color), "{scene_id} root color");
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
                for dimension in ["width", "height"] {
                    if let Some(value) = item.get(dimension).and_then(Value::as_str) {
                        assert!(source.contains(value), "{scene_id}/{name} {dimension}");
                    }
                }
            }
        }
        assert!(runtime_source("template-area-repeat-unlisted").is_none());
    }
}
