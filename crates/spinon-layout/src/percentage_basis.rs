use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    FlexDirection, LayoutCalcId, LayoutDimension, LayoutDisplay, LayoutInput,
    LayoutLengthPercentage, LayoutNode, RootSizingPolicy, error::LayoutError,
};

#[derive(Clone, Copy, Debug, Default)]
struct DefiniteAxes {
    width: bool,
    height: bool,
}

pub(super) fn validate_spacing_percentage_bases(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<(), LayoutError> {
    let calc_percentages = input
        .css_math
        .iter()
        .map(|value| {
            value
                .contains_percentage()
                .map(|contains| (value.id, contains))
                .map_err(|reason| value.error(reason))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let mut parents = BTreeMap::new();
    for node in &input.nodes {
        for child in &node.children {
            parents.insert(*child, node.id);
        }
    }

    let mut definite = BTreeMap::<NodeId, DefiniteAxes>::new();
    let mut hidden = BTreeMap::<NodeId, bool>::new();
    let mut pending = vec![input.root];

    while let Some(id) = pending.pop() {
        let node = &input.nodes[index[&id]];
        let parent = parents
            .get(&id)
            .map(|parent_id| &input.nodes[index[parent_id]]);
        let parent_axes = parents
            .get(&id)
            .and_then(|parent_id| definite.get(parent_id))
            .copied()
            .unwrap_or_else(|| match input.root_sizing {
                RootSizingPolicy::BlockFormatting if id == input.root => DefiniteAxes {
                    width: true,
                    height: true,
                },
                _ => DefiniteAxes::default(),
            });
        let position_parent_axes = if id == input.root {
            // 레이아웃 루트의 containing block은 호출 환경의 확정 viewport입니다.
            DefiniteAxes {
                width: true,
                height: true,
            }
        } else {
            parent_axes
        };
        let is_hidden = parent
            .and_then(|parent| hidden.get(&parent.id))
            .copied()
            .unwrap_or(false)
            || node.style.display == LayoutDisplay::None;
        hidden.insert(id, is_hidden);
        let axes = if id == input.root {
            root_axes(input.root_sizing, node, &calc_percentages)
        } else {
            DefiniteAxes {
                width: is_axis_definite(
                    LayoutAxis::Width,
                    node,
                    parent.expect("검증된 비루트 노드에는 부모가 있습니다"),
                    parent_axes,
                    &calc_percentages,
                ),
                height: is_axis_definite(
                    LayoutAxis::Height,
                    node,
                    parent.expect("검증된 비루트 노드에는 부모가 있습니다"),
                    parent_axes,
                    &calc_percentages,
                ),
            }
        };
        if !is_hidden {
            if id == input.root {
                validate_root_gap_percentage(node, &calc_percentages)?;
            }
            validate_edge_percentages(node, parent_axes, &calc_percentages)?;
            validate_position_percentage_bases(
                id,
                input.positioning.get(&id).copied(),
                position_parent_axes,
                &calc_percentages,
            )?;
            validate_main_axis_gap(node, axes, &calc_percentages)?;
        }
        definite.insert(id, axes);
        pending.extend(node.children.iter().rev().copied());
    }
    Ok(())
}

fn validate_position_percentage_bases(
    node: NodeId,
    positioning: Option<crate::LayoutPositioning>,
    parent_axes: DefiniteAxes,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> Result<(), LayoutError> {
    let Some(positioning) = positioning else {
        return Ok(());
    };
    if positioning.position != crate::LayoutPosition::Relative {
        return Ok(());
    }
    for (property, value, is_definite, axis) in [
        ("top", positioning.inset.top, parent_axes.height, "height"),
        ("right", positioning.inset.right, parent_axes.width, "width"),
        (
            "bottom",
            positioning.inset.bottom,
            parent_axes.height,
            "height",
        ),
        ("left", positioning.inset.left, parent_axes.width, "width"),
    ] {
        if has_percentage(value, calc_percentages) && !is_definite {
            return Err(LayoutError::IndefinitePercentageBasis {
                node,
                property,
                axis: if axis == "width" {
                    "containing block width"
                } else {
                    "containing block height"
                },
            });
        }
    }
    Ok(())
}

fn validate_root_gap_percentage(
    node: &LayoutNode,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> Result<(), LayoutError> {
    for (property, value) in [
        ("row-gap", node.style.gap.row),
        ("column-gap", node.style.gap.column),
    ] {
        if has_percentage(value, calc_percentages) {
            return Err(LayoutError::UnsupportedRootPercentageSpacing {
                node: node.id,
                property,
            });
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum LayoutAxis {
    Width,
    Height,
}

impl LayoutAxis {
    fn dimension(self, style: crate::LayoutStyle) -> LayoutDimension {
        match self {
            Self::Width => style.width,
            Self::Height => style.height,
        }
    }

    fn parent_definite(self, axes: DefiniteAxes) -> bool {
        match self {
            Self::Width => axes.width,
            Self::Height => axes.height,
        }
    }
}

fn root_axes(
    policy: RootSizingPolicy,
    node: &LayoutNode,
    _calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> DefiniteAxes {
    match policy {
        RootSizingPolicy::Match => DefiniteAxes {
            width: true,
            height: true,
        },
        RootSizingPolicy::ResolveWithin | RootSizingPolicy::BlockFormatting => DefiniteAxes {
            width: match node.style.width {
                LayoutDimension::Fixed(_)
                | LayoutDimension::Percent(_)
                | LayoutDimension::Calc(_) => true,
                LayoutDimension::Auto => {
                    matches!(
                        node.style.display,
                        LayoutDisplay::Block | LayoutDisplay::FlowRoot | LayoutDisplay::Flex
                    )
                }
            },
            height: !matches!(node.style.height, LayoutDimension::Auto),
        },
    }
}

fn is_axis_definite(
    axis: LayoutAxis,
    node: &LayoutNode,
    parent: &LayoutNode,
    parent_axes: DefiniteAxes,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> bool {
    match axis.dimension(node.style) {
        LayoutDimension::Fixed(_) => true,
        LayoutDimension::Percent(_) => axis.parent_definite(parent_axes),
        LayoutDimension::Calc(id) => {
            !calc_percentages.get(&id).copied().unwrap_or(true) || axis.parent_definite(parent_axes)
        }
        LayoutDimension::Auto => {
            let block_auto_width = matches!(axis, LayoutAxis::Width)
                && matches!(
                    parent.style.display,
                    LayoutDisplay::Block | LayoutDisplay::FlowRoot
                )
                && parent_axes.width;
            let stretched_cross_size = parent.style.display == LayoutDisplay::Flex
                && parent.style.align_items.uses_stretch_behavior()
                && match (parent.style.flex_direction, axis) {
                    (FlexDirection::Row, LayoutAxis::Height)
                    | (FlexDirection::RowReverse, LayoutAxis::Height)
                    | (FlexDirection::Column, LayoutAxis::Width)
                    | (FlexDirection::ColumnReverse, LayoutAxis::Width) => {
                        axis.parent_definite(parent_axes)
                    }
                    _ => false,
                };
            block_auto_width || stretched_cross_size
        }
    }
}

fn validate_edge_percentages(
    node: &LayoutNode,
    parent_axes: DefiniteAxes,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> Result<(), LayoutError> {
    for (property, value) in [
        ("margin-top", node.style.margin.top),
        ("margin-right", node.style.margin.right),
        ("margin-bottom", node.style.margin.bottom),
        ("margin-left", node.style.margin.left),
        ("padding-top", node.style.padding.top),
        ("padding-right", node.style.padding.right),
        ("padding-bottom", node.style.padding.bottom),
        ("padding-left", node.style.padding.left),
    ] {
        if has_percentage(value, calc_percentages) && !parent_axes.width {
            return Err(LayoutError::IndefinitePercentageBasis {
                node: node.id,
                property,
                axis: "containing block width",
            });
        }
    }
    Ok(())
}

fn validate_main_axis_gap(
    node: &LayoutNode,
    definite: DefiniteAxes,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> Result<(), LayoutError> {
    if node.style.display != LayoutDisplay::Flex {
        return Ok(());
    }
    let (property, value, axis, is_definite) = match node.style.flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            ("column-gap", node.style.gap.column, "width", definite.width)
        }
        FlexDirection::Column | FlexDirection::ColumnReverse => {
            ("row-gap", node.style.gap.row, "height", definite.height)
        }
    };
    if has_percentage(value, calc_percentages) && !is_definite {
        return Err(LayoutError::IndefinitePercentageBasis {
            node: node.id,
            property,
            axis,
        });
    }
    Ok(())
}

fn has_percentage(
    value: LayoutLengthPercentage,
    calc_percentages: &BTreeMap<LayoutCalcId, bool>,
) -> bool {
    match value {
        LayoutLengthPercentage::Auto => false,
        LayoutLengthPercentage::LengthPx(_) => false,
        LayoutLengthPercentage::Percentage(_) => true,
        LayoutLengthPercentage::Calc(id) => calc_percentages.get(&id).copied().unwrap_or(true),
    }
}
