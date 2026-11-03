//! Fixed-track Root HTML Grid content-alignment admission for C11.

use std::collections::BTreeSet;

use spinon_core::{HostDocumentSnapshot, HostNodeKind, NodeId};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssOverflow,
    ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleSnapshot,
};

use super::RuntimeGridNodes;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

pub(super) fn eligible_nodes(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
) -> BTreeSet<NodeId> {
    let elements = snapshot
        .elements
        .iter()
        .map(|element| (element.node_id, element))
        .collect::<std::collections::BTreeMap<_, _>>();

    nodes
        .grid_containers
        .iter()
        .copied()
        .filter(|node| {
            let Some(element) = elements.get(node).copied() else {
                return false;
            };
            let Some(grid) = element.grid_layout.as_ref() else {
                return false;
            };
            supports_grid(element, grid)
                && supports_empty_direct_items(tree, element, grid, nodes, &elements)
        })
        .collect()
}

pub(super) fn supports_content_alignment_value(property: &str, value: &str) -> bool {
    match property {
        "align-content" | "justify-content" => matches!(
            value,
            "normal"
                | "stretch"
                | "start"
                | "end"
                | "flex-start"
                | "flex-end"
                | "center"
                | "space-between"
                | "space-around"
                | "space-evenly"
                | "safe center"
                | "unsafe center"
                | "safe end"
        ),
        _ => false,
    }
}

fn supports_grid(element: &ComputedElementStyle, grid: &ComputedCssGridLayout) -> bool {
    grid.auto_flow == ComputedCssGridAutoFlow::Row
        && !grid.columns.is_empty()
        && !grid.rows.is_empty()
        && grid.columns.iter().all(fixed_nonnegative_track)
        && grid.rows.iter().all(fixed_nonnegative_track)
        && grid.auto_rows == ComputedCssGridAutoRows::Auto
        && grid.auto_columns == ComputedCssGridAutoColumns::Auto
        && grid.columns_auto_repeat.is_none()
        && grid.rows_auto_repeat.is_none()
        && grid.template_areas.is_none()
        && grid.columns_fixed_repeat_group_count == 0
        && grid.rows_fixed_repeat_group_count == 0
        && grid.columns_line_names.iter().all(Vec::is_empty)
        && grid.rows_line_names.iter().all(Vec::is_empty)
        && fixed_nonnegative_dimension(element.layout_dimensions.width)
        && fixed_nonnegative_dimension(element.layout_dimensions.height)
        && element.layout_dimensions.min_width == ComputedCssDimension::Auto
        && element.layout_dimensions.min_height == ComputedCssDimension::Auto
        && element.layout_dimensions.max_width == ComputedCssMaxSize::None
        && element.layout_dimensions.max_height == ComputedCssMaxSize::None
        && element.layout_aspect_ratio.is_none()
        && fixed_nonnegative_gap(element.layout_spacing.column_gap)
        && fixed_nonnegative_gap(element.layout_spacing.row_gap)
        && zero_spacing(element.layout_spacing.margin.top)
        && zero_spacing(element.layout_spacing.margin.right)
        && zero_spacing(element.layout_spacing.margin.bottom)
        && zero_spacing(element.layout_spacing.margin.left)
        && zero_spacing(element.layout_spacing.padding.top)
        && zero_spacing(element.layout_spacing.padding.right)
        && zero_spacing(element.layout_spacing.padding.bottom)
        && zero_spacing(element.layout_spacing.padding.left)
        && element.layout_border.top == 0.0
        && element.layout_border.right == 0.0
        && element.layout_border.bottom == 0.0
        && element.layout_border.left == 0.0
        && element.layout_position == spinon_style::ComputedCssPosition::Static
        && element.layout_overflow_x == Some(ComputedCssOverflow::Visible)
        && element.layout_overflow_y == Some(ComputedCssOverflow::Visible)
        && element
            .properties
            .get("writing-mode")
            .is_some_and(|value| value == "horizontal-tb")
        && element
            .properties
            .get("direction")
            .is_some_and(|value| value == "ltr")
}

