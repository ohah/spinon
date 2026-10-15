use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::LayoutSourceRevision;
#[cfg(feature = "c13-3-gesture-fixture-test")]
use spinon_render::RuntimeTouchAction;
use spinon_render::{
    CssClip, CssRect, CssSize, OpaqueCssSrgb, RuntimeBorder, RuntimeBorderSide, RuntimePaint,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeScrollRenderMetadata,
};
use spinon_style::{
    ComputedBackgroundPaint, ComputedCssPosition, ComputedCssVisibility, ComputedCssZIndex,
    ComputedStyleProfile,
};
use spinon_style_to_layout::StyleLayoutOutput;

use crate::StyleRenderError;
use crate::adapter::{
    CurrentLayoutInputs, indexed_styles, validate_layout_node_set, validate_revisions,
    validate_style_node_set, validate_viewport,
};

#[cfg(test)]
mod deep_graph_tests;
mod overflow_clip;
mod reserve;
pub(crate) mod scroll;
#[cfg(test)]
mod tests;

mod ordering;
mod tree;
mod types;

use ordering::{
    add_context, add_node, flatten_contexts, flatten_hit_test_order, paint_auto_positioned_subtrees,
};
use overflow_clip::{runtime_clips_for_node, validate_computed_overflow_values};
use reserve::{FallibleReserve, SystemReserve};
use scroll::{
    RuntimeScrollGeometryContext, RuntimeScrollOffsetRequest, reconcile_scroll_state,
    scroll_ancestors_for_node,
};
use tree::{child_handles, ordered_children, root_display_is_flex, validate_runtime_frame};
use types::{PendingNode, StackingContext};

pub(super) fn build_runtime_render_snapshot(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    build_runtime_render_snapshot_with_reserve(document, root, output, current, &mut SystemReserve)
}

pub(super) fn build_runtime_render_snapshot_with_scroll_state(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
    previous: Option<&RuntimeRenderSnapshot>,
    requests: &[RuntimeScrollOffsetRequest],
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    build_runtime_render_snapshot_with_scroll_state_and_reserve(
        document,
        root,
        output,
        current,
        previous,
        requests,
        &mut SystemReserve,
    )
}

#[cfg(feature = "c13-1-overflow-clip-worker-test")]
#[doc(hidden)]
pub fn build_runtime_render_snapshot_with_overflow_clip_ancestry_allocation_failure(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    let mut reserve =
        reserve::FaultInjectingReserve::failing_at_stage_occurrence("overflow clip ancestry", 3);
    build_runtime_render_snapshot_with_reserve(document, root, output, current, &mut reserve)
}

fn build_runtime_render_snapshot_with_reserve<R: FallibleReserve>(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
    reserve: &mut R,
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    build_runtime_render_snapshot_with_scroll_state_and_reserve(
        document,
        root,
        output,
        current,
        None,
        &[],
        reserve,
    )
}

