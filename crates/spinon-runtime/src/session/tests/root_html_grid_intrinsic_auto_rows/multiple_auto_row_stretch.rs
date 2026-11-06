use super::*;
use serde_json::Value;
use spinon_render::RuntimePaintCommand;

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-multiple-auto-row-stretch-runtime.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-multiple-auto-row-stretch-precomparison-2026-11-07/expected.json"
);

#[test]
fn parsed_root_html_stretches_two_implicit_auto_rows_equally() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("multiple auto-row stretch oracle should be valid JSON");
    let (session, layout) = run(SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}; cascade={:?}",
        layout.error,
        layout.diagnostics,
        cascade.error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("two implicit auto rows should publish a complete layout");
    assert_eq!(
        completed.key,
        cascade
            .completed
            .as_ref()
            .expect("the style snapshot should complete")
            .key,
        "style and layout revisions should match"
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the Root HTML route should publish paint boxes");
    assert_eq!(scene.texts().len(), 2, "both direct texts should paint");

    let color_box = |color| {
        scene
            .boxes()
            .iter()
            .filter(|render_box| {
                matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
            })
            .collect::<Vec<_>>()
    };
    let grid = color_box(OpaqueCssSrgb::new(233, 237, 242));
    let fixed = color_box(OpaqueCssSrgb::new(216, 221, 227));
    let mut text_boxes = color_box(OpaqueCssSrgb::new(91, 159, 245));
    assert_eq!(grid.len(), 1);
    assert_eq!(fixed.len(), 1);
    assert_eq!(text_boxes.len(), 2);
    text_boxes.sort_by(|left, right| left.frame_css_px().y().total_cmp(&right.frame_css_px().y()));

    let expected_grid = &expected["independent_expectations"]["auto-normal-two-implicit"];
    assert_frame_values(
        [
            grid[0].frame_css_px().x(),
            grid[0].frame_css_px().y(),
            grid[0].frame_css_px().width(),
            grid[0].frame_css_px().height(),
        ],
        [0.0, 0.0, 40.0, 40.0],
        "two-row stretch Grid",
    );
    let expected_frames = expected_grid["item_rects_grid_css_px"]
        .as_array()
        .expect("oracle should list all direct item frames")
        .iter()
        .map(|frame| {
            frame
                .as_array()
                .expect("item frame should be an array")
                .iter()
                .map(|value| value.as_f64().expect("frame component should be numeric") as f32)
                .collect::<Vec<_>>()
                .try_into()
                .expect("item frame should have four components")
        })
        .collect::<Vec<[f32; 4]>>();
    assert_eq!(expected_frames.len(), 3);
    let fixed_frame = fixed[0].frame_css_px();
    assert_frame_values(
        [
            fixed_frame.x(),
            fixed_frame.y(),
            fixed_frame.width(),
            fixed_frame.height(),
        ],
        expected_frames[0],
        "fixed direct item",
    );
    for (index, render_box) in text_boxes.iter().enumerate() {
        let frame = render_box.frame_css_px();
        assert_frame_values(
            [frame.x(), frame.y(), frame.width(), frame.height()],
            expected_frames[index + 1],
            &format!("text item {index}"),
        );
        assert!(
            completed
                .frames
                .iter()
                .any(|entry| entry.node_id == render_box.node_id().get()),
            "text item {index} should retain its NodeId in completed frames"
        );
    }

    for (index, text) in scene.texts().iter().enumerate() {
        assert_eq!(text.glyphs().len(), 1, "each direct text is one glyph");
        assert_eq!(text.origin_y_css_px(), [10.0, 25.0][index]);
        assert_eq!(text.source().generation().get(), completed.key.generation);
        assert_eq!(
            text.source().document_revision().get(),
            completed.key.document_revision
        );
        assert_eq!(
            text.source().render_tree_revision().get(),
            completed.key.render_tree_revision
        );
        assert_eq!(text.source().style_revision(), completed.key.style_revision);
    }
    assert!(
        text_boxes[0].frame_css_px().y() < text_boxes[1].frame_css_px().y(),
        "text item frames should preserve source order"
    );

    let fixed_box_index = scene
        .boxes()
        .iter()
        .position(|render_box| render_box.node_id() == fixed[0].node_id())
        .expect("fixed item should retain its render box");
    let first_text_draw = scene
        .paint_commands()
        .iter()
        .position(|command| matches!(command, RuntimePaintCommand::DrawText { index: 0 }))
        .expect("first direct text should have a draw command");
    let second_text_draw = scene
        .paint_commands()
        .iter()
        .position(|command| matches!(command, RuntimePaintCommand::DrawText { index: 1 }))
        .expect("second direct text should have a draw command");
    let fixed_draw = scene
        .paint_commands()
        .iter()
        .position(|command| {
            matches!(command, RuntimePaintCommand::DrawBox { index } if *index == fixed_box_index)
        })
        .expect("fixed item should have a draw command");
    assert!(fixed_draw < first_text_draw && first_text_draw < second_text_draw);

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    assert_two_auto_row_stretch_trace(completed, &text_boxes);
}

