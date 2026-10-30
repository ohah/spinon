#[derive(Clone, Copy)]
struct Scene {
    route_id: &'static str,
    root_id: &'static str,
    row_template: &'static str,
    first_id: &'static str,
    first_text: &'static str,
    first_ratio: Option<&'static str>,
    second_id: &'static str,
    second_text: &'static str,
    second_ratio: Option<&'static str>,
}

const SCENES: [Scene; 16] = [
    Scene {
        route_id: "grid-text-aspect-ratio-row",
        root_id: "row-ratio",
        row_template: "auto",
        first_id: "row-ratio-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-ratio-long",
        second_text: "AAAA",
        second_ratio: Some("2 / 1"),
    },
    Scene {
        route_id: "grid-text-aspect-ratio-row-control",
        root_id: "row-control",
        row_template: "auto",
        first_id: "row-control-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-control-long",
        second_text: "AAAA",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-column",
        root_id: "column-ratio",
        row_template: "40px",
        first_id: "column-ratio-long",
        first_text: "AAAA",
        first_ratio: Some("2 / 1"),
        second_id: "column-ratio-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-column-control",
        root_id: "column-control",
        row_template: "40px",
        first_id: "column-control-long",
        first_text: "AAAA",
        first_ratio: None,
        second_id: "column-control-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-row-1-2",
        root_id: "row-1-2",
        row_template: "auto",
        first_id: "row-1-2-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-1-2-long",
        second_text: "AAAA",
        second_ratio: Some("1 / 2"),
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-row-4-3",
        root_id: "row-4-3",
        row_template: "auto",
        first_id: "row-4-3-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-4-3-long",
        second_text: "AAAA",
        second_ratio: Some("4 / 3"),
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-row-3-2",
        root_id: "row-3-2",
        row_template: "auto",
        first_id: "row-3-2-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-3-2-long",
        second_text: "AAAA",
        second_ratio: Some("3 / 2"),
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-row-5-2",
        root_id: "row-5-2",
        row_template: "auto",
        first_id: "row-5-2-short",
        first_text: "A",
        first_ratio: None,
        second_id: "row-5-2-long",
        second_text: "AAAA",
        second_ratio: Some("5 / 2"),
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-1-2",
        root_id: "column-1-2",
        row_template: "60px",
        first_id: "column-1-2-long",
        first_text: "AAAA",
        first_ratio: Some("1 / 2"),
        second_id: "column-1-2-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-4-3",
        root_id: "column-4-3",
        row_template: "45px",
        first_id: "column-4-3-long",
        first_text: "AAAA",
        first_ratio: Some("4 / 3"),
        second_id: "column-4-3-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-3-2",
        root_id: "column-3-2",
        row_template: "30px",
        first_id: "column-3-2-long",
        first_text: "AAAA",
        first_ratio: Some("3 / 2"),
        second_id: "column-3-2-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-5-2",
        root_id: "column-5-2",
        row_template: "20px",
        first_id: "column-5-2-long",
        first_text: "AAAA",
        first_ratio: Some("5 / 2"),
        second_id: "column-5-2-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-control-60",
        root_id: "column-control-60",
        row_template: "60px",
        first_id: "column-control-60-long",
        first_text: "AAAA",
        first_ratio: None,
        second_id: "column-control-60-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-control-45",
        root_id: "column-control-45",
        row_template: "45px",
        first_id: "column-control-45-long",
        first_text: "AAAA",
        first_ratio: None,
        second_id: "column-control-45-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-control-30",
        root_id: "column-control-30",
        row_template: "30px",
        first_id: "column-control-30-long",
        first_text: "AAAA",
        first_ratio: None,
        second_id: "column-control-30-short",
        second_text: "A",
        second_ratio: None,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-range-column-control-20",
        root_id: "column-control-20",
        row_template: "20px",
        first_id: "column-control-20-long",
        first_text: "AAAA",
        first_ratio: None,
        second_id: "column-control-20-short",
        second_text: "A",
        second_ratio: None,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.route_id == scene_id)?;
    let first_ratio = scene
        .first_ratio
        .map(|ratio| format!("aspect-ratio:{ratio};"))
        .unwrap_or_default();
    let second_ratio = scene
        .second_ratio
        .map(|ratio| format!("aspect-ratio:{ratio};"))
        .unwrap_or_default();
    let first_ratio_class = if scene.first_ratio.is_some() {
        " ratio"
    } else {
        ""
    };
    let second_ratio_class = if scene.second_ratio.is_some() {
        " ratio"
    } else {
        ""
    };
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{}');
root.setAttribute('data-node', '');
root.setAttribute('class', 'grid');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;grid-template-columns:max-content max-content;grid-template-rows:{};grid-auto-flow:row;column-gap:0;row-gap:0;align-items:normal;justify-items:normal;align-content:normal;justify-content:normal;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#ffffff');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', '{}');
first.setAttribute('data-node', '');
first.setAttribute('class', 'item{}');
first.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#111;{}');
first.appendChild(document.createTextNode('{}'));
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', '{}');
second.setAttribute('data-node', '');
second.setAttribute('class', 'item{}');
second.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:inherit;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;overflow-wrap:normal;background-color:#3a86ff;color:#111;{}');
second.appendChild(document.createTextNode('{}'));
root.appendChild(second);
return {{ sceneId:'{}', rootId:'{}', nodeIds:['{}', '{}', '{}'] }};
}})()"#,
        scene.root_id,
        scene.row_template,
        scene.first_id,
        first_ratio_class,
        first_ratio,
        scene.first_text,
        scene.second_id,
        second_ratio_class,
        second_ratio,
        scene.second_text,
        scene.route_id,
        scene.root_id,
        scene.root_id,
        scene.first_id,
        scene.second_id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_fixed_direct_text_aspect_ratio_range_and_regression_scenes() {
        assert_eq!(SCENES.len(), 16);
        for scene in SCENES {
            let source = runtime_source(scene.route_id).expect("registered ratio scene");
            assert!(source.len() < 4 * 1024, "{}", scene.route_id);
            assert!(source.contains(&format!("sceneId:'{}'", scene.route_id)));
            assert!(source.contains(&format!("rootId:'{}'", scene.root_id)));
            assert!(source.contains("grid-template-columns:max-content max-content"));
            assert!(source.contains("column-gap:0;row-gap:0;align-items:normal"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode('").count(), 2);
            assert_eq!(
                source.matches("aspect-ratio:").count(),
                usize::from(scene.first_ratio.is_some())
                    + usize::from(scene.second_ratio.is_some())
            );
            for ratio in [scene.first_ratio, scene.second_ratio]
                .into_iter()
                .flatten()
            {
                assert!(source.contains(&format!("aspect-ratio:{ratio};")));
            }
        }
        assert!(runtime_source("unknown-grid-text-aspect-ratio-scene").is_none());
    }
}
