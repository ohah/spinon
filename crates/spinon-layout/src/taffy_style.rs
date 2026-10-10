use std::collections::BTreeMap;

use spinon_core::NodeId;
use taffy::geometry::Point;
use taffy::prelude::{
    AlignItems, Dimension, Display, FlexDirection as TaffyFlexDirection, FlexWrap as TaffyFlexWrap,
    JustifyContent, LengthPercentage, LengthPercentageAuto, Rect, Size, Style,
};
use taffy::style::{BoxSizing, Direction as TaffyDirection, Overflow};

use crate::{
    FlexDirection, FlexWrap, LayoutAlignItems, LayoutBoxSizing, LayoutCalcId,
    LayoutCssMathProperty, LayoutDimension, LayoutDisplay, LayoutError, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutStyle, TextDirection,
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
        align_items: Some(match style.align_items {
            LayoutAlignItems::Stretch => AlignItems::STRETCH,
            LayoutAlignItems::FlexStart => AlignItems::FLEX_START,
            LayoutAlignItems::FlexEnd => AlignItems::FLEX_END,
            LayoutAlignItems::Center => AlignItems::CENTER,
        }),
        justify_content: Some(match style.justify_content {
            LayoutJustifyContent::FlexStart => JustifyContent::FLEX_START,
            LayoutJustifyContent::FlexEnd => JustifyContent::FLEX_END,
            LayoutJustifyContent::Center => JustifyContent::CENTER,
            LayoutJustifyContent::SpaceBetween => JustifyContent::SPACE_BETWEEN,
            LayoutJustifyContent::SpaceAround => JustifyContent::SPACE_AROUND,
            LayoutJustifyContent::SpaceEvenly => JustifyContent::SPACE_EVENLY,
        }),
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
        },
        flex_wrap: match style.flex_wrap {
            FlexWrap::NoWrap => TaffyFlexWrap::NoWrap,
            FlexWrap::Wrap => TaffyFlexWrap::Wrap,
        },
        flex_grow: style.flex_grow,
        flex_shrink: style.flex_shrink,
        ..Default::default()
    })
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
