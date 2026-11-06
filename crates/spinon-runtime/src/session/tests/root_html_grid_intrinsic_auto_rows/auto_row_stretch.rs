use super::*;
use spinon_render::RuntimePaintCommand;

const AUTO_ROW_STRETCH_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-auto-row-stretch-precomparison.html"
);
const AUTO_ROW_STRETCH_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-precomparison-2026-11-07/expected.json"
);

#[test]
fn parsed_root_html_default_auto_row_stretches_its_implicit_track() {
    let expected: Value = serde_json::from_str(AUTO_ROW_STRETCH_EXPECTED)
        .expect("auto-row stretch oracle should be valid JSON");
    let (session, layout) = run(AUTO_ROW_STRETCH_SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; layout diagnostics={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        layout.diagnostics,
        cascade.state,
        cascade.error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("the auto-row stretch scene should publish a complete layout");
    assert_eq!(
        completed.key,
        cascade
            .completed
            .as_ref()
            .expect("cascade should complete for the same source revision")
            .key,
        "style and layout revisions should match"
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the default Root HTML route should publish paint boxes");
    assert_eq!(
        scene.texts().len(),
        1,
        "the direct text source should paint once"
    );
    let tested_box_count = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if [
                    OpaqueCssSrgb::new(233, 237, 242),
                    OpaqueCssSrgb::new(216, 221, 227),
                    OpaqueCssSrgb::new(91, 159, 245),
                ]
                .contains(&color))
        })
        .count();
    assert_eq!(
        tested_box_count, 3,
        "one Grid and two direct items should paint"
    );

    let expected_grid = &expected["independent_expectations"];
    let grid_frame = painted_frame(
        scene,
        OpaqueCssSrgb::new(233, 237, 242),
        "auto-row stretch Grid",
    );
    assert_frame_values(grid_frame, [0.0, 0.0, 40.0, 40.0], "auto-row stretch Grid");
    let fixed_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(216, 221, 227))
        })
        .expect("the first fixed-height direct item should paint");
    let text_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(91, 159, 245))
        })
        .expect("the direct-text Grid item should paint");
    let expected_text_frame = expected_grid["auto-default"]["text_item_rect_grid_css_px"]
        .as_array()
        .expect("oracle should list the text item frame")
        .iter()
        .map(|value| value.as_f64().expect("frame component should be numeric") as f32)
        .collect::<Vec<_>>();
    let expected_text_frame: [f32; 4] = expected_text_frame
        .try_into()
        .expect("text item frame should have four components");
    let text_frame = text_box.frame_css_px();
    assert_frame_values(
        [
            text_frame.x(),
            text_frame.y(),
            text_frame.width(),
            text_frame.height(),
        ],
        expected_text_frame,
        "auto-row stretch text item",
    );
    let fixed_frame = fixed_box.frame_css_px();
    assert_frame_values(
        [
            fixed_frame.x(),
            fixed_frame.y(),
            fixed_frame.width(),
            fixed_frame.height(),
        ],
        [0.0, 0.0, 40.0, 10.0],
        "auto-row stretch fixed item",
    );
    assert!(
        completed
            .frames
            .iter()
            .any(|frame| frame.node_id == text_box.node_id().get()),
        "the text item's NodeId should be retained in completed frames"
    );

    let text = &scene.texts()[0];
    assert_eq!(
        text.glyphs().len(),
        6,
        "spaces do not produce glyph bitmaps"
    );
    let baselines = text
        .glyphs()
        .iter()
        .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(baselines.len(), 2, "the text should paint on two lines");
    assert_eq!(text.origin_y_css_px(), 10.0);
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

    let fixed_position = scene
        .boxes()
        .iter()
        .position(|render_box| render_box.node_id() == fixed_box.node_id())
        .expect("fixed item should preserve its scene identity");
    let text_position = scene
        .boxes()
        .iter()
        .position(|render_box| render_box.node_id() == text_box.node_id())
        .expect("text item should preserve its scene identity");
    assert!(
        fixed_position < text_position,
        "paint boxes should preserve source order"
    );
    let fixed_draw = scene
        .paint_commands()
        .iter()
        .position(|command| {
            matches!(command, RuntimePaintCommand::DrawBox { index } if *index == fixed_position)
        })
        .expect("the fixed item box should have a draw command");
    let text_draw = scene
        .paint_commands()
        .iter()
        .position(
            |command| matches!(command, RuntimePaintCommand::DrawText { index } if *index == 0),
        )
        .expect("the text should have a draw command");
    assert!(
        fixed_draw < text_draw,
        "the fixed item should paint before direct text"
    );

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    assert_auto_row_stretch_trace(completed, text_box.node_id());
}

