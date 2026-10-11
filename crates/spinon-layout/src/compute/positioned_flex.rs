use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::positioning::collect_positioned_owners;
use crate::{LayoutError, LayoutFrame, LayoutInput};

pub(super) fn absolute_static_position_adjustments(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    visual_frames: &BTreeMap<NodeId, LayoutFrame>,
    flow_frames: &BTreeMap<NodeId, LayoutFrame>,
) -> Result<BTreeMap<NodeId, (f32, f32)>, LayoutError> {
    let owners = collect_positioned_owners(input, index);
    let source_parents = input
        .nodes
        .iter()
        .flat_map(|node| node.children.iter().map(move |child| (*child, node.id)))
        .collect::<BTreeMap<_, _>>();
    let mut adjustments = BTreeMap::new();
    for (&id, positioning) in &input.positioning {
        if positioning.position != crate::LayoutPosition::Absolute {
            continue;
        }
        let Some(flow_frame) = flow_frames.get(&id).copied() else {
            return Err(LayoutError::MissingComputedLayout(id));
        };
        let Some(visual_frame) = visual_frames.get(&id).copied() else {
            return Err(LayoutError::MissingComputedLayout(id));
        };
        if let Some(&source_parent) = source_parents.get(&id)
            && input.nodes[index[&source_parent]].style.display == crate::LayoutDisplay::Flex
        {
            let Some(parent_frame) = visual_frames.get(&source_parent).copied() else {
                return Err(LayoutError::MissingComputedLayout(source_parent));
            };
            let parent_style = input.nodes[index[&source_parent]].style;
            let child_style = input.nodes[index[&id]].style;
            let (static_x, static_y) =
                flex_static_position(id, parent_style, child_style, parent_frame, visual_frame)?;
            let delta_x = if positioning.inset.left == crate::LayoutLengthPercentage::Auto
                && positioning.inset.right == crate::LayoutLengthPercentage::Auto
            {
                static_x - visual_frame.x
            } else {
                0.0
            };
            let delta_y = if positioning.inset.top == crate::LayoutLengthPercentage::Auto
                && positioning.inset.bottom == crate::LayoutLengthPercentage::Auto
            {
                static_y - visual_frame.y
            } else {
                0.0
            };
            if !delta_x.is_finite() || !delta_y.is_finite() {
                return Err(LayoutError::NonFiniteFrame(id));
            }
            if delta_x != 0.0 || delta_y != 0.0 {
                adjustments.insert(id, (delta_x, delta_y));
            }
            continue;
        }
        let owner_delta = match owners[&id] {
            crate::PositionedContainingBlockOwner::Viewport => (0.0, 0.0),
            crate::PositionedContainingBlockOwner::Node(owner) => {
                let Some(flow_owner) = flow_frames.get(&owner) else {
                    return Err(LayoutError::MissingComputedLayout(owner));
                };
                let Some(visual_owner) = visual_frames.get(&owner) else {
                    return Err(LayoutError::MissingComputedLayout(owner));
                };
                (visual_owner.x - flow_owner.x, visual_owner.y - flow_owner.y)
            }
            crate::PositionedContainingBlockOwner::NoBox => continue,
        };
        let delta_x = if positioning.inset.left == crate::LayoutLengthPercentage::Auto
            && positioning.inset.right == crate::LayoutLengthPercentage::Auto
        {
            flow_frame.x + owner_delta.0 - visual_frame.x
        } else {
            0.0
        };
        let delta_y = if positioning.inset.top == crate::LayoutLengthPercentage::Auto
            && positioning.inset.bottom == crate::LayoutLengthPercentage::Auto
        {
            flow_frame.y + owner_delta.1 - visual_frame.y
        } else {
            0.0
        };
        if !delta_x.is_finite() || !delta_y.is_finite() {
            return Err(LayoutError::NonFiniteFrame(id));
        }
        if delta_x != 0.0 || delta_y != 0.0 {
            adjustments.insert(id, (delta_x, delta_y));
        }
    }
    Ok(adjustments)
}

