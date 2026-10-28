const WPT_SCENE: &str = "fit-content-auto-min-floor-wpt";
const ZERO_SCENE: &str = "fit-content-auto-min-floor-zero-control";

const WPT_SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-auto-min-floor-wpt-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);background-color:#17212b');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', 'c114-fit-content-auto-min-floor-wpt-first-item');
first.setAttribute('data-node', '');
first.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d');
const firstChild = document.createElement('div');
firstChild.setAttribute('id', 'c114-fit-content-auto-min-floor-wpt-first-child');
firstChild.setAttribute('data-node', '');
firstChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:10px;margin:0;padding:0;border:0;background-color:#ffd166');
first.appendChild(firstChild);
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', 'c114-fit-content-auto-min-floor-wpt-second-item');
second.setAttribute('data-node', '');
second.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d');
const secondChild = document.createElement('div');
secondChild.setAttribute('id', 'c114-fit-content-auto-min-floor-wpt-second-child');
secondChild.setAttribute('data-node', '');
secondChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:100px;margin:0;padding:0;border:0;background-color:#ffd166');
second.appendChild(secondChild);
root.appendChild(second);
return { sceneId:'fit-content-auto-min-floor-wpt', rootId:'c114-fit-content-auto-min-floor-wpt-grid', nodeIds:['c114-fit-content-auto-min-floor-wpt-grid','c114-fit-content-auto-min-floor-wpt-first-item','c114-fit-content-auto-min-floor-wpt-first-child','c114-fit-content-auto-min-floor-wpt-second-item','c114-fit-content-auto-min-floor-wpt-second-child'] };
})()"#;

const ZERO_SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-auto-min-floor-zero-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);background-color:#17212b');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', 'c114-fit-content-auto-min-floor-zero-first-item');
first.setAttribute('data-node', '');
first.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d');
const firstChild = document.createElement('div');
firstChild.setAttribute('id', 'c114-fit-content-auto-min-floor-zero-first-child');
firstChild.setAttribute('data-node', '');
firstChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:10px;margin:0;padding:0;border:0;background-color:#ffd166');
first.appendChild(firstChild);
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', 'c114-fit-content-auto-min-floor-zero-second-item');
second.setAttribute('data-node', '');
second.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d');
const secondChild = document.createElement('div');
secondChild.setAttribute('id', 'c114-fit-content-auto-min-floor-zero-second-child');
secondChild.setAttribute('data-node', '');
secondChild.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:100px;margin:0;padding:0;border:0;background-color:#ffd166');
second.appendChild(secondChild);
root.appendChild(second);
return { sceneId:'fit-content-auto-min-floor-zero-control', rootId:'c114-fit-content-auto-min-floor-zero-grid', nodeIds:['c114-fit-content-auto-min-floor-zero-grid','c114-fit-content-auto-min-floor-zero-first-item','c114-fit-content-auto-min-floor-zero-first-child','c114-fit-content-auto-min-floor-zero-second-item','c114-fit-content-auto-min-floor-zero-second-child'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    match scene_id {
        WPT_SCENE => Some(WPT_SOURCE),
        ZERO_SCENE => Some(ZERO_SOURCE),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{WPT_SCENE, ZERO_SCENE, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_two_pinned_auto_minimum_scenes() {
        for scene in [WPT_SCENE, ZERO_SCENE] {
            let source = runtime_source(scene).expect("allowlisted scene");
            assert!(source.len() < 5 * 1024);
            assert!(source.contains("grid-auto-rows:fit-content(40px)"));
            assert!(source.contains("height:300px"));
            assert_eq!(source.matches("document.createElement('div')").count(), 5);
            assert_eq!(source.matches("root.appendChild(").count(), 2);
            assert_eq!(source.matches("first.appendChild(").count(), 1);
            assert_eq!(source.matches("second.appendChild(").count(), 1);
            assert!(c11_4_grid_track_sizing_runtime_source(scene).is_ok());
        }
        assert!(runtime_source("fit-content-auto-min-floor-wpt;throw 1").is_none());
        assert!(
            c11_4_grid_track_sizing_runtime_source("fit-content-auto-min-floor-unlisted").is_err()
        );
        assert!(!runtime_source(WPT_SCENE).unwrap().contains("min-height:0"));
        assert!(runtime_source(ZERO_SCENE).unwrap().contains("min-height:0"));
    }
}
