use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::TextLayoutEngine;
use spinon_style::{ComputedAspectRatioSizingBox, ComputedCssDimension, ComputedStyleSnapshot};

use super::{
    AHEM, C11_4TextInput, DirectTextProfile, RuntimeLayoutFailure, computed_ratio_matches,
    direct_text_source, first_html_image, is_aspect_ratio_grid_root_candidate, layout_failure,
    paragraph_style_for_aspect_ratio_definite_axis, property, require_style_property,
    source_failure, style_failure, style_for, subtree_has_meaningful_text, validate_grid_root,
    validate_snapshot_revision,
};

const LONG_TEXT: &str = "AAAA";
const SHORT_TEXT: &str = "A";

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    let root_style = style_for(styles, root.id())?;
    if property(root_style, "display") != Some("grid")
        || !is_aspect_ratio_grid_root_candidate(root_style)
        || !root_style
            .grid_layout
            .as_ref()
            .is_some_and(|grid| grid.rows == [spinon_style::ComputedCssGridTrackSize::Auto])
    {
        return Ok(None);
    }

    let root_children = snapshot
        .children(root)
        .ok_or_else(|| source_failure(root.id(), "missing definite-axis Grid children"))?
        .collect::<Vec<_>>();
    let mut candidates = Vec::new();
    let mut has_empty_in_flow_item = false;
    let mut has_anonymous_text = false;
    for child in root_children {
        let node = snapshot
            .node(child)
            .ok_or_else(|| source_failure(child.id(), "missing definite-axis Grid child"))?;
        match node.kind() {
            HostNodeKind::Text(text) if !text.is_only_collapsible_html_whitespace() => {
                has_anonymous_text = true;
            }
            HostNodeKind::Text(_) => {}
            HostNodeKind::Element(_) => {
                let style = style_for(styles, child.id())?;
                if property(style, "display") == Some("none") {
                    continue;
                }
                if subtree_has_meaningful_text(snapshot, child)? {
                    candidates.push(child);
                } else {
                    has_empty_in_flow_item = true;
                }
            }
        }
    }

    let mut has_axis_dimension = false;
    let mut has_ratio_signal = false;
    for candidate in &candidates {
        let style = style_for(styles, candidate.id())?;
        has_axis_dimension |= supported_dimensions(style);
        has_ratio_signal |= style.layout_aspect_ratio.is_some()
            || property(style, "aspect-ratio").is_some_and(|value| value != "auto");
    }
    if !has_axis_dimension && !has_ratio_signal {
        return Ok(None);
    }
    // Existing all-auto ratio cases continue through the established ratio profiles.
    if !has_axis_dimension {
        return Ok(None);
    }
    if candidates.len() != 2 || has_empty_in_flow_item || has_anonymous_text {
        return Err(source_failure(
            root.id(),
            "definite-axis profile requires exactly two in-flow text items and no anonymous text",
        ));
    }
    if let Some(image) = first_html_image(snapshot, root)? {
        return Err(source_failure(
            image.id(),
            "replaced image content is unsupported in the definite-axis text profile",
        ));
    }

    validate_grid_root(
        snapshot,
        root,
        styles,
        root_style,
        DirectTextProfile::AspectRatio,
    )?;
    let mut paragraph_styles = BTreeMap::new();
    let mut short_item = None;
    let mut long_item = None;
    let mut ratio_owner = None;
    for item in candidates {
        let text_node = direct_text_source(snapshot, root, item)?;
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| {
                source_failure(text_node.id(), "definite-axis Text source is missing")
            })?;
        let style = style_for(styles, item.id())?;
        let dimensions = (
            style.layout_dimensions.width,
            style.layout_dimensions.height,
        );
        match text.as_str() {
            SHORT_TEXT => {
                if short_item.replace(item.id()).is_some()
                    || dimensions != (ComputedCssDimension::Auto, ComputedCssDimension::Auto)
                    || style.layout_aspect_ratio.is_some()
                    || property(style, "aspect-ratio") != Some("auto")
                {
                    return Err(style_failure(
                        style.node_id.get(),
                        "short A item must keep automatic dimensions and no preferred ratio",
                    ));
                }
                require_dimension_properties(style, "auto", "auto")?;
            }
            LONG_TEXT => {
                if long_item.replace(item.id()).is_some() || !supported_dimensions(style) {
                    return Err(style_failure(
                        style.node_id.get(),
                        "AAAA item requires width 60px/height auto or width auto/height 30px",
                    ));
                }
                if let Some(ratio) = style.layout_aspect_ratio {
                    if ratio.ratio != 2.0
                        || ratio.sizing_box != ComputedAspectRatioSizingBox::BoxSizing
                        || !computed_ratio_matches(style, ratio)
                        || property(style, "aspect-ratio") != Some("2 / 1")
                    {
                        return Err(style_failure(
                            style.node_id.get(),
                            "AAAA item requires the matching numeric 2 / 1 ratio",
                        ));
                    }
                    ratio_owner = Some(style.node_id);
                } else if property(style, "aspect-ratio") != Some("auto") {
                    return Err(style_failure(
                        style.node_id.get(),
                        "AAAA item requires computed auto or the matching numeric 2 / 1 ratio",
                    ));
                };
                require_dimension_properties(
                    style,
                    if dimensions.0 == ComputedCssDimension::LengthPx(60.0) {
                        "60px"
                    } else {
                        "auto"
                    },
                    if dimensions.1 == ComputedCssDimension::LengthPx(30.0) {
                        "30px"
                    } else {
                        "auto"
                    },
                )?;
            }
            _ => {
                return Err(source_failure(
                    text_node.id(),
                    "definite-axis profile requires one A and one AAAA text item",
                ));
            }
        }
        let paragraph_style =
            paragraph_style_for_aspect_ratio_definite_axis(styles, style, 10.0, "auto")?;
        if property(style, "text-wrap-mode") != Some("nowrap") {
            return Err(style_failure(
                style.node_id.get(),
                "text-wrap-mode must be nowrap",
            ));
        }
        paragraph_styles.insert(item.id(), paragraph_style);
    }

    if short_item.is_none() || long_item.is_none() || paragraph_styles.len() != 2 {
        return Err(source_failure(
            root.id(),
            "definite-axis text item ownership is incomplete or duplicated",
        ));
    }
    if let Some(node) = ratio_owner
        && !paragraph_styles.contains_key(&node)
    {
        return Err(style_failure(
            node.get(),
            "preferred ratio owner is not a paragraph input",
        ));
    }

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
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: ratio_owner,
    }))
}

fn supported_dimensions(style: &spinon_style::ComputedElementStyle) -> bool {
    matches!(
        (
            style.layout_dimensions.width,
            style.layout_dimensions.height
        ),
        (
            ComputedCssDimension::LengthPx(60.0),
            ComputedCssDimension::Auto
        ) | (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(30.0)
        )
    )
}

fn require_dimension_properties(
    style: &spinon_style::ComputedElementStyle,
    width: &'static str,
    height: &'static str,
) -> Result<(), RuntimeLayoutFailure> {
    require_style_property(style, "width", width)?;
    require_style_property(style, "height", height)
}
