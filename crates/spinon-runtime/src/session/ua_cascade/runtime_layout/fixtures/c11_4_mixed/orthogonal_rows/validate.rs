use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, IntrinsicSizeRevision, NodeId, TextOrientation,
    WritingMode,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingTrace, LayoutInputRevision,
    LayoutIntrinsicMeasurementSource, LayoutSourceRevision,
};
use spinon_style::ComputedStyleSnapshot;

use super::super::super::super::{RuntimeLayoutCompleted, RuntimeLayoutFailure};
use super::super::super::c11_4_nested_grid_depth_two::{children, element_identity, style_for};
use super::helpers::{assert_frame, fail, frame_matches, near, option_near, unsupported};
use super::scene::{ItemKind, Scene, scene_for_root_id};
use super::style::{validate_grid_style, validate_item_style};

pub(in crate::session::ua_cascade::runtime_layout) fn validate_runtime_output(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    intrinsic_revision: IntrinsicSizeRevision,
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let (_, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = scene_for_root_id(&root_id) else {
        return Ok(());
    };
    validate_grid_style(style_for(styles, root.id())?, styles, root, scene)?;

    let children = children(snapshot, root)?;
    let expected_items = scene.items();
    if children.len() != expected_items.len() {
        return Err(unsupported(
            root.id(),
            "exactly two direct Grid items required",
        ));
    }
    let mut text = None;
    let mut image = None;
    for (handle, expected) in children.iter().zip(expected_items) {
        let (tag, id) = element_identity(snapshot, *handle)?;
        if id != scene.item_id(expected.kind) {
            return Err(unsupported(
                handle.id(),
                "direct child order or identity differs",
            ));
        }
        validate_item_style(style_for(styles, handle.id())?, styles, scene, expected)?;
        match (expected.kind, tag.as_str()) {
            (ItemKind::Text, "div") => text = Some(handle.id()),
            (ItemKind::Image, "img") => image = Some(handle.id()),
            _ => {
                return Err(unsupported(
                    handle.id(),
                    "item tag differs from fixed scene",
                ));
            }
        }
    }
    let text = text.ok_or_else(|| fail("c114_orthogonal_text_missing", root.id(), "text item"))?;
    let image =
        image.ok_or_else(|| fail("c114_orthogonal_image_missing", root.id(), "image item"))?;
    let row_height = scene.row_height();

    assert_frame(completed, root.id(), (0.0, 0.0, 74.0, row_height), "root")?;
    assert_frame(completed, text, (0.0, 0.0, 50.0, row_height), "text item")?;
    assert_frame(completed, image, (50.0, 0.0, 24.0, 16.0), "image item")?;

    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_deref()
        .ok_or_else(|| {
            fail(
                "c114_orthogonal_trace_missing",
                root.id(),
                "track diagnostics",
            )
        })?;
    let expected_revision = LayoutInputRevision::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        styles.style_revision,
        styles.viewport.environment_revision,
    )
    .with_intrinsic_size_revision(intrinsic_revision);
    if diagnostics.revision != expected_revision
        || diagnostics.item_inputs.len() != 2
        || diagnostics.item_frames.len() != 2
        || diagnostics
            .item_inputs
            .iter()
            .any(|input| input.grid_container != root.id() || input.revision != expected_revision)
        || diagnostics
            .intrinsic_measurements
            .iter()
            .any(|measurement| measurement.revision != expected_revision)
    {
        return Err(fail(
            "c114_orthogonal_layout_revision_mismatch",
            root.id(),
            format!("expected complete provenance for {expected_revision:?}"),
        ));
    }

    let mut text_font_revision = None;
    let mut text_font_identity = None;
    for (source_order, (input, expected)) in diagnostics
        .item_inputs
        .iter()
        .zip(expected_items)
        .enumerate()
    {
        let expected_node = match expected.kind {
            ItemKind::Text => text,
            ItemKind::Image => image,
        };
        if input.node != expected_node || input.source_order != source_order {
            return Err(fail(
                "c114_orthogonal_item_order_mismatch",
                input.node,
                format!("expected node {expected_node:?} at source order {source_order}"),
            ));
        }
        let frame = diagnostics
            .item_frames
            .iter()
            .find_map(|(node, frame)| (*node == input.node).then_some(frame))
            .ok_or_else(|| fail("c114_orthogonal_item_frame_missing", input.node, "trace"))?;
        let expected_frame = if input.node == text {
            (0.0, 0.0, 50.0, row_height)
        } else {
            (50.0, 0.0, 24.0, 16.0)
        };
        if !frame_matches(
            (frame.x, frame.y, frame.width, frame.height),
            expected_frame,
        ) {
            return Err(fail(
                "c114_orthogonal_item_frame_mismatch",
                input.node,
                format!("expected {expected_frame:?}, got {frame:?}"),
            ));
        }

        match (&input.source, expected.kind) {
            (
                LayoutGridItemIntrinsicSource::OrthogonalText {
                    min_content,
                    max_content,
                    child_writing_mode,
                    parent_writing_mode,
                    text_orientation,
                    font_registry_revision,
                    font_registry_identity,
                },
                ItemKind::Text,
            ) => {
                let expected_min_content = scene.min_content();
                let expected_max_content = scene.max_content();
                if *child_writing_mode != WritingMode::VerticalRl
                    || *parent_writing_mode != WritingMode::HorizontalTb
                    || *text_orientation != TextOrientation::Mixed
                    || font_registry_revision.0 != 1
                    || font_registry_identity.font_registry_revision() != *font_registry_revision
                    || !near(min_content.inline_px, expected_min_content.0)
                    || !near(min_content.block_px, expected_min_content.1)
                    || !near(max_content.inline_px, expected_max_content.0)
                    || !near(max_content.block_px, expected_max_content.1)
                {
                    return Err(fail(
                        "c114_orthogonal_logical_intrinsic_mismatch",
                        input.node,
                        format!("min={min_content:?} max={max_content:?}"),
                    ));
                }
                text_font_revision = Some(*font_registry_revision);
                text_font_identity = Some(font_registry_identity.clone());
                let physical = input.contribution_input().map_err(|error| {
                    fail("c114_orthogonal_contribution_input", input.node, error)
                })?;
                if !option_near(physical.width.min_content, expected_min_content.1)
                    || !option_near(physical.width.max_content, expected_max_content.1)
                    || !option_near(physical.height.min_content, expected_min_content.0)
                    || !option_near(physical.height.max_content, expected_max_content.0)
                {
                    return Err(fail(
                        "c114_orthogonal_row_contribution_mismatch",
                        input.node,
                        format!("physical row input {physical:?}"),
                    ));
                }
            }
            (LayoutGridItemIntrinsicSource::Replaced { natural }, ItemKind::Image) => {
                if !option_near(natural.width, 24.0) || !option_near(natural.height, 16.0) {
                    return Err(fail(
                        "c114_orthogonal_image_intrinsic_mismatch",
                        input.node,
                        format!("natural {natural:?}"),
                    ));
                }
            }
            _ => {
                return Err(fail(
                    "c114_orthogonal_intrinsic_kind_mismatch",
                    input.node,
                    "expected orthogonal text and natural replaced image sources",
                ));
            }
        }
    }

    let orthogonal_text_measurements = diagnostics
        .intrinsic_measurements
        .iter()
        .filter(|measurement| measurement.node == text)
        .filter(|measurement| {
            matches!(
                &measurement.source,
                LayoutIntrinsicMeasurementSource::OrthogonalText { .. }
            )
        })
        .collect::<Vec<_>>();
    if orthogonal_text_measurements.is_empty()
        || orthogonal_text_measurements.iter().any(|measurement| {
            let expected_min_content = scene.min_content();
            let expected_max_content = scene.max_content();
            !matches!(
                &measurement.source,
                LayoutIntrinsicMeasurementSource::OrthogonalText {
                    min_content,
                    max_content,
                    child_writing_mode: WritingMode::VerticalRl,
                    parent_writing_mode: WritingMode::HorizontalTb,
                    text_orientation: TextOrientation::Mixed,
                    font_registry_revision,
                    font_registry_identity,
                } if near(min_content.inline_px, expected_min_content.0)
                    && near(min_content.block_px, expected_min_content.1)
                    && near(max_content.inline_px, expected_max_content.0)
                    && near(max_content.block_px, expected_max_content.1)
                    && Some(*font_registry_revision) == text_font_revision
                    && font_registry_identity.font_registry_revision() == *font_registry_revision
                    && font_registry_identity.paragraph() == measurement.node
                    && font_registry_identity.writing_mode() == WritingMode::VerticalRl
                    && font_registry_identity.text_orientation() == TextOrientation::Mixed
                    && font_registry_identity.min_content() == *min_content
                    && font_registry_identity.max_content() == *max_content
                    && text_font_identity.as_ref().is_some_and(|identity| {
                        identity.uses_same_font_registry_revision(font_registry_identity)
                    })
            )
        })
    {
        return Err(fail(
            "c114_orthogonal_text_measurement_missing",
            text,
            "matching orthogonal measurement and font revision",
        ));
    }

    validate_track_trace(diagnostics.traces.as_ref(), root.id(), text, image, scene)
}

