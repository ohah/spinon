#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    intrinsic: &'static str,
    image_before_text: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "depth-three-min-content-text-before-image",
        intrinsic: "min-content",
        image_before_text: false,
    },
    Scene {
        id: "depth-three-min-content-image-before-text",
        intrinsic: "min-content",
        image_before_text: true,
    },
    Scene {
        id: "depth-three-max-content-text-before-image",
        intrinsic: "max-content",
        image_before_text: false,
    },
    Scene {
        id: "depth-three-max-content-image-before-text",
        intrinsic: "max-content",
        image_before_text: true,
    },
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let grid_ids = [
        format!("c114-{}-root", scene.id),
        format!("c114-{}-middle", scene.id),
        format!("c114-{}-branch", scene.id),
        format!("c114-{}-inner", scene.id),
    ];
    let image_id = format!("c114-{}-image", scene.id);
    let node_ids = grid_ids
        .iter()
        .cloned()
        .chain(std::iter::once(image_id.clone()))
        .collect::<Vec<_>>();
    let grid_ids_js = format!("{grid_ids:?}");
    let node_ids_js = format!("{node_ids:?}");
    let text_then_image =
        "inner.appendChild(document.createTextNode('XX XXXXXXX'));\ninner.appendChild(image);";
    let image_then_text =
        "inner.appendChild(image);\ninner.appendChild(document.createTextNode('XX XXXXXXX'));";
    let item_order = if scene.image_before_text {
        image_then_text
    } else {
        text_then_image
    };
    format!(
        r#"(() => {{
const gridIds = {grid_ids_js};
const nodeIds = {node_ids_js};
const backgrounds = ['#e9f1ff', '#e7f6ed', '#f1e9ff', '#fff0e1'];
const grids = gridIds.map((id, index) => {{
  const grid = document.createElement('div');
  const tracks = index === 3 ? '{intrinsic} {intrinsic}' : '{intrinsic}';
  grid.setAttribute('id', id);
  grid.setAttribute('data-node', '');
  grid.setAttribute('style', `box-sizing:border-box;display:grid;width:{intrinsic};height:{intrinsic};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:${{tracks}};grid-template-rows:{intrinsic};align-items:start;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:${{backgrounds[index]}}`);
  return grid;
}});
document.appendChild(grids[0]);
for (let index = 1; index < grids.length; index += 1) grids[index - 1].appendChild(grids[index]);
const image = document.createElement('img');
image.setAttribute('id', {image_id:?});
image.setAttribute('data-node', '');
image.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
image.setAttribute('alt', '');
image.setAttribute('style', 'box-sizing:border-box;display:inline;width:auto;height:auto;margin:0;padding:0;border:0');
const inner = grids[3];
{item_order}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds, gridIds }};
}})()"#,
        intrinsic = scene.intrinsic,
        image_id = image_id,
        item_order = item_order,
        scene_id = scene.id,
        root_id = grid_ids[0],
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn emits_only_four_depth_three_anonymous_text_image_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("listed mixed nested scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("const gridIds = ["));
            assert!(source.contains("grid-template-rows:"));
            assert!(source.contains("document.createTextNode('XX XXXXXXX')"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert!(source.contains("grid-template-columns:${tracks}"));
            assert!(source.contains("width:auto;height:auto"));
            assert!(!source.contains("grid-column:"));
            assert!(!source.contains("depth-four"));
        }
        assert!(runtime_source("depth-three-min-content-unknown").is_none());
        assert!(runtime_source("depth-four-min-content-text-before-image").is_none());
    }
}
