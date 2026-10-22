#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
    height: &'static str,
    columns: &'static str,
    rows: &'static str,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-min-column",
        width: "max-content",
        height: "10px",
        columns: "minmax(calc(200% + 4px),max-content)",
        rows: "10px",
    },
    Scene {
        id: "cyclic-minmax-calc-min-row",
        width: "10px",
        height: "max-content",
        columns: "10px",
        rows: "minmax(calc(200% + 2px),max-content)",
    },
    Scene {
        id: "definite-minmax-calc-min-column",
        width: "10px",
        height: "10px",
        columns: "minmax(calc(200% + 4px),max-content)",
        rows: "10px",
    },
    Scene {
        id: "definite-minmax-calc-min-row",
        width: "10px",
        height: "10px",
        columns: "10px",
        rows: "minmax(calc(200% + 2px),max-content)",
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
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:{height};margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode('a'));
root.appendChild(item);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {item_id:?}] }};
}})()"#,
        width = scene.width,
        height = scene.height,
        columns = scene.columns,
        rows = scene.rows,
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_minimum_calc_scenes_with_real_border_paint() {
        let unique_ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(unique_ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted minimum calc scene");
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:{};height:{}", scene.width, scene.height)));
            assert!(source.contains(&format!("grid-template-columns:{}", scene.columns)));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.rows)));
            assert!(source.contains("border:2px solid #f0c674"));
            assert!(source.contains("document.createTextNode('a')"));
            assert!(source.contains("font-family:Ahem;font-size:10px"));
            assert!(source.contains("background-color:#e5484d"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
        }
        assert!(runtime_source("cyclic-minmax-calc-min-unlisted").is_none());
    }
}
