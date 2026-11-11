use super::*;

pub(super) fn supports_topology(input: &LayoutInput, items: &[NodeId]) -> bool {
    implicit_row_count(input, items).is_some()
}

pub(super) fn implicit_row_count(input: &LayoutInput, items: &[NodeId]) -> Option<usize> {
    if !matches!(items.len(), 3..=6) {
        return None;
    }
    let first = input.nodes().iter().find(|node| node.id == items[0])?;
    let first_placement = input.grid_items.get(&items[0])?;
    if !first.children.is_empty()
        || first_placement.row_start != GridPlacementInput::Line(1)
        || first_placement.row_end != GridPlacementInput::Line(2)
        || first_placement.column_start != GridPlacementInput::Line(1)
        || first_placement.column_end != GridPlacementInput::Line(2)
    {
        return None;
    }

    let mut placements = Vec::<(i32, u32)>::with_capacity(items.len() - 1);
    for item_id in items.iter().copied().skip(1) {
        let item = input.nodes().iter().find(|node| node.id == item_id)?;
        let placement = input.grid_items.get(&item_id)?;
        if placement.column_start != GridPlacementInput::Line(1)
            || placement.column_end != GridPlacementInput::Line(2)
        {
            return None;
        }
        let kind = match (&placement.row_start, &placement.row_end) {
            (GridPlacementInput::Line(start @ 2..=6), GridPlacementInput::Span(span @ 2..=3)) => {
                (*start, *span)
            }
            _ => return None,
        };
        let [payload] = item.children.as_slice() else {
            return None;
        };
        if !has_supported_text(input, *payload) {
            return None;
        }
        placements.push(kind);
    }

    let count = |candidate| {
        placements
            .iter()
            .filter(|placement| **placement == candidate)
            .count()
    };
    let exact = |expected: &[(i32, u32)]| {
        placements.len() == expected.len()
            && expected.iter().all(|placement| count(*placement) == 1)
    };
    if exact(&[(2, 2), (3, 2)]) || exact(&[(2, 2), (3, 2), (2, 3)]) {
        Some(3)
    } else if exact(&[(2, 2), (3, 2), (4, 2), (5, 2), (6, 2)])
        || exact(&[(2, 2), (3, 2), (4, 3), (5, 3)])
    {
        Some(6)
    } else {
        None
    }
}

