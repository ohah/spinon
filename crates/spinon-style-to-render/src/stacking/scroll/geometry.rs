use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::PositionedContainingBlockOwner;
use spinon_render::{CssRect, RuntimeScrollContainer};
use spinon_style::{ComputedCssOverflow, ComputedCssPosition, ComputedElementStyle};
use spinon_style_to_layout::StyleLayoutOutput;

use super::super::overflow_clip::clip_axis_bounds;
use super::super::reserve::FallibleReserve;
use super::RuntimeScrollGeometryContext;
use crate::StyleRenderError;
pub(in crate::stacking) fn runtime_scroll_containers<R: FallibleReserve>(
    context: &RuntimeScrollGeometryContext<'_>,
    reserve: &mut R,
) -> Result<Vec<RuntimeScrollContainer>, StyleRenderError> {
    let document = context.document;
    let root = context.root;
    let document_nodes = context.document_nodes;
    let parent_by_node = context.parent_by_node;
    let styles = context.styles;
    let output = context.output;
    let viewport = context.viewport;
    let mut containers = Vec::new();
    reserve.reserve(&mut containers, document_nodes.len(), "scroll containers")?;
    let html_body = html_body_for_root(document, root);
    let html_overflow = if html_body.is_some() {
        let root_style = styles
            .get(&root.id())
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(root.id()))?;
        Some(overflow_pair(root.id(), root_style)?)
    } else {
        None
    };
    let propagated_body = match (html_body, html_overflow) {
        (Some(body), Some((ComputedCssOverflow::Visible, ComputedCssOverflow::Visible))) => {
            Some(body)
        }
        _ => None,
    };

    for owner in document_nodes.iter().copied() {
        if propagated_body == Some(owner) {
            continue;
        }
        if has_display_none_ancestor(owner, parent_by_node, styles)? {
            continue;
        }
        let owner_style = styles
            .get(&owner)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(owner))?;
        let (overflow_x, overflow_y) = overflow_pair(owner, owner_style)?;

        if owner == root.id()
            && let (Some(body), Some(root_overflow)) = (html_body, html_overflow)
        {
            let body_style = styles
                .get(&body)
                .copied()
                .ok_or(StyleRenderError::MissingComputedStyle(body))?;
            let body_overflow = overflow_pair(body, body_style)?;
            let viewport_overflow = match root_overflow {
                (ComputedCssOverflow::Visible, ComputedCssOverflow::Visible) => body_overflow,
                value => value,
            };
            let scrollport = CssRect::new(0.0, 0.0, viewport.width_css_px, viewport.height_css_px)
                .map_err(|_| StyleRenderError::InvalidViewport)?;
            let (content_right, content_bottom) =
                if viewport_axis_can_scroll(root_overflow.0, viewport_overflow.0)
                    || viewport_axis_can_scroll(root_overflow.1, viewport_overflow.1)
                {
                    scrollable_extent(root.id(), context, scrollport, propagated_body, reserve)?
                } else {
                    (viewport.width_css_px, viewport.height_css_px)
                };
            let scrollable_width = if viewport_axis_can_scroll(root_overflow.0, viewport_overflow.0)
            {
                (content_right - scrollport.x()).max(viewport.width_css_px)
            } else {
                viewport.width_css_px
            };
            let scrollable_height =
                if viewport_axis_can_scroll(root_overflow.1, viewport_overflow.1) {
                    (content_bottom - scrollport.y()).max(viewport.height_css_px)
                } else {
                    viewport.height_css_px
                };
            containers.push(
                RuntimeScrollContainer::new(
                    owner,
                    scrollport,
                    scrollable_width,
                    scrollable_height,
                    0.0,
                    0.0,
                )?
                .with_parent(None),
            );
            continue;
        }
        if !is_scroll_container(overflow_x) && !is_scroll_container(overflow_y) {
            continue;
        }

        let owner_frame = output
            .layout
            .frames
            .get(&owner)
            .copied()
            .ok_or(StyleRenderError::MissingLayoutFrame(owner))?;
        validate_frame(owner, owner_frame)?;
        let owner_edges = output.layout.used_box_edges.get(&owner).copied().ok_or(
            StyleRenderError::MissingComputedProperty {
                node: owner,
                property: "used padding and border edges",
            },
        )?;
        let scrollport_x = owner_frame.x + owner_edges.border.left;
        let scrollport_y = owner_frame.y + owner_edges.border.top;
        let client_width =
            (owner_frame.width - owner_edges.border.left - owner_edges.border.right).max(0.0);
        let client_height =
            (owner_frame.height - owner_edges.border.top - owner_edges.border.bottom).max(0.0);
        let scrollport = CssRect::new(scrollport_x, scrollport_y, client_width, client_height)
            .map_err(|_| StyleRenderError::InvalidFrame(owner))?;
        let (scrollable_right, scrollable_bottom) =
            scrollable_extent(owner, context, scrollport, None, reserve)?;

        let scrollable_width = (scrollable_right - scrollport_x).max(client_width);
        let scrollable_height = (scrollable_bottom - scrollport_y).max(client_height);
        containers.push(
            RuntimeScrollContainer::new(
                owner,
                scrollport,
                scrollable_width,
                scrollable_height,
                0.0,
                0.0,
            )?
            .with_parent(parent_by_node.get(&owner).copied()),
        );
    }
    Ok(containers)
}

