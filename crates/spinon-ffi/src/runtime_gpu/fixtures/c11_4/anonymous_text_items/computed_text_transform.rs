#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    transform: &'static str,
    columns: &'static str,
    text: &'static str,
    source_text_nodes: Option<&'static [&'static str]>,
}

const SCENES: [Scene; 23] = [
    Scene {
        id: "text-transform-none-min-content",
        transform: "none",
        columns: "min-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-none-max-content",
        transform: "none",
        columns: "max-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-uppercase-min-content",
        transform: "uppercase",
        columns: "min-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-uppercase-max-content",
        transform: "uppercase",
        columns: "max-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-lowercase-min-content",
        transform: "lowercase",
        columns: "min-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-lowercase-max-content",
        transform: "lowercase",
        columns: "max-content",
        text: "IIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-none-min-content",
        transform: "none",
        columns: "min-content",
        text: "iiii ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-none-max-content",
        transform: "none",
        columns: "max-content",
        text: "iiii ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-min-content",
        transform: "capitalize",
        columns: "min-content",
        text: "iiii ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "iiii ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-mixed-case",
        transform: "capitalize",
        columns: "max-content",
        text: "iIII ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-capitalize-collapsed-spaces",
        transform: "capitalize",
        columns: "max-content",
        text: "ii   ii",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-sharp-s-min-content",
        transform: "capitalize",
        columns: "min-content",
        text: "ßeta",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-sharp-s-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "ßeta",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-source-split-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "ßeta",
        source_text_nodes: Some(&["ß", "eta"]),
    },
    Scene {
        id: "text-transform-unicode-accented-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "éclair",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-combining-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "e\u{0301}lan",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-hyphen-boundary-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "well-known",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-apostrophe-word-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "o'neill",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-opening-punctuation-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "«élan»",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-number-first-none",
        transform: "capitalize",
        columns: "max-content",
        text: "49ers",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-number-first-control",
        transform: "none",
        columns: "max-content",
        text: "49ers",
        source_text_nodes: None,
    },
    Scene {
        id: "text-transform-unicode-collapsed-space-max-content",
        transform: "capitalize",
        columns: "max-content",
        text: "ßeta   éclair",
        source_text_nodes: None,
    },
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:\"Noto Sans KR\";font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:{};letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.columns, scene.transform
    );
    let single_text_node = [scene.text];
    let source_text_nodes = scene.source_text_nodes.unwrap_or(&single_text_node);
    let text_nodes = source_text_nodes
        .iter()
        .map(|text| format!("root.appendChild(document.createTextNode({text:?}));"))
        .collect::<String>();
    Some(format!(
        r#"(() => {{
const previousState = globalThis.__spinonC114TextTransformRetryRoot;
if (previousState !== undefined
    && previousState.sceneId === {scene_id:?}
    && previousState.root.parentNode === document) {{
  return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:previousState.nodeIds }};
}}
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('lang', 'en');
root.setAttribute('style', {style:?});
document.appendChild(root);
const nodeIds = [{root_id:?}];
{text_nodes}
globalThis.__spinonC114TextTransformRetryRoot = {{ sceneId:{scene_id:?}, root, nodeIds }};
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        scene_id = scene.id,
        text_nodes = text_nodes,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_text_transform_generates_the_fixed_direct_language_scene_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 23);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted text-transform scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("root.setAttribute('lang', 'en')"));
            assert!(source.contains("previousState.root.parentNode === document"));
            assert!(source.contains(&format!("previousState.sceneId === {:?}", scene.id)));
            assert!(source.contains("nodeIds:previousState.nodeIds"));
            assert!(source.contains(&format!(
                "globalThis.__spinonC114TextTransformRetryRoot = {{ sceneId:{:?}, root, nodeIds }}",
                scene.id
            )));
            assert!(
                source
                    .find("previousState.root.parentNode === document")
                    .unwrap()
                    < source
                        .find("const root = document.createElement('div')")
                        .unwrap()
            );
            assert!(source.contains(&format!("grid-template-columns:{};", scene.columns)));
            assert!(source.contains(&format!("text-transform:{};", scene.transform)));
            assert!(source.contains("font-family:\\\"Noto Sans KR\\\";"));
            let single_source_node = [scene.text];
            let source_text_nodes = scene.source_text_nodes.unwrap_or(&single_source_node);
            for text in source_text_nodes {
                assert!(source.contains(&format!("createTextNode({text:?})")));
            }
            let retry_state_assignment = source
                .find(&format!(
                    "globalThis.__spinonC114TextTransformRetryRoot = {{ sceneId:{:?}, root, nodeIds }}",
                    scene.id
                ))
                .unwrap();
            let final_text_node = source.rfind("document.createTextNode(").unwrap();
            assert!(final_text_node < retry_state_assignment);
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                source_text_nodes.len()
            );
            assert_eq!(source.matches("document.createElement('div')").count(), 1);
        }
        assert!(runtime_source("unlisted-text-transform-scene").is_none());
    }
}
