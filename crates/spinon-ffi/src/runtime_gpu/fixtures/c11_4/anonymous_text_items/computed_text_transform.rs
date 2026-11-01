#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    transform: &'static str,
    columns: &'static str,
    text: &'static str,
}

const SCENES: [Scene; 12] = [
    Scene {
        id: "text-transform-none-min-content",
        transform: "none",
        columns: "min-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-none-max-content",
        transform: "none",
        columns: "max-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-uppercase-min-content",
        transform: "uppercase",
        columns: "min-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-uppercase-max-content",
        transform: "uppercase",
        columns: "max-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-lowercase-min-content",
        transform: "lowercase",
        columns: "min-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-lowercase-max-content",
        transform: "lowercase",
        columns: "max-content",
        text: "IIII ii",
    },
    Scene {
        id: "text-transform-capitalize-none-min-content",
        transform: "none",
        columns: "min-content",
        text: "iiii ii",
    },
    Scene {
        id: "text-transform-capitalize-none-max-content",
        transform: "none",
        columns: "max-content",
        text: "iiii ii",
    },
    Scene {
        id: "text-transform-capitalize-min-content",
        transform: "capitalize",
        columns: "min-content",
        text: "iiii ii",
    },
    Scene {
        id: "text-transform-capitalize-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "iiii ii",
    },
    Scene {
        id: "text-transform-capitalize-mixed-case",
        transform: "capitalize",
        columns: "max-content",
        text: "iIII ii",
    },
    Scene {
        id: "text-transform-capitalize-collapsed-spaces",
        transform: "capitalize",
        columns: "max-content",
        text: "ii   ii",
    },
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:\"Noto Sans KR\";font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:{};letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.columns, scene.transform
    );
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('lang', 'en');
root.setAttribute('style', {style:?});
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        scene_id = scene.id,
        text = scene.text,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_text_transform_generates_the_fixed_direct_language_scene_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 12);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted text-transform scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("root.setAttribute('lang', 'en')"));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.columns)));
            assert!(source.contains(&format!("text-transform:{};", scene.transform)));
            assert!(source.contains("font-family:\\\"Noto Sans KR\\\";"));
            assert!(source.contains(&format!("createTextNode({:?})", scene.text)));
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 1);
        }
        assert!(runtime_source("unlisted-text-transform-scene").is_none());
    }
}