fn html_body_for_root(document: &HostDocumentSnapshot, root: HostNodeHandle) -> Option<NodeId> {
    let node = document.node(root)?;
    let HostNodeKind::Element(element) = node.kind() else {
        return None;
    };
    if element.namespace() != "http://www.w3.org/1999/xhtml" || element.local_name() != "html" {
        return None;
    }
    document.children(root)?.find_map(|child| {
        let node = document.node(child)?;
        let HostNodeKind::Element(element) = node.kind() else {
            return None;
        };
        (element.namespace() == "http://www.w3.org/1999/xhtml" && element.local_name() == "body")
            .then_some(child.id())
    })
}

fn viewport_axis_can_scroll(
    root_overflow: ComputedCssOverflow,
    effective_overflow: ComputedCssOverflow,
) -> bool {
    !matches!(
        root_overflow,
        ComputedCssOverflow::Hidden | ComputedCssOverflow::Clip
    ) && effective_overflow != ComputedCssOverflow::Clip
}

fn scrollable_extent<R: FallibleReserve>(
    owner: NodeId,
    context: &RuntimeScrollGeometryContext<'_>,
    scrollport: CssRect,
    skipped_clip: Option<NodeId>,
    reserve: &mut R,
) -> Result<(f32, f32), StyleRenderError> {
    let document_nodes = context.document_nodes;
    let parent_by_node = context.parent_by_node;
    let styles = context.styles;
    let output = context.output;
    let mut scrollable_right = scrollport.x() + scrollport.width();
    let mut scrollable_bottom = scrollport.y() + scrollport.height();
    let mut lineage = Vec::new();
    for descendant in document_nodes.iter().copied() {
        if descendant == owner {
            continue;
        }
        lineage.clear();
        if !collect_lineage_to_owner(descendant, owner, parent_by_node, &mut lineage, reserve)? {
            continue;
        }
        if lineage_contains_display_none(&lineage, styles)? {
            continue;
        }
        let descendant_style = styles
            .get(&descendant)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(descendant))?;
        if descendant_style.layout_position == ComputedCssPosition::Fixed
            || !absolute_containing_block_is_inside(
                descendant,
                descendant_style,
                owner,
                parent_by_node,
                output,
            )?
        {
            continue;
        }
        let frame = output
            .layout
            .frames
            .get(&descendant)
            .copied()
            .ok_or(StyleRenderError::MissingLayoutFrame(descendant))?;
        validate_frame(descendant, frame)?;
        let (left, top, right, bottom) =
            clip_descendant_frame(frame, &lineage, styles, output, skipped_clip)?;
        if right > left {
            scrollable_right = scrollable_right.max(right);
        }
        if bottom > top {
            scrollable_bottom = scrollable_bottom.max(bottom);
        }
    }
    Ok((scrollable_right, scrollable_bottom))
}

