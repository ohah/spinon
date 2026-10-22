const SCENES_JSON: &str = include_str!("c11_4/auto_repeat_named_line_scenes.json");

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scenes: serde_json::Value = serde_json::from_str(SCENES_JSON).ok()?;
    let scene_ids = scenes.as_array()?;
    scene_ids
        .iter()
        .any(|scene| {
            scene
                .get("id")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|id| id == scene_id)
        })
        .then(|| {
            format!(
                r#"(() => {{
const scenes = JSON.parse({SCENES_JSON:?});
const scene = scenes.find(candidate => candidate.id === {scene_id:?});
if (!scene) throw new Error('unknown auto-repeat named line scene');
const rootId = `c114-${{scene.id}}-grid`;
const itemId = `c114-${{scene.id}}-item`;
const root = document.createElement('div');
root.setAttribute('id', rootId);
root.setAttribute('data-node', '');
root.setAttribute('style', [
  'box-sizing:border-box', 'display:grid', `width:${{scene.width}}`,
  `height:${{scene.height}}`, 'margin:0', 'padding:0', 'border:0',
  'align-content:start', 'justify-content:start', 'background-color:#e2e8f0',
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  `column-gap:${{scene.columnGap}}`, `row-gap:${{scene.rowGap}}`,
].join(';'));
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', itemId);
item.setAttribute('data-node', '');
item.setAttribute('style', [
  'box-sizing:border-box', 'display:block', 'width:auto', 'height:auto',
  'min-width:0', 'min-height:0', 'margin:0', 'padding:0', 'border:0',
  `grid-column-start:${{scene.item.columnStart}}`,
  `grid-column-end:${{scene.item.columnEnd}}`,
  `grid-row-start:${{scene.item.rowStart}}`, `grid-row-end:${{scene.item.rowEnd}}`,
  'background-color:#39a5db',
].join(';'));
root.appendChild(item);
return {{ sceneId:scene.id, rootId, nodeIds:[rootId, itemId] }};
}})()"#,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::{SCENES_JSON, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_all_fixed_named_auto_repeat_runtime_scenes() {
        let scenes: serde_json::Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(scenes.len(), 8);
        assert_eq!(ids.len(), scenes.len());
        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id} source is too large");
            assert!(source.contains(&format!("candidate.id === {scene_id:?}")));
            assert!(source.contains("grid-column-start"));
            assert!(source.contains("grid-row-end"));
        }
        assert!(runtime_source("unknown-auto-repeat-named-line").is_none());
    }
}
