#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Scene {
    id: &'static str,
    text: &'static str,
    row_track: &'static str,
}

const PNG_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

const SCENES: [Scene; 10] = [
    Scene {
        id: "single-token-min-content",
        text: "ABC",
        row_track: "min-content",
    },
    Scene {
        id: "single-token-max-content",
        text: "ABC",
        row_track: "max-content",
    },
    Scene {
        id: "two-words-min-content",
        text: "A BC",
        row_track: "min-content",
    },
    Scene {
        id: "two-words-max-content",
        text: "A BC",
        row_track: "max-content",
    },
    Scene {
        id: "three-words-min-content",
        text: "AB CDE F",
        row_track: "min-content",
    },
    Scene {
        id: "three-words-max-content",
        text: "AB CDE F",
        row_track: "max-content",
    },
    Scene {
        id: "collapsed-whitespace-min-content",
        text: "  A   9Z  ",
        row_track: "min-content",
    },
    Scene {
        id: "collapsed-whitespace-max-content",
        text: "  A   9Z  ",
        row_track: "max-content",
    },
    Scene {
        id: "spaces-only-min-content",
        text: "   ",
        row_track: "min-content",
    },
    Scene {
        id: "spaces-only-max-content",
        text: "   ",
        row_track: "max-content",
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let root_id = format!("c114-orthogonal-text-{}-grid", scene.id);
    let text_id = format!("c114-orthogonal-text-{}-text", scene.id);
    let image_id = format!("c114-orthogonal-text-{}-image", scene.id);
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:74px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;direction:ltr;writing-mode:horizontal-tb;align-items:normal;align-content:start;justify-items:normal;justify-content:start;grid-template-columns:50px 24px;grid-template-rows:{row_track};background-color:#101820');
document.appendChild(root);
const text = document.createElement('div');
text.setAttribute('id', {text_id:?});
text.setAttribute('data-node', '');
text.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1 / span 1;grid-row:1;writing-mode:vertical-rl;direction:ltr;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;line-break:normal;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#fff');
text.appendChild(document.createTextNode({text:?}));
root.appendChild(text);
const image = document.createElement('img');
image.setAttribute('id', {image_id:?});
image.setAttribute('data-node', '');
image.setAttribute('src', {PNG_URL:?});
image.setAttribute('alt', '');
image.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:2 / span 1;grid-row:1;background-color:#2864dc');
root.appendChild(image);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?},{text_id:?},{image_id:?}] }};
}})()"#,
        scene_id = scene.id,
        row_track = scene.row_track,
        text = scene.text,
    )
}

#[cfg(test)]
mod tests {
    use super::{PNG_URL, SCENES, runtime_source};

    #[test]
    fn exposes_the_ten_fixed_orthogonal_text_intrinsic_scenes() {
        assert_eq!(SCENES.len(), 10);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            assert!(
                scene
                    .text
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == ' ')
            );
            assert!(!scene.text.is_empty());
            let source = runtime_source(scene.id).expect("listed orthogonal text scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("width:74px;height:auto"));
            assert!(source.contains("grid-template-columns:50px 24px"));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.row_track)));
            assert!(source.contains("writing-mode:horizontal-tb"));
            assert!(source.contains("writing-mode:vertical-rl;direction:ltr"));
            assert!(!source.contains("text-orientation:"));
            assert!(source.contains("font-family:Ahem;font-size:10px;font-weight:400"));
            assert!(
                source
                    .contains("line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap")
            );
            assert!(source.contains("word-break:normal;line-break:normal"));
            assert!(source.contains(&format!("document.createTextNode({:?})", scene.text)));
            assert!(source.contains(PNG_URL));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("grid-row:1").count(), 2);
        }
        assert!(runtime_source("spaces-only-no-image-min-content").is_none());
        assert!(runtime_source("unknown;throw 1").is_none());
    }
}
