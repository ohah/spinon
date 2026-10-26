const SCENE_ID: &str = "align-content-normal-equal-auto-rows";
const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-align-content-normal-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:160px;height:100px;margin:0;padding:0;border:0;column-gap:0;row-gap:10px;grid-template-columns:160px;grid-template-rows:auto auto 5px;grid-auto-flow:row;align-content:normal;justify-content:start;background-color:#101820');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', 'c114-align-content-normal-item-first');
first.setAttribute('data-node', '');
first.setAttribute('style', 'box-sizing:border-box;display:block;width:160px;height:10px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', 'c114-align-content-normal-item-second');
second.setAttribute('data-node', '');
second.setAttribute('style', 'box-sizing:border-box;display:block;width:160px;height:20px;margin:0;padding:0;border:0;background-color:#3a86ff');
root.appendChild(second);
const third = document.createElement('div');
third.setAttribute('id', 'c114-align-content-normal-item-fixed');
third.setAttribute('data-node', '');
third.setAttribute('style', 'box-sizing:border-box;display:block;width:160px;height:5px;margin:0;padding:0;border:0;background-color:#06d6a0');
root.appendChild(third);
return { sceneId:'align-content-normal-equal-auto-rows', rootId:'c114-align-content-normal-grid', nodeIds:['c114-align-content-normal-grid', 'c114-align-content-normal-item-first', 'c114-align-content-normal-item-second', 'c114-align-content-normal-item-fixed'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_ID, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_equal_auto_rows_scene() {
        let source = runtime_source(SCENE_ID).expect("allowlisted C11.4 scene");

        assert!(source.len() < 4 * 1024);
        assert!(source.contains("width:160px;height:100px"));
        assert!(source.contains("row-gap:10px"));
        assert!(source.contains("grid-template-rows:auto auto 5px"));
        assert!(source.contains("align-content:normal"));
        assert!(source.contains("justify-content:start"));
        assert_eq!(source.matches("document.createElement('div')").count(), 4);
        assert_eq!(source.matches("root.appendChild(").count(), 3);
        assert!(source.contains("height:10px"));
        assert!(source.contains("height:20px"));
        assert!(source.contains("height:5px"));
        assert!(runtime_source("align-content-normal-equal-auto-rows;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(
            c11_4_grid_track_sizing_runtime_source("align-content-normal-equal-auto-rows-unlisted")
                .is_err()
        );
    }
}
