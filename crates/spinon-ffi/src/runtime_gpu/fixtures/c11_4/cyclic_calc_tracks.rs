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
        id: "cyclic-calc-column",
        width: "max-content",
        height: "max-content",
        columns: "calc(50% + 4px)",
        rows: "16px",
    },
    Scene {
        id: "cyclic-calc-row",
        width: "24px",
        height: "max-content",
        columns: "24px",
        rows: "calc(50% + 2px)",
    },
    Scene {
        id: "cyclic-calc-two-axis",
        width: "max-content",
        height: "max-content",
        columns: "calc(50% + 4px)",
        rows: "calc(50% + 2px)",
    },
    Scene {
        id: "definite-calc-control",
        width: "40px",
        height: "20px",
        columns: "calc(50% + 4px)",
        rows: "calc(50% + 2px)",
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
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:24px;height:16px;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1;grid-row:1;font-family:Ahem;font-size:12px;font-weight:400;font-style:normal;line-height:16px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode('XX'));
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

    #[test]
    fn exposes_only_four_fixed_cyclic_calc_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted calc scene");
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("grid-template-columns:{}", scene.columns)));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.rows)));
            assert!(source.contains(&format!("width:{};height:{}", scene.width, scene.height)));
            assert!(source.contains("width:24px;height:16px"));
            assert!(source.contains("document.createTextNode('XX')"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
        }
        assert!(runtime_source("cyclic-calc-unlisted").is_none());
    }
}
