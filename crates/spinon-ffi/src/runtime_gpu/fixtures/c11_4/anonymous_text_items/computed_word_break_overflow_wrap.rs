#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WordBreak {
    Normal,
    BreakWord,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OverflowWrap {
    Normal,
    BreakWord,
    Anywhere,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Track {
    MinContent,
    MaxContent,
    Fixed10Px,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    word_break: WordBreak,
    overflow_wrap: OverflowWrap,
    track: Track,
}

const SCENES: [Scene; 18] = [
    scene(WordBreak::Normal, OverflowWrap::Normal, Track::MinContent),
    scene(WordBreak::Normal, OverflowWrap::Normal, Track::MaxContent),
    scene(WordBreak::Normal, OverflowWrap::Normal, Track::Fixed10Px),
    scene(
        WordBreak::Normal,
        OverflowWrap::BreakWord,
        Track::MinContent,
    ),
    scene(
        WordBreak::Normal,
        OverflowWrap::BreakWord,
        Track::MaxContent,
    ),
    scene(WordBreak::Normal, OverflowWrap::BreakWord, Track::Fixed10Px),
    scene(WordBreak::Normal, OverflowWrap::Anywhere, Track::MinContent),
    scene(WordBreak::Normal, OverflowWrap::Anywhere, Track::MaxContent),
    scene(WordBreak::Normal, OverflowWrap::Anywhere, Track::Fixed10Px),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::Normal,
        Track::MinContent,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::Normal,
        Track::MaxContent,
    ),
    scene(WordBreak::BreakWord, OverflowWrap::Normal, Track::Fixed10Px),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::BreakWord,
        Track::MinContent,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::BreakWord,
        Track::MaxContent,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::BreakWord,
        Track::Fixed10Px,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::Anywhere,
        Track::MinContent,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::Anywhere,
        Track::MaxContent,
    ),
    scene(
        WordBreak::BreakWord,
        OverflowWrap::Anywhere,
        Track::Fixed10Px,
    ),
];

const fn scene(word_break: WordBreak, overflow_wrap: OverflowWrap, track: Track) -> Scene {
    Scene {
        id: match (word_break, overflow_wrap, track) {
            (WordBreak::Normal, OverflowWrap::Normal, Track::MinContent) => {
                "word-break-normal-overflow-wrap-normal-min-content"
            }
            (WordBreak::Normal, OverflowWrap::Normal, Track::MaxContent) => {
                "word-break-normal-overflow-wrap-normal-max-content"
            }
            (WordBreak::Normal, OverflowWrap::Normal, Track::Fixed10Px) => {
                "word-break-normal-overflow-wrap-normal-10px"
            }
            (WordBreak::Normal, OverflowWrap::BreakWord, Track::MinContent) => {
                "word-break-normal-overflow-wrap-break-word-min-content"
            }
            (WordBreak::Normal, OverflowWrap::BreakWord, Track::MaxContent) => {
                "word-break-normal-overflow-wrap-break-word-max-content"
            }
            (WordBreak::Normal, OverflowWrap::BreakWord, Track::Fixed10Px) => {
                "word-break-normal-overflow-wrap-break-word-10px"
            }
            (WordBreak::Normal, OverflowWrap::Anywhere, Track::MinContent) => {
                "word-break-normal-overflow-wrap-anywhere-min-content"
            }
            (WordBreak::Normal, OverflowWrap::Anywhere, Track::MaxContent) => {
                "word-break-normal-overflow-wrap-anywhere-max-content"
            }
            (WordBreak::Normal, OverflowWrap::Anywhere, Track::Fixed10Px) => {
                "word-break-normal-overflow-wrap-anywhere-10px"
            }
            (WordBreak::BreakWord, OverflowWrap::Normal, Track::MinContent) => {
                "word-break-break-word-overflow-wrap-normal-min-content"
            }
            (WordBreak::BreakWord, OverflowWrap::Normal, Track::MaxContent) => {
                "word-break-break-word-overflow-wrap-normal-max-content"
            }
            (WordBreak::BreakWord, OverflowWrap::Normal, Track::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-normal-10px"
            }
            (WordBreak::BreakWord, OverflowWrap::BreakWord, Track::MinContent) => {
                "word-break-break-word-overflow-wrap-break-word-min-content"
            }
            (WordBreak::BreakWord, OverflowWrap::BreakWord, Track::MaxContent) => {
                "word-break-break-word-overflow-wrap-break-word-max-content"
            }
            (WordBreak::BreakWord, OverflowWrap::BreakWord, Track::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-break-word-10px"
            }
            (WordBreak::BreakWord, OverflowWrap::Anywhere, Track::MinContent) => {
                "word-break-break-word-overflow-wrap-anywhere-min-content"
            }
            (WordBreak::BreakWord, OverflowWrap::Anywhere, Track::MaxContent) => {
                "word-break-break-word-overflow-wrap-anywhere-max-content"
            }
            (WordBreak::BreakWord, OverflowWrap::Anywhere, Track::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-anywhere-10px"
            }
        },
        word_break,
        overflow_wrap,
        track,
    }
}

impl WordBreak {
    const fn css(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::BreakWord => "break-word",
        }
    }
}

impl OverflowWrap {
    const fn css(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::BreakWord => "break-word",
            Self::Anywhere => "anywhere",
        }
    }
}

impl Track {
    const fn css(self) -> &'static str {
        match self {
            Self::MinContent => "min-content",
            Self::MaxContent => "max-content",
            Self::Fixed10Px => "10px",
        }
    }
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let style = format!(
        "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:{};direction:ltr;line-break:auto;text-align:start;overflow-wrap:{};text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820",
        scene.track.css(),
        scene.word_break.css(),
        scene.overflow_wrap.css(),
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
    fn combined_word_break_scenes_generate_exact_allowlisted_inputs() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 18);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted combined text scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("root.setAttribute('lang', 'en')"));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.track.css())));
            assert!(source.contains(&format!("word-break:{};", scene.word_break.css())));
            assert!(source.contains(&format!("overflow-wrap:{};", scene.overflow_wrap.css())));
            assert!(source.contains("grid-template-rows:auto;"));
            assert!(source.contains("createTextNode('XXXX')"));
            assert_eq!(source.matches("createTextNode(").count(), 1);
            assert_eq!(source.matches("createElement('div')").count(), 1);
        }
        assert!(runtime_source("word-break-unlisted-overflow-wrap-normal-10px").is_none());
    }
}
