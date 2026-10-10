use std::collections::BTreeMap;

use spinon_core::NodeId;
use taffy::geometry::Point;
use taffy::prelude::{
    Dimension, Display, FlexDirection as TaffyFlexDirection, FlexWrap as TaffyFlexWrap,
    LengthPercentage, LengthPercentageAuto, Rect, Size, Style,
};
use taffy::style::{
    AlignContent, AlignContentKeyword, AlignItems, AlignItemsKeyword, AlignSelf,
    AlignmentSafety as TaffyAlignmentSafety, BoxSizing, Direction as TaffyDirection,
    JustifyContent, Overflow,
};

use crate::{
    AlignmentSafety, ContentAlignmentPosition, FlexDirection, FlexWrap, ItemAlignmentPosition,
    JustifyContentPosition, LayoutAlignContent, LayoutAlignItems, LayoutAlignSelf, LayoutBoxSizing,
    LayoutCalcId, LayoutCssMathProperty, LayoutDimension, LayoutDisplay, LayoutError,
    LayoutJustifyContent, LayoutLengthPercentage, LayoutStyle, TextDirection,
};

pub(super) fn to_taffy_style(
    node: NodeId,
    style: LayoutStyle,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<Style, LayoutError> {
    Ok(Style {
        display: match style.display {
            LayoutDisplay::Flex => Display::Flex,
            LayoutDisplay::Block => Display::Block,
            LayoutDisplay::FlowRoot => Display::FlowRoot,
            LayoutDisplay::None => Display::None,
        },
        box_sizing: match style.box_sizing {
            LayoutBoxSizing::BorderBox => BoxSizing::BorderBox,
            LayoutBoxSizing::ContentBox => BoxSizing::ContentBox,
        },
        direction: match style.direction {
            TextDirection::Ltr => TaffyDirection::Ltr,
            TextDirection::Rtl => TaffyDirection::Rtl,
        },
        size: Size {
            width: to_taffy_dimension(
                node,
                LayoutCssMathProperty::Width,
                style.width,
                calc_handles,
            )?,
            height: to_taffy_dimension(
                node,
                LayoutCssMathProperty::Height,
                style.height,
                calc_handles,
            )?,
        },
        min_size: Size {
            width: to_taffy_size_constraint(
                node,
                LayoutCssMathProperty::MinWidth,
                style.min_width,
                calc_handles,
            )?,
            height: to_taffy_size_constraint(
                node,
                LayoutCssMathProperty::MinHeight,
                style.min_height,
                calc_handles,
            )?,
        },
        max_size: Size {
            width: to_taffy_size_constraint(
                node,
                LayoutCssMathProperty::MaxWidth,
                style.max_width,
                calc_handles,
            )?,
            height: to_taffy_size_constraint(
                node,
                LayoutCssMathProperty::MaxHeight,
                style.max_height,
                calc_handles,
            )?,
        },
        aspect_ratio: style.aspect_ratio,
        padding: Rect {
            top: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::PaddingTop,
                style.padding.top,
                calc_handles,
            )?,
            right: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::PaddingRight,
                style.padding.right,
                calc_handles,
            )?,
            bottom: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::PaddingBottom,
                style.padding.bottom,
                calc_handles,
            )?,
            left: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::PaddingLeft,
                style.padding.left,
                calc_handles,
            )?,
        },
        border: Rect {
            top: LengthPercentage::length(style.border.top),
            right: LengthPercentage::length(style.border.right),
            bottom: LengthPercentage::length(style.border.bottom),
            left: LengthPercentage::length(style.border.left),
        },
        gap: Size {
            width: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::ColumnGap,
                style.gap.column,
                calc_handles,
            )?,
            height: to_taffy_length_percentage(
                node,
                LayoutCssMathProperty::RowGap,
                style.gap.row,
                calc_handles,
            )?,
        },
        align_items: Some(to_taffy_align_items(style.align_items)),
        align_self: to_taffy_align_self(style.align_self),
        align_content: style.align_content.map(to_taffy_align_content),
        justify_content: Some(to_taffy_justify_content(
            style.justify_content,
            style.flex_direction,
            style.direction,
        )),
        margin: Rect {
            top: to_taffy_length_percentage_auto(
                node,
                LayoutCssMathProperty::MarginTop,
                style.margin.top,
                calc_handles,
            )?,
            right: to_taffy_length_percentage_auto(
                node,
                LayoutCssMathProperty::MarginRight,
                style.margin.right,
                calc_handles,
            )?,
            bottom: to_taffy_length_percentage_auto(
                node,
                LayoutCssMathProperty::MarginBottom,
                style.margin.bottom,
                calc_handles,
            )?,
            left: to_taffy_length_percentage_auto(
                node,
                LayoutCssMathProperty::MarginLeft,
                style.margin.left,
                calc_handles,
            )?,
        },
        flex_basis: to_taffy_dimension(
            node,
            LayoutCssMathProperty::FlexBasis,
            style.flex_basis,
            calc_handles,
        )?,
        flex_direction: match style.flex_direction {
            FlexDirection::Row => TaffyFlexDirection::Row,
            FlexDirection::Column => TaffyFlexDirection::Column,
            FlexDirection::RowReverse => TaffyFlexDirection::RowReverse,
            FlexDirection::ColumnReverse => TaffyFlexDirection::ColumnReverse,
        },
        flex_wrap: match style.flex_wrap {
            FlexWrap::NoWrap => TaffyFlexWrap::NoWrap,
            FlexWrap::Wrap => TaffyFlexWrap::Wrap,
            FlexWrap::WrapReverse => TaffyFlexWrap::WrapReverse,
        },
        flex_grow: style.flex_grow,
        flex_shrink: style.flex_shrink,
        ..Default::default()
    })
}

