use super::super::three_fit_auto_minimum::{
    SceneResult, configure_six_fit_mixed_minimum_scene, eligible_for_auto_minimum, observe_current,
    row_values, source_for_six_fit_mixed_minimum_track_list,
};
use super::{
    alignment_key, batches, encoded_minimum_mask, grid_end_overflow, records, source_order_key,
    topology_item_ids,
};
use std::collections::{BTreeMap, BTreeSet};

fn assert_rows_equal(actual: &[f32], expected: &[f32], label: &str) {
    assert_eq!(actual.len(), expected.len(), "{label}: used row count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.01,
            "{label}: row {index} differs: {actual:?} vs {expected:?}"
        );
    }
}

fn assert_frame_maps_equal(
    actual: &BTreeMap<String, [f32; 4]>,
    expected: &BTreeMap<String, [f32; 4]>,
    label: &str,
) {
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>(),
        "{label}: semantic frame owners"
    );
    for (id, frame) in actual {
        super::super::assert_frame(*frame, expected[id], &format!("{label}: {id}"));
    }
}

fn mode_index(auto: bool) -> usize {
    if auto { 1 } else { 0 }
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_stretched_auto_tail_rounding_matches_chrome() {
    let record = records()
        .iter()
        .find(|record| record["sampleIndex"] == 556)
        .expect("sample includes the mixed stretch rounding boundary");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) =
        super::super::super::run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "four-fit mixed stretched auto-tail rounding boundary",
    );
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_earlier_span_residual_matches_chrome() {
    let record = records()
        .iter()
        .find(|record| record["sampleIndex"] == 833)
        .expect("sample includes an automatic minimum on an earlier two-row span");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) =
        super::super::super::run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "four-fit mixed earlier-span residual boundary",
    );
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_later_span_residual_matches_chrome() {
    let record = records()
        .iter()
        .find(|record| record["sampleIndex"] == 2956)
        .expect("sample includes a later three-row span rounding boundary");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) =
        super::super::super::run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "four-fit mixed later-span residual boundary",
    );
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_metadata_covers_balanced_3840_scene_sample() {
    let all_records = records();
    let batches = batches();
    assert_eq!(all_records.len(), 3_840);
    assert_eq!(batches.len(), 20);

    let mut sample_indices = BTreeSet::new();
    let mut original_indices = BTreeSet::new();
    let mut fit_positions = BTreeSet::new();
    let mut cap_orders = BTreeSet::new();
    let mut tail_orders = BTreeSet::new();
    let mut masks_by_topology = BTreeMap::<String, BTreeSet<usize>>::new();
    let mut sample_track_list_order = Vec::new();
    let mut seen_track_lists = BTreeSet::new();

    for (expected_sample_index, record) in all_records.iter().enumerate() {
        let sample_index = record["sampleIndex"].as_u64().unwrap();
        let original_index = record["index"].as_u64().unwrap();
        assert_eq!(sample_index, expected_sample_index as u64);
        assert!(sample_indices.insert(sample_index));
        assert!(original_indices.insert(original_index));
        if expected_sample_index > 0 {
            assert!(
                record["index"].as_u64().unwrap()
                    > all_records[expected_sample_index - 1]["index"]
                        .as_u64()
                        .unwrap()
            );
        }
        let track_list = record["scene"]["trackList"].as_str().unwrap();
        if seen_track_lists.insert(track_list.to_owned()) {
            sample_track_list_order.push(track_list.to_owned());
        }
    }

    for (track_list, scenes) in batches {
        assert_eq!(scenes.len(), 192, "{track_list}: complete scene batch");
        let first_index = scenes[0]["index"].as_u64().unwrap();
        for (offset, record) in scenes.iter().enumerate() {
            let original = record["index"].as_u64().unwrap();
            assert_eq!(
                original,
                first_index + offset as u64,
                "{track_list}: source ordering"
            );
            let scene = &record["scene"];
            assert_eq!(scene["trackList"].as_str(), Some(track_list.as_str()));
            fit_positions.insert(scene["fitPositions"].to_string());
            cap_orders.insert(scene["fitCaps"].to_string());
            tail_orders.insert(scene["tail"].to_string());
            let topology = scene["topology"].as_str().unwrap().to_owned();
            masks_by_topology
                .entry(topology)
                .or_default()
                .insert(encoded_minimum_mask(scene));
            assert!(!topology_item_ids(scene["topology"].as_str().unwrap()).is_empty());
        }
    }

    assert_eq!(sample_indices, (0..3_840).collect());
    assert_eq!(original_indices.len(), 3_840);
    assert_eq!(sample_track_list_order.len(), 20);
    assert_eq!(fit_positions.len(), 15);
    assert_eq!(cap_orders.len(), 20);
    assert_eq!(tail_orders.len(), 2);
    for (topology, expected_count) in [
        ("span2Six-forward", 32),
        ("span2Six-reverse", 32),
        ("span2Span3Six-forward", 16),
        ("span2Span3Six-reverse", 16),
    ] {
        assert_eq!(
            masks_by_topology[topology],
            (0..expected_count).collect(),
            "{topology}: all masks are represented"
        );
    }
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_matrix_chunk_matches_chrome() {
    let batches = batches();
    assert_eq!(batches.len(), 20);
    let mut expected_floor_shortfalls = [0; 2];
    let mut actual_floor_shortfalls = [0; 2];
    let mut expected_grid_end_overflows = [0; 2];
    let mut actual_grid_end_overflows = [0; 2];
    let mut scene_count = 0;

    for (batch_index, (track_list, scenes)) in batches.iter().enumerate() {
        let (session, initial) =
            super::super::super::run(&source_for_six_fit_mixed_minimum_track_list(track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: four-fit mixed source initializes: {:?}; diagnostics={:?}",
            initial.error,
            initial.diagnostics
        );
        let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
        let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();

        for record in scenes {
            let scene = &record["scene"];
            let label = format!("four-fit mixed scene {}: {scene}", record["sampleIndex"]);
            assert_eq!(
                encoded_minimum_mask(scene),
                scene["minimumMask"].as_u64().unwrap() as usize
            );
            configure_six_fit_mixed_minimum_scene(&session, record);
            let result = observe_current(&session, record, &label);
            assert_rows_equal(&result.rows, &row_values(record), &label);

            let grid_frame = super::super::frame(&record["gridFrame"]);
            for item in record["items"].as_array().unwrap() {
                let id = item["id"].as_str().unwrap();
                if id == "empty" {
                    continue;
                }
                let auto = scene["autoItems"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|value| value.as_str() == Some(id));
                let mode = mode_index(auto);
                let expected_item = super::super::frame(&item["frame"]);
                let expected_payload = super::super::frame(&item["payloadFrame"]);
                let eligible = eligible_for_auto_minimum(record, id);
                let expected_floor = eligible && expected_payload[3] > expected_item[3] + 0.01;
                let actual_floor =
                    eligible && result.payload_frames[id][3] > result.item_frames[id][3] + 0.01;
                assert_eq!(actual_floor, expected_floor, "{label}: {id} payload floor");
                expected_floor_shortfalls[mode] += expected_floor as usize;
                actual_floor_shortfalls[mode] += actual_floor as usize;

                let expected_overflow = grid_end_overflow(expected_payload, grid_frame);
                let actual_overflow = grid_end_overflow(result.payload_frames[id], grid_frame);
                assert_eq!(actual_overflow, expected_overflow, "{label}: {id} Grid end");
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
            scene_count += 1;
        }

        assert_eq!(
            source_order_groups.len(),
            96,
            "{track_list}: paired DOM orders"
        );
        for (key, mut pair) in source_order_groups {
            pair.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(pair.len(), 2, "{key}: forward/reverse pair");
            assert_ne!(pair[0].0, pair[1].0);
            assert_rows_equal(&pair[0].1.rows, &pair[1].1.rows, &key);
            assert_frame_maps_equal(&pair[0].1.item_frames, &pair[1].1.item_frames, &key);
            assert_frame_maps_equal(
                &pair[0].1.payload_frames,
                &pair[1].1.payload_frames,
                &format!("{key}: payload"),
            );
        }

        assert_eq!(
            alignment_groups.len(),
            96,
            "{track_list}: paired alignments"
        );
        for (key, mut pair) in alignment_groups {
            pair.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(pair.len(), 2, "{key}: start/stretch pair");
            assert_eq!(pair[0].0, "start");
            assert_eq!(pair[1].0, "stretch");
            let auto_row = key
                .split('|')
                .next()
                .unwrap()
                .split_whitespace()
                .position(|member| member == "auto")
                .expect("one auto tail is present")
                + 1;
            for (row_index, (start, stretch)) in
                pair[0].1.rows.iter().zip(&pair[1].1.rows).enumerate()
            {
                if row_index == auto_row {
                    assert!(stretch + 0.01 >= *start, "{key}: auto row may grow");
                } else {
                    assert!(
                        (stretch - start).abs() <= 0.01,
                        "{key}: other tracks stay fixed"
                    );
                }
            }
        }

        eprintln!("four-fit mixed track lists: {} / 20", batch_index + 1);
    }

    assert_eq!(scene_count, 3_840);
    assert_eq!(actual_floor_shortfalls, expected_floor_shortfalls);
    assert_eq!(actual_grid_end_overflows, expected_grid_end_overflows);
    assert_eq!(
        actual_floor_shortfalls[1], 0,
        "auto preserves eligible payload floor"
    );
    eprintln!(
        "four-fit mixed 3,840 scenes: floor shortfalls zero/auto={actual_floor_shortfalls:?}; Grid-end overflows zero/auto={actual_grid_end_overflows:?}"
    );
}
