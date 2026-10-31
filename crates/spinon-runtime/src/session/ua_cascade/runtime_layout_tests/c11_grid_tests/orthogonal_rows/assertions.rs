use std::collections::BTreeMap;

use super::super::*;
use super::AHEM;
use super::scene::Scene;
use super::snapshot::orthogonal_snapshot;
use spinon_core::{HostDocumentSnapshot, IntrinsicSizeRevision, TextOrientation, WritingMode};
use spinon_layout::{
    FontStyleMode, LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutIntrinsicMeasurementSource, LayoutIntrinsicSize,
    LayoutIntrinsicSizeSnapshot, LayoutSourceRevision, LigatureMode, LineBreakMode, ParagraphStyle,
    TextLayoutEngine, TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};

pub(super) fn assert_runtime_scene(scene: Scene) {
    let (snapshot, root, text, image) = orthogonal_snapshot(scene);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot: Arc::clone(&snapshot),
        viewport: CssViewport {
            width_css_px: 74.0,
            height_css_px: 120.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("{}: {error:?}", scene.id));
    let layout = match calculation.layout {
        Ok(layout) => layout,
        Err(error) => {
            let styles = calculation
                .roots
                .iter()
                .find(|root_style| root_style.root_node_id == root.id().get())
                .unwrap_or_else(|| panic!("{}: computed root style missing", scene.id))
                .styles
                .clone();
            let raw =
                debug_raw_runtime_output(&snapshot, root, text, image, request.viewport, styles);
            panic!("{}: runtime layout failed: {error:?}\n{raw}", scene.id);
        }
    };
    assert_frame(
        &layout.frames,
        root.id().get(),
        (0.0, 0.0, 74.0, scene.row_height()),
        scene,
    );
    assert_frame(
        &layout.frames,
        text.id().get(),
        (0.0, 0.0, 50.0, scene.row_height()),
        scene,
    );
    assert_frame(
        &layout.frames,
        image.id().get(),
        (50.0, 0.0, 24.0, 16.0),
        scene,
    );

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .unwrap_or_else(|| panic!("{}: track diagnostics missing", scene.id));
    assert_eq!(diagnostics.item_inputs.len(), 2, "{}", scene.id);
    assert_eq!(diagnostics.item_frames.len(), 2, "{}", scene.id);
    let expected_order = if scene.reversed {
        [image.id(), text.id()]
    } else {
        [text.id(), image.id()]
    };
    for (source_order, expected_node) in expected_order.into_iter().enumerate() {
        let input = diagnostics
            .item_inputs
            .iter()
            .find(|input| input.source_order == source_order)
            .unwrap_or_else(|| panic!("{}: source order {source_order} missing", scene.id));
        assert_eq!(input.node, expected_node, "{}", scene.id);
        assert_eq!(input.grid_container, root.id(), "{}", scene.id);
        assert_eq!(input.revision, diagnostics.revision, "{}", scene.id);
    }

    let text_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == text.id())
        .unwrap_or_else(|| panic!("{}: text intrinsic input missing", scene.id));
    let LayoutGridItemIntrinsicSource::OrthogonalText {
        min_content,
        max_content,
        child_writing_mode,
        parent_writing_mode,
        text_orientation,
        font_registry_revision,
        font_registry_identity,
    } = &text_input.source
    else {
        panic!("{}: expected orthogonal text source", scene.id);
    };
    assert_logical_size(*min_content, scene.min_logical(), scene);
    assert_logical_size(*max_content, scene.max_logical(), scene);
    assert_eq!(*child_writing_mode, WritingMode::VerticalRl, "{}", scene.id);
    assert_eq!(
        *parent_writing_mode,
        WritingMode::HorizontalTb,
        "{}",
        scene.id
    );
    assert_eq!(*text_orientation, TextOrientation::Mixed, "{}", scene.id);
    assert_eq!(
        font_registry_revision.0, 1,
        "{}: unexpected font registry revision in item source: {:#?}",
        scene.id, text_input.source
    );
    assert_eq!(
        font_registry_identity.font_registry_revision(),
        *font_registry_revision,
        "{}: item identity/revision mismatch",
        scene.id
    );
    let physical = text_input
        .contribution_input()
        .unwrap_or_else(|error| panic!("{}: contribution input: {error:?}", scene.id));
    assert_optional_near(physical.width.min_content, scene.min_logical().1, scene);
    assert_optional_near(physical.width.max_content, scene.max_logical().1, scene);
    assert_optional_near(physical.height.min_content, scene.min_logical().0, scene);
    assert_optional_near(physical.height.max_content, scene.max_logical().0, scene);

    let image_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == image.id())
        .unwrap_or_else(|| panic!("{}: image intrinsic input missing", scene.id));
    let LayoutGridItemIntrinsicSource::Replaced { natural } = &image_input.source else {
        panic!("{}: expected replaced image source", scene.id);
    };
    assert_optional_near(natural.width, 24.0, scene);
    assert_optional_near(natural.height, 16.0, scene);

    let text_measurements = diagnostics
        .intrinsic_measurements
        .iter()
        .filter(|measurement| measurement.node == text.id())
        .collect::<Vec<_>>();
    eprintln!(
        "{}: text intrinsic measurements: {text_measurements:#?}",
        scene.id
    );
    let orthogonal_measurements = text_measurements
        .iter()
        .filter(|measurement| {
            matches!(
                &measurement.source,
                LayoutIntrinsicMeasurementSource::OrthogonalText { .. }
            )
        })
        .collect::<Vec<_>>();
    assert!(
        !orthogonal_measurements.is_empty(),
        "{}: orthogonal measurement missing; all measurements: {text_measurements:#?}",
        scene.id
    );
    assert!(
        orthogonal_measurements.iter().all(|measurement| {
            let expected_min = scene.min_logical();
            let expected_max = scene.max_logical();
            measurement.revision == diagnostics.revision
                && matches!(
                    &measurement.source,
                    LayoutIntrinsicMeasurementSource::OrthogonalText {
                        min_content,
                        max_content,
                        child_writing_mode: WritingMode::VerticalRl,
                        parent_writing_mode: WritingMode::HorizontalTb,
                        text_orientation: TextOrientation::Mixed,
                        font_registry_revision: measured_font_revision,
                        font_registry_identity: measured_registry_identity,
                    } if near(min_content.inline_px, expected_min.0)
                        && near(min_content.block_px, expected_min.1)
                        && near(max_content.inline_px, expected_max.0)
                        && near(max_content.block_px, expected_max.1)
                        && *measured_font_revision == *font_registry_revision
                        && *measured_font_revision == measured_registry_identity.font_registry_revision()
                        && font_registry_identity.uses_same_font_registry_revision(measured_registry_identity)
                )
        }),
        "{}: orthogonal measurement provenance mismatch; all measurements: {text_measurements:#?}",
        scene.id,
    );

    assert_track_diagnostics(&diagnostics.traces, root.id(), text.id(), image.id(), scene);
}

