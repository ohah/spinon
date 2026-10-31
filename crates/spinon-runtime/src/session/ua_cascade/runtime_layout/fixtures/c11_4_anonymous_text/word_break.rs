use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::WordBreakMode;

use super::super::super::{RuntimeLayoutFailure, layout_failure};

pub(super) const NOTO_SANS_KR: &[u8] =
    include_bytes!("../../../../../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

#[derive(Clone, Copy, Debug, PartialEq)]
struct IntrinsicSizes {
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WordBreakSpec {
    computed: &'static str,
    mode: WordBreakMode,
}

const NORMAL: WordBreakSpec = WordBreakSpec {
    computed: "normal",
    mode: WordBreakMode::Normal,
};
const BREAK_ALL: WordBreakSpec = WordBreakSpec {
    computed: "break-all",
    mode: WordBreakMode::BreakAll,
};
const KEEP_ALL: WordBreakSpec = WordBreakSpec {
    computed: "keep-all",
    mode: WordBreakMode::KeepAll,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) font_family: &'static str,
    pub(super) language: &'static str,
    pub(super) computed_word_break: &'static str,
    pub(super) word_break: WordBreakMode,
    pub(super) is_min_content: bool,
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
}

pub(super) const SCENES: [Scene; 12] = [
    scene(
        "word-break-normal-unbroken-min-content",
        "XXXX",
        "Ahem",
        "en",
        NORMAL,
        true,
        intrinsic_sizes(40.0, 40.0, 10.0, 10.0),
    ),
    scene(
        "word-break-normal-unbroken-max-content",
        "XXXX",
        "Ahem",
        "en",
        NORMAL,
        false,
        intrinsic_sizes(40.0, 40.0, 10.0, 10.0),
    ),
    scene(
        "word-break-break-all-unbroken-min-content",
        "XXXX",
        "Ahem",
        "en",
        BREAK_ALL,
        true,
        intrinsic_sizes(10.0, 40.0, 40.0, 10.0),
    ),
    scene(
        "word-break-break-all-unbroken-max-content",
        "XXXX",
        "Ahem",
        "en",
        BREAK_ALL,
        false,
        intrinsic_sizes(10.0, 40.0, 40.0, 10.0),
    ),
    scene(
        "word-break-normal-two-words-min-content",
        "XX XX",
        "Ahem",
        "en",
        NORMAL,
        true,
        intrinsic_sizes(20.0, 50.0, 20.0, 10.0),
    ),
    scene(
        "word-break-normal-two-words-max-content",
        "XX XX",
        "Ahem",
        "en",
        NORMAL,
        false,
        intrinsic_sizes(20.0, 50.0, 20.0, 10.0),
    ),
    scene(
        "word-break-break-all-two-words-min-content",
        "XX XX",
        "Ahem",
        "en",
        BREAK_ALL,
        true,
        intrinsic_sizes(10.0, 50.0, 40.0, 10.0),
    ),
    scene(
        "word-break-break-all-two-words-max-content",
        "XX XX",
        "Ahem",
        "en",
        BREAK_ALL,
        false,
        intrinsic_sizes(10.0, 50.0, 40.0, 10.0),
    ),
    scene(
        "word-break-normal-hangul-min-content",
        "가나다라",
        "Noto Sans KR",
        "ko",
        NORMAL,
        true,
        intrinsic_sizes(9.2, 36.8, 40.0, 10.0),
    ),
    scene(
        "word-break-normal-hangul-max-content",
        "가나다라",
        "Noto Sans KR",
        "ko",
        NORMAL,
        false,
        intrinsic_sizes(9.2, 36.8, 40.0, 10.0),
    ),
    scene(
        "word-break-keep-all-hangul-min-content",
        "가나다라",
        "Noto Sans KR",
        "ko",
        KEEP_ALL,
        true,
        intrinsic_sizes(36.8, 36.8, 10.0, 10.0),
    ),
    scene(
        "word-break-keep-all-hangul-max-content",
        "가나다라",
        "Noto Sans KR",
        "ko",
        KEEP_ALL,
        false,
        intrinsic_sizes(36.8, 36.8, 10.0, 10.0),
    ),
];

const fn scene(
    id: &'static str,
    text: &'static str,
    font_family: &'static str,
    language: &'static str,
    word_break: WordBreakSpec,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Scene {
    Scene {
        id,
        text,
        font_family,
        language,
        computed_word_break: word_break.computed,
        word_break: word_break.mode,
        is_min_content,
        min_content: sizes.min_content,
        max_content: sizes.max_content,
        min_content_height: sizes.min_content_height,
        max_content_height: sizes.max_content_height,
    }
}

const fn intrinsic_sizes(
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
) -> IntrinsicSizes {
    IntrinsicSizes {
        min_content,
        max_content,
        min_content_height,
        max_content_height,
    }
}

pub(super) fn find_scene(scene_id: &str) -> Option<&'static Scene> {
    SCENES.iter().find(|scene| scene.id == scene_id)
}

pub(super) fn validate_owner_language(
    snapshot: &HostDocumentSnapshot,
    owner: HostNodeHandle,
    scene_id: &str,
) -> Result<Option<String>, RuntimeLayoutFailure> {
    let Some(scene) = find_scene(scene_id) else {
        return Ok(None);
    };
    let element = snapshot
        .node(owner)
        .and_then(|node| match node.kind() {
            HostNodeKind::Element(element) => Some(element),
            HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| unsupported(owner, "Grid style owner element expected"))?;
    let attributes = element.attributes().iter().collect::<Vec<_>>();
    if attributes.iter().any(|(name, _)| {
        name.namespace() == Some("http://www.w3.org/XML/1998/namespace")
            && name.local_name() == "lang"
    }) {
        return Err(unsupported(
            owner,
            "xml:lang is outside the fixture profile",
        ));
    }
    let languages = attributes
        .iter()
        .filter(|(name, _)| name.namespace().is_none() && name.local_name() == "lang")
        .map(|(_, value)| value.to_string_lossy())
        .collect::<Vec<_>>();
    if languages.len() != 1 || languages[0] != scene.language {
        return Err(unsupported(
            owner,
            "Grid style owner lang must exactly match the allowlisted scene",
        ));
    }
    Ok(Some(scene.language.to_owned()))
}

fn unsupported(owner: HostNodeHandle, reason: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_text_style",
        Some(owner.id().get()),
        Some(reason.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, find_scene};
    use std::collections::BTreeSet;

    #[test]
    fn computed_word_break_scenes_are_unique_and_cover_the_reference_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 12);
        assert_eq!(ids.len(), SCENES.len());
        assert!(SCENES.iter().all(|scene| {
            scene.min_content <= scene.max_content
                && scene.min_content_height > 0.0
                && scene.max_content_height > 0.0
                && matches!(scene.language, "en" | "ko")
        }));
        assert_eq!(
            find_scene("word-break-keep-all-hangul-min-content")
                .unwrap()
                .language,
            "ko"
        );
        assert!(find_scene("word-break-break-word-unbroken-min-content").is_none());
    }
}
