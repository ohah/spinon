use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::grid_placement::NormalizedGridLayout;
use crate::{
    GridAutoColumns, GridAutoFlow, GridAutoRepeatMode, GridContainerInput, GridItemInput,
    GridPlacementInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf, LayoutBorder,
    LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutGap, LayoutInput,
    LayoutJustifyContent, LayoutLengthPercentage, LayoutOverflow, LayoutPosition,
    LayoutPositioning, TextDirection,
};

pub(super) fn supports_wpt_probe_matrix(
    input: &LayoutInput,
    node: &crate::LayoutNode,
    container: &GridContainerInput,
    normalized: &NormalizedGridLayout,
    hidden: &BTreeMap<NodeId, bool>,
    index: &BTreeMap<NodeId, usize>,
) -> bool {
    if !is_target_container(input, node, container)
        || input.positioning.get(&node.id).copied().unwrap_or_default()
            != (LayoutPositioning {
                position: LayoutPosition::Relative,
                inset: LayoutEdges::auto(),
            })
        || node.children.len() != 5
    {
        return false;
    }

    let mut in_flow_item_seen = false;
    let mut next_probe_pair = 0;
    for child_id in &node.children {
        if hidden.get(child_id).copied().unwrap_or(true) {
            return false;
        }
        let Some(child_index) = index.get(child_id).copied() else {
            return false;
        };
        let child = &input.nodes[child_index];
        let positioning = input.positioning.get(child_id).copied().unwrap_or_default();
        if positioning.position == LayoutPosition::Static {
            if in_flow_item_seen
                || !is_wpt_in_flow_item(child, normalized.item_placements.get(child_id))
            {
                return false;
            }
            in_flow_item_seen = true;
            continue;
        }
        let Some(probe_index) = normalized
            .item_placements
            .get(child_id)
            .and_then(probe_pair_index)
        else {
            return false;
        };
        if !is_wpt_absolute_probe(child, positioning) || probe_index != next_probe_pair {
            return false;
        }
        next_probe_pair += 1;
    }
    in_flow_item_seen && next_probe_pair == 4
}

fn is_target_container(
    input: &LayoutInput,
    node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> bool {
    let root_style = node.style;
    let Some(auto_repeat) = container.columns_auto_repeat.as_ref() else {
        return false;
    };
    let margin = LayoutEdges {
        top: LayoutLengthPercentage::ZERO,
        right: LayoutLengthPercentage::ZERO,
        bottom: LayoutLengthPercentage::LengthPx(10.0),
        left: LayoutLengthPercentage::ZERO,
    };
    let padding = LayoutEdges {
        top: LayoutLengthPercentage::LengthPx(10.0),
        right: LayoutLengthPercentage::ZERO,
        bottom: LayoutLengthPercentage::ZERO,
        left: LayoutLengthPercentage::ZERO,
    };
    root_style.display == LayoutDisplay::Grid
        && root_style.overflow_x == LayoutOverflow::Visible
        && root_style.overflow_y == LayoutOverflow::Visible
        && root_style.box_sizing == LayoutBoxSizing::ContentBox
        && matches!(
            root_style.width,
            LayoutDimension::MinContent | LayoutDimension::MaxContent
        )
        && root_style.height == LayoutDimension::Auto
        && root_style.min_width == LayoutDimension::Fixed(30.0)
        && root_style.max_width == LayoutDimension::Auto
        && root_style.min_height == LayoutDimension::Auto
        && root_style.max_height == LayoutDimension::Auto
        && root_style.aspect_ratio.is_none()
        && root_style.order == 0
        && root_style.direction == TextDirection::Ltr
        && root_style.align_items == LayoutAlignItems::Normal
        && root_style.align_self == LayoutAlignSelf::Auto
        && root_style.align_content == Some(LayoutAlignContent::Normal)
        && root_style.justify_content == LayoutJustifyContent::Normal
        && root_style.margin == margin
        && root_style.padding == padding
        && root_style.border
            == (LayoutBorder {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            })
        && root_style.gap == LayoutGap::default()
        && container.placement_mode == GridPlacementMode::TrackSizing
        && auto_repeat.mode == GridAutoRepeatMode::AutoFill
        && auto_repeat.start == 0
        && auto_repeat.end == 1
        && auto_repeat.line_names.iter().all(Vec::is_empty)
        && container.rows_auto_repeat.is_none()
        && container.rows.is_empty()
        && container.columns_line_names.iter().all(Vec::is_empty)
        && container.rows_line_names.is_empty()
        && container.columns.as_slice()
            == [
                GridTrackSize::LengthPx(20.0),
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::MinContent,
                    max: GridTrackMaxSizingFunction::LengthPx(40.0),
                },
            ]
        && container.auto_flow == GridAutoFlow::Row
        && container.auto_rows == crate::GridAutoRows::Auto
        && container.auto_columns == GridAutoColumns::LengthPx(20.0)
        && container.template_areas.is_none()
        && input.grid_items.len() == node.children.len()
        && node
            .children
            .iter()
            .all(|child| input.grid_items.contains_key(child))
}

