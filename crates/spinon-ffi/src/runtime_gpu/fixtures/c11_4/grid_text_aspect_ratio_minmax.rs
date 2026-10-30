#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    ratio: &'static str,
    constraints: &'static str,
    edges: bool,
}

const SCENES: [Scene; 19] = [
    Scene {
        id: "numeric-baseline",
        ratio: "2 / 1",
        constraints: "",
        edges: false,
    },
    Scene {
        id: "min-width-60",
        ratio: "2 / 1",
        constraints: "min-width:60px",
        edges: false,
    },
    Scene {
        id: "min-width-45",
        ratio: "2 / 1",
        constraints: "min-width:45px",
        edges: false,
    },
    Scene {
        id: "max-width-30-min-width-0",
        ratio: "2 / 1",
        constraints: "min-width:0;max-width:30px",
        edges: false,
    },
    Scene {
        id: "min-height-30",
        ratio: "2 / 1",
        constraints: "min-height:30px",
        edges: false,
    },
    Scene {
        id: "max-height-15-min-height-0",
        ratio: "2 / 1",
        constraints: "min-height:0;max-height:15px",
        edges: false,
    },
    Scene {
        id: "max-height-15-min-height-0-min-width-0",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:0;max-height:15px",
        edges: false,
    },
    Scene {
        id: "max-height-17-75-min-controls",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:0;max-height:17.75px",
        edges: false,
    },
    Scene {
        id: "min-width-60-clamped-by-max-height-20",
        ratio: "2 / 1",
        constraints: "min-width:60px;max-height:20px",
        edges: false,
    },
    Scene {
        id: "min-height-20-clamps-transferred-max-width-30",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:20px;max-width:30px",
        edges: false,
    },
    Scene {
        id: "source-min-width-60-over-max-width-40",
        ratio: "2 / 1",
        constraints: "min-width:60px;max-width:40px",
        edges: false,
    },
    Scene {
        id: "source-min-height-30-over-max-height-20",
        ratio: "2 / 1",
        constraints: "min-height:30px;max-height:20px",
        edges: false,
    },
    Scene {
        id: "auto-ratio-min-width-60",
        ratio: "auto 2 / 1",
        constraints: "min-width:60px",
        edges: false,
    },
    Scene {
        id: "no-ratio-min-width-60",
        ratio: "auto",
        constraints: "min-width:60px",
        edges: false,
    },
    Scene {
        id: "no-ratio-min-height-30",
        ratio: "auto",
        constraints: "min-height:30px",
        edges: false,
    },
    Scene {
        id: "numeric-border-box-min-width-60",
        ratio: "2 / 1",
        constraints: "min-width:60px",
        edges: true,
    },
    Scene {
        id: "auto-content-box-min-width-60",
        ratio: "auto 2 / 1",
        constraints: "min-width:60px",
        edges: true,
    },
    Scene {
        id: "numeric-border-box-max-height-15-min-controls",
        ratio: "2 / 1",
        constraints: "min-width:0;min-height:0;max-height:15px",
        edges: true,
    },
    Scene {
        id: "auto-content-box-max-height-15-min-controls",
        ratio: "auto 2 / 1",
        constraints: "min-width:0;min-height:0;max-height:15px",
        edges: true,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let case_id = scene_id.strip_prefix("grid-text-aspect-ratio-minmax-")?;
    let scene = SCENES.iter().find(|scene| scene.id == case_id)?;
    let edges = if scene.edges {
        "padding:2px 4px;border:1px solid;"
    } else {
        "padding:0;border:0;"
    };
    let long_id = format!("{}-long", scene.id);
    let short_id = format!("{}-short", scene.id);
    let root_id = format!("grid-text-ratio-minmax-{}", scene.id);
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root_id}');
root.setAttribute('data-node', '');
root.setAttribute('class', 'grid');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;grid-template-columns:max-content max-content;grid-template-rows:auto;grid-auto-flow:row;column-gap:0;row-gap:0;align-items:normal;justify-items:normal;align-content:normal;justify-content:normal;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#ffffff');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', '{long_id}');
first.setAttribute('data-node', '');
first.setAttribute('class', 'item ratio');
first.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;{edges}aspect-ratio:{ratio};{constraints};font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#111');
first.appendChild(document.createTextNode('AAAA'));
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', '{short_id}');
second.setAttribute('data-node', '');
second.setAttribute('class', 'item');
second.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#3a86ff;color:#111');
second.appendChild(document.createTextNode('A'));
root.appendChild(second);
return {{ sceneId:'grid-text-aspect-ratio-minmax-{id}', rootId:'{root_id}', nodeIds:['{root_id}', '{long_id}', '{short_id}'] }};
}})()"#,
        id = scene.id,
        ratio = scene.ratio,
        constraints = scene.constraints,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_fixed_direct_text_ratio_minmax_scenes() {
        assert_eq!(SCENES.len(), 19);
        for scene in SCENES {
            let route = format!("grid-text-aspect-ratio-minmax-{}", scene.id);
            let source =
                runtime_source(&route).expect("registered direct-text ratio min/max scene");
            let routed = super::super::c11_4_grid_track_sizing_runtime_source(&route)
                .expect("runtime dispatcher accepts the ratio min/max scene");
            assert!(source.len() < 4 * 1024, "{route}");
            assert!(routed.contains(&format!("sceneId:'{route}'")), "{route}");
            assert!(source.contains("grid-template-columns:max-content max-content"));
            assert!(source.contains("aspect-ratio:"));
            assert!(source.contains(scene.ratio));
            assert!(source.contains(scene.constraints));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode('").count(), 2);
        }
        assert!(runtime_source("unknown-grid-text-aspect-ratio-minmax").is_none());
    }
}
