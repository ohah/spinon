const SCENES: [(&str, &str, &str, &str); 3] = [
    (
        "fit-content-limit-40-wpt",
        "c114-fit-content-limit-40-grid",
        "c114-fit-content-limit-40-item",
        "c114-fit-content-limit-40-child",
    ),
    (
        "fit-content-limit-30-wpt",
        "c114-fit-content-limit-30-grid",
        "c114-fit-content-limit-30-item",
        "c114-fit-content-limit-30-child",
    ),
    (
        "fit-content-limit-200-wpt",
        "c114-fit-content-limit-200-grid",
        "c114-fit-content-limit-200-item",
        "c114-fit-content-limit-200-child",
    ),
];

fn source(scene: &str, root: &str, item: &str, child: &str, limit: &str) -> String {
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root}');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content({limit}px);background-color:#17212b');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', '{item}');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d');
const child = document.createElement('div');
child.setAttribute('id', '{child}');
child.setAttribute('data-node', '');
child.setAttribute('style', 'box-sizing:border-box;display:block;width:60px;height:100px;margin:0;padding:0;border:0;background-color:#ffd166');
item.appendChild(child);
root.appendChild(item);
return {{ sceneId:'{scene}', rootId:'{root}', nodeIds:['{root}','{item}','{child}'] }};
}})()"#
    )
}

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let (_, root, item, child) = SCENES.iter().find(|(id, ..)| *id == scene_id)?;
    let limit = match scene_id {
        "fit-content-limit-40-wpt" => "40",
        "fit-content-limit-30-wpt" => "30",
        "fit-content-limit-200-wpt" => "200",
        _ => return None,
    };
    Some(source(scene_id, root, item, child, limit))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_three_fit_content_limit_boundaries() {
        assert_eq!(SCENES.len(), 3);
        for (scene_id, root_id, item_id, child_id) in SCENES {
            let source = runtime_source(scene_id).expect("listed boundary scene");
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains(&format!("id', '{root_id}'")));
            assert!(source.contains(&format!("id', '{item_id}'")));
            assert!(source.contains(&format!("id', '{child_id}'")));
            assert!(source.contains("min-height:0"));
            assert!(source.contains("height:100px"));
            assert!(!source.contains(concat!("overflow:", "visible")));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        assert!(runtime_source("fit-content-limit-40-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("fit-content-limit-unlisted").is_err());
    }
}