pub(super) fn normalize_layout_frames(
    input: &LayoutInput,
    output: &mut crate::LayoutOutput,
    grid_id: NodeId,
    container: &crate::GridContainerInput,
    items: &[NodeId],
    index: &BTreeMap<NodeId, usize>,
    positive_implicit_rows: usize,
) -> Result<(), LayoutError> {
    let unsupported_trace = || LayoutError::UnsupportedGridContainerStyle {
        node: grid_id,
        property: "C11.4 repeated text row-span track trace",
    };
    let grid_frame = output
        .frames
        .get(&grid_id)
        .copied()
        .ok_or(LayoutError::MissingComputedLayout(grid_id))?;
    let expected_track_count = positive_implicit_rows + 1;
    let row_sizes = final_row_track_sizes(output, grid_id, positive_implicit_rows)
        .ok_or_else(unsupported_trace)?;
    if row_sizes.len() != expected_track_count {
        return Err(unsupported_trace());
    }
    let snapped_row_size_units = blink_layout_unit_distribute_rows(
        input, container, output, grid_id, grid_frame, &row_sizes,
    )
    .ok_or_else(unsupported_trace)?;
    if snapped_row_size_units.len() != row_sizes.len() {
        return Err(unsupported_trace());
    }

    let mut raw_row_offsets = Vec::with_capacity(row_sizes.len() + 1);
    raw_row_offsets.push(0.0_f32);
    for &row_size in &row_sizes {
        let next = raw_row_offsets.last().copied().unwrap_or_default() + row_size;
        raw_row_offsets.push(next);
    }
    let mut snapped_row_offsets = Vec::with_capacity(snapped_row_size_units.len() + 1);
    snapped_row_offsets.push(0_i64);
    for &row_size in &snapped_row_size_units {
        let next = snapped_row_offsets.last().copied().unwrap_or_default() + row_size;
        snapped_row_offsets.push(next);
    }

    for item_id in items.iter().copied() {
        let placement = input
            .grid_items
            .get(&item_id)
            .ok_or_else(unsupported_trace)?;
        let (row_start, row_end) = match (&placement.row_start, &placement.row_end) {
            (GridPlacementInput::Line(1), GridPlacementInput::Line(2)) => (0_usize, 1_usize),
            (GridPlacementInput::Line(start), GridPlacementInput::Span(span))
                if *start > 0 && *span > 0 =>
            {
                let start = usize::try_from(*start - 1).map_err(|_| unsupported_trace())?;
                let end = start
                    .checked_add(usize::try_from(*span).map_err(|_| unsupported_trace())?)
                    .ok_or_else(unsupported_trace)?;
                (start, end)
            }
            _ => return Err(unsupported_trace()),
        };
        if row_start >= row_end || row_end >= raw_row_offsets.len() {
            return Err(unsupported_trace());
        }
        let frame = output
            .frames
            .get(&item_id)
            .copied()
            .ok_or(LayoutError::MissingComputedLayout(item_id))?;
        let expected_raw_top = grid_frame.y + raw_row_offsets[row_start];
        let expected_raw_height = raw_row_offsets[row_end] - raw_row_offsets[row_start];
        if (frame.y - expected_raw_top).abs() > 0.01
            || (frame.height - expected_raw_height).abs() > 0.01
        {
            return Err(unsupported_trace());
        }
        let snapped_top = grid_frame.y + blink_layout_unit_to_px(snapped_row_offsets[row_start]);
        let snapped_height =
            blink_layout_unit_to_px(snapped_row_offsets[row_end] - snapped_row_offsets[row_start]);
        let delta_y = snapped_top - frame.y;

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
    Ok(())
}

fn final_row_track_sizes(
    output: &crate::LayoutOutput,
    grid_id: NodeId,
    positive_implicit_rows: usize,
) -> Option<Vec<f32>> {
    let mut traces = output
        .grid_track_sizing_traces
        .iter()
        .filter(|trace| trace.node == grid_id);
    let trace = traces.next()?;
    if traces.next().is_some()
        || trace.used_rows.explicit_track_count != 1
        || trace.used_rows.negative_implicit_track_count != 0
        || usize::from(trace.used_rows.positive_implicit_track_count) != positive_implicit_rows
        || !trace.used_rows.collapsed_track_indices.is_empty()
    {
        return None;
    }
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)?;
    let final_stage = pass.stages.last()?;
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    if tracks.len() != positive_implicit_rows + 1
        || tracks.iter().enumerate().any(|(index, track)| {
            usize::from(track.internal_index) != index * 2 + 1
                || track.is_collapsed
                || !track.base_size.is_finite()
                || track.base_size < 0.0
        })
    {
        return None;
    }
    Some(tracks.into_iter().map(|track| track.base_size).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        GridContainerInput, LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingResolvedAxis,
        LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrackState, LayoutInputRevision,
        LayoutNode, LayoutSourceRevision, LayoutStyle, RootSizingPolicy, Viewport,
    };
    use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};
    use std::collections::BTreeMap;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).unwrap()
    }

    fn input(grid: NodeId, item: NodeId) -> LayoutInput {
        LayoutInput {
            root: grid,
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::Tree(Revision::default()),
                StyleRevision::default(),
                EnvironmentRevision::default(),
            ),
            viewport: Viewport {
                width: 40.0,
                height: 220.0,
            },
            root_sizing: RootSizingPolicy::Match,
            nodes: vec![
                LayoutNode {
                    id: grid,
                    children: vec![item],
                    style: LayoutStyle::default(),
                },
                LayoutNode {
                    id: item,
                    children: Vec::new(),
                    style: LayoutStyle::default(),
                },
            ],
            css_math: Vec::new(),
            positioning: BTreeMap::new(),
            grid_containers: BTreeMap::new(),
            grid_items: BTreeMap::new(),
            text_paragraphs: BTreeMap::new(),
        }
    }

    fn trace(grid: NodeId, row_sizes: &[f32]) -> LayoutGridTrackSizingTrace {
        let tracks = row_sizes
            .iter()
            .enumerate()
            .map(|(index, base_size)| LayoutGridTrackSizingTrackState {
                internal_index: (index as u16) * 2 + 1,
                is_gutter: false,
                is_collapsed: false,
                base_size: *base_size,
                growth_limit: Some(*base_size),
                infinitely_growable: false,
            })
            .collect::<Vec<_>>();
        LayoutGridTrackSizingTrace {
            node: grid,
            passes: vec![LayoutGridTrackSizingPassTrace {
                sequence: 1,
                axis: crate::LayoutGridTrackSizingAxis::Rows,
                stages: vec![
                    LayoutGridTrackSizingStageTrace {
                        stage: crate::LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
                        tracks: tracks.clone(),
                    },
                    LayoutGridTrackSizingStageTrace {
                        stage: crate::LayoutGridTrackSizingStageKind::StretchAutoTracks {
                            applied: false,
                        },
                        tracks,
                    },
                ],
                contributions: Vec::new(),
                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                fixture_phase: None,
                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                fixture_auto_repeat_count: None,
                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                auto_repeat_count: None,
                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                fixture_percentage_basis: None,
            }],
            used_columns: LayoutGridTrackSizingResolvedAxis::default(),
            used_rows: LayoutGridTrackSizingResolvedAxis {
                used_track_list: row_sizes
                    .iter()
                    .map(|size| format!("{size}px"))
                    .collect::<Vec<_>>()
                    .join(" "),
                explicit_track_count: 1,
                negative_implicit_track_count: 0,
                positive_implicit_track_count: u16::try_from(row_sizes.len() - 1).unwrap(),
                collapsed_track_indices: Vec::new(),
            },
        }
    }

    fn assert_trace_error_is_atomic(
        trace: Option<LayoutGridTrackSizingTrace>,
        positive_implicit_rows: usize,
    ) {
        let grid = id(1);
        let item = id(2);
        let mut output = crate::LayoutOutput::default();
        output.frames.insert(
            grid,
            crate::LayoutFrame {
                x: 0.0,
                y: 0.0,
                width: 40.0,
                height: 220.0,
            },
        );
        output.frames.insert(
            item,
            crate::LayoutFrame {
                x: 0.0,
                y: 10.0,
                width: 40.0,
                height: 20.0,
            },
        );
        output.flow_frames = output.frames.clone();
        if let Some(trace) = trace {
            output.grid_track_sizing_traces.push(trace);
        }
        let frames_before = output.frames.clone();
        let flow_frames_before = output.flow_frames.clone();
        let error = normalize_layout_frames(
            &input(grid, item),
            &mut output,
            grid,
            &GridContainerInput::default(),
            &[item],
            &BTreeMap::new(),
            positive_implicit_rows,
        )
        .expect_err("missing or inconsistent row trace must fail closed");
        assert!(matches!(
            error,
            LayoutError::UnsupportedGridContainerStyle {
                property: "C11.4 repeated text row-span track trace",
                ..
            }
        ));
        assert_eq!(output.frames, frames_before, "frames remain unpublished");
        assert_eq!(
            output.flow_frames, flow_frames_before,
            "flow frames remain unpublished"
        );
    }

    #[test]
    fn repeated_text_row_span_missing_or_stale_owner_trace_rejects_atomically() {
        assert_trace_error_is_atomic(None, 3);
        assert_trace_error_is_atomic(Some(trace(id(99), &[10.0, 0.0, 20.0, 15.0])), 3);
    }

    #[test]
    fn repeated_text_row_span_wrong_track_count_or_internal_index_rejects_atomically() {
        assert_trace_error_is_atomic(Some(trace(id(1), &[10.0, 0.0, 20.0])), 3);
        let mut mismatched = trace(id(1), &[10.0, 0.0, 20.0, 15.0]);
        mismatched.passes[0].stages[1].tracks[2].internal_index = 17;
        assert_trace_error_is_atomic(Some(mismatched), 3);
    }

    #[test]
    fn repeated_text_row_span_trace_accepts_only_the_expected_three_or_six_rows() {
        let grid = id(1);
        for (positive_implicit_rows, row_sizes) in [
            (3, vec![10.0, 0.0, 20.0, 15.0]),
            (6, vec![10.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0]),
        ] {
            let mut output = crate::LayoutOutput::default();
            output
                .grid_track_sizing_traces
                .push(trace(grid, &row_sizes));
            assert_eq!(
                final_row_track_sizes(&output, grid, positive_implicit_rows),
                Some(row_sizes.clone()),
                "expected {positive_implicit_rows} positive implicit rows"
            );
            assert_eq!(
                final_row_track_sizes(
                    &output,
                    grid,
                    if positive_implicit_rows == 3 { 6 } else { 3 }
                ),
                None,
                "a trace for a different topology must fail closed"
            );
        }
    }
}
