use super::*;
use spinon_core::HostNodeHandle;

pub(super) fn validate_payload(
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

pub(super) fn direct_element_children(
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

pub(super) fn explicit_empty_span_item_placement(style: &ComputedElementStyle) -> bool {
    style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Line(2)
            && placement.column_start == ComputedCssGridPlacement::Line(1)
            && placement.column_end == ComputedCssGridPlacement::Line(2)
    })
}

pub(super) fn repeated_span_item_kind(style: &ComputedElementStyle) -> Option<(i32, i32)> {
    let placement = style.grid_item_placement.as_ref()?;
    if placement.column_start != ComputedCssGridPlacement::Line(1)
        || placement.column_end != ComputedCssGridPlacement::Line(2)
    {
        return None;
    }
    match (&placement.row_start, &placement.row_end) {
        (ComputedCssGridPlacement::Line(2), ComputedCssGridPlacement::Span(2)) => Some((2, 2)),
        (ComputedCssGridPlacement::Line(3), ComputedCssGridPlacement::Span(2)) => Some((3, 2)),
        (ComputedCssGridPlacement::Line(4), ComputedCssGridPlacement::Span(2)) => Some((4, 2)),
        (ComputedCssGridPlacement::Line(5), ComputedCssGridPlacement::Span(2)) => Some((5, 2)),
        (ComputedCssGridPlacement::Line(6), ComputedCssGridPlacement::Span(2)) => Some((6, 2)),
        (ComputedCssGridPlacement::Line(2), ComputedCssGridPlacement::Span(3)) => Some((2, 3)),
        (ComputedCssGridPlacement::Line(4), ComputedCssGridPlacement::Span(3)) => Some((4, 3)),
        (ComputedCssGridPlacement::Line(5), ComputedCssGridPlacement::Span(3)) => Some((5, 3)),
        _ => None,
    }
}

pub(super) fn repeated_span_item_placement(style: &ComputedElementStyle) -> bool {
    repeated_span_item_kind(style).is_some()
}

pub(super) fn repeated_span_writing_context(style: &ComputedElementStyle) -> bool {
    style.properties.get("direction").map(String::as_str) == Some("ltr")
        && style.properties.get("writing-mode").map(String::as_str) == Some("horizontal-tb")
}

pub(super) fn repeated_span_topology_implicit_rows(placements: &[(i32, i32)]) -> Option<usize> {
    let count = |candidate| {
        placements
            .iter()
            .filter(|placement| **placement == candidate)
            .count()
    };
    let exact = |expected: &[(i32, i32)]| {
        placements.len() == expected.len()
            && expected.iter().all(|placement| count(*placement) == 1)
    };

    if exact(&[(2, 2), (3, 2)]) || exact(&[(2, 2), (3, 2), (2, 3)]) {
        Some(3)
    } else if exact(&[(2, 2), (3, 2), (4, 2), (5, 2), (6, 2)])
        || exact(&[(2, 2), (3, 2), (4, 3), (5, 3)])
    {
        Some(6)
    } else {
        None
    }
}

pub(super) fn repeated_span_fit_positions_are_supported(
    positive_implicit_rows: usize,
    fit_positions: &[usize],
) -> bool {
    match fit_positions {
        [position] => match positive_implicit_rows {
            3 => (1..=3).contains(position),
            6 => (4..=6).contains(position),
            _ => false,
        },
        [first, second] => {
            positive_implicit_rows == 6
                && (1..=6).contains(first)
                && (1..=6).contains(second)
                && first < second
        }
        [first, second, third] => {
            positive_implicit_rows == 6
                && (1..=6).contains(first)
                && (1..=6).contains(second)
                && (1..=6).contains(third)
                && first < second
                && second < third
        }
        [first, second, third, fourth] => {
            positive_implicit_rows == 6
                && (1..=6).contains(first)
                && (1..=6).contains(second)
                && (1..=6).contains(third)
                && (1..=6).contains(fourth)
                && first < second
                && second < third
                && third < fourth
        }
        [first, second, third, fourth, fifth] => {
            positive_implicit_rows == 6
                && (1..=6).contains(first)
                && (1..=6).contains(second)
                && (1..=6).contains(third)
                && (1..=6).contains(fourth)
                && (1..=6).contains(fifth)
                && first < second
                && second < third
                && third < fourth
                && fourth < fifth
        }
        [1, 2, 3, 4, 5, 6] => positive_implicit_rows == 6,
        _ => false,
    }
}

