#[derive(Clone, Copy)]
struct Scene {
    route_id: &'static str,
    root_id: &'static str,
    long_width: &'static str,
    long_height: &'static str,
    ratio: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        route_id: "grid-text-aspect-ratio-definite-width-ratio",
        root_id: "definite-width-ratio",
        long_width: "60px",
        long_height: "auto",
        ratio: true,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-definite-width-control",
        root_id: "definite-width-control",
        long_width: "60px",
        long_height: "auto",
        ratio: false,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-definite-height-ratio",
        root_id: "definite-height-ratio",
        long_width: "auto",
        long_height: "30px",
        ratio: true,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-definite-height-control",
        root_id: "definite-height-control",
        long_width: "auto",
        long_height: "30px",
        ratio: false,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.route_id == scene_id)?;
    let aspect_ratio = if scene.ratio {
        "aspect-ratio:2 / 1;"
    } else {
        "aspect-ratio:auto;"
    };
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{}');
root.setAttribute('data-node', '');
root.setAttribute('class', 'grid');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;grid-template-columns:max-content max-content;grid-template-rows:auto;grid-auto-flow:row;column-gap:0;row-gap:0;align-items:normal;justify-items:normal;align-content:normal;justify-content:normal;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#ffffff');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', 'definite-axis-short');
first.setAttribute('data-node', '');
first.setAttribute('class', 'item short');
first.setAttribute('style', 'box-sizing:content-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#111;aspect-ratio:auto');
first.appendChild(document.createTextNode('A'));
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', 'definite-axis-long');
second.setAttribute('data-node', '');
second.setAttribute('class', 'item long');
second.setAttribute('style', 'box-sizing:content-box;display:block;position:static;width:{};height:{};min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#3a86ff;color:#111;{}');
second.appendChild(document.createTextNode('AAAA'));
root.appendChild(second);
return {{ sceneId:'{}', rootId:'{}', nodeIds:['{}', 'definite-axis-short', 'definite-axis-long'] }};
}})()"#,
        scene.root_id,
        scene.long_width,
        scene.long_height,
        aspect_ratio,
        scene.route_id,
        scene.root_id,
        scene.root_id,
    ))
}

#[cfg(test)]
mod tests {
    use super::super::c11_4_grid_track_sizing_runtime_source;
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_only_the_four_definite_axis_ratio_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.route_id).expect("known definite-axis scene");
            assert!(source.len() < 4 * 1024, "{}", scene.route_id);
            assert!(source.contains(&format!("sceneId:'{}'", scene.route_id)));
            assert!(source.contains(&format!("rootId:'{}'", scene.root_id)));
            assert!(source.contains("grid-template-columns:max-content max-content"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains(&format!(
                "width:{};height:{};",
                scene.long_width, scene.long_height
            )));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode('").count(), 2);
            assert_eq!(
                source.matches("aspect-ratio:2 / 1;").count(),
                usize::from(scene.ratio)
            );
            assert!(c11_4_grid_track_sizing_runtime_source(scene.route_id).is_ok());
        }
        assert!(runtime_source("grid-text-aspect-ratio-definite-width-typo").is_none());
        assert!(
            c11_4_grid_track_sizing_runtime_source("grid-text-aspect-ratio-definite-width-typo")
                .is_err()
        );
    }
}
