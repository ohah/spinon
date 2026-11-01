use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::LineBreakMode;

use super::super::super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) font_family: &'static str,
    pub(super) language: &'static str,
    pub(super) computed_line_break: &'static str,
    pub(super) line_break: LineBreakMode,
    pub(super) is_min_content: bool,
    pub(super) sizes: IntrinsicSizes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct IntrinsicSizes {
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
}

pub(super) const SCENES: [Scene; 9] = [
    scene(
        "line-break-ja-normal-min-content",
        "ja",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    scene(
        "line-break-ja-normal-max-content",
        "ja",
        "normal",
        LineBreakMode::Normal,
        false,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    scene(
        "line-break-ja-strict-min-content",
        "ja",
        "strict",
        LineBreakMode::Strict,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    scene(
        "line-break-ja-strict-max-content",
        "ja",
        "strict",
        LineBreakMode::Strict,
        false,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    scene(
        "line-break-en-normal-min-content",
        "en",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    scene_with_text(
        "line-break-ja-loose-min-content",
        "文＄文",
        "ja",
        "loose",
        LineBreakMode::Loose,
        true,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    scene_with_text(
        "line-break-ja-loose-max-content",
        "文＄文",
        "ja",
        "loose",
        LineBreakMode::Loose,
        false,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    scene_with_text(
        "line-break-en-loose-min-content",
        "文＄文",
        "en",
        "loose",
        LineBreakMode::Loose,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    scene_with_text(
        "line-break-ja-normal-prefix-control-min-content",
        "文＄文",
        "ja",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
];

const fn scene(
    id: &'static str,
    language: &'static str,
    computed_line_break: &'static str,
    line_break: LineBreakMode,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Scene {
    scene_with_text(
        id,
        "文〜文",
        language,
        computed_line_break,
        line_break,
        is_min_content,
        sizes,
    )
}

const fn scene_with_text(
    id: &'static str,
    text: &'static str,
    language: &'static str,
    computed_line_break: &'static str,
    line_break: LineBreakMode,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Scene {
    Scene {
        id,
        text,
        font_family: "Noto Sans KR",
        language,
        computed_line_break,
        line_break,
        is_min_content,
        sizes,
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
    use super::{SCENES, find_scene, validate_owner_language};
    use spinon_core::{
        AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    };
    use std::collections::BTreeSet;

    #[test]
    fn computed_line_break_scenes_are_unique_and_cover_the_fixed_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 9);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let loose_prefix_scene = matches!(
                scene.id,
                "line-break-ja-loose-min-content"
                    | "line-break-ja-loose-max-content"
                    | "line-break-en-loose-min-content"
                    | "line-break-ja-normal-prefix-control-min-content"
            );
            assert_eq!(
                scene.text,
                if loose_prefix_scene {
                    "文＄文"
                } else {
                    "文〜文"
                }
            );
            assert_eq!(scene.font_family, "Noto Sans KR");
            assert_eq!(find_scene(scene.id), Some(&scene));
        }
        assert!(find_scene("line-break-loose-min-content").is_none());
    }

    #[test]
    fn owner_language_is_not_inherited_from_an_ancestor() {
        const HTML: &str = "http://www.w3.org/1999/xhtml";
        let mut document = HostDocument::new().unwrap();
        let ancestor = document.reserve_node_handle().unwrap();
        let owner = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(12_897).unwrap(), document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: ancestor,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: ancestor,
            name: AttributeName::new(None, "lang").unwrap(),
            value: "ja".into(),
        });
        batch.push(DocumentOperation::CreateElement {
            node: owner,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: ancestor,
            before: None,
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(ancestor),
            node: owner,
            before: None,
        });
        document.commit(batch).unwrap();

        let error = validate_owner_language(
            &document.snapshot(),
            owner,
            "line-break-ja-normal-min-content",
        )
        .expect_err("ancestor-only language must not satisfy this fixture");
        assert!(format!("{error:?}").contains("Grid style owner lang"));
    }
}