fn assert_track_diagnostics(
    traces: &[spinon_layout::LayoutGridTrackSizingTrace],
    root: spinon_core::NodeId,
    text: spinon_core::NodeId,
    image: spinon_core::NodeId,
    scene: Scene,
) {
    assert_eq!(traces.len(), 1, "{}", scene.id);
    assert_eq!(traces[0].node, root, "{}", scene.id);
    let passes = &traces[0].passes;
    let last_columns = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .max_by_key(|pass| pass.sequence)
        .unwrap_or_else(|| panic!("{}: column pass missing", scene.id));
    let first_rows = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .min_by_key(|pass| pass.sequence)
        .unwrap_or_else(|| panic!("{}: row pass missing", scene.id));
    assert!(last_columns.sequence < first_rows.sequence, "{}", scene.id);
    assert_track_sizes(last_columns, &[50.0, 24.0], scene);
    let last_rows = passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .unwrap_or_else(|| panic!("{}: final row pass missing", scene.id));
    assert_track_sizes(last_rows, &[scene.row_height()], scene);
    assert!(!last_rows.contributions.is_empty(), "{}", scene.id);
    for contribution in &last_rows.contributions {
        let expected = match (contribution.item, contribution.kind) {
            (node, LayoutGridTrackSizingContributionKind::MinContent) if node == text => {
                scene.inline_contribution(true)
            }
            (node, LayoutGridTrackSizingContributionKind::MaxContent) if node == text => {
                scene.inline_contribution(false)
            }
            (node, LayoutGridTrackSizingContributionKind::MinContent)
            | (node, LayoutGridTrackSizingContributionKind::MaxContent)
                if node == image =>
            {
                16.0
            }
            (node, kind) => panic!(
                "{}: unexpected row contribution for {node:?}: {kind:?}",
                scene.id
            ),
        };
        assert_eq!(contribution.span, 1, "{}", scene.id);
        assert!(
            !contribution.crosses_flexible_track,
            "{}: fixed row contribution unexpectedly crosses a flexible track: {contribution:#?}",
            scene.id
        );
        assert_near(contribution.value, expected, scene);
    }
    for node in [text, image] {
        assert!(
            last_rows.contributions.iter().any(|contribution| {
                contribution.item == node && contribution.kind == scene.row_contribution_kind()
            }),
            "{}: {:?} row contribution missing for {node:?}",
            scene.id,
            scene.row_contribution_kind(),
        );
    }
}