fn to_taffy_alignment_safety(safety: Option<AlignmentSafety>) -> TaffyAlignmentSafety {
    match safety {
        Some(AlignmentSafety::Safe) => TaffyAlignmentSafety::Safe,
        Some(AlignmentSafety::Unsafe) | None => TaffyAlignmentSafety::Unsafe,
    }
}

fn to_taffy_item_alignment_keyword(position: ItemAlignmentPosition) -> AlignItemsKeyword {
    match position {
        ItemAlignmentPosition::Start => AlignItemsKeyword::Start,
        ItemAlignmentPosition::End => AlignItemsKeyword::End,
        ItemAlignmentPosition::FlexStart => AlignItemsKeyword::FlexStart,
        ItemAlignmentPosition::FlexEnd => AlignItemsKeyword::FlexEnd,
        ItemAlignmentPosition::SelfStart => AlignItemsKeyword::SelfStart,
        ItemAlignmentPosition::SelfEnd => AlignItemsKeyword::SelfEnd,
        ItemAlignmentPosition::Center => AlignItemsKeyword::Center,
    }
}

fn to_taffy_align_items(value: LayoutAlignItems) -> AlignItems {
    match value {
        LayoutAlignItems::Normal | LayoutAlignItems::Stretch => AlignItems::STRETCH,
        LayoutAlignItems::FlexStart => AlignItems::FLEX_START,
        LayoutAlignItems::FlexEnd => AlignItems::FLEX_END,
        LayoutAlignItems::Center => AlignItems::CENTER,
        // Taffy 0.14는 row first-baseline 알고리즘을 제공합니다. 레이아웃 후 어댑터에서
        // first/last 그룹을 분리하고 last baseline 위치를 보정합니다.
        LayoutAlignItems::FirstBaseline | LayoutAlignItems::LastBaseline => AlignItems::BASELINE,
        LayoutAlignItems::Position { position, safety } => AlignItems {
            keyword: to_taffy_item_alignment_keyword(position),
            safety: to_taffy_alignment_safety(safety),
        },
    }
}

