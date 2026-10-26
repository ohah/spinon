const SCENE_ID: &str = "fit-content-auto-neighbor-wpt";
const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-auto-neighbor-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px) auto;background-color:#17212b');
document.appendChild(root);
const fitItem = document.createElement('div');
fitItem.setAttribute('id', 'c114-fit-content-auto-neighbor-fit-item');
fitItem.setAttribute('data-node', '');
fitItem.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d');
const fitChild = document.createElement('div');
fitChild.setAttribute('id', 'c114-fit-content-auto-neighbor-fit-child');
fitChild.setAttribute('data-node', '');
fitChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:10px;margin:0;padding:0;border:0;background-color:#ffd166');
fitItem.appendChild(fitChild);
root.appendChild(fitItem);
const autoItem = document.createElement('div');
autoItem.setAttribute('id', 'c114-fit-content-auto-neighbor-auto-item');
autoItem.setAttribute('data-node', '');
autoItem.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#3a86ff');
const autoChild = document.createElement('div');
autoChild.setAttribute('id', 'c114-fit-content-auto-neighbor-auto-child');
autoChild.setAttribute('data-node', '');
autoChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:10px;margin:0;padding:0;border:0;background-color:#06d6a0');
autoItem.appendChild(autoChild);
root.appendChild(autoItem);
return { sceneId:'fit-content-auto-neighbor-wpt', rootId:'c114-fit-content-auto-neighbor-grid', nodeIds:['c114-fit-content-auto-neighbor-grid', 'c114-fit-content-auto-neighbor-fit-item', 'c114-fit-content-auto-neighbor-fit-child', 'c114-fit-content-auto-neighbor-auto-item', 'c114-fit-content-auto-neighbor-auto-child'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_fit_content_auto_neighbor_scene() {
        let source = runtime_source(SCENE_ID).expect("allowlisted C11.4 scene");
        assert!(source.len() < 4 * 1024);
        assert!(source.contains("grid-template-rows:none"));
        assert!(source.contains("grid-auto-rows:fit-content(40px) auto"));
        assert!(source.contains("align-content:normal"));
        assert_eq!(source.matches("document.createElement('div')").count(), 5);
        assert_eq!(source.matches("root.appendChild(").count(), 2);
        assert_eq!(source.matches("fitItem.appendChild(").count(), 1);
        assert_eq!(source.matches("autoItem.appendChild(").count(), 1);
        assert!(source.contains("fit-content-auto-neighbor-fit-item"));
        assert!(source.contains("fit-content-auto-neighbor-auto-item"));
        assert!(runtime_source("fit-content-auto-neighbor-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(
            c11_4_grid_track_sizing_runtime_source("fit-content-auto-neighbor-unlisted").is_err()
        );
    }
}
