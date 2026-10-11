use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::{LayoutFrame, LayoutSourceRevision};
use spinon_render::{
    CssRect, CssSize, OpaqueCssSrgb, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey,
    RuntimeRenderSnapshot,
};
use spinon_style::{ComputedBackgroundPaint, ComputedStyleProfile};

use crate::adapter::{
    indexed_styles, validate_layout_node_set, validate_revisions, validate_style_node_set,
    validate_viewport,
};
use crate::{CurrentLayoutInputs, StyleRenderError};
use spinon_style_to_layout::StyleLayoutOutput;

/// 같은 HostDocument에서 계산한 runtime style/layout 결과를 dynamic scene으로 변환합니다.
pub fn build_runtime_render_snapshot(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    let styles = &output.computed_styles;
    if !matches!(
        styles.profile,
        ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    ) {
        return Err(StyleRenderError::UnsupportedRuntimeProfile);
    }
    if !styles.diagnostics.is_empty() {
        return Err(StyleRenderError::CascadeDiagnostics);
    }
    validate_viewport(styles.viewport)?;
    validate_viewport(current.viewport)?;
    validate_revisions(document, styles, output.layout.revision, current)?;

    let styles_by_node = indexed_styles(styles)?;
    let order = runtime_document_order(document, root, &styles_by_node)?;
    validate_style_node_set(&order.document_nodes, &styles_by_node)?;
    validate_layout_node_set(&order.document_nodes, &output.layout.frames)?;

    let viewport_css_px = CssSize::new(styles.viewport.width_css_px, styles.viewport.height_css_px)
        .map_err(|_| StyleRenderError::InvalidViewport)?;
    let mut boxes = Vec::with_capacity(order.paint_nodes.len());
    for node_id in order.paint_nodes {
        let style = styles_by_node
            .get(&node_id)
            .ok_or(StyleRenderError::MissingComputedStyle(node_id))?;
        let frame = output
            .layout
            .frames
            .get(&node_id)
            .copied()
            .ok_or(StyleRenderError::MissingLayoutFrame(node_id))?;
        validate_runtime_frame(node_id, frame)?;
        if style.properties.get("display").map(String::as_str) == Some("none")
            || frame.width == 0.0
            || frame.height == 0.0
        {
            continue;
        }
        let frame_css_px = CssRect::new(frame.x, frame.y, frame.width, frame.height)
            .map_err(|_| StyleRenderError::InvalidFrame(node_id))?;
        let paint = match style.background_paint {
            Some(ComputedBackgroundPaint::Transparent) => RuntimePaint::None,
            Some(ComputedBackgroundPaint::Opaque(color)) => {
                RuntimePaint::Opaque(OpaqueCssSrgb::new(color.red, color.green, color.blue))
            }
            None => return Err(StyleRenderError::MissingBackgroundColor(node_id)),
        };
        let paint_order =
            u32::try_from(boxes.len()).map_err(|_| StyleRenderError::PaintOrderOverflow)?;
        boxes.push(RuntimeRenderBox::new(
            node_id,
            frame_css_px,
            paint,
            paint_order,
        ));
    }

    let current_source = current.revision.source();
    let (generation, document_revision, render_tree_revision) = match current_source {
        LayoutSourceRevision::HostDocument {
            generation,
            document,
            render_tree,
        } => (generation, document, render_tree),
        LayoutSourceRevision::Tree(_) => {
            return Err(StyleRenderError::SnapshotMismatch {
                field: "CurrentLayoutSourceRevision",
            });
        }
    };
    let key = RuntimeRenderKey::new(
        generation,
        document_revision,
        render_tree_revision,
        current.revision.style(),
        current.revision.environment(),
    );
    RuntimeRenderSnapshot::new(key, viewport_css_px, boxes).map_err(Into::into)
}

struct RuntimeDocumentOrder {
    document_nodes: Vec<NodeId>,
    paint_nodes: Vec<NodeId>,
}

fn runtime_document_order(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<RuntimeDocumentOrder, StyleRenderError> {
    if document.parent(root) != Some(spinon_core::HostParent::Root)
        || !document
            .node(root)
            .is_some_and(|node| matches!(node.kind(), spinon_core::HostNodeKind::Element(_)))
    {
        return Err(StyleRenderError::InvalidRoot);
    }

    let mut document_nodes = Vec::new();
    let mut visible_nodes = std::collections::BTreeSet::new();
    let mut positioned_roots = Vec::new();
    let mut visited = std::collections::BTreeSet::new();
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        if !visited.insert(handle.id()) {
            return Err(StyleRenderError::DuplicateDocumentNode(handle.id()));
        }
        let node = document.node(handle).ok_or(StyleRenderError::InvalidRoot)?;
        if matches!(node.kind(), spinon_core::HostNodeKind::Text(_)) {
            if ancestor_hidden {
                continue;
            }
            return Err(StyleRenderError::UnsupportedTextNode(node.id()));
        }
        let style = styles
            .get(&node.id())
            .ok_or(StyleRenderError::MissingComputedStyle(node.id()))?;
        let hidden =
            ancestor_hidden || style.properties.get("display").map(String::as_str) == Some("none");
        document_nodes.push(node.id());
        let absolute = style.properties.get("position").map(String::as_str) == Some("absolute");
        if !hidden {
            visible_nodes.insert(node.id());
            if absolute {
                positioned_roots.push(handle);
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(
                children
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|child| (child, hidden)),
            );
        }
    }

    let mut paint_nodes = flow_paint_preorder(document, root, styles)?;
    for positioned_root in positioned_roots {
        append_positioned_subtree(document, positioned_root, styles, &mut paint_nodes)?;
    }
    let unique_paint_nodes = paint_nodes
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if unique_paint_nodes.len() != paint_nodes.len() || unique_paint_nodes != visible_nodes {
        return Err(StyleRenderError::InvalidRoot);
    }
    Ok(RuntimeDocumentOrder {
        document_nodes,
        paint_nodes,
    })
}