fn to_taffy_align_self(value: LayoutAlignSelf) -> Option<AlignSelf> {
    match value {
        LayoutAlignSelf::Auto => None,
        LayoutAlignSelf::Normal | LayoutAlignSelf::Stretch => Some(AlignSelf::STRETCH),
        LayoutAlignSelf::FlexStart => Some(AlignSelf::FLEX_START),
        LayoutAlignSelf::FlexEnd => Some(AlignSelf::FLEX_END),
        LayoutAlignSelf::Center => Some(AlignSelf::CENTER),
        LayoutAlignSelf::FirstBaseline | LayoutAlignSelf::LastBaseline => Some(AlignSelf::BASELINE),
        LayoutAlignSelf::Position { position, safety } => Some(AlignSelf {
            keyword: to_taffy_item_alignment_keyword(position),
            safety: to_taffy_alignment_safety(safety),
        }),
    }
}

fn to_taffy_content_alignment_keyword(position: ContentAlignmentPosition) -> AlignContentKeyword {
    match position {
        ContentAlignmentPosition::Start => AlignContentKeyword::Start,
        ContentAlignmentPosition::End => AlignContentKeyword::End,
        ContentAlignmentPosition::FlexStart => AlignContentKeyword::FlexStart,
        ContentAlignmentPosition::FlexEnd => AlignContentKeyword::FlexEnd,
        ContentAlignmentPosition::Center => AlignContentKeyword::Center,
    }
}

fn to_taffy_align_content(value: LayoutAlignContent) -> AlignContent {
    match value {
        LayoutAlignContent::Normal | LayoutAlignContent::Stretch => AlignContent::STRETCH,
        LayoutAlignContent::FlexStart => AlignContent::FLEX_START,
        LayoutAlignContent::FlexEnd => AlignContent::FLEX_END,
        LayoutAlignContent::Center => AlignContent::CENTER,
        LayoutAlignContent::SpaceBetween => AlignContent::SPACE_BETWEEN,
        LayoutAlignContent::SpaceAround => AlignContent::SPACE_AROUND,
        LayoutAlignContent::SpaceEvenly => AlignContent::SPACE_EVENLY,
        LayoutAlignContent::FirstBaseline => AlignContent::FLEX_START,
        LayoutAlignContent::Position { position, safety } => AlignContent {
            keyword: to_taffy_content_alignment_keyword(position),
            safety: to_taffy_alignment_safety(safety),
        },
    }
}

fn to_taffy_justify_content_keyword(
    position: JustifyContentPosition,
    direction: TextDirection,
    flex_direction: FlexDirection,
) -> AlignContentKeyword {
    match position {
        JustifyContentPosition::Start => AlignContentKeyword::Start,
        JustifyContentPosition::End => AlignContentKeyword::End,
        JustifyContentPosition::FlexStart => AlignContentKeyword::FlexStart,
        JustifyContentPosition::FlexEnd => AlignContentKeyword::FlexEnd,
        JustifyContentPosition::Center => AlignContentKeyword::Center,
        JustifyContentPosition::Left | JustifyContentPosition::Right
            if matches!(
                flex_direction,
                FlexDirection::Row | FlexDirection::RowReverse
            ) =>
        {
            let logical_start_is_left = direction == TextDirection::Ltr;
            let aligns_to_start = match position {
                JustifyContentPosition::Left => logical_start_is_left,
                JustifyContentPosition::Right => !logical_start_is_left,
                _ => unreachable!(),
            };
            if aligns_to_start {
                AlignContentKeyword::Start
            } else {
                AlignContentKeyword::End
            }
        }
        JustifyContentPosition::Left | JustifyContentPosition::Right => AlignContentKeyword::Start,
    }
}

