#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    outer_row_end: &'static str,
}

const SCENES: [Scene; 2] = [
    Scene {
        id: "nested-grid-implicit-row-span-two",
        outer_row_end: "3",
    },
    Scene {
        id: "nested-grid-implicit-row-one-row",
        outer_row_end: "2",
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
    let explicit_id = format!("c114-{}-explicit", scene.id);
    let first_implicit_id = format!("c114-{}-implicit-min", scene.id);
    let second_implicit_id = format!("c114-{}-implicit-max", scene.id);
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:24px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:4px;grid-template-columns:24px;grid-template-rows:min-content min-content;background-color:#e9f1ff');
document.appendChild(root);
const nested = document.createElement('div');
nested.setAttribute('id', {nested_id:?});
nested.setAttribute('data-node', '');
nested.setAttribute('style', 'box-sizing:border-box;display:grid;width:24px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:4px;grid-row:1 / {outer_row_end};grid-template-columns:24px;grid-template-rows:min-content;grid-auto-rows:min-content max-content;align-items:start;justify-items:normal;background-color:#e7f6ed');
root.appendChild(nested);
const explicit = document.createElement('div');
explicit.setAttribute('id', {explicit_id:?});
explicit.setAttribute('data-node', '');
explicit.setAttribute('style', 'box-sizing:border-box;display:block;width:24px;height:8px;margin:0;padding:0;border:0;grid-column:1;grid-row:1 / 2;background-color:#2a6');
nested.appendChild(explicit);
const firstImplicit = document.createElement('div');
firstImplicit.setAttribute('id', {first_implicit_id:?});
firstImplicit.setAttribute('data-node', '');
firstImplicit.setAttribute('style', 'box-sizing:border-box;display:block;width:4px;height:8px;margin:0;padding:0;border:0;grid-column:1;grid-row:2 / 3;background-color:#e90');
nested.appendChild(firstImplicit);
const secondImplicit = document.createElement('img');
secondImplicit.setAttribute('id', {second_implicit_id:?});
secondImplicit.setAttribute('data-node', '');
secondImplicit.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
secondImplicit.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1;grid-row:3 / 4;background-color:#44ddee');
nested.appendChild(secondImplicit);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {nested_id:?}, {explicit_id:?}, {first_implicit_id:?}, {second_implicit_id:?}] }};
}})()"#,
        outer_row_end = scene.outer_row_end,
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn emits_only_two_fixed_nested_positive_implicit_row_scenes() {
        assert_eq!(SCENES.len(), 2);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted nested implicit row scene");
            assert!(source.len() < 5 * 1024);
            assert!(
                source.contains(
                    "grid-template-rows:min-content;grid-auto-rows:min-content max-content"
                )
            );
            assert!(source.contains("grid-row:2 / 3"));
            assert!(source.contains("grid-row:3 / 4"));
            assert!(source.contains("c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(source.matches("document.createElement('div')").count(), 4);
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
        }
        assert!(runtime_source("nested-grid-implicit-row-unlisted").is_none());
    }
}
