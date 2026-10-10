use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    FlexDirection, FlexWrap, LayoutAlignContent, LayoutCssMathProperty, LayoutError, LayoutInput,
    LayoutLengthPercentage, LayoutStyle, calc_tree::CalcLayoutTree,
};

use super::{FlexLine, GEOMETRY_EPSILON};

#[derive(Clone, Copy, Debug)]
pub(super) struct LineBounds {
    pub(super) start: f32,
    pub(super) end: f32,
}

#[derive(Clone, Copy)]
enum SameMainDisposition {
    SameLine,
    NewLine,
}

pub(super) fn collect(
    parent: NodeId,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    tree: &CalcLayoutTree,
) -> Result<Vec<FlexLine>, LayoutError> {
    let style = input.nodes[index[&parent]].style;
    let mut children = input.nodes[index[&parent]]
        .children
        .iter()
        .copied()
        .filter(|child| input.nodes[index[child]].style.display != crate::LayoutDisplay::None)
        .collect::<Vec<_>>();
    children.sort_by_key(|child| input.nodes[index[child]].style.order);
    if children.is_empty() {
        return Ok(Vec::new());
    }
    if style.flex_wrap == FlexWrap::NoWrap {
        return Ok(vec![FlexLine { items: children }]);
    }

    let horizontal_main = matches!(
        style.flex_direction,
        FlexDirection::Row | FlexDirection::RowReverse
    );
    let main_reverse = matches!(
        style.flex_direction,
        FlexDirection::RowReverse | FlexDirection::ColumnReverse
    );
    let mut lines = vec![FlexLine { items: Vec::new() }];
    let mut previous: Option<f32> = None;
    for child in children {
        let layout = tree
            .layout_for(child)
            .ok_or(LayoutError::MissingComputedLayout(child))?;
        let main = if horizontal_main {
            layout.location.x
        } else {
            layout.location.y
        };
        let main_reset = previous.is_some_and(|previous_main| {
            if main_reverse {
                main > previous_main + GEOMETRY_EPSILON
            } else {
                main < previous_main - GEOMETRY_EPSILON
            }
        });
        let same_main =
            previous.is_some_and(|previous_main| (main - previous_main).abs() <= GEOMETRY_EPSILON);
        let same_main_disposition = if same_main {
            let previous_id = lines
                .last()
                .and_then(|line| line.items.last())
                .copied()
                .ok_or_else(|| unsupported(parent, "Flex line의 앞선 item이 없습니다"))?;
            let previous_layout = tree
                .layout_for(previous_id)
                .ok_or(LayoutError::MissingComputedLayout(previous_id))?;
            Some(same_main_disposition(
                parent,
                style,
                previous_layout,
                layout,
                input,
                tree,
            )?)
        } else {
            None
        };
        if main_reset || matches!(same_main_disposition, Some(SameMainDisposition::NewLine)) {
            lines.push(FlexLine { items: Vec::new() });
        }

        if let Some(line) = lines.last_mut() {
            if let Some(previous_id) = line.items.last().copied() {
                let previous_layout = tree
                    .layout_for(previous_id)
                    .ok_or(LayoutError::MissingComputedLayout(previous_id))?;
                let previous_main = if horizontal_main {
                    previous_layout.location.x
                } else {
                    previous_layout.location.y
                };
                let main_delta = if main_reverse {
                    previous_main - main
                } else {
                    main - previous_main
                };
                if main_delta < -GEOMETRY_EPSILON {
                    return Err(unsupported(
                        parent,
                        "Flex line 경계를 frame으로 하나로 확정할 수 없습니다",
                    ));
                }
            }
            line.items.push(child);
        }
        previous = Some(main);
    }
    Ok(lines)
}

