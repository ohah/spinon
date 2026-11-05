use std::collections::BTreeMap;

use super::CalcLayoutTree;
use crate::compact_taffy_style::CompactTaffyStyle;
use crate::{GridPlacementMode, LayoutError, LayoutInput, LayoutLengthPercentage};
use spinon_core::NodeId;
use taffy::{DetailedGridInfo, DetailedLayoutInfo, LayoutGridContainer, NodeId as TaffyNodeId};

impl CalcLayoutTree {
    /// frame 수집 전에 유한하지 않은 Taffy track 위치를 거부한다.
    /// 비어 있는 explicit track은 화면의 모든 item frame이 유한해도 overflow할 수 있다.
    pub(crate) fn validate_root_html_intrinsic_gap_extents(
        &self,
        input: &LayoutInput,
        index: &BTreeMap<NodeId, usize>,
    ) -> Result<(), LayoutError> {
        for node in &self.nodes {
            let Some(spinon_node) = node.spinon_node else {
                continue;
            };
            let Some(container) = input.grid_containers.get(&spinon_node) else {
                continue;
            };
            if container.placement_mode != GridPlacementMode::RootHtmlIntrinsicTrackSizingV1 {
                continue;
            }
            let Some(&source_index) = index.get(&spinon_node) else {
                continue;
            };
            let source_node = &input.nodes()[source_index];
            let grid_info = match &node.detailed_layout_info {
                DetailedLayoutInfo::Grid(grid_info) => grid_info,
                _ => {
                    if matches!(
                        source_node.style.gap.column,
                        LayoutLengthPercentage::LengthPx(value) if value > 0.0
                    ) {
                        return Err(LayoutError::UnsupportedGridContainerStyle {
                            node: spinon_node,
                            property: "column-gap",
                        });
                    }
                    if matches!(
                        source_node.style.gap.row,
                        LayoutLengthPercentage::LengthPx(value) if value > 0.0
                    ) {
                        return Err(LayoutError::UnsupportedGridContainerStyle {
                            node: spinon_node,
                            property: "row-gap",
                        });
                    }
                    continue;
                }
            };

            for (positions, gap, property) in [
                (
                    &grid_info.columns.positions,
                    source_node.style.gap.column,
                    "column-gap",
                ),
                (
                    &grid_info.rows.positions,
                    source_node.style.gap.row,
                    "row-gap",
                ),
            ] {
                if !matches!(gap, LayoutLengthPercentage::LengthPx(value) if value > 0.0) {
                    continue;
                }
                if positions
                    .iter()
                    .any(|position| !position.start.is_finite() || !position.end.is_finite())
                {
                    return Err(LayoutError::UnsupportedGridContainerStyle {
                        node: spinon_node,
                        property,
                    });
                }
            }
        }
        Ok(())
    }
}

impl LayoutGridContainer for CalcLayoutTree {
    type GridContainerStyle<'a> = &'a CompactTaffyStyle;
    type GridItemStyle<'a> = &'a CompactTaffyStyle;

    fn get_grid_container_style(&self, node_id: TaffyNodeId) -> Self::GridContainerStyle<'_> {
        &self.node(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: TaffyNodeId) -> Self::GridItemStyle<'_> {
        &self.node(child_node_id).style
    }

    fn set_detailed_grid_info(
        &mut self,
        node_id: TaffyNodeId,
        detailed_grid_info: DetailedGridInfo<Self::CustomIdent>,
    ) {
        self.node_mut(node_id).detailed_layout_info =
            DetailedLayoutInfo::Grid(Box::new(detailed_grid_info));
    }
}
