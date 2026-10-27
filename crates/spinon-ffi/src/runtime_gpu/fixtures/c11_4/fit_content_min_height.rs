const SCENE_ID: &str = "fit-content-min-height-wpt";
const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-min-height-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:auto;min-height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);background-color:#17212b');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-min-height-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d');
const child = document.createElement('div');
child.setAttribute('id', 'c114-fit-content-min-height-child');
child.setAttribute('data-node', '');
child.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:10px;margin:0;padding:0;border:0;background-color:#ffd166');
item.appendChild(child);
root.appendChild(item);
return { sceneId:'fit-content-min-height-wpt', rootId:'c114-fit-content-min-height-grid', nodeIds:['c114-fit-content-min-height-grid', 'c114-fit-content-min-height-item', 'c114-fit-content-min-height-child'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_fit_content_min_height_scene() {
        let source = runtime_source(SCENE_ID).expect("allowlisted C11.4 scene");
        assert!(source.len() < 3 * 1024);
        assert!(source.contains("height:auto;min-height:300px"));
        assert!(source.contains("grid-template-rows:none"));
        assert!(source.contains("grid-auto-rows:fit-content(40px)"));
        assert!(source.contains("align-content:normal"));
        assert_eq!(source.matches("document.createElement('div')").count(), 3);
        assert_eq!(source.matches("root.appendChild(").count(), 1);
        assert_eq!(source.matches("item.appendChild(").count(), 1);
        assert!(source.contains("c114-fit-content-min-height-item"));
        assert!(source.contains("c114-fit-content-min-height-child"));
        assert!(runtime_source("fit-content-min-height-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(c11_4_grid_track_sizing_runtime_source("fit-content-min-height-unlisted").is_err());
    }
}
