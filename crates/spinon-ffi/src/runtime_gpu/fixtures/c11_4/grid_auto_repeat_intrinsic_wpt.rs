#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Scene {
    id: &'static str,
    repeat: &'static str,
    width: &'static str,
    neighbor: Neighbor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Neighbor {
    Fixed,
    Auto,
}

const SCENES: [Scene; 8] = [
    Scene {
        id: "auto-repeat-intrinsic-fill-fixed-max",
        repeat: "auto-fill",
        width: "max-content",
        neighbor: Neighbor::Fixed,
    },
    Scene {
        id: "auto-repeat-intrinsic-fill-fixed-min",
        repeat: "auto-fill",
        width: "min-content",
        neighbor: Neighbor::Fixed,
    },
    Scene {
        id: "auto-repeat-intrinsic-fill-auto-max",
        repeat: "auto-fill",
        width: "max-content",
        neighbor: Neighbor::Auto,
    },
    Scene {
        id: "auto-repeat-intrinsic-fill-auto-min",
        repeat: "auto-fill",
        width: "min-content",
        neighbor: Neighbor::Auto,
    },
    Scene {
        id: "auto-repeat-intrinsic-fit-fixed-max",
        repeat: "auto-fit",
        width: "max-content",
        neighbor: Neighbor::Fixed,
    },
    Scene {
        id: "auto-repeat-intrinsic-fit-fixed-min",
        repeat: "auto-fit",
        width: "min-content",
        neighbor: Neighbor::Fixed,
    },
    Scene {
        id: "auto-repeat-intrinsic-fit-auto-max",
        repeat: "auto-fit",
        width: "max-content",
        neighbor: Neighbor::Auto,
    },
    Scene {
        id: "auto-repeat-intrinsic-fit-auto-min",
        repeat: "auto-fit",
        width: "min-content",
        neighbor: Neighbor::Auto,
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
    let neighbor = match scene.neighbor {
        Neighbor::Fixed => "minmax(min-content,40px)",
        Neighbor::Auto => "minmax(60px,auto)",
    };
    let repeated_track = match scene.neighbor {
        Neighbor::Fixed => "20px",
        Neighbor::Auto => "10px",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'display:grid;background-color:grey;border:2px solid black;position:relative;padding-top:10px;justify-items:start;width:{width};grid-template-columns:repeat({repeat},{repeated_track}) {neighbor}');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'grid-column:1 / -1;width:100px;height:20px;background-color:cyan');
root.appendChild(item);
nodeIds.push({item_id:?});
"#,
        width = scene.width,
        repeat = scene.repeat,
    );

    for (line, color) in ["purple", "orange", "yellow", "green"]
        .into_iter()
        .enumerate()
    {
        let line = line + 1;
        let probe_id = format!("c114-{}-a{line}", scene.id);
        source.push_str(&format!(
            r#"const probe{line} = document.createElement('div');
probe{line}.setAttribute('id', {probe_id:?});
probe{line}.setAttribute('data-node', '');
probe{line}.setAttribute('style', 'height:5px;position:absolute;width:100%;grid-column:{line} / {end};background-color:{color}');
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
    use super::{Neighbor, SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_the_eight_wpt_intrinsic_auto_repeat_scenes_without_extra_css() {
        assert_eq!(SCENES.len(), 8);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("listed WPT scene");
            let repeated_track = match scene.neighbor {
                Neighbor::Fixed => "20px",
                Neighbor::Auto => "10px",
            };
            let trailing_track = match scene.neighbor {
                Neighbor::Fixed => "minmax(min-content,40px)",
                Neighbor::Auto => "minmax(60px,auto)",
            };
            assert!(source.contains(&format!(
                "width:{};grid-template-columns:repeat({},{repeated_track}) {trailing_track}",
                scene.width, scene.repeat
            )));
            assert!(source.contains("display:grid;background-color:grey;border:2px solid black"));
            assert!(source.contains("padding-top:10px;justify-items:start"));
            assert!(
                source.contains("grid-column:1 / -1;width:100px;height:20px;background-color:cyan")
            );
            assert!(!source.contains("min-width:"));
            assert!(!source.contains("grid-auto-columns:"));
            assert!(!source.contains("grid-auto-rows:"));
            assert_eq!(
                source
                    .matches("position:absolute;width:100%;grid-column:")
                    .count(),
                4
            );
            for (line, color) in ["purple", "orange", "yellow", "green"]
                .into_iter()
                .enumerate()
            {
                assert!(source.contains(&format!(
                    "grid-column:{} / {};background-color:{color}",
                    line + 1,
                    line + 2
                )));
            }
            assert_eq!(source.matches("nodeIds.push(").count(), 5);
            assert!(c11_4_grid_track_sizing_runtime_source(scene.id).is_ok());
        }
        assert!(runtime_source("auto-repeat-intrinsic-unknown").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("auto-repeat-intrinsic-unknown").is_err());
    }
}
