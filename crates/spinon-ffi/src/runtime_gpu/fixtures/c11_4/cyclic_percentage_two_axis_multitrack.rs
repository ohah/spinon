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
        id: "cyclic-percent-two-axis-multitrack",
        width: "max-content",
        height: "max-content",
        columns: "50% 25% 20px",
        rows: "50% 25% 10px",
    },
    Scene {
        id: "cyclic-percent-columns-multitrack",
        width: "max-content",
        height: "20px",
        columns: "50% 25% 20px",
        rows: "10px 5px 5px",
    },
    Scene {
        id: "cyclic-percent-rows-multitrack",
        width: "40px",
        height: "max-content",
        columns: "20px 10px 10px",
        rows: "50% 25% 10px",
    },
    Scene {
        id: "definite-percent-two-axis-multitrack",
        width: "40px",
        height: "20px",
        columns: "50% 25% 20px",
        rows: "50% 25% 10px",
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
    let text_id = format!("c114-{}-text", scene.id);
    let overflow_id = format!("c114-{}-overflow", scene.id);
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820');
document.appendChild(root);
const textItem = document.createElement('div');
textItem.setAttribute('id', {text_id:?});
textItem.setAttribute('data-node', '');
textItem.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1;grid-row:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
textItem.appendChild(document.createTextNode('XX'));
root.appendChild(textItem);
const overflow = document.createElement('div');
overflow.setAttribute('id', {overflow_id:?});
overflow.setAttribute('data-node', '');
overflow.setAttribute('style', 'box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;border:0;grid-column:3;grid-row:3;background-color:#20bf6b');
root.appendChild(overflow);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {text_id:?}, {overflow_id:?}] }};
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
    fn exposes_only_four_fixed_two_axis_multitrack_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted cyclic scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("grid-template-columns:{}", scene.columns)));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.rows)));
            assert!(source.contains(&format!("width:{};height:{}", scene.width, scene.height)));
            assert!(source.contains("document.createTextNode('XX')"));
            assert!(source.contains("grid-column:3;grid-row:3"));
            assert!(source.contains("width:20px;height:10px"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("data-node").count(), 3);
            assert_eq!(source.matches("nodeIds:").count(), 1);
        }
        assert!(runtime_source("cyclic-percent-two-axis-unlisted").is_none());
    }
}