fn same_main_disposition(
    parent: NodeId,
    style: LayoutStyle,
    previous: taffy::Layout,
    current: taffy::Layout,
    input: &LayoutInput,
    tree: &CalcLayoutTree,
) -> Result<SameMainDisposition, LayoutError> {
    let (previous_size, previous_end_margin, current_start_margin, gap, property, negative_margin) =
        match style.flex_direction {
            FlexDirection::Row => (
                previous.size.width,
                previous.margin.right,
                current.margin.left,
                style.gap.column,
                LayoutCssMathProperty::ColumnGap,
                previous.margin.right < 0.0 || current.margin.left < 0.0,
            ),
            FlexDirection::RowReverse => (
                previous.size.width,
                previous.margin.left,
                current.margin.right,
                style.gap.column,
                LayoutCssMathProperty::ColumnGap,
                previous.margin.left < 0.0 || current.margin.right < 0.0,
            ),
            FlexDirection::Column => (
                previous.size.height,
                previous.margin.bottom,
                current.margin.top,
                style.gap.row,
                LayoutCssMathProperty::RowGap,
                previous.margin.bottom < 0.0 || current.margin.top < 0.0,
            ),
            FlexDirection::ColumnReverse => (
                previous.size.height,
                previous.margin.top,
                current.margin.bottom,
                style.gap.row,
                LayoutCssMathProperty::RowGap,
                previous.margin.top < 0.0 || current.margin.bottom < 0.0,
            ),
        };

    let parent_layout = tree
        .layout_for(parent)
        .ok_or(LayoutError::MissingComputedLayout(parent))?;
    let basis = match style.flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            parent_layout.size.width
                - parent_layout.border.left
                - parent_layout.border.right
                - parent_layout.padding.left
                - parent_layout.padding.right
        }
        FlexDirection::Column | FlexDirection::ColumnReverse => {
            parent_layout.size.height
                - parent_layout.border.top
                - parent_layout.border.bottom
                - parent_layout.padding.top
                - parent_layout.padding.bottom
        }
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(unsupported(
            parent,
            "Flex main-axis content 크기가 유효하지 않습니다",
        ));
    }
    let gap_value = match gap {
        LayoutLengthPercentage::LengthPx(value) => value,
        LayoutLengthPercentage::Percentage(fraction) => basis * fraction,
        LayoutLengthPercentage::Calc(id) => {
            let value = input.css_math.iter().find(|value| value.id == id).ok_or(
                LayoutError::MissingCssMath {
                    node: parent,
                    property: match property {
                        LayoutCssMathProperty::ColumnGap => "column-gap",
                        LayoutCssMathProperty::RowGap => "row-gap",
                        _ => "flex gap",
                    },
                    id,
                },
            )?;
            if value.property != property || value.node_id != parent {
                return Err(LayoutError::CssMathBindingMismatch {
                    node: parent,
                    property: match property {
                        LayoutCssMathProperty::ColumnGap => "column-gap",
                        LayoutCssMathProperty::RowGap => "row-gap",
                        _ => "flex gap",
                    },
                    id,
                });
            }
            value.resolve(basis).map_err(|reason| value.error(reason))?
        }
        LayoutLengthPercentage::Auto => {
            return Err(unsupported(parent, "Flex gap은 auto일 수 없습니다"));
        }
    };
    if !gap_value.is_finite() || gap_value < 0.0 {
        return Err(unsupported(
            parent,
            "Flex main gap이 유한한 0 이상 값이 아닙니다",
        ));
    }
    let occupied = previous_size + previous_end_margin + current_start_margin + gap_value;
    if !occupied.is_finite() {
        return Err(unsupported(
            parent,
            "Flex main-axis item 간격이 유한하지 않습니다",
        ));
    }
    if occupied > GEOMETRY_EPSILON {
        Ok(SameMainDisposition::NewLine)
    } else if !negative_margin && occupied >= -GEOMETRY_EPSILON {
        // 주축 점유 폭이 0이면 item은 같은 줄 끝에 정확히 맞습니다. Flex line 수집은
        // exact fit인 item을 다음 줄로 보내지 않습니다.
        Ok(SameMainDisposition::SameLine)
    } else {
        Err(unsupported(
            parent,
            "음수 main margin으로 같은 좌표의 line 경계를 확정할 수 없습니다",
        ))
    }
}

