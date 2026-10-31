#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    text: &'static str,
    language: &'static str,
    font_family: &'static str,
    word_break: &'static str,
    track: &'static str,
}

const SCENES: [Scene; 12] = [
    scene(
        "word-break-normal-unbroken-min-content",
        "XXXX",
        "en",
        "Ahem",
        "normal",
        "min-content",
    ),
    scene(
        "word-break-normal-unbroken-max-content",
        "XXXX",
        "en",
        "Ahem",
        "normal",
        "max-content",
    ),
    scene(
        "word-break-break-all-unbroken-min-content",
        "XXXX",
        "en",
        "Ahem",
        "break-all",
        "min-content",
    ),
    scene(
        "word-break-break-all-unbroken-max-content",
        "XXXX",
        "en",
        "Ahem",
        "break-all",
        "max-content",
    ),
    scene(
        "word-break-normal-two-words-min-content",
        "XX XX",
        "en",
        "Ahem",
        "normal",
        "min-content",
    ),
    scene(
        "word-break-normal-two-words-max-content",
        "XX XX",
        "en",
        "Ahem",
        "normal",
        "max-content",
    ),
    scene(
        "word-break-break-all-two-words-min-content",
        "XX XX",
        "en",
        "Ahem",
        "break-all",
        "min-content",
    ),
    scene(
        "word-break-break-all-two-words-max-content",
        "XX XX",
        "en",
        "Ahem",
        "break-all",
        "max-content",
    ),
    scene(
        "word-break-normal-hangul-min-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        "normal",
        "min-content",
    ),
    scene(
        "word-break-normal-hangul-max-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        "normal",
        "max-content",
    ),
    scene(
        "word-break-keep-all-hangul-min-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        "keep-all",
        "min-content",
    ),
    scene(
        "word-break-keep-all-hangul-max-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        "keep-all",
        "max-content",
    ),
];

const fn scene(
    id: &'static str,
    text: &'static str,
    language: &'static str,
    font_family: &'static str,
    word_break: &'static str,
    track: &'static str,
) -> Scene {
    Scene {
        id,
        text,
        language,
        font_family,
        word_break,
        track,
    }
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let font_family = if scene.font_family == "Noto Sans KR" {
        "\"Noto Sans KR\""
    } else {
        scene.font_family
    };
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:{};font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:{};direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.track, font_family, scene.word_break
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
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        language = scene.language,
        style = style,
        text = scene.text,
        scene_id = scene.id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_word_break_scenes_generate_exact_allowlisted_direct_text_inputs() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 12);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted word-break scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("root.setAttribute('lang', {:?})", scene.language)));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.track)));
            assert!(source.contains(&format!("word-break:{};", scene.word_break)));
            assert!(source.contains(&format!("createTextNode({:?})", scene.text)));
            assert!(source.contains("line-break:auto;"));
            assert!(source.contains("overflow-wrap:normal;"));
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 1);
        }
        assert!(runtime_source("word-break-break-word-unbroken-min-content").is_none());
    }
}
