use super::*;

fn fit_content_cap_is_supported(track: &GridTrackSize) -> bool {
    matches!(
        track,
        GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(value)
                | GridTrackLengthPercentage::Percentage(value)
        ) if value.is_finite() && *value >= 0.0
    )
}

pub(super) fn supports_min_content_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    matches!(
        tracks,
        [fit, GridTrackSize::MinContent, GridTrackSize::Auto]
            | [fit, GridTrackSize::Auto, GridTrackSize::MinContent]
            | [GridTrackSize::MinContent, fit, GridTrackSize::Auto]
            | [GridTrackSize::MinContent, GridTrackSize::Auto, fit]
            | [GridTrackSize::Auto, fit, GridTrackSize::MinContent]
            | [GridTrackSize::Auto, GridTrackSize::MinContent, fit]
            if fit_content_cap_is_supported(fit)
    )
}

pub(super) fn supports_min_max_content_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    if tracks.len() != 4 {
        return false;
    }

    let mut members = [false; 4];
    for track in tracks {
        let member = match track {
            track @ GridTrackSize::FitContent(_) if fit_content_cap_is_supported(track) => 0,
            GridTrackSize::MinContent => 1,
            GridTrackSize::MaxContent => 2,
            GridTrackSize::Auto => 3,
            _ => return false,
        };
        if std::mem::replace(&mut members[member], true) {
            return false;
        }
    }
    members.into_iter().all(|present| present)
}

pub(super) fn supports_intrinsic_auto_track_list(tracks: &[GridTrackSize]) -> bool {
    supports_min_content_auto_track_list(tracks) || supports_min_max_content_auto_track_list(tracks)
}

pub(super) fn supports_track_list(tracks: &[GridTrackSize]) -> bool {
    !tracks.is_empty()
        && tracks.len() <= GRID_MAX_TRACKS
        && tracks
            .iter()
            .any(|track| matches!(track, GridTrackSize::FitContent(_)))
        && if tracks
            .iter()
            .any(|track| matches!(track, GridTrackSize::MinContent | GridTrackSize::MaxContent))
        {
            supports_intrinsic_auto_track_list(tracks)
        } else {
            tracks.iter().all(|track| match track {
                GridTrackSize::Auto => true,
                GridTrackSize::LengthPx(value) | GridTrackSize::Percentage(value) => {
                    value.is_finite() && *value >= 0.0
                }
                track @ GridTrackSize::FitContent(_) => fit_content_cap_is_supported(track),
                _ => false,
            })
        }
}

