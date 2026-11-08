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
        (ComputedCssGridPlacement::Line(2), ComputedCssGridPlacement::Span(3)) => Some((2, 3)),
        _ => None,
    }
}

pub(super) fn repeated_span_item_placement(style: &ComputedElementStyle) -> bool {
    repeated_span_item_kind(style).is_some()
}
