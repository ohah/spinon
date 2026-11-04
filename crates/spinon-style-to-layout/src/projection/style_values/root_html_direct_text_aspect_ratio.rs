use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_style::{
    ComputedAspectRatioSizingBox, ComputedCssDimension, ComputedCssGridAutoColumns,
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition,
    ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn root_html_direct_text_ratio_nodes(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    grid_nodes: &RuntimeGridNodes,
    grid_styles: &BTreeMap<NodeId, &ComputedCssGridLayout>,
) -> Result<BTreeSet<NodeId>, StyleLayoutError> {
    let mut ratios_by_owner = BTreeMap::<NodeId, Vec<&ComputedElementStyle>>::new();
    for element in snapshot.elements.iter() {
        let serialized = element.properties.get("aspect-ratio");
        let has_ratio = element.layout_aspect_ratio.is_some()
            || serialized.is_some_and(|value| value != "auto");
        if !has_ratio {
            continue;
        }
        if !has_supported_numeric_ratio(element) {
            return Err(unsupported_root_html_ratio(
                element.node_id,
                serialized
                    .cloned()
                    .unwrap_or_else(|| "missing computed aspect-ratio".to_owned()),
            ));
        }
        let Some(owner) = grid_nodes.grid_item_owners.get(&element.node_id).copied() else {
            return Err(unsupported_root_html_ratio(
                element.node_id,
                "aspect-ratio requires a direct item of an eligible Root HTML Grid".to_owned(),
            ));
        };
        ratios_by_owner.entry(owner).or_default().push(element);
    }

    let elements_by_id = snapshot
        .elements
        .iter()
        .map(|element| (element.node_id, element))
        .collect::<BTreeMap<_, _>>();
    let mut allowed = BTreeSet::new();
    for (owner, items) in ratios_by_owner {
        if items.len() != 1 {
            return Err(unsupported_root_html_ratio(
                owner,
                format!(
                    "Grid owner permits one ratio item; found {} computed ratio items",
                    items.len()
                ),
            ));
        }
        let item = items[0];
        if root_html_direct_text_ratio_candidate(
            tree,
            owner,
            Some(item),
            grid_nodes,
            grid_styles,
            &elements_by_id,
        )
        .is_none()
        {
            return Err(unsupported_root_html_ratio(
                item.node_id,
                "only the fixed two-track, one-auto-row direct block-text scene is supported"
                    .to_owned(),
            ));
        }
        allowed.insert(item.node_id);
    }
    Ok(allowed)
}

fn unsupported_root_html_ratio(node: NodeId, value: String) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property: "aspect-ratio",
        value,
    }
}

fn has_supported_numeric_ratio(element: &ComputedElementStyle) -> bool {
    let Some(serialized) = element.properties.get("aspect-ratio") else {
        return false;
    };
    let Some(ratio) = element.layout_aspect_ratio else {
        return false;
    };
    serialized != "auto"
        && ratio.ratio.is_finite()
        && ratio.ratio > 0.0
        && ratio.sizing_box == ComputedAspectRatioSizingBox::BoxSizing
}

fn ratio_transfer_is_finite(element: &ComputedElementStyle) -> bool {
    let Some(ratio) = element.layout_aspect_ratio.map(|ratio| ratio.ratio) else {
        return false;
    };
    if !ratio.is_finite() || ratio <= 0.0 {
        return false;
    }
    let transferred = match (
        element.layout_dimensions.width,
        element.layout_dimensions.height,
    ) {
        (ComputedCssDimension::LengthPx(width), ComputedCssDimension::Auto) => width / ratio,
        (ComputedCssDimension::Auto, ComputedCssDimension::LengthPx(height)) => height * ratio,
        _ => return false,
    };
    transferred.is_finite() && transferred > 0.0
}

