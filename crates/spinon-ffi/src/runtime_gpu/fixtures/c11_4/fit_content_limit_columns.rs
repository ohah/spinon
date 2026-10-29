const SCENES: [(&str, &str, &str, &str, &str, &str, bool); 4] = [
    (
        "column-fit-content-40",
        "c114-fit-content-column-limit-40-grid",
        "c114-fit-content-column-limit-40-item",
        "c114-fit-content-column-limit-40-child",
        "60",
        "40",
        false,
    ),
    (
        "column-fit-content-30",
        "c114-fit-content-column-limit-30-grid",
        "c114-fit-content-column-limit-30-item",
        "c114-fit-content-column-limit-30-child",
        "60",
        "30",
        false,
    ),
    (
        "column-fit-content-200",
        "c114-fit-content-column-limit-200-grid",
        "c114-fit-content-column-limit-200-item",
        "c114-fit-content-column-limit-200-child",
        "300",
        "200",
        false,
    ),
    (
        "column-fit-content-auto-minimum-control",
        "c114-fit-content-column-limit-auto-minimum-grid",
        "c114-fit-content-column-limit-auto-minimum-item",
        "c114-fit-content-column-limit-auto-minimum-child",
        "60",
        "40",
        true,
    ),
];

fn source(
    scene: &str,
    root: &str,
    item: &str,
    child: &str,
    root_width: &str,
    limit: &str,
    automatic_minimum: bool,
) -> String {
    let minimum = if automatic_minimum { "auto" } else { "0px" };
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root}');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{root_width}px;height:60px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;align-items:start;grid-auto-flow:column;grid-template-columns:none;grid-template-rows:60px;grid-auto-columns:fit-content({limit}px);grid-auto-rows:auto;background-color:#17212b');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', '{item}');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:{minimum};margin:0;padding:0;border:0;background-color:#e5484d');
const child = document.createElement('div');
child.setAttribute('id', '{child}');
child.setAttribute('data-node', '');
child.setAttribute('style', 'box-sizing:border-box;display:block;width:100px;height:10px;margin:0;padding:0;border:0;background-color:#ffd166');
item.appendChild(child);
root.appendChild(item);
return {{ sceneId:'{scene}', rootId:'{root}', nodeIds:['{root}','{item}','{child}'] }};
}})()"#
    )
}

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let (scene, root, item, child, root_width, limit, automatic_minimum) =
        SCENES.iter().find(|(id, ..)| *id == scene_id)?;
    Some(source(
        scene,
        root,
        item,
        child,
        root_width,
        limit,
        *automatic_minimum,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_four_column_fit_content_limit_boundaries() {
        assert_eq!(SCENES.len(), 4);
        for (scene_id, root_id, item_id, child_id, _, _, automatic_minimum) in SCENES {
            let source = runtime_source(scene_id).expect("listed column boundary scene");
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains(&format!("id', '{root_id}'")));
            assert!(source.contains(&format!("id', '{item_id}'")));
            assert!(source.contains(&format!("id', '{child_id}'")));
            assert!(source.contains("grid-template-columns:none"));
            assert!(source.contains("grid-template-rows:60px"));
            assert!(source.contains("grid-auto-flow:column"));
            assert!(source.contains("grid-auto-columns:fit-content("));
            assert!(source.contains("width:100px;height:10px"));
            assert!(source.contains(if automatic_minimum {
                "min-width:auto"
            } else {
                "min-width:0px"
            }));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        assert!(runtime_source("column-fit-content-40;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("column-fit-content-unlisted").is_err());
    }
}
