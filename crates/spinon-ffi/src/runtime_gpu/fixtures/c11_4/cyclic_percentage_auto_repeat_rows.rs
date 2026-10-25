const SCENES: [(&str, &str, &str); 4] = [
    (
        "cyclic-max-content-auto-fill-prefix-suffix",
        "max-content",
        "auto-fill",
    ),
    (
        "cyclic-max-content-auto-fit-prefix-suffix",
        "max-content",
        "auto-fit",
    ),
    (
        "definite-44px-auto-fill-prefix-suffix-control",
        "44px",
        "auto-fill",
    ),
    (
        "definite-44px-auto-fit-prefix-suffix-control",
        "44px",
        "auto-fit",
    ),
];

fn scene_source(scene_id: &str, root_height: &str, repeat_mode: &str) -> String {
    let root_id = format!("c114-{scene_id}-grid");
    let prefix_id = format!("c114-{scene_id}-prefix");
    let primary_id = format!("c114-{scene_id}-primary");
    let secondary_id = format!("c114-{scene_id}-secondary");

    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root_id}');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:24px;height:{root_height};grid-template-columns:24px;grid-template-rows:8px repeat({repeat_mode},minmax(0px,25%)) 8px;column-gap:0;row-gap:2px;margin:0;padding:0;border:0;align-content:start;justify-content:start;background-color:#101820');
document.appendChild(root);
const prefix = document.createElement('div');
prefix.setAttribute('id', '{prefix_id}');
prefix.setAttribute('data-node', '');
prefix.setAttribute('style', 'box-sizing:border-box;width:24px;height:8px;min-width:0;min-height:0;grid-row:1;grid-column:1;margin:0;padding:0;border:0;background-color:#2f9e44');
root.appendChild(prefix);
const primary = document.createElement('div');
primary.setAttribute('id', '{primary_id}');
primary.setAttribute('data-node', '');
primary.setAttribute('style', 'box-sizing:border-box;width:24px;height:24px;min-width:0;min-height:0;grid-row:2;grid-column:1;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(primary);
const secondary = document.createElement('div');
secondary.setAttribute('id', '{secondary_id}');
secondary.setAttribute('data-node', '');
secondary.setAttribute('style', 'box-sizing:border-box;width:16px;height:16px;min-width:0;min-height:0;grid-row:2;grid-column:1;margin:0;padding:0;border:0;background-color:#f08c00');
root.appendChild(secondary);
return {{ sceneId:'{scene_id}', rootId:'{root_id}', nodeIds:['{root_id}', '{prefix_id}', '{primary_id}', '{secondary_id}'] }};
}})()"#
    )
}

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|(known_id, _, _)| *known_id == scene_id)
        .map(|(known_id, root_height, repeat_mode)| {
            scene_source(known_id, root_height, repeat_mode)
        })
}

#[cfg(test)]
mod tests {
    use super::super::c11_4_grid_track_sizing_runtime_source;
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_four_generalized_cyclic_row_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (scene_id, root_height, repeat_mode) in SCENES {
            let source = runtime_source(scene_id).unwrap();
            let root_id = format!("c114-{scene_id}-grid");
            let prefix_id = format!("c114-{scene_id}-prefix");
            let primary_id = format!("c114-{scene_id}-primary");
            let secondary_id = format!("c114-{scene_id}-secondary");
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(source.contains(&format!("id', '{root_id}'")));
            assert!(source.contains(&format!("width:24px;height:{root_height}")));
            assert!(source.contains("grid-template-columns:24px"));
            assert!(source.contains(&format!(
                "grid-template-rows:8px repeat({repeat_mode},minmax(0px,25%)) 8px"
            )));
            assert!(source.contains("column-gap:0;row-gap:2px"));
            assert!(source.contains("background-color:#2f9e44"));
            assert!(source.contains("width:24px;height:24px;min-width:0;min-height:0"));
            assert!(source.contains("width:16px;height:16px;min-width:0;min-height:0"));
            assert_eq!(source.matches("document.createElement('div')").count(), 4);
            assert_eq!(source.matches("data-node").count(), 4);
            assert_eq!(source.matches("nodeIds:").count(), 1);
            assert!(source.contains(&format!(
                "nodeIds:['{root_id}', '{prefix_id}', '{primary_id}', '{secondary_id}']"
            )));
            assert_eq!(source.matches("root.appendChild(").count(), 3);
            assert!(!source.contains("suffix-probe"));
            assert!(!source.contains("document.createElement('span')"));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        assert!(runtime_source("cyclic-max-content-auto-fill-rows-unlisted").is_none());
        assert!(
            c11_4_grid_track_sizing_runtime_source("cyclic-max-content-auto-fill-rows-typo")
                .is_err()
        );
    }
}
