#[derive(Clone, Copy)]
enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}

#[derive(Clone, Copy)]
enum Track {
    MinContent,
    MaxContent,
    Fixed10Px,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    overflow_wrap: &'static str,
    columns: &'static str,
}

const SCENES: [Scene; 9] = [
    scene(OverflowWrap::Normal, Track::MinContent),
    scene(OverflowWrap::Normal, Track::MaxContent),
    scene(OverflowWrap::Normal, Track::Fixed10Px),
    scene(OverflowWrap::Anywhere, Track::MinContent),
    scene(OverflowWrap::Anywhere, Track::MaxContent),
    scene(OverflowWrap::Anywhere, Track::Fixed10Px),
    scene(OverflowWrap::BreakWord, Track::MinContent),
    scene(OverflowWrap::BreakWord, Track::MaxContent),
    scene(OverflowWrap::BreakWord, Track::Fixed10Px),
];

const fn scene(overflow_wrap: OverflowWrap, track: Track) -> Scene {
    let css_overflow_wrap = match overflow_wrap {
        OverflowWrap::Normal => "normal",
        OverflowWrap::Anywhere => "anywhere",
        OverflowWrap::BreakWord => "break-word",
    };
    let columns = match track {
        Track::MinContent => "min-content",
        Track::MaxContent => "max-content",
        Track::Fixed10Px => "10px",
    };
    Scene {
        id: match (overflow_wrap, track) {
            (OverflowWrap::Normal, Track::MinContent) => "overflow-wrap-normal-min-content",
            (OverflowWrap::Normal, Track::MaxContent) => "overflow-wrap-normal-max-content",
            (OverflowWrap::Normal, Track::Fixed10Px) => "overflow-wrap-normal-fixed-10px",
            (OverflowWrap::Anywhere, Track::MinContent) => "overflow-wrap-anywhere-min-content",
            (OverflowWrap::Anywhere, Track::MaxContent) => "overflow-wrap-anywhere-max-content",
            (OverflowWrap::Anywhere, Track::Fixed10Px) => "overflow-wrap-anywhere-fixed-10px",
            (OverflowWrap::BreakWord, Track::MinContent) => "overflow-wrap-break-word-min-content",
            (OverflowWrap::BreakWord, Track::MaxContent) => "overflow-wrap-break-word-max-content",
            (OverflowWrap::BreakWord, Track::Fixed10Px) => "overflow-wrap-break-word-fixed-10px",
        },
        overflow_wrap: css_overflow_wrap,
        columns,
    }
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:{};text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.columns, scene.overflow_wrap
    );
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', {style:?});
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode('XXXX'));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        scene_id = scene.id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_overflow_wrap_scenes_generate_exact_direct_text_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 9);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted overflow-wrap scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("overflow-wrap:{};", scene.overflow_wrap)));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.columns)));
            assert!(source.contains("grid-template-rows:auto;"));
            assert!(source.contains("createTextNode('XXXX')"));
            assert!(source.contains("word-break:normal;"));
            assert_eq!(source.matches("createTextNode(").count(), 1);
            assert_eq!(source.matches("createElement('div')").count(), 1);
        }
        assert!(runtime_source("overflow-wrap-unlisted").is_none());
    }
}
