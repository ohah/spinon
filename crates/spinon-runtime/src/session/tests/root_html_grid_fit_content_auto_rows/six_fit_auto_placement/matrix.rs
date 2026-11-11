use super::*;
use serde_json::Value;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

fn frame(value: &Value) -> [f32; 4] {
    super::super::two_fit_auto_placement::frame(value)
}

fn row_sizes(value: &Value) -> Vec<f32> {
    value
        .as_array()
        .expect("Chrome record contains used row sizes")
        .iter()
        .map(|row| row.as_f64().expect("row size is numeric") as f32)
        .collect()
}

fn cap_permutations() -> BTreeSet<String> {
    fn visit(values: &mut [&str], index: usize, permutations: &mut BTreeSet<String>) {
        if index == values.len() {
            permutations.insert(serde_json::to_string(values).unwrap());
            return;
        }
        for next in index..values.len() {
            values.swap(index, next);
            visit(values, index + 1, permutations);
            values.swap(index, next);
        }
    }
    let mut values = ["0px", "10px", "20%", "30px", "40px", "50px"];
    let mut permutations = BTreeSet::new();
    visit(&mut values, 0, &mut permutations);
    permutations
}

#[test]
fn parsed_root_html_six_fit_auto_placement_matches_all_5760_chrome_scenes() {
    let scenes = scenes();
    assert_eq!(scenes.len(), 5_760);
    let track_lists = scenes
        .iter()
        .map(|scene| scene["scene"]["trackList"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let cap_assignments = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitCaps"]).unwrap())
        .collect::<BTreeSet<_>>();
    let alignments = scenes
        .iter()
        .map(|scene| scene["scene"]["alignContent"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let dom_orders = scenes
        .iter()
        .map(|scene| scene["scene"]["domOrder"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let minimum_modes = scenes
        .iter()
        .map(|scene| scene["scene"]["minHeight"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(track_lists.len(), 720);
    assert_eq!(cap_assignments, cap_permutations());
    assert_eq!(alignments, BTreeSet::from(["start", "stretch"]));
    assert_eq!(dom_orders, BTreeSet::from(["forward", "reverse"]));
    assert_eq!(minimum_modes, BTreeSet::from(["0px", "auto"]));

    let mut baseline_keys = BTreeSet::new();
    let mut candidate_keys = BTreeSet::new();
    let mut row_pairs = BTreeMap::<String, [Option<Vec<f32>>; 2]>::new();
    let mut alignment_pairs = BTreeMap::<String, [Option<Vec<f32>>; 2]>::new();
    let mut baseline_scenes = 0;
    let mut candidate_scenes = 0;
    let mut baseline_under_floor = 0;
    let mut candidate_floor_violations = 0;
    let mut candidate_item_occurrences = 0;
    let mut baseline_visible_overflow = 0;
    let mut candidate_visible_overflow = 0;
    let mut item_grid_end_overflow = 0;
    let mut payload_grid_end_overflow = 0;
    let mut max_payload_end = BTreeMap::<&str, f32>::new();

    for (scene_index, expected_scene) in scenes.iter().enumerate() {
        let scene = &expected_scene["scene"];
        let automatic_minimum = is_automatic_minimum(expected_scene);
        let track_list = scene["trackList"].as_str().unwrap();
        let alignment = scene["alignContent"].as_str().unwrap();
        let dom_order = scene["domOrder"].as_str().unwrap();
        let minimum = scene["minHeight"].as_str().unwrap();
        let key = format!("{track_list}|{alignment}|{dom_order}");
        let row_key = format!("{track_list}|{dom_order}|{alignment}");
        let row_slot = usize::from(automatic_minimum);
        if automatic_minimum {
            candidate_scenes += 1;
            assert!(candidate_keys.insert(key));
        } else {
            baseline_scenes += 1;
            assert!(baseline_keys.insert(key));
        }
        let alignment_key = format!("{track_list}|{dom_order}|{minimum}");
        let alignment_slot = usize::from(alignment == "stretch");

        let (session, layout) = run(&source_for(expected_scene));
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index}: layout={:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported scene publishes a complete layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("supported scene publishes a render snapshot");
        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("same revision publishes computed styles");
        assert_eq!(completed.key, cascade.key, "scene {scene_index}: revision");
        let styles = &cascade.roots[0].styles.elements;
        let grid_style = styles
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("fixture has one Grid owner");
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(computed_tracks(expected_scene)),
            "scene {scene_index}: typed six-fit cap order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(String::as_str),
            Some(expected_scene["computedAutoRows"].as_str().unwrap()),
            "scene {scene_index}: computed cap serialization"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            Some(expected_scene["computedAlignContent"].as_str().unwrap()),
            "scene {scene_index}: content alignment"
        );
        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame should be published");
        let expected_grid_size = expected_scene["gridFrame"]
            .as_array()
            .expect("Chrome Grid frame has width and height");
        assert_eq!(expected_grid_size.len(), 2);
        super::super::assert_frame(
            super::super::two_fit_auto_placement::runtime_frame(grid_frame),
            [
                0.0,
                0.0,
                expected_grid_size[0].as_f64().unwrap() as f32,
                expected_grid_size[1].as_f64().unwrap() as f32,
            ],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["items"]
            .as_array()
            .expect("Chrome records item frames");
        let expected_rows = row_sizes(&expected_scene["rows"]);
        assert_eq!(expected_items.len(), 9);
        assert_eq!(expected_rows.len(), 9);
        let mut actual_item_frames = BTreeMap::new();
        let mut actual_row_sizes = Vec::with_capacity(expected_rows.len());
        for (item_index, expected_item) in expected_items.iter().enumerate() {
            let id = if item_index == 0 {
                0
            } else {
                item_id(expected_item)
            };
            let color = super::super::two_fit_auto_placement::ITEM_COLORS[id];
            let item_style = styles
                .iter()
                .filter(|style| {
                    style.background_color.is_some_and(|actual| {
                        (actual.red, actual.green, actual.blue)
                            == (color.red(), color.green(), color.blue())
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(item_style.len(), 1, "scene {scene_index} item {id}: style");
            assert_eq!(
                item_style[0]
                    .properties
                    .get("min-height")
                    .map(String::as_str),
                expected_item["minHeight"].as_str(),
                "scene {scene_index} item {id}: computed minimum"
            );
            let actual_item = super::super::two_fit_auto_placement::frame_for_background(
                completed,
                styles,
                color,
                &format!("scene {scene_index} item {id}"),
            );
            super::super::assert_frame(
                actual_item,
                frame(&expected_item["frame"]),
                &format!("scene {scene_index} item {id}"),
            );
            assert!((actual_item[1] + actual_item[3] - 220.0).is_finite());
            if actual_item[1] + actual_item[3] > 220.01 {
                item_grid_end_overflow += 1;
            }
            super::super::assert_frame(
                [actual_item[3], 0.0, 0.0, 0.0],
                [expected_rows[item_index], 0.0, 0.0, 0.0],
                &format!("scene {scene_index} used row {item_index}"),
            );
            actual_row_sizes.push(actual_item[3]);
            actual_item_frames.insert(id, actual_item);
        }

        let mut texts = render.texts().iter().collect::<Vec<_>>();
        texts.sort_unstable_by_key(|text| text.node_id().get());
        assert_eq!(
            texts.len(),
            8,
            "scene {scene_index}: eight payload text nodes"
        );
        for (text, expected_item) in texts.into_iter().zip(expected_items.iter().skip(1)) {
            let id = item_id(expected_item);
            let expected_payload = frame(&expected_item["payload"]);
            let actual_payload = super::super::two_fit_auto_placement::frame_for_payload(
                completed,
                text.node_id().get(),
                &format!("scene {scene_index} payload {id}"),
            );
            super::super::assert_frame(
                actual_payload,
                expected_payload,
                &format!("scene {scene_index} payload {id}"),
            );
            let item_frame = actual_item_frames[&id];
            if minimum == "auto" {
                candidate_item_occurrences += 1;
                if item_frame[3] + 0.01 < actual_payload[3] {
                    candidate_floor_violations += 1;
                }
            } else if item_frame[3] + 0.01 < actual_payload[3] {
                baseline_under_floor += 1;
            }
            if actual_payload[3] > item_frame[3] + 0.01 {
                if minimum == "auto" {
                    candidate_visible_overflow += 1;
                } else {
                    baseline_visible_overflow += 1;
                }
            }
            let payload_end = actual_payload[1] + actual_payload[3];
            if payload_end > 220.01 {
                payload_grid_end_overflow += 1;
            }
            max_payload_end
                .entry(minimum)
                .and_modify(|end| *end = end.max(payload_end))
                .or_insert(payload_end);
            let text_value = super::super::two_fit_auto_placement::PAYLOAD_TEXTS[id - 1];
            assert_eq!(
                text.glyphs().len(),
                text_value.bytes().filter(u8::is_ascii_uppercase).count(),
                "scene {scene_index} payload {id}: glyph count"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                expected_item["lineCount"].as_u64().unwrap() as usize,
                "scene {scene_index} payload {id}: line count"
            );
        }
        assert_eq!(actual_row_sizes.len(), 9);
        assert!(
            row_pairs.entry(row_key).or_default()[row_slot]
                .replace(actual_row_sizes.clone())
                .is_none()
        );
        assert!(
            alignment_pairs.entry(alignment_key).or_default()[alignment_slot]
                .replace(actual_row_sizes)
                .is_none()
        );
    }

    assert_eq!(baseline_scenes, 2_880);
    assert_eq!(candidate_scenes, 2_880);
    assert_eq!(baseline_keys.len(), 2_880);
    assert_eq!(candidate_keys.len(), 2_880);
    assert_eq!(
        baseline_keys, candidate_keys,
        "baseline and candidate keys pair"
    );
    assert_eq!(candidate_item_occurrences, 23_040);
    assert_eq!(baseline_under_floor, 7_200);
    assert_eq!(candidate_floor_violations, 0);
    assert_eq!(baseline_visible_overflow, 7_200);
    assert_eq!(candidate_visible_overflow, 0);
    assert_eq!(item_grid_end_overflow, 0);
    assert_eq!(payload_grid_end_overflow, 0);
    assert!((max_payload_end["0px"] - 150.0).abs() <= 0.01);
    assert!((max_payload_end["auto"] - 180.0).abs() <= 0.01);

    let changed_rows = row_pairs
        .values()
        .filter(|pair| match pair {
            [Some(baseline), Some(candidate)] => baseline != candidate,
            _ => panic!("each Chrome key has baseline and candidate rows"),
        })
        .count();
    assert_eq!(changed_rows, 2_880);
    let alignment_differences = alignment_pairs
        .values()
        .filter(|pair| match pair {
            [Some(start), Some(stretch)] => start != stretch,
            _ => panic!("each cap/order/minimum key has start and stretch rows"),
        })
        .count();
    assert_eq!(alignment_differences, 0);
    eprintln!(
        "six-fit RuntimeSession matrix: 5760 scenes; 720 cap orders; baseline under-floor/overflow={baseline_under_floor}/{baseline_visible_overflow}; candidate floor/overflow={candidate_floor_violations}/{candidate_visible_overflow}; changed rows={changed_rows}; alignment deltas={alignment_differences}"
    );
}
