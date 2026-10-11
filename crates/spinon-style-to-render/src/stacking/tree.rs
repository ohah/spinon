use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::LayoutFrame;
use spinon_render::CssRect;
use spinon_style::ComputedCssPosition;

use super::reserve::FallibleReserve;
use crate::StyleRenderError;

pub(super) fn ordered_children(
    document: &HostDocumentSnapshot,
    parent: NodeId,
    children: Vec<HostNodeHandle>,
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
    reserve: &mut impl FallibleReserve,
) -> Result<Vec<HostNodeHandle>, StyleRenderError> {
    if !root_display_is_flex(parent, styles)? {
        return Ok(children);
    }
    let reverse_paint_order = flex_direction_reverses_paint(parent, styles)?;
    let mut in_flow = Vec::new();
    for (source_order, child) in children.iter().copied().enumerate() {
        let Some(node) = document.node(child) else {
            return Err(StyleRenderError::InvalidRoot);
        };
        if !matches!(node.kind(), HostNodeKind::Element(_)) {
            continue;
        }
        let style = styles
            .get(&child.id())
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(child.id()))?;
        let is_displayed = style.properties.get("display").map(String::as_str) != Some("none");
        let is_in_flow = !matches!(
            style.layout_position,
            ComputedCssPosition::Absolute | ComputedCssPosition::Fixed
        );
        if is_displayed && is_in_flow {
            let value =
                style
                    .properties
                    .get("order")
                    .ok_or(StyleRenderError::MissingComputedProperty {
                        node: child.id(),
                        property: "order",
                    })?;
            let order =
                value
                    .parse::<i32>()
                    .map_err(|_| StyleRenderError::UnsupportedComputedValue {
                        node: child.id(),
                        property: "order",
                        value: value.clone(),
                    })?;
            reserve.reserve(&mut in_flow, 1, "flex order")?;
            in_flow.push((order, source_order, child));
        }
    }
    in_flow.sort_unstable_by_key(|(order, source_order, _)| (*order, *source_order));
    // 고정 Chromium 154는 reverse 주축에서 겹친 Flex item을 역순으로 hit-test/paint한다.
    // 문서 트리와 computed order를 바꾸지 않고 painter용 item sequence만 뒤집는다.
    if reverse_paint_order {
        in_flow.reverse();
    }
    let mut ordered = children;
    let mut items = in_flow.into_iter().map(|(_, _, child)| child);
    for child in &mut ordered {
        let node = document.node(*child).ok_or(StyleRenderError::InvalidRoot)?;
        if !matches!(node.kind(), HostNodeKind::Element(_)) {
            continue;
        }
        let style = styles
            .get(&child.id())
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(child.id()))?;
        let is_displayed = style.properties.get("display").map(String::as_str) != Some("none");
        let is_in_flow = !matches!(
            style.layout_position,
            ComputedCssPosition::Absolute | ComputedCssPosition::Fixed
        );
        if is_displayed && is_in_flow {
            *child = items.next().ok_or(StyleRenderError::InvalidRoot)?;
        }
    }
    Ok(ordered)
}

pub(super) fn child_handles(
    document: &HostDocumentSnapshot,
    parent: HostNodeHandle,
    reserve: &mut impl FallibleReserve,
) -> Result<Vec<HostNodeHandle>, StyleRenderError> {
    let mut children = Vec::new();
    if let Some(handles) = document.children(parent) {
        for handle in handles {
            reserve.reserve(&mut children, 1, "document children")?;
            children.push(handle);
        }
    }
    Ok(children)
}

fn computed_display<'a>(
    node: NodeId,
    styles: &'a BTreeMap<NodeId, &'a spinon_style::ComputedElementStyle>,
) -> Result<&'a str, StyleRenderError> {
    styles
        .get(&node)
        .ok_or(StyleRenderError::MissingComputedStyle(node))?
        .properties
        .get("display")
        .map(String::as_str)
        .ok_or(StyleRenderError::MissingComputedProperty {
            node,
            property: "display",
        })
}

fn flex_direction_reverses_paint(
    node: NodeId,
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<bool, StyleRenderError> {
    let style = styles
        .get(&node)
        .ok_or(StyleRenderError::MissingComputedStyle(node))?;
    let value = style
        .properties
        .get("flex-direction")
        .map(String::as_str)
        .ok_or(StyleRenderError::MissingComputedProperty {
            node,
            property: "flex-direction",
        })?;
    match value {
        "row" | "column" => Ok(false),
        "row-reverse" | "column-reverse" => Ok(true),
        _ => Err(StyleRenderError::UnsupportedComputedValue {
            node,
            property: "flex-direction",
            value: value.to_owned(),
        }),
    }
}

pub(super) fn root_display_is_flex(
    node: NodeId,
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<bool, StyleRenderError> {
    Ok(computed_display(node, styles)? == "flex")
}

pub(super) fn validate_runtime_frame(
    node: NodeId,
    frame: LayoutFrame,
) -> Result<(), StyleRenderError> {
    CssRect::new(frame.x, frame.y, frame.width, frame.height)
        .map(|_| ())
        .map_err(|_| StyleRenderError::InvalidFrame(node))
}
