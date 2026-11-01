use spinon_core::{HostDocumentSnapshot, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridItemPlacement, ComputedCssGridPlacement,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn validate_root_html_named_area_candidate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
) -> Result<(), StyleLayoutError> {
    let mut area_containers = snapshot.elements.iter().filter(|element| {
        element
            .grid_layout
            .as_ref()
            .is_some_and(|grid| grid.template_areas.is_some())
    });
    let Some(container) = area_containers.next() else {
        for item_node in &nodes.grid_items {
            let placement = snapshot
                .elements
                .iter()
                .find(|element| element.node_id == *item_node)
                .and_then(|element| element.grid_item_placement.as_ref());
            if placement.is_some_and(root_html_has_non_auto_grid_placement) {
                return Err(StyleLayoutError::UnsupportedComputedValue {
                    node: *item_node,
                    property: "grid-area",
                    value: "Root HTML Grid item 배치는 제한 named-area template를 요구합니다"
                        .to_owned(),
                });
            }
        }
        return Ok(());
    };
    let owner = container.node_id;
    let unsupported = |node, property, value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    };
    if area_containers.next().is_some()
        || nodes.grid_containers.len() != 1
        || !nodes.grid_containers.contains(&owner)
    {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML named-area 제한 경로는 visible Grid container 하나만 허용합니다",
        ));
    }
    for (property, value, expected) in [
        ("width", container.layout_dimensions.width, 120.0),
        ("height", container.layout_dimensions.height, 70.0),
    ] {
        if !matches!(value, ComputedCssDimension::LengthPx(actual) if actual == expected) {
            return Err(unsupported(
                owner,
                property,
                "Root HTML named-area 제한 경로는 120px × 70px Grid container를 요구합니다",
            ));
        }
    }
    if !matches!(
        container.layout_dimensions.min_width,
        ComputedCssDimension::Auto
    ) || container.layout_dimensions.max_width != ComputedCssMaxSize::None
        || !matches!(
            container.layout_dimensions.min_height,
            ComputedCssDimension::Auto
        )
        || container.layout_dimensions.max_height != ComputedCssMaxSize::None
    {
        return Err(unsupported(
            owner,
            "min-width",
            "Root HTML named-area 제한 경로는 Grid container의 min/max 크기 제한을 허용하지 않습니다",
        ));
    }
    for (property, value) in [
        ("row-gap", container.layout_spacing.row_gap),
        ("column-gap", container.layout_spacing.column_gap),
    ] {
        if !matches!(value, ComputedCssSpacingValue::Normal)
            && !matches!(value, ComputedCssSpacingValue::LengthPx(gap) if gap == 0.0)
        {
            return Err(unsupported(
                owner,
                property,
                "Root HTML named-area 제한 경로는 Grid gap 0을 요구합니다",
            ));
        }
    }
    let container_handle = nodes
        .grid_container_handles
        .get(&owner)
        .copied()
        .ok_or(StyleLayoutError::MissingComputedElement(owner))?;
    let children = tree
        .children(container_handle)
        .ok_or(StyleLayoutError::MissingComputedElement(owner))?;
    let mut direct_items = [None; 3];
    let mut direct_item_count = 0;
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
                        "Root HTML named-area 제한 경로는 익명 텍스트 item을 허용하지 않습니다",
                    ));
                }
            }
            HostNodeKind::Element(element) => {
                let item = child.id();
                if nodes.hidden.contains(&item) {
                    return Err(unsupported(
                        item,
                        "display",
                        "Root HTML named-area 제한 경로는 숨겨진 직계 자식을 허용하지 않습니다",
                    ));
                }
                if element.namespace() != HTML_NAMESPACE || element.local_name() != "div" {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML named-area 제한 경로는 HTML div 직계 item을 요구합니다",
                    ));
                }
                if !child_node.children().is_empty() {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML named-area 제한 경로는 비어 있는 div 직계 item을 요구합니다",
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
                if direct_item_count == direct_items.len() {
                    return Err(unsupported(
                        owner,
                        "grid-template-areas",
                        "Root HTML named-area 제한 경로는 visible div 직계 item 세 개만 허용합니다",
                    ));
                }
                direct_items[direct_item_count] = Some(item);
                direct_item_count += 1;
            }
        }
    }
    if direct_item_count != direct_items.len() {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML named-area 제한 경로는 visible div 직계 item 세 개를 요구합니다",
        ));
    }
    let mut projected_item_count = 0;
    for (item, item_owner) in &nodes.grid_item_owners {
        if *item_owner != owner {
            continue;
        }
        projected_item_count += 1;
        if !direct_items.contains(&Some(*item)) {
            return Err(unsupported(
                *item,
                "grid-item",
                "visible Grid item은 named-area Grid의 직접 자식이어야 합니다",
            ));
        }
    }
    if projected_item_count != direct_item_count {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML named-area 제한 경로는 visible div 직계 item 세 개를 요구합니다",
        ));
    }
    let mut item_areas = [None; 3];
    for item in direct_items.into_iter().flatten() {
        let placement = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == item)
            .and_then(|element| element.grid_item_placement.as_ref())
            .ok_or(StyleLayoutError::MissingComputedProperty {
                node: item,
                property: "grid-area",
            })?;
        let Some(area) = root_html_named_area_name(placement) else {
            return Err(unsupported(
                item,
                "grid-area",
                "각 item의 네 placement edge는 같은 computed named-area를 가리켜야 합니다",
            ));
        };
        let area_index = match area {
            "header" => 0,
            "side" => 1,
            "content" => 2,
            _ => {
                return Err(unsupported(
                    item,
                    "grid-area",
                    "header, side, content 영역은 각각 직계 item 하나에만 연결해야 합니다",
                ));
            }
        };
        if item_areas[area_index].replace(area).is_some() {
            return Err(unsupported(
                item,
                "grid-area",
                "header, side, content 영역은 각각 직계 item 하나에만 연결해야 합니다",
            ));
        }
    }
    if item_areas.iter().any(Option::is_none) {
        return Err(unsupported(
            owner,
            "grid-area",
            "header, side, content 영역마다 직계 item 하나가 필요합니다",
        ));
    }
    Ok(())
}

fn root_html_has_non_auto_grid_placement(placement: &ComputedCssGridItemPlacement) -> bool {
    [
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ]
    .into_iter()
    .any(|value| !matches!(value, ComputedCssGridPlacement::Auto))
}

fn root_html_named_area_name(placement: &ComputedCssGridItemPlacement) -> Option<&str> {
    let endpoints = [
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ];
    let name = match endpoints.first()? {
        ComputedCssGridPlacement::NamedLine {
            name,
            occurrence: None,
        } => name.as_str(),
        _ => return None,
    };
    endpoints
        .iter()
        .all(|value| {
            matches!(
                value,
                ComputedCssGridPlacement::NamedLine {
                    name: endpoint_name,
                    occurrence: None,
                } if endpoint_name == name
            )
        })
        .then_some(name)
}
