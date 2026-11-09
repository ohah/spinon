use super::*;

#[path = "repeated_text/span.rs"]
mod span;
#[path = "repeated_text/track_trace.rs"]
mod track_trace;
use track_trace::{
    repeated_text_blink_layout_unit_distribute_rows, repeated_text_final_row_track_sizes,
    repeated_text_track_trace_error,
};

pub(super) fn blink_layout_unit_to_px(units: i64) -> f32 {
    track_trace::blink_layout_unit_to_px(units)
}

pub(super) fn blink_layout_unit_distribute_rows(
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
) -> Option<Vec<i64>> {
    track_trace::blink_layout_unit_distribute_rows(
        container,
        output,
        grid_id,
        grid_frame,
        raw_row_sizes,
    )
}

pub(super) fn supports_span_topology(input: &LayoutInput, items: &[NodeId]) -> bool {
    span::supports_topology(input, items)
}

pub(super) fn span_implicit_row_count(input: &LayoutInput, items: &[NodeId]) -> Option<usize> {
    span::implicit_row_count(input, items)
}

pub(super) fn normalize_layout_frames(
    input: &LayoutInput,
    output: &mut crate::LayoutOutput,
) -> Result<(), LayoutError> {
    let repeated_text_grids = input
        .grid_containers
        .iter()
        .filter(|(_, container)| {
            container.placement_mode == GridPlacementMode::RootHtmlFitContentAutoRowsV1
                && matches!(
                    &container.auto_rows,
                    GridAutoRows::TrackList(tracks)
                        if super::supports_repeated_min_max_content_auto_track_list(tracks)
                            || super::profiles::supports_repeated_min_max_content_auto_span_track_list(tracks)
                )
        })
        .map(|(grid_id, _)| *grid_id)
        .collect::<Vec<_>>();
    if repeated_text_grids.is_empty() {
        return Ok(());
    }
    if repeated_text_grids.len() != 1 || output.revision != input.revision {
        return Err(repeated_text_track_trace_error(repeated_text_grids[0]));
    }

    let index = input
        .nodes()
        .iter()
        .enumerate()
        .map(|(position, node)| (node.id, position))
        .collect::<BTreeMap<_, _>>();
    let hidden = crate::grid_placement::hidden_subtrees(input, &index);

    for grid_id in repeated_text_grids {
        let container = &input.grid_containers[&grid_id];
        let Some(&grid_index) = index.get(&grid_id) else {
            return Err(repeated_text_track_trace_error(grid_id));
        };
        let grid = &input.nodes()[grid_index];
        let items = grid
            .children
            .iter()
            .copied()
            .filter(|child| !hidden.get(child).copied().unwrap_or(true))
            .collect::<Vec<_>>();
        let repeated_text_profile = items.len() == 9
            && items.iter().copied().skip(1).all(|item_id| {
                let item = &input.nodes()[index[&item_id]];
                let [payload] = item.children.as_slice() else {
                    return false;
                };
                has_supported_text(input, *payload)
            });
        if let Some(positive_implicit_rows) = span_implicit_row_count(input, &items) {
            let GridAutoRows::TrackList(tracks) = &container.auto_rows else {
                return Err(repeated_text_track_trace_error(grid_id));
            };
            if !super::profiles::supports_repeated_span_fit_positions(
                tracks,
                positive_implicit_rows,
            ) {
                return Err(repeated_text_track_trace_error(grid_id));
            }
            span::normalize_layout_frames(
                input,
                output,
                grid_id,
                container,
                &items,
                &index,
                positive_implicit_rows,
            )?;
            continue;
        }
        let GridAutoRows::TrackList(tracks) = &container.auto_rows else {
            return Err(repeated_text_track_trace_error(grid_id));
        };
        if !super::supports_repeated_min_max_content_auto_track_list(tracks) {
            return Err(repeated_text_track_trace_error(grid_id));
        }
        if !repeated_text_profile {
            continue;
        }

        let grid_frame = output
            .frames
            .get(&grid_id)
            .copied()
            .ok_or(LayoutError::MissingComputedLayout(grid_id))?;
        let Some(raw_row_sizes) = repeated_text_final_row_track_sizes(
            output,
            grid_id,
            items.len(),
            grid_frame.height,
            container,
        ) else {
            return Err(repeated_text_track_trace_error(grid_id));
        };
        let mut row_offsets = Vec::with_capacity(items.len() + 1);
        row_offsets.push(0.0_f32);
        for &row_size in &raw_row_sizes {
            let next = row_offsets.last().copied().unwrap_or_default() + row_size;
            row_offsets.push(next);
        }
        let mut item_frames = Vec::with_capacity(items.len());
        for (row, item_id) in items.iter().copied().enumerate() {
            let frame = output
                .frames
                .get(&item_id)
                .copied()
                .ok_or(LayoutError::MissingComputedLayout(item_id))?;
            let flow_frame = output
                .flow_frames
                .get(&item_id)
                .copied()
                .ok_or_else(|| repeated_text_track_trace_error(grid_id))?;
            let expected_y = grid_frame.y + row_offsets[row];
            let expected_height = raw_row_sizes[row];
            if !frame_matches_row(frame, grid_frame, expected_y, expected_height)
                || !frame_matches_row(flow_frame, grid_frame, expected_y, expected_height)
            {
                return Err(repeated_text_track_trace_error(grid_id));
            }
            item_frames.push((item_id, frame));
        }
        let snapped_row_size_units = repeated_text_blink_layout_unit_distribute_rows(
            container,
            output,
            grid_id,
            grid_frame,
            &raw_row_sizes,
        )
        .ok_or_else(|| repeated_text_track_trace_error(grid_id))?;
        let mut snapped_top_units = 0_i64;
        let mut adjustments = Vec::with_capacity(item_frames.len());
        for (position, (item_id, item_frame)) in item_frames.iter().copied().enumerate() {
            let local_top = item_frame.y - grid_frame.y;
            let snapped_top = blink_layout_unit_to_px(snapped_top_units);
            let delta_y = snapped_top - local_top;
            let row_size_units = snapped_row_size_units[position];
            let snapped_height = blink_layout_unit_to_px(row_size_units);
            adjustments.push((item_id, delta_y, snapped_height));
            snapped_top_units += row_size_units;
        }

        for (item_id, delta_y, snapped_height) in adjustments {
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

fn frame_matches_row(
    frame: crate::LayoutFrame,
    grid_frame: crate::LayoutFrame,
    expected_y: f32,
    expected_height: f32,
) -> bool {
    [frame.x, frame.y, frame.width, frame.height]
        .into_iter()
        .all(f32::is_finite)
        && (frame.x - grid_frame.x).abs() <= 0.01
        && (frame.y - expected_y).abs() <= 0.01
        && (frame.width - grid_frame.width).abs() <= 0.01
        && (frame.height - expected_height).abs() <= 0.01
}

pub(super) fn has_supported_text(input: &LayoutInput, node: NodeId) -> bool {
    let Some(crate::TextParagraphInput::Paragraph(paragraph)) = input.text_paragraphs.get(&node)
    else {
        return false;
    };
    let style = &paragraph.style;
    paragraph.node == node
        && paragraph.sources.len() == 1
        && supports_content_grammar(&paragraph.sources[0].text)
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

fn supports_content_grammar(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes.len() > 32 {
        return false;
    }

    let mut word_length = 0;
    for byte in bytes {
        match byte {
            b'A'..=b'Z' => {
                word_length += 1;
                if word_length > 6 {
                    return false;
                }
            }
            b' ' => {
                if word_length == 0 {
                    return false;
                }
                word_length = 0;
            }
            _ => return false,
        }
    }

    word_length > 0
}

#[cfg(test)]
mod tests {
    use super::supports_content_grammar;

    #[test]
    fn content_grammar_accepts_ascii_word_and_byte_boundaries() {
        for text in ["A", "ABCDEF", "A B", "ABCDEF ABCDEF ABCDEF ABCDEF ABCD"] {
            assert!(
                supports_content_grammar(text),
                "expected acceptance: {text:?}"
            );
        }
    }

    #[test]
    fn content_grammar_rejects_non_ascii_and_malformed_spacing() {
        for text in [
            "",
            " ",
            " A",
            "A ",
            "A  B",
            "a",
            "1",
            "A?",
            "ABCDEFG",
            "ABCDEF ABCDEF ABCDEF ABCDEF ABCDE",
            "A\tB",
            "A\nB",
            "가",
        ] {
            assert!(
                !supports_content_grammar(text),
                "expected rejection: {text:?}"
            );
        }
    }
}
