#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    ratio: &'static str,
    constraints: &'static str,
}

const SCENES: [Scene; 9] = [
    Scene {
        id: "span-baseline",
        ratio: "2 / 1",
        constraints: "",
    },
    Scene {
        id: "span-min-width-45",
        ratio: "2 / 1",
        constraints: "min-width:45px",
    },
    Scene {
        id: "span-min-width-60",
        ratio: "2 / 1",
        constraints: "min-width:60px",
    },
    Scene {
        id: "span-min-height-30",
        ratio: "2 / 1",
        constraints: "min-height:30px",
    },
    Scene {
        id: "span-max-width-30-min-width-0",
        ratio: "2 / 1",
        constraints: "min-width:0;max-width:30px",
    },
    Scene {
        id: "span-max-height-15-auto-min-control",
        ratio: "2 / 1",
        constraints: "min-height:0;max-height:15px",
    },
    Scene {
        id: "span-max-height-15-min-width-0",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:0;max-height:15px",
    },
    Scene {
        id: "span-max-height-17-75-range",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:0;max-height:17.75px",
    },
    Scene {
        id: "span-no-ratio",
        ratio: "auto",
        constraints: "",
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let case_id = scene_id.strip_prefix("grid-text-aspect-ratio-multispan-")?;
    let scene = SCENES.iter().find(|scene| scene.id == case_id)?;
    let root_id = format!("grid-text-aspect-ratio-multispan-{}", scene.id);
    let item_id = format!("{}-item", root_id);
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root_id}');
root.setAttribute('data-node', '');
root.setAttribute('class', 'grid');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;grid-template-columns:max-content max-content;grid-template-rows:auto;grid-auto-flow:row;column-gap:0;row-gap:0;align-items:normal;justify-items:normal;align-content:normal;justify-content:normal;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#ffffff');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', '{item_id}');
item.setAttribute('data-node', '');
item.setAttribute('class', 'item ratio');
item.setAttribute('style', 'box-sizing:content-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;grid-column:1 / span 2;aspect-ratio:{ratio};{constraints};font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#111');
item.appendChild(document.createTextNode('AAAA'));
root.appendChild(item);
return {{ sceneId:'{scene_id}', rootId:'{root_id}', nodeIds:['{root_id}', '{item_id}'] }};
}})()"#,
        ratio = scene.ratio,
        constraints = scene.constraints,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_exact_direct_text_ratio_multispan_cases() {
        assert_eq!(SCENES.len(), 9);
        for scene in SCENES {
            let route = format!("grid-text-aspect-ratio-multispan-{}", scene.id);
            let source = runtime_source(&route).expect("registered two-track span scene");
            let routed = super::super::c11_4_grid_track_sizing_runtime_source(&route)
                .expect("runtime dispatcher accepts the span scene");
            assert!(source.len() < 4 * 1024, "{route}");
            assert!(routed.contains(&format!("sceneId:'{route}'")), "{route}");
            assert!(source.contains("grid-template-columns:max-content max-content"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains("grid-column:1 / span 2"));
            assert!(!source.contains("grid-row:"));
            assert!(source.contains("aspect-ratio:"));
            assert!(source.contains(scene.ratio));
            assert!(source.contains(scene.constraints));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("document.createTextNode('AAAA')").count(), 1);
        }
        assert!(runtime_source("grid-text-aspect-ratio-multispan-unlisted").is_none());
    }
}
