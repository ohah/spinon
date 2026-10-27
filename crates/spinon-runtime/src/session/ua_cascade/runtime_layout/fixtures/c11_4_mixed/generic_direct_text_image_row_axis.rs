use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
    TextOrientation, WritingMode,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::TextLayoutEngine;
use spinon_style::{ComputedCssGridTrackSize, ComputedElementStyle, ComputedStyleSnapshot};

use super::generic_direct_text_image::{
    has_mixed_signature, require_property, validate_snapshot_revision,
};
use super::{
    HTML_NAMESPACE, Item, ItemKind, RuntimeLayoutFailure, Scene, SceneGrid, ahem_paragraph_style,
    element_identity, failure, style, style_for, unsupported,
};

const GENERIC_TEXT_ITEM: Item = Item {
    id: "generic-mixed-row-text",
    kind: ItemKind::Text("XX XXXXX"),
    column_start: 1,
    column_span: 1,
    row: 1,
};
const GENERIC_IMAGE_ITEM: Item = Item {
    id: "generic-mixed-row-image",
    kind: ItemKind::Image { height: None },
    column_start: 2,
    column_span: 1,
    row: 1,
};
const GENERIC_ITEMS: [Item; 2] = [GENERIC_TEXT_ITEM, GENERIC_IMAGE_ITEM];

const MIN_CONTENT_ROW_SCENE: Scene = Scene {
    id: "generic-direct-mixed-min-content-row",
    items: &GENERIC_ITEMS,
    grid: SceneGrid::GenericDirectMinContentRow,
    #[cfg(test)]
    columns: &[50.0, 24.0],
    #[cfg(test)]
    rows: &[20.0],
};
const MAX_CONTENT_ROW_SCENE: Scene = Scene {
    id: "generic-direct-mixed-max-content-row",
    items: &GENERIC_ITEMS,
    grid: SceneGrid::GenericDirectMaxContentRow,
    #[cfg(test)]
    columns: &[50.0, 24.0],
    #[cfg(test)]
    rows: &[20.0],
};

