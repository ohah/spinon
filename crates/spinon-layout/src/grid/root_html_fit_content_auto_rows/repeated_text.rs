use super::*;

#[path = "repeated_text/span.rs"]
mod span;

pub(super) fn supports_span_topology(input: &LayoutInput, items: &[NodeId]) -> bool {
    span::supports_topology(input, items)
}

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
            && items.iter().copied().skip(1).all(|item_id| {
                let item = &input.nodes()[index[&item_id]];
                let [payload] = item.children.as_slice() else {
                    return false;
                };
                has_supported_text(input, *payload)
            });
        if supports_span_topology(input, &items) {
            span::normalize_layout_frames(input, output, grid_id, container, &items, &index)?;
            continue;
        }
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
        let raw_row_sizes = item_frames
            .iter()
            .enumerate()
            .map(|(position, (_, item_frame))| {
                item_frames
                    .get(position + 1)
                    .map_or(item_frame.height, |(_, next_frame)| {
                        next_frame.y - item_frame.y
                    })
            })
            .collect::<Vec<_>>();
        let snapped_row_size_units = blink_layout_unit_distribute_rows(
            container,
            output,
            grid_id,
            grid_frame,
            &raw_row_sizes,
        )
        .unwrap_or_else(|| {
            item_frames
                .iter()
                .enumerate()
                .map(|(position, (_, item_frame))| {
                    let local_top = item_frame.y - grid_frame.y;
                    let local_bottom = item_frames
                        .get(position + 1)
                        .map_or(local_top + item_frame.height, |(_, next_frame)| {
                            next_frame.y - grid_frame.y
                        });
                    blink_layout_unit_floor_units(local_bottom)
                        - blink_layout_unit_floor_units(local_top)
                })
                .collect()
        });
        let mut snapped_top_units =
            blink_layout_unit_floor_units(item_frames[0].1.y - grid_frame.y);
        for (position, (item_id, item_frame)) in item_frames.iter().copied().enumerate() {
            let local_top = item_frame.y - grid_frame.y;
            let snapped_top = blink_layout_unit_to_px(snapped_top_units);
            let delta_y = snapped_top - local_top;
            let row_size_units = snapped_row_size_units[position];
            let snapped_height = blink_layout_unit_to_px(row_size_units);

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
            snapped_top_units += row_size_units;
        }
    }
    Ok(())
}

fn blink_layout_unit_floor_units(value: f32) -> i64 {
    let value = f64::from(value);
    let rounding_error_bound = f64::from(f32::EPSILON) * value.abs().max(1.0) * 4.0 * 64.0;
    (value * 64.0 + rounding_error_bound).floor() as i64
}

pub(super) fn blink_layout_unit_to_px(units: i64) -> f32 {
    (units as f64 / 64.0) as f32
}

pub(super) fn blink_layout_unit_distribute_rows(
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    grid_frame: crate::LayoutFrame,
    raw_row_sizes: &[f32],
) -> Option<Vec<i64>> {
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid_id)?;
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)?;
    let intrinsic_stage = pass.stages.iter().find(|stage| {
        stage.stage == crate::LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
    })?;
    let final_stage = pass.stages.last()?;
    let intrinsic_tracks = intrinsic_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    let final_tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    if intrinsic_tracks.len() != raw_row_sizes.len() || final_tracks.len() != raw_row_sizes.len() {
        return None;
    }

    let mut quantized = Vec::with_capacity(raw_row_sizes.len());
    let mut growth_potentials = Vec::new();
    let mut raw_total = 0.0_f64;
    let mut floor_total = 0_i64;
    for (row, &raw_size) in raw_row_sizes.iter().enumerate() {
        if !raw_size.is_finite() || raw_size < 0.0 {
            return None;
        }
        if (final_tracks[row].base_size - raw_size).abs() > 0.01 {
            return None;
        }

        let scaled = f64::from(raw_size) * 64.0;
        let floor_units = blink_layout_unit_floor_units(raw_size);
        let nearest_units = scaled.round();
        let rounding_error_bound =
            f64::from(f32::EPSILON) * f64::from(raw_size).abs().max(1.0) * 4.0 * 64.0;
        let is_fractional = (scaled - nearest_units).abs() > rounding_error_bound;
        quantized.push(floor_units);
        floor_total += floor_units;
        raw_total += f64::from(raw_size);

        if !is_fractional {
            continue;
        }
        let track = intrinsic_tracks[row];
        let growth_limit = track.growth_limit.unwrap_or(f32::INFINITY);
        let fit_content_limit = repeated_row_fit_content_limit(container, row, grid_frame.height)?;
        let growth_limit = fit_content_limit.map_or(growth_limit, |limit| growth_limit.min(limit));
        let growth_potential = (growth_limit - track.base_size).max(0.0);
        if growth_potential < 1.0 / 64.0 {
            return None;
        }
        growth_potentials.push((row, growth_potential));
    }

    let target_total = (raw_total * 64.0).round() as i64;
    let leftover_units = target_total - floor_total;
    if leftover_units < 0 || leftover_units as usize > growth_potentials.len() {
        return None;
    }
    // Blink distributes 1/64px shares in growth-potential order; Taffy keeps the
    // equivalent track sizes as f32 values, so assign the residual units here.
    growth_potentials.sort_by(|left, right| left.1.total_cmp(&right.1));
    for (row, _) in growth_potentials.iter().rev().take(leftover_units as usize) {
        quantized[*row] += 1;
    }
    Some(quantized)
}

fn repeated_row_fit_content_limit(
    container: &crate::GridContainerInput,
    row: usize,
    grid_height: f32,
) -> Option<Option<f32>> {
    let track = if row < container.rows.len() {
        container.rows.get(row)?
    } else {
        let crate::GridAutoRows::TrackList(tracks) = &container.auto_rows else {
            return None;
        };
        if tracks.is_empty() {
            return None;
        }
        &tracks[(row - container.rows.len()) % tracks.len()]
    };
    let crate::GridTrackSize::FitContent(limit) = track else {
        return Some(None);
    };
    let limit = match limit {
        crate::GridTrackLengthPercentage::LengthPx(value) => *value,
        crate::GridTrackLengthPercentage::Percentage(value) => *value * grid_height,
        crate::GridTrackLengthPercentage::Calc(_) => return None,
    };
    Some(Some(limit))
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
