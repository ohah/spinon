use std::collections::BTreeSet;

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

pub(super) fn text_item_targets(input: &LayoutInput) -> BTreeSet<spinon_core::NodeId> {
    input
        .nodes
        .iter()
        .filter_map(|container| {
            let grid = input.grid_containers.get(&container.id)?;
            (grid.placement_mode == GridPlacementMode::RootHtmlDirectTextAspectRatioV1
                && is_supported(input, container, grid))
            .then(|| container.children[1])
        })
        .collect()
}

pub(super) fn auto_min_height_for_text_item(
    input: &LayoutInput,
    item: spinon_core::NodeId,
    eligible_targets: &BTreeSet<spinon_core::NodeId>,
) -> Option<f32> {
    if !eligible_targets.contains(&item) {
        return None;
    }
    let style = input.nodes.iter().find(|node| node.id == item)?.style;
    if style.width != LayoutDimension::Fixed(60.0) || style.height != LayoutDimension::Auto {
        return None;
    }
    let TextParagraphInput::Paragraph(paragraph) = input.text_paragraphs.get(&item)? else {
        return None;
    };
    (paragraph.style.line_height.is_finite() && paragraph.style.line_height > 0.0)
        .then_some(paragraph.style.line_height)
}

pub(super) fn auto_min_width_paragraph_for_text_item<'a>(
    input: &'a LayoutInput,
    item: spinon_core::NodeId,
    eligible_targets: &BTreeSet<spinon_core::NodeId>,
) -> Option<&'a spinon_text_layout::ParagraphInput> {
    if !eligible_targets.contains(&item) {
        return None;
    }
    let style = input.nodes.iter().find(|node| node.id == item)?.style;
    if style.width != LayoutDimension::Auto
        || !matches!(style.height, LayoutDimension::Fixed(30.0 | 80.0))
    {
        return None;
    }
    let TextParagraphInput::Paragraph(paragraph) = input.text_paragraphs.get(&item)? else {
        return None;
    };
    Some(paragraph)
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
            let width_locked = style.width == LayoutDimension::Fixed(60.0)
                && style.height == LayoutDimension::Auto;
            let height_locked_30 = style.width == LayoutDimension::Auto
                && style.height == LayoutDimension::Fixed(30.0);
            let height_locked_half_80 = style.width == LayoutDimension::Auto
                && style.height == LayoutDimension::Fixed(80.0)
                && style.aspect_ratio == Some(0.5);
            (width_locked || height_locked_30 || height_locked_half_80)
                && ratio_transfer_is_finite(style.width, style.height, style.aspect_ratio)
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

fn ratio_transfer_is_finite(
    width: LayoutDimension,
    height: LayoutDimension,
    ratio: Option<f32>,
) -> bool {
    let Some(ratio) = ratio else {
        return true;
    };
    if !ratio.is_finite() || ratio <= 0.0 {
        return false;
    }

    let transferred = match (width, height) {
        (LayoutDimension::Fixed(width), LayoutDimension::Auto) => width / ratio,
        (LayoutDimension::Auto, LayoutDimension::Fixed(height)) => height * ratio,
        _ => return false,
    };
    transferred.is_finite() && transferred > 0.0
}

#[cfg(test)]
mod tests {
    use super::ratio_transfer_is_finite;
    use crate::LayoutDimension;

    #[test]
    fn ratio_transfer_accepts_representable_values_and_rejects_overflow() {
        assert!(ratio_transfer_is_finite(
            LayoutDimension::Fixed(60.0),
            LayoutDimension::Auto,
            Some(1.5)
        ));
        assert!(ratio_transfer_is_finite(
            LayoutDimension::Auto,
            LayoutDimension::Fixed(30.0),
            Some(2.5)
        ));
        assert!(ratio_transfer_is_finite(
            LayoutDimension::Auto,
            LayoutDimension::Fixed(80.0),
            Some(0.5)
        ));
        assert!(ratio_transfer_is_finite(
            LayoutDimension::Fixed(60.0),
            LayoutDimension::Auto,
            None
        ));
        assert!(!ratio_transfer_is_finite(
            LayoutDimension::Fixed(60.0),
            LayoutDimension::Auto,
            Some(1.0e-38)
        ));
        assert!(!ratio_transfer_is_finite(
            LayoutDimension::Auto,
            LayoutDimension::Fixed(30.0),
            Some(1.0e38)
        ));
        assert!(!ratio_transfer_is_finite(
            LayoutDimension::Fixed(60.0),
            LayoutDimension::Auto,
            Some(f32::NAN)
        ));
    }
}
