use super::*;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn parsed_root_html_two_fit_auto_placement_matches_all_2880_chrome_scenes() {
    let scenes = super::scenes();
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
        let source = source_for(expected_scene);
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
    assert_eq!(overflow_count, 288);
    assert_eq!(fit_row_occurrences, 7_680);
    assert_eq!(auto_stretch_pairs.len(), 1_440);
    assert!(auto_stretch_pairs.values().all(|count| *count == 1));
}