fn validate_track_trace(
    traces: &[LayoutGridTrackSizingTrace],
    root: NodeId,
    text: NodeId,
    image: NodeId,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    if traces.len() != 1 || traces[0].node != root {
        return Err(fail(
            "c114_orthogonal_trace_grid_mismatch",
            root,
            "one root trace",
        ));
    }
    let passes = &traces[0].passes;
    let last_column = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .max_by_key(|pass| pass.sequence)
        .ok_or_else(|| fail("c114_orthogonal_columns_trace_missing", root, "Columns"))?;
    let first_row = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .min_by_key(|pass| pass.sequence)
        .ok_or_else(|| fail("c114_orthogonal_rows_trace_missing", root, "Rows"))?;
    if last_column.sequence >= first_row.sequence
        || last_column.stages.is_empty()
        || first_row.stages.is_empty()
    {
        return Err(fail(
            "c114_orthogonal_axis_trace_order_mismatch",
            root,
            "all Columns passes must complete before Rows",
        ));
    }
    assert_pass_tracks(last_column, root, &[50.0, 24.0], "Columns")?;
    let last_row = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .ok_or_else(|| fail("c114_orthogonal_rows_trace_missing", root, "final Rows"))?;
    assert_pass_tracks(last_row, root, &[scene.row_height()], "Rows")?;
    let selected_kind = scene.track.contribution_kind();
    let mut text_selected_contribution_seen = false;
    let mut image_selected_contribution_seen = false;
    for contribution in &last_row.contributions {
        let expected = match (contribution.item, contribution.kind) {
            (node, LayoutGridTrackSizingContributionKind::MinContent) if node == text => {
                scene.min_content().0
            }
            (node, LayoutGridTrackSizingContributionKind::MaxContent) if node == text => {
                scene.max_content().0
            }
            (node, LayoutGridTrackSizingContributionKind::MinContent)
            | (node, LayoutGridTrackSizingContributionKind::MaxContent)
                if node == image =>
            {
                16.0
            }
            (node, _) if node != text && node != image => {
                return Err(fail(
                    "c114_orthogonal_row_contribution_unexpected_item",
                    contribution.item,
                    "only the direct text and image may contribute to the row",
                ));
            }
            _ => {
                return Err(fail(
                    "c114_orthogonal_row_contribution_unexpected_kind",
                    contribution.item,
                    format!("unexpected row contribution {contribution:?}"),
                ));
            }
        };
        if contribution.span != 1
            || contribution.crosses_flexible_track
            || !near(contribution.value, expected)
        {
            return Err(fail(
                "c114_orthogonal_row_contribution_mismatch",
                contribution.item,
                format!(
                    "expected {:?} non-flex-crossing span-1 value {expected}, got {:?}",
                    contribution.kind, contribution
                ),
            ));
        }
        if contribution.kind == selected_kind {
            if contribution.item == text {
                text_selected_contribution_seen = true;
            } else if contribution.item == image {
                image_selected_contribution_seen = true;
            }
        }
    }
    if !text_selected_contribution_seen || !image_selected_contribution_seen {
        return Err(fail(
            "c114_orthogonal_row_contribution_missing",
            root,
            format!(
                "selected {selected_kind:?}: text={text_selected_contribution_seen}, image={image_selected_contribution_seen}"
            ),
        ));
    }
    Ok(())
}

fn assert_pass_tracks(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    root: NodeId,
    expected: &[f32],
    axis: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let stage = pass
        .stages
        .last()
        .ok_or_else(|| fail("c114_orthogonal_trace_stage_missing", root, axis))?;
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    if tracks.len() != expected.len()
        || tracks
            .iter()
            .zip(expected)
            .any(|(actual, expected)| !near(*actual, *expected))
    {
        return Err(fail(
            "c114_orthogonal_trace_track_mismatch",
            root,
            format!("{axis}: expected {expected:?}, got {tracks:?}"),
        ));
    }
    Ok(())
}