fn flow_paint_preorder(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<Vec<NodeId>, StyleRenderError> {
    let mut result = Vec::new();
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = document.node(handle).ok_or(StyleRenderError::InvalidRoot)?;
        let style = styles
            .get(&node.id())
            .ok_or(StyleRenderError::MissingComputedStyle(node.id()))?;
        let hidden =
            ancestor_hidden || style.properties.get("display").map(String::as_str) == Some("none");
        if hidden || style.properties.get("position").map(String::as_str) == Some("absolute") {
            continue;
        }
        result.push(node.id());
        let Some(children) = document.children(handle) else {
            continue;
        };
        let mut element_children = children
            .filter(|child| {
                document.node(*child).is_some_and(|child_node| {
                    matches!(child_node.kind(), spinon_core::HostNodeKind::Element(_))
                })
            })
            .collect::<Vec<_>>();
        if style.properties.get("display").map(String::as_str) == Some("flex") {
            element_children = flex_in_flow_children(document, element_children, styles)?;
        }
        pending.extend(
            element_children
                .into_iter()
                .rev()
                .map(|child| (child, hidden)),
        );
    }
    Ok(result)
}

fn append_positioned_subtree(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
    output: &mut Vec<NodeId>,
) -> Result<(), StyleRenderError> {
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = document.node(handle).ok_or(StyleRenderError::InvalidRoot)?;
        let style = styles
            .get(&node.id())
            .ok_or(StyleRenderError::MissingComputedStyle(node.id()))?;
        let hidden =
            ancestor_hidden || style.properties.get("display").map(String::as_str) == Some("none");
        if hidden {
            continue;
        }
        if handle != root
            && style.properties.get("position").map(String::as_str) == Some("absolute")
        {
            continue;
        }
        output.push(node.id());
        let Some(children) = document.children(handle) else {
            continue;
        };
        let element_children = children
            .filter(|child| {
                document.node(*child).is_some_and(|child_node| {
                    matches!(child_node.kind(), spinon_core::HostNodeKind::Element(_))
                })
            })
            .collect::<Vec<_>>();
        let ordered = if style.properties.get("display").map(String::as_str) == Some("flex") {
            flex_in_flow_children(document, element_children, styles)?
        } else {
            element_children
        };
        pending.extend(ordered.into_iter().rev().map(|child| (child, hidden)));
    }
    Ok(())
}

fn flex_in_flow_children(
    document: &HostDocumentSnapshot,
    children: Vec<HostNodeHandle>,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<Vec<HostNodeHandle>, StyleRenderError> {
    let children = children
        .into_iter()
        .filter(|child| {
            document.node(*child).is_some_and(|node| {
                styles
                    .get(&node.id())
                    .and_then(|style| style.properties.get("position"))
                    .is_none_or(|position| position != "absolute")
            })
        })
        .collect();
    order_flex_children(document, children, styles)
}

fn order_flex_children(
    document: &HostDocumentSnapshot,
    children: Vec<HostNodeHandle>,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<Vec<HostNodeHandle>, StyleRenderError> {
    let mut ordered =
        children
            .into_iter()
            .map(|child| {
                let node = document.node(child).ok_or(StyleRenderError::InvalidRoot)?;
                let style = styles
                    .get(&node.id())
                    .ok_or(StyleRenderError::MissingComputedStyle(node.id()))?;
                let order_value = style.properties.get("order").ok_or(
                    StyleRenderError::MissingComputedProperty {
                        node: node.id(),
                        property: "order",
                    },
                )?;
                let order = order_value.parse::<i32>().map_err(|_| {
                    StyleRenderError::UnsupportedComputedValue {
                        node: node.id(),
                        property: "order",
                        value: order_value.clone(),
                    }
                })?;
                Ok((order, child))
            })
            .collect::<Result<Vec<_>, StyleRenderError>>()?;
    // 같은 order 값에서는 HostDocument source order를 유지합니다.
    ordered.sort_by_key(|(order, _)| *order);
    Ok(ordered.into_iter().map(|(_, child)| child).collect())
}

fn validate_runtime_frame(node_id: NodeId, frame: LayoutFrame) -> Result<(), StyleRenderError> {
    CssRect::new(frame.x, frame.y, frame.width, frame.height)
        .map(|_| ())
        .map_err(|_| StyleRenderError::InvalidFrame(node_id))
}
