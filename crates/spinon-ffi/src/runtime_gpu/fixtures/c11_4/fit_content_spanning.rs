const SCENE_ID: &str = "fit-content-spanning-wpt";

const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-spanning-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);background-color:#17212b');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-spanning-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;grid-row:span 2;margin:0;padding:0;border:0;background-color:#e5484d');
const child = document.createElement('div');
child.setAttribute('id', 'c114-fit-content-spanning-child');
child.setAttribute('data-node', '');
child.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:200px;margin:0;padding:0;border:0;background-color:#ffd166');
item.appendChild(child);
root.appendChild(item);
return { sceneId:'fit-content-spanning-wpt', rootId:'c114-fit-content-spanning-grid', nodeIds:['c114-fit-content-spanning-grid','c114-fit-content-spanning-item','c114-fit-content-spanning-child'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, SOURCE, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_fit_content_span_scene() {
        assert_eq!(runtime_source(SCENE_ID), Some(SOURCE));
        assert!(SOURCE.len() < 3 * 1024);
        assert_eq!(SOURCE.matches("document.createElement('div')").count(), 3);
        assert!(SOURCE.contains("grid-auto-rows:fit-content(40px)"));
        assert!(SOURCE.contains("grid-row:span 2"));
        assert!(SOURCE.contains("min-height:0"));
        assert!(SOURCE.contains("height:200px"));
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(runtime_source("fit-content-spanning-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("fit-content-spanning-unlisted").is_err());
    }
}