#[test]
fn parsed_root_html_auto_row_stretch_rejects_wrong_alignment_and_extra_items() {
    for (label, source) in [
        (
            "wrong align-content",
            AUTO_ROW_STRETCH_SOURCE.replace("align-content: normal;", "align-content: start;"),
        ),
        (
            "wrong align-items",
            AUTO_ROW_STRETCH_SOURCE.replace("align-items: normal;", "align-items: start;"),
        ),
        (
            "extra direct item",
            AUTO_ROW_STRETCH_SOURCE.replace(
                "</div>\n  </body>",
                "<div id=\"extra\"></div></div>\n  </body>",
            ),
        ),
        (
            "unsupported text source",
            AUTO_ROW_STRETCH_SOURCE.replace("A A A A A A", "A&nbsp;A"),
        ),
        (
            "non-candidate grid width",
            AUTO_ROW_STRETCH_SOURCE.replace("width: 40px;", "width: 41px;"),
        ),
        (
            "unsupported implicit row sizing",
            AUTO_ROW_STRETCH_SOURCE
                .replace("grid-auto-rows: auto;", "grid-auto-rows: max-content;"),
        ),
    ] {
        assert_ne!(
            source, AUTO_ROW_STRETCH_SOURCE,
            "{label}: mutation should apply"
        );
        assert_rejected_without_layout(&source, label);
    }
}

#[test]
fn parsed_root_html_auto_row_stretch_recovers_after_rejected_environment() {
    let source = AUTO_ROW_STRETCH_SOURCE.replace(
        "      #fixed {",
        "      @media (prefers-color-scheme: dark) {\n        #grid { align-content: start; }\n      }\n\n      #fixed {",
    );
    assert_ne!(source, AUTO_ROW_STRETCH_SOURCE);
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("media-query Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light Root HTML metadata should configure the CSS environment");
    let initial = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());

    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("dark Root HTML metadata should configure the CSS environment");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.error.is_some());
    assert!(rejected.completed.is_none());

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light Root HTML metadata should restore the candidate style");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={:?}",
        recovered.error,
        session.ua_cascade_snapshot()
    );
    let scene = recovered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("supported auto-row stretch should publish a render snapshot after recovery");
    let frame = painted_frame(
        &scene,
        OpaqueCssSrgb::new(91, 159, 245),
        "recovered auto-row stretch text",
    );
    assert_frame_values(
        frame,
        [0.0, 10.0, 40.0, 30.0],
        "recovered auto-row stretch text",
    );
}

fn assert_rejected_without_layout(source: &str, label: &str) {
    let (_session, layout) = run(source);
    assert_ne!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{label}: unsupported source should not produce a ready layout"
    );
    assert!(
        layout.completed.is_none(),
        "{label}: rejected source must not publish a partial layout"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_auto_row_stretch_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    text_item: spinon_core::NodeId,
) {
    use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};

    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture-test diagnostics should retain Root HTML track sizing");
    let text_layout = diagnostics
        .item_text_layouts
        .iter()
        .find(|layout| layout.input.node == text_item)
        .expect("direct-text Grid item should retain its text layout");
    assert_eq!(text_layout.input.sources.len(), 1);
    assert_eq!(text_layout.input.sources[0].text, "A A A A A A");
    assert_eq!(text_layout.layout.lines.len(), 2);
    assert_eq!(text_layout.layout.height, 20.0);

    let item_input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == text_item)
        .expect("direct-text Grid item should retain its intrinsic input");
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == item_input.grid_container)
        .expect("Grid owner should retain the track-sizing trace");
    assert_eq!(trace.used_rows.used_track_list, "10px 30px");
    assert_eq!(trace.used_rows.explicit_track_count, 1);
    assert_eq!(trace.used_rows.positive_implicit_track_count, 1);

    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    assert!(
        !row_passes.is_empty(),
        "Grid trace should include a row sizing pass"
    );
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
    assert_eq!(applied_stages.len(), 1, "auto stretch should run once");
    let (pass, stage_index, stretch_stage) = applied_stages[0];
    let before_stage = pass
        .stages
        .get(
            stage_index
                .checked_sub(1)
                .expect("stretch should not be the first stage"),
        )
        .expect("stretch should follow intrinsic sizing or track maximization");
    assert_track_bases(before_stage, &[10.0, 20.0], "before auto stretch");
    assert_track_bases(stretch_stage, &[10.0, 30.0], "after auto stretch");
    let row_contribution = pass
        .contributions
        .iter()
        .find(|contribution| contribution.item == text_item)
        .expect("text item should contribute to its implicit row");
    assert_eq!(
        row_contribution.kind,
        spinon_layout::LayoutGridTrackSizingContributionKind::Minimum
    );
    assert_eq!(row_contribution.value, 20.0);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_bases(
    stage: &spinon_layout::LayoutGridTrackSizingStageTrace,
    expected: &[f32],
    label: &str,
) {
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{label}");
}