#[test]
fn parsed_root_html_multiple_auto_row_stretch_rejects_other_shapes() {
    for (label, source) in [
        (
            "wrong alignment",
            SOURCE.replace("align-content: normal;", "align-content: start;"),
        ),
        (
            "wrong first text",
            SOURCE.replace("id=\"text-a\">A", "id=\"text-a\">C"),
        ),
        (
            "wrong second text",
            SOURCE.replace("id=\"text-b\">B", "id=\"text-b\">A"),
        ),
        (
            "reordered text items",
            SOURCE.replace(
                "<div id=\"text-a\">A</div><div id=\"text-b\">B</div>",
                "<div id=\"text-b\">B</div><div id=\"text-a\">A</div>",
            ),
        ),
        (
            "fourth item",
            SOURCE.replace(
                "</div>\n  </body>",
                "<div id=\"extra\"></div></div>\n  </body>",
            ),
        ),
    ] {
        assert_ne!(source, SOURCE, "{label}: mutation should apply");
        let (_session, layout) = run(&source);
        assert_ne!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: unsupported source must be rejected"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: rejection must not publish partial layout"
        );
    }
}

#[test]
fn parsed_root_html_multiple_auto_row_stretch_recovers_after_rejection() {
    let source = SOURCE.replace(
        "      #fixed {",
        "      @media (prefers-color-scheme: dark) {\n        #grid { align-content: start; }\n      }\n\n      #fixed {",
    );
    assert_ne!(source, SOURCE);
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("the multiple-row Root HTML scene should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light mode should configure the CSS environment");
    assert_eq!(
        session
            .wait_for_layout_snapshot(Duration::from_secs(5))
            .state,
        crate::session::RuntimeLayoutState::Ready
    );

    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("dark mode should update the CSS environment");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none());

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light mode should restore the accepted profile");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "recovery error={:?}",
        recovered.error
    );
    let scene = recovered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("recovered scene should publish both text rows");
    let mut frames = scene
        .boxes()
        .iter()
        .filter_map(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(91, 159, 245))
            .then_some(render_box.frame_css_px())
        })
        .map(|frame| [frame.x(), frame.y(), frame.width(), frame.height()])
        .collect::<Vec<_>>();
    frames.sort_by(|left, right| left[1].total_cmp(&right[1]));
    assert_eq!(frames, [[0.0, 10.0, 40.0, 15.0], [0.0, 25.0, 40.0, 15.0]]);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_two_auto_row_stretch_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    text_boxes: &[&spinon_render::RuntimeRenderBox],
) {
    use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};

    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture-test diagnostics should retain track sizing");
    let item_layouts = text_boxes
        .iter()
        .map(|render_box| {
            diagnostics
                .item_text_layouts
                .iter()
                .find(|layout| layout.input.node == render_box.node_id())
                .expect("each direct-text item should retain its measured layout")
        })
        .collect::<Vec<_>>();
    assert_eq!(item_layouts.len(), 2);
    for (index, layout) in item_layouts.iter().enumerate() {
        let expected_text = ["A", "B"][index];
        assert_eq!(layout.layout.lines.len(), 1);
        assert_eq!(layout.layout.height, 10.0);
        assert_eq!(layout.input.sources.len(), 1);
        assert_eq!(layout.input.sources[0].text, expected_text);
    }

    let item_inputs = item_layouts
        .iter()
        .map(|layout| {
            diagnostics
                .item_inputs
                .iter()
                .find(|input| input.node == layout.input.node)
                .expect("each text item should retain intrinsic contribution input")
        })
        .collect::<Vec<_>>();
    let owner = item_inputs[0].grid_container;
    assert!(
        item_inputs
            .iter()
            .all(|input| input.grid_container == owner)
    );
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == owner)
        .expect("Grid owner should retain a track-sizing trace");
    assert_eq!(trace.used_rows.used_track_list, "10px 15px 15px");
    assert_eq!(trace.used_rows.explicit_track_count, 1);
    assert_eq!(trace.used_rows.positive_implicit_track_count, 2);

    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    let applied_stages = row_passes
        .iter()
        .flat_map(|pass| {
            pass.stages
                .iter()
                .enumerate()
                .filter_map(move |(index, stage)| {
                    (stage.stage
                        == LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true })
                    .then_some((*pass, index, stage))
                })
        })
        .collect::<Vec<_>>();
    assert_eq!(applied_stages.len(), 1, "auto-row stretch should run once");
    let (pass, stage_index, stretch) = applied_stages[0];
    let before = pass
        .stages
        .get(
            stage_index
                .checked_sub(1)
                .expect("stretch has a preceding stage"),
        )
        .expect("the preceding track sizing stage should be recorded");
    assert_track_bases(before, &[10.0, 10.0, 10.0]);
    assert_track_bases(stretch, &[10.0, 15.0, 15.0]);

    for (index, input) in item_inputs.into_iter().enumerate() {
        assert_eq!(input.source_order, index + 1);
        let contribution = pass
            .contributions
            .iter()
            .find(|contribution| contribution.item == input.node)
            .expect("each text item should contribute to its own implicit row");
        assert_eq!(
            contribution.kind,
            spinon_layout::LayoutGridTrackSizingContributionKind::Minimum
        );
        assert_eq!(contribution.value, 10.0);
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_bases(stage: &spinon_layout::LayoutGridTrackSizingStageTrace, expected: &[f32]) {
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
