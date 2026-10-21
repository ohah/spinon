const SCENES: [(&str, &str); 3] = [
    (
        "cyclic-fit-content-column",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-cyclic-fit-content-column-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:max-content;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:fit-content(50%);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-cyclic-fit-content-column-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode('aa aa'));
root.appendChild(item);
return { sceneId:'cyclic-fit-content-column', rootId:'c114-cyclic-fit-content-column-grid', nodeIds:['c114-cyclic-fit-content-column-grid', 'c114-cyclic-fit-content-column-item'] };
})()"#,
    ),
    (
        "cyclic-fit-content-row",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-cyclic-fit-content-row-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:20px;height:max-content;margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:20px;grid-template-rows:fit-content(50%);background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-cyclic-fit-content-row-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode('a a a a'));
root.appendChild(item);
return { sceneId:'cyclic-fit-content-row', rootId:'c114-cyclic-fit-content-row-grid', nodeIds:['c114-cyclic-fit-content-row-grid', 'c114-cyclic-fit-content-row-item'] };
})()"#,
    ),
    (
        "definite-fit-content-column",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-definite-fit-content-column-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:fit-content(50%);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-definite-fit-content-column-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode('aa aa'));
root.appendChild(item);
return { sceneId:'definite-fit-content-column', rootId:'c114-definite-fit-content-column-grid', nodeIds:['c114-definite-fit-content-column-grid', 'c114-definite-fit-content-column-item'] };
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
    fn exposes_only_the_three_fixed_cyclic_fit_content_scenes() {
        assert_eq!(SCENES.len(), 3);
        for (scene_id, source) in SCENES {
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains("fit-content(50%)"));
            assert!(source.contains("min-width:0;min-height:0"));
            assert!(source.contains("font-family:Ahem;font-size:10px"));
            assert!(source.contains("line-height:10px"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
            assert_eq!(source.matches("nodeIds:").count(), 1);
            assert_eq!(runtime_source(scene_id), Some(source.to_owned()));
        }
        assert!(runtime_source("cyclic-fit-content-unlisted").is_none());
    }
}