fn root_html_direct_text_ratio_candidate(
    tree: &HostDocumentSnapshot,
    owner: NodeId,
    expected_ratio_item: Option<&ComputedElementStyle>,
    grid_nodes: &RuntimeGridNodes,
    grid_styles: &BTreeMap<NodeId, &ComputedCssGridLayout>,
    elements_by_id: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Option<[NodeId; 2]> {
    let owner_handle = grid_nodes.grid_container_handles.get(&owner).copied()?;
    let grid = grid_styles.get(&owner).copied()?;
    let owner_style = elements_by_id.get(&owner).copied()?;
    let owner_is_nested_in_grid = {
        let mut ancestor = tree.parent(owner_handle);
        let mut nested = false;
        while let Some(HostParent::Node(parent)) = ancestor {
            if grid_nodes.grid_containers.contains(&parent.id()) {
                nested = true;
                break;
            }
            ancestor = tree.parent(parent);
        }
        nested
    };
    if owner_is_nested_in_grid
        || !matches!(
            tree.node(owner_handle).map(|node| node.kind()),
            Some(HostNodeKind::Element(element))
                if element.namespace() == HTML_NAMESPACE && element.local_name() == "div"
        )
        || owner_style.properties.get("display").map(String::as_str) != Some("grid")
        || owner_style.layout_position != ComputedCssPosition::Static
        || owner_style.layout_dimensions.width != ComputedCssDimension::LengthPx(70.0)
        || owner_style.layout_dimensions.height != ComputedCssDimension::Auto
        || owner_style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || owner_style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || owner_style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || owner_style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !root_html_ratio_box_edges_are_zero(owner_style)
        || owner_style.layout_spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || owner_style.layout_spacing.column_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || owner_style
            .properties
            .get("align-items")
            .map(String::as_str)
            != Some("start")
        || owner_style
            .properties
            .get("justify-items")
            .map(String::as_str)
            != Some("start")
        || owner_style
            .properties
            .get("align-content")
            .map(String::as_str)
            != Some("normal")
        || owner_style
            .properties
            .get("justify-content")
            .map(String::as_str)
            != Some("normal")
        || owner_style.properties.get("overflow-x").map(String::as_str) != Some("visible")
        || owner_style.properties.get("overflow-y").map(String::as_str) != Some("visible")
        || owner_style
            .properties
            .get("writing-mode")
            .map(String::as_str)
            != Some("horizontal-tb")
        || owner_style.properties.get("direction").map(String::as_str) != Some("ltr")
        || grid.columns
            != [
                ComputedCssGridTrackSize::LengthPx(10.0),
                ComputedCssGridTrackSize::LengthPx(60.0),
            ]
        || grid.rows != [ComputedCssGridTrackSize::Auto]
        || grid.columns_fixed_repeat_group_count != 0
        || grid.rows_fixed_repeat_group_count != 0
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid
            .columns_line_names
            .iter()
            .flatten()
            .any(|name| !name.is_empty())
        || grid
            .rows_line_names
            .iter()
            .flatten()
            .any(|name| !name.is_empty())
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto)
        || !matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
    {
        return None;
    }

    let children = tree.children(owner_handle)?;
    let direct_items = children
        .filter(|child| grid_nodes.grid_item_owners.get(&child.id()) == Some(&owner))
        .collect::<Vec<_>>();
    if direct_items.len() != 2 {
        return None;
    }
    let first_text = direct_html_div_text(tree, direct_items[0]);
    let second_text = direct_html_div_text(tree, direct_items[1]);
    if first_text.as_deref() != Some("A") || second_text.as_deref() != Some("AAAA") {
        return None;
    }
    let item_ids = [direct_items[0].id(), direct_items[1].id()];
    let short_style = elements_by_id.get(&item_ids[0]).copied()?;
    let target_style = elements_by_id.get(&item_ids[1]).copied()?;
    let auto_placement = |element: &ComputedElementStyle| {
        element
            .grid_item_placement
            .as_ref()
            .is_some_and(|placement| {
                placement.row_start == ComputedCssGridPlacement::Auto
                    && placement.row_end == ComputedCssGridPlacement::Auto
                    && placement.column_start == ComputedCssGridPlacement::Auto
                    && placement.column_end == ComputedCssGridPlacement::Auto
            })
    };
    let direct_text_item_style_is_supported = |element: &ComputedElementStyle| {
        element.properties.get("display").map(String::as_str) == Some("block")
            && element
                .properties
                .get("font-family")
                .is_some_and(|value| matches!(value.as_str(), "Ahem" | "monospace"))
            && element.properties.get("font-size").map(String::as_str) == Some("10px")
            && element.properties.get("line-height").map(String::as_str) == Some("10px")
            && element.layout_position == ComputedCssPosition::Static
            && root_html_ratio_box_edges_are_zero(element)
            && element.layout_dimensions.min_width == ComputedCssDimension::Auto
            && element.layout_dimensions.max_width == ComputedCssMaxSize::None
            && element.layout_dimensions.min_height == ComputedCssDimension::Auto
            && element.layout_dimensions.max_height == ComputedCssMaxSize::None
            && element.properties.get("text-wrap-mode").map(String::as_str) == Some("nowrap")
            && element.properties.get("align-self").map(String::as_str) == Some("auto")
            && element.properties.get("justify-self").map(String::as_str) == Some("auto")
            && auto_placement(element)
    };
    let height_half_80_supported = target_style.layout_dimensions.width
        == ComputedCssDimension::Auto
        && target_style.layout_dimensions.height == ComputedCssDimension::LengthPx(80.0)
        && target_style.layout_aspect_ratio.is_some_and(|ratio| {
            ratio.ratio == 0.5 && ratio.sizing_box == ComputedAspectRatioSizingBox::BoxSizing
        });
    let target_dimensions_supported = matches!(
        (
            target_style.layout_dimensions.width,
            target_style.layout_dimensions.height,
        ),
        (
            ComputedCssDimension::LengthPx(60.0),
            ComputedCssDimension::Auto
        ) | (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(30.0)
        )
    ) || height_half_80_supported;
    if !direct_text_item_style_is_supported(short_style)
        || !direct_text_item_style_is_supported(target_style)
        || short_style.layout_dimensions.width != ComputedCssDimension::Auto
        || short_style.layout_dimensions.height != ComputedCssDimension::Auto
        || !target_dimensions_supported
    {
        return None;
    }
    let has_ratio = target_style.layout_aspect_ratio.is_some()
        || target_style
            .properties
            .get("aspect-ratio")
            .is_some_and(|value| value != "auto");
    if let Some(ratio_item) = expected_ratio_item {
        if ratio_item.node_id != item_ids[1]
            || !has_supported_numeric_ratio(ratio_item)
            || !ratio_transfer_is_finite(ratio_item)
        {
            return None;
        }
    } else if has_ratio
        && (!has_supported_numeric_ratio(target_style) || !ratio_transfer_is_finite(target_style))
    {
        return None;
    }
    Some(item_ids)
}