pub(super) fn automatic_limits(style: &ComputedElementStyle) -> bool {
    style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && style.layout_dimensions.min_height == ComputedCssDimension::Auto
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None
}

pub(super) fn fixed_item(
    style: &ComputedElementStyle,
    height: f32,
    allow_zero_min_height: bool,
) -> bool {
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

pub(super) fn fit_content_item(
    style: &ComputedElementStyle,
    allow_minimum_floor: bool,
    allow_auto_minimum: bool,
) -> bool {
    style.properties.get("display").map(String::as_str) == Some("block")
        && style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::Auto
        && (style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(0.0)
            || (allow_minimum_floor
                && style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(28.0))
            || (allow_auto_minimum
                && style.layout_dimensions.min_height == ComputedCssDimension::Auto))
        && (!allow_auto_minimum
            || (style.properties.get("overflow-x").map(String::as_str) == Some("visible")
                && style.properties.get("overflow-y").map(String::as_str) == Some("visible")))
        && style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None
        && style.layout_position == ComputedCssPosition::Static
        && style.layout_aspect_ratio.is_none()
        && zero_edges(style.layout_spacing.margin)
        && zero_edges(style.layout_spacing.padding)
        && zero_border(style)
}

pub(super) fn auto_placement(style: &ComputedElementStyle) -> bool {
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

pub(super) fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0))
}

pub(super) fn zero_border(style: &ComputedElementStyle) -> bool {
    style.layout_border.top == 0.0
        && style.layout_border.right == 0.0
        && style.layout_border.bottom == 0.0
        && style.layout_border.left == 0.0
}

#[cfg(test)]
mod tests {
    use super::repeated_span_fit_positions_are_supported;

    #[test]
    fn repeated_span_five_and_six_fit_positions_require_six_implicit_rows() {
        assert!(repeated_span_fit_positions_are_supported(
            6,
            &[1, 2, 3, 4, 5, 6]
        ));
        assert!(!repeated_span_fit_positions_are_supported(
            3,
            &[1, 2, 3, 4, 5, 6]
        ));
        assert!(repeated_span_fit_positions_are_supported(
            6,
            &[1, 2, 3, 4, 5]
        ));
        assert!(!repeated_span_fit_positions_are_supported(
            3,
            &[1, 2, 3, 4, 5]
        ));
    }

    #[test]
    fn repeated_span_one_fit_positions_follow_their_row_profile() {
        assert!(repeated_span_fit_positions_are_supported(3, &[1]));
        assert!(repeated_span_fit_positions_are_supported(3, &[3]));
        assert!(!repeated_span_fit_positions_are_supported(3, &[4]));
        assert!(!repeated_span_fit_positions_are_supported(6, &[1]));
        assert!(repeated_span_fit_positions_are_supported(6, &[4]));
        assert!(repeated_span_fit_positions_are_supported(6, &[6]));
    }

    #[test]
    fn repeated_span_three_fit_positions_are_limited_to_six_rows() {
        for first in 1..=4 {
            for second in first + 1..=5 {
                for third in second + 1..=6 {
                    let positions = [first, second, third];
                    assert!(repeated_span_fit_positions_are_supported(6, &positions));
                    assert!(!repeated_span_fit_positions_are_supported(3, &positions));
                }
            }
        }

        for positions in [[1, 1, 2], [4, 3, 6], [0, 2, 4], [2, 4, 7]] {
            assert!(!repeated_span_fit_positions_are_supported(6, &positions));
        }
    }
}
