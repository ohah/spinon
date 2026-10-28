use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostParent, IntrinsicSizeRevision, TextOrientation,
    WritingMode,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{LineBreakMode, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssMaxSize, ComputedCssPosition, ComputedElementStyle,
    ComputedStyleSnapshot,
};

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
    let children = snapshot
        .children(root)
        .ok_or_else(|| super::fail(root.id(), "root children unavailable"))?
        .collect::<Vec<_>>();
    if children.len() != 4 {
        return Err(super::reject(
            root.id(),
            "exactly hidden span, text, image, text direct children required",
        ));
    }

    let hidden = children[0];
    let a = children[1];
    let image = children[2];
    let c = children[3];
    validate_element(
        snapshot,
        hidden,
        "span",
        &item_id(scene_id, "hidden"),
        HostParent::Node(root),
    )?;
    validate_element(
        snapshot,
        a,
        "div",
        &item_id(scene_id, "a"),
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

    let hidden_style = super::super::style_for(styles, hidden)?;
    validate_hidden_style(hidden_style, styles, axis)?;
    if super::direct_text(snapshot, hidden)? != "XXXXXXXXXXXXXXX" {
        return Err(super::reject(hidden.id(), "fixed hidden Ahem text differs"));
    }

    let a_track = super::item_track(super::super::style_for(styles, a)?, axis);
    let b_track = super::item_track(super::super::style_for(styles, image)?, axis);
    let c_track = super::item_track(super::super::style_for(styles, c)?, axis);
    if a_track != Some(1) || b_track != Some(2) || c_track != Some(3) {
        return Err(super::reject(
            root.id(),
            "visible text and image must occupy the three explicit tracks",
        ));
    }
    for (handle, track, is_image) in [(a, 1, false), (image, 2, true), (c, 3, false)] {
        super::validate_item_style(
            super::super::style_for(styles, handle)?,
            styles,
            axis,
            track,
            is_image,
        )?;
    }
    let a_text = super::direct_text(snapshot, a)?;
    let c_text = super::direct_text(snapshot, c)?;
    if a_text != "XXXXXXXXXX" || c_text != "XXXX" {
        return Err(super::reject(root.id(), "fixed visible Ahem text differs"));
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
    for handle in [a, c] {
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

fn validate_hidden_style(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    axis: Axis,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "none"),
        ("visibility", "visible"),
        ("box-sizing", "border-box"),
        ("position", "static"),
        ("justify-self", "auto"),
        ("align-self", "auto"),
        ("grid-column-start", "1"),
        ("grid-column-end", "2"),
        ("grid-row-start", "1"),
        ("grid-row-end", "2"),
        ("font-family", "Ahem"),
        ("font-size", "10px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "nowrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("text-align", "start"),
        ("overflow-wrap", "normal"),
        ("text-transform", "none"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        super::require_property(style, name, expected)?;
    }
    let writing_mode = if axis == Axis::Rows {
        WritingMode::VerticalRl
    } else {
        WritingMode::HorizontalTb
    };
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
        || style.layout_aspect_ratio.is_some()
        || style.grid_item_placement.is_some()
        || styles.writing_style_for(style.node_id) != Some((writing_mode, TextOrientation::Mixed))
        || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
    {
        return Err(super::reject(
            style.node_id,
            "hidden item dimensions, writing mode, or placement differs",
        ));
    }
    Ok(())
}

fn scene_for_root(root_id: &str) -> Option<(&'static str, Axis)> {
    match root_id {
        "novel-mixed-intrinsic-flex-fixed-column-hidden-root" => {
            Some(("column-hidden", Axis::Columns))
        }
        "novel-mixed-intrinsic-flex-fixed-row-hidden-root" => Some(("row-hidden", Axis::Rows)),
        _ => None,
    }
}

fn item_id(scene_id: &str, suffix: &str) -> String {
    format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-{suffix}")
}