pub(super) fn bounds(
    parent: NodeId,
    lines: &[FlexLine],
    parent_style: LayoutStyle,
    input: &LayoutInput,
    tree: &CalcLayoutTree,
) -> Result<Vec<LineBounds>, LayoutError> {
    if lines.is_empty() {
        return Ok(Vec::new());
    }

    let parent_layout = tree
        .layout_for(parent)
        .ok_or(LayoutError::MissingComputedLayout(parent))?;
    let content_start = parent_layout.border.top + parent_layout.padding.top;
    let content_end =
        parent_layout.size.height - parent_layout.border.bottom - parent_layout.padding.bottom;
    if !content_start.is_finite() || !content_end.is_finite() || content_end < content_start {
        return Err(unsupported(
            parent,
            "Flex content cross-axis 경계를 계산할 수 없습니다",
        ));
    }

    // Flexbox는 줄이 하나뿐이면 container의 inner cross 크기 전체를 사용합니다.
    if parent_style.flex_wrap == FlexWrap::NoWrap || lines.len() == 1 {
        return Ok(vec![LineBounds {
            start: content_start,
            end: content_end,
        }]);
    }

    let mut result = Vec::with_capacity(lines.len());
    for line in lines {
        let mut start = f32::INFINITY;
        let mut end = f32::NEG_INFINITY;
        for child in &line.items {
            let layout = tree
                .layout_for(*child)
                .ok_or(LayoutError::MissingComputedLayout(*child))?;
            finite_margin(parent, layout.margin.top)?;
            finite_margin(parent, layout.margin.bottom)?;
            start = start.min(layout.location.y - layout.margin.top);
            end = end.max(layout.location.y + layout.size.height + layout.margin.bottom);
        }
        if !start.is_finite() || !end.is_finite() || end < start {
            return Err(unsupported(
                parent,
                "wrapped line의 cross-axis 경계를 계산할 수 없습니다",
            ));
        }
        result.push(LineBounds { start, end });
    }

    if matches!(
        parent_style
            .align_content
            .unwrap_or(LayoutAlignContent::Normal),
        LayoutAlignContent::Normal | LayoutAlignContent::Stretch
    ) {
        let gap = resolve_row_gap(parent, parent_style.gap.row, input, tree)?;
        let occupied = result
            .iter()
            .map(|bounds| bounds.end - bounds.start)
            .sum::<f32>()
            + gap * result.len().saturating_sub(1) as f32;
        let content_size = content_end - content_start;
        let extra = (content_size - occupied).max(0.0) / result.len() as f32;
        if !extra.is_finite() {
            return Err(unsupported(
                parent,
                "stretch line 확장값이 유한하지 않습니다",
            ));
        }
        for bounds in &mut result {
            if parent_style.flex_wrap == FlexWrap::WrapReverse {
                bounds.start -= extra;
            } else {
                bounds.end += extra;
            }
        }
    }

    Ok(result)
}

fn resolve_row_gap(
    parent: NodeId,
    gap: LayoutLengthPercentage,
    input: &LayoutInput,
    tree: &CalcLayoutTree,
) -> Result<f32, LayoutError> {
    let layout = tree
        .layout_for(parent)
        .ok_or(LayoutError::MissingComputedLayout(parent))?;
    let basis = layout.size.height
        - layout.border.top
        - layout.border.bottom
        - layout.padding.top
        - layout.padding.bottom;
    let value = match gap {
        LayoutLengthPercentage::LengthPx(value) => value,
        LayoutLengthPercentage::Percentage(fraction) => basis * fraction,
        LayoutLengthPercentage::Calc(id) => {
            let value = input.css_math.iter().find(|value| value.id == id).ok_or(
                LayoutError::MissingCssMath {
                    node: parent,
                    property: "row-gap",
                    id,
                },
            )?;
            if value.property != LayoutCssMathProperty::RowGap || value.node_id != parent {
                return Err(LayoutError::CssMathBindingMismatch {
                    node: parent,
                    property: "row-gap",
                    id,
                });
            }
            value.resolve(basis).map_err(|reason| value.error(reason))?
        }
        LayoutLengthPercentage::Auto => {
            return Err(unsupported(parent, "row-gap은 auto일 수 없습니다"));
        }
    };
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(unsupported(
            parent,
            "row-gap이 유한한 0 이상 CSS px가 아닙니다",
        ))
    }
}

pub(super) fn cross_margins(
    parent: NodeId,
    child: NodeId,
    tree: &CalcLayoutTree,
) -> Result<(f32, f32), LayoutError> {
    let layout = tree
        .layout_for(child)
        .ok_or(LayoutError::MissingComputedLayout(child))?;
    finite_margin(parent, layout.margin.top)?;
    finite_margin(parent, layout.margin.bottom)?;
    Ok((layout.margin.top, layout.margin.bottom))
}

pub(super) fn has_cross_axis_auto_margin(style: LayoutStyle) -> bool {
    matches!(style.margin.top, LayoutLengthPercentage::Auto)
        || matches!(style.margin.bottom, LayoutLengthPercentage::Auto)
}

fn finite_margin(parent: NodeId, margin: f32) -> Result<(), LayoutError> {
    if margin.is_finite() {
        Ok(())
    } else {
        Err(unsupported(
            parent,
            "cross-axis used margin이 유한하지 않습니다",
        ))
    }
}

fn unsupported(node: NodeId, reason: &'static str) -> LayoutError {
    LayoutError::UnsupportedBaseline { node, reason }
}
