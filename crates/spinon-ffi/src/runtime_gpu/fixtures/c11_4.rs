const C11_4_GRID_TRACK_SIZING_SCENES: [(&str, &str); 7] = [
    (
        "auto-fixed-width-item",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:auto;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fixed-width-item', rootId:'c114-auto-grid', nodeIds:['c114-auto-grid', 'c114-auto-item'] };
})()"#,
    ),
    (
        "minmax-auto-fixed-width-item",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-minmax-auto-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:minmax(0,auto);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-minmax-auto-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'minmax-auto-fixed-width-item', rootId:'c114-minmax-auto-grid', nodeIds:['c114-minmax-auto-grid', 'c114-minmax-auto-item'] };
})()"#,
    ),
    (
        "fit-content-below-item-size",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:fit-content(12px);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'fit-content-below-item-size', rootId:'c114-fit-content-grid', nodeIds:['c114-fit-content-grid', 'c114-fit-content-item'] };
})()"#,
    ),
    (
        "spanning-auto-tracks",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-spanning-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:30px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-template-columns:repeat(2,minmax(0,auto));grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-spanning-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:70px;height:10px;margin:0;padding:0;border:0;grid-column:1 / span 2;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'spanning-auto-tracks', rootId:'c114-spanning-grid', nodeIds:['c114-spanning-grid', 'c114-spanning-item'] };
})()"#,
    ),
    (
        "auto-fill-repeat",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fill-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fill-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:100%;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fill-repeat', rootId:'c114-auto-fill-grid', nodeIds:['c114-auto-fill-grid', 'c114-auto-fill-item'] };
})()"#,
    ),
    (
        "auto-fit-collapse",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fit-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fit,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fit-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:100%;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fit-collapse', rootId:'c114-auto-fit-grid', nodeIds:['c114-auto-fit-grid', 'c114-auto-fit-item'] };
})()"#,
    ),
    (
        "auto-fill-track-overflow",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fill-overflow-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fill-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:120px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fill-track-overflow', rootId:'c114-auto-fill-overflow-grid', nodeIds:['c114-auto-fill-overflow-grid', 'c114-auto-fill-overflow-item'] };
})()"#,
    ),
];

pub(crate) fn c11_4_grid_track_sizing_runtime_source(
    scene_id: &str,
) -> Result<&'static str, String> {
    C11_4_GRID_TRACK_SIZING_SCENES
        .iter()
        .find_map(|(known_id, source)| (*known_id == scene_id).then_some(*source))
        .ok_or_else(|| format!("알 수 없는 C11.4 scene ID: {scene_id}"))
}

#[cfg(test)]
mod tests {
    use super::{C11_4_GRID_TRACK_SIZING_SCENES, c11_4_grid_track_sizing_runtime_source};

    #[test]
    fn exposes_only_bounded_intrinsic_track_sizing_scenes() {
        assert_eq!(C11_4_GRID_TRACK_SIZING_SCENES.len(), 7);
        for (scene_id, source) in C11_4_GRID_TRACK_SIZING_SCENES {
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("document.createElement"));
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        assert!(c11_4_grid_track_sizing_runtime_source("unknown;throw 1").is_err());
    }
}
