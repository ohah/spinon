use super::CalcLayoutTree;
use crate::compact_taffy_style::CompactTaffyStyle;
use taffy::{DetailedGridInfo, DetailedLayoutInfo, LayoutGridContainer, NodeId as TaffyNodeId};

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
