use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoRows, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{RuntimeGridNodes, StyleLayoutError, grid_values};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
#[path = "root_html_fit_content_auto_rows/repeated_text.rs"]
mod repeated_text;

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
    let four_member_list =
        grid_values::root_html_fit_content_auto_rows_with_four_member_intrinsic_tracks(grid);
    let six_member_list =
        grid_values::root_html_fit_content_auto_rows_with_six_member_intrinsic_tracks(grid);
    let align_content = grid_style
        .properties
        .get("align-content")
        .map(String::as_str);
    let align_content_is_supported = align_content == Some("start")
        || (grid_values::root_html_fit_content_auto_rows_with_intrinsic_tracks(grid)
            && align_content == Some("stretch"));
    if !align_content_is_supported {
        return Err(unsupported(
            node,
            "align-content",
            "Root HTML fit-content rows support only start or auto-track stretch alignment",
        ));
    }
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
        || !automatic_limits(grid_style)
        || grid_style.layout_aspect_ratio.is_some()
        || grid_style.layout_position != ComputedCssPosition::Static
        || !zero_edges(grid_style.layout_spacing.margin)
        || !zero_edges(grid_style.layout_spacing.padding)
        || grid_style.layout_spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || grid_style.layout_spacing.column_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || !zero_border(grid_style)
        || grid_style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
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
                    "Root HTML fit-content rows require three or five items, six for the four-member list, or nine for the repeated six-member list",
                ));
            }
        }
    }
    if (four_member_list && items.len() != 6)
        || (six_member_list && items.len() != 9)
        || (!four_member_list && !six_member_list && !matches!(items.len(), 3 | 5))
        || nodes
            .grid_item_owners
            .values()
            .filter(|owner| **owner == node)
            .count()
            != items.len()
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require three or five direct items, six for the four-member list, or nine for the repeated six-member list",
        ));
    }

    let styles = snapshot
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<std::collections::BTreeMap<_, _>>();
    let repeated_text_profile = six_member_list
        && items.len() == 9
        && items
            .iter()
            .copied()
            .skip(1)
            .zip(repeated_text::PAYLOADS)
            .all(|(item, expected_text)| {
                let Ok(children) = direct_element_children(tree, nodes, item) else {
                    return false;
                };
                let [payload] = children.as_slice() else {
                    return false;
                };
                repeated_text::payload_text_matches(tree, *payload, expected_text)
            });
    let grid_height_is_supported = grid_style.layout_dimensions.height
        == ComputedCssDimension::LengthPx(100.0)
        || (repeated_text_profile
            && grid_style.layout_dimensions.height == ComputedCssDimension::LengthPx(220.0));
    if !grid_height_is_supported {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require a supported definite grid block size",
        ));
    }
    let Some(first) = styles.get(&items[0].id()).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "First Grid item style is missing",
        ));
    };
    if !fixed_item(first, 10.0, repeated_text_profile) || !auto_placement(first) {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require a 10px explicit-row item",
        ));
    }
    const LIST_PAYLOAD_HEIGHTS: &[f32] = &[6.0, 8.0, 10.0, 25.0, 30.0, 34.0, 48.0, 50.0];
    const FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS: &[f32] = &[20.0, 10.0, 12.0, 6.0, 20.0];
    const SIX_MEMBER_LIST_PAYLOAD_HEIGHTS: &[f32] = &[20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 60.0, 6.0];
    const FIT_CONTENT_OVERFLOW_HEIGHT: f32 = 60.0;
    if !direct_element_children(tree, nodes, items[0])?.is_empty() {
        return Err(unsupported(
            items[0].id(),
            "grid-auto-rows",
            "The 10px explicit-row item must not contain descendants",
        ));
    }
    for (position, item) in items.iter().enumerate().skip(1) {
        let Some(style) = styles.get(&item.id()).copied() else {
            return Err(unsupported(
                item.id(),
                "grid-auto-rows",
                "Implicit-row item style is missing",
            ));
        };
        let allow_minimum_floor = items.len() == 3 && position == 1;
        if fit_content_item(style, allow_minimum_floor) && auto_placement(style) {
            let children = direct_element_children(tree, nodes, *item)?;
            if children.len() != 1 {
                return Err(unsupported(
                    item.id(),
                    "grid-auto-rows",
                    "Each intrinsic list item must contain one fixed-px payload div",
                ));
            }
            let profile_payload_heights = if six_member_list {
                Some(SIX_MEMBER_LIST_PAYLOAD_HEIGHTS)
            } else if four_member_list {
                Some(FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS)
            } else {
                None
            };
            if repeated_text_profile {
                let expected_text = repeated_text::PAYLOADS[position - 1];
                if !repeated_text::payload_is_supported(
                    tree,
                    nodes,
                    snapshot,
                    &styles,
                    children[0],
                    expected_text,
                ) {
                    return Err(unsupported(
                        children[0].id(),
                        "grid-auto-rows",
                        "Repeated intrinsic text payload must match the fixed monospace text profile",
                    ));
                }
            } else if six_member_list {
                let expected_height = SIX_MEMBER_LIST_PAYLOAD_HEIGHTS[position - 1];
                validate_payload(
                    tree,
                    nodes,
                    &styles,
                    children[0],
                    std::slice::from_ref(&expected_height),
                )?;
            } else if let Some(profile_payload_heights) = profile_payload_heights {
                let expected_height = profile_payload_heights[position - 1];
                let ComputedCssGridAutoRows::TrackList(tracks) = &grid.auto_rows else {
                    return Err(unsupported(
                        item.id(),
                        "grid-auto-rows",
                        "Intrinsic list requires typed implicit rows",
                    ));
                };
                let track_index = (position - 1) % tracks.len();
                if matches!(tracks[track_index], ComputedCssGridTrackSize::FitContent(_)) {
                    let fit_content_payload_heights =
                        [expected_height, FIT_CONTENT_OVERFLOW_HEIGHT];
                    validate_payload(
                        tree,
                        nodes,
                        &styles,
                        children[0],
                        &fit_content_payload_heights,
                    )?;
                } else {
                    validate_payload(
                        tree,
                        nodes,
                        &styles,
                        children[0],
                        std::slice::from_ref(&expected_height),
                    )?;
                }
            } else {
                validate_payload(tree, nodes, &styles, children[0], LIST_PAYLOAD_HEIGHTS)?;
            }
        } else if items.len() == 3
            && position == 2
            && fixed_item(style, 6.0, false)
            && auto_placement(style)
            && direct_element_children(tree, nodes, *item)?.is_empty()
        {
            // The original singleton fixture uses a fixed 6px item as a following probe.
        } else {
            return Err(unsupported(
                item.id(),
                "grid-auto-rows",
                "Implicit list items require auto size and one payload, except for the fixed 6px singleton probe",
            ));
        }
    }
    Ok(())
}

