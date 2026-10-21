#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
    columns: &'static str,
    nested_placement: &'static str,
}

const SCENES: [Scene; 3] = [
    Scene {
        id: "nested-grid-span-two",
        width: "max-content",
        columns: "min-content min-content",
        nested_placement: "1 / span 2",
    },
    Scene {
        id: "nested-grid-one-track",
        width: "max-content",
        columns: "min-content min-content",
        nested_placement: "1 / 2",
    },
    Scene {
        id: "nested-grid-definite-flex-control",
        width: "120px",
        columns: "minmax(auto,1fr) minmax(auto,1fr)",
        nested_placement: "1 / span 2",
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
    let nested_id = format!("c114-{}-nested", scene.id);
    let item_a_id = format!("c114-{}-item-a", scene.id);
    let item_b_id = format!("c114-{}-item-b", scene.id);
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width};height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-template-columns:{columns};grid-template-rows:auto;background-color:#e9f1ff');
document.appendChild(root);
const nested = document.createElement('div');
nested.setAttribute('id', {nested_id:?});
nested.setAttribute('data-node', '');
nested.setAttribute('style', 'box-sizing:border-box;display:grid;width:auto;height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-column:{nested_placement};grid-template-columns:min-content max-content;grid-template-rows:auto;background-color:#e7f6ed');
root.appendChild(nested);
const itemA = document.createElement('div');
itemA.setAttribute('id', {item_a_id:?});
itemA.setAttribute('data-node', '');
itemA.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#fff0e1');
itemA.appendChild(document.createTextNode('XX XXXXXXX'));
nested.appendChild(itemA);
const itemB = document.createElement('div');
itemB.setAttribute('id', {item_b_id:?});
itemB.setAttribute('data-node', '');
itemB.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
itemB.appendChild(document.createTextNode('XXXXXX'));
nested.appendChild(itemB);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {nested_id:?}, {item_a_id:?}, {item_b_id:?}] }};
}})()"#,
        width = scene.width,
        columns = scene.columns,
        nested_placement = scene.nested_placement,
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn emits_only_the_three_allowlisted_nested_grid_scenes() {
        assert_eq!(SCENES.len(), 3);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted nested Grid scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("grid-template-columns:min-content max-content"));
            assert!(source.contains("document.createTextNode('XX XXXXXXX')"));
            assert!(source.contains("document.createTextNode('XXXXXX')"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(source.matches("document.createElement('div')").count(), 4);
        }
        assert!(runtime_source("nested-grid-unlisted").is_none());
    }
}
