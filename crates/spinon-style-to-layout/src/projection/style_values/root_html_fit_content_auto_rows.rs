use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMaxSize,
    ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{RuntimeGridNodes, StyleLayoutError, grid_values};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

pub(super) fn validate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
) -> Result<(), StyleLayoutError> {
    let candidates = snapshot
        .elements
        .iter()
        .filter(|element| {
            nodes.grid_containers.contains(&element.node_id)
                && element
                    .grid_layout
                    .as_ref()
                    .is_some_and(grid_values::root_html_fit_content_implicit_rows_candidate)
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Ok(());
    }

    let node = candidates[0].node_id;
    let unsupported =
        |node, property, value: &'static str| StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: value.to_owned(),
        };
    if candidates.len() != 1 || nodes.grid_containers.len() != 1 {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require exactly one visible Grid owner",
        ));
    }

    let grid_style = candidates[0];
    let grid = grid_style
        .grid_layout
        .as_ref()
        .expect("fit-content candidate has computed Grid tracks");
    if grid.columns.as_slice() != [ComputedCssGridTrackSize::LengthPx(40.0)]
        || grid.rows.as_slice() != [ComputedCssGridTrackSize::LengthPx(10.0)]
        || grid
            .columns_line_names
            .iter()
            .any(|group| !group.is_empty())
        || grid.rows_line_names.iter().any(|group| !group.is_empty())
    {
        return Err(unsupported(
            node,
            "grid-template-rows",
            "Root HTML fit-content rows require the fixed 40px column and 10px explicit row shape",
        ));
    }
    if grid_style.layout_dimensions.width != ComputedCssDimension::LengthPx(40.0)
        || grid_style.layout_dimensions.height != ComputedCssDimension::LengthPx(100.0)
        || !automatic_limits(grid_style)
        || grid_style.layout_aspect_ratio.is_some()
        || grid_style.layout_position != ComputedCssPosition::Static
        || !zero_edges(grid_style.layout_spacing.margin)
        || !zero_edges(grid_style.layout_spacing.padding)
        || grid_style.layout_spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || grid_style.layout_spacing.column_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || !zero_border(grid_style)
        || grid_style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || grid_style
            .properties
            .get("align-content")
            .map(String::as_str)
            != Some("start")
        || grid_style.properties.get("align-items").map(String::as_str) != Some("normal")
        || grid_style
            .properties
            .get("justify-content")
            .map(String::as_str)
            != Some("start")
        || grid_style
            .properties
            .get("justify-items")
            .map(String::as_str)
            != Some("normal")
        || grid_style.properties.get("overflow-x").map(String::as_str) != Some("visible")
        || grid_style.properties.get("overflow-y").map(String::as_str) != Some("visible")
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require positive definite px axes, zero spacing and gaps, static positioning, start alignment, and visible overflow",
        ));
    }

    let Some(owner) = nodes.grid_container_handles.get(&node).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require an inspectable parsed Grid owner",
        ));
    };
    let children = tree
        .children(owner)
        .ok_or_else(|| unsupported(node, "grid-auto-rows", "Grid item children are unavailable"))?
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    for child in children {
        if nodes.hidden.contains(&child.id()) {
            continue;
        }
        let child_node = tree
            .node(child)
            .ok_or_else(|| unsupported(node, "grid-auto-rows", "Grid item is unavailable"))?;
        match child_node.kind() {
            HostNodeKind::Text(text) if text.to_string_lossy().trim().is_empty() => {}
            HostNodeKind::Element(element)
                if element.namespace() == HTML_NAMESPACE
                    && element.local_name() == "div"
                    && !nodes.grid_containers.contains(&child.id()) =>
            {
                items.push(child);
            }
            _ => {
                return Err(unsupported(
                    child.id(),
                    "grid-auto-rows",
                    "Root HTML fit-content rows require three visible direct HTML div items",
                ));
            }
        }
    }
    if items.len() != 3
        || nodes
            .grid_item_owners
            .values()
            .filter(|owner| **owner == node)
            .count()
            != 3
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require exactly three direct items",
        ));
    }

    let styles = snapshot
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<std::collections::BTreeMap<_, _>>();
    let Some(first) = styles.get(&items[0].id()).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "First Grid item style is missing",
        ));
    };
    let Some(middle) = styles.get(&items[1].id()).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Implicit-row item style is missing",
        ));
    };
    let Some(probe) = styles.get(&items[2].id()).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Following probe style is missing",
        ));
    };
    if !fixed_item(first, 10.0)
        || !auto_placement(first)
        || !fixed_item(probe, 6.0)
        || !auto_placement(probe)
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require 10px explicit-row and 6px following probe items",
        ));
    }
    if !middle_item(middle) || !auto_placement(middle) {
        return Err(unsupported(
            items[1].id(),
            "grid-auto-rows",
            "The capped implicit-row item requires auto placement, auto width/height, and min-height 0px or 28px",
        ));
    }

    let first_children = direct_element_children(tree, nodes, items[0])?;
    let middle_children = direct_element_children(tree, nodes, items[1])?;
    let probe_children = direct_element_children(tree, nodes, items[2])?;
    if !first_children.is_empty() || !probe_children.is_empty() || middle_children.len() != 1 {
        return Err(unsupported(
            items[1].id(),
            "grid-auto-rows",
            "Only the capped item may contain one nested payload div",
        ));
    }
    let payload = middle_children[0];
    let Some(payload_style) = styles.get(&payload.id()).copied() else {
        return Err(unsupported(
            payload.id(),
            "height",
            "Nested payload style is missing",
        ));
    };
    if nodes.grid_containers.contains(&payload.id()) || !fixed_item(payload_style, 34.0) {
        return Err(unsupported(
            payload.id(),
            "height",
            "Nested fit-content payload must be a fixed 34px block div",
        ));
    }
    let payload_children = direct_element_children(tree, nodes, payload)?;
    if !payload_children.is_empty() {
        return Err(unsupported(
            payload.id(),
            "grid-auto-rows",
            "Nested fit-content payload must not contain descendants",
        ));
    }
    Ok(())
}

