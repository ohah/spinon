use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::TextTransformMode;

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextTransformScene {
    pub(super) id: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) mode: TextTransformMode,
    pub(super) owner_language: &'static str,
    pub(super) text: &'static str,
    pub(super) source_text_nodes: Option<&'static [&'static str]>,
    pub(super) expected_layout_text: &'static str,
    pub(super) is_min_content: bool,
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
}

pub(super) const SCENES: [TextTransformScene; 27] = [
    scene(TextTransformMode::None, true),
    scene(TextTransformMode::None, false),
    scene(TextTransformMode::Uppercase, true),
    scene(TextTransformMode::Uppercase, false),
    scene(TextTransformMode::Lowercase, true),
    scene(TextTransformMode::Lowercase, false),
    scene_with_text(
        "text-transform-capitalize-none-min-content",
        TextTransformMode::None,
        "iiii ii",
        "iiii ii",
        true,
        (11.0, 18.71875),
    ),
    scene_with_text(
        "text-transform-capitalize-none-max-content",
        TextTransformMode::None,
        "iiii ii",
        "iiii ii",
        false,
        (11.0, 18.71875),
    ),
    scene_with_text(
        "text-transform-capitalize-min-content",
        TextTransformMode::Capitalize,
        "iiii ii",
        "Iiii Ii",
        true,
        (11.1875, 19.09375),
    ),
    scene_with_text(
        "text-transform-capitalize-max-content",
        TextTransformMode::Capitalize,
        "iiii ii",
        "Iiii Ii",
        false,
        (11.1875, 19.09375),
    ),
    scene_with_text(
        "text-transform-capitalize-mixed-case",
        TextTransformMode::Capitalize,
        "iIII ii",
        "IIII Ii",
        false,
        (11.734375, 19.65625),
    ),
    scene_with_text(
        "text-transform-capitalize-collapsed-spaces",
        TextTransformMode::Capitalize,
        "ii   ii",
        "Ii Ii",
        false,
        (5.6875, 13.59375),
    ),
    unicode_scene(
        "text-transform-unicode-sharp-s-min-content",
        TextTransformMode::Capitalize,
        "ßeta",
        "Sseta",
        true,
        (25.1875, 25.1875),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-sharp-s-max-content",
        TextTransformMode::Capitalize,
        "ßeta",
        "Sseta",
        false,
        (25.1875, 25.1875),
        (10.0, 10.0),
    ),
    TextTransformScene {
        source_text_nodes: Some(&["ß", "eta"]),
        ..unicode_scene(
            "text-transform-unicode-source-split-max-content",
            TextTransformMode::Capitalize,
            "ßeta",
            "Sseta",
            false,
            (25.1875, 25.1875),
            (10.0, 10.0),
        )
    },
    unicode_scene(
        "text-transform-unicode-accented-max-content",
        TextTransformMode::Capitalize,
        "éclair",
        "Éclair",
        false,
        (25.953125, 25.953125),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-combining-max-content",
        TextTransformMode::Capitalize,
        "e\u{0301}lan",
        "E\u{0301}lan",
        false,
        (20.46875, 20.46875),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-hyphen-boundary-max-content",
        TextTransformMode::Capitalize,
        "well-known",
        "Well-Known",
        false,
        (32.671875, 56.165863),
        (20.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-apostrophe-word-max-content",
        TextTransformMode::Capitalize,
        "o'neill",
        "O'neill",
        false,
        (30.28125, 30.28125),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-opening-punctuation-max-content",
        TextTransformMode::Capitalize,
        "«élan»",
        "«Élan»",
        false,
        (30.046875, 30.046875),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-number-first-none",
        TextTransformMode::Capitalize,
        "49ers",
        "49ers",
        false,
        (24.984375, 24.984375),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-number-first-control",
        TextTransformMode::None,
        "49ers",
        "49ers",
        false,
        (24.984375, 24.984375),
        (10.0, 10.0),
    ),
    unicode_scene(
        "text-transform-unicode-collapsed-space-max-content",
        TextTransformMode::Capitalize,
        "ßeta   éclair",
        "Sseta Éclair",
        false,
        (25.953125, 53.359375),
        (20.0, 10.0),
    ),
    full_width_scene(
        "text-transform-full-width-none-max-content",
        TextTransformMode::None,
        "AB 12",
        "AB 12",
        false,
        (12.65625, 25.984375),
    ),
    full_width_scene(
        "text-transform-full-width-max-content",
        TextTransformMode::FullWidth,
        "AB 12",
        "ＡＢ　１２",
        false,
        (20.0, 50.0),
    ),
    full_width_scene(
        "text-transform-full-width-min-content",
        TextTransformMode::FullWidth,
        "AB 12",
        "ＡＢ　１２",
        true,
        (20.0, 50.0),
    ),
    full_width_scene(
        "text-transform-full-width-collapsed-spaces-max-content",
        TextTransformMode::FullWidth,
        "AB   12",
        "ＡＢ　１２",
        false,
        (20.0, 50.0),
    ),
];

pub(super) const SCENE_IDS: [&str; SCENES.len()] = [
    "text-transform-none-min-content",
    "text-transform-none-max-content",
    "text-transform-uppercase-min-content",
    "text-transform-uppercase-max-content",
    "text-transform-lowercase-min-content",
    "text-transform-lowercase-max-content",
    "text-transform-capitalize-none-min-content",
    "text-transform-capitalize-none-max-content",
    "text-transform-capitalize-min-content",
    "text-transform-capitalize-max-content",
    "text-transform-capitalize-mixed-case",
    "text-transform-capitalize-collapsed-spaces",
    "text-transform-unicode-sharp-s-min-content",
    "text-transform-unicode-sharp-s-max-content",
    "text-transform-unicode-source-split-max-content",
    "text-transform-unicode-accented-max-content",
    "text-transform-unicode-combining-max-content",
    "text-transform-unicode-hyphen-boundary-max-content",
    "text-transform-unicode-apostrophe-word-max-content",
    "text-transform-unicode-opening-punctuation-max-content",
    "text-transform-unicode-number-first-none",
    "text-transform-unicode-number-first-control",
    "text-transform-unicode-collapsed-space-max-content",
    "text-transform-full-width-none-max-content",
    "text-transform-full-width-max-content",
    "text-transform-full-width-min-content",
    "text-transform-full-width-collapsed-spaces-max-content",
];

const fn scene(mode: TextTransformMode, is_min_content: bool) -> TextTransformScene {
    let (computed_value, id, min_content, max_content, expected_layout_text) = match mode {
        TextTransformMode::None => (
            "none",
            if is_min_content {
                "text-transform-none-min-content"
            } else {
                "text-transform-none-max-content"
            },
            11.734375,
            19.46875,
            "IIII ii",
        ),
        TextTransformMode::Uppercase => (
            "uppercase",
            if is_min_content {
                "text-transform-uppercase-min-content"
            } else {
                "text-transform-uppercase-max-content"
            },
            11.734375,
            19.84375,
            "IIII II",
        ),
        TextTransformMode::Lowercase => (
            "lowercase",
            if is_min_content {
                "text-transform-lowercase-min-content"
            } else {
                "text-transform-lowercase-max-content"
            },
            11.0,
            18.71875,
            "iiii ii",
        ),
        TextTransformMode::Capitalize => (
            "capitalize",
            if is_min_content {
                "text-transform-capitalize-min-content"
            } else {
                "text-transform-capitalize-max-content"
            },
            11.1875,
            19.09375,
            "Iiii Ii",
        ),
        TextTransformMode::FullWidth => (
            "full-width",
            if is_min_content {
                "text-transform-full-width-min-content"
            } else {
                "text-transform-full-width-max-content"
            },
            20.0,
            50.0,
            "ＡＢ　１２",
        ),
    };
    TextTransformScene {
        id,
        computed_value,
        mode,
        owner_language: match mode {
            TextTransformMode::FullWidth => "ja",
            TextTransformMode::None
            | TextTransformMode::Uppercase
            | TextTransformMode::Lowercase
            | TextTransformMode::Capitalize => "en",
        },
        text: match mode {
            TextTransformMode::FullWidth => "AB 12",
            TextTransformMode::None
            | TextTransformMode::Uppercase
            | TextTransformMode::Lowercase
            | TextTransformMode::Capitalize => "IIII ii",
        },
        source_text_nodes: None,
        expected_layout_text,
        is_min_content,
        min_content,
        max_content,
        min_content_height: 20.0,
        max_content_height: 10.0,
    }
}

const fn scene_with_text(
    id: &'static str,
    mode: TextTransformMode,
    text: &'static str,
    expected_layout_text: &'static str,
    is_min_content: bool,
    intrinsic_sizes: (f32, f32),
) -> TextTransformScene {
    TextTransformScene {
        id,
        computed_value: match mode {
            TextTransformMode::None => "none",
            TextTransformMode::Uppercase => "uppercase",
            TextTransformMode::Lowercase => "lowercase",
            TextTransformMode::Capitalize => "capitalize",
            TextTransformMode::FullWidth => "full-width",
        },
        mode,
        owner_language: "en",
        text,
        source_text_nodes: None,
        expected_layout_text,
        is_min_content,
        min_content: intrinsic_sizes.0,
        max_content: intrinsic_sizes.1,
        min_content_height: 20.0,
        max_content_height: 10.0,
    }
}

const fn unicode_scene(
    id: &'static str,
    mode: TextTransformMode,
    text: &'static str,
    expected_layout_text: &'static str,
    is_min_content: bool,
    intrinsic_sizes: (f32, f32),
    intrinsic_heights: (f32, f32),
) -> TextTransformScene {
    TextTransformScene {
        id,
        computed_value: match mode {
            TextTransformMode::None => "none",
            TextTransformMode::Uppercase => "uppercase",
            TextTransformMode::Lowercase => "lowercase",
            TextTransformMode::Capitalize => "capitalize",
            TextTransformMode::FullWidth => "full-width",
        },
        mode,
        owner_language: "en",
        text,
        source_text_nodes: None,
        expected_layout_text,
        is_min_content,
        min_content: intrinsic_sizes.0,
        max_content: intrinsic_sizes.1,
        min_content_height: intrinsic_heights.0,
        max_content_height: intrinsic_heights.1,
    }
}

const fn full_width_scene(
    id: &'static str,
    mode: TextTransformMode,
    text: &'static str,
    expected_layout_text: &'static str,
    is_min_content: bool,
    intrinsic_sizes: (f32, f32),
) -> TextTransformScene {
    TextTransformScene {
        id,
        computed_value: match mode {
            TextTransformMode::None => "none",
            TextTransformMode::FullWidth => "full-width",
            TextTransformMode::Uppercase
            | TextTransformMode::Lowercase
            | TextTransformMode::Capitalize => "unsupported",
        },
        mode,
        owner_language: "ja",
        text,
        source_text_nodes: None,
        expected_layout_text,
        is_min_content,
        min_content: intrinsic_sizes.0,
        max_content: intrinsic_sizes.1,
        min_content_height: 20.0,
        max_content_height: 10.0,
    }
}

pub(super) fn find_scene(scene_id: &str) -> Option<&'static TextTransformScene> {
    SCENES.iter().find(|scene| scene.id == scene_id)
}

