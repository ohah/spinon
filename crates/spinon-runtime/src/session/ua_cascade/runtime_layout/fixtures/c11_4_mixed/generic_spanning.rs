use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridTrackSize, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::{
    HTML_NAMESPACE, Item, ItemKind, RuntimeLayoutFailure, Scene, SceneGrid, ahem_paragraph_style,
    element_identity, failure, style, style_for, unsupported,
};

const TEXT_A: Item = Item {
    id: "generic-span-text-a",
    kind: ItemKind::Text("XX XXXXX"),
    column_start: 1,
    column_span: 2,
    row: 1,
};
const IMAGE_B: Item = Item {
    id: "generic-span-image-b",
    kind: ItemKind::Image { height: Some(48.0) },
    column_start: 2,
    column_span: 2,
    row: 2,
};
const TEXT_C: Item = Item {
    id: "generic-span-text-c",
    kind: ItemKind::Text("XXXXXXXXXXXXXXXXXX"),
    column_start: 1,
    column_span: 3,
    row: 3,
};

const TWO_FORWARD: [Item; 2] = [TEXT_A, IMAGE_B];
const TWO_REVERSED: [Item; 2] = [IMAGE_B, TEXT_A];
const THREE_FORWARD: [Item; 3] = [TEXT_A, IMAGE_B, TEXT_C];
const THREE_REVERSED: [Item; 3] = [TEXT_C, IMAGE_B, TEXT_A];

const GENERIC_SCENES: [Scene; 4] = [
    Scene {
        id: "mixed-text-image-span2-forward",
        items: &TWO_FORWARD,
        grid: SceneGrid::GenericSpanningMultiTrack,
        #[cfg(test)]
        columns: &[25.0, 36.0, 36.0, 0.0],
        #[cfg(test)]
        rows: &[60.0, 60.0, 60.0],
    },
    Scene {
        id: "mixed-text-image-span2-reversed",
        items: &TWO_REVERSED,
        grid: SceneGrid::GenericSpanningMultiTrack,
        #[cfg(test)]
        columns: &[25.0, 36.0, 36.0, 0.0],
        #[cfg(test)]
        rows: &[60.0, 60.0, 60.0],
    },
    Scene {
        id: "mixed-text-image-span2-before-span3",
        items: &THREE_FORWARD,
        grid: SceneGrid::GenericSpanningMultiTrack,
        #[cfg(test)]
        columns: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
        #[cfg(test)]
        rows: &[60.0, 60.0, 60.0],
    },
    Scene {
        id: "mixed-text-image-span3-before-span2",
        items: &THREE_REVERSED,
        grid: SceneGrid::GenericSpanningMultiTrack,
        #[cfg(test)]
        columns: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
        #[cfg(test)]
        rows: &[60.0, 60.0, 60.0],
    },
];