fn to_taffy_justify_content(
    value: LayoutJustifyContent,
    flex_direction: FlexDirection,
    direction: TextDirection,
) -> JustifyContent {
    match value {
        LayoutJustifyContent::Normal | LayoutJustifyContent::FlexStart => {
            JustifyContent::FLEX_START
        }
        LayoutJustifyContent::Stretch => JustifyContent::STRETCH,
        LayoutJustifyContent::FlexEnd => JustifyContent::FLEX_END,
        LayoutJustifyContent::Center => JustifyContent::CENTER,
        LayoutJustifyContent::SpaceBetween => JustifyContent::SPACE_BETWEEN,
        LayoutJustifyContent::SpaceAround => JustifyContent::SPACE_AROUND,
        LayoutJustifyContent::SpaceEvenly => JustifyContent::SPACE_EVENLY,
        LayoutJustifyContent::Position { position, safety } => JustifyContent {
            keyword: to_taffy_justify_content_keyword(position, direction, flex_direction),
            safety: to_taffy_alignment_safety(safety),
        },
    }
}

pub(super) fn viewport_block_containing_style(viewport: crate::Viewport) -> Style {
    Style {
        display: Display::Block,
        size: Size {
            width: Dimension::length(viewport.width),
            height: Dimension::length(viewport.height),
        },
        overflow: Point {
            x: Overflow::Hidden,
            y: Overflow::Hidden,
        },
        ..Default::default()
    }
}

fn calc_handle(
    node: NodeId,
    property: LayoutCssMathProperty,
    id: LayoutCalcId,
    handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<*const (), LayoutError> {
    handles
        .get(&id)
        .copied()
        .ok_or(LayoutError::MissingCssMath {
            node,
            property: property.name(),
            id,
        })
}

fn to_taffy_length_percentage(
    node: NodeId,
    property: LayoutCssMathProperty,
    value: LayoutLengthPercentage,
    handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<LengthPercentage, LayoutError> {
    match value {
        LayoutLengthPercentage::Auto => Err(LayoutError::InvalidStyle {
            node,
            field: property.name(),
        }),
        LayoutLengthPercentage::LengthPx(value) => Ok(LengthPercentage::length(value)),
        LayoutLengthPercentage::Percentage(value) => Ok(LengthPercentage::percent(value)),
        LayoutLengthPercentage::Calc(id) => Ok(LengthPercentage::calc(calc_handle(
            node, property, id, handles,
        )?)),
    }
}

fn to_taffy_length_percentage_auto(
    node: NodeId,
    property: LayoutCssMathProperty,
    value: LayoutLengthPercentage,
    handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<LengthPercentageAuto, LayoutError> {
    match value {
        LayoutLengthPercentage::Auto => Ok(LengthPercentageAuto::auto()),
        LayoutLengthPercentage::LengthPx(value) => Ok(LengthPercentageAuto::length(value)),
        LayoutLengthPercentage::Percentage(value) => Ok(LengthPercentageAuto::percent(value)),
        LayoutLengthPercentage::Calc(id) => Ok(LengthPercentageAuto::calc(calc_handle(
            node, property, id, handles,
        )?)),
    }
}

fn to_taffy_dimension(
    node: NodeId,
    property: LayoutCssMathProperty,
    dimension: LayoutDimension,
    handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<Dimension, LayoutError> {
    match dimension {
        LayoutDimension::Auto => Ok(Dimension::auto()),
        LayoutDimension::Fixed(value) => Ok(Dimension::length(value)),
        LayoutDimension::Percent(value) => Ok(Dimension::percent(value)),
        LayoutDimension::Calc(id) => Ok(Dimension::calc(calc_handle(node, property, id, handles)?)),
    }
}

fn to_taffy_size_constraint(
    node: NodeId,
    property: LayoutCssMathProperty,
    dimension: LayoutDimension,
    handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<LengthPercentageAuto, LayoutError> {
    match dimension {
        LayoutDimension::Auto => Ok(LengthPercentageAuto::auto()),
        LayoutDimension::Fixed(value) => Ok(LengthPercentageAuto::length(value)),
        LayoutDimension::Percent(value) => Ok(LengthPercentageAuto::percent(value)),
        LayoutDimension::Calc(id) => Ok(LengthPercentageAuto::calc(calc_handle(
            node, property, id, handles,
        )?)),
    }
}