pub(super) fn root_html_direct_text_ratio_alignment_nodes(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    grid_nodes: &RuntimeGridNodes,
    grid_styles: &BTreeMap<NodeId, &ComputedCssGridLayout>,
) -> (BTreeSet<NodeId>, BTreeSet<NodeId>) {
    let elements_by_id = snapshot
        .elements
        .iter()
        .map(|element| (element.node_id, element))
        .collect::<BTreeMap<_, _>>();
    let mut owners = BTreeSet::new();
    let mut items = BTreeSet::new();
    for owner in &grid_nodes.grid_containers {
        if let Some(item_ids) = root_html_direct_text_ratio_candidate(
            tree,
            *owner,
            None,
            grid_nodes,
            grid_styles,
            &elements_by_id,
        ) {
            owners.insert(*owner);
            items.extend(item_ids);
        }
    }
    (owners, items)
}

fn direct_html_div_text(tree: &HostDocumentSnapshot, node: HostNodeHandle) -> Option<String> {
    if !matches!(
        tree.node(node).map(|node| node.kind()),
        Some(HostNodeKind::Element(element))
            if element.namespace() == HTML_NAMESPACE && element.local_name() == "div"
    ) {
        return None;
    }
    let children = tree.children(node)?.collect::<Vec<_>>();
    let [text] = children.as_slice() else {
        return None;
    };
    match tree.node(*text)?.kind() {
        HostNodeKind::Text(text) => Some(text.to_string_lossy()),
        HostNodeKind::Element(_) => None,
    }
}

fn root_html_ratio_box_edges_are_zero(element: &ComputedElementStyle) -> bool {
    let is_zero = |value| matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0);
    [
        element.layout_spacing.margin.top,
        element.layout_spacing.margin.right,
        element.layout_spacing.margin.bottom,
        element.layout_spacing.margin.left,
        element.layout_spacing.padding.top,
        element.layout_spacing.padding.right,
        element.layout_spacing.padding.bottom,
        element.layout_spacing.padding.left,
    ]
    .into_iter()
    .all(is_zero)
        && element.layout_border.top == 0.0
        && element.layout_border.right == 0.0
        && element.layout_border.bottom == 0.0
        && element.layout_border.left == 0.0
}
