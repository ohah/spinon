#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
}

const TRACK: &str = "minmax(calc(10% + 2px),max-content)";
const TEXT: &str = "aaaaaa";

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-two-axis-span-text-min-content",
        width: "min-content",
    },
    Scene {
        id: "cyclic-minmax-calc-two-axis-span-text-max-content",
        width: "max-content",
    },
    Scene {
        id: "definite-minmax-calc-two-axis-span-text-min-content",
        width: "10px",
    },
    Scene {
        id: "definite-minmax-calc-two-axis-span-text-max-content",
        width: "60px",
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
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:{track} {track};grid-template-rows:{track} {track};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:1 / 3;grid-row:1 / 3;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode({text:?}));
root.appendChild(item);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {item_id:?}] }};
}})()"#,
        width = scene.width,
        track = TRACK,
        text = TEXT,
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_two_axis_span_text_scenes() {
        let unique_ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(unique_ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted two-axis span text scene");
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:{};height:max-content", scene.width)));
            assert_eq!(
                source
                    .matches("grid-template-columns:minmax(calc(10% + 2px),max-content)")
                    .count(),
                1
            );
            assert_eq!(
                source
                    .matches("grid-template-rows:minmax(calc(10% + 2px),max-content)")
                    .count(),
                1
            );
            assert_eq!(source.matches("column-gap:2px;row-gap:2px").count(), 1);
            assert!(source.contains("min-width:auto;min-height:auto"));
            assert!(source.contains("grid-column:1 / 3;grid-row:1 / 3"));
            assert!(source.contains("word-break:break-all"));
            assert!(source.contains("document.createTextNode(\"aaaaaa\")"));
            assert!(source.contains("border:2px solid #f0c674"));
            assert!(source.contains("background-color:#e5484d"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
        }
        for unlisted in [
            "cyclic-minmax-calc-two-axis-span-text-unlisted",
            "cyclic-minmax-calc-two-axis-span-text-3-track",
            "definite-minmax-calc-two-axis-span-text-grid-column-only",
        ] {
            assert!(runtime_source(unlisted).is_none(), "{unlisted}");
        }
    }
}
