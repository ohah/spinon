use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
    TextOrientation, WritingMode,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::super::super::RuntimeLayoutFailure;
use super::super::super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};
use super::super::super::{C11_4MixedInput, c11_4_image};
use super::super::{AHEM, IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG};
use super::helpers::{fail, unsupported};
use super::scene::{ItemKind, scene_for_root_id};
use super::style::{validate_grid_style, validate_item_style};

pub(in crate::session::ua_cascade::runtime_layout::fixtures::c11_4_mixed) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4MixedInput>, RuntimeLayoutFailure> {
    let (tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = scene_for_root_id(&root_id) else {
        if root_id.starts_with("c114-orthogonal-") && root_id.ends_with("-grid") {
            return Err(unsupported(root.id(), "unknown orthogonal row scene"));
        }
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
        || !has_empty_attribute(snapshot, root, "data-node")
    {
        return Err(unsupported(root.id(), "root identity, profile, or owner"));
    }

    validate_grid_style(style_for(styles, root.id())?, styles, root, scene)?;
    let grid_children = children(snapshot, root)?;
    let expected_items = scene.items();
    if grid_children.len() != expected_items.len() {
        return Err(unsupported(
            root.id(),
            "exactly two direct Grid items required",
        ));
    }

    let mut paragraph_styles = BTreeMap::new();
    let mut image_completions = Vec::with_capacity(1);
    let mut image_node = None;
    for (handle, expected) in grid_children.iter().zip(expected_items) {
        let (tag, id) = element_identity(snapshot, *handle)?;
        if id != scene.item_id(expected.kind)
            || snapshot.parent(*handle) != Some(HostParent::Node(root))
            || !has_empty_attribute(snapshot, *handle, "data-node")
        {
            return Err(unsupported(
                handle.id(),
                "direct child identity, source order, or owner differs",
            ));
        }
        let style = style_for(styles, handle.id())?;
        validate_item_style(style, styles, scene, expected)?;

        match (expected.kind, tag.as_str()) {
            (ItemKind::Text, "div") => {
                let text_children = children(snapshot, *handle)?;
                if text_children.len() != 1 {
                    return Err(unsupported(handle.id(), "one direct text leaf required"));
                }
                let leaf = text_children[0];
                let actual = snapshot.node(leaf).and_then(|node| match node.kind() {
                    HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
                if actual.as_deref() != Some(scene.content())
                    || snapshot.parent(leaf) != Some(HostParent::Node(*handle))
                {
                    return Err(unsupported(leaf.id(), "fixed Ahem text differs"));
                }
                paragraph_styles.insert(handle.id(), orthogonal_paragraph_style());
            }
            (ItemKind::Image, "img") => {
                if !children(snapshot, *handle)?.is_empty()
                    || c11_4_image::image_src(snapshot, *handle)?.as_deref() != Some(IMAGE_URL)
                {
                    return Err(unsupported(
                        handle.id(),
                        "fixed PNG image leaf/source required",
                    ));
                }
                let completion = ImageRequest::from_document(snapshot, *handle)
                    .map_err(|error| fail("c114_orthogonal_image_request", handle.id(), error))?
                    .decode(PNG)
                    .map_err(|error| fail("c114_orthogonal_image_decode", handle.id(), error))?;
                image_completions.push(completion);
                image_node = Some(handle.id());
            }
            _ => {
                return Err(unsupported(
                    handle.id(),
                    "item tag differs from fixed scene",
                ));
            }
        }
    }
    if paragraph_styles.len() != 1 || image_completions.len() != 1 {
        return Err(unsupported(
            root.id(),
            "one text item and one decoded image required",
        ));
    }

    let image_node =
        image_node.ok_or_else(|| fail("c114_orthogonal_image_missing", root.id(), "image item"))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| fail("c114_orthogonal_revision_exhausted", root.id(), "revision"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, image_completions)
        .map_err(|error| fail("c114_orthogonal_image_snapshot", root.id(), error))?;
    let image = resources.images().get(&image_node).ok_or_else(|| {
        fail(
            "c114_orthogonal_image_metadata_missing",
            image_node,
            "decoded image metadata",
        )
    })?;
    let dimensions = image.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(fail(
            "c114_orthogonal_image_metadata_mismatch",
            image_node,
            format!("{}x{}", dimensions.width(), dimensions.height()),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| fail("c114_orthogonal_intrinsic_snapshot", root.id(), error))?;
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| fail("c114_orthogonal_font_registration", root.id(), error))?;

    Ok(Some(C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

fn orthogonal_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode: WritingMode::VerticalRl,
        text_orientation: TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        locale: None,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Normal,
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
        text_transform: Default::default(),
    }
}