fn direct_element_children(
    tree: &HostDocumentSnapshot,
    nodes: &RuntimeGridNodes,
    parent: HostNodeHandle,
) -> Result<Vec<HostNodeHandle>, StyleLayoutError> {
    let node = parent.id();
    let unsupported = |value: &'static str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property: "grid-auto-rows",
        value: value.to_owned(),
    };
    let children = tree
        .children(parent)
        .ok_or_else(|| unsupported("Grid item children are unavailable"))?
        .collect::<Vec<_>>();
    let mut elements = Vec::new();
    for child in children {
        if nodes.hidden.contains(&child.id()) {
            continue;
        }
        let child_node = tree
            .node(child)
            .ok_or_else(|| unsupported("Grid item descendant is unavailable"))?;
        match child_node.kind() {
            HostNodeKind::Text(text) if text.to_string_lossy().trim().is_empty() => {}
            HostNodeKind::Element(element)
                if element.namespace() == HTML_NAMESPACE && element.local_name() == "div" =>
            {
                elements.push(child);
            }
            _ => {
                return Err(unsupported(
                    "Grid items allow only nested HTML div elements",
                ));
            }
        }
    }
    Ok(elements)
}

fn automatic_limits(style: &ComputedElementStyle) -> bool {
    style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && style.layout_dimensions.min_height == ComputedCssDimension::Auto
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None
}

fn fixed_item(style: &ComputedElementStyle, height: f32) -> bool {
    style.properties.get("display").map(String::as_str) == Some("block")
        && style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::LengthPx(height)
        && automatic_limits(style)
        && style.layout_position == ComputedCssPosition::Static
        && style.layout_aspect_ratio.is_none()
        && zero_edges(style.layout_spacing.margin)
        && zero_edges(style.layout_spacing.padding)
        && zero_border(style)
}

fn middle_item(style: &ComputedElementStyle) -> bool {
    style.properties.get("display").map(String::as_str) == Some("block")
        && style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::Auto
        && matches!(
            style.layout_dimensions.min_height,
            ComputedCssDimension::LengthPx(value) if value == 0.0 || value == 28.0
        )
        && style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None
        && style.layout_position == ComputedCssPosition::Static
        && style.layout_aspect_ratio.is_none()
        && zero_edges(style.layout_spacing.margin)
        && zero_edges(style.layout_spacing.padding)
        && zero_border(style)
}

fn auto_placement(style: &ComputedElementStyle) -> bool {
    style.grid_item_placement.as_ref().is_some_and(|placement| {
        [
            &placement.row_start,
            &placement.row_end,
            &placement.column_start,
            &placement.column_end,
        ]
        .into_iter()
        .all(|value| matches!(value, ComputedCssGridPlacement::Auto))
    })
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0))
}

fn zero_border(style: &ComputedElementStyle) -> bool {
    style.layout_border.top == 0.0
        && style.layout_border.right == 0.0
        && style.layout_border.bottom == 0.0
        && style.layout_border.left == 0.0
}
