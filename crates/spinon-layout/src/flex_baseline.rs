mod lines;
mod metrics;

use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::{
    FlexDirection, FlexWrap, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf, LayoutDisplay,
    LayoutError, LayoutInput, LayoutStyle, TextDirection, calc_tree::CalcLayoutTree,
};

const GEOMETRY_EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BaselineKind {
    First,
    Last,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BaselineMetrics {
    pub(super) first: f32,
    pub(super) last: f32,
}

#[derive(Clone, Debug)]
pub(super) struct FlexLine {
    pub(super) items: Vec<NodeId>,
}

struct BaselineAlignmentContext<'a> {
    input: &'a LayoutInput,
    index: &'a BTreeMap<NodeId, usize>,
    tree: &'a mut CalcLayoutTree,
    metrics: &'a mut BTreeMap<NodeId, BaselineMetrics>,
    measuring: &'a mut BTreeSet<NodeId>,
}

/// Taffy가 제공하는 row first-baseline 정렬을 바탕으로 first/last 그룹을 따로 계산하고
/// 보정한 border-box 위치를 결과에 반영합니다.
pub(super) fn apply(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    postorder: &[NodeId],
    tree: &mut CalcLayoutTree,
) -> Result<(), LayoutError> {
    validate_baseline_content_alignment(input, index)?;

    let mut metrics = BTreeMap::new();
    let mut measuring = BTreeSet::new();
    for parent in postorder {
        let parent_style = input.nodes[index[parent]].style;
        if parent_style.display != LayoutDisplay::Flex
            || !has_baseline_participant(*parent, parent_style, input, index)
        {
            continue;
        }

        if matches!(
            parent_style.flex_direction,
            FlexDirection::Column | FlexDirection::ColumnReverse
        ) {
            // horizontal-tb에서 column container의 cross axis는 inline baseline과
            // 수직입니다. 이 경우 CSS의 cross-start fallback을 사용합니다.
            continue;
        }
        if parent_style.direction != TextDirection::Ltr {
            return Err(unsupported(
                *parent,
                "RTL baseline 축은 C17 전까지 지원하지 않습니다",
            ));
        }

        let flex_lines = lines::collect(*parent, input, index, tree)?;
        let line_bounds = lines::bounds(*parent, &flex_lines, parent_style, input, tree)?;
        let mut context = BaselineAlignmentContext {
            input,
            index,
            tree,
            metrics: &mut metrics,
            measuring: &mut measuring,
        };
        for (line, bounds) in flex_lines.iter().zip(line_bounds) {
            for kind in [BaselineKind::First, BaselineKind::Last] {
                let items = baseline_group(line, kind, parent_style, input, index);
                align_group(
                    *parent,
                    &items,
                    kind,
                    bounds,
                    parent_style.flex_wrap == FlexWrap::WrapReverse,
                    &mut context,
                )?;
            }
        }
    }
    Ok(())
}

