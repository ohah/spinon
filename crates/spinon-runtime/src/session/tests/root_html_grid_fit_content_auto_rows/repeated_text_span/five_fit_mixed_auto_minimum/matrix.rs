use super::*;
use std::collections::{BTreeMap, BTreeSet};

fn source_order_key(scene: &Value) -> String {
    let topology = scene["topology"].as_str().unwrap();
    let family = topology
        .strip_suffix("-forward")
        .or_else(|| topology.strip_suffix("-reverse"))
        .expect("topology records DOM order");
    format!(
        "{}|{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["alignContent"].as_str().unwrap(),
        scene["minimumMask"].as_u64().unwrap(),
        family
    )
}

fn alignment_key(scene: &Value) -> String {
    format!(
        "{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["topology"].as_str().unwrap(),
        scene["minimumMask"].as_u64().unwrap()
    )
}

fn mode_index(is_auto: bool) -> usize {
    if is_auto { 1 } else { 0 }
}

fn grid_end_overflow(frame: [f32; 4], grid_frame: [f32; 4]) -> bool {
    frame[1] + frame[3] > grid_frame[1] + grid_frame[3] + 0.01
}

fn run_matrix_chunk(start: usize, end: usize) {
    let batches = batches();
    assert!(start < end && end <= batches.len());
    assert_eq!(batches.len(), 120);

    let mut expected_floor_shortfalls = [0; 2];
    let mut actual_floor_shortfalls = [0; 2];
    let mut expected_grid_end_overflows = [0; 2];
    let mut actual_grid_end_overflows = [0; 2];
    for (batch_index, (track_list, scenes)) in
        batches.iter().enumerate().skip(start).take(end - start)
    {
        assert!(
            matches!(scenes.len(), 8 | 176),
            "{track_list}: sample group size"
        );
        let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: mixed profile initializes: {:?}; diagnostics={:?}",
            initial.error,
            initial.diagnostics
        );

        let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
        let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
        for record in scenes {
            let scene = &record["scene"];
            let label = format!("five-fit mixed sample {}: {scene}", record["sampleIndex"]);
            assert_eq!(
                encoded_minimum_mask(scene),
                scene["minimumMask"].as_u64().unwrap() as usize
            );
            configure_six_fit_mixed_minimum_scene(&session, record);
            let result = observe_current(&session, record, &label);
            let rows = row_values(record);
            assert_eq!(result.rows.len(), rows.len(), "{label}: used-row count");
            for (index, (actual, expected)) in result.rows.iter().zip(rows).enumerate() {
                assert!(
                    (actual - expected).abs() <= 0.01,
                    "{label}: row {index}: {actual} vs {expected}"
                );
            }

            let grid_frame = frame(&record["gridFrame"]);
            for item in record["items"].as_array().unwrap() {
                let id = item["id"].as_str().unwrap();
                if id == "empty" {
                    continue;
                }
                let is_auto = scene["autoItems"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|value| value.as_str() == Some(id));
                let mode = mode_index(is_auto);
                let expected_item = frame(&item["frame"]);
                let expected_payload = frame(&item["payloadFrame"]);
                let actual_item = result.item_frames[id];
                let actual_payload = result.payload_frames[id];
                let eligible = eligible_for_auto_minimum(record, id);
                let expected_shortfall = eligible && expected_payload[3] > expected_item[3] + 0.01;
                let actual_shortfall = eligible && actual_payload[3] > actual_item[3] + 0.01;
                expected_floor_shortfalls[mode] += expected_shortfall as usize;
                actual_floor_shortfalls[mode] += actual_shortfall as usize;

                let expected_overflow = grid_end_overflow(expected_payload, grid_frame);
                let actual_overflow = grid_end_overflow(actual_payload, grid_frame);
                assert_eq!(
                    actual_overflow, expected_overflow,
                    "{label}: {id} grid-end overflow"
                );
                expected_grid_end_overflows[mode] += expected_overflow as usize;
                actual_grid_end_overflows[mode] += actual_overflow as usize;
            }

            source_order_groups
                .entry(source_order_key(scene))
                .or_default()
                .push((
                    scene["topology"].as_str().unwrap().to_owned(),
                    result.clone(),
                ));
            alignment_groups
                .entry(alignment_key(scene))
                .or_default()
                .push((scene["alignContent"].as_str().unwrap().to_owned(), result));
        }

        let selected_masks = scenes
            .iter()
            .map(|record| {
                let scene = &record["scene"];
                let family = scene["topology"]
                    .as_str()
                    .unwrap()
                    .rsplit_once('-')
                    .unwrap()
                    .0;
                (family.to_owned(), encoded_minimum_mask(scene))
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(source_order_groups.len(), selected_masks.len() * 2);
        for (key, mut pair) in source_order_groups {
            pair.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(pair.len(), 2, "{key}: source-order pair");
            assert_ne!(pair[0].0, pair[1].0, "{key}: DOM order differs");
            assert_eq!(
                pair[0].1.rows.len(),
                pair[1].1.rows.len(),
                "{key}: row count"
            );
            for (left, right) in pair[0].1.rows.iter().zip(&pair[1].1.rows) {
                assert!(
                    (left - right).abs() <= 0.01,
                    "{key}: source-order row geometry"
                );
            }
            assert_frame_maps_equal(&pair[0].1.item_frames, &pair[1].1.item_frames, &key);
            assert_frame_maps_equal(
                &pair[0].1.payload_frames,
                &pair[1].1.payload_frames,
                &format!("{key}: payload"),
            );
        }

        assert_eq!(alignment_groups.len(), selected_masks.len() * 2);
        for (key, mut pair) in alignment_groups {
            pair.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(pair.len(), 2, "{key}: alignment pair");
            assert_eq!(pair[0].0, "start");
            assert_eq!(pair[1].0, "stretch");
            assert_eq!(
                pair[0].1.rows.len(),
                pair[1].1.rows.len(),
                "{key}: row count"
            );
            let track_list = key
                .split('|')
                .next()
                .expect("alignment key starts with tracks");
            let members = track_list.split_whitespace().collect::<Vec<_>>();
            let auto_tail_row = members
                .iter()
                .position(|member| *member == "auto")
                .map(|implicit_index| implicit_index + 1);
            let start_rows = &pair[0].1.rows;
            let stretch_rows = &pair[1].1.rows;
            let start_total = start_rows.iter().sum::<f32>();
            let stretch_total = stretch_rows.iter().sum::<f32>();
            if let Some(auto_tail_row) = auto_tail_row {
                assert!(
                    (stretch_total - start_total.max(220.0)).abs() <= 0.02,
                    "{key}: auto tail stretch fills only remaining Grid block size"
                );
                for (index, (start, stretch)) in start_rows.iter().zip(stretch_rows).enumerate() {
                    if index == auto_tail_row {
                        assert!(
                            stretch + 0.01 >= *start,
                            "{key}: stretch cannot shrink auto tail"
                        );
                    } else {
                        assert!(
                            (stretch - start).abs() <= 0.01,
                            "{key}: non-auto row {index} is fixed"
                        );
                    }
                }
            } else {
                for (start, stretch) in start_rows.iter().zip(stretch_rows) {
                    assert!(
                        (stretch - start).abs() <= 0.01,
                        "{key}: intrinsic tail rows are fixed"
                    );
                }
                assert_frame_maps_equal(&pair[0].1.item_frames, &pair[1].1.item_frames, &key);
                assert_frame_maps_equal(
                    &pair[0].1.payload_frames,
                    &pair[1].1.payload_frames,
                    &format!("{key}: payload"),
                );
            }
        }

        if (batch_index + 1) % 10 == 0 || batch_index + 1 == end {
            eprintln!("five-fit mixed track lists: {} / {end}", batch_index + 1);
        }
    }
    assert_eq!(actual_floor_shortfalls, expected_floor_shortfalls);
    assert_eq!(actual_grid_end_overflows, expected_grid_end_overflows);
    assert_eq!(
        actual_floor_shortfalls[1], 0,
        "automatic minimum preserves eligible payload floor"
    );
    eprintln!(
        "five-fit mixed chunk {start}..{end}: floor shortfalls zero/auto={:?}; Grid-end overflows zero/auto={:?}",
        actual_floor_shortfalls, actual_grid_end_overflows
    );
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_metadata_covers_balanced_3984_scene_sample() {
    let batches = batches();
    assert_eq!(records().len(), 3_984);
    assert_eq!(batches.len(), 120);
    let mut fit_positions = BTreeSet::new();
    let mut cap_orders = BTreeSet::new();
    let mut tails = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut masks_by_topology = BTreeMap::<String, BTreeSet<usize>>::new();
    let mut masks_by_context = BTreeMap::<(String, String, String), BTreeSet<usize>>::new();
    let mut variants_by_list_mask =
        BTreeMap::<(String, String, usize), BTreeSet<(String, String)>>::new();
    let mut zero_floor_shortfalls = 0;
    let mut auto_floor_shortfalls = 0;
    let mut zero_grid_overflows = 0;
    let mut auto_grid_overflows = 0;
    let mut representative_lists = BTreeSet::new();
    let mut representative_contexts = BTreeSet::new();
    let mut rotation_lists = 0;
    let mut scene_count = 0;
    let mut scene_indices = BTreeSet::new();
    let mut sample_indices = BTreeSet::new();
    let expected_variants = BTreeSet::from([
        ("start".to_owned(), "forward".to_owned()),
        ("start".to_owned(), "reverse".to_owned()),
        ("stretch".to_owned(), "forward".to_owned()),
        ("stretch".to_owned(), "reverse".to_owned()),
    ]);

    for (track_list, scenes) in batches {
        assert!(
            matches!(scenes.len(), 8 | 176),
            "{track_list}: sample batch size"
        );
        let is_representative = scenes.len() == 176;
        let first_scene = &scenes[0]["scene"];
        fit_positions.insert(first_scene["fitPositions"].to_string());
        tails.insert(first_scene["tail"].to_string());
        cap_orders.insert(first_scene["fitCaps"].to_string());
        if is_representative {
            representative_lists.insert(track_list.clone());
            representative_contexts.insert((
                first_scene["fitPositions"].to_string(),
                first_scene["tail"].to_string(),
            ));
        } else {
            rotation_lists += 1;
        }
        for record in scenes {
            assert!(sample_indices.insert(record["sampleIndex"].as_u64().unwrap()));
            assert!(scene_indices.insert(record["index"].as_u64().unwrap()));
            let scene = &record["scene"];
            let topology = scene["topology"].as_str().unwrap().to_owned();
            let (family, order) = topology
                .rsplit_once('-')
                .expect("topology has source order");
            assert!(matches!(order, "forward" | "reverse"));
            let ids = topology_item_ids(&topology);
            let mask = encoded_minimum_mask(scene);
            assert!(
                mask > 0 && mask < (1 << ids.len()) - 1,
                "sample contains only mixed masks"
            );
            masks_by_topology
                .entry(topology.clone())
                .or_default()
                .insert(mask);
            let position_key = scene["fitPositions"].to_string();
            let tail_key = scene["tail"].to_string();
            masks_by_context
                .entry((family.to_owned(), position_key, tail_key))
                .or_default()
                .insert(mask);
            alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
            topologies.insert(topology.clone());
            variants_by_list_mask
                .entry((track_list.clone(), family.to_owned(), mask))
                .or_default()
                .insert((
                    scene["alignContent"].as_str().unwrap().to_owned(),
                    order.to_owned(),
                ));

            for item in record["items"].as_array().unwrap() {
                let id = item["id"].as_str().unwrap();
                let expected_auto = id != "empty"
                    && scene["autoItems"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|value| value.as_str() == Some(id));
                assert_eq!(
                    item["minHeight"].as_str(),
                    Some(if expected_auto { "auto" } else { "0px" })
                );
                if id == "empty" {
                    assert_eq!(item["minHeight"], "0px");
                    continue;
                }
                assert!(ids.contains(&id));
                let eligible = eligible_for_auto_minimum(record, id);
                let item_frame = frame(&item["frame"]);
                let payload_frame = frame(&item["payloadFrame"]);
                let floor_shortfall = eligible && payload_frame[3] > item_frame[3] + 0.01;
                let overflow = grid_end_overflow(payload_frame, frame(&record["gridFrame"]));
                if expected_auto {
                    auto_floor_shortfalls += floor_shortfall as usize;
                    auto_grid_overflows += overflow as usize;
                } else {
                    zero_floor_shortfalls += floor_shortfall as usize;
                    zero_grid_overflows += overflow as usize;
                }
            }
            scene_count += 1;
        }
    }

    assert_eq!(scene_count, 3_984);
    assert_eq!(scene_indices.len(), 3_984);
    assert_eq!(sample_indices.len(), 3_984);
    assert_eq!(sample_indices.first(), Some(&0));
    assert_eq!(sample_indices.last(), Some(&3_983));
    assert_eq!(fit_positions.len(), 6);
    assert_eq!(cap_orders.len(), 120);
    assert_eq!(tails.len(), 3);
    assert_eq!(
        alignments,
        BTreeSet::from(["start".to_owned(), "stretch".to_owned()])
    );
    assert_eq!(topologies.len(), 4);
    assert_eq!(representative_lists.len(), 18);
    assert_eq!(representative_contexts.len(), 18);
    assert_eq!(rotation_lists, 102);
    assert_eq!(auto_floor_shortfalls, 0);
    assert_eq!(zero_floor_shortfalls, 1_388);
    assert_eq!(auto_grid_overflows, 0);
    assert_eq!(zero_grid_overflows, 4);
    for (topology, masks) in masks_by_topology {
        let mask_count = if topology.starts_with("span2Six-") {
            32
        } else {
            16
        };
        assert_eq!(
            masks,
            (1..mask_count - 1).collect(),
            "{topology}: partial masks"
        );
    }
    for (family, mask_count) in [("span2Six", 32), ("span2Span3Six", 16)] {
        let expected_masks = (1..mask_count - 1).collect::<BTreeSet<_>>();
        for (position, tail) in &representative_contexts {
            assert_eq!(
                masks_by_context[&(family.to_owned(), position.clone(), tail.clone())],
                expected_masks,
                "{family}/{position}/{tail}: full partial-mask set"
            );
        }
    }
    for (key, variants) in variants_by_list_mask {
        assert_eq!(
            variants, expected_variants,
            "selected mask has all geometry variants"
        );
        let _ = key;
    }
}

macro_rules! matrix_chunk {
    ($name:ident, $start:expr, $end:expr) => {
        #[test]
        fn $name() {
            run_matrix_chunk($start, $end);
        }
    };
}

matrix_chunk!(
    parsed_root_html_five_fit_mixed_minimum_matrix_chunk_01_matches_chrome,
    0,
    40
);
matrix_chunk!(
    parsed_root_html_five_fit_mixed_minimum_matrix_chunk_02_matches_chrome,
    40,
    80
);
matrix_chunk!(
    parsed_root_html_five_fit_mixed_minimum_matrix_chunk_03_matches_chrome,
    80,
    120
);
