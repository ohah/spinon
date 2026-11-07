use super::*;

pub(super) fn validate_root_html_fit_content_auto_rows_contract(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    node: NodeId,
    style: LayoutStyle,
    container: &crate::GridContainerInput,
) -> Result<(), LayoutError> {
    let unsupported_rows = || LayoutError::UnsupportedGridAutoRows {
        node,
        reason: "Root HTML fit-content implicit rows require one finite non-negative px or percentage cap",
    };
    let crate::GridAutoRows::TrackList(tracks) = &container.auto_rows else {
        return Err(unsupported_rows());
    };
    if !matches!(
        tracks.as_slice(),
        [GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(cap)
                | GridTrackLengthPercentage::Percentage(cap)
        )] if cap.is_finite() && *cap >= 0.0
    ) {
        return Err(unsupported_rows());
    }

    let exact_columns = matches!(
        container.columns.as_slice(),
        [GridTrackSize::LengthPx(value)] if *value == 40.0
    );
    let exact_rows = matches!(
        container.rows.as_slice(),
        [GridTrackSize::LengthPx(value)] if *value == 10.0
    );
    let line_names_are_empty = container
        .columns_line_names
        .iter()
        .chain(&container.rows_line_names)
        .flatten()
        .all(String::is_empty);
    if !exact_columns
        || !exact_rows
        || !line_names_are_empty
        || container.columns_auto_repeat.is_some()
        || container.rows_auto_repeat.is_some()
        || container.template_areas.is_some()
        || container.auto_flow != GridAutoFlow::Row
        || container.auto_columns != GridAutoColumns::Auto
    {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content implicit row track shape",
        });
    }

    let grid_style_is_supported = style.display == LayoutDisplay::Grid
        && style.width == LayoutDimension::Fixed(40.0)
        && style.height == LayoutDimension::Fixed(100.0)
        && style.box_sizing == LayoutBoxSizing::BorderBox
        && style.min_width == LayoutDimension::Auto
        && style.max_width == LayoutDimension::Auto
        && style.min_height == LayoutDimension::Auto
        && style.max_height == LayoutDimension::Auto
        && style.aspect_ratio.is_none()
        && style.direction == crate::TextDirection::Ltr
        && style.overflow_x == crate::LayoutOverflow::Visible
        && style.overflow_y == crate::LayoutOverflow::Visible
        && style.margin == crate::LayoutEdges::default()
        && style.padding == crate::LayoutEdges::default()
        && style.border == crate::LayoutBorder::default()
        && style.gap.row == LayoutLengthPercentage::ZERO
        && style.gap.column == LayoutLengthPercentage::ZERO
        && style.align_items == crate::LayoutAlignItems::Normal
        && style.align_content.is_some_and(|value| {
            root_html_intrinsic_align_content_is_supported(
                value,
                GridPlacementMode::RootHtmlIntrinsicTrackSizingV1,
            )
        })
        && matches!(
            style.justify_content,
            crate::LayoutJustifyContent::Normal
                | crate::LayoutJustifyContent::FlexStart
                | crate::LayoutJustifyContent::Position {
                    position: crate::JustifyContentPosition::Start,
                    safety: None,
                }
        )
        && container.justify_items == crate::LayoutAlignItems::Normal
        && input
            .positioning
            .get(&node)
            .is_none_or(|position| position.position == LayoutPosition::Static);
    if !grid_style_is_supported {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content implicit row container profile",
        });
    }

    let hidden = crate::grid_placement::hidden_subtrees(input, index);
    let grid_node = &input.nodes[index[&node]];
    let visible_children = grid_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden.get(child).copied().unwrap_or(true))
        .collect::<Vec<_>>();
    let child_has_no_text = |child: NodeId| !has_non_collapsible_text(input, child);
    let child_has_auto_placement = |child: NodeId| {
        input
            .grid_items
            .get(&child)
            .is_none_or(crate::GridItemInput::is_auto)
    };
    let child_is_static = |child: NodeId| {
        input
            .positioning
            .get(&child)
            .is_none_or(|position| position.position == LayoutPosition::Static)
    };
    if visible_children.len() != 3 || has_non_collapsible_text(input, node) {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content implicit row item count",
        });
    }

    let first_id = visible_children[0];
    let middle_id = visible_children[1];
    let probe_id = visible_children[2];
    let first = &input.nodes[index[&first_id]];
    let middle = &input.nodes[index[&middle_id]];
    let probe = &input.nodes[index[&probe_id]];
    let fixed_item_is_supported = |item: &crate::LayoutNode, height| {
        item.style.display == LayoutDisplay::Block
            && item.style.width == LayoutDimension::Auto
            && item.style.height == LayoutDimension::Fixed(height)
            && item.style.min_width == LayoutDimension::Auto
            && item.style.max_width == LayoutDimension::Auto
            && item.style.min_height == LayoutDimension::Auto
            && item.style.max_height == LayoutDimension::Auto
            && item.style.aspect_ratio.is_none()
            && item.style.order == 0
            && item.style.overflow_x == crate::LayoutOverflow::Visible
            && item.style.overflow_y == crate::LayoutOverflow::Visible
            && item.style.margin == crate::LayoutEdges::default()
            && item.style.padding == crate::LayoutEdges::default()
            && item.style.border == crate::LayoutBorder::default()
            && item.style.align_self == crate::LayoutAlignSelf::Auto
            && item.style.justify_self == crate::LayoutAlignSelf::Auto
            && item.children.is_empty()
            && child_has_no_text(item.id)
            && child_has_auto_placement(item.id)
            && child_is_static(item.id)
    };
    let middle_is_supported = middle.style.display == LayoutDisplay::Block
        && middle.style.width == LayoutDimension::Auto
        && middle.style.height == LayoutDimension::Auto
        && middle.style.min_width == LayoutDimension::Auto
        && matches!(
            middle.style.min_height,
            LayoutDimension::Fixed(value) if value == 0.0 || value == 28.0
        )
        && middle.style.max_width == LayoutDimension::Auto
        && middle.style.max_height == LayoutDimension::Auto
        && middle.style.aspect_ratio.is_none()
        && middle.style.order == 0
        && middle.style.overflow_x == crate::LayoutOverflow::Visible
        && middle.style.overflow_y == crate::LayoutOverflow::Visible
        && middle.style.margin == crate::LayoutEdges::default()
        && middle.style.padding == crate::LayoutEdges::default()
        && middle.style.border == crate::LayoutBorder::default()
        && middle.style.align_self == crate::LayoutAlignSelf::Auto
        && middle.style.justify_self == crate::LayoutAlignSelf::Auto
        && middle.children.len() == 1
        && child_has_no_text(middle_id)
        && child_has_auto_placement(middle_id)
        && child_is_static(middle_id);
    if !fixed_item_is_supported(first, 10.0)
        || !middle_is_supported
        || !fixed_item_is_supported(probe, 6.0)
    {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content implicit row item profile",
        });
    }

    let payload_id = middle.children[0];
    let payload = &input.nodes[index[&payload_id]];
    if payload.style.display != LayoutDisplay::Block
        || payload.style.width != LayoutDimension::Auto
        || payload.style.height != LayoutDimension::Fixed(34.0)
        || payload.style.min_width != LayoutDimension::Auto
        || payload.style.max_width != LayoutDimension::Auto
        || payload.style.min_height != LayoutDimension::Auto
        || payload.style.max_height != LayoutDimension::Auto
        || payload.style.aspect_ratio.is_some()
        || payload.style.order != 0
        || payload.style.overflow_x != crate::LayoutOverflow::Visible
        || payload.style.overflow_y != crate::LayoutOverflow::Visible
        || payload.style.margin != crate::LayoutEdges::default()
        || payload.style.padding != crate::LayoutEdges::default()
        || payload.style.border != crate::LayoutBorder::default()
        || !payload.children.is_empty()
        || !child_has_no_text(payload_id)
        || !child_is_static(payload_id)
    {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node: payload_id,
            property: "C11.4 fit-content implicit row intrinsic payload",
        });
    }
    Ok(())
}

fn has_non_collapsible_text(input: &LayoutInput, node: NodeId) -> bool {
    let paragraph_has_content = |paragraph: &crate::ParagraphInput| {
        paragraph.sources.iter().any(|source| {
            source
                .text
                .chars()
                .any(|character| !matches!(character, ' ' | '\t' | '\n' | '\u{000c}' | '\r'))
        })
    };
    match input.text_paragraphs.get(&node) {
        Some(crate::TextParagraphInput::Paragraph(paragraph)) => paragraph_has_content(paragraph),
        Some(crate::TextParagraphInput::AnonymousGridText { .. }) => true,
        Some(crate::TextParagraphInput::Flow(flow)) => flow.items.iter().any(|item| match item {
            crate::TextFlowItem::Paragraph(paragraph) => paragraph_has_content(paragraph),
            crate::TextFlowItem::Block(_) => false,
        }),
        None => false,
    }
}
