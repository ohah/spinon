use super::*;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

fn all_cap_assignments() -> BTreeSet<String> {
    fn permute(values: &mut [&str], index: usize, result: &mut BTreeSet<String>) {
        if index == values.len() {
            result.insert(serde_json::to_string(values).unwrap());
            return;
        }
        for next in index..values.len() {
            values.swap(index, next);
            permute(values, index + 1, result);
            values.swap(index, next);
        }
    }

    let mut values = ["0px", "10px", "20%", "30px", "40px"];
    let mut result = BTreeSet::new();
    permute(&mut values, 0, &mut result);
    result
}

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

#[test]
fn parsed_root_html_five_fit_auto_placement_matches_all_17280_chrome_scenes() {
    let scenes = scenes();
    assert_eq!(scenes.len(), 17_280);
    let track_lists = scenes
        .iter()
        .map(|scene| scene["scene"]["trackList"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let fit_positions = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitPositions"]).unwrap())
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
    let tail_positions = scenes
        .iter()
        .map(|scene| {
            let members = scene["scene"]["trackMembers"].as_array().unwrap();
            let position = members
                .iter()
                .position(|member| !member.as_str().unwrap().starts_with("fit-content("))
                .expect("five-fit list has exactly one tail")
                + 1;
            serde_json::to_string(&serde_json::json!([scene["scene"]["tail"][0], position]))
                .unwrap()
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(track_lists.len(), 2_160);
    let mut expected_fit_positions = BTreeSet::new();
    for first in 1..=2 {
        for second in first + 1..=3 {
            for third in second + 1..=4 {
                for fourth in third + 1..=5 {
                    for fifth in fourth + 1..=6 {
                        expected_fit_positions.insert(
                            serde_json::to_string(&[first, second, third, fourth, fifth]).unwrap(),
                        );
                    }
                }
            }
        }
    }
    assert_eq!(fit_positions, expected_fit_positions);
    assert_eq!(cap_assignments, all_cap_assignments());
    let expected_tail_positions = ["min-content", "max-content", "auto"]
        .into_iter()
        .flat_map(|tail| {
            (1..=6).map(move |position| {
                serde_json::to_string(&serde_json::json!([tail, position])).unwrap()
            })
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(tail_positions, expected_tail_positions);
    assert_eq!(alignments, BTreeSet::from(["start", "stretch"]));
    assert_eq!(dom_orders, BTreeSet::from(["forward", "reverse"]));
    assert_eq!(minimum_modes, BTreeSet::from(["0px", "auto"]));

    let mut baseline_keys = BTreeSet::new();
    let mut candidate_keys = BTreeSet::new();
    let mut baseline_scene_count = 0;
    let mut candidate_scene_count = 0;
    let mut baseline_fit_occurrences_below_payload_floor = 0;
    let mut candidate_fit_occurrences = 0;
    let mut candidate_fit_floor_violations = 0;
    let mut tail_payload_floor_violations = 0;
    let mut baseline_item_overflow_occurrences = BTreeMap::<String, usize>::new();
    let mut candidate_item_overflow_occurrences = BTreeMap::<String, usize>::new();
    let mut item_frame_grid_end_overflows = 0;
    let mut payload_grid_end_overflows = BTreeMap::<String, usize>::new();
    let mut maximum_payload_end = BTreeMap::<&str, f32>::new();
    let mut row_pairs = BTreeMap::<(String, String, String, String), [Option<Vec<f32>>; 2]>::new();

    for (scene_index, expected_scene) in scenes.iter().enumerate() {
        let auto_minimum = is_automatic_minimum(expected_scene);
        let scene = &expected_scene["scene"];
        let track_list = scene["trackList"].as_str().unwrap();
        let alignment = scene["alignContent"].as_str().unwrap();
        let dom_order = scene["domOrder"].as_str().unwrap();
        let minimum = scene["minHeight"].as_str().unwrap();
        let tail = scene["tail"][0].as_str().unwrap();
        let key = format!("{track_list}|{alignment}|{dom_order}");
        if auto_minimum {
            candidate_scene_count += 1;
            assert!(candidate_keys.insert(key));
        } else {
            baseline_scene_count += 1;
            assert!(baseline_keys.insert(key));
        }

        let row_key = (
            track_list.to_owned(),
            dom_order.to_owned(),
            tail.to_owned(),
            minimum.to_owned(),
        );
        let alignment_slot = usize::from(alignment == "stretch");
        assert!(
            row_pairs.entry(row_key).or_default()[alignment_slot]
                .replace(row_sizes(&expected_scene["rows"]))
                .is_none()
        );

        let (session, layout) = super::super::run(&source_for(expected_scene));
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
        super::super::assert_frame(
            super::super::two_fit_auto_placement::runtime_frame(grid_frame),
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["items"]
            .as_array()
            .expect("Chrome records item frames");
        assert_eq!(expected_items.len(), 9);
        let expected_rows = expected_scene["rows"]
            .as_array()
            .expect("Chrome records used rows");
        assert_eq!(expected_rows.len(), 9);
        let track_members = scene["trackMembers"]
            .as_array()
            .expect("Chrome records the six authored track members");
        assert_eq!(track_members.len(), 6);
        let mut actual_fit_positions = Vec::new();
        let mut actual_tail = Vec::new();
        let mut actual_caps = Vec::new();
        for (position, member) in track_members.iter().enumerate() {
            let member = member.as_str().unwrap();
            if member.starts_with("fit-content(") {
                actual_fit_positions.push(position + 1);
                actual_caps.push(
                    member
                        .strip_prefix("fit-content(")
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap(),
                );
            } else {
                actual_tail.push(member);
            }
        }
        assert_eq!(
            scene["fitPositions"],
            serde_json::json!(actual_fit_positions),
            "scene {scene_index}: fit position metadata matches the authored list"
        );
        assert_eq!(
            scene["tail"],
            serde_json::json!(actual_tail),
            "scene {scene_index}: one tail value matches the authored list"
        );
        assert_eq!(
            scene["fitCaps"],
            serde_json::json!(actual_caps),
            "scene {scene_index}: cap assignment matches the authored list"
        );

        let mut item_overflow_occurrences = 0;
        let mut actual_item_frames = BTreeMap::new();
        for (item_index, expected_item) in expected_items.iter().enumerate() {
            let id = if item_index == 0 {
                0
            } else {
                item_id(expected_item)
            };
            let color = super::super::two_fit_auto_placement::ITEM_COLORS[id];
            let item_styles = styles
                .iter()
                .filter(|style| {
                    style.background_color.is_some_and(|actual| {
                        (actual.red, actual.green, actual.blue)
                            == (color.red(), color.green(), color.blue())
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(item_styles.len(), 1, "scene {scene_index} item {id}: style");
            assert_eq!(
                item_styles[0]
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
            if actual_item[1] + actual_item[3] > 220.01 {
                item_frame_grid_end_overflows += 1;
            }
            super::super::assert_frame(
                [actual_item[3], 0.0, 0.0, 0.0],
                [
                    expected_rows[item_index].as_f64().unwrap() as f32,
                    0.0,
                    0.0,
                    0.0,
                ],
                &format!("scene {scene_index} row {item_index}"),
            );
            actual_item_frames.insert(id, actual_item);
            if id == 0 {
                continue;
            }

            let expected_payload = frame(&expected_item["payload"]);
            let track_position = (item_index - 1) % track_members.len();
            let track_member = track_members[track_position].as_str().unwrap();
            let fit_track = track_member.starts_with("fit-content(");
            let item_end = actual_item[1] + actual_item[3];
            let payload_end = expected_payload[1] + expected_payload[3];
            maximum_payload_end
                .entry(minimum)
                .and_modify(|end| *end = end.max(payload_end))
                .or_insert(payload_end);

            if fit_track {
                if auto_minimum {
                    candidate_fit_occurrences += 1;
                    if actual_item[3] + 0.01 < expected_payload[3] {
                        candidate_fit_floor_violations += 1;
                    }
                } else if frame(&expected_item["frame"])[3] + 0.01 < expected_payload[3] {
                    baseline_fit_occurrences_below_payload_floor += 1;
                }
            } else if actual_item[3] + 0.01 < expected_payload[3] {
                tail_payload_floor_violations += 1;
            }

            let item_overflows_payload = expected_payload[3] > actual_item[3] + 0.01;
            if item_overflows_payload {
                item_overflow_occurrences += 1;
            }
            if payload_end > 220.01 {
                *payload_grid_end_overflows
                    .entry(format!("{tail}/{minimum}"))
                    .or_default() += 1;
            }
            assert!(
                !auto_minimum || item_end <= 220.01,
                "scene {scene_index} item {id}: candidate item frame stays within Grid end"
            );
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
            assert!(actual_payload[1] + actual_payload[3] <= 240.01);
            let actual_item = actual_item_frames[&id];
            assert_eq!(
                expected_payload[3] > actual_item[3] + 0.01,
                frame(&expected_item["payload"])[3] > frame(&expected_item["frame"])[3] + 0.01,
                "scene {scene_index} item {id}: visible payload overflow"
            );
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

        let tail_mode = format!("{tail}/{minimum}");
        let overflow_counter = if auto_minimum {
            &mut candidate_item_overflow_occurrences
        } else {
            &mut baseline_item_overflow_occurrences
        };
        *overflow_counter.entry(tail_mode).or_default() += item_overflow_occurrences;
    }

    assert_eq!(baseline_scene_count, 8_640);
    assert_eq!(candidate_scene_count, 8_640);
    assert_eq!(baseline_keys.len(), 8_640);
    assert_eq!(candidate_keys.len(), 8_640);
    assert_eq!(baseline_keys, candidate_keys, "all Chrome keys are paired");
    assert_eq!(candidate_fit_occurrences, 57_600);
    assert_eq!(baseline_fit_occurrences_below_payload_floor, 21_600);
    assert_eq!(candidate_fit_floor_violations, 0);
    assert_eq!(tail_payload_floor_violations, 0);
    assert_eq!(item_frame_grid_end_overflows, 0);
    assert_eq!(
        baseline_item_overflow_occurrences,
        BTreeMap::from([
            ("auto/0px".to_owned(), 7_200),
            ("max-content/0px".to_owned(), 7_200),
            ("min-content/0px".to_owned(), 7_200),
        ])
    );
    assert_eq!(
        candidate_item_overflow_occurrences,
        BTreeMap::from([
            ("auto/auto".to_owned(), 0),
            ("max-content/auto".to_owned(), 0),
            ("min-content/auto".to_owned(), 0),
        ])
    );
    assert_eq!(
        payload_grid_end_overflows,
        BTreeMap::from([("auto/0px".to_owned(), 600)])
    );
    assert!((maximum_payload_end["0px"] - 240.0).abs() <= 0.01);
    assert!((maximum_payload_end["auto"] - 220.0).abs() <= 0.01);

    let mut stretch_differences = BTreeMap::<String, usize>::new();
    for ((_, _, tail, minimum), pair) in row_pairs {
        let [Some(start), Some(stretch)] = pair else {
            panic!("every matrix key has start and stretch rows");
        };
        if start != stretch {
            *stretch_differences
                .entry(format!("{tail}/{minimum}"))
                .or_default() += 1;
        }
    }
    assert_eq!(
        stretch_differences,
        BTreeMap::from([
            ("auto/0px".to_owned(), 1_440),
            ("auto/auto".to_owned(), 1_440),
        ])
    );
}
