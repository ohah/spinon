use std::collections::{BTreeMap, HashSet};

use spinon_core::{HostDocumentSnapshot, HostNodeKind, NodeId};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridLayout, ComputedCssGridTemplateAreas,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedCssMaxSize,
    ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn validate_root_html_intrinsic_named_area_candidate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    container: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
    areas: &ComputedCssGridTemplateAreas,
) -> Result<(), StyleLayoutError> {
    let owner = container.node_id;
    validate_root_html_intrinsic_named_area_template(owner, grid, areas)?;

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
            .is_some_and(|candidate| candidate.template_areas.is_some())
    });
    if area_containers.next().map(|element| element.node_id) != Some(owner)
        || area_containers.next().is_some()
        || nodes.grid_containers.len() != 1
        || !nodes.grid_containers.contains(&owner)
    {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML intrinsic named-area 경로는 visible Grid container 하나만 허용합니다",
        ));
    }

    for (property, value) in [
        ("width", container.layout_dimensions.width),
        ("height", container.layout_dimensions.height),
    ] {
        if !matches!(value, ComputedCssDimension::LengthPx(actual) if actual.is_finite() && actual > 0.0)
        {
            return Err(unsupported(
                owner,
                property,
                "Root HTML intrinsic named-area 경로는 definite positive CSS px 크기를 요구합니다",
            ));
        }
    }
    for (property, is_default) in [
        (
            "min-width",
            matches!(
                container.layout_dimensions.min_width,
                ComputedCssDimension::Auto
            ),
        ),
        (
            "max-width",
            container.layout_dimensions.max_width == ComputedCssMaxSize::None,
        ),
        (
            "min-height",
            matches!(
                container.layout_dimensions.min_height,
                ComputedCssDimension::Auto
            ),
        ),
        (
            "max-height",
            container.layout_dimensions.max_height == ComputedCssMaxSize::None,
        ),
    ] {
        if !is_default {
            return Err(unsupported(
                owner,
                property,
                "Root HTML intrinsic named-area 경로는 min/max 크기 제한을 허용하지 않습니다",
            ));
        }
    }

    for (property, value, basis) in [
        (
            "row-gap",
            container.layout_spacing.row_gap,
            container.layout_dimensions.height,
        ),
        (
            "column-gap",
            container.layout_spacing.column_gap,
            container.layout_dimensions.width,
        ),
    ] {
        super::root_html_grid_gap::validate_root_html_gap(container, property, value, basis)?;
    }

    if areas.areas.is_empty() {
        return Err(unsupported(
            owner,
            "grid-template-areas",
            "Root HTML intrinsic named-area 경로는 하나 이상의 named area를 요구합니다",
        ));
    }

    let mut area_names = Vec::<(&str, usize)>::new();
    area_names
        .try_reserve_exact(areas.areas.len())
        .map_err(|_| allocation_failure())?;
    for (index, area) in areas.areas.iter().enumerate() {
        area_names.push((area.name.as_str(), index));
    }
    area_names.sort_unstable_by(|left, right| left.0.cmp(right.0));

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
                        "Root HTML intrinsic named-area 경로는 익명 텍스트 item을 허용하지 않습니다",
                    ));
                }
            }
            HostNodeKind::Element(element) => {
                let item = child.id();
                if nodes.hidden.contains(&item) {
                    return Err(unsupported(
                        item,
                        "display",
                        "Root HTML intrinsic named-area 경로는 숨겨진 직계 자식을 허용하지 않습니다",
                    ));
                }
                if element.namespace() != HTML_NAMESPACE || element.local_name() != "div" {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML intrinsic named-area 경로는 HTML div 직계 item을 요구합니다",
                    ));
                }
                let item_children = tree
                    .children(child)
                    .ok_or(StyleLayoutError::MissingComputedElement(item))?;
                if item_children.into_iter().any(|nested| {
                    !tree
                        .node(nested)
                        .is_some_and(|node| matches!(node.kind(), HostNodeKind::Text(_)))
                }) {
                    return Err(unsupported(
                        item,
                        "grid-item",
                        "Root HTML intrinsic named-area 경로는 직계 text 내용만 있는 div item을 요구합니다",
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

pub(super) fn validate_root_html_intrinsic_named_area_template(
    node: NodeId,
    grid: &ComputedCssGridLayout,
    areas: &ComputedCssGridTemplateAreas,
) -> Result<(), StyleLayoutError> {
    let invalid = |value: &str| StyleLayoutError::UnsupportedComputedValue {
        node,
        property: "grid-template-areas",
        value: value.to_owned(),
    };
    let allocation_failure = || {
        StyleLayoutError::Layout(spinon_layout::LayoutError::GridAllocationFailure {
            node,
            property: "grid-template-areas",
        })
    };
    if areas.rows == 0
        || areas.columns == 0
        || areas.rows > spinon_layout::GRID_MAX_TRACKS
        || areas.columns > spinon_layout::GRID_MAX_TRACKS
        || areas.rows != grid.rows.len()
        || areas.columns != grid.columns.len()
    {
        return Err(invalid(
            "intrinsic named-area dimension은 non-empty explicit track list와 일치해야 합니다",
        ));
    }
    for (property, tracks) in [
        ("grid-template-columns", grid.columns.as_slice()),
        ("grid-template-rows", grid.rows.as_slice()),
    ] {
        if tracks.iter().any(|track| {
            matches!(
                track,
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::Auto,
                    max: ComputedCssGridTrackBreadth::LengthPx(_)
                        | ComputedCssGridTrackBreadth::Percentage(_),
                }
            )
        }) {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value: "Root HTML named-area intrinsic 경로는 minmax(auto, px|%) track을 지원하지 않습니다"
                    .to_owned(),
            });
        }
    }
    if areas.areas.len() > areas.rows.saturating_mul(areas.columns) {
        return Err(invalid("named area 수가 template cell 수를 넘었습니다"));
    }
    let mut metadata_bytes = areas
        .areas
        .len()
        .checked_mul(std::mem::size_of::<spinon_style::ComputedCssGridArea>())
        .filter(|bytes| *bytes <= super::grid_values::MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES)
        .ok_or_else(|| invalid("named-area metadata가 1 MiB 한도를 넘었습니다"))?;

    let mut names = HashSet::new();
    names
        .try_reserve(areas.areas.len())
        .map_err(|_| allocation_failure())?;
    let mut events = Vec::new();
    let event_count = areas
        .areas
        .len()
        .checked_mul(2)
        .ok_or_else(|| invalid("named-area rectangle event 수가 범위를 넘었습니다"))?;
    events
        .try_reserve_exact(event_count)
        .map_err(|_| allocation_failure())?;
    for (index, area) in areas.areas.iter().enumerate() {
        metadata_bytes = metadata_bytes
            .checked_add(area.name.len())
            .filter(|bytes| *bytes <= super::grid_values::MAX_GRID_TEMPLATE_AREAS_METADATA_BYTES)
            .ok_or_else(|| invalid("named-area metadata가 1 MiB 한도를 넘었습니다"))?;
        if area.name.is_empty()
            || area.name.as_bytes().contains(&0)
            || area.row_start >= area.row_end
            || area.row_end > areas.rows
            || area.column_start >= area.column_end
            || area.column_end > areas.columns
            || !names.insert(area.name.as_str())
        {
            return Err(invalid(
                "area 이름은 고유해야 하며 rectangle은 template 내부의 non-empty 범위여야 합니다",
            ));
        }
        events.push((area.row_end, false, index));
        events.push((area.row_start, true, index));
    }
    events.sort_unstable_by_key(|(row, starts, index)| (*row, *starts, *index));

    let mut active_columns = BTreeMap::<usize, usize>::new();
    for (_, starts, index) in events {
        let area = &areas.areas[index];
        if !starts {
            if active_columns.remove(&area.column_start) != Some(area.column_end) {
                return Err(invalid(
                    "named-area rectangle sweep metadata가 일치하지 않습니다",
                ));
            }
            continue;
        }
        let overlaps_previous = active_columns
            .range(..=area.column_start)
            .next_back()
            .is_some_and(|(_, end)| *end > area.column_start);
        let overlaps_next = active_columns
            .range(area.column_start..)
            .next()
            .is_some_and(|(start, _)| *start < area.column_end);
        if overlaps_previous || overlaps_next {
            return Err(invalid("named-area rectangle은 서로 겹칠 수 없습니다"));
        }
        active_columns.insert(area.column_start, area.column_end);
    }
    if !active_columns.is_empty() {
        return Err(invalid("named-area rectangle sweep가 끝나지 않았습니다"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "root_html_intrinsic_named_area_tests.rs"]
mod tests;
