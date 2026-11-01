use spinon_layout::{OverflowWrapMode, WordBreakMode};

use super::overflow_wrap::TrackKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WordBreakSpec {
    computed: &'static str,
    mode: WordBreakMode,
}

const NORMAL: WordBreakSpec = WordBreakSpec {
    computed: "normal",
    mode: WordBreakMode::Normal,
};
const BREAK_WORD: WordBreakSpec = WordBreakSpec {
    computed: "break-word",
    mode: WordBreakMode::BreakWord,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OverflowWrapSpec {
    computed: &'static str,
    mode: OverflowWrapMode,
}

const OW_NORMAL: OverflowWrapSpec = OverflowWrapSpec {
    computed: "normal",
    mode: OverflowWrapMode::Normal,
};
const OW_BREAK_WORD: OverflowWrapSpec = OverflowWrapSpec {
    computed: "break-word",
    mode: OverflowWrapMode::BreakWord,
};
const ANYWHERE: OverflowWrapSpec = OverflowWrapSpec {
    computed: "anywhere",
    mode: OverflowWrapMode::Anywhere,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) language: &'static str,
    pub(super) computed_word_break: &'static str,
    pub(super) word_break: WordBreakMode,
    pub(super) computed_overflow_wrap: &'static str,
    pub(super) overflow_wrap: OverflowWrapMode,
    pub(super) track: TrackKind,
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
    pub(super) fixed_line_count: usize,
}

pub(super) const SCENES: [Scene; 18] = [
    scene(NORMAL, OW_NORMAL, TrackKind::MinContent),
    scene(NORMAL, OW_NORMAL, TrackKind::MaxContent),
    scene(NORMAL, OW_NORMAL, TrackKind::Fixed10Px),
    scene(NORMAL, OW_BREAK_WORD, TrackKind::MinContent),
    scene(NORMAL, OW_BREAK_WORD, TrackKind::MaxContent),
    scene(NORMAL, OW_BREAK_WORD, TrackKind::Fixed10Px),
    scene(NORMAL, ANYWHERE, TrackKind::MinContent),
    scene(NORMAL, ANYWHERE, TrackKind::MaxContent),
    scene(NORMAL, ANYWHERE, TrackKind::Fixed10Px),
    scene(BREAK_WORD, OW_NORMAL, TrackKind::MinContent),
    scene(BREAK_WORD, OW_NORMAL, TrackKind::MaxContent),
    scene(BREAK_WORD, OW_NORMAL, TrackKind::Fixed10Px),
    scene(BREAK_WORD, OW_BREAK_WORD, TrackKind::MinContent),
    scene(BREAK_WORD, OW_BREAK_WORD, TrackKind::MaxContent),
    scene(BREAK_WORD, OW_BREAK_WORD, TrackKind::Fixed10Px),
    scene(BREAK_WORD, ANYWHERE, TrackKind::MinContent),
    scene(BREAK_WORD, ANYWHERE, TrackKind::MaxContent),
    scene(BREAK_WORD, ANYWHERE, TrackKind::Fixed10Px),
];

const fn scene(
    word_break: WordBreakSpec,
    overflow_wrap: OverflowWrapSpec,
    track: TrackKind,
) -> Scene {
    let effective_anywhere = matches!(
        (word_break.mode, overflow_wrap.mode),
        (WordBreakMode::BreakWord, _) | (_, OverflowWrapMode::Anywhere)
    );
    let min_content = if effective_anywhere { 10.0 } else { 40.0 };
    let min_content_height = if effective_anywhere { 40.0 } else { 10.0 };
    let fixed_line_count = match track {
        TrackKind::MinContent => {
            if effective_anywhere {
                4
            } else {
                1
            }
        }
        TrackKind::MaxContent => 1,
        TrackKind::Fixed10Px => match (word_break.mode, overflow_wrap.mode) {
            (WordBreakMode::Normal, OverflowWrapMode::Normal) => 1,
            _ => 4,
        },
    };
    Scene {
        id: match (word_break.mode, overflow_wrap.mode, track) {
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-normal-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-normal-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-normal-10px"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-break-word-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-break-word-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-break-word-10px"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-anywhere-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-anywhere-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-anywhere-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-normal-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-normal-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-normal-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-break-word-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-break-word-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-break-word-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-anywhere-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-anywhere-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-anywhere-10px"
            }
            _ => unreachable!(),
        },
        text: "XXXX",
        language: "en",
        computed_word_break: word_break.computed,
        word_break: word_break.mode,
        computed_overflow_wrap: overflow_wrap.computed,
        overflow_wrap: overflow_wrap.mode,
        track,
        min_content,
        max_content: 40.0,
        min_content_height,
        max_content_height: 10.0,
        fixed_line_count,
    }
}

pub(super) fn find_scene(scene_id: &str) -> Option<&'static Scene> {
    SCENES.iter().find(|scene| scene.id == scene_id)
}

#[cfg(test)]
mod tests {
    use super::{SCENES, find_scene};
    use std::collections::BTreeSet;

    #[test]
    fn combined_word_break_and_overflow_wrap_scenes_are_unique_and_match_the_normative_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 18);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            assert_eq!(scene.text, "XXXX");
            assert_eq!(scene.language, "en");
            assert_eq!(scene.max_content, 40.0);
            assert_eq!(scene.max_content_height, 10.0);
            let effective_anywhere = scene.computed_word_break == "break-word"
                || scene.computed_overflow_wrap == "anywhere";
            assert_eq!(
                scene.min_content,
                if effective_anywhere { 10.0 } else { 40.0 }
            );
            assert_eq!(
                scene.min_content_height,
                if effective_anywhere { 40.0 } else { 10.0 }
            );
            assert_eq!(
                scene.fixed_line_count,
                match scene.track {
                    super::TrackKind::MinContent => {
                        if effective_anywhere { 4 } else { 1 }
                    }
                    super::TrackKind::MaxContent => 1,
                    super::TrackKind::Fixed10Px => {
                        if scene.word_break == super::WordBreakMode::Normal
                            && scene.overflow_wrap == super::OverflowWrapMode::Normal
                        {
                            1
                        } else {
                            4
                        }
                    }
                }
            );
        }
        assert!(find_scene("word-break-break-word-overflow-wrap-normal-10px").is_some());
        assert!(find_scene("word-break-break-word-overflow-wrap-unlisted-min-content").is_none());
    }
}