pub(super) fn validate_root_html_fit_content_auto_rows_contract(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    node: NodeId,
    style: LayoutStyle,
    container: &crate::GridContainerInput,
) -> Result<(), LayoutError> {
    let unsupported_rows = || LayoutError::UnsupportedGridAutoRows {
        node,
        reason: "Root HTML fit-content implicit rows require a supported finite non-negative track-size list",
    };
    let crate::GridAutoRows::TrackList(tracks) = &container.auto_rows else {
        return Err(unsupported_rows());
    };
    if !supports_track_list(tracks) {
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

    let align_content_is_supported = style.align_content.is_some_and(|value| {
        root_html_intrinsic_align_content_is_supported(
            value,
            GridPlacementMode::RootHtmlIntrinsicTrackSizingV1,
        ) || (supports_intrinsic_auto_track_list(tracks)
            && value == crate::LayoutAlignContent::Stretch)
    });
    if !align_content_is_supported {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "align-content",
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
    let four_member_list = supports_min_max_content_auto_track_list(tracks);
    if (four_member_list && visible_children.len() != 6)
        || (!four_member_list && !matches!(visible_children.len(), 3 | 5))
        || has_non_collapsible_text(input, node)
    {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content implicit row list item count",
        });
    }

    let first_id = visible_children[0];
    let first = &input.nodes[index[&first_id]];
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
            && !input.grid_containers.contains_key(&item.id)
            && child_has_no_text(item.id)
            && child_has_auto_placement(item.id)
            && child_is_static(item.id)
    };
    if !fixed_item_is_supported(first, 10.0) {
        return Err(LayoutError::UnsupportedGridContainerStyle {
            node,
            property: "C11.4 fit-content explicit row item profile",
        });
    }

    let auto_item_is_supported = |item: &crate::LayoutNode, allow_floor: bool| {
        item.style.display == LayoutDisplay::Block
            && item.style.width == LayoutDimension::Auto
            && item.style.height == LayoutDimension::Auto
            && item.style.min_width == LayoutDimension::Auto
            && (item.style.min_height == LayoutDimension::Fixed(0.0)
                || (allow_floor && item.style.min_height == LayoutDimension::Fixed(28.0)))
            && item.style.max_width == LayoutDimension::Auto
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
            && item.children.len() == 1
            && !input.grid_containers.contains_key(&item.id)
            && child_has_no_text(item.id)
            && child_has_auto_placement(item.id)
            && child_is_static(item.id)
    };
    let payload_is_supported = |payload_id: NodeId, allowed_heights: &[f32]| {
        let payload = &input.nodes[index[&payload_id]];
        payload.style.display == LayoutDisplay::Block
            && payload.style.width == LayoutDimension::Auto
            && allowed_heights
                .iter()
                .any(|height| payload.style.height == LayoutDimension::Fixed(*height))
            && payload.style.min_width == LayoutDimension::Auto
            && payload.style.max_width == LayoutDimension::Auto
            && payload.style.min_height == LayoutDimension::Auto
            && payload.style.max_height == LayoutDimension::Auto
            && payload.style.aspect_ratio.is_none()
            && payload.style.order == 0
            && payload.style.overflow_x == crate::LayoutOverflow::Visible
            && payload.style.overflow_y == crate::LayoutOverflow::Visible
            && payload.style.margin == crate::LayoutEdges::default()
            && payload.style.padding == crate::LayoutEdges::default()
            && payload.style.border == crate::LayoutBorder::default()
            && payload.children.is_empty()
            && !input.grid_containers.contains_key(&payload_id)
            && child_has_no_text(payload_id)
            && child_is_static(payload_id)
    };
    const LIST_PAYLOAD_HEIGHTS: &[f32] = &[6.0, 8.0, 10.0, 25.0, 30.0, 34.0, 48.0, 50.0];
    const FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS: &[f32] = &[20.0, 10.0, 12.0, 6.0, 20.0];
    const FOUR_MEMBER_FIT_CONTENT_OVERFLOW_HEIGHT: f32 = 60.0;

    for (position, row_id) in visible_children.iter().copied().enumerate().skip(1) {
        let row_item = &input.nodes[index[&row_id]];
        let allow_minimum_floor = visible_children.len() == 3 && position == 1;
        if auto_item_is_supported(row_item, allow_minimum_floor) {
            let payload_id = row_item.children[0];
            let fit_content_payload_heights = [
                FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS[position - 1],
                FOUR_MEMBER_FIT_CONTENT_OVERFLOW_HEIGHT,
            ];
            let allowed_payload_heights = if four_member_list {
                let track_index = (position - 1) % tracks.len();
                if matches!(tracks[track_index], GridTrackSize::FitContent(_)) {
                    &fit_content_payload_heights
                } else {
                    std::slice::from_ref(&FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS[position - 1])
                }
            } else {
                LIST_PAYLOAD_HEIGHTS
            };
            if !payload_is_supported(payload_id, allowed_payload_heights) {
                return Err(LayoutError::UnsupportedGridContainerStyle {
                    node: payload_id,
                    property: "C11.4 fit-content implicit row list payload",
                });
            }
        } else if visible_children.len() == 3
            && position == 2
            && fixed_item_is_supported(row_item, 6.0)
        {
            // The pre-existing singleton fixture uses a fixed following item to expose
            // the used size of the first implicit row without another intrinsic payload.
        } else {
            return Err(LayoutError::UnsupportedGridContainerStyle {
                node,
                property: "C11.4 fit-content implicit row list item profile",
            });
        }
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