fn validate_baseline_content_alignment(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<(), LayoutError> {
    for node in &input.nodes {
        if node.style.display == LayoutDisplay::None
            || node.style.align_content != Some(LayoutAlignContent::FirstBaseline)
        {
            continue;
        }
        if node.style.display != LayoutDisplay::Flex {
            return Err(unsupported(
                node.id,
                "align-content:first baseline의 Block 동작은 현재 profile에서 지원하지 않습니다",
            ));
        }
        for child in &node.children {
            let child_node = &input.nodes[index[child]];
            if child_node.style.display == LayoutDisplay::None {
                continue;
            }
            let has_visible_children = child_node.children.iter().any(|descendant| {
                input.nodes[index[descendant]].style.display != LayoutDisplay::None
            });
            if has_visible_children
                || matches!(child_node.style.width, crate::LayoutDimension::Auto)
                || matches!(child_node.style.height, crate::LayoutDimension::Auto)
            {
                return Err(unsupported(
                    node.id,
                    "align-content:first baseline은 빈 fixed-size item의 pinned Chrome geometry만 지원합니다",
                ));
            }
        }
    }
    Ok(())
}

fn has_baseline_participant(
    parent: NodeId,
    parent_style: LayoutStyle,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> bool {
    input.nodes[index[&parent]].children.iter().any(|child| {
        let style = input.nodes[index[child]].style;
        style.display != LayoutDisplay::None
            && effective_baseline_kind(parent_style, style).is_some()
            && !lines::has_cross_axis_auto_margin(style)
    })
}

fn baseline_group(
    line: &FlexLine,
    kind: BaselineKind,
    parent_style: LayoutStyle,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Vec<NodeId> {
    line.items
        .iter()
        .copied()
        .filter(|child| {
            let style = input.nodes[index[child]].style;
            style.display != LayoutDisplay::None
                && effective_baseline_kind(parent_style, style) == Some(kind)
                && !lines::has_cross_axis_auto_margin(style)
        })
        .collect()
}

fn align_group(
    parent: NodeId,
    items: &[NodeId],
    kind: BaselineKind,
    bounds: lines::LineBounds,
    wrap_reverse: bool,
    context: &mut BaselineAlignmentContext<'_>,
) -> Result<(), LayoutError> {
    if items.is_empty() {
        return Ok(());
    }

    let mut entries = Vec::with_capacity(items.len());
    for child in items {
        let layout = context
            .tree
            .layout_for(*child)
            .ok_or(LayoutError::MissingComputedLayout(*child))?;
        let baseline = metrics::item_baseline(
            *child,
            kind,
            context.input,
            context.index,
            context.tree,
            context.metrics,
            context.measuring,
        )?;
        let (margin_start, margin_end) = lines::cross_margins(parent, *child, context.tree)?;
        entries.push((*child, layout, baseline, margin_start, margin_end));
    }

    // 참여자가 하나뿐이면 해당 first/last 값의 flex-start/flex-end fallback을 사용합니다.
    let use_cross_start = (kind == BaselineKind::First) != wrap_reverse;
    if entries.len() == 1 {
        let (child, layout, _, margin_start, margin_end) = entries[0];
        let desired_y = if use_cross_start {
            bounds.start + margin_start
        } else {
            bounds.end - margin_end - layout.size.height
        };
        return context.tree.shift_y(child, desired_y - layout.location.y);
    }

    // First baseline 그룹은 flex-start, last baseline 그룹은 flex-end에 둡니다.
    // wrap-reverse는 물리적 edge만 뒤집고 그룹 구성은 바꾸지 않습니다.
    let target = if use_cross_start {
        bounds.start
            + entries
                .iter()
                .map(|(_, _, baseline, margin_start, _)| baseline + margin_start)
                .fold(f32::NEG_INFINITY, f32::max)
    } else {
        bounds.end
            - entries
                .iter()
                .map(|(_, layout, baseline, _, margin_end)| {
                    layout.size.height + margin_end - baseline
                })
                .fold(f32::NEG_INFINITY, f32::max)
    };
    if !target.is_finite() {
        return Err(unsupported(
            parent,
            "baseline target가 유한한 CSS 좌표가 아닙니다",
        ));
    }

    for (child, layout, baseline, _, _) in entries {
        context
            .tree
            .shift_y(child, target - baseline - layout.location.y)?;
    }
    Ok(())
}

pub(super) fn effective_baseline_kind(
    parent: LayoutStyle,
    child: LayoutStyle,
) -> Option<BaselineKind> {
    match child.align_self {
        LayoutAlignSelf::Auto => align_items_baseline(parent.align_items),
        LayoutAlignSelf::FirstBaseline => Some(BaselineKind::First),
        LayoutAlignSelf::LastBaseline => Some(BaselineKind::Last),
        _ => None,
    }
}

fn align_items_baseline(value: LayoutAlignItems) -> Option<BaselineKind> {
    match value {
        LayoutAlignItems::FirstBaseline => Some(BaselineKind::First),
        LayoutAlignItems::LastBaseline => Some(BaselineKind::Last),
        _ => None,
    }
}

fn unsupported(node: NodeId, reason: &'static str) -> LayoutError {
    LayoutError::UnsupportedBaseline { node, reason }
}
