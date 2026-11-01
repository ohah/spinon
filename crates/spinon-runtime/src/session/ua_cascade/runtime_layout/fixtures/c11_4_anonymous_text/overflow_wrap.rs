use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::OverflowWrapMode;

use super::super::super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TrackKind {
    MinContent,
    MaxContent,
    Fixed10Px,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OverflowWrapSpec {
    css: &'static str,
    mode: OverflowWrapMode,
}

const NORMAL: OverflowWrapSpec = OverflowWrapSpec {
    css: "normal",
    mode: OverflowWrapMode::Normal,
};
const ANYWHERE: OverflowWrapSpec = OverflowWrapSpec {
    css: "anywhere",
    mode: OverflowWrapMode::Anywhere,
};
const BREAK_WORD: OverflowWrapSpec = OverflowWrapSpec {
    css: "break-word",
    mode: OverflowWrapMode::BreakWord,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) computed_overflow_wrap: &'static str,
    pub(super) overflow_wrap: OverflowWrapMode,
    pub(super) track: TrackKind,
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
    pub(super) fixed_line_count: usize,
}

pub(super) const SCENES: [Scene; 9] = [
    scene(NORMAL, TrackKind::MinContent),
    scene(NORMAL, TrackKind::MaxContent),
    scene(NORMAL, TrackKind::Fixed10Px),
    scene(ANYWHERE, TrackKind::MinContent),
    scene(ANYWHERE, TrackKind::MaxContent),
    scene(ANYWHERE, TrackKind::Fixed10Px),
    scene(BREAK_WORD, TrackKind::MinContent),
    scene(BREAK_WORD, TrackKind::MaxContent),
    scene(BREAK_WORD, TrackKind::Fixed10Px),
];

const fn scene(mode: OverflowWrapSpec, track: TrackKind) -> Scene {
    let id = match (mode.mode, track) {
        (OverflowWrapMode::Normal, TrackKind::MinContent) => "overflow-wrap-normal-min-content",
        (OverflowWrapMode::Normal, TrackKind::MaxContent) => "overflow-wrap-normal-max-content",
        (OverflowWrapMode::Normal, TrackKind::Fixed10Px) => "overflow-wrap-normal-fixed-10px",
        (OverflowWrapMode::Anywhere, TrackKind::MinContent) => "overflow-wrap-anywhere-min-content",
        (OverflowWrapMode::Anywhere, TrackKind::MaxContent) => "overflow-wrap-anywhere-max-content",
        (OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => "overflow-wrap-anywhere-fixed-10px",
        (OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
            "overflow-wrap-break-word-min-content"
        }
        (OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
            "overflow-wrap-break-word-max-content"
        }
        (OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
            "overflow-wrap-break-word-fixed-10px"
        }
    };
    let (min_content, min_content_height) = match mode.mode {
        OverflowWrapMode::Anywhere => (10.0, 40.0),
        OverflowWrapMode::Normal | OverflowWrapMode::BreakWord => (40.0, 10.0),
    };
    let fixed_line_count = match (mode.mode, track) {
        (OverflowWrapMode::Anywhere, TrackKind::MinContent | TrackKind::Fixed10Px)
        | (OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => 4,
        _ => 1,
    };
    Scene {
        id,
        text: "XXXX",
        computed_overflow_wrap: mode.css,
        overflow_wrap: mode.mode,
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

pub(super) fn validate_computed_overflow_wrap(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<OverflowWrapMode, RuntimeLayoutFailure> {
    let expected = super::word_break_overflow_wrap::find_scene(scene_id)
        .map(|scene| scene.computed_overflow_wrap)
        .or_else(|| find_scene(scene_id).map(|scene| scene.computed_overflow_wrap))
        .unwrap_or("normal");
    let actual = properties.get("overflow-wrap").map(String::as_str);
    let mode = match actual {
        Some("normal") => OverflowWrapMode::Normal,
        Some("anywhere") => OverflowWrapMode::Anywhere,
        Some("break-word") => OverflowWrapMode::BreakWord,
        _ => {
            return Err(unsupported(
                node_id,
                format!("overflow-wrap: expected {expected}, got {actual:?}"),
            ));
        }
    };
    if actual != Some(expected) {
        return Err(unsupported(
            node_id,
            format!("overflow-wrap: expected {expected}, got {actual:?}"),
        ));
    }
    Ok(mode)
}

fn unsupported(node_id: NodeId, reason: String) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_text_style",
        Some(node_id.get()),
        Some(reason),
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, TrackKind, find_scene, validate_computed_overflow_wrap};
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_overflow_wrap_scenes_are_unique_and_cover_the_full_track_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 9);
        assert_eq!(ids.len(), SCENES.len());
        for mode in ["normal", "anywhere", "break-word"] {
            for track in [
                TrackKind::MinContent,
                TrackKind::MaxContent,
                TrackKind::Fixed10Px,
            ] {
                assert!(
                    SCENES.iter().any(|scene| {
                        scene.computed_overflow_wrap == mode && scene.track == track
                    })
                );
            }
        }
        assert_eq!(
            find_scene("overflow-wrap-break-word-min-content")
                .unwrap()
                .min_content,
            40.0
        );
        assert_eq!(
            find_scene("overflow-wrap-break-word-fixed-10px")
                .unwrap()
                .fixed_line_count,
            4
        );
        assert!(find_scene("overflow-wrap-unlisted").is_none());
    }

    #[test]
    fn computed_overflow_wrap_requires_present_exact_supported_value() {
        let node = NodeId::new(1).unwrap();
        for scene in SCENES {
            let properties = BTreeMap::from([(
                "overflow-wrap".to_owned(),
                scene.computed_overflow_wrap.to_owned(),
            )]);
            assert_eq!(
                validate_computed_overflow_wrap(node, &properties, scene.id).unwrap(),
                scene.overflow_wrap
            );
        }
        for properties in [
            BTreeMap::new(),
            BTreeMap::from([("overflow-wrap".to_owned(), "anywhere".to_owned())]),
            BTreeMap::from([("overflow-wrap".to_owned(), "break-word".to_owned())]),
            BTreeMap::from([("overflow-wrap".to_owned(), "auto".to_owned())]),
        ] {
            assert!(
                validate_computed_overflow_wrap(
                    node,
                    &properties,
                    "overflow-wrap-normal-min-content"
                )
                .is_err()
            );
        }
    }
}
