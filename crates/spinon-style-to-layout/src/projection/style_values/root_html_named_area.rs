mod auto_minimum;

use spinon_core::{HostDocumentSnapshot, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridItemPlacement, ComputedCssGridPlacement,
    ComputedCssMaxSize, ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn validate_root_html_named_area_candidate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_placement_route_nodes: &std::collections::BTreeSet<spinon_core::NodeId>,
    explicit_placement_route_nodes: &std::collections::BTreeSet<spinon_core::NodeId>,
    one_axis_placement_route_nodes: &std::collections::BTreeSet<spinon_core::NodeId>,
) -> Result<(), StyleLayoutError> {
    if let Some(container) = snapshot.elements.iter().find(|element| {
        element.grid_layout.as_ref().is_some_and(|grid| {
            grid.template_areas.as_ref().is_some_and(|areas| {
                super::grid_values::root_html_dimension_matched_fit_content_area_tracks(grid, areas)
            })
        })
    }) {
        let grid = container
            .grid_layout
            .as_ref()
            .expect("fit-content area lookup requires a Grid value");
        let areas = grid
            .template_areas
            .as_ref()
            .expect("fit-content area lookup requires parsed areas");
        return super::root_html_fit_content_named_area::validate_root_html_fit_content_named_area_candidate(
            tree,
            snapshot,
            nodes,
            container,
            areas,
        );
    }

    if let Some(container) = snapshot.elements.iter().find(|element| {
        element.grid_layout.as_ref().is_some_and(|grid| {
            grid.template_areas.is_some()
                && super::grid_values::root_html_has_intrinsic_explicit_tracks(grid)
        })
    }) {
        let grid = container
            .grid_layout
            .as_ref()
            .expect("intrinsic named-area lookup requires a Grid value");
        let areas = grid
            .template_areas
            .as_ref()
            .expect("intrinsic named-area lookup requires parsed areas");
        return super::root_html_intrinsic_named_area::validate_root_html_intrinsic_named_area_candidate(
            tree, snapshot, nodes, container, grid, areas,
        );
    }

    let mut area_containers = snapshot.elements.iter().filter(|element| {
        element
            .grid_layout
            .as_ref()
            .is_some_and(|grid| grid.template_areas.is_some())
    });
    let Some(container) = area_containers.next() else {
        for item_node in &nodes.grid_items {
            let owner_allows_dense_placement = nodes
                .grid_item_owners
                .get(item_node)
                .is_some_and(|owner| dense_placement_route_nodes.contains(owner));
            let owner_allows_explicit_placement = nodes
                .grid_item_owners
                .get(item_node)
                .is_some_and(|owner| explicit_placement_route_nodes.contains(owner));
            let owner_allows_one_axis_placement = nodes
                .grid_item_owners
                .get(item_node)
                .is_some_and(|owner| one_axis_placement_route_nodes.contains(owner));
            let placement = snapshot
                .elements
                .iter()
                .find(|element| element.node_id == *item_node)
                .and_then(|element| element.grid_item_placement.as_ref());
            if owner_allows_dense_placement {
                if let Some(placement) = placement {
                    validate_root_html_dense_route_placement(*item_node, placement)?;
                }
            } else if owner_allows_explicit_placement || owner_allows_one_axis_placement {
                // The selected placement route validates numeric values and visible siblings later.
                if placement.is_some_and(root_html_has_named_grid_placement) {
                    return Err(StyleLayoutError::UnsupportedComputedValue {
                        node: *item_node,
                        property: "grid-area",
                        value: "이 Root HTML 경로에서 named grid line을 쓰려면 검증된 template-area map이 필요합니다"
                            .to_owned(),
                    });
                }
            } else if placement.is_some_and(root_html_has_non_auto_grid_placement) {
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
    let auto_minimum_axis = container
        .grid_layout
        .as_ref()
        .and_then(super::grid_values::root_html_named_area_auto_minimum_axis);
    let expected_size = match auto_minimum_axis {
        Some(super::grid_values::RootHtmlNamedAreaAutoMinimumAxis::Columns) => (100.0, 40.0),
        Some(super::grid_values::RootHtmlNamedAreaAutoMinimumAxis::Rows) => (80.0, 100.0),
        None => (120.0, 70.0),
    };
    for (property, value, expected) in [
        ("width", container.layout_dimensions.width, expected_size.0),
        (
            "height",
            container.layout_dimensions.height,
            expected_size.1,
        ),
    ] {
        if !matches!(value, ComputedCssDimension::LengthPx(actual) if actual == expected) {
            return Err(unsupported(
                owner,
                property,
                "Root HTML named-area 제한 profile은 지정된 고정 CSS px 크기를 요구합니다",
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
                "Root HTML named-area 제한 경로는 Grid container의 min/max 크기 제한을 허용하지 않습니다",
            ));
        }
    }
    let is_parsed_root_html = tree.root_children().any(|root| {
        tree.node(root).is_some_and(|node| {
            matches!(
                node.kind(),
                HostNodeKind::Element(element)
                    if element.namespace() == HTML_NAMESPACE && element.local_name() == "html"
            )
        })
    });
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
        if is_parsed_root_html
            && (container.layout_math_values.contains_key(property)
                || super::root_html_grid_gap::resolved_root_html_gap(value, basis) != Some(0.0))
        {
            return Err(unsupported(
                owner,
                property,
                "기존 parsed Root HTML named-area runtime 경로는 zero gap만 지원합니다",
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
    if let Some(axis) = auto_minimum_axis {
        auto_minimum::validate_box(container, axis, &unsupported)?;
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
    let mut area_items = [None; 3];
    for item in direct_items.into_iter().flatten() {
        let element = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == item)
            .ok_or(StyleLayoutError::MissingComputedElement(item))?;
        let placement = element.grid_item_placement.as_ref().ok_or(
            StyleLayoutError::MissingComputedProperty {
                node: item,
                property: "grid-area",
            },
        )?;
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
        area_items[area_index] = Some(element);
    }
    if item_areas.iter().any(Option::is_none) {
        return Err(unsupported(
            owner,
            "grid-area",
            "header, side, content 영역마다 직계 item 하나가 필요합니다",
        ));
    }
    if let Some(axis) = auto_minimum_axis {
        auto_minimum::validate_items(owner, axis, &area_items, &unsupported)?;
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

fn root_html_has_named_grid_placement(placement: &ComputedCssGridItemPlacement) -> bool {
    [
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ]
    .into_iter()
    .any(|value| {
        matches!(
            value,
            ComputedCssGridPlacement::NamedLine { .. } | ComputedCssGridPlacement::SpanNamed { .. }
        )
    })
}

fn validate_root_html_dense_route_placement(
    node: spinon_core::NodeId,
    placement: &ComputedCssGridItemPlacement,
) -> Result<(), StyleLayoutError> {
    for (property, value) in [
        ("grid-row-start", &placement.row_start),
        ("grid-row-end", &placement.row_end),
        ("grid-column-start", &placement.column_start),
        ("grid-column-end", &placement.column_end),
    ] {
        let supported = match value {
            ComputedCssGridPlacement::Auto => true,
            ComputedCssGridPlacement::Line(line) => *line > 0,
            ComputedCssGridPlacement::Span(span) => *span > 0,
            ComputedCssGridPlacement::NamedLine { .. }
            | ComputedCssGridPlacement::SpanNamed { .. }
            | ComputedCssGridPlacement::Unsupported(_) => false,
        };
        if !supported {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node,
                property,
                value:
                    "dense placement route accepts only auto, positive lines, and positive spans"
                        .to_owned(),
            });
        }
    }
    Ok(())
}

pub(super) fn root_html_named_area_name(placement: &ComputedCssGridItemPlacement) -> Option<&str> {
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

#[cfg(test)]
mod dense_route_placement_tests {
    use super::validate_root_html_dense_route_placement;
    use crate::StyleLayoutError;
    use spinon_core::NodeId;
    use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridPlacement};

    #[test]
    fn dense_route_ancestors_accept_only_auto_and_positive_numeric_placements() {
        let node = NodeId::new(11_252).unwrap();
        let mut placement = ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Auto,
            row_end: ComputedCssGridPlacement::Span(2),
            column_start: ComputedCssGridPlacement::Line(1),
            column_end: ComputedCssGridPlacement::Line(3),
        };
        assert!(validate_root_html_dense_route_placement(node, &placement).is_ok());

        placement.column_start = ComputedCssGridPlacement::Line(-1);
        assert!(matches!(
            validate_root_html_dense_route_placement(node, &placement),
            Err(StyleLayoutError::UnsupportedComputedValue {
                node: actual,
                property: "grid-column-start",
                ..
            }) if actual == node
        ));

        placement.column_start = ComputedCssGridPlacement::NamedLine {
            name: "track".to_owned(),
            occurrence: Some(1),
        };
        assert!(matches!(
            validate_root_html_dense_route_placement(node, &placement),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: "grid-column-start",
                ..
            })
        ));
    }
}
