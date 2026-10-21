const SCENE_ID: &str = "spanning-auto-min-flex-tracks";

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    (scene_id == SCENE_ID).then(|| {
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-spanning-auto-min-flex-tracks-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:30px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(2,minmax(0,1fr));grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-spanning-auto-min-flex-tracks-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1 / span 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XXXXXXXXXXXXXXXXXXXX'));
root.appendChild(item);
return { sceneId:'spanning-auto-min-flex-tracks', rootId:'c114-spanning-auto-min-flex-tracks-grid', nodeIds:['c114-spanning-auto-min-flex-tracks-grid', 'c114-spanning-auto-min-flex-tracks-item'] };
})()"#
            .to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, runtime_source};

    #[test]
    fn exposes_only_the_fixed_spanning_auto_min_flex_text_scene() {
        let source = runtime_source(SCENE_ID).expect("known spanning text scene");
        assert!(source.len() < 4 * 1024);
        assert!(source.contains("repeat(2,minmax(0,1fr))"));
        assert!(source.contains("column-gap:4px"));
        assert!(source.contains("text-wrap-mode:nowrap"));
        assert!(source.contains("document.createTextNode('XXXXXXXXXXXXXXXXXXXX')"));
        assert!(source.contains("sceneId:'spanning-auto-min-flex-tracks'"));
        assert_eq!(source.matches("document.createElement('div')").count(), 2);
        assert!(runtime_source("unknown-spanning-text").is_none());
    }
}