fn build_runtime_render_snapshot_with_scroll_state_and_reserve<R: FallibleReserve>(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &StyleLayoutOutput,
    current: CurrentLayoutInputs,
    previous_scroll_snapshot: Option<&RuntimeRenderSnapshot>,
    scroll_requests: &[RuntimeScrollOffsetRequest],
    reserve: &mut R,
) -> Result<RuntimeRenderSnapshot, StyleRenderError> {
    let styles = &output.computed_styles;
    if !is_runtime_stacking_profile(styles.profile) {
        return Err(StyleRenderError::UnsupportedRuntimeProfile);
    }
    if !styles.diagnostics.is_empty() {
        return Err(StyleRenderError::CascadeDiagnostics);
    }
    validate_viewport(styles.viewport)?;
    validate_viewport(current.viewport)?;
    validate_revisions(document, styles, output.layout.revision, current)?;
    if document.parent(root) != Some(HostParent::Root)
        || !document
            .node(root)
            .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(_)))
    {
        return Err(StyleRenderError::InvalidRoot);
    }

    let styles_by_node = indexed_styles(styles)?;
    validate_computed_stacking_values(&styles_by_node)?;
    if uses_overflow_clip_profile(styles.profile) {
        validate_computed_overflow_values(&styles_by_node)?;
    }
    let mut document_nodes = Vec::new();
    let mut parent_by_node = BTreeMap::new();
    let mut contexts = Vec::new();
    reserve.reserve(&mut contexts, 1, "root context")?;
    contexts.push(StackingContext {
        root: Some(root.id()),
        ..StackingContext::default()
    });
    let mut visited = BTreeSet::new();
    let mut next_tree_order = 0_u64;
    let mut visible_node_count = 0_usize;
    let mut pending = Vec::new();
    reserve.reserve(&mut pending, 1, "document traversal")?;
    pending.push(PendingNode {
        handle: root,
        parent_context: 0,
        parent_is_flex: false,
        ancestor_display_none: false,
    });

    while let Some(work) = pending.pop() {
        if !visited.insert(work.handle.id()) {
            return Err(StyleRenderError::DuplicateDocumentNode(work.handle.id()));
        }
        let node = document
            .node(work.handle)
            .ok_or(StyleRenderError::InvalidRoot)?;
        if matches!(node.kind(), HostNodeKind::Text(_)) {
            if work.ancestor_display_none {
                continue;
            }
            return Err(StyleRenderError::UnsupportedTextNode(node.id()));
        }
        let style = styles_by_node
            .get(&node.id())
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(node.id()))?;
        if node.id() != root.id() {
            let parent = match document.parent(work.handle) {
                Some(HostParent::Node(parent)) => parent.id(),
                Some(HostParent::Root) | None => return Err(StyleRenderError::InvalidRoot),
            };
            parent_by_node.insert(node.id(), parent);
        }
        reserve.reserve(&mut document_nodes, 1, "document nodes")?;
        document_nodes.push(node.id());
        let hidden = work.ancestor_display_none
            || style.properties.get("display").map(String::as_str) == Some("none");
        let display =
            style
                .properties
                .get("display")
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: node.id(),
                    property: "display",
                })?;
        if !matches!(display.as_str(), "block" | "flow-root" | "flex" | "none") {
            return Err(StyleRenderError::UnsupportedComputedValue {
                node: node.id(),
                property: "display",
                value: display.clone(),
            });
        }

        let mut child_context = work.parent_context;
        if !hidden {
            visible_node_count = visible_node_count
                .checked_add(1)
                .ok_or(StyleRenderError::PaintOrderOverflow)?;
            let tree_order = next_tree_order;
            next_tree_order = next_tree_order
                .checked_add(1)
                .ok_or(StyleRenderError::PaintOrderOverflow)?;
            let position = style.layout_position;
            if position == ComputedCssPosition::Sticky {
                return Err(StyleRenderError::UnsupportedComputedValue {
                    node: node.id(),
                    property: "position",
                    value: "sticky".to_owned(),
                });
            }
            let is_flex_item = work.parent_is_flex
                && !matches!(
                    position,
                    ComputedCssPosition::Absolute | ComputedCssPosition::Fixed
                );
            if node.id() != root.id() {
                if let Some(stack_level) =
                    stacking_context_level(position, style.stacking_z_index, is_flex_item)?
                {
                    reserve.reserve(&mut contexts, 1, "stacking contexts")?;
                    let context_id = contexts.len();
                    contexts.push(StackingContext {
                        root: Some(node.id()),
                        ..StackingContext::default()
                    });
                    add_context(
                        &mut contexts[work.parent_context],
                        context_id,
                        stack_level,
                        tree_order,
                        reserve,
                    )?;
                    child_context = context_id;
                } else {
                    add_node(
                        &mut contexts[work.parent_context],
                        node.id(),
                        tree_order,
                        position,
                        is_flex_item,
                        reserve,
                    )?;
                }
            } else {
                contexts[0].root = Some(node.id());
            }
        }

        if document.children(work.handle).is_some() {
            let children = child_handles(document, work.handle, reserve)?;
            let ordered =
                ordered_children(document, node.id(), children, &styles_by_node, reserve)?;
            let parent_is_flex = !hidden && root_display_is_flex(node.id(), &styles_by_node)?;
            reserve.reserve(&mut pending, ordered.len(), "document traversal")?;
            pending.extend(ordered.into_iter().rev().map(|handle| PendingNode {
                handle,
                parent_context: child_context,
                parent_is_flex,
                ancestor_display_none: hidden,
            }));
        }
    }

    validate_style_node_set(&document_nodes, &styles_by_node)?;
    validate_layout_node_set(&document_nodes, &output.layout.frames)?;
    let mut paint_nodes = flatten_contexts(
        document_nodes.len(),
        visible_node_count,
        &mut contexts,
        output,
        reserve,
    )?;
    if uses_overflow_clip_profile(styles.profile) {
        paint_auto_positioned_subtrees(
            &document_nodes,
            &parent_by_node,
            &mut paint_nodes,
            &styles_by_node,
            reserve,
        )?;
    }
    let hit_test_order_by_node = if uses_overflow_clip_profile(styles.profile) {
        let mut hit_render_nodes = Vec::new();
        reserve.reserve(
            &mut hit_render_nodes,
            paint_nodes.len(),
            "hit-test render nodes",
        )?;
        for node_id in &paint_nodes {
            let frame = output
                .layout
                .frames
                .get(node_id)
                .copied()
                .ok_or(StyleRenderError::MissingLayoutFrame(*node_id))?;
            validate_runtime_frame(*node_id, frame)?;
            if frame.width != 0.0 && frame.height != 0.0 {
                hit_render_nodes.push(*node_id);
            }
        }
        Some(flatten_hit_test_order(
            &document_nodes,
            &parent_by_node,
            &hit_render_nodes,
            reserve,
        )?)
    } else {
        None
    };
    let viewport_css_px = CssSize::new(styles.viewport.width_css_px, styles.viewport.height_css_px)
        .map_err(|_| StyleRenderError::InvalidViewport)?;
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
    let scroll_context = if uses_overflow_clip_profile(styles.profile) {
        Some(RuntimeScrollGeometryContext {
            document,
            root,
            document_nodes: &document_nodes,
            parent_by_node: &parent_by_node,
            styles: &styles_by_node,
            output,
            viewport: styles.viewport,
        })
    } else {
        None
    };
    let scroll_state = reconcile_scroll_state(
        scroll_context.as_ref(),
        previous_scroll_snapshot,
        scroll_requests,
        generation,
        reserve,
    )?;
    let scroll_containers = scroll_state.containers;
    let scroll_state_revision = scroll_state.revision;
    let scroll_offsets = scroll_state.offsets;
    let key = RuntimeRenderKey::new(
        generation,
        document_revision,
        render_tree_revision,
        current.revision.style(),
        current.revision.environment(),
    )
    .with_intrinsic_size_revision(current.revision.intrinsic_size())
    .with_scroll_state_revision(scroll_state_revision);
    let mut boxes = Vec::new();
    reserve.reserve(&mut boxes, visible_node_count, "render boxes")?;
    let mut scroll_metadata = Vec::new();
    reserve.reserve(
        &mut scroll_metadata,
        visible_node_count,
        "scroll render metadata",
    )?;
    for node_id in paint_nodes {
        let frame = output
            .layout
            .frames
            .get(&node_id)
            .copied()
            .ok_or(StyleRenderError::MissingLayoutFrame(node_id))?;
        validate_runtime_frame(node_id, frame)?;
        if frame.width == 0.0 || frame.height == 0.0 {
            continue;
        }
        let style = styles_by_node
            .get(&node_id)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(node_id))?;
        let frame_css_px = CssRect::new(frame.x, frame.y, frame.width, frame.height)
            .map_err(|_| StyleRenderError::InvalidFrame(node_id))?;
        let paint = match style.background_paint {
            Some(ComputedBackgroundPaint::Transparent) => RuntimePaint::None,
            Some(ComputedBackgroundPaint::Opaque(color)) => {
                RuntimePaint::Opaque(OpaqueCssSrgb::new(color.red, color.green, color.blue))
            }
            None => return Err(StyleRenderError::MissingBackgroundColor(node_id)),
        };
        let border = match style.layout_border_paint {
            Some(border_paint) => RuntimeBorder::new(
                RuntimeBorderSide::new(
                    style.layout_border.top,
                    runtime_border_paint(border_paint.top),
                )?,
                RuntimeBorderSide::new(
                    style.layout_border.right,
                    runtime_border_paint(border_paint.right),
                )?,
                RuntimeBorderSide::new(
                    style.layout_border.bottom,
                    runtime_border_paint(border_paint.bottom),
                )?,
                RuntimeBorderSide::new(
                    style.layout_border.left,
                    runtime_border_paint(border_paint.left),
                )?,
            ),
            None => RuntimeBorder::none(),
        };
        let paint_order =
            u32::try_from(boxes.len()).map_err(|_| StyleRenderError::PaintOrderOverflow)?;
        let hit_test_order = match &hit_test_order_by_node {
            Some(order_by_node) => order_by_node
                .get(&node_id)
                .copied()
                .ok_or(StyleRenderError::InvalidRoot)?,
            None => paint_order,
        };
        let (paint_clip, hit_test_clip, clip_layers) = if uses_overflow_clip_profile(styles.profile)
        {
            runtime_clips_for_node(
                node_id,
                &parent_by_node,
                &styles_by_node,
                output,
                &scroll_offsets,
                reserve,
            )?
        } else {
            (
                CssClip::unconstrained(),
                CssClip::unconstrained(),
                Vec::new(),
            )
        };
        let scroll_ancestors =
            scroll_ancestors_for_node(node_id, style, &parent_by_node, &scroll_offsets, reserve)?;
        boxes.push(
            RuntimeRenderBox::new(node_id, frame_css_px, paint, paint_order)
                .with_border(border)
                .with_hit_test_order(hit_test_order)
                .with_clip(paint_clip)
                .with_hit_test_clip(hit_test_clip),
        );
        let metadata =
            RuntimeScrollRenderMetadata::new(frame_css_px, scroll_ancestors, clip_layers);
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        let metadata = if styles.profile == ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 {
            metadata.with_touch_action_chain(touch_action_chain_for_node(
                node_id,
                &parent_by_node,
                &styles_by_node,
                &scroll_offsets,
                reserve,
            )?)
        } else {
            metadata
        };
        scroll_metadata.push(metadata);
    }

    RuntimeRenderSnapshot::new_with_scroll_metadata(
        key,
        viewport_css_px,
        boxes,
        scroll_containers,
        scroll_metadata,
    )
    .map_err(Into::into)
}

