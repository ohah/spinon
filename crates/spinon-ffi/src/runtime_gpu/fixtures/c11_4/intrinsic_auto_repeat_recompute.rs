#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    sizing: &'static str,
    item_width: u16,
}

const SCENES: [Scene; 8] = [
    Scene {
        id: "intrinsic-auto-repeat-max-content-100",
        sizing: "max-content",
        item_width: 100,
    },
    Scene {
        id: "intrinsic-auto-repeat-max-content-80",
        sizing: "max-content",
        item_width: 80,
    },
    Scene {
        id: "intrinsic-auto-repeat-max-content-15",
        sizing: "max-content",
        item_width: 15,
    },
    Scene {
        id: "intrinsic-auto-repeat-max-content-120",
        sizing: "max-content",
        item_width: 120,
    },
    Scene {
        id: "intrinsic-auto-repeat-min-content-100",
        sizing: "min-content",
        item_width: 100,
    },
    Scene {
        id: "intrinsic-auto-repeat-min-content-80",
        sizing: "min-content",
        item_width: 80,
    },
    Scene {
        id: "intrinsic-auto-repeat-min-content-15",
        sizing: "min-content",
        item_width: 15,
    },
    Scene {
        id: "intrinsic-auto-repeat-min-content-120",
        sizing: "min-content",
        item_width: 120,
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
    let root_id = format!("c114-{}-grid", scene.id);
    let item_id = format!("c114-{}-item", scene.id);
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'display:grid;position:relative;width:{sizing};min-width:30px;border:2px solid black;padding-top:10px;margin-bottom:10px;grid-auto-columns:20px;grid-template-columns:repeat(auto-fill,20px) minmax(min-content,40px);grid-template-rows:none;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'grid-column:1 / -1;width:{item_width}px;height:30px;background-color:#e5484d');
root.appendChild(item);
nodeIds.push({item_id:?});
"#,
        sizing = scene.sizing,
        item_width = scene.item_width,
    );

    for line in 1..=4 {
        let probe_id = format!("c114-{}-a{line}", scene.id);
        source.push_str(&format!(
            r#"const probe{line} = document.createElement('div');
probe{line}.setAttribute('id', {probe_id:?});
probe{line}.setAttribute('data-node', '');
probe{line}.setAttribute('style', 'height:5px;position:absolute;width:100%;grid-column:{line} / {end};background-color:#35b7aa');
root.appendChild(probe{line});
nodeIds.push({probe_id:?});
"#,
            end = line + 1,
        ));
    }

    source.push_str(&format!(
        "return {{ sceneId:{:?}, rootId:{root_id:?}, nodeIds }};\n}})()",
        scene.id
    ));
    source
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_eight_wpt_intrinsic_auto_repeat_states() {
        let expected = [
            ("intrinsic-auto-repeat-max-content-100", "max-content", 100),
            ("intrinsic-auto-repeat-max-content-80", "max-content", 80),
            ("intrinsic-auto-repeat-max-content-15", "max-content", 15),
            ("intrinsic-auto-repeat-max-content-120", "max-content", 120),
            ("intrinsic-auto-repeat-min-content-100", "min-content", 100),
            ("intrinsic-auto-repeat-min-content-80", "min-content", 80),
            ("intrinsic-auto-repeat-min-content-15", "min-content", 15),
            ("intrinsic-auto-repeat-min-content-120", "min-content", 120),
        ];
        assert_eq!(SCENES.len(), expected.len());

        for (scene, (id, sizing, item_width)) in SCENES.iter().zip(expected) {
            assert_eq!(
                (scene.id, scene.sizing, scene.item_width),
                (id, sizing, item_width)
            );
            let source = runtime_source(id).expect("known WPT state");
            assert!(source.contains(&format!("width:{sizing};min-width:30px")));
            assert!(source.contains("repeat(auto-fill,20px) minmax(min-content,40px)"));
            assert!(source.contains(&format!("width:{item_width}px;height:30px")));
            assert_eq!(
                source
                    .matches("height:5px;position:absolute;width:100%;grid-column:")
                    .count(),
                4
            );
            for line in 1..=4 {
                assert!(source.contains(&format!("grid-column:{line} / {}", line + 1)));
                assert!(source.contains(&format!("c114-{id}-a{line}")));
            }
            assert_eq!(source.matches("nodeIds.push(").count(), 5);
        }

        assert!(runtime_source("max-content-100-unknown").is_none());
    }
}