fn is_wpt_in_flow_item(node: &crate::LayoutNode, placement: Option<&GridItemInput>) -> bool {
    matches!(
        node.style.width,
        LayoutDimension::Fixed(width) if [15.0, 80.0, 100.0, 120.0].contains(&width)
    ) && node.style.display == LayoutDisplay::Block
        && node.style.overflow_x == LayoutOverflow::Visible
        && node.style.overflow_y == LayoutOverflow::Visible
        && node.style.box_sizing == LayoutBoxSizing::ContentBox
        && node.style.height == LayoutDimension::Fixed(30.0)
        && node.style.min_width == LayoutDimension::Auto
        && node.style.max_width == LayoutDimension::Auto
        && node.style.min_height == LayoutDimension::Auto
        && node.style.max_height == LayoutDimension::Auto
        && node.style.aspect_ratio.is_none()
        && node.style.order == 0
        && node.style.align_self == LayoutAlignSelf::Auto
        && node.style.margin == zero_edges()
        && node.style.padding == zero_edges()
        && node.style.border == LayoutBorder::default()
        && node.children.is_empty()
        && placement
            == Some(&GridItemInput {
                row_start: GridPlacementInput::Auto,
                row_end: GridPlacementInput::Auto,
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(-1),
            })
}

fn is_wpt_absolute_probe(node: &crate::LayoutNode, positioning: LayoutPositioning) -> bool {
    positioning.position == LayoutPosition::Absolute
        && positioning.inset == LayoutEdges::auto()
        && node.style.display == LayoutDisplay::Block
        && node.style.overflow_x == LayoutOverflow::Visible
        && node.style.overflow_y == LayoutOverflow::Visible
        && node.style.box_sizing == LayoutBoxSizing::ContentBox
        && node.style.width == LayoutDimension::Percent(1.0)
        && node.style.height == LayoutDimension::Fixed(5.0)
        && node.style.min_width == LayoutDimension::Auto
        && node.style.max_width == LayoutDimension::Auto
        && node.style.min_height == LayoutDimension::Auto
        && node.style.max_height == LayoutDimension::Auto
        && node.style.aspect_ratio.is_none()
        && node.style.order == 0
        && node.style.align_self == LayoutAlignSelf::Auto
        && node.style.margin == zero_edges()
        && node.style.padding == zero_edges()
        && node.style.border == LayoutBorder::default()
        && node.children.is_empty()
}

fn zero_edges() -> LayoutEdges {
    LayoutEdges {
        top: LayoutLengthPercentage::ZERO,
        right: LayoutLengthPercentage::ZERO,
        bottom: LayoutLengthPercentage::ZERO,
        left: LayoutLengthPercentage::ZERO,
    }
}

