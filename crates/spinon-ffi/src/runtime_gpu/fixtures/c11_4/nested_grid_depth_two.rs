#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    depth: usize,
    constraint: &'static str,
    two_axis: bool,
}

const SCENES: [Scene; 10] = [
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
    Scene {
        id: "nested-depth-three-min-content-two-axis",
        depth: 3,
        constraint: "min-content",
        two_axis: true,
    },
    Scene {
        id: "nested-depth-three-max-content-two-axis",
        depth: 3,
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
    let text_id = format!("c114-{}-text", scene.id);
    let nested_roles: &[&str] = match scene.depth {
        1 => &["inner"],
        2 => &["middle", "inner"],
        3 => &["middle", "branch", "inner"],
        _ => unreachable!("scene depth must remain explicitly allowlisted"),
    };
    let mut grid_ids = vec![root_id.clone()];
    grid_ids.extend(
        nested_roles
            .iter()
            .map(|role| format!("c114-{}-{role}", scene.id)),
    );
    let mut node_ids = grid_ids.clone();
    node_ids.push(text_id.clone());
    format!(
        r#"(() => {{
const gridIds = {grid_ids:?};
const grids = gridIds.map((id) => {{
  const role = id.slice(id.lastIndexOf('-') + 1);
  const background = role === 'root' ? '#e9f1ff'
    : role === 'middle' ? '#e7f6ed'
      : role === 'branch' ? '#f1e9ff' : '#fff0e1';
  const grid = document.createElement('div');
  grid.setAttribute('id', id);
  grid.setAttribute('data-node', '');
  grid.setAttribute('style', `box-sizing:border-box;display:grid;width:{constraint};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:{constraint};grid-template-rows:{rows};align-items:start;background-color:${{background}}`);
  return grid;
}});
document.appendChild(grids[0]);
for (let index = 1; index < grids.length; index += 1) grids[index - 1].appendChild(grids[index]);
const text = document.createElement('div');
text.setAttribute('id', {text_id:?});
text.setAttribute('data-node', '');
text.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#ffffff');
text.appendChild(document.createTextNode('XX XXXXXXX'));
grids[grids.length - 1].appendChild(text);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids:?}, gridIds }};
}})()"#,
        grid_ids = grid_ids,
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
        node_ids = node_ids,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn emits_only_ten_fixed_nested_grid_depth_scenes() {
        assert_eq!(SCENES.len(), 10);
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
            let grid_ids = match scene.depth {
                1 => vec![
                    format!("c114-{}-root", scene.id),
                    format!("c114-{}-inner", scene.id),
                ],
                2 => vec![
                    format!("c114-{}-root", scene.id),
                    format!("c114-{}-middle", scene.id),
                    format!("c114-{}-inner", scene.id),
                ],
                3 => vec![
                    format!("c114-{}-root", scene.id),
                    format!("c114-{}-middle", scene.id),
                    format!("c114-{}-branch", scene.id),
                    format!("c114-{}-inner", scene.id),
                ],
                _ => unreachable!("scene depth must remain explicitly allowlisted"),
            };
            let mut node_ids = grid_ids.clone();
            node_ids.push(format!("c114-{}-text", scene.id));
            assert!(source.contains(&format!("const gridIds = {grid_ids:?};")));
            assert!(source.contains(&format!("nodeIds:{node_ids:?}")));
        }
        assert!(runtime_source("nested-depth-three-min-content").is_none());
        assert!(runtime_source("nested-depth-four-min-content-two-axis").is_none());
        assert!(runtime_source("nested-depth-two-unlisted").is_none());
    }
}
