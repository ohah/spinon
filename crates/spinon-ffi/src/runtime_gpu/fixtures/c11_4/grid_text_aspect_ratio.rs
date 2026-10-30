#[derive(Clone, Copy)]
struct Scene {
    route_id: &'static str,
    root_id: &'static str,
    row_template: &'static str,
    first_id: &'static str,
    first_text: &'static str,
    first_has_ratio: bool,
    second_id: &'static str,
    second_text: &'static str,
    second_has_ratio: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        route_id: "grid-text-aspect-ratio-row",
        root_id: "row-ratio",
        row_template: "auto",
        first_id: "row-ratio-short",
        first_text: "A",
        first_has_ratio: false,
        second_id: "row-ratio-long",
        second_text: "AAAA",
        second_has_ratio: true,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-row-control",
        root_id: "row-control",
        row_template: "auto",
        first_id: "row-control-short",
        first_text: "A",
        first_has_ratio: false,
        second_id: "row-control-long",
        second_text: "AAAA",
        second_has_ratio: false,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-column",
        root_id: "column-ratio",
        row_template: "40px",
        first_id: "column-ratio-long",
        first_text: "AAAA",
        first_has_ratio: true,
        second_id: "column-ratio-short",
        second_text: "A",
        second_has_ratio: false,
    },
    Scene {
        route_id: "grid-text-aspect-ratio-column-control",
        root_id: "column-control",
        row_template: "40px",
        first_id: "column-control-long",
        first_text: "AAAA",
        first_has_ratio: false,
        second_id: "column-control-short",
        second_text: "A",
        second_has_ratio: false,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.route_id == scene_id)?;
    let first_ratio = if scene.first_has_ratio {
        "aspect-ratio:2 / 1;"
    } else {
        ""
    };
    let second_ratio = if scene.second_has_ratio {
        "aspect-ratio:2 / 1;"
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
        if scene.first_has_ratio { " ratio" } else { "" },
        first_ratio,
        scene.first_text,
        scene.second_id,
        if scene.second_has_ratio { " ratio" } else { "" },
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
    fn exposes_four_paired_direct_text_aspect_ratio_scenes() {
        assert_eq!(SCENES.len(), 4);
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
                source.matches("aspect-ratio:2 / 1;").count(),
                usize::from(scene.first_has_ratio) + usize::from(scene.second_has_ratio)
            );
        }
        assert!(runtime_source("unknown-grid-text-aspect-ratio-scene").is_none());
    }
}
