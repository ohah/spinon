use std::collections::BTreeMap;

use spinon_core::NodeId;
use taffy::{AvailableSpace, NodeId as TaffyNodeId, Size};

use crate::{LayoutError, LayoutInput, LayoutTextFlowSegment, LayoutTextSnapshot};

use super::CalcLayoutTree;

pub(crate) type FinalTextLayouts = (
    BTreeMap<NodeId, LayoutTextSnapshot>,
    BTreeMap<NodeId, Vec<LayoutTextFlowSegment>>,
);

impl CalcLayoutTree {
    pub(crate) fn final_text_layouts(
        &self,
        input: &LayoutInput,
    ) -> Result<FinalTextLayouts, LayoutError> {
        let mut result = BTreeMap::new();
        let mut flow_result = BTreeMap::new();
        let text_layouts = self.text_layouts.borrow();
        for (&engine_id, paragraph) in &self.text_paragraphs {
            if self
                .text_flow_nodes
                .values()
                .any(|segments| segments.iter().any(|(_, id)| *id == engine_id))
            {
                continue;
            }
            let layout = self
                .layout(engine_id)
                .ok_or(LayoutError::MissingComputedLayout(paragraph.node))?;
            let content_width = layout.size.width
                - layout.border.left
                - layout.border.right
                - layout.padding.left
                - layout.padding.right;
            let width_key = normalize_zero(content_width).to_bits();
            let snapshot = text_layouts
                .get(&engine_id)
                .and_then(|by_width| by_width.get(&width_key))
                .ok_or(LayoutError::TextLayoutWidthMismatch(paragraph.node))?;
            if snapshot.available_width.to_bits() != width_key {
                return Err(LayoutError::TextLayoutWidthMismatch(paragraph.node));
            }
            let node_style = input.nodes().iter().find(|node| node.id == paragraph.node);
            let content_height = layout.size.height
                - layout.border.top
                - layout.border.bottom
                - layout.padding.top
                - layout.padding.bottom;
            if node_style.is_some_and(|node| node.style.height == crate::LayoutDimension::Auto)
                && (content_height - snapshot.height).abs() > 0.05
            {
                return Err(LayoutError::TextLayoutHeightMismatch(paragraph.node));
            }
            result.insert(
                paragraph.node,
                LayoutTextSnapshot {
                    revision: input.revision(),
                    input: paragraph.clone(),
                    layout: snapshot.clone(),
                },
            );
        }
        for (&owner, segments) in &self.text_flow_nodes {
            let mut owner_segments = Vec::with_capacity(segments.len());
            for &(item_index, engine_id) in segments {
                let paragraph = &self.text_paragraphs[&engine_id];
                let layout = self
                    .layout(engine_id)
                    .ok_or(LayoutError::MissingComputedLayout(paragraph.node))?;
                let content_width = layout.size.width
                    - layout.border.left
                    - layout.border.right
                    - layout.padding.left
                    - layout.padding.right;
                let width_key = normalize_zero(content_width).to_bits();
                let snapshot = text_layouts
                    .get(&engine_id)
                    .and_then(|by_width| by_width.get(&width_key))
                    .ok_or(LayoutError::TextLayoutWidthMismatch(owner))?;
                if snapshot.available_width.to_bits() != width_key {
                    return Err(LayoutError::TextLayoutWidthMismatch(owner));
                }
                let content_height = layout.size.height
                    - layout.border.top
                    - layout.border.bottom
                    - layout.padding.top
                    - layout.padding.bottom;
                if (content_height - snapshot.height).abs() > 0.05 {
                    return Err(LayoutError::TextLayoutHeightMismatch(owner));
                }
                owner_segments.push(LayoutTextFlowSegment {
                    item_index,
                    frame: crate::LayoutFrame {
                        x: layout.location.x,
                        y: layout.location.y,
                        width: layout.size.width,
                        height: layout.size.height,
                    },
                    snapshot: LayoutTextSnapshot {
                        revision: input.revision(),
                        input: paragraph.clone(),
                        layout: snapshot.clone(),
                    },
                });
            }
            flow_result.insert(owner, owner_segments);
        }
        Ok((result, flow_result))
    }

    pub(super) fn measure_text(
        &self,
        node_id: TaffyNodeId,
        available_width: AvailableSpace,
    ) -> Result<Size<f32>, LayoutError> {
        let Some(paragraph) = self.text_paragraphs.get(&node_id).cloned() else {
            return Ok(Size::ZERO);
        };
        let AvailableSpace::Definite(width) = available_width else {
            return Err(LayoutError::IndefiniteTextAvailableWidth(paragraph.node));
        };
        let width = normalize_zero(width);
        let engine = self
            .text_layout_engine
            .as_ref()
            .ok_or(LayoutError::MissingTextLayoutEngine(paragraph.node))?;
        let snapshot = engine
            .borrow_mut()
            .layout_paragraph(&paragraph, width)
            .map_err(LayoutError::TextLayout)?;
        self.text_layouts
            .borrow_mut()
            .entry(node_id)
            .or_default()
            .insert(width.to_bits(), snapshot.clone());
        Ok(Size {
            width: snapshot.width,
            height: snapshot.height,
        })
    }
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}
