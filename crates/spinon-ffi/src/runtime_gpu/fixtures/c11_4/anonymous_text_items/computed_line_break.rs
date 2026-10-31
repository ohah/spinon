#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    language: &'static str,
    line_break: &'static str,
    track: &'static str,
}

const SCENES: [Scene; 5] = [
    scene(
        "line-break-ja-normal-min-content",
        "ja",
        "normal",
        "min-content",
    ),
    scene(
        "line-break-ja-normal-max-content",
        "ja",
        "normal",
        "max-content",
    ),
    scene(
        "line-break-ja-strict-min-content",
        "ja",
        "strict",
        "min-content",
    ),
    scene(
        "line-break-ja-strict-max-content",
        "ja",
        "strict",
        "max-content",
    ),
    scene(
        "line-break-en-normal-min-content",
        "en",
        "normal",
        "min-content",
    ),
];

const fn scene(
    id: &'static str,
    language: &'static str,
    line_break: &'static str,
    track: &'static str,
) -> Scene {
    Scene {
        id,
        language,
        line_break,
        track,
    }
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:\"Noto Sans KR\";font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:{};text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.track, scene.line_break
    );
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('lang', {language:?});
root.setAttribute('style', {style:?});
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode('文〜文'));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        language = scene.language,
        style = style,
        scene_id = scene.id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_line_break_scenes_generate_the_exact_allowlisted_inputs() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 5);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted line-break scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("root.setAttribute('lang', {:?})", scene.language)));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.track)));
            assert!(source.contains(&format!("line-break:{};", scene.line_break)));
            assert!(source.contains("font-family:\\\"Noto Sans KR\\\";"));
            assert!(source.contains("createTextNode('文〜文')"));
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 1);
        }
        assert!(runtime_source("line-break-loose-min-content").is_none());
    }
}