fn uses_overflow_clip_profile(profile: ComputedStyleProfile) -> bool {
    match profile {
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1 => true,
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => true,
        _ => false,
    }
}

#[cfg(feature = "c13-3-gesture-fixture-test")]
fn touch_action_chain_for_node<R: FallibleReserve>(
    node: NodeId,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
    scroll_owners: &BTreeMap<NodeId, (f32, f32)>,
    reserve: &mut R,
) -> Result<Vec<RuntimeTouchAction>, StyleRenderError> {
    let mut actions = Vec::new();
    let mut cursor = Some(node);
    let mut visited = 0_usize;
    while let Some(current) = cursor {
        visited = visited
            .checked_add(1)
            .ok_or(StyleRenderError::InvalidRoot)?;
        if visited > parent_by_node.len().saturating_add(1) {
            return Err(StyleRenderError::InvalidRoot);
        }
        reserve.reserve(&mut actions, 1, "touch-action style ancestry")?;
        let style = styles
            .get(&current)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(current))?;
        let action = style
            .input_touch_action
            .ok_or(StyleRenderError::MissingComputedProperty {
                node: current,
                property: "computed touch-action",
            })?;
        actions.push(match action {
            spinon_style::ComputedCssTouchAction::Auto => RuntimeTouchAction::Auto,
            spinon_style::ComputedCssTouchAction::None => RuntimeTouchAction::None,
            spinon_style::ComputedCssTouchAction::PanX => RuntimeTouchAction::PanX,
            spinon_style::ComputedCssTouchAction::PanY => RuntimeTouchAction::PanY,
            spinon_style::ComputedCssTouchAction::Unsupported => RuntimeTouchAction::Unsupported,
        });
        if scroll_owners.contains_key(&current) {
            break;
        }
        cursor = parent_by_node.get(&current).copied();
    }
    Ok(actions)
}

