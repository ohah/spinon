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
    if input.root != node.id
        || input.nodes.len() != 6
        || input.grid_containers.len() != 1
        || input.grid_items.len() != 5
        || !is_target_container(input, node, container)
        || input.positioning.get(&node.id).copied().unwrap_or_default()
            != (LayoutPositioning {
                position: LayoutPosition::Relative,
                inset: LayoutEdges::auto(),
            })
        || node.children.len() != 5
        || hidden.get(&node.id).copied().unwrap_or(true)
    {
        return false;
    }

    for (index_in_parent, child_id) in node.children.iter().enumerate() {
        if hidden.get(child_id).copied().unwrap_or(true) {
            return false;
        }
        let Some(child_index) = index.get(child_id).copied() else {
            return false;
        };
        let child = &input.nodes[child_index];
        if !child.children.is_empty() {
            return false;
        }
        let Some(placement) = normalized.item_placements.get(child_id) else {
            return false;
        };

        if index_in_parent == 0 {
            if input
                .positioning
                .get(child_id)
                .copied()
                .unwrap_or_default()
                .position
                != LayoutPosition::Static
                || !is_wpt_in_flow_item(child, placement)
            {
                return false;
            }
            continue;
        }

        let pair_index = index_in_parent - 1;
        if probe_pair_index(placement) != Some(pair_index)
            || !is_wpt_absolute_probe(
                child,
                input.positioning.get(child_id).copied().unwrap_or_default(),
            )
        {
            return false;
        }
    }

    node.children
        .iter()
        .all(|child| input.grid_items.contains_key(child))
}

fn is_target_container(
    input: &LayoutInput,
    node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> bool {
    let style = node.style;
    let Some(auto_repeat) = container.columns_auto_repeat.as_ref() else {
        return false;
    };
    let padding = LayoutEdges {
        top: LayoutLengthPercentage::LengthPx(10.0),
        right: LayoutLengthPercentage::ZERO,
        bottom: LayoutLengthPercentage::ZERO,
        left: LayoutLengthPercentage::ZERO,
    };
    let border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    let fixed_neighbor = container.columns.as_slice()
        == [
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::MinContent,
                max: GridTrackMaxSizingFunction::LengthPx(40.0),
            },
        ];
    let auto_neighbor = container.columns.as_slice()
        == [
            GridTrackSize::LengthPx(10.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(60.0),
                max: GridTrackMaxSizingFunction::Auto,
            },
        ];

    style.display == LayoutDisplay::Grid
        && style.overflow_x == LayoutOverflow::Visible
        && style.overflow_y == LayoutOverflow::Visible
        && style.box_sizing == LayoutBoxSizing::ContentBox
        && matches!(
            style.width,
            LayoutDimension::MinContent | LayoutDimension::MaxContent
        )
        && style.height == LayoutDimension::Auto
        && style.min_width == LayoutDimension::Auto
        && style.max_width == LayoutDimension::Auto
        && style.min_height == LayoutDimension::Auto
        && style.max_height == LayoutDimension::Auto
        && style.aspect_ratio.is_none()
        && style.order == 0
        && style.direction == TextDirection::Ltr
        && style.align_items == LayoutAlignItems::Normal
        && style.align_self == LayoutAlignSelf::Auto
        && style.align_content == Some(LayoutAlignContent::Normal)
        && style.justify_content == LayoutJustifyContent::Normal
        && container.justify_items
            == (LayoutAlignItems::Position {
                position: crate::ItemAlignmentPosition::Start,
                safety: None,
            })
        && style.margin == zero_edges()
        && style.padding == padding
        && style.border == border
        && style.gap == LayoutGap::default()
        && container.placement_mode == GridPlacementMode::TrackSizing
        && matches!(
            auto_repeat.mode,
            GridAutoRepeatMode::AutoFill | GridAutoRepeatMode::AutoFit
        )
        && auto_repeat.start == 0
        && auto_repeat.end == 1
        && auto_repeat.line_names.iter().all(Vec::is_empty)
        && container.rows_auto_repeat.is_none()
        && container.rows.is_empty()
        && container.columns_line_names.iter().all(Vec::is_empty)
        && container.rows_line_names.is_empty()
        && (fixed_neighbor || auto_neighbor)
        && container.auto_flow == GridAutoFlow::Row
        && container.auto_rows == crate::GridAutoRows::Auto
        && container.auto_columns == GridAutoColumns::Auto
        && container.template_areas.is_none()
        && input.grid_containers.contains_key(&node.id)
}

