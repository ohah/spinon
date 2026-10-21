#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    depth: usize,
    constraint: &'static str,
    two_axis: bool,
}

const SCENES: [Scene; 8] = [
    Scene {
        id: "nested-depth-one-min-content",
        depth: 1,
        constraint: "min-content",
        two_axis: false,
    },
    Scene {
        id: "nested-depth-two-min-content",
        depth: 2,
        constraint: "min-content",
        two_axis: false,
    },
    Scene {
        id: "nested-depth-one-max-content",
        depth: 1,
        constraint: "max-content",
        two_axis: false,
    },
    Scene {
        id: "nested-depth-two-max-content",
        depth: 2,
        constraint: "max-content",
        two_axis: false,
    },
    Scene {
        id: "nested-depth-one-min-content-two-axis",
        depth: 1,
        constraint: "min-content",
        two_axis: true,
    },
    Scene {
        id: "nested-depth-two-min-content-two-axis",
        depth: 2,
        constraint: "min-content",
        two_axis: true,
    },
    Scene {
        id: "nested-depth-one-max-content-two-axis",
        depth: 1,
        constraint: "max-content",
        two_axis: true,
    },
    Scene {
        id: "nested-depth-two-max-content-two-axis",
        depth: 2,
        constraint: "max-content",
        two_axis: true,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let root_id = format!("c114-{}-root", scene.id);
    let middle_id = format!("c114-{}-middle", scene.id);
    let inner_id = format!("c114-{}-inner", scene.id);
    let text_id = format!("c114-{}-text", scene.id);
    let mut node_ids = vec![root_id.clone()];
    let mut grid_ids = vec![root_id.clone()];
    if scene.depth == 2 {
        node_ids.push(middle_id.clone());
        grid_ids.push(middle_id.clone());
    }
    node_ids.extend([inner_id.clone(), text_id.clone()]);
    grid_ids.push(inner_id.clone());
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{constraint};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:{constraint};grid-template-rows:{rows};align-items:start;background-color:#e9f1ff');
document.appendChild(root);
const middle = {middle_element};
if (middle) {{
  middle.setAttribute('id', {middle_id:?});
  middle.setAttribute('data-node', '');
  middle.setAttribute('style', 'box-sizing:border-box;display:grid;width:{constraint};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:{constraint};grid-template-rows:{rows};align-items:start;background-color:#e7f6ed');
  root.appendChild(middle);
}}
const inner = document.createElement('div');
inner.setAttribute('id', {inner_id:?});
inner.setAttribute('data-node', '');
inner.setAttribute('style', 'box-sizing:border-box;display:grid;width:{constraint};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:{constraint};grid-template-rows:{rows};align-items:start;background-color:#fff0e1');
if (middle) middle.appendChild(inner); else root.appendChild(inner);
const text = document.createElement('div');
text.setAttribute('id', {text_id:?});
text.setAttribute('data-node', '');
text.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#ffffff');
text.appendChild(document.createTextNode('XX XXXXXXX'));
inner.appendChild(text);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids:?}, gridIds:{grid_ids:?} }};
}})()"#,
        root_id = root_id,
        middle_id = middle_id,
        inner_id = inner_id,
        text_id = text_id,
        scene_id = scene.id,
        constraint = scene.constraint,
        height = if scene.two_axis {
            scene.constraint
        } else {
            "auto"
        },
        rows = if scene.two_axis {
            scene.constraint
        } else {
            "auto"
        },
        middle_element = if scene.depth == 2 {
            "document.createElement('div')".to_owned()
        } else {
            "null".to_owned()
        },
        node_ids = node_ids,
        grid_ids = grid_ids,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn emits_only_eight_fixed_nested_grid_depth_scenes() {
        assert_eq!(SCENES.len(), 8);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted nested Grid depth scene");
            assert!(source.len() < 5 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!(
                "width:{};height:{}",
                scene.constraint,
                if scene.two_axis {
                    scene.constraint
                } else {
                    "auto"
                }
            )));
            assert!(source.contains(&format!("grid-template-columns:{}", scene.constraint)));
            assert!(source.contains(&format!(
                "grid-template-rows:{}",
                if scene.two_axis {
                    scene.constraint
                } else {
                    "auto"
                }
            )));
            assert!(source.contains("document.createTextNode('XX XXXXXXX')"));
            assert!(source.contains("font-family:Ahem;font-size:10px"));
            assert!(source.contains("column-gap:0;row-gap:0"));
            assert!(!source.contains("writing-mode:"));
            assert!(source.contains(&format!(
                "gridIds:{:?}",
                if scene.depth == 1 {
                    vec![
                        format!("c114-{}-root", scene.id),
                        format!("c114-{}-inner", scene.id),
                    ]
                } else {
                    vec![
                        format!("c114-{}-root", scene.id),
                        format!("c114-{}-middle", scene.id),
                        format!("c114-{}-inner", scene.id),
                    ]
                }
            )));
        }
        assert!(runtime_source("nested-depth-three-min-content").is_none());
        assert!(runtime_source("nested-depth-two-unlisted").is_none());
    }
}