fn is_runtime_stacking_profile(profile: ComputedStyleProfile) -> bool {
    match profile {
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
        | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1 => true,
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => true,
        _ => false,
    }
}

fn runtime_border_paint(paint: ComputedBackgroundPaint) -> RuntimePaint {
    match paint {
        ComputedBackgroundPaint::Transparent => RuntimePaint::None,
        ComputedBackgroundPaint::Opaque(color) => {
            RuntimePaint::Opaque(OpaqueCssSrgb::new(color.red, color.green, color.blue))
        }
    }
}

fn validate_computed_stacking_values(
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<(), StyleRenderError> {
    for (node, style) in styles {
        match style.stacking_z_index {
            Some(_) => {}
            None => {
                return Err(StyleRenderError::MissingComputedProperty {
                    node: *node,
                    property: "z-index",
                });
            }
        }
        match style.stacking_visibility {
            Some(ComputedCssVisibility::Visible) => {}
            Some(ComputedCssVisibility::Hidden) => {
                return Err(StyleRenderError::UnsupportedComputedValue {
                    node: *node,
                    property: "visibility",
                    value: "hidden".to_owned(),
                });
            }
            Some(ComputedCssVisibility::Collapse) => {
                return Err(StyleRenderError::UnsupportedComputedValue {
                    node: *node,
                    property: "visibility",
                    value: "collapse".to_owned(),
                });
            }
            None => {
                return Err(StyleRenderError::MissingComputedProperty {
                    node: *node,
                    property: "visibility",
                });
            }
        }
    }
    Ok(())
}

fn stacking_context_level(
    position: ComputedCssPosition,
    z_index: Option<ComputedCssZIndex>,
    is_flex_item: bool,
) -> Result<Option<i32>, StyleRenderError> {
    let z_index = z_index.ok_or(StyleRenderError::UnsupportedRuntimeProfile)?;
    let integer = match z_index {
        ComputedCssZIndex::Auto => None,
        ComputedCssZIndex::Integer(value) => Some(value),
    };
    match position {
        ComputedCssPosition::Fixed => Ok(Some(integer.unwrap_or(0))),
        ComputedCssPosition::Relative | ComputedCssPosition::Absolute => Ok(integer),
        ComputedCssPosition::Static if is_flex_item => Ok(integer),
        ComputedCssPosition::Static | ComputedCssPosition::Sticky => Ok(None),
    }
}