/// Accepts the fixed mixed row-axis shape independently of HostNode IDs.
pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4MixedInput>, RuntimeLayoutFailure> {
    if !has_mixed_signature(snapshot, root)? {
        return Ok(None);
    }
    validate_snapshot_revision(snapshot, root.id(), styles)?;

    let (namespace, tag, _) = element_identity(snapshot, root)?;
    let root_style = style_for(styles, root)?;
    let Some(scene) = scene_for(root_style) else {
        return Ok(None);
    };
    if namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id(),
            scene,
            "generic row root must be an HTML div directly under the document root",
        ));
    }
    style::validate_grid_style(root_style, scene)?;
    require_property(root_style, root.id(), "direction", "ltr", scene)?;
    require_property(
        root_style,
        root.id(),
        "writing-mode",
        "horizontal-tb",
        scene,
    )?;
    if styles.writing_style_for(root.id())
        != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    {
        return Err(unsupported(
            root.id(),
            scene,
            "root writing style must be horizontal-tb/mixed",
        ));
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| failure("c114_generic_mixed_row_missing_children", root.id(), None))?
        .collect::<Vec<_>>();
    if children.len() != 2 {
        return Err(unsupported(
            root.id(),
            scene,
            "exactly two direct row items required",
        ));
    }

    let mut text_item = None;
    let mut image_item = None;
    for child in children {
        let (child_namespace, child_tag, _) = element_identity(snapshot, child)?;
        if child_namespace != HTML_NAMESPACE
            || snapshot.parent(child) != Some(HostParent::Node(root))
        {
            return Err(unsupported(
                child.id(),
                scene,
                "row item must be a direct HTML element child",
            ));
        }
        match child_tag.as_str() {
            "div" if text_item.replace(child).is_none() => {}
            "img" if image_item.replace(child).is_none() => {}
            _ => {
                return Err(unsupported(
                    child.id(),
                    scene,
                    "one text div and one image item required",
                ));
            }
        }
    }
    let text_item = text_item.ok_or_else(|| unsupported(root.id(), scene, "text item missing"))?;
    let image_item =
        image_item.ok_or_else(|| unsupported(root.id(), scene, "image item missing"))?;

    let text_style = style_for(styles, text_item)?;
    style::validate_item_style(text_style, scene, GENERIC_TEXT_ITEM)?;
    if styles.writing_style_for(text_item.id())
        != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    {
        return Err(unsupported(
            text_item.id(),
            scene,
            "text writing style must be horizontal-tb/mixed",
        ));
    }
    let text_children = snapshot
        .children(text_item)
        .ok_or_else(|| {
            failure(
                "c114_generic_mixed_row_missing_text_children",
                text_item.id(),
                None,
            )
        })?
        .collect::<Vec<_>>();
    if text_children.len() != 1
        || snapshot.parent(text_children[0]) != Some(HostParent::Node(text_item))
    {
        return Err(unsupported(
            text_item.id(),
            scene,
            "text source must be one direct text node",
        ));
    }
    let text_node = text_children[0];
    let text = snapshot.node(text_node).and_then(|node| match node.kind() {
        HostNodeKind::Text(value) => Some(value.to_string_lossy()),
        HostNodeKind::Element(_) => None,
    });
    if text.as_deref() != Some("XX XXXXX") {
        return Err(unsupported(
            text_node.id(),
            scene,
            "fixed Ahem text differs",
        ));
    }

    let image_style = style_for(styles, image_item)?;
    style::validate_item_style(image_style, scene, GENERIC_IMAGE_ITEM)?;
    if snapshot
        .children(image_item)
        .is_none_or(|mut descendants| descendants.next().is_some())
        || super::image_src(snapshot, image_item)?.as_deref() != Some(super::IMAGE_URL)
    {
        return Err(unsupported(
            image_item.id(),
            scene,
            "fixed leaf image source required",
        ));
    }

    let completion = ImageRequest::from_document(snapshot, image_item)
        .map_err(|error| {
            failure(
                "c114_generic_mixed_row_image_request",
                image_item.id(),
                Some(error.to_string()),
            )
        })?
        .decode(super::PNG)
        .map_err(|error| {
            failure(
                "c114_generic_mixed_row_image_decode",
                image_item.id(),
                Some(error.to_string()),
            )
        })?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_generic_mixed_row_revision_exhausted", root.id(), None))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, vec![completion]).map_err(|error| {
            failure(
                "c114_generic_mixed_row_image_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    let image = resources.images().get(&image_item.id()).ok_or_else(|| {
        failure(
            "c114_generic_mixed_row_image_metadata_missing",
            image_item.id(),
            None,
        )
    })?;
    let dimensions = image.dimensions();
    if dimensions.width() != super::NATURAL_WIDTH || dimensions.height() != super::NATURAL_HEIGHT {
        return Err(failure(
            "c114_generic_mixed_row_image_metadata_mismatch",
            image_item.id(),
            Some(format!("{}x{}", dimensions.width(), dimensions.height())),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_generic_mixed_row_intrinsic_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;

    let mut paragraph_styles = BTreeMap::new();
    paragraph_styles.insert(text_item.id(), ahem_paragraph_style());
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", super::AHEM).map_err(|error| {
        failure(
            "c114_generic_mixed_row_font_registration",
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

fn scene_for(root_style: &ComputedElementStyle) -> Option<Scene> {
    let rows = &root_style.grid_layout.as_ref()?.rows;
    if rows
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::MinContent))
    {
        Some(MIN_CONTENT_ROW_SCENE)
    } else if rows
        .iter()
        .any(|track| matches!(track, ComputedCssGridTrackSize::MaxContent))
    {
        Some(MAX_CONTENT_ROW_SCENE)
    } else {
        None
    }
}

#[cfg(test)]
#[path = "generic_direct_text_image_row_axis/fixture.rs"]
mod fixture;

#[cfg(test)]
#[path = "generic_direct_text_image_row_axis/tests.rs"]
mod tests;