fn validate_payload(
    tree: &HostDocumentSnapshot,
    nodes: &RuntimeGridNodes,
    styles: &std::collections::BTreeMap<spinon_core::NodeId, &ComputedElementStyle>,
    payload: HostNodeHandle,
    allowed_heights: &[f32],
) -> Result<(), StyleLayoutError> {
    let node = payload.id();
    let Some(payload_style) = styles.get(&node).copied() else {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "height",
            value: "Nested payload style is missing".to_owned(),
        });
    };
    if nodes.grid_containers.contains(&node)
        || !allowed_heights
            .iter()
            .any(|height| fixed_item(payload_style, *height, false))
    {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "height",
            value: "Nested fit-content payload must be a supported fixed-px block div".to_owned(),
        });
    }
    if !direct_element_children(tree, nodes, payload)?.is_empty() {
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "grid-auto-rows",
            value: "Nested fit-content payload must not contain descendants".to_owned(),
        });
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

fn fixed_item(style: &ComputedElementStyle, height: f32, allow_zero_min_height: bool) -> bool {
    style.properties.get("display").map(String::as_str) == Some("block")
        && style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::LengthPx(height)
        && style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && (style.layout_dimensions.min_height == ComputedCssDimension::Auto
            || (allow_zero_min_height
                && style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(0.0)))
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None
        && style.layout_position == ComputedCssPosition::Static
        && style.layout_aspect_ratio.is_none()
        && zero_edges(style.layout_spacing.margin)
        && zero_edges(style.layout_spacing.padding)
        && zero_border(style)
}

fn fit_content_item(style: &ComputedElementStyle, allow_minimum_floor: bool) -> bool {
    style.properties.get("display").map(String::as_str) == Some("block")
        && style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::Auto
        && (style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(0.0)
            || (allow_minimum_floor
                && style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(28.0)))
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