fn assert_track_sizes(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    expected: &[f32],
    scene: Scene,
) {
    let tracks = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: track stage missing", scene.id))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), expected.len(), "{}", scene.id);
    for (actual, expected) in tracks.into_iter().zip(expected) {
        assert_near(actual, *expected, scene);
    }
}

fn assert_frame(
    frames: &[RuntimeLayoutFrame],
    node: u64,
    expected: (f32, f32, f32, f32),
    scene: Scene,
) {
    let actual = frames
        .iter()
        .find(|frame| frame.node_id == node)
        .unwrap_or_else(|| panic!("{}: frame missing for {node}", scene.id));
    for (actual, expected) in [
        (actual.x, expected.0),
        (actual.y, expected.1),
        (actual.width, expected.2),
        (actual.height, expected.3),
    ] {
        assert_near(actual, expected, scene);
    }
}

fn assert_logical_size(
    actual: spinon_text_layout::LogicalIntrinsicSize,
    expected: (f32, f32),
    scene: Scene,
) {
    assert_near(actual.inline_px, expected.0, scene);
    assert_near(actual.block_px, expected.1, scene);
}

fn assert_optional_near(actual: Option<f32>, expected: f32, scene: Scene) {
    let actual = actual.unwrap_or_else(|| panic!("{}: expected value {expected}", scene.id));
    assert_near(actual, expected, scene);
}

fn assert_near(actual: f32, expected: f32, scene: Scene) {
    assert!(
        near(actual, expected),
        "{}: expected {expected}, got {actual}",
        scene.id
    );
}

fn near(actual: f32, expected: f32) -> bool {
    actual.is_finite() && (actual - expected).abs() <= 1.0 / 64.0
}

fn debug_raw_runtime_output(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    text: HostNodeHandle,
    image: HostNodeHandle,
    viewport: CssViewport,
    styles: spinon_style::ComputedStyleSnapshot,
) -> String {
    let paragraph_styles = BTreeMap::from([(
        text.id(),
        ParagraphStyle {
            writing_mode: WritingMode::VerticalRl,
            text_orientation: TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: TextParagraphDirection::Ltr,
            line_break: LineBreakMode::Normal,
            ligatures: LigatureMode::Normal,
            letter_spacing: 0.0,
            word_spacing: 0.0,
            text_indent: 0.0,
        },
    )]);
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    if let Err(error) = engine.register_font("Ahem", AHEM) {
        return format!("debug font registration failed: {error:?}");
    }
    let Some(intrinsic_revision) = IntrinsicSizeRevision::INITIAL.checked_next() else {
        return "debug intrinsic revision exhausted".to_owned();
    };
    let intrinsic_sizes = match LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        intrinsic_revision,
        [(image.id(), LayoutIntrinsicSize::new(24.0, 16.0))],
    ) {
        Ok(intrinsic_sizes) => intrinsic_sizes,
        Err(error) => return format!("debug intrinsic snapshot failed: {error:?}"),
    };
    match spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
        snapshot,
        root,
        styles,
        viewport,
        &paragraph_styles,
        &engine,
        intrinsic_sizes,
    ) {
        Ok(output) => format!(
            "raw intrinsic measurements: {:#?}\nraw item inputs: {:#?}\nraw track traces: {:#?}",
            output.layout.intrinsic_measurements,
            output.layout.grid_item_intrinsic_inputs,
            output.layout.grid_track_sizing_traces
        ),
        Err(error) => format!("debug direct layout failed: {error:?}"),
    }
}