fn flex_static_position(
    node: NodeId,
    parent: crate::LayoutStyle,
    child: crate::LayoutStyle,
    parent_frame: LayoutFrame,
    child_frame: LayoutFrame,
) -> Result<(f32, f32), LayoutError> {
    use crate::{FlexDirection, FlexWrap, LayoutDimension};

    if !matches!(parent.width, LayoutDimension::Fixed(_))
        || !matches!(parent.height, LayoutDimension::Fixed(_))
        || !matches!(child.width, LayoutDimension::Fixed(_))
        || !matches!(child.height, LayoutDimension::Fixed(_))
    {
        return Err(LayoutError::UnsupportedPositioning {
            node,
            reason: "C10.3.5 static position은 fixed-size Flex parent와 element child만 지원합니다",
        });
    }
    let border = parent.border;
    let padding_left = static_padding(node, parent.padding.left)?;
    let padding_right = static_padding(node, parent.padding.right)?;
    let padding_top = static_padding(node, parent.padding.top)?;
    let padding_bottom = static_padding(node, parent.padding.bottom)?;
    let content_left = border.left + padding_left;
    let content_top = border.top + padding_top;
    let content_width =
        (parent_frame.width - border.left - border.right - padding_left - padding_right).max(0.0);
    let content_height =
        (parent_frame.height - border.top - border.bottom - padding_top - padding_bottom).max(0.0);
    let margin_left = static_margin(node, child.margin.left)?;
    let margin_right = static_margin(node, child.margin.right)?;
    let margin_top = static_margin(node, child.margin.top)?;
    let margin_bottom = static_margin(node, child.margin.bottom)?;
    let cross_reverse = parent.flex_wrap == FlexWrap::WrapReverse;

    let (x, y) = match parent.flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            let main_offset = justify_offset(
                node,
                parent.justify_content,
                AxisMetrics::new(
                    content_width,
                    child_frame.width,
                    margin_left,
                    margin_right,
                    parent.flex_direction == FlexDirection::RowReverse,
                ),
                true,
            )?;
            let cross_offset = align_offset(
                node,
                parent.align_items,
                child.align_self,
                AxisMetrics::new(
                    content_height,
                    child_frame.height,
                    margin_top,
                    margin_bottom,
                    cross_reverse,
                ),
            )?;
            (
                parent_frame.x + content_left + main_offset + margin_left,
                parent_frame.y + content_top + cross_offset + margin_top,
            )
        }
        FlexDirection::Column | FlexDirection::ColumnReverse => {
            let main_offset = justify_offset(
                node,
                parent.justify_content,
                AxisMetrics::new(
                    content_height,
                    child_frame.height,
                    margin_top,
                    margin_bottom,
                    parent.flex_direction == FlexDirection::ColumnReverse,
                ),
                false,
            )?;
            let cross_offset = align_offset(
                node,
                parent.align_items,
                child.align_self,
                AxisMetrics::new(
                    content_width,
                    child_frame.width,
                    margin_left,
                    margin_right,
                    cross_reverse,
                ),
            )?;
            (
                parent_frame.x + content_left + cross_offset + margin_left,
                parent_frame.y + content_top + main_offset + margin_top,
            )
        }
    };
    if [x, y].into_iter().all(f32::is_finite) {
        Ok((x, y))
    } else {
        Err(LayoutError::NonFiniteFrame(node))
    }
}

fn static_padding(node: NodeId, value: crate::LayoutLengthPercentage) -> Result<f32, LayoutError> {
    match value {
        crate::LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0 => {
            Ok(value)
        }
        crate::LayoutLengthPercentage::LengthPx(_) => Err(LayoutError::InvalidStyle {
            node,
            field: "padding",
        }),
        _ => Err(LayoutError::UnsupportedPositioning {
            node,
            reason: "C10.3.5 static position은 고정 CSS px padding만 지원합니다",
        }),
    }
}

fn static_margin(node: NodeId, value: crate::LayoutLengthPercentage) -> Result<f32, LayoutError> {
    match value {
        crate::LayoutLengthPercentage::Auto => Ok(0.0),
        crate::LayoutLengthPercentage::LengthPx(value) if value.is_finite() => Ok(value),
        crate::LayoutLengthPercentage::LengthPx(_) => Err(LayoutError::InvalidStyle {
            node,
            field: "margin",
        }),
        _ => Err(LayoutError::UnsupportedPositioning {
            node,
            reason: "C10.3.5 static position은 auto 또는 고정 CSS px margin만 지원합니다",
        }),
    }
}

#[derive(Clone, Copy)]
struct AxisMetrics {
    content_size: f32,
    child_size: f32,
    margin_start: f32,
    margin_end: f32,
    reverse: bool,
}

impl AxisMetrics {
    fn new(
        content_size: f32,
        child_size: f32,
        margin_start: f32,
        margin_end: f32,
        reverse: bool,
    ) -> Self {
        Self {
            content_size,
            child_size,
            margin_start,
            margin_end,
            reverse,
        }
    }

    fn free_space(self) -> f32 {
        self.content_size - self.child_size - self.margin_start - self.margin_end
    }

    fn flex_start(self) -> f32 {
        if self.reverse { self.free_space() } else { 0.0 }
    }

    fn flex_end(self) -> f32 {
        if self.reverse { 0.0 } else { self.free_space() }
    }

