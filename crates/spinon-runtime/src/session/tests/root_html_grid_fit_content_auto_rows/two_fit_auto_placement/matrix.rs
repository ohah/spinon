use super::*;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn parsed_root_html_two_fit_auto_minimum_media_schemes_skip_hidden_grid_alignment() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};
    use std::time::Duration;

    let source = include_str!(
        "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-simulators-2026-11-10/root-html-auto-minimum-runtime.html"
    );
    let (session, _) = crate::session::RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("app fixture initializes");
    for (label, color_scheme, visible_alignment) in [
        ("light", CssColorScheme::Light, "start"),
        ("dark", CssColorScheme::Dark, "stretch"),
    ] {
        let mut media = CssMediaEnvironment::MOBILE;
        media.color_scheme = color_scheme;
        let resolved = session
            .set_root_html_environment(402.0, 778.0, 3.0, media)
            .expect("environment applies");
        let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: {:?}",
            layout.error
        );
        let completed = layout.completed.expect("visible Grid publishes layout");
        assert_eq!(
            completed.key.environment_revision,
            resolved.environment_revision.get(),
            "{label}: current environment publishes layout"
        );
        let cascade = session.ua_cascade_snapshot();
        let styles = &cascade
            .completed
            .expect("cascade publishes same environment")
            .roots[0]
            .styles
            .elements;
        let grids = styles
            .iter()
            .filter(|style| style.grid_layout.is_some())
            .collect::<Vec<_>>();
        assert_eq!(
            grids.len(),
            1,
            "{label}: only the visible media Grid lays out"
        );
        assert_eq!(
            grids[0].properties.get("align-content").map(String::as_str),
            Some(visible_alignment),
            "{label}: visible Grid uses its media alignment"
        );
    }
}

#[test]
fn parsed_root_html_two_fit_auto_placement_matches_all_2880_chrome_scenes() {
    assert_matches_chrome_scenes(super::scenes(), false);
}

#[test]
fn parsed_root_html_two_fit_auto_placement_auto_minimum_matches_all_2880_chrome_scenes() {
    assert_matches_chrome_scenes(super::auto_minimum_scenes(), true);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn parsed_root_html_two_fit_auto_minimum_trace_preserves_floor_above_zero_caps() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
        LayoutGridTrackSizingStageKind,
    };

    let scene = auto_minimum_scenes()
        .into_iter()
        .find(|scene| scene["scene"]["index"] == 0)
        .expect("Chrome matrix includes the zero-cap first scene");
    let (session, layout) = run(&source_for_auto_minimum(&scene));
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Ready);
    let completed = layout.completed.as_ref().expect("scene publishes layout");
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes the track-sizing trace");
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    let used_row_count = usize::from(trace.used_rows.explicit_track_count)
        + usize::from(trace.used_rows.negative_implicit_track_count)
        + usize::from(trace.used_rows.positive_implicit_track_count);
    assert_eq!(used_row_count, 9, "one explicit plus eight implicit rows");
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0);
    assert_eq!(
        trace.used_rows.used_track_list.split_whitespace().count(),
        9,
        "resolved trace preserves every used row"
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the same revision publishes computed styles");
    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(trace.node.get(), grid.node_id.get());
    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("the final row-sizing pass is recorded");
    let final_stage = row_pass
        .stages
        .iter()
        .find(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        })
        .expect("the final auto-stretch stage is recorded");

    let expected_items = scene["items"].as_array().unwrap();
    for text_index in 0..2 {
        let color = ITEM_COLORS[text_index + 1];
        let style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| {
                style.background_color.is_some_and(|actual| {
                    (actual.red, actual.green, actual.blue)
                        == (color.red(), color.green(), color.blue())
                })
            })
            .expect("each zero-cap fit row has a computed item style");
        let minimums = row_pass
            .contributions
            .iter()
            .filter(|contribution| {
                contribution.item == style.node_id
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            })
            .collect::<Vec<_>>();
        assert_eq!(minimums.len(), 1, "fit item {text_index}: minimum trace");
        let minimum = minimums[0];
        let content_based = matches!(
            minimum.minimum_source,
            Some(MinimumSource::AutomaticMinimum {
                spans_auto_min_track: true,
                spans_multiple_tracks: false,
                spans_flexible_max_track: false,
                content_based: true,
                is_compressible_replaced: false,
                ..
            })
        );
        assert!(content_based, "fit item {text_index}: {minimum:?}");
        let expected_lines = expected_items[text_index + 1]["lineCount"]
            .as_u64()
            .expect("Chrome records the payload line count") as f32;
        let expected_floor = expected_lines * 10.0;
        assert!(
            minimum.value.is_finite() && (minimum.value - expected_floor).abs() <= 0.01,
            "fit item {text_index}: automatic minimum {:?}, expected {expected_floor}px",
            minimum.value
        );

        let row_internal_index = u16::try_from((text_index + 1) * 2 + 1).unwrap();
        let row = final_stage
            .tracks
            .iter()
            .find(|track| !track.is_gutter && track.internal_index == row_internal_index)
            .expect("fit row has a final track state");
        assert!(
            row.base_size + 0.01 >= minimum.value,
            "fit item {text_index}: row {:?} dropped below its minimum {:?}",
            row.base_size,
            minimum.value
        );
        assert!(
            minimum.value > 0.0,
            "zero cap must not clamp the content floor"
        );
    }
}