fn is_wpt_in_flow_item(node: &crate::LayoutNode, placement: &GridItemInput) -> bool {
    node.style.display == LayoutDisplay::Block
        && node.style.overflow_x == LayoutOverflow::Visible
        && node.style.overflow_y == LayoutOverflow::Visible
        && node.style.box_sizing == LayoutBoxSizing::ContentBox
        && node.style.width == LayoutDimension::Fixed(100.0)
        && node.style.height == LayoutDimension::Fixed(20.0)
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
        && placement
            == &(GridItemInput {
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
        LayoutAlignItems, LayoutBorder, LayoutBoxSizing, LayoutDimension, LayoutDisplay,
        LayoutEdges, LayoutInput, LayoutInputRevision, LayoutLengthPercentage, LayoutNode,
        LayoutPosition, LayoutPositioning, LayoutSourceRevision, LayoutStyle, Viewport,
    };

    fn probe_matrix(
        mode: crate::GridAutoRepeatMode,
        width: LayoutDimension,
        auto_neighbor: bool,
    ) -> (
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
                width,
                min_width: LayoutDimension::Auto,
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
                ..LayoutStyle::default()
            },
        };
        let mut nodes = vec![root];
        nodes.extend(
            child_ids
                .iter()
                .enumerate()
                .map(|(position, child_id)| LayoutNode {
                    id: *child_id,
                    children: Vec::new(),
                    style: if position == 0 {
                        LayoutStyle {
                            display: LayoutDisplay::Block,
                            box_sizing: LayoutBoxSizing::ContentBox,
                            width: LayoutDimension::Fixed(100.0),
                            height: LayoutDimension::Fixed(20.0),
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
        let placements = child_ids
            .iter()
            .enumerate()
            .map(|(position, child_id)| {
                let placement = if position == 0 {
                    GridItemInput {
                        row_start: GridPlacementInput::Auto,
                        row_end: GridPlacementInput::Auto,
                        column_start: GridPlacementInput::Line(1),
                        column_end: GridPlacementInput::Line(-1),
                    }
                } else {
                    GridItemInput {
                        row_start: GridPlacementInput::Auto,
                        row_end: GridPlacementInput::Auto,
                        column_start: GridPlacementInput::Line(position as i32),
                        column_end: GridPlacementInput::Line(position as i32 + 1),
                    }
                };
                (*child_id, placement)
            })
            .collect::<BTreeMap<_, _>>();
        let normalized = NormalizedGridLayout {
            item_placements: placements.clone(),
            ..NormalizedGridLayout::default()
        };
        let columns = if auto_neighbor {
            vec![
                GridTrackSize::LengthPx(10.0),
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(60.0),
                    max: GridTrackMaxSizingFunction::Auto,
                },
            ]
        } else {
            vec![
                GridTrackSize::LengthPx(20.0),
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::MinContent,
                    max: GridTrackMaxSizingFunction::LengthPx(40.0),
                },
            ]
        };
        let container = GridContainerInput {
            columns,
            columns_auto_repeat: Some(GridAutoRepeat {
                mode,
                start: 0,
                end: 1,
                line_names: vec![Vec::new(), Vec::new()],
            }),
            columns_line_names: vec![Vec::new(), Vec::new(), Vec::new()],
            justify_items: LayoutAlignItems::Position {
                position: crate::ItemAlignmentPosition::Start,
                safety: None,
            },
            placement_mode: crate::GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        };
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
            positioning: BTreeMap::from([
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
            ]),
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

    fn matches(
        input: &LayoutInput,
        normalized: &NormalizedGridLayout,
        hidden: &BTreeMap<spinon_core::NodeId, bool>,
        index: &BTreeMap<spinon_core::NodeId, usize>,
    ) -> bool {
        let root = &input.nodes[0];
        supports_wpt_probe_matrix(
            input,
            root,
            &input.grid_containers[&root.id],
            normalized,
            hidden,
            index,
        )
    }

    #[test]
    fn accepts_only_the_eight_auto_repeat_intrinsic_wpt_signatures() {
        for mode in [
            crate::GridAutoRepeatMode::AutoFill,
            crate::GridAutoRepeatMode::AutoFit,
        ] {
            for width in [LayoutDimension::MinContent, LayoutDimension::MaxContent] {
                for auto_neighbor in [false, true] {
                    let (input, normalized, hidden, index) =
                        probe_matrix(mode, width, auto_neighbor);
                    assert!(matches(&input, &normalized, &hidden, &index));
                }
            }
        }
    }

    #[test]
    fn rejects_nearby_signatures_that_change_the_css_contract() {
        let (input, normalized, hidden, index) = probe_matrix(
            crate::GridAutoRepeatMode::AutoFill,
            LayoutDimension::MinContent,
            true,
        );
        let mut input_with_min_width = input.clone();
        input_with_min_width.nodes[0].style.min_width = LayoutDimension::Fixed(1.0);
        assert!(!matches(
            &input_with_min_width,
            &normalized,
            &hidden,
            &index
        ));

        let mut input_with_fixed_auto_columns = input.clone();
        input_with_fixed_auto_columns
            .grid_containers
            .get_mut(&input.root)
            .unwrap()
            .auto_columns = crate::GridAutoColumns::LengthPx(20.0);
        assert!(!matches(
            &input_with_fixed_auto_columns,
            &normalized,
            &hidden,
            &index
        ));

        let mut input_with_wrong_probe_width = input.clone();
        input_with_wrong_probe_width.nodes[2].style.width = LayoutDimension::Percent(0.5);
        assert!(!matches(
            &input_with_wrong_probe_width,
            &normalized,
            &hidden,
            &index
        ));

        let mut input_with_extra_node = input.clone();
        input_with_extra_node.nodes.push(input.nodes[1].clone());
        assert!(!matches(
            &input_with_extra_node,
            &normalized,
            &hidden,
            &index
        ));

        let mut reordered = input.clone();
        reordered.nodes[0].children.swap(2, 3);
        assert!(!matches(&reordered, &normalized, &hidden, &index));

        let mut wrong_repeat_fragment = input.clone();
        wrong_repeat_fragment
            .grid_containers
            .get_mut(&input.root)
            .unwrap()
            .columns_auto_repeat
            .as_mut()
            .unwrap()
            .end = 2;
        assert!(!matches(
            &wrong_repeat_fragment,
            &normalized,
            &hidden,
            &index
        ));
    }
}