pub(super) fn validate_computed_text_transform(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<TextTransformMode, RuntimeLayoutFailure> {
    let scene = find_scene(scene_id);
    let expected = scene.map_or("none", |scene| scene.computed_value);
    let actual = properties.get("text-transform").map(String::as_str);
    if actual != Some(expected) {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "text-transform: expected {expected}, got {actual:?}"
            )),
        ));
    }
    Ok(scene.map_or(TextTransformMode::None, |scene| scene.mode))
}

pub(super) fn validate_owner_language(
    snapshot: &HostDocumentSnapshot,
    owner: HostNodeHandle,
    scene_id: &str,
) -> Result<Option<String>, RuntimeLayoutFailure> {
    if find_scene(scene_id).is_none() {
        return Ok(None);
    }
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
    let expected_language = find_scene(scene_id)
        .map(|scene| scene.owner_language)
        .unwrap_or("en");
    if languages.len() != 1 || languages[0] != expected_language {
        return Err(unsupported(
            owner,
            "Grid style owner lang must match the exact fixture scene language",
        ));
    }
    Ok(Some(expected_language.to_owned()))
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
    use super::super::validate_snapshot_revision;
    use super::{SCENE_IDS, SCENES, validate_computed_text_transform, validate_owner_language};
    use spinon_core::{
        AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, NodeId,
        OwnerId, StyleRevision,
    };
    use spinon_layout::TextTransformMode;
    use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot, CssViewport};
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::Arc;

    #[test]
    fn computed_text_transform_matrix_has_unique_mode_track_unicode_and_full_width_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 27);
        assert_eq!(SCENE_IDS.len(), SCENES.len());
        assert_eq!(ids.len(), SCENES.len());
        assert_eq!(
            SCENES.iter().map(|scene| scene.id).collect::<Vec<_>>(),
            SCENE_IDS
        );
        assert!(SCENES.iter().any(|scene| {
            scene.id == "text-transform-capitalize-min-content"
                && scene.text == "iiii ii"
                && scene.expected_layout_text == "Iiii Ii"
        }));
        assert!(SCENES.iter().any(|scene| {
            scene.id == "text-transform-unicode-source-split-max-content"
                && scene.source_text_nodes == Some(&["ß", "eta"])
                && scene.expected_layout_text == "Sseta"
        }));
        assert!(SCENES.iter().any(|scene| {
            scene.id == "text-transform-full-width-min-content"
                && scene.owner_language == "ja"
                && scene.expected_layout_text == "ＡＢ　１２"
                && scene.min_content == 20.0
                && scene.max_content == 50.0
        }));
    }

    #[test]
    fn computed_text_transform_accepts_only_the_expected_value_for_each_scene() {
        let node = NodeId::new(1).unwrap();
        for scene in SCENES {
            let properties =
                BTreeMap::from([("text-transform".to_owned(), scene.computed_value.to_owned())]);
            assert_eq!(
                validate_computed_text_transform(node, &properties, scene.id),
                Ok(scene.mode),
                "{}",
                scene.id
            );
        }
        for value in ["full-width", "uppercase ", "Uppercase"] {
            let properties = BTreeMap::from([("text-transform".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_text_transform(
                    node,
                    &properties,
                    "text-transform-uppercase-min-content"
                )
                .is_err()
            );
        }
        assert_eq!(
            validate_computed_text_transform(
                node,
                &BTreeMap::from([("text-transform".to_owned(), "none".to_owned())]),
                "unlisted-anonymous-text-scene"
            ),
            Ok(TextTransformMode::None)
        );
        assert!(
            validate_computed_text_transform(
                node,
                &BTreeMap::new(),
                "text-transform-lowercase-max-content"
            )
            .is_err()
        );
    }

    fn owner_document(
        ancestor_language: Option<&str>,
        owner_language: Option<&str>,
    ) -> (HostDocument, spinon_core::HostNodeHandle) {
        const HTML: &str = "http://www.w3.org/1999/xhtml";
        let mut document = HostDocument::new().unwrap();
        let ancestor = document.reserve_node_handle().unwrap();
        let owner = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(12_909).unwrap(), document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: ancestor,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        if let Some(language) = ancestor_language {
            batch.push(DocumentOperation::SetAttribute {
                node: ancestor,
                name: AttributeName::new(None, "lang").unwrap(),
                value: language.into(),
            });
        }
        batch.push(DocumentOperation::CreateElement {
            node: owner,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        if let Some(language) = owner_language {
            batch.push(DocumentOperation::SetAttribute {
                node: owner,
                name: AttributeName::new(None, "lang").unwrap(),
                value: language.into(),
            });
        }
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
        (document, owner)
    }

    #[test]
    fn text_transform_requires_exact_language_on_the_grid_style_owner() {
        let (document, owner) = owner_document(None, Some("en"));
        assert_eq!(
            validate_owner_language(
                &document.snapshot(),
                owner,
                "text-transform-uppercase-min-content"
            ),
            Ok(Some("en".to_owned()))
        );

        let (document, owner) = owner_document(Some("en"), None);
        assert!(
            validate_owner_language(
                &document.snapshot(),
                owner,
                "text-transform-uppercase-min-content"
            )
            .is_err()
        );

        let (document, owner) = owner_document(None, Some("ja"));
        assert_eq!(
            validate_owner_language(
                &document.snapshot(),
                owner,
                "text-transform-full-width-max-content"
            ),
            Ok(Some("ja".to_owned()))
        );
        let (document, owner) = owner_document(Some("ja"), None);
        assert!(
            validate_owner_language(
                &document.snapshot(),
                owner,
                "text-transform-full-width-max-content"
            )
            .is_err()
        );

        let (document, owner) = owner_document(None, Some("en-US"));
        assert!(
            validate_owner_language(
                &document.snapshot(),
                owner,
                "text-transform-uppercase-min-content"
            )
            .is_err()
        );
    }

    #[test]
    fn text_transform_rejects_computed_style_from_another_document_revision() {
        let (document, owner) = owner_document(None, Some("en"));
        let snapshot = document.snapshot();
        let mut styles = ComputedStyleSnapshot {
            profile: ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            viewport: CssViewport::C04_FIXTURE,
            style_revision: StyleRevision::INITIAL,
            generation: snapshot.generation(),
            document_revision: snapshot.document_revision(),
            render_tree_revision: snapshot.render_tree_revision(),
            document_root_font_size_css_px: 16.0,
            elements: Arc::from([]),
            writing_styles: Arc::from([]),
            diagnostics: Vec::new(),
        };
        assert!(validate_snapshot_revision(&snapshot, owner, &styles).is_ok());

        styles.document_revision = Default::default();
        assert!(validate_snapshot_revision(&snapshot, owner, &styles).is_err());

        styles.document_revision = snapshot.document_revision();
        styles.render_tree_revision = Default::default();
        assert!(validate_snapshot_revision(&snapshot, owner, &styles).is_err());
    }
}