/// Routes the fixed mixed-span topology by document/style content rather than HostNode IDs.
pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4MixedInput>, RuntimeLayoutFailure> {
    if !super::generic_direct_text_image::has_mixed_signature(snapshot, root)? {
        return Ok(None);
    }
    let (namespace, tag, root_dom_id) = element_identity(snapshot, root)?;
    if root_dom_id.starts_with("c114-") {
        return Ok(None);
    }
    let root_style = style_for(styles, root)?;
    if !has_spanning_grid_signature(root_style) {
        return Ok(None);
    }
    super::generic_direct_text_image::validate_snapshot_revision(snapshot, root.id(), styles)?;
    if namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id(),
            GENERIC_SCENES[0],
            "generic spanning root must be an HTML div directly under the document root",
        ));
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| failure("c114_generic_spanning_missing_children", root.id(), None))?
        .collect::<Vec<_>>();
    let scene = GENERIC_SCENES
        .iter()
        .copied()
        .find(|scene| scene_matches_children(snapshot, &children, *scene))
        .ok_or_else(|| {
            unsupported(
                root.id(),
                GENERIC_SCENES[0],
                "child count, order, tag, or exact text differs from fixed scenes",
            )
        })?;
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(failure(
            "c114_generic_spanning_profile_mismatch",
            root.id(),
            None,
        ));
    }
    style::validate_grid_style(root_style, scene)?;

    let mut paragraph_styles = BTreeMap::new();
    let mut image_completions = Vec::new();
    for (handle, expected) in children.iter().zip(scene.items) {
        let (child_namespace, child_tag, _) = element_identity(snapshot, *handle)?;
        if child_namespace != HTML_NAMESPACE
            || snapshot.parent(*handle) != Some(HostParent::Node(root))
        {
            return Err(unsupported(
                handle.id(),
                scene,
                "direct child must be an HTML element under the grid root",
            ));
        }
        let item_style = style_for(styles, *handle)?;
        style::validate_item_style(item_style, scene, *expected)?;
        match (expected.kind, child_tag.as_str()) {
            (ItemKind::Text(text), "div") => {
                let text_children = snapshot
                    .children(*handle)
                    .ok_or_else(|| {
                        failure(
                            "c114_generic_spanning_missing_text_children",
                            handle.id(),
                            None,
                        )
                    })?
                    .collect::<Vec<_>>();
                if text_children.len() != 1
                    || snapshot.parent(text_children[0]) != Some(HostParent::Node(*handle))
                {
                    return Err(unsupported(
                        handle.id(),
                        scene,
                        "text item must have one direct text child",
                    ));
                }
                let text_node = text_children[0];
                let actual = snapshot.node(text_node).and_then(|node| match node.kind() {
                    HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
                if actual.as_deref() != Some(text)
                    || styles.writing_style_for(handle.id())
                        != Some((
                            spinon_core::WritingMode::HorizontalTb,
                            spinon_core::TextOrientation::Mixed,
                        ))
                {
                    return Err(unsupported(
                        text_node.id(),
                        scene,
                        "fixed Ahem text or writing style differs",
                    ));
                }
                paragraph_styles.insert(handle.id(), ahem_paragraph_style());
            }
            (ItemKind::Image { .. }, "img") => {
                if snapshot
                    .children(*handle)
                    .is_none_or(|mut descendants| descendants.next().is_some())
                    || super::image_src(snapshot, *handle)?.as_deref() != Some(super::IMAGE_URL)
                {
                    return Err(unsupported(
                        handle.id(),
                        scene,
                        "fixed leaf PNG source required",
                    ));
                }
                let completion = ImageRequest::from_document(snapshot, *handle)
                    .map_err(|error| {
                        failure(
                            "c114_generic_spanning_image_request",
                            handle.id(),
                            Some(error.to_string()),
                        )
                    })?
                    .decode(super::PNG)
                    .map_err(|error| {
                        failure(
                            "c114_generic_spanning_image_decode",
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
                    "element tag does not match its intrinsic source",
                ));
            }
        }
    }

    if paragraph_styles.len() != scene.items.len() - 1 || image_completions.len() != 1 {
        return Err(unsupported(
            root.id(),
            scene,
            "all text and exactly one image contribution are required",
        ));
    }
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_generic_spanning_revision_exhausted", root.id(), None))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, image_completions).map_err(|error| {
            failure(
                "c114_generic_spanning_image_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    let image_node = children
        .iter()
        .find(|handle| {
            snapshot
                .node(**handle)
                .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(element) if element.local_name() == "img"))
        })
        .ok_or_else(|| failure("c114_generic_spanning_image_node_missing", root.id(), None))?;
    let image = resources.images().get(&image_node.id()).ok_or_else(|| {
        failure(
            "c114_generic_spanning_image_metadata_missing",
            image_node.id(),
            None,
        )
    })?;
    let dimensions = image.dimensions();
    if dimensions.width() != super::NATURAL_WIDTH || dimensions.height() != super::NATURAL_HEIGHT {
        return Err(failure(
            "c114_generic_spanning_image_metadata_mismatch",
            image_node.id(),
            Some(format!("{}x{}", dimensions.width(), dimensions.height())),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_generic_spanning_intrinsic_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", super::AHEM).map_err(|error| {
        failure(
            "c114_generic_spanning_font_registration",
            root.id(),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(super::super::C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
        allow_two_axis_root_intrinsic: false,
    }))
}

fn has_spanning_grid_signature(style: &ComputedElementStyle) -> bool {
    let Some(grid) = style.grid_layout.as_ref() else {
        return false;
    };
    grid.columns.len() == 4
        && grid
            .columns
            .iter()
            .all(|track| *track == ComputedCssGridTrackSize::MinContent)
        && grid.rows
            == [
                ComputedCssGridTrackSize::LengthPx(60.0),
                ComputedCssGridTrackSize::LengthPx(60.0),
                ComputedCssGridTrackSize::LengthPx(60.0),
            ]
        && style.layout_dimensions.width == ComputedCssDimension::LengthPx(220.0)
        && style.layout_dimensions.height == ComputedCssDimension::LengthPx(180.0)
}

fn scene_matches_children(
    snapshot: &HostDocumentSnapshot,
    children: &[HostNodeHandle],
    scene: Scene,
) -> bool {
    if children.len() != scene.items.len() {
        return false;
    }
    children.iter().zip(scene.items).all(|(handle, expected)| {
        let Some(node) = snapshot.node(*handle) else {
            return false;
        };
        let HostNodeKind::Element(element) = node.kind() else {
            return false;
        };
        if element.namespace() != HTML_NAMESPACE {
            return false;
        }
        match (expected.kind, element.local_name()) {
            (ItemKind::Text(expected_text), "div") => {
                snapshot.children(*handle).is_some_and(|mut children| {
                    let Some(text_handle) = children.next() else {
                        return false;
                    };
                    if children.next().is_some()
                        || snapshot.parent(text_handle) != Some(HostParent::Node(*handle))
                    {
                        return false;
                    }
                    snapshot
                        .node(text_handle)
                        .is_some_and(|node| match node.kind() {
                            HostNodeKind::Text(value) => value.to_string_lossy() == expected_text,
                            HostNodeKind::Element(_) => false,
                        })
                })
            }
            (ItemKind::Image { height: Some(48.0) }, "img") => true,
            _ => false,
        }
    })
}

#[cfg(test)]
#[path = "generic_spanning/fixture.rs"]
mod fixture;

#[cfg(test)]
#[path = "generic_spanning/tests.rs"]
mod tests;
