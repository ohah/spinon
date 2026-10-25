const SCENES: [(&str, &str); 2] = [
    (
        "cyclic-max-content-auto-fill-rows",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-cyclic-max-content-auto-fill-rows-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:24px;height:max-content;grid-template-columns:24px;grid-template-rows:repeat(auto-fill,minmax(0px,50%));column-gap:0;row-gap:0;margin:0;padding:0;border:0;align-content:start;justify-content:start;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-cyclic-max-content-auto-fill-rows-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;min-width:0;min-height:0;grid-row:1;grid-column:1;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'cyclic-max-content-auto-fill-rows', rootId:'c114-cyclic-max-content-auto-fill-rows-grid', nodeIds:['c114-cyclic-max-content-auto-fill-rows-grid', 'c114-cyclic-max-content-auto-fill-rows-item'] };
})()"#,
    ),
    (
        "definite-40px-auto-fill-control-rows",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-definite-40px-auto-fill-control-rows-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:24px;height:40px;grid-template-columns:24px;grid-template-rows:repeat(auto-fill,minmax(0px,50%));column-gap:0;row-gap:0;margin:0;padding:0;border:0;align-content:start;justify-content:start;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-definite-40px-auto-fill-control-rows-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;min-width:0;min-height:0;grid-row:1;grid-column:1;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'definite-40px-auto-fill-control-rows', rootId:'c114-definite-40px-auto-fill-control-rows-grid', nodeIds:['c114-definite-40px-auto-fill-control-rows-grid', 'c114-definite-40px-auto-fill-control-rows-item'] };
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
    use super::super::c11_4_grid_track_sizing_runtime_source;
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_cyclic_max_content_rows_and_definite_control_scenes() {
        assert_eq!(SCENES.len(), 2);
        for (scene_id, source) in SCENES {
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains("grid-template-columns:24px"));
            assert!(source.contains("grid-template-rows:repeat(auto-fill,minmax(0px,50%))"));
            assert!(source.contains("width:24px;height:16px;min-width:0;min-height:0"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
            assert_eq!(source.matches("nodeIds:").count(), 1);
            assert_eq!(runtime_source(scene_id), Some(source.to_owned()));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        assert!(runtime_source("cyclic-max-content-auto-fill-rows-unlisted").is_none());
        assert!(
            c11_4_grid_track_sizing_runtime_source("cyclic-max-content-auto-fill-rows-typo")
                .is_err()
        );
    }
}
