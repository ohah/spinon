const SCENES: [(&str, &str, &str); 5] = [
    (
        "columns-span2-visible-content-minimum",
        "c114-auto-minmax-span-overflow-visible",
        "visible",
    ),
    (
        "columns-span2-clip-content-minimum",
        "c114-auto-minmax-span-overflow-clip",
        "clip",
    ),
    (
        "columns-span2-hidden-zero-minimum",
        "c114-auto-minmax-span-overflow-hidden",
        "hidden",
    ),
    (
        "columns-span2-auto-zero-minimum",
        "c114-auto-minmax-span-overflow-auto",
        "auto",
    ),
    (
        "columns-span2-scroll-zero-minimum",
        "c114-auto-minmax-span-overflow-scroll",
        "scroll",
    ),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (scene_id, root_id, overflow_x) = SCENES
        .iter()
        .copied()
        .find(|(known_scene_id, _, _)| *known_scene_id == scene_id)?;
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root_id}');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:80px;height:20px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:minmax(auto,max-content) minmax(auto,max-content);grid-template-rows:20px');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-minmax-span-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;background-color:#e5484d;grid-column:1 / span 2;grid-row:1;font-family:Ahem;font-size:10px;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;overflow-x:{overflow_x}');
item.appendChild(document.createTextNode('XXXXXXXXXX'));
root.appendChild(item);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, 'c114-auto-minmax-span-overflow-item'] }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_five_fixed_overflow_automatic_minimum_scenes() {
        for (scene_id, root_id, overflow_x) in SCENES {
            let source = runtime_source(scene_id).expect("known fixed scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("id', '{root_id}'")));
            assert!(source.contains(&format!("overflow-x:{overflow_x}")));
            assert!(source.contains("background-color:#e5484d"));
            assert!(source.contains(
                "grid-template-columns:minmax(auto,max-content) minmax(auto,max-content)"
            ));
            assert!(source.contains("grid-column:1 / span 2"));
            assert!(source.contains("XXXXXXXXXX"));
        }
        assert!(runtime_source("columns-span2-hidden-zero-minimun").is_none());
        assert!(runtime_source("unlisted-overflow-auto-minimum").is_none());
    }
}
