const SCENES: [(&str, &str); 3] = [
    (
        "cyclic-minmax-column",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-cyclic-minmax-column-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:max-content;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(0px,50%);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-cyclic-minmax-column-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'cyclic-minmax-column', rootId:'c114-cyclic-minmax-column-grid', nodeIds:['c114-cyclic-minmax-column-grid', 'c114-cyclic-minmax-column-item'] };
})()"#,
    ),
    (
        "cyclic-minmax-row",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-cyclic-minmax-row-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:24px;height:max-content;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:auto;grid-template-rows:minmax(0px,50%);background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-cyclic-minmax-row-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'cyclic-minmax-row', rootId:'c114-cyclic-minmax-row-grid', nodeIds:['c114-cyclic-minmax-row-grid', 'c114-cyclic-minmax-row-item'] };
})()"#,
    ),
    (
        "definite-minmax-column",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-definite-minmax-column-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(0px,50%);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-definite-minmax-column-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'definite-minmax-column', rootId:'c114-definite-minmax-column-grid', nodeIds:['c114-definite-minmax-column-grid', 'c114-definite-minmax-column-item'] };
})()"#,
    ),
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|(known_id, _)| *known_id == scene_id)
        .map(|(_, source)| (*source).to_owned())
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_fixed_cyclic_percentage_minmax_scenes() {
        assert_eq!(SCENES.len(), 3);

        for (scene_id, source) in SCENES {
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains("minmax(0px,50%)") || scene_id == "cyclic-minmax-row");
            assert!(source.contains("width:24px;height:16px"));
            assert!(source.contains("margin:0;padding:0;border:0"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
            assert_eq!(source.matches("nodeIds:").count(), 1);
            assert_eq!(runtime_source(scene_id), Some(source.to_owned()));
        }

        assert!(runtime_source("cyclic-minmax-unlisted").is_none());
    }
}
