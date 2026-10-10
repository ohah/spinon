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
    let preorder = runtime_document_preorder(document, root, &styles_by_node)?;
    validate_style_node_set(&preorder, &styles_by_node)?;
    validate_layout_node_set(&preorder, &output.layout.frames)?;

    let viewport_css_px = CssSize::new(styles.viewport.width_css_px, styles.viewport.height_css_px)
        .map_err(|_| StyleRenderError::InvalidViewport)?;
    let mut boxes = Vec::with_capacity(preorder.len());
    for node_id in preorder {
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

fn runtime_document_preorder(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &std::collections::BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<Vec<NodeId>, StyleRenderError> {
    if document.parent(root) != Some(spinon_core::HostParent::Root)
        || !document
            .node(root)
            .is_some_and(|node| matches!(node.kind(), spinon_core::HostNodeKind::Element(_)))
    {
        return Err(StyleRenderError::InvalidRoot);
    }

    let mut result = Vec::new();
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
        result.push(node.id());
        if let Some(children) = document.children(handle) {
            let mut children = children.collect::<Vec<_>>();
            for child in &children {
                let child_node = document.node(*child).ok_or(StyleRenderError::InvalidRoot)?;
                if matches!(child_node.kind(), spinon_core::HostNodeKind::Text(_)) && !hidden {
                    return Err(StyleRenderError::UnsupportedTextNode(child_node.id()));
                }
            }
            if !hidden && style.properties.get("display").map(String::as_str) == Some("flex") {
                let mut ordered = children
                    .into_iter()
                    .filter(|child| {
                        document.node(*child).is_some_and(|node| {
                            matches!(node.kind(), spinon_core::HostNodeKind::Element(_))
                        })
                    })
                    .map(|child| {
                        let child_node =
                            document.node(child).ok_or(StyleRenderError::InvalidRoot)?;
                        let child_style = styles
                            .get(&child_node.id())
                            .ok_or(StyleRenderError::MissingComputedStyle(child_node.id()))?;
                        let order_value = child_style.properties.get("order").ok_or(
                            StyleRenderError::MissingComputedProperty {
                                node: child_node.id(),
                                property: "order",
                            },
                        )?;
                        let order = order_value.parse::<i32>().map_err(|_| {
                            StyleRenderError::UnsupportedComputedValue {
                                node: child_node.id(),
                                property: "order",
                                value: order_value.clone(),
                            }
                        })?;
                        Ok((order, child))
                    })
                    .collect::<Result<Vec<_>, StyleRenderError>>()?;
                // order 값이 같으면 원본 순서를 유지하도록 안정 정렬합니다.
                ordered.sort_by_key(|(order, _)| *order);
                children = ordered.into_iter().map(|(_, child)| child).collect();
            }
            pending.extend(
                children
                    .into_iter()
                    .rev()
                    .filter(|child| {
                        document.node(*child).is_some_and(|node| {
                            matches!(node.kind(), spinon_core::HostNodeKind::Element(_))
                        })
                    })
                    .map(|child| (child, hidden)),
            );
        }
    }
    Ok(result)
}

fn validate_runtime_frame(node_id: NodeId, frame: LayoutFrame) -> Result<(), StyleRenderError> {
    CssRect::new(frame.x, frame.y, frame.width, frame.height)
        .map(|_| ())
        .map_err(|_| StyleRenderError::InvalidFrame(node_id))
}
