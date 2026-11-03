use spinon_core::{HostDocumentSnapshot, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssMaxSize, ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn validate_root_html_fit_content_named_area_candidate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    container: &ComputedElementStyle,
    areas: &spinon_style::ComputedCssGridTemplateAreas,
) -> Result<(), StyleLayoutError> {
    let owner = container.node_id;
    let dimensions = container.layout_dimensions;
    let spacing = container.layout_spacing;
    let unsupported = |node, property, value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    };
    let allocation_failure = || {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node: owner,
            property: "grid-template-areas",
        })
    };
    let mut area_containers = snapshot.elements.iter().filter(|element| {
        element
            .grid_layout
            .as_ref()
            .is_some_and(|grid| grid.template_areas.is_some())
    });
    if area_containers.next().map(|element| element.node_id) != Some(owner)
        || area_containers.next().is_some()
        || !nodes.grid_containers.contains(&owner)
    {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML fit-content named-area 경로는 visible named-area Grid 하나를 요구합니다",
        ));
    }
    for (property, value) in [("width", dimensions.width), ("height", dimensions.height)] {
        if !matches!(value, ComputedCssDimension::LengthPx(actual) if actual.is_finite() && actual > 0.0)
        {
            return Err(unsupported(
                owner,
                property,
                "Root HTML fit-content named-area 경로는 definite positive px 크기를 요구합니다",
            ));
        }
    }
    for (property, is_default) in [
        (
            "min-width",
            matches!(dimensions.min_width, ComputedCssDimension::Auto),
        ),
        (
            "max-width",
            dimensions.max_width == ComputedCssMaxSize::None,
        ),
        (
            "min-height",
            matches!(dimensions.min_height, ComputedCssDimension::Auto),
        ),
        (
            "max-height",
            dimensions.max_height == ComputedCssMaxSize::None,
        ),
    ] {
        if !is_default {
            return Err(unsupported(
                owner,
                property,
                "Root HTML fit-content named-area 경로는 min/max 크기 제한을 허용하지 않습니다",
            ));
        }
    }
    for (property, value, basis) in [
        ("row-gap", spacing.row_gap, dimensions.height),
        ("column-gap", spacing.column_gap, dimensions.width),
    ] {
        super::root_html_grid_gap::validate_root_html_gap(container, property, value, basis)?;
    }

    if areas.areas.is_empty() {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "fit-content named-area map은 비어 있지 않은 parsed area를 요구합니다",
        ));
    }
    let mut area_names = Vec::<(&str, usize)>::new();
    area_names
        .try_reserve_exact(areas.areas.len())
        .map_err(|_| allocation_failure())?;
    for (index, area) in areas.areas.iter().enumerate() {
        if area.name.is_empty() || area.name.as_bytes().contains(&0) {
            return Err(unsupported(
                owner,
                "grid-template-areas",
                "area name은 비어 있지 않은 CSS identifier여야 합니다",
            ));
        }
        area_names.push((area.name.as_str(), index));
    }
    area_names.sort_unstable_by(|left, right| left.0.cmp(right.0));
    if area_names.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "fit-content named-area map은 중복 area name을 허용하지 않습니다",
        ));
    }

    let container_handle = nodes
        .grid_container_handles
        .get(&owner)
        .copied()
        .ok_or(StyleLayoutError::MissingComputedElement(owner))?;
    let children = tree
        .children(container_handle)
        .ok_or(StyleLayoutError::MissingComputedElement(owner))?;
    let mut direct_items = Vec::new();
    direct_items
        .try_reserve_exact(areas.areas.len())
        .map_err(|_| allocation_failure())?;
    for child in children {
        let child_node = tree
            .node(child)
            .ok_or(StyleLayoutError::MissingComputedElement(child.id()))?;
        match child_node.kind() {
            HostNodeKind::Text(text) => {
                if !text
                    .to_string_lossy()
                    .chars()
                    .all(|character| matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' '))
                {
                    return Err(unsupported(
                        owner,
                        "grid-item",
                        "Root HTML fit-content named-area 경로는 익명 텍스트 item을 허용하지 않습니다",
                    ));
                }
            }
            HostNodeKind::Element(element) => {
                let item = child.id();
                if nodes.hidden.contains(&item) {
                    return Err(unsupported(
                        item,
                        "display",
                        "Root HTML fit-content named-area 경로는 숨겨진 직계 자식을 허용하지 않습니다",
                    ));
                }
                if element.namespace() != HTML_NAMESPACE || element.local_name() != "div" {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML fit-content named-area 경로는 HTML div 직계 item을 요구합니다",
                    ));
                }
                if !child_node.children().is_empty() {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML fit-content named-area 경로는 비어 있는 div 직계 item을 요구합니다",
                    ));
                }
                if !nodes.grid_items.contains(&item)
                    || nodes.grid_item_owners.get(&item) != Some(&owner)
                {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "직계 자식이 visible in-flow Grid item이 아닙니다",
                    ));
                }
                if direct_items.len() == areas.areas.len() {
                    return Err(unsupported(
                        owner,
                        "grid-template-areas",
                        "named-area 수보다 많은 direct Grid item이 있습니다",
                    ));
                }
                direct_items.push(item);
            }
        }
    }
    if direct_items.len() != areas.areas.len() {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "각 named area에는 visible div direct item 하나가 필요합니다",
        ));
    }

    let mut used_areas = Vec::new();
    used_areas
        .try_reserve_exact(area_names.len())
        .map_err(|_| allocation_failure())?;
    used_areas.resize(area_names.len(), false);
    for item in &direct_items {
        let placement = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == *item)
            .and_then(|element| element.grid_item_placement.as_ref())
            .ok_or(StyleLayoutError::MissingComputedProperty {
                node: *item,
                property: "grid-area",
            })?;
        let Some(area_name) = super::root_html_named_area::root_html_named_area_name(placement)
        else {
            return Err(unsupported(
                *item,
                "grid-area",
                "각 item의 네 placement edge는 같은 computed named-area를 가리켜야 합니다",
            ));
        };
        let Ok(sorted_index) = area_names.binary_search_by(|(name, _)| name.cmp(&area_name)) else {
            return Err(unsupported(
                *item,
                "grid-area",
                "grid-area는 parsed template의 named area를 가리켜야 합니다",
            ));
        };
        let area_index = area_names[sorted_index].1;
        if std::mem::replace(&mut used_areas[area_index], true) {
            return Err(unsupported(
                *item,
                "grid-area",
                "각 named area에는 직계 item 하나만 연결할 수 있습니다",
            ));
        }
    }
    if used_areas.iter().any(|used| !used) {
        return Err(unsupported(
            owner,
            "grid-area",
            "각 parsed named area에 직계 item 하나가 필요합니다",
        ));
    }

    direct_items.sort_unstable();
    let owner_items = nodes
        .grid_item_owners
        .iter()
        .filter_map(|(item, item_owner)| (*item_owner == owner).then_some(*item));
    if !direct_items.iter().copied().eq(owner_items) {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "visible Grid item은 named-area Grid의 direct child여야 합니다",
        ));
    }
    Ok(())
}
