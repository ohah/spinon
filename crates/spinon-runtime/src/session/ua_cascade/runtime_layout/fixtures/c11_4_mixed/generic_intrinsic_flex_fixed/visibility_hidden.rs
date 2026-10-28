use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostParent, IntrinsicSizeRevision, WritingMode,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{LineBreakMode, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::ComputedStyleSnapshot;

use super::super::generic_direct_text_image::validate_snapshot_revision;
use super::super::{HTML_NAMESPACE, RuntimeLayoutFailure, ahem_paragraph_style, element_identity};
use super::Axis;

const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    png: &[u8],
) -> Result<Option<super::super::super::C11_4MixedInput>, RuntimeLayoutFailure> {
    let (_, _, root_id) = element_identity(snapshot, root)?;
    let Some((scene_id, axis)) = scene_for_root(&root_id) else {
        return Ok(None);
    };
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    validate_element(snapshot, root, "div", &root_id, HostParent::Root)?;

    let root_style = super::super::style_for(styles, root)?;
    super::validate_root_style(root_style, styles, root, axis)?;
    super::require_property(root_style, "visibility", "visible")?;

    let children = snapshot
        .children(root)
        .ok_or_else(|| super::fail(root.id(), "root children unavailable"))?
        .collect::<Vec<_>>();
    if children.len() != 4 {
        return Err(super::reject(
            root.id(),
            "exactly text, visibility span, image, text direct children required",
        ));
    }
    let [a, span, image, c] = children.as_slice() else {
        unreachable!("four direct children checked above")
    };
    let a = *a;
    let span = *span;
    let image = *image;
    let c = *c;
    validate_element(
        snapshot,
        a,
        "div",
        &item_id(scene_id, "a"),
        HostParent::Node(root),
    )?;
    validate_element(
        snapshot,
        span,
        "span",
        &item_id(scene_id, "visibility"),
        HostParent::Node(root),
    )?;
    validate_element(
        snapshot,
        image,
        "img",
        &item_id(scene_id, "b"),
        HostParent::Node(root),
    )?;
    validate_element(
        snapshot,
        c,
        "div",
        &item_id(scene_id, "c"),
        HostParent::Node(root),
    )?;

    let a_style = super::super::style_for(styles, a)?;
    let span_style = super::super::style_for(styles, span)?;
    let image_style = super::super::style_for(styles, image)?;
    let c_style = super::super::style_for(styles, c)?;
    for style in [a_style, image_style, c_style] {
        super::require_property(style, "visibility", "visible")?;
    }
    super::require_property(
        span_style,
        "visibility",
        if scene_id.ends_with("-visible-control") {
            "visible"
        } else {
            "hidden"
        },
    )?;

    for (handle, style, track, is_image) in [
        (a, a_style, 1, false),
        (span, span_style, 1, false),
        (image, image_style, 2, true),
        (c, c_style, 3, false),
    ] {
        if super::item_track(style, axis) != Some(track) {
            return Err(super::reject(
                handle.id(),
                "fixed visibility scene item track differs",
            ));
        }
        super::validate_item_style(style, styles, axis, track, is_image)?;
    }
    if super::direct_text(snapshot, a)? != "XXXXXXXXXX"
        || super::direct_text(snapshot, span)? != "XXXXXXXXXXXXXXX"
        || super::direct_text(snapshot, c)? != "XXXX"
    {
        return Err(super::reject(
            root.id(),
            "fixed visibility scene Ahem text differs",
        ));
    }
    if snapshot
        .children(image)
        .is_none_or(|mut descendants| descendants.next().is_some())
        || super::super::image_src(snapshot, image)?.as_deref() != Some(IMAGE_URL)
    {
        return Err(super::reject(
            image.id(),
            "fixed leaf image source required",
        ));
    }

    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| super::fail(image.id(), format!("image request: {error}")))?
        .decode(png)
        .map_err(|error| super::fail(image.id(), format!("image decode: {error}")))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| super::fail(root.id(), "intrinsic-size revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, vec![completion])
        .map_err(|error| super::fail(root.id(), format!("image snapshot: {error}")))?;
    let dimensions = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| super::fail(image.id(), "image intrinsic metadata missing"))?
        .dimensions();
    if dimensions.width() != super::super::NATURAL_WIDTH
        || dimensions.height() != super::super::NATURAL_HEIGHT
    {
        return Err(super::fail(
            image.id(),
            format!(
                "image natural size is {}x{}",
                dimensions.width(),
                dimensions.height()
            ),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| super::fail(root.id(), format!("intrinsic-size snapshot: {error}")))?;

    let mut paragraph_styles = BTreeMap::new();
    for handle in [a, span, c] {
        let mut paragraph = ahem_paragraph_style();
        paragraph.white_space = WhiteSpaceMode::NoWrap;
        if axis == Axis::Rows {
            paragraph.writing_mode = WritingMode::VerticalRl;
            paragraph.line_break = LineBreakMode::Normal;
        }
        paragraph_styles.insert(handle.id(), paragraph);
    }
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::super::AHEM)
        .map_err(|error| super::fail(root.id(), format!("Ahem font registration: {error}")))?;

    Ok(Some(super::super::super::C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

fn validate_element(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    expected_tag: &str,
    expected_id: &str,
    expected_parent: HostParent,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, handle)?;
    if namespace != HTML_NAMESPACE || tag != expected_tag || id != expected_id {
        return Err(super::reject(
            handle.id(),
            "fixed HTML element identity or ID differs",
        ));
    }
    if snapshot.parent(handle) != Some(expected_parent) {
        return Err(super::reject(handle.id(), "fixed direct parent differs"));
    }
    Ok(())
}

fn scene_for_root(root_id: &str) -> Option<(&'static str, Axis)> {
    match root_id {
        "novel-mixed-intrinsic-flex-fixed-column-visibility-hidden-root" => {
            Some(("column-visibility-hidden", Axis::Columns))
        }
        "novel-mixed-intrinsic-flex-fixed-column-visibility-visible-control-root" => {
            Some(("column-visibility-visible-control", Axis::Columns))
        }
        "novel-mixed-intrinsic-flex-fixed-row-visibility-hidden-root" => {
            Some(("row-visibility-hidden", Axis::Rows))
        }
        "novel-mixed-intrinsic-flex-fixed-row-visibility-visible-control-root" => {
            Some(("row-visibility-visible-control", Axis::Rows))
        }
        _ => None,
    }
}

fn item_id(scene_id: &str, suffix: &str) -> String {
    format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-{suffix}")
}
