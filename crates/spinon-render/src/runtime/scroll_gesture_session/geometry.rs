use std::collections::BTreeSet;

use spinon_core::{
    DocumentGeneration, DocumentRevision, EnvironmentRevision, IntrinsicSizeRevision, NodeId,
    RenderTreeRevision,
};

use super::super::{CssClip, CssRect, CssSize, RuntimeRenderSnapshot};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct RuntimeScrollGestureGeometryKey {
    pub(super) generation: DocumentGeneration,
    pub(super) document_revision: DocumentRevision,
    pub(super) render_tree_revision: RenderTreeRevision,
    pub(super) environment_revision: EnvironmentRevision,
    intrinsic_size_revision: IntrinsicSizeRevision,
    surface_generation: u64,
    viewport: CssSize,
    target: NodeId,
    target_frame: CssRect,
    target_hit_clip: CssClip,
    target_hit_test_order: u32,
    pub(super) owner_chain: Vec<NodeId>,
    owners: Vec<OwnerGeometry>,
    clips: Vec<ClipGeometry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct OwnerGeometry {
    owner: NodeId,
    parent: Option<NodeId>,
    scrollport: CssRect,
    max_x_css_px: f32,
    max_y_css_px: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ClipGeometry {
    paint_clip: CssClip,
    hit_test_clip: CssClip,
    scroll_ancestors: Vec<NodeId>,
}

impl RuntimeRenderSnapshot {
    pub(super) fn gesture_geometry_key(
        &self,
        target: NodeId,
        surface_generation: u64,
    ) -> Option<RuntimeScrollGestureGeometryKey> {
        let index = self
            .base_boxes
            .iter()
            .position(|render_box| render_box.node_id() == target)?;
        let target_box = self.base_boxes[index];
        let metadata = self.scroll_metadata.get(index)?;
        let mut owner_chain = Vec::new();
        if self
            .scroll_containers
            .iter()
            .any(|container| container.owner() == target)
        {
            owner_chain.push(target);
        }
        owner_chain.extend_from_slice(metadata.scroll_ancestors());
        let mut seen = BTreeSet::new();
        if owner_chain.iter().any(|owner| !seen.insert(*owner)) {
            return None;
        }
        let owners = owner_chain
            .iter()
            .map(|owner| {
                let container = self
                    .scroll_containers
                    .iter()
                    .find(|container| container.owner() == *owner)?;
                Some(OwnerGeometry {
                    owner: *owner,
                    parent: container.parent(),
                    scrollport: container.scrollport_css_px(),
                    max_x_css_px: container.max_offset_x_css_px(),
                    max_y_css_px: container.max_offset_y_css_px(),
                })
            })
            .collect::<Option<Vec<_>>>()?;
        let clips = metadata
            .clip_layers()
            .iter()
            .map(|layer| ClipGeometry {
                paint_clip: layer.paint_clip_css_px(),
                hit_test_clip: layer.hit_test_clip_css_px(),
                scroll_ancestors: layer.scroll_ancestors().to_vec(),
            })
            .collect();
        let key = self.key();
        Some(RuntimeScrollGestureGeometryKey {
            generation: key.generation(),
            document_revision: key.document_revision(),
            render_tree_revision: key.render_tree_revision(),
            environment_revision: key.environment_revision(),
            intrinsic_size_revision: key.intrinsic_size_revision(),
            surface_generation,
            viewport: self.viewport_css_px,
            target,
            target_frame: metadata.base_frame_css_px(),
            target_hit_clip: target_box.hit_test_clip_css_px(),
            target_hit_test_order: target_box.hit_test_order(),
            owner_chain,
            owners,
            clips,
        })
    }
}