fn assert_matches_chrome_scenes(scenes: Vec<serde_json::Value>, auto_minimum: bool) {
    assert_eq!(scenes.len(), 2_880);
    let track_lists = scenes
        .iter()
        .map(|scene| scene["scene"]["trackList"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(track_lists.len(), 1_440, "start/stretch share track lists");
    let fit_pairs = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitPositions"]).unwrap())
        .collect::<BTreeSet<_>>();
    let tail_orders = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["tail"]).unwrap())
        .collect::<BTreeSet<_>>();
    let cap_assignments = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitCaps"]).unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(fit_pairs.len(), 15);
    assert_eq!(tail_orders.len(), 12);
    assert_eq!(cap_assignments.len(), 8);

    let mut overflow_count = 0;
    let mut fit_row_occurrences = 0;
    let mut auto_stretch_pairs = BTreeMap::<String, usize>::new();
    for (scene_index, expected_scene) in scenes.iter().enumerate() {
        let source = if auto_minimum {
            source_for_auto_minimum(expected_scene)
        } else {
            source_for(expected_scene)
        };
        let (session, layout) = run(&source);
        let cascade_snapshot = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index}: layout={:?}; cascade={:?}",
            layout.error,
            cascade_snapshot.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("a supported scene publishes complete layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("a supported scene publishes its render snapshot");
        let cascade = cascade_snapshot
            .completed
            .expect("the same revision publishes computed styles");
        assert_eq!(
            completed.key, cascade.key,
            "scene {scene_index}: revision key"
        );
        let styles = &cascade.roots[0].styles.elements;
        let grid_style = styles
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("fixture has one Grid owner");
        let expected_tracks = computed_tracks(expected_scene);
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "scene {scene_index}: typed caps and authored member order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(String::as_str),
            Some(expected_scene["computedAutoRows"].as_str().unwrap()),
            "scene {scene_index}: serialized computed track list"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            Some(expected_scene["computedAlignContent"].as_str().unwrap()),
            "scene {scene_index}: computed content alignment"
        );
        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame should be published");
        assert_frame(
            runtime_frame(grid_frame),
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["items"]
            .as_array()
            .expect("Chrome item frames should be an array");
        assert_eq!(expected_items.len(), ITEM_COLORS.len());
        let expected_rows = expected_scene["rows"]
            .as_array()
            .expect("Chrome used rows should be an array");
        assert_eq!(expected_rows.len(), expected_items.len());
        for (item_index, (expected_item, color)) in
            expected_items.iter().zip(ITEM_COLORS).enumerate()
        {
            let item_styles = styles
                .iter()
                .filter(|style| {
                    style.background_color.is_some_and(|actual| {
                        (actual.red, actual.green, actual.blue)
                            == (color.red(), color.green(), color.blue())
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(
                item_styles.len(),
                1,
                "scene {scene_index} item {item_index}: unique style"
            );
            if auto_minimum {
                assert_eq!(
                    item_styles[0]
                        .properties
                        .get("min-height")
                        .map(String::as_str),
                    expected_item["minHeight"].as_str(),
                    "scene {scene_index} item {item_index}: computed min-height"
                );
            }
            let actual_item = frame_for_background(
                completed,
                styles,
                color,
                &format!("scene {scene_index} item {item_index}"),
            );
            let expected_item_frame = frame(&expected_item["frame"]);
            assert_frame(
                actual_item,
                expected_item_frame,
                &format!("scene {scene_index} item {item_index}"),
            );
            assert_frame(
                [actual_item[3], 0.0, 0.0, 0.0],
                [
                    expected_rows[item_index].as_f64().unwrap() as f32,
                    0.0,
                    0.0,
                    0.0,
                ],
                &format!("scene {scene_index} row {item_index}"),
            );
            fit_row_occurrences += usize::from(
                expected_scene["scene"]["fitPositions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|position| {
                        position.as_u64().unwrap() as usize
                            == (item_index.saturating_sub(1) % 6) + 1
                    })
                    && item_index > 0,
            );
        }

        let mut texts = render.texts().iter().collect::<Vec<_>>();
        texts.sort_unstable_by_key(|text| text.node_id().get());
        assert_eq!(
            texts.len(),
            PAYLOAD_TEXTS.len(),
            "scene {scene_index} text nodes"
        );
        let mut actual_payload_bottom = f32::NEG_INFINITY;
        for (text_index, text) in texts.into_iter().enumerate() {
            let expected_item = &expected_items[text_index + 1];
            let expected_payload = frame(&expected_item["payload"]);
            let actual_payload = frame_for_payload(
                completed,
                text.node_id().get(),
                &format!("scene {scene_index} payload {text_index}"),
            );
            assert_frame(
                actual_payload,
                expected_payload,
                &format!("scene {scene_index} payload {text_index}"),
            );
            let expected_glyph_count = PAYLOAD_TEXTS[text_index]
                .bytes()
                .filter(|byte| byte.is_ascii_uppercase())
                .count();
            assert_eq!(
                text.glyphs().len(),
                expected_glyph_count,
                "scene {scene_index} payload {text_index} glyph count"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                expected_item["lineCount"].as_u64().unwrap() as usize,
                "scene {scene_index} payload {text_index} line count"
            );
            actual_payload_bottom =
                actual_payload_bottom.max(actual_payload[1] + actual_payload[3]);
        }
        let has_overflow = actual_payload_bottom > 220.01;
        assert_eq!(
            has_overflow,
            expected_items.iter().skip(1).any(|item| {
                let payload = frame(&item["payload"]);
                payload[1] + payload[3] > 220.01
            }),
            "scene {scene_index}: visible text overflow"
        );
        overflow_count += usize::from(has_overflow);
        if expected_scene["scene"]["alignContent"] == "stretch" {
            *auto_stretch_pairs
                .entry(
                    expected_scene["scene"]["trackList"]
                        .as_str()
                        .unwrap()
                        .to_owned(),
                )
                .or_default() += 1;
        }
    }
    assert_eq!(overflow_count, if auto_minimum { 0 } else { 288 });
    assert_eq!(fit_row_occurrences, 7_680);
    assert_eq!(auto_stretch_pairs.len(), 1_440);
    assert!(auto_stretch_pairs.values().all(|count| *count == 1));
}
