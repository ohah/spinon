use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_text_layout::{FontRegistryRevision, TextIntrinsicInlineSizes};
use taffy::{AvailableSpace, NodeId as TaffyNodeId, Size};

use crate::{LayoutError, LayoutInput, LayoutTextFlowSegment, LayoutTextSnapshot};

use super::CalcLayoutTree;

pub(crate) type FinalTextLayouts = (
    BTreeMap<NodeId, LayoutTextSnapshot>,
    BTreeMap<NodeId, Vec<LayoutTextFlowSegment>>,
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextMeasureResult {
    pub(super) size: Size<f32>,
    pub(super) font_registry_revision: FontRegistryRevision,
}

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
    ) -> Result<TextMeasureResult, LayoutError> {
        let Some(paragraph) = self.text_paragraphs.get(&node_id).cloned() else {
            return Ok(TextMeasureResult {
                size: Size::ZERO,
                font_registry_revision: FontRegistryRevision::default(),
            });
        };
        let engine = self
            .text_layout_engine
            .as_ref()
            .ok_or(LayoutError::MissingTextLayoutEngine(paragraph.node))?;
        match available_width {
            AvailableSpace::Definite(width) => {
                let width = normalize_zero(width);
                let snapshot = engine
                    .borrow_mut()
                    .layout_paragraph(&paragraph, width)
                    .map_err(LayoutError::TextLayout)?;
                self.text_layouts
                    .borrow_mut()
                    .entry(node_id)
                    .or_default()
                    .insert(width.to_bits(), snapshot.clone());
                Ok(TextMeasureResult {
                    size: Size {
                        width: snapshot.width,
                        height: snapshot.height,
                    },
                    font_registry_revision: snapshot.font_registry_revision,
                })
            }
            AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                let sizes = self.text_intrinsic_inline_sizes(node_id, paragraph.node)?;
                Ok(TextMeasureResult {
                    size: match available_width {
                        AvailableSpace::MinContent => Size {
                            width: sizes.min_content(),
                            height: sizes.min_content_height(),
                        },
                        AvailableSpace::MaxContent => Size {
                            width: sizes.max_content(),
                            height: sizes.max_content_height(),
                        },
                        AvailableSpace::Definite(_) => {
                            unreachable!("matched intrinsic width")
                        }
                    },
                    font_registry_revision: sizes.font_registry_revision(),
                })
            }
        }
    }

    pub(super) fn text_intrinsic_inline_sizes(
        &self,
        node_id: TaffyNodeId,
        owner: NodeId,
    ) -> Result<TextIntrinsicInlineSizes, LayoutError> {
        if let Some(sizes) = self.text_intrinsic_layouts.borrow().get(&node_id).cloned() {
            return Ok(sizes);
        }
        let Some(paragraph) = self.text_paragraphs.get(&node_id) else {
            return Err(LayoutError::UnsupportedTextParagraph {
                node: owner,
                reason: "Grid text intrinsic input에 대응하는 paragraph가 없습니다",
            });
        };
        let engine = self
            .text_layout_engine
            .as_ref()
            .ok_or(LayoutError::MissingTextLayoutEngine(paragraph.node))?;
        let sizes = engine
            .borrow_mut()
            .intrinsic_inline_sizes(paragraph)
            .map_err(LayoutError::TextLayout)?;
        self.text_intrinsic_layouts
            .borrow_mut()
            .insert(node_id, sizes.clone());
        Ok(sizes)
    }
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::{BTreeMap, HashMap},
    };

    use spinon_core::NodeId;
    use spinon_text_layout::{
        ParagraphInput, ParagraphStyle, TextLayoutEngine, TextSource, WhiteSpaceMode, WordBreakMode,
    };
    use taffy::{AvailableSpace, NodeId as TaffyNodeId};

    use super::super::intrinsic_measurements::GridItemIntrinsicMetadata;
    use super::super::{CalcLayoutTree, Node};

    const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

    #[test]
    fn grid_text_source_collection_needs_no_callback_and_keeps_definite_snapshot_separate() {
        let spinon_node = NodeId::new(10).unwrap();
        let grid_container = NodeId::new(12).unwrap();
        let mut text_engine = TextLayoutEngine::new_without_system_fonts();
        text_engine.register_font("Ahem", AHEM).unwrap();
        let paragraph = ParagraphInput {
            node: spinon_node,
            sources: vec![TextSource {
                node: NodeId::new(11).unwrap(),
                inline_ancestors: Vec::new(),
                text: "XX XXXXXXX".to_owned(),
            }],
            style: ParagraphStyle {
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                ..ParagraphStyle::default()
            },
            inline_styles: BTreeMap::new(),
            inline_run_styles: BTreeMap::new(),
        };
        let taffy_node = TaffyNodeId::from(0_usize);
        let item_style = crate::LayoutStyle::default();
        let compact_style = crate::taffy_style::to_taffy_style(
            spinon_node,
            item_style,
            false,
            crate::LayoutPositioning::default(),
            false,
            &BTreeMap::new(),
            crate::taffy_style::GridTaffyStyleInput {
                container: None,
                expanded_rows: None,
                expanded_columns: None,
                item: None,
            },
        )
        .unwrap();
        let tree = CalcLayoutTree {
            nodes: vec![Node::new(
                Vec::new(),
                compact_style,
                None,
                0,
                Some(spinon_node),
            )],
            engine_ids: BTreeMap::from([(spinon_node, taffy_node)]),
            layout_children: BTreeMap::new(),
            viewport_children: Vec::new(),
            viewport_root: None,
            calc_values: HashMap::new(),
            calc_error: RefCell::new(None),
            text_layout_engine: Some(RefCell::new(text_engine)),
            text_paragraphs: HashMap::from([(taffy_node, paragraph)]),
            text_flow_nodes: BTreeMap::new(),
            text_layouts: RefCell::new(HashMap::new()),
            text_intrinsic_layouts: RefCell::new(HashMap::new()),
            grid_item_intrinsic_metadata: BTreeMap::from([(
                spinon_node,
                GridItemIntrinsicMetadata {
                    container: grid_container,
                    source_order: 2,
                    is_leaf: true,
                    style: item_style,
                },
            )]),
            record_intrinsic_measurements: true,
            intrinsic_measurements: RefCell::new(Vec::new()),
        };

        let inputs = tree
            .collect_grid_item_intrinsic_inputs(crate::LayoutInputRevision::default())
            .unwrap();
        assert!(tree.intrinsic_measurements.borrow().is_empty());
        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].grid_container, grid_container);
        assert_eq!(inputs[0].node, spinon_node);
        assert_eq!(inputs[0].source_order, 2);
        let crate::LayoutGridItemIntrinsicSource::Text { sizes } = &inputs[0].source else {
            panic!("text grid item must retain its intrinsic source");
        };
        assert_eq!(
            (sizes.min_content(), sizes.min_content_height()),
            (70.0, 20.0)
        );
        assert_eq!(
            (sizes.max_content(), sizes.max_content_height()),
            (100.0, 10.0)
        );
        assert_eq!(sizes.font_registry_revision().0, 1);

        let min_content = tree
            .measure_text(taffy_node, AvailableSpace::MinContent)
            .unwrap();
        let max_content = tree
            .measure_text(taffy_node, AvailableSpace::MaxContent)
            .unwrap();
        assert_eq!(
            (min_content.size.width, min_content.size.height),
            (70.0, 20.0)
        );
        assert_eq!(
            (max_content.size.width, max_content.size.height),
            (100.0, 10.0)
        );
        assert_eq!(
            min_content.font_registry_revision,
            max_content.font_registry_revision
        );
        assert!(tree.text_layouts.borrow().is_empty());
        assert_eq!(tree.text_intrinsic_layouts.borrow().len(), 1);

        let definite = tree
            .measure_text(taffy_node, AvailableSpace::Definite(30.0))
            .unwrap();
        assert_eq!(definite.size.width, 70.0);
        assert_eq!(
            definite.font_registry_revision,
            min_content.font_registry_revision
        );
        assert!(tree.text_layouts.borrow()[&taffy_node].contains_key(&30.0_f32.to_bits()));
        assert_eq!(tree.text_intrinsic_layouts.borrow().len(), 1);
    }
}
