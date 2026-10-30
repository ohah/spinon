use std::collections::BTreeMap;

use spinon_core::{HostNodeHandle, HostNodeKind};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedAspectRatioSizingBox, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedStyleSnapshot,
};

use super::{
    AHEM, C11_4TextInput, DirectTextProfile, RuntimeLayoutFailure, computed_ratio_matches,
    direct_text_source, first_html_image, has_minmax_constraints, layout_failure,
    paragraph_style_for_aspect_ratio_minmax_span, property, source_failure, style_failure,
    style_for, subtree_has_meaningful_text, supported_minmax_constraints, validate_grid_root,
    validate_ratio_geometry, validate_snapshot_revision, zero_edges,
};

pub(super) fn prepare(
    snapshot: &spinon_core::HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    let root_style = style_for(styles, root.id())?;
    if property(root_style, "display") != Some("grid") {
        return Ok(None);
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| source_failure(root.id(), "missing span Grid root children"))?
        .collect::<Vec<_>>();
    let mut element_children = Vec::new();
    let mut meaningful_items = Vec::new();
    let mut has_meaningful_anonymous_text = false;
    let mut has_explicit_placement = false;
    for child in children {
        let node = snapshot
            .node(child)
            .ok_or_else(|| source_failure(child.id(), "missing direct span Grid child"))?;
        match node.kind() {
            HostNodeKind::Text(text) => {
                has_meaningful_anonymous_text |= !text.is_only_collapsible_html_whitespace();
            }
            HostNodeKind::Element(_) => {
                element_children.push(child);
                if subtree_has_meaningful_text(snapshot, child)? {
                    meaningful_items.push(child);
                }
                let child_style = style_for(styles, child.id())?;
                has_explicit_placement |=
                    child_style
                        .grid_item_placement
                        .as_ref()
                        .is_some_and(|placement| {
                            placement.row_start != ComputedCssGridPlacement::Auto
                                || placement.row_end != ComputedCssGridPlacement::Auto
                                || placement.column_start != ComputedCssGridPlacement::Auto
                                || placement.column_end != ComputedCssGridPlacement::Auto
                        });
            }
        }
    }

    // Automatic-placement ratio/minmax cases remain on their original adapters.
    if !has_explicit_placement || (meaningful_items.is_empty() && !has_meaningful_anonymous_text) {
        return Ok(None);
    }
    if element_children.len() != 1 || meaningful_items.len() != 1 {
        return Err(source_failure(
            root.id(),
            "ratio/minmax span profile requires exactly one direct text item",
        ));
    }
    if has_meaningful_anonymous_text {
        return Err(source_failure(
            root.id(),
            "anonymous direct Grid text is unsupported in the ratio/minmax span profile",
        ));
    }
    if let Some(image) = first_html_image(snapshot, root)? {
        return Err(source_failure(
            image.id(),
            "replaced content is unsupported in the ratio/minmax span profile",
        ));
    }

    validate_grid_root(
        snapshot,
        root,
        styles,
        root_style,
        DirectTextProfile::AspectRatioMinMax,
    )?;
    let grid = root_style
        .grid_layout
        .as_ref()
        .ok_or_else(|| style_failure(root.id().get(), "span Grid layout is missing"))?;
    if grid.rows != [ComputedCssGridTrackSize::Auto] {
        return Err(style_failure(
            root.id().get(),
            "ratio/minmax span profile requires one auto row",
        ));
    }

    let item = meaningful_items[0];
    let text_node = direct_text_source(snapshot, root, item)?;
    let text = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| source_failure(text_node.id(), "span direct Text source is missing"))?;
    if text != "AAAA" {
        return Err(source_failure(
            text_node.id(),
            "ratio/minmax span profile requires direct Ahem text AAAA",
        ));
    }

    let style = style_for(styles, item.id())?;
    let placement = style
        .grid_item_placement
        .as_ref()
        .ok_or_else(|| style_failure(style.node_id.get(), "span Grid item placement is missing"))?;
    if placement.column_start != ComputedCssGridPlacement::Line(1)
        || placement.column_end != ComputedCssGridPlacement::Span(2)
        || placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
    {
        return Err(style_failure(
            style.node_id.get(),
            "span placement must be grid-column: 1 / span 2 with automatic row placement",
        ));
    }
    if !zero_edges(style.layout_spacing.padding)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(style_failure(
            style.node_id.get(),
            "ratio/minmax span profile requires zero padding and border",
        ));
    }

    let has_minmax = has_minmax_constraints(style);
    if has_minmax && !supported_minmax_constraints(style) {
        return Err(style_failure(
            style.node_id.get(),
            "span min/max constraints must be finite fixed lengths from 0px through 60px",
        ));
    }
    let mut aspect_ratio_minmax_node = None;
    match style.layout_aspect_ratio {
        Some(ratio) => {
            if property(style, "aspect-ratio") != Some("2 / 1")
                || ratio.sizing_box != ComputedAspectRatioSizingBox::BoxSizing
                || ratio.ratio != 2.0
                || !computed_ratio_matches(style, ratio)
            {
                return Err(style_failure(
                    style.node_id.get(),
                    "ratio/minmax span profile requires numeric border-box aspect-ratio 2 / 1",
                ));
            }
            validate_ratio_geometry(root_style, ratio.ratio, style.node_id.get())?;
            if has_minmax {
                aspect_ratio_minmax_node = Some(style.node_id);
            }
        }
        None if property(style, "aspect-ratio") == Some("auto") => {}
        None => {
            return Err(style_failure(
                style.node_id.get(),
                "ratio/minmax span profile requires aspect-ratio: 2 / 1 or a min/max auto-ratio control",
            ));
        }
    }

    let paragraph_style =
        paragraph_style_for_aspect_ratio_minmax_span(styles, style, 10.0, "auto")?;
    let mut paragraph_styles = BTreeMap::new();
    paragraph_styles.insert(item.id(), paragraph_style);
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_text_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles,
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
        aspect_ratio_minmax_node,
    }))
}