fn supports_empty_direct_items(
    tree: &HostDocumentSnapshot,
    container: &ComputedElementStyle,
    grid: &ComputedCssGridLayout,
    nodes: &RuntimeGridNodes,
    elements: &std::collections::BTreeMap<NodeId, &ComputedElementStyle>,
) -> bool {
    let Some(handle) = nodes
        .grid_container_handles
        .get(&container.node_id)
        .copied()
    else {
        return false;
    };
    let Some(children) = tree.children(handle) else {
        return false;
    };
    let Some(cell_capacity) = grid.columns.len().checked_mul(grid.rows.len()) else {
        return false;
    };
    let mut item_count = 0usize;

    for child in children {
        let Some(child_node) = tree.node(child) else {
            return false;
        };
        match child_node.kind() {
            HostNodeKind::Text(text)
                if text.to_string_lossy().chars().all(|character| {
                    matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' ')
                }) => {}
            HostNodeKind::Element(host_element)
                if host_element.namespace() == HTML_NAMESPACE
                    && host_element.local_name() == "div" =>
            {
                if tree
                    .children(child)
                    .is_none_or(|mut grandchildren| grandchildren.next().is_some())
                {
                    return false;
                }
                let Some(style) = elements.get(&child.id()).copied() else {
                    return false;
                };
                if !supports_item(style)
                    || nodes.grid_item_owners.get(&child.id()) != Some(&container.node_id)
                {
                    return false;
                }
                item_count += 1;
            }
            _ => return false,
        }
    }

    item_count != 0 && item_count <= cell_capacity
}

fn supports_item(element: &ComputedElementStyle) -> bool {
    element
        .properties
        .get("display")
        .is_some_and(|value| value == "block")
        && element.layout_position == spinon_style::ComputedCssPosition::Static
        && fixed_nonnegative_dimension(element.layout_dimensions.width)
        && fixed_nonnegative_dimension(element.layout_dimensions.height)
        && element.layout_dimensions.min_width == ComputedCssDimension::Auto
        && element.layout_dimensions.min_height == ComputedCssDimension::Auto
        && element.layout_dimensions.max_width == ComputedCssMaxSize::None
        && element.layout_dimensions.max_height == ComputedCssMaxSize::None
        && element.layout_aspect_ratio.is_none()
        && element.layout_overflow_x == Some(ComputedCssOverflow::Visible)
        && element.layout_overflow_y == Some(ComputedCssOverflow::Visible)
        && element
            .properties
            .get("visibility")
            .is_some_and(|value| value == "visible")
        && zero_spacing(element.layout_spacing.margin.top)
        && zero_spacing(element.layout_spacing.margin.right)
        && zero_spacing(element.layout_spacing.margin.bottom)
        && zero_spacing(element.layout_spacing.margin.left)
        && zero_spacing(element.layout_spacing.padding.top)
        && zero_spacing(element.layout_spacing.padding.right)
        && zero_spacing(element.layout_spacing.padding.bottom)
        && zero_spacing(element.layout_spacing.padding.left)
        && element.layout_border.top == 0.0
        && element.layout_border.right == 0.0
        && element.layout_border.bottom == 0.0
        && element.layout_border.left == 0.0
        && element
            .properties
            .get("writing-mode")
            .is_some_and(|value| value == "horizontal-tb")
        && element
            .properties
            .get("direction")
            .is_some_and(|value| value == "ltr")
        && item_placement_is_auto(element.grid_item_placement.as_ref(), &element.properties)
}

fn item_placement_is_auto(
    placement: Option<&ComputedCssGridItemPlacement>,
    properties: &std::collections::BTreeMap<String, String>,
) -> bool {
    if let Some(placement) = placement {
        return [
            &placement.row_start,
            &placement.row_end,
            &placement.column_start,
            &placement.column_end,
        ]
        .into_iter()
        .all(|value| matches!(value, ComputedCssGridPlacement::Auto));
    }
    [
        "grid-row-start",
        "grid-row-end",
        "grid-column-start",
        "grid-column-end",
    ]
    .into_iter()
    .all(|property| {
        properties
            .get(property)
            .is_some_and(|value| value == "auto")
    })
}

fn fixed_nonnegative_track(track: &ComputedCssGridTrackSize) -> bool {
    matches!(track, ComputedCssGridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}

fn fixed_nonnegative_dimension(value: ComputedCssDimension) -> bool {
    matches!(value, ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0)
}

fn fixed_nonnegative_gap(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if value.is_finite() && value >= 0.0)
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0)
}

#[cfg(test)]
mod tests {
    use super::supports_content_alignment_value;

    #[test]
    fn content_alignment_allowlist_rejects_baseline_and_unknown_values() {
        for value in [
            "normal",
            "stretch",
            "start",
            "end",
            "flex-start",
            "flex-end",
            "center",
            "space-between",
            "space-around",
            "space-evenly",
            "safe center",
            "unsafe center",
            "safe end",
        ] {
            assert!(supports_content_alignment_value("align-content", value));
            assert!(supports_content_alignment_value("justify-content", value));
        }
        for value in [
            "baseline",
            "first baseline",
            "last baseline",
            "unsafe end",
            "safe start",
        ] {
            assert!(!supports_content_alignment_value("align-content", value));
            assert!(!supports_content_alignment_value("justify-content", value));
        }
        assert!(!supports_content_alignment_value("align-items", "center"));
    }
}
