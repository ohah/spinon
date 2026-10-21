use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_text::AHEM;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[path = "c11_4_mixed/style.rs"]
mod style;

#[derive(Clone, Copy)]
enum ItemKind {
    Text(&'static str),
    Image { height: f32 },
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    kind: ItemKind,
    column_start: i32,
    column_span: i32,
    row: i32,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
    #[cfg(test)]
    tracks: &'static [f32],
}

const A_FORWARD: Item = Item {
    id: "mixed-text-a-forward",
    kind: ItemKind::Text("XX XXXXX"),
    column_start: 1,
    column_span: 2,
    row: 1,
};
const B_FORWARD: Item = Item {
    id: "mixed-image-b-forward",
    kind: ItemKind::Image { height: 48.0 },
    column_start: 2,
    column_span: 2,
    row: 2,
};
const B_REVERSED: Item = Item {
    id: "mixed-image-b-reversed",
    ..B_FORWARD
};
const A_REVERSED: Item = Item {
    id: "mixed-text-a-reversed",
    ..A_FORWARD
};
const A_THREE: Item = Item {
    id: "mixed-text-c-a",
    ..A_FORWARD
};
const B_THREE: Item = Item {
    id: "mixed-image-c-b",
    ..B_FORWARD
};
const C_THREE: Item = Item {
    id: "mixed-text-c-c",
    kind: ItemKind::Text("XXXXXXXXXXXXXXXXXX"),
    column_start: 1,
    column_span: 3,
    row: 3,
};
const C_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-c",
    ..C_THREE
};
const B_THREE_REVERSED: Item = Item {
    id: "mixed-image-r-b",
    ..B_THREE
};
const A_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-a",
    ..A_THREE
};

const SPAN2_FORWARD: [Item; 2] = [A_FORWARD, B_FORWARD];
const SPAN2_REVERSED: [Item; 2] = [B_REVERSED, A_REVERSED];
const SPAN2_THEN_SPAN3: [Item; 3] = [A_THREE, B_THREE, C_THREE];
const SPAN3_THEN_SPAN2: [Item; 3] = [C_THREE_REVERSED, B_THREE_REVERSED, A_THREE_REVERSED];

const SCENES: [Scene; 4] = [
    Scene {
        id: "mixed-text-image-span2-forward",
        items: &SPAN2_FORWARD,
        #[cfg(test)]
        tracks: &[25.0, 36.0, 36.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span2-reversed",
        items: &SPAN2_REVERSED,
        #[cfg(test)]
        tracks: &[25.0, 36.0, 36.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span2-before-span3",
        items: &SPAN2_THEN_SPAN3,
        #[cfg(test)]
        tracks: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span3-before-span2",
        items: &SPAN3_THEN_SPAN2,
        #[cfg(test)]
        tracks: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
    },
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4MixedInput>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|id| id.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).copied() else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(failure("c114_mixed_profile_mismatch", root.id(), None));
    }
    if namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene, "root identity or parent"));
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| failure("missing_node", root.id(), None))?
        .collect::<Vec<_>>();
    if children.len() != scene.items.len() {
        return Err(unsupported(root.id(), scene, "direct child count"));
    }
    let root_style = style_for(styles, root)?;
    style::validate_grid_style(root_style, scene)?;

    let mut paragraph_styles = BTreeMap::new();
    let mut image_completions = Vec::new();
    for (handle, expected) in children.iter().zip(scene.items) {
        let (namespace, tag, id) = element_identity(snapshot, *handle)?;
        if namespace != HTML_NAMESPACE
            || id != expected.id
            || snapshot.parent(*handle) != Some(HostParent::Node(root))
        {
            return Err(unsupported(
                handle.id(),
                scene,
                "child identity, namespace, or parent",
            ));
        }
        let style = style_for(styles, *handle)?;
        style::validate_item_style(style, scene, *expected)?;

        match (expected.kind, tag.as_str()) {
            (ItemKind::Text(text), "div") => {
                let text_children = snapshot
                    .children(*handle)
                    .ok_or_else(|| failure("missing_node", handle.id(), None))?
                    .collect::<Vec<_>>();
                if text_children.len() != 1
                    || snapshot.parent(text_children[0]) != Some(HostParent::Node(*handle))
                {
                    return Err(unsupported(handle.id(), scene, "direct text-leaf topology"));
                }
                let text_node = text_children[0];
                let actual = snapshot.node(text_node).and_then(|node| match node.kind() {
                    HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
                if actual.as_deref() != Some(text) {
                    return Err(unsupported(
                        text_node.id(),
                        scene,
                        "fixed Ahem text differs",
                    ));
                }
                paragraph_styles.insert(handle.id(), ahem_paragraph_style());
            }
            (ItemKind::Image { .. }, "img") => {
                if snapshot
                    .children(*handle)
                    .is_none_or(|mut descendants| descendants.next().is_some())
                    || image_src(snapshot, *handle)?.as_deref() != Some(IMAGE_URL)
                {
                    return Err(unsupported(
                        handle.id(),
                        scene,
                        "fixed image leaf/source required",
                    ));
                }
                let completion = ImageRequest::from_document(snapshot, *handle)
                    .map_err(|error| {
                        failure(
                            "c114_mixed_image_request",
                            handle.id(),
                            Some(error.to_string()),
                        )
                    })?
                    .decode(PNG)
                    .map_err(|error| {
                        failure(
                            "c114_mixed_image_decode",
                            handle.id(),
                            Some(error.to_string()),
                        )
                    })?;
                image_completions.push(completion);
            }
            _ => {
                return Err(unsupported(
                    handle.id(),
                    scene,
                    "tag does not match intrinsic source",
                ));
            }
        }
    }

    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_mixed_revision_exhausted", root.id(), None))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, image_completions).map_err(|error| {
            failure(
                "c114_mixed_image_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    for (handle, expected) in children.iter().zip(scene.items) {
        if matches!(expected.kind, ItemKind::Image { .. }) {
            let image = resources
                .images()
                .get(&handle.id())
                .ok_or_else(|| failure("c114_mixed_image_metadata_missing", handle.id(), None))?;
            let dimensions = image.dimensions();
            if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
                return Err(failure(
                    "c114_mixed_image_metadata_mismatch",
                    handle.id(),
                    Some(format!("{}x{}", dimensions.width(), dimensions.height())),
                ));
            }
        }
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_mixed_intrinsic_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        failure(
            "c114_mixed_font_registration",
            root.id(),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(super::C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
    }))
}

fn style_for<'a>(
    styles: &'a ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> Result<&'a ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == handle.id())
        .ok_or_else(|| failure("missing_computed_style", handle.id(), None))
}

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    }
}

fn failure(
    code: &'static str,
    node: spinon_core::NodeId,
    detail: Option<String>,
) -> RuntimeLayoutFailure {
    layout_failure(code, Some(node.get()), detail)
}

fn unsupported(node: spinon_core::NodeId, scene: Scene, detail: &str) -> RuntimeLayoutFailure {
    failure(
        "unsupported_c114_mixed_case",
        node,
        Some(format!("{}: {detail}", scene.id)),
    )
}

#[cfg(test)]
#[path = "c11_4_mixed/tests.rs"]
mod tests;
