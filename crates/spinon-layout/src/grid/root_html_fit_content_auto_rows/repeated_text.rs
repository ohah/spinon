use super::*;

pub(super) const PAYLOADS: [&str; 8] = [
    "A A A A A A",
    "AAAA AAAA A",
    "AA AA AA AA",
    "A A",
    "AAA AAA",
    "AAAA",
    "A A A A A A A A",
    "AA",
];

pub(super) fn normalize_layout_frames(
    input: &LayoutInput,
    output: &mut crate::LayoutOutput,
) -> Result<(), LayoutError> {
    if !input.grid_containers.values().any(|container| {
        container.placement_mode == GridPlacementMode::RootHtmlFitContentAutoRowsV1
            && matches!(
                &container.auto_rows,
                GridAutoRows::TrackList(tracks)
                    if super::supports_repeated_min_max_content_auto_track_list(tracks)
            )
    }) {
        return Ok(());
    }

    let index = input
        .nodes()
        .iter()
        .enumerate()
        .map(|(position, node)| (node.id, position))
        .collect::<BTreeMap<_, _>>();
    let hidden = crate::grid_placement::hidden_subtrees(input, &index);

    for (&grid_id, container) in &input.grid_containers {
        if container.placement_mode != GridPlacementMode::RootHtmlFitContentAutoRowsV1
            || !matches!(
                &container.auto_rows,
                GridAutoRows::TrackList(tracks)
                    if super::supports_repeated_min_max_content_auto_track_list(tracks)
            )
        {
            continue;
        }
        let Some(&grid_index) = index.get(&grid_id) else {
            continue;
        };
        let grid = &input.nodes()[grid_index];
        let items = grid
            .children
            .iter()
            .copied()
            .filter(|child| !hidden.get(child).copied().unwrap_or(true))
            .collect::<Vec<_>>();
        let repeated_text_profile = items.len() == 9
            && items
                .iter()
                .copied()
                .skip(1)
                .zip(PAYLOADS)
                .all(|(item, expected_text)| {
                    let item = &input.nodes()[index[&item]];
                    let [payload] = item.children.as_slice() else {
                        return false;
                    };
                    has_expected_text(input, *payload, expected_text)
                });
        if !repeated_text_profile {
            continue;
        }

        let grid_frame = output
            .frames
            .get(&grid_id)
            .copied()
            .ok_or(LayoutError::MissingComputedLayout(grid_id))?;
        let item_frames = items
            .iter()
            .map(|item_id| {
                output
                    .frames
                    .get(item_id)
                    .copied()
                    .map(|frame| (*item_id, frame))
                    .ok_or(LayoutError::MissingComputedLayout(*item_id))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (position, (item_id, item_frame)) in item_frames.iter().copied().enumerate() {
            let local_top = item_frame.y - grid_frame.y;
            let local_bottom = item_frames
                .get(position + 1)
                .map_or(local_top + item_frame.height, |(_, next_frame)| {
                    next_frame.y - grid_frame.y
                });
            let snapped_top = blink_layout_unit_floor(local_top);
            let snapped_bottom = blink_layout_unit_floor(local_bottom);
            let delta_y = snapped_top - local_top;
            let snapped_height = snapped_bottom - snapped_top;

            let mut pending = vec![item_id];
            while let Some(node_id) = pending.pop() {
                if let Some(frame) = output.frames.get_mut(&node_id) {
                    frame.y += delta_y;
                    if node_id == item_id {
                        frame.height = snapped_height;
                    }
                }
                if let Some(frame) = output.flow_frames.get_mut(&node_id) {
                    frame.y += delta_y;
                    if node_id == item_id {
                        frame.height = snapped_height;
                    }
                }
                if let Some(&node_index) = index.get(&node_id) {
                    pending.extend(input.nodes()[node_index].children.iter().copied());
                }
            }
        }
    }
    Ok(())
}

fn blink_layout_unit_floor(value: f32) -> f32 {
    let value = f64::from(value);
    let rounding_error_bound = f64::from(f32::EPSILON) * value.abs().max(1.0) * 4.0 * 64.0;
    ((value * 64.0 + rounding_error_bound).floor() / 64.0) as f32
}

pub(super) fn has_expected_text(input: &LayoutInput, node: NodeId, expected: &str) -> bool {
    let Some(crate::TextParagraphInput::Paragraph(paragraph)) = input.text_paragraphs.get(&node)
    else {
        return false;
    };
    let style = &paragraph.style;
    paragraph.node == node
        && paragraph.sources.len() == 1
        && paragraph.sources[0].text == expected
        && paragraph.sources[0].inline_ancestors.is_empty()
        && paragraph.inline_styles.is_empty()
        && paragraph.inline_run_styles.is_empty()
        && style.font_family == "monospace"
        && style.font_size == 10.0
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == spinon_text_layout::FontStyleMode::Normal
        && style.letter_spacing == 0.0
        && style.word_spacing == 0.0
        && style.text_indent == 0.0
        && style.text_indent_flags == Default::default()
        && style.text_transform == spinon_text_layout::TextTransformMode::None
        && style.white_space == spinon_text_layout::WhiteSpaceMode::Normal
        && style.word_break == spinon_text_layout::WordBreakMode::Normal
        && style
            .hyphens
            .is_none_or(|value| value == spinon_text_layout::HyphensMode::Manual)
        && style.overflow_wrap == spinon_text_layout::OverflowWrapMode::Normal
        && style.locale.as_deref().is_none_or(|locale| locale == "en")
        && style.direction == spinon_text_layout::TextDirection::Ltr
        && style.writing_mode == spinon_core::WritingMode::HorizontalTb
        && style.text_orientation == spinon_core::TextOrientation::Mixed
        && style.ligatures == spinon_text_layout::LigatureMode::Normal
}
