use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::{
    FlexDirection, FlexWrap, LayoutDisplay, LayoutError, LayoutInput, LayoutStyle, TextDirection,
    calc_tree::CalcLayoutTree,
};

use super::{
    BaselineKind, BaselineMetrics, FlexLine, GEOMETRY_EPSILON, effective_baseline_kind, lines,
};

struct BaselineMeasurement<'a> {
    input: &'a LayoutInput,
    index: &'a BTreeMap<NodeId, usize>,
    tree: &'a CalcLayoutTree,
    metrics: &'a mut BTreeMap<NodeId, BaselineMetrics>,
    measuring: &'a mut BTreeSet<NodeId>,
}

pub(super) fn item_baseline(
    node: NodeId,
    kind: BaselineKind,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    tree: &CalcLayoutTree,
    metrics: &mut BTreeMap<NodeId, BaselineMetrics>,
    measuring: &mut BTreeSet<NodeId>,
) -> Result<f32, LayoutError> {
    let style = input.nodes[index[&node]].style;
    let layout = tree
        .layout_for(node)
        .ok_or(LayoutError::MissingComputedLayout(node))?;
    let baseline = if style.display == LayoutDisplay::Flex {
        let nested = container_baselines(node, input, index, tree, metrics, measuring)?;
        match kind {
            BaselineKind::First => nested.first,
            BaselineKind::Last => nested.last,
        }
    } else {
        let visible_children = input.nodes[index[&node]]
            .children
            .iter()
            .filter(|child| input.nodes[index[child]].style.display != LayoutDisplay::None)
            .count();
        if visible_children > 0 {
            return Err(unsupported(
                node,
                "일반 block·텍스트 baseline은 C15 line layout 전까지 지원하지 않습니다",
            ));
        }
        // 비어 있는 Flex item은 border edge에서 baseline을 합성합니다.
        layout.size.height
    };
    if baseline.is_finite() {
        Ok(baseline)
    } else {
        Err(unsupported(node, "item baseline이 유한하지 않습니다"))
    }
}

pub(super) fn container_baselines(
    node: NodeId,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    tree: &CalcLayoutTree,
    metrics: &mut BTreeMap<NodeId, BaselineMetrics>,
    measuring: &mut BTreeSet<NodeId>,
) -> Result<BaselineMetrics, LayoutError> {
    if let Some(found) = metrics.get(&node) {
        return Ok(*found);
    }
    if !measuring.insert(node) {
        return Err(LayoutError::Cycle(node));
    }

    let style = input.nodes[index[&node]].style;
    if style.display != LayoutDisplay::Flex {
        return Err(unsupported(
            node,
            "Flex container가 아닌 노드에서 baseline을 요청했습니다",
        ));
    }
    if style.direction != TextDirection::Ltr {
        return Err(unsupported(
            node,
            "RTL baseline 전파는 C17 전까지 지원하지 않습니다",
        ));
    }
    let flex_lines = lines::collect(node, input, index, tree)?;
    let layout = tree
        .layout_for(node)
        .ok_or(LayoutError::MissingComputedLayout(node))?;
    let mut context = BaselineMeasurement {
        input,
        index,
        tree,
        metrics,
        measuring,
    };
    let result = if flex_lines.is_empty() {
        // 빈 Flex container에는 baseline set이 없습니다. 바깥 정렬 문맥이 baseline을
        // 요구하면 border-box의 block-end edge에서 합성합니다.
        BaselineMetrics {
            first: layout.size.height,
            last: layout.size.height,
        }
    } else {
        let first_line = visual_line_index(style.flex_wrap, flex_lines.len(), BaselineKind::First);
        let last_line = visual_line_index(style.flex_wrap, flex_lines.len(), BaselineKind::Last);
        let first = container_baseline_for_line(
            node,
            &flex_lines[first_line],
            BaselineKind::First,
            style,
            &mut context,
        )?;
        let last = container_baseline_for_line(
            node,
            &flex_lines[last_line],
            BaselineKind::Last,
            style,
            &mut context,
        )?;
        BaselineMetrics { first, last }
    };
    context.measuring.remove(&node);
    context.metrics.insert(node, result);
    Ok(result)
}