    fn positional(self, position: crate::ItemAlignmentPosition) -> f32 {
        use crate::ItemAlignmentPosition;

        match position {
            ItemAlignmentPosition::Start | ItemAlignmentPosition::SelfStart => 0.0,
            ItemAlignmentPosition::End | ItemAlignmentPosition::SelfEnd => self.free_space(),
            ItemAlignmentPosition::FlexStart => self.flex_start(),
            ItemAlignmentPosition::FlexEnd => self.flex_end(),
            ItemAlignmentPosition::Center => self.free_space() / 2.0,
        }
    }
}

fn justify_offset(
    node: NodeId,
    alignment: crate::LayoutJustifyContent,
    axis: AxisMetrics,
    horizontal_axis: bool,
) -> Result<f32, LayoutError> {
    use crate::{JustifyContentPosition, LayoutJustifyContent};

    let free = axis.free_space();
    let offset = match alignment {
        LayoutJustifyContent::Normal
        | LayoutJustifyContent::Stretch
        | LayoutJustifyContent::FlexStart => axis.flex_start(),
        LayoutJustifyContent::FlexEnd => axis.flex_end(),
        LayoutJustifyContent::Center => free / 2.0,
        // Chrome 154는 Flex static-position 대상 하나의 분배 정렬을 음수 여유 공간에서도
        // 가운데에 둡니다. 일반 in-flow Flex 분배의 fallback과 같은 규칙으로 취급하지 않습니다.
        LayoutJustifyContent::SpaceAround | LayoutJustifyContent::SpaceEvenly => free / 2.0,
        LayoutJustifyContent::SpaceBetween => axis.flex_start(),
        LayoutJustifyContent::Position {
            position,
            safety: _safety,
        } => {
            let physical = match position {
                JustifyContentPosition::Start => 0.0,
                JustifyContentPosition::End => free,
                JustifyContentPosition::FlexStart => axis.flex_start(),
                JustifyContentPosition::FlexEnd => axis.flex_end(),
                JustifyContentPosition::Center => free / 2.0,
                JustifyContentPosition::Left if horizontal_axis => 0.0,
                JustifyContentPosition::Right if horizontal_axis => free,
                JustifyContentPosition::Left | JustifyContentPosition::Right => {
                    return Err(LayoutError::UnsupportedPositioning {
                        node,
                        reason: "C10.3.5 justify-content:left/right는 세로 주축에서 미지원입니다",
                    });
                }
            };
            // Chrome 154의 absolute Flex static position은 safe/unsafe를 구분하지 않습니다.
            physical
        }
    };
    if offset.is_finite() {
        Ok(offset)
    } else {
        Err(LayoutError::NonFiniteFrame(node))
    }
}

fn align_offset(
    node: NodeId,
    parent_alignment: crate::LayoutAlignItems,
    child_alignment: crate::LayoutAlignSelf,
    axis: AxisMetrics,
) -> Result<f32, LayoutError> {
    use crate::{LayoutAlignItems, LayoutAlignSelf};

    let alignment = match child_alignment {
        LayoutAlignSelf::Auto => parent_alignment,
        LayoutAlignSelf::Normal | LayoutAlignSelf::Stretch | LayoutAlignSelf::FlexStart => {
            return Ok(axis.flex_start());
        }
        LayoutAlignSelf::FlexEnd => {
            return Ok(axis.flex_end());
        }
        LayoutAlignSelf::Center => {
            return Ok(axis.free_space() / 2.0);
        }
        LayoutAlignSelf::FirstBaseline | LayoutAlignSelf::LastBaseline => {
            return Err(LayoutError::UnsupportedPositioning {
                node,
                reason: "C10.3.5 baseline align-self는 미지원입니다",
            });
        }
        LayoutAlignSelf::Position {
            position,
            safety: _safety,
        } => {
            let offset = axis.positional(position);
            return Ok(offset);
        }
    };
    match alignment {
        LayoutAlignItems::Normal | LayoutAlignItems::Stretch | LayoutAlignItems::FlexStart => {
            Ok(axis.flex_start())
        }
        LayoutAlignItems::FlexEnd => Ok(axis.flex_end()),
        LayoutAlignItems::Center => Ok(axis.free_space() / 2.0),
        LayoutAlignItems::FirstBaseline | LayoutAlignItems::LastBaseline => {
            Err(LayoutError::UnsupportedPositioning {
                node,
                reason: "C10.3.5 baseline align-items는 미지원입니다",
            })
        }
        LayoutAlignItems::Position {
            position,
            safety: _safety,
        } => {
            let offset = axis.positional(position);
            Ok(offset)
        }
    }
}
