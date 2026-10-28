const SCENE_ID: &str = "fit-content-percentage-row-wpt";
const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-percentage-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(50%);background-color:#17212b');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-percentage-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d');
const child = document.createElement('div');
child.setAttribute('id', 'c114-fit-content-percentage-child');
child.setAttribute('data-node', '');
child.setAttribute('style', 'box-sizing:border-box;display:block;width:30px;height:400px;margin:0;padding:0;border:0;background-color:#ffd166');
item.appendChild(child);
root.appendChild(item);
return { sceneId:'fit-content-percentage-row-wpt', rootId:'c114-fit-content-percentage-grid', nodeIds:['c114-fit-content-percentage-grid', 'c114-fit-content-percentage-item', 'c114-fit-content-percentage-child'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_definite_height_percentage_scene() {
        let source = runtime_source(SCENE_ID).expect("allowlisted C11.4 scene");
        assert!(source.len() < 3 * 1024);
        assert!(source.contains("width:60px;height:300px"));
        assert!(source.contains("grid-template-rows:none"));
        assert!(source.contains("grid-auto-rows:fit-content(50%)"));
        assert!(source.contains("min-height:0;"));
        assert!(!source.contains("overflow:visible"));
        assert!(source.contains("width:30px;height:400px"));
        assert_eq!(source.matches("document.createElement('div')").count(), 3);
        assert_eq!(source.matches("root.appendChild(").count(), 1);
        assert_eq!(source.matches("item.appendChild(").count(), 1);
        assert!(runtime_source("fit-content-percentage-row-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(c11_4_grid_track_sizing_runtime_source("fit-content-percentage-unlisted").is_err());
    }
}