fn visual_line_index(wrap: FlexWrap, line_count: usize, kind: BaselineKind) -> usize {
    let cross_start_is_last = wrap == FlexWrap::WrapReverse;
    let select_end = kind == BaselineKind::Last;
    if cross_start_is_last ^ select_end {
        line_count - 1
    } else {
        0
    }
}

fn container_baseline_for_line(
    parent: NodeId,
    line: &FlexLine,
    kind: BaselineKind,
    parent_style: LayoutStyle,
    context: &mut BaselineMeasurement<'_>,
) -> Result<f32, LayoutError> {
    if matches!(
        parent_style.flex_direction,
        FlexDirection::Column | FlexDirection::ColumnReverse
    ) {
        // 가로 writing mode의 column에서는 inline axis가 cross axis입니다.
        // order를 적용한 main-axis 순서에서 first/last Flex item을 고릅니다.
        let reverse = parent_style.flex_direction == FlexDirection::ColumnReverse;
        let select_end = (kind == BaselineKind::Last) ^ reverse;
        let child = if select_end {
            line.items.last()
        } else {
            line.items.first()
        }
        .copied()
        .ok_or_else(|| unsupported(parent, "빈 column baseline line을 계산할 수 없습니다"))?;
        let layout = context
            .tree
            .layout_for(child)
            .ok_or(LayoutError::MissingComputedLayout(child))?;
        return finite_baseline(
            parent,
            layout.location.y
                + item_baseline(
                    child,
                    kind,
                    context.input,
                    context.index,
                    context.tree,
                    context.metrics,
                    context.measuring,
                )?,
        );
    }

    let first = baseline_group(
        line,
        BaselineKind::First,
        parent_style,
        context.input,
        context.index,
    );
    let last = baseline_group(
        line,
        BaselineKind::Last,
        parent_style,
        context.input,
        context.index,
    );
    let selected = match kind {
        BaselineKind::First if !first.is_empty() => Some((&first, BaselineKind::First)),
        BaselineKind::First if !last.is_empty() => Some((&last, BaselineKind::Last)),
        BaselineKind::Last if !last.is_empty() => Some((&last, BaselineKind::Last)),
        BaselineKind::Last if !first.is_empty() => Some((&first, BaselineKind::First)),
        _ => None,
    };
    if let Some((items, group_kind)) = selected {
        let mut coordinates = Vec::with_capacity(items.len());
        for child in items {
            let layout = context
                .tree
                .layout_for(*child)
                .ok_or(LayoutError::MissingComputedLayout(*child))?;
            coordinates.push(
                layout.location.y
                    + item_baseline(
                        *child,
                        group_kind,
                        context.input,
                        context.index,
                        context.tree,
                        context.metrics,
                        context.measuring,
                    )?,
            );
        }
        let baseline = coordinates
            .first()
            .copied()
            .ok_or_else(|| unsupported(parent, "빈 baseline sharing group입니다"))?;
        if coordinates
            .iter()
            .any(|value| (value - baseline).abs() > GEOMETRY_EPSILON)
        {
            return Err(unsupported(
                parent,
                "중첩 Flex baseline-sharing group의 실제 좌표가 일치하지 않습니다",
            ));
        }
        return finite_baseline(parent, baseline);
    }

    // Flexbox는 order를 다시 정렬한 뒤 startmost/endmost item에서 container baseline을
    // 합성합니다. row-reverse는 main 시작 방향만 바꾸며 CalcLayoutTree가 제공한
    // order-modified item 목록의 순서는 바꾸지 않습니다.
    let child = match kind {
        BaselineKind::First => line.items.first(),
        BaselineKind::Last => line.items.last(),
    }
    .copied()
    .ok_or_else(|| unsupported(parent, "빈 row baseline line을 계산할 수 없습니다"))?;
    let layout = context
        .tree
        .layout_for(child)
        .ok_or(LayoutError::MissingComputedLayout(child))?;
    finite_baseline(
        parent,
        layout.location.y
            + item_baseline(
                child,
                kind,
                context.input,
                context.index,
                context.tree,
                context.metrics,
                context.measuring,
            )?,
    )
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

fn finite_baseline(parent: NodeId, value: f32) -> Result<f32, LayoutError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(unsupported(
            parent,
            "Flex container baseline이 유한하지 않습니다",
        ))
    }
}

fn unsupported(node: NodeId, reason: &'static str) -> LayoutError {
    LayoutError::UnsupportedBaseline { node, reason }
}
