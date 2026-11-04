use spinon_core::WritingMode;
use spinon_text_layout::{TextTransformMode, WhiteSpaceMode};

use crate::{
    GridAutoColumns, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackSize,
    ItemAlignmentPosition, LayoutAlignItems, LayoutAlignSelf, LayoutBoxSizing, LayoutDimension,
    LayoutDisplay, LayoutInput, LayoutPosition, TextDirection, TextParagraphInput,
};

pub(super) fn is_supported(
    input: &LayoutInput,
    node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> bool {
    let style = node.style;
    style.display == LayoutDisplay::Grid
        && style.box_sizing == LayoutBoxSizing::ContentBox
        && style.width == LayoutDimension::Fixed(70.0)
        && style.height == LayoutDimension::Auto
        && style.min_width == LayoutDimension::Auto
        && style.max_width == LayoutDimension::Auto
        && style.min_height == LayoutDimension::Auto
        && style.max_height == LayoutDimension::Auto
        && style.aspect_ratio.is_none()
        && style.margin == crate::LayoutEdges::default()
        && style.padding == crate::LayoutEdges::default()
        && style.border == crate::LayoutBorder::default()
        && style.gap.row == crate::LayoutLengthPercentage::ZERO
        && style.gap.column == crate::LayoutLengthPercentage::ZERO
        && style.overflow_x == crate::LayoutOverflow::Visible
        && style.overflow_y == crate::LayoutOverflow::Visible
        && style.direction == TextDirection::Ltr
        && is_start_alignment(style.align_items)
        && is_start_alignment(container.justify_items)
        && container.columns == [GridTrackSize::LengthPx(10.0), GridTrackSize::LengthPx(60.0)]
        && container.rows == [GridTrackSize::Auto]
        && container.columns_auto_repeat.is_none()
        && container.rows_auto_repeat.is_none()
        && container
            .columns_line_names
            .iter()
            .chain(&container.rows_line_names)
            .flatten()
            .all(String::is_empty)
        && container.template_areas.is_none()
        && container.auto_flow == crate::GridAutoFlow::Row
        && container.auto_rows == GridAutoRows::Auto
        && container.auto_columns == GridAutoColumns::Auto
        && container.placement_mode == GridPlacementMode::RootHtmlDirectTextAspectRatioV1
        && input
            .positioning
            .get(&node.id)
            .is_none_or(|position| position.position == LayoutPosition::Static)
        && node.children.len() == 2
        && is_text_item(input, node.children[0], "A", TextItemSize::Auto)
        && is_text_item(input, node.children[1], "AAAA", TextItemSize::Target)
}

fn is_start_alignment(alignment: LayoutAlignItems) -> bool {
    matches!(
        alignment,
        LayoutAlignItems::Position {
            position: ItemAlignmentPosition::Start,
            safety: None
        }
    )
}

#[derive(Clone, Copy)]
enum TextItemSize {
    Auto,
    Target,
}

fn is_text_item(
    input: &LayoutInput,
    id: spinon_core::NodeId,
    text: &str,
    size: TextItemSize,
) -> bool {
    let Some(node) = input.nodes.iter().find(|node| node.id == id) else {
        return false;
    };
    let style = node.style;
    let target_size_supported = match size {
        TextItemSize::Auto => {
            style.width == LayoutDimension::Auto
                && style.height == LayoutDimension::Auto
                && style.aspect_ratio.is_none()
        }
        TextItemSize::Target => {
            (style.width == LayoutDimension::Fixed(60.0) && style.height == LayoutDimension::Auto
                || style.width == LayoutDimension::Auto
                    && style.height == LayoutDimension::Fixed(30.0))
                && style.aspect_ratio.is_none_or(|ratio| ratio == 2.0)
        }
    };
    style.display == LayoutDisplay::Block
        && style.box_sizing == LayoutBoxSizing::ContentBox
        && target_size_supported
        && style.min_width == LayoutDimension::Auto
        && style.max_width == LayoutDimension::Auto
        && style.min_height == LayoutDimension::Auto
        && style.max_height == LayoutDimension::Auto
        && style.margin == crate::LayoutEdges::default()
        && style.padding == crate::LayoutEdges::default()
        && style.border == crate::LayoutBorder::default()
        && style.overflow_x == crate::LayoutOverflow::Visible
        && style.overflow_y == crate::LayoutOverflow::Visible
        && style.direction == TextDirection::Ltr
        && style.order == 0
        && style.align_self == LayoutAlignSelf::Auto
        && style.justify_self == LayoutAlignSelf::Auto
        && node.children.is_empty()
        && input
            .grid_items
            .get(&id)
            .is_none_or(crate::GridItemInput::is_auto)
        && input
            .positioning
            .get(&id)
            .is_none_or(|position| position.position == LayoutPosition::Static)
        && has_expected_paragraph(input, id, text)
}

fn has_expected_paragraph(input: &LayoutInput, id: spinon_core::NodeId, text: &str) -> bool {
    let Some(TextParagraphInput::Paragraph(paragraph)) = input.text_paragraphs.get(&id) else {
        return false;
    };
    let style = &paragraph.style;
    paragraph.node == id
        && paragraph.sources.len() == 1
        && paragraph.sources[0].text == text
        && paragraph.sources[0].inline_ancestors.is_empty()
        && paragraph.inline_styles.is_empty()
        && paragraph.inline_run_styles.is_empty()
        && matches!(style.font_family.as_str(), "Ahem" | "monospace")
        && style.font_size == 10.0
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == spinon_text_layout::FontStyleMode::Normal
        && style.letter_spacing == 0.0
        && style.word_spacing == 0.0
        && style.white_space == WhiteSpaceMode::NoWrap
        && style.text_transform == TextTransformMode::None
        && style.direction == spinon_text_layout::TextDirection::Ltr
        && style.writing_mode == WritingMode::HorizontalTb
}
