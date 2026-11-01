use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::TextTransformMode;

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextTransformScene {
    pub(super) id: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) mode: TextTransformMode,
    pub(super) text: &'static str,
    pub(super) expected_layout_text: &'static str,
    pub(super) is_min_content: bool,
    pub(super) min_content: f32,
    pub(super) max_content: f32,
    pub(super) min_content_height: f32,
    pub(super) max_content_height: f32,
}

pub(super) const SCENES: [TextTransformScene; 12] = [
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
    };
    TextTransformScene {
        id,
        computed_value,
        mode,
        text: "IIII ii",
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
        },
        mode,
        text,
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
    if languages.len() != 1 || languages[0] != "en" {
        return Err(unsupported(
            owner,
            "Grid style owner lang must be exactly en",
        ));
    }
    Ok(Some("en".to_owned()))
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
    fn computed_text_transform_matrix_has_twelve_unique_mode_track_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 12);
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