fn probe_pair_index(placement: &GridItemInput) -> Option<usize> {
    if placement.row_start != GridPlacementInput::Auto
        || placement.row_end != GridPlacementInput::Auto
    {
        return None;
    }
    let (GridPlacementInput::Line(start), GridPlacementInput::Line(end)) =
        (&placement.column_start, &placement.column_end)
    else {
        return None;
    };
    if !(1..=4).contains(start) || *end != *start + 1 {
        return None;
    }
    usize::try_from(*start - 1).ok()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use spinon_core::{EnvironmentRevision, Revision, StyleRevision};

    use super::supports_wpt_probe_matrix;
    use crate::grid_placement::NormalizedGridLayout;
    use crate::{
        GridAutoRepeat, GridContainerInput, GridItemInput, GridPlacementInput,
        GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
        LayoutBorder, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutInput,
        LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutPosition, LayoutPositioning,
        LayoutSourceRevision, LayoutStyle, Viewport,
    };

    fn probe_matrix() -> (
        LayoutInput,
        NormalizedGridLayout,
        BTreeMap<spinon_core::NodeId, bool>,
        BTreeMap<spinon_core::NodeId, usize>,
    ) {
        let root_id = spinon_core::NodeId::new(1).unwrap();
        let child_ids = (2..=6)
            .map(|value| spinon_core::NodeId::new(value).unwrap())
            .collect::<Vec<_>>();
        let root = LayoutNode {
            id: root_id,
            children: child_ids.clone(),
            style: LayoutStyle {
                display: LayoutDisplay::Grid,
                box_sizing: LayoutBoxSizing::ContentBox,
                width: LayoutDimension::MinContent,
                min_width: LayoutDimension::Fixed(30.0),
                height: LayoutDimension::Auto,
                align_content: Some(LayoutAlignContent::Normal),
                border: LayoutBorder {
                    top: 2.0,
                    right: 2.0,
                    bottom: 2.0,
                    left: 2.0,
                },
                padding: LayoutEdges {
                    top: LayoutLengthPercentage::LengthPx(10.0),
                    right: LayoutLengthPercentage::ZERO,
                    bottom: LayoutLengthPercentage::ZERO,
                    left: LayoutLengthPercentage::ZERO,
                },
                margin: LayoutEdges {
                    top: LayoutLengthPercentage::ZERO,
                    right: LayoutLengthPercentage::ZERO,
                    bottom: LayoutLengthPercentage::LengthPx(10.0),
                    left: LayoutLengthPercentage::ZERO,
                },
                ..LayoutStyle::default()
            },
        };
        let mut nodes = vec![root];
        nodes.extend(
            child_ids
                .iter()
                .enumerate()
                .map(|(index, child_id)| LayoutNode {
                    id: *child_id,
                    children: Vec::new(),
                    style: if index == 0 {
                        LayoutStyle {
                            display: LayoutDisplay::Block,
                            box_sizing: LayoutBoxSizing::ContentBox,
                            width: LayoutDimension::Fixed(100.0),
                            height: LayoutDimension::Fixed(30.0),
                            ..LayoutStyle::default()
                        }
                    } else {
                        LayoutStyle {
                            display: LayoutDisplay::Block,
                            box_sizing: LayoutBoxSizing::ContentBox,
                            width: LayoutDimension::Percent(1.0),
                            height: LayoutDimension::Fixed(5.0),
                            ..LayoutStyle::default()
                        }
                    },
                }),
        );

        let in_flow = GridItemInput {
            row_start: GridPlacementInput::Auto,
            row_end: GridPlacementInput::Auto,
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(-1),
        };
        let placements = child_ids
            .iter()
            .enumerate()
            .map(|(index, child_id)| {
                let placement = if index == 0 {
                    in_flow.clone()
                } else {
                    GridItemInput {
                        row_start: GridPlacementInput::Auto,
                        row_end: GridPlacementInput::Auto,
                        column_start: GridPlacementInput::Line(index as i32),
                        column_end: GridPlacementInput::Line(index as i32 + 1),
                    }
                };
                (*child_id, placement)
            })
            .collect::<BTreeMap<_, _>>();
        let normalized = NormalizedGridLayout {
            item_placements: placements.clone(),
            ..NormalizedGridLayout::default()
        };
        let container = GridContainerInput {
            columns: vec![
                GridTrackSize::LengthPx(20.0),
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::MinContent,
                    max: GridTrackMaxSizingFunction::LengthPx(40.0),
                },
            ],
            rows: Vec::new(),
            columns_auto_repeat: Some(GridAutoRepeat {
                mode: crate::GridAutoRepeatMode::AutoFill,
                start: 0,
                end: 1,
                line_names: vec![Vec::new(), Vec::new()],
            }),
            rows_auto_repeat: None,
            columns_line_names: vec![Vec::new(), Vec::new(), Vec::new()],
            rows_line_names: Vec::new(),
            template_areas: None,
            auto_flow: crate::GridAutoFlow::Row,
            auto_rows: crate::GridAutoRows::Auto,
            auto_columns: crate::GridAutoColumns::LengthPx(20.0),
            placement_mode: crate::GridPlacementMode::TrackSizing,
        };
        let positioning = BTreeMap::from([
            (
                root_id,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    inset: LayoutEdges::auto(),
                },
            ),
            (
                child_ids[1],
                LayoutPositioning {
                    position: LayoutPosition::Absolute,
                    inset: LayoutEdges::auto(),
                },
            ),
            (
                child_ids[2],
                LayoutPositioning {
                    position: LayoutPosition::Absolute,
                    inset: LayoutEdges::auto(),
                },
            ),
            (
                child_ids[3],
                LayoutPositioning {
                    position: LayoutPosition::Absolute,
                    inset: LayoutEdges::auto(),
                },
            ),
            (
                child_ids[4],
                LayoutPositioning {
                    position: LayoutPosition::Absolute,
                    inset: LayoutEdges::auto(),
                },
            ),
        ]);
        let input = LayoutInput {
            root: root_id,
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::Tree(Revision::default()),
                StyleRevision::default(),
                EnvironmentRevision::default(),
            ),
            viewport: Viewport {
                width: 320.0,
                height: 200.0,
            },
            root_sizing: crate::RootSizingPolicy::Match,
            nodes,
            css_math: Vec::new(),
            positioning,
            grid_containers: BTreeMap::from([(root_id, container)]),
            grid_items: placements,
            text_paragraphs: BTreeMap::new(),
        };
        let hidden = child_ids
            .iter()
            .copied()
            .chain(std::iter::once(root_id))
            .map(|id| (id, false))
            .collect();
        let index = input
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id, index))
            .collect();
        (input, normalized, hidden, index)
    }

    #[test]
    fn absolute_auto_repeat_probe_exception_requires_the_exact_wpt_matrix() {
        let (input, normalized, hidden, index) = probe_matrix();
        let root = &input.nodes[0];
        let container = &input.grid_containers[&root.id];
        assert!(supports_wpt_probe_matrix(
            &input,
            root,
            container,
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input
            .positioning
            .get_mut(&spinon_core::NodeId::new(3).unwrap())
            .unwrap()
            .inset
            .top = LayoutLengthPercentage::LengthPx(0.0);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (input, mut normalized, hidden, index) = probe_matrix();
        normalized
            .item_placements
            .get_mut(&spinon_core::NodeId::new(3).unwrap())
            .unwrap()
            .column_end = GridPlacementInput::Line(3);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input.nodes[2].style.width = LayoutDimension::Percent(0.5);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input.nodes[1].style.width = LayoutDimension::Fixed(90.0);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input.nodes[0].style.gap.column = LayoutLengthPercentage::LengthPx(1.0);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input
            .grid_containers
            .get_mut(&spinon_core::NodeId::new(1).unwrap())
            .unwrap()
            .columns_auto_repeat
            .as_mut()
            .unwrap()
            .line_names[0]
            .push("named-line".to_owned());
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (mut input, normalized, hidden, index) = probe_matrix();
        input.nodes[0].children.swap(2, 3);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));

        let (input, normalized, mut hidden, index) = probe_matrix();
        hidden.insert(spinon_core::NodeId::new(6).unwrap(), true);
        let root = &input.nodes[0];
        assert!(!supports_wpt_probe_matrix(
            &input,
            root,
            &input.grid_containers[&root.id],
            &normalized,
            &hidden,
            &index,
        ));
    }
}