fn has_display_none_ancestor(
    node: NodeId,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<bool, StyleRenderError> {
    let mut cursor = Some(node);
    while let Some(current) = cursor {
        if is_display_none(current, styles)? {
            return Ok(true);
        }
        cursor = parent_by_node.get(&current).copied();
    }
    Ok(false)
}

fn lineage_contains_display_none(
    lineage: &[NodeId],
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<bool, StyleRenderError> {
    for node in lineage.iter().copied() {
        if is_display_none(node, styles)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn is_display_none(
    node: NodeId,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<bool, StyleRenderError> {
    let style = styles
        .get(&node)
        .copied()
        .ok_or(StyleRenderError::MissingComputedStyle(node))?;
    Ok(style.properties.get("display").map(String::as_str) == Some("none"))
}

fn collect_lineage_to_owner<R: FallibleReserve>(
    descendant: NodeId,
    owner: NodeId,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    lineage: &mut Vec<NodeId>,
    reserve: &mut R,
) -> Result<bool, StyleRenderError> {
    let mut cursor = descendant;
    while cursor != owner {
        reserve.reserve(lineage, 1, "scroll descendant ancestry")?;
        lineage.push(cursor);
        let Some(parent) = parent_by_node.get(&cursor).copied() else {
            return Ok(false);
        };
        cursor = parent;
    }
    Ok(true)
}

fn absolute_containing_block_is_inside(
    node: NodeId,
    style: &ComputedElementStyle,
    scroll_owner: NodeId,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    output: &StyleLayoutOutput,
) -> Result<bool, StyleRenderError> {
    if style.layout_position != ComputedCssPosition::Absolute {
        return Ok(true);
    }
    let owner = output.layout.positioned_owners.get(&node).copied().ok_or(
        StyleRenderError::MissingComputedProperty {
            node,
            property: "absolute containing block owner",
        },
    )?;
    let PositionedContainingBlockOwner::Node(mut cursor) = owner else {
        return Ok(false);
    };
    loop {
        if cursor == scroll_owner {
            return Ok(true);
        }
        let Some(parent) = parent_by_node.get(&cursor).copied() else {
            return Ok(false);
        };
        cursor = parent;
    }
}

fn clip_descendant_frame(
    frame: spinon_layout::LayoutFrame,
    lineage: &[NodeId],
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
    output: &StyleLayoutOutput,
    skipped_clip: Option<NodeId>,
) -> Result<(f32, f32, f32, f32), StyleRenderError> {
    let mut left = frame.x;
    let mut top = frame.y;
    let mut right = frame.x + frame.width;
    let mut bottom = frame.y + frame.height;
    for clip_node in lineage.iter().copied().skip(1) {
        if skipped_clip == Some(clip_node) {
            continue;
        }
        let style = styles
            .get(&clip_node)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(clip_node))?;
        let (overflow_x, overflow_y) = overflow_pair(clip_node, style)?;
        if overflow_x != ComputedCssOverflow::Visible {
            let (min, max) = ancestor_clip_bounds(clip_node, true, overflow_x, style, output)?;
            left = left.max(min);
            right = right.min(max);
        }
        if overflow_y != ComputedCssOverflow::Visible {
            let (min, max) = ancestor_clip_bounds(clip_node, false, overflow_y, style, output)?;
            top = top.max(min);
            bottom = bottom.min(max);
        }
    }
    Ok((left, top, right, bottom))
}

fn ancestor_clip_bounds(
    node: NodeId,
    horizontal: bool,
    overflow: ComputedCssOverflow,
    style: &ComputedElementStyle,
    output: &StyleLayoutOutput,
) -> Result<(f32, f32), StyleRenderError> {
    let frame = output
        .layout
        .frames
        .get(&node)
        .copied()
        .ok_or(StyleRenderError::MissingLayoutFrame(node))?;
    let edges = output.layout.used_box_edges.get(&node).copied().ok_or(
        StyleRenderError::MissingComputedProperty {
            node,
            property: "used padding and border edges",
        },
    )?;
    let margin = style
        .overflow_clip_margin
        .ok_or(StyleRenderError::MissingComputedProperty {
            node,
            property: "overflow-clip-margin",
        })?;
    if horizontal {
        Ok(clip_axis_bounds(
            overflow,
            margin,
            frame.x,
            frame.x + frame.width,
            frame.x + edges.border.left,
            frame.x + frame.width - edges.border.right,
            frame.x + edges.border.left + edges.padding.left,
            frame.x + frame.width - edges.border.right - edges.padding.right,
        ))
    } else {
        Ok(clip_axis_bounds(
            overflow,
            margin,
            frame.y,
            frame.y + frame.height,
            frame.y + edges.border.top,
            frame.y + frame.height - edges.border.bottom,
            frame.y + edges.border.top + edges.padding.top,
            frame.y + frame.height - edges.border.bottom - edges.padding.bottom,
        ))
    }
}

fn overflow_pair(
    node: NodeId,
    style: &ComputedElementStyle,
) -> Result<(ComputedCssOverflow, ComputedCssOverflow), StyleRenderError> {
    let overflow_x = style
        .layout_overflow_x
        .ok_or(StyleRenderError::MissingComputedProperty {
            node,
            property: "overflow-x",
        })?;
    let overflow_y = style
        .layout_overflow_y
        .ok_or(StyleRenderError::MissingComputedProperty {
            node,
            property: "overflow-y",
        })?;
    Ok((overflow_x, overflow_y))
}

fn is_scroll_container(value: ComputedCssOverflow) -> bool {
    matches!(
        value,
        ComputedCssOverflow::Hidden | ComputedCssOverflow::Scroll | ComputedCssOverflow::Auto
    )
}

fn validate_frame(node: NodeId, frame: spinon_layout::LayoutFrame) -> Result<(), StyleRenderError> {
    let values = [frame.x, frame.y, frame.width, frame.height];
    if values.into_iter().any(|value| !value.is_finite()) || frame.width < 0.0 || frame.height < 0.0
    {
        return Err(StyleRenderError::InvalidFrame(node));
    }
    Ok(())
}
