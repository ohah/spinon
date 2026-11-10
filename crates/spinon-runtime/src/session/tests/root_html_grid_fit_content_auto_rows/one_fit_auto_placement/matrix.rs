use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
    LayoutGridTrackSizingStageKind,
};
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn parsed_root_html_one_fit_auto_minimum_matches_all_1440_chrome_scenes() {
    let baseline_by_key = super::assert_scene_key_sets_match();
    let scenes = super::auto_minimum_scenes();
    let mut rows_by_track_list = BTreeMap::<String, [Option<Vec<f32>>; 2]>::new();
    let mut fit_row_occurrences = 0usize;
    let mut floor_rescue_occurrences = 0usize;
    let mut overflow_scenes = 0usize;

    for (scene_index, scene) in scenes.iter().enumerate() {
        let key = super::scene_key(scene);
        let baseline = &baseline_by_key[&key];
        let source = super::source_for(scene, true);
        let (session, layout) = super::super::run(&source);
        let cascade_snapshot = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index} {key}: layout={:?}; cascade={:?}",
            layout.error,
            cascade_snapshot.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported scene publishes layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("supported scene publishes render output");
        let cascade = cascade_snapshot
            .completed
            .expect("same revision publishes computed styles");
        assert_eq!(completed.key, cascade.key, "scene {scene_index}: revision");
        let styles = &cascade.roots[0].styles.elements;
        let grid = styles
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("scene contains one Grid owner");
        assert_eq!(
            grid.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(super::computed_tracks(scene)),
            "scene {scene_index}: authored track types and order"
        );
        assert_eq!(
            grid.properties.get("grid-auto-rows").map(String::as_str),
            Some(scene["computedAutoRows"].as_str().unwrap()),
            "scene {scene_index}: computed track serialization"
        );
        assert_eq!(
            grid.properties.get("align-content").map(String::as_str),
            Some(scene["align"].as_str().unwrap()),
            "scene {scene_index}: alignment"
        );
        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid.node_id.get())
            .expect("Grid frame is published");
        super::super::assert_frame(
            [
                grid_frame.x,
                grid_frame.y,
                grid_frame.width,
                grid_frame.height,
            ],
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = scene["items"].as_array().expect("items are recorded");
        let expected_rows = super::rows(scene);
        assert_eq!(expected_items.len(), 9);
        assert_eq!(expected_rows.len(), expected_items.len());
        for (item_index, (expected, color)) in expected_items
            .iter()
            .zip(super::super::two_fit_auto_placement::ITEM_COLORS)
            .enumerate()
        {
            let matching_styles = styles
                .iter()
                .filter(|style| {
                    style.background_color.is_some_and(|actual| {
                        (actual.red, actual.green, actual.blue)
                            == (color.red(), color.green(), color.blue())
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(
                matching_styles.len(),
                1,
                "scene {scene_index} item {item_index}: style"
            );
            assert_eq!(
                matching_styles[0]
                    .properties
                    .get("min-height")
                    .map(String::as_str),
                Some(expected["minHeight"].as_str().unwrap()),
                "scene {scene_index} item {item_index}: computed minimum"
            );
            let actual = super::super::two_fit_auto_placement::frame_for_background(
                completed,
                styles,
                color,
                &format!("scene {scene_index} item {item_index}"),
            );
            let expected_frame = super::expected_frame(&expected["frame"]);
            super::super::assert_frame(
                actual,
                expected_frame,
                &format!("scene {scene_index} item {item_index}"),
            );
            assert!((actual[3] - expected_rows[item_index]).abs() <= 0.01);

            if item_index > 0
                && (item_index - 1) % 6 == scene["fitPosition"].as_u64().unwrap() as usize - 1
            {
                fit_row_occurrences += 1;
                let floor = expected["lineCount"].as_u64().unwrap() as f32 * 10.0;
                assert!(
                    actual[3] + 0.01 >= floor,
                    "scene {scene_index} fit item {item_index} fell below {floor}px content floor"
                );
                let baseline_height = baseline["rows"][item_index].as_f64().unwrap() as f32;
                if baseline_height + 0.01 < floor {
                    floor_rescue_occurrences += 1;
                    assert!(
                        actual[3] + 0.01 >= floor,
                        "scene {scene_index} did not rescue baseline row {baseline_height} to {floor}"
                    );
                }
            }
        }

        let mut texts = render.texts().iter().collect::<Vec<_>>();
        texts.sort_unstable_by_key(|text| text.node_id().get());
        assert_eq!(
            texts.len(),
            super::super::two_fit_auto_placement::PAYLOAD_TEXTS.len()
        );
        let mut content_bottom = f32::NEG_INFINITY;
        for (text_index, text) in texts.into_iter().enumerate() {
            let expected_item = &expected_items[text_index + 1];
            let actual_payload = super::super::two_fit_auto_placement::frame_for_payload(
                completed,
                text.node_id().get(),
                &format!("scene {scene_index} payload {text_index}"),
            );
            super::super::assert_frame(
                actual_payload,
                super::expected_frame(&expected_item["payload"]),
                &format!("scene {scene_index} payload {text_index}"),
            );
            let expected_text = super::super::two_fit_auto_placement::PAYLOAD_TEXTS[text_index];
            assert_eq!(
                text.glyphs().len(),
                expected_text.bytes().filter(u8::is_ascii_uppercase).count(),
                "scene {scene_index} payload {text_index}: glyph count"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                expected_item["lineCount"].as_u64().unwrap() as usize,
                "scene {scene_index} payload {text_index}: line count"
            );
            content_bottom = content_bottom.max(actual_payload[1] + actual_payload[3]);
        }
        let has_overflow = content_bottom > 220.01;
        assert!(
            !has_overflow,
            "scene {scene_index}: visible content overflow"
        );
        overflow_scenes += usize::from(has_overflow);

        let pair = rows_by_track_list
            .entry(super::track_list_key(scene))
            .or_insert([None, None]);
        let alignment_slot = if scene["align"].as_str() == Some("stretch") {
            1
        } else {
            0
        };
        assert!(pair[alignment_slot].replace(expected_rows).is_none());
    }

    assert_eq!(fit_row_occurrences, 1_920);
    assert_eq!(floor_rescue_occurrences, 600);
    assert_eq!(overflow_scenes, 0);
    assert_eq!(rows_by_track_list.len(), 720);
    assert_stretch_only_increases_auto_rows(&rows_by_track_list);
}

fn assert_stretch_only_increases_auto_rows(
    rows_by_track_list: &BTreeMap<String, [Option<Vec<f32>>; 2]>,
) {
    assert_eq!(rows_by_track_list.len(), 720);
    for (track_list, pair) in rows_by_track_list {
        let start = pair[0].as_ref().expect("start scene paired");
        let stretch = pair[1].as_ref().expect("stretch scene paired");
        assert_eq!(start.len(), 9, "{track_list}: explicit plus implicit rows");
        assert_eq!(
            stretch.len(),
            9,
            "{track_list}: explicit plus implicit rows"
        );
        assert!((start.iter().sum::<f32>() - 180.0).abs() <= 0.01);
        assert!((stretch.iter().sum::<f32>() - 220.0).abs() <= 0.01);
        let tracks = track_list.split_whitespace().collect::<Vec<_>>();
        let mut auto_deltas = Vec::new();
        for row in 1..9 {
            let delta = stretch[row] - start[row];
            if tracks[(row - 1) % tracks.len()] == "auto" {
                assert!(delta > 0.0, "{track_list}: auto track receives stretch");
                auto_deltas.push(delta);
            } else {
                assert!(
                    delta.abs() <= 0.01,
                    "{track_list}: non-auto row {row} changed by {delta}"
                );
            }
        }
        assert!(
            !auto_deltas.is_empty(),
            "{track_list}: auto occurrence exists"
        );
        assert!(
            auto_deltas
                .windows(2)
                .all(|pair| (pair[0] - pair[1]).abs() <= 0.01)
        );
        assert!((auto_deltas.iter().sum::<f32>() - 40.0).abs() <= 0.01);
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn parsed_root_html_one_fit_zero_cap_trace_uses_content_floor_above_cap() {
    let scene = super::auto_minimum_scenes()
        .into_iter()
        .find(|scene| {
            scene["fitPosition"].as_u64() == Some(1)
                && scene["cap"].as_str() == Some("0%")
                && scene["align"].as_str() == Some("start")
        })
        .expect("matrix includes one-fit zero-cap start scene");
    let (session, layout) = super::super::run(&super::source_for(&scene, true));
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Ready);
    let completed = layout.completed.as_ref().expect("scene publishes layout");
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes track-sizing trace");
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    assert_eq!(
        usize::from(trace.used_rows.explicit_track_count)
            + usize::from(trace.used_rows.negative_implicit_track_count)
            + usize::from(trace.used_rows.positive_implicit_track_count),
        9
    );
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("Grid owner exists");
    assert_eq!(trace.node.get(), grid.node_id.get());
    let rows_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("final row-sizing pass exists");
    let final_stage = rows_pass
        .stages
        .iter()
        .find(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        })
        .expect("auto stretch stage exists");
    let expected_items = scene["items"].as_array().unwrap();
    let mut fit_items = 0;
    for (item_index, _) in expected_items.iter().enumerate().skip(1) {
        if (item_index - 1) % 6 != 0 {
            continue;
        }
        fit_items += 1;
        let color = super::super::two_fit_auto_placement::ITEM_COLORS[item_index];
        let style = styles
            .iter()
            .find(|style| {
                style.background_color.is_some_and(|actual| {
                    (actual.red, actual.green, actual.blue)
                        == (color.red(), color.green(), color.blue())
                })
            })
            .expect("fit item style exists");
        let minimums = rows_pass
            .contributions
            .iter()
            .filter(|contribution| {
                contribution.item == style.node_id
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            })
            .collect::<Vec<_>>();
        assert_eq!(minimums.len(), 1, "item {item_index} minimum contribution");
        let minimum = minimums[0];
        assert!(
            matches!(
                minimum.minimum_source,
                Some(MinimumSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: false,
                    spans_flexible_max_track: false,
                    content_based: true,
                    is_compressible_replaced: false,
                    ..
                })
            ),
            "item {item_index}: {minimum:?}"
        );
        let floor = expected_items[item_index]["lineCount"].as_u64().unwrap() as f32 * 10.0;
        assert!((minimum.value - floor).abs() <= 0.01);
        let internal_index = u16::try_from(item_index * 2 + 1).unwrap();
        let final_track = final_stage
            .tracks
            .iter()
            .find(|track| !track.is_gutter && track.internal_index == internal_index)
            .expect("fit-content row has final track state");
        assert!(final_track.base_size + 0.01 >= minimum.value);
        assert!(minimum.value > 0.0, "0% cap must not clamp content minimum");
    }
    assert_eq!(fit_items, 2, "fit position 1 repeats for rows 2 and 8");
}
