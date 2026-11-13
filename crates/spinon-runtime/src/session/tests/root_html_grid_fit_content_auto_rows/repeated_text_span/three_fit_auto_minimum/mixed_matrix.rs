use super::mixed_dynamic::{
    configure_three_fit_mixed_minimum_scene, source_for_three_fit_mixed_minimum_track_list,
};
use super::*;
use flate2::read::GzDecoder;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::sync::OnceLock;

const OBSERVATIONS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/runtime-matrix-6912.ndjson.gz"
));

pub(super) fn records() -> &'static [Value] {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let mut decoder = GzDecoder::new(OBSERVATIONS);
        let mut contents = String::new();
        decoder
            .read_to_string(&mut contents)
            .expect("three-fit mixed RuntimeSession sample decompresses as UTF-8");
        let mut parsed = contents.lines().map(|line| {
            serde_json::from_str::<Value>(line)
                .expect("three-fit mixed RuntimeSession sample is JSON")
        });
        let metadata = parsed.next().expect("runtime sample metadata is present");
        assert_eq!(metadata["record"], "metadata");
        assert_eq!(
            metadata["schema"],
            "spinon.c11-4.repeated-text-row-span-three-fit-mixed-runtime-sample/v1"
        );
        assert_eq!(metadata["sceneCount"], 6_912);
        assert_eq!(metadata["fullOracleSceneCount"], 138_240);
        assert_eq!(metadata["selectedTrackListCount"], 36);
        assert_eq!(
            metadata["fullOracleSha256"],
            "d6f96c63404dbf6f432b2abc1f5bc0aea5d630ebdc6dfa8055c9d7456895b4ea"
        );
        assert_eq!(metadata["trackListSelection"].as_array().unwrap().len(), 36);
        parsed
            .filter(|record| record["record"] == "scene")
            .collect()
    })
}

fn batches() -> BTreeMap<String, Vec<&'static Value>> {
    let mut batches = BTreeMap::<String, Vec<&'static Value>>::new();
    for record in records() {
        let track_list = record["scene"]["trackList"]
            .as_str()
            .expect("sample track list is a string")
            .to_owned();
        batches.entry(track_list).or_default().push(record);
    }
    batches
}

fn topology_item_ids(topology: &str) -> &'static [&'static str] {
    if topology.starts_with("span2Six-") {
        &["A", "B", "C", "D", "E"]
    } else if topology.starts_with("span2Span3Six-") {
        &["A", "B", "C", "D"]
    } else {
        panic!("unknown three-fit mixed topology {topology:?}")
    }
}

fn encoded_minimum_mask(scene: &Value) -> usize {
    let topology = scene["topology"].as_str().unwrap();
    let ids = topology_item_ids(topology);
    let auto_items = scene["autoItems"].as_array().unwrap();
    let mut mask = 0;
    for (bit, id) in ids.iter().enumerate() {
        if auto_items.iter().any(|item| item.as_str() == Some(id)) {
            mask |= 1 << bit;
        }
    }
    assert_eq!(scene["minimumMask"].as_u64(), Some(mask as u64));
    assert!(
        auto_items
            .iter()
            .all(|item| { item.as_str().is_some_and(|id| ids.contains(&id)) })
    );
    mask
}

fn source_order_key(scene: &Value) -> String {
    let topology = scene["topology"].as_str().unwrap();
    let family = topology
        .strip_suffix("-forward")
        .or_else(|| topology.strip_suffix("-reverse"))
        .expect("topology records source order");
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

fn assert_rows_equal(actual: &[f32], expected: &[f32], label: &str) {
    assert_eq!(actual.len(), expected.len(), "{label}: used row count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.01,
            "{label}: row {index} differs: {actual:?} vs {expected:?}"
        );
    }
}

fn assert_frames_equal(
    actual: &BTreeMap<String, [f32; 4]>,
    expected: &BTreeMap<String, [f32; 4]>,
    label: &str,
) {
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );
    for (id, frame) in actual {
        assert_frame(*frame, expected[id], &format!("{label}: {id}"));
    }
}

#[test]
fn parsed_root_html_three_fit_mixed_minimum_matches_6912_chrome_scenes() {
    let batches = batches();
    assert_eq!(
        batches.len(),
        36,
        "fixed runtime sample selects 36 track lists"
    );
    let mut fit_positions = BTreeSet::new();
    let mut tails = BTreeSet::new();
    let mut cap_assignments = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut masks_by_track_topology = BTreeMap::<(String, String), BTreeSet<usize>>::new();
    let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
    let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
    let mut grid_end_overflow_scenes = 0;
    let mut mixed_grid_end_overflow_scenes = 0;
    let mut maximum_grid_end_overflow = 0.0_f32;
    let mut sample_index = 0;

    for (track_list, scenes) in batches {
        assert_eq!(scenes.len(), 192, "{track_list}: complete matrix context");
        let (session, initial) = run(&source_for_three_fit_mixed_minimum_track_list(&track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: three-fit mixed source initializes: {:?}; diagnostics={:?}",
            initial.error,
            initial.diagnostics
        );
        for record in scenes {
            let scene = &record["scene"];
            let label = format!("three-fit mixed sample {sample_index}: {scene}");
            let mask = encoded_minimum_mask(scene);
            configure_three_fit_mixed_minimum_scene(&session, record);
            let result = observe_current(&session, record, &label);

            fit_positions.insert(scene["fitPositions"].to_string());
            tails.insert(scene["tail"].to_string());
            cap_assignments.insert(scene["fitCaps"].to_string());
            alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
            topologies.insert(scene["topology"].as_str().unwrap().to_owned());
            masks_by_track_topology
                .entry((
                    track_list.clone(),
                    scene["topology"].as_str().unwrap().to_owned(),
                ))
                .or_default()
                .insert(mask);
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

            let grid = frame(&record["gridFrame"]);
            let content_bottom = grid[1] + grid[3] - 1.0;
            let scene_overflow = record["items"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|item| item.get("payloadFrame"))
                .filter(|payload| !payload.is_null())
                .map(|payload| {
                    let payload = frame(payload);
                    payload[1] + payload[3] - content_bottom
                })
                .fold(0.0_f32, f32::max);
            if scene_overflow > 0.01 {
                grid_end_overflow_scenes += 1;
                maximum_grid_end_overflow = maximum_grid_end_overflow.max(scene_overflow);
                if mask != 0
                    && mask
                        != (1 << topology_item_ids(scene["topology"].as_str().unwrap()).len()) - 1
                {
                    mixed_grid_end_overflow_scenes += 1;
                }
            }
            sample_index += 1;
        }
    }

    assert_eq!(sample_index, 6_912);
    for (expected_index, record) in records().iter().enumerate() {
        assert_eq!(record["sampleIndex"].as_u64(), Some(expected_index as u64));
        if expected_index > 0 {
            assert!(
                records()[expected_index - 1]["index"].as_u64().unwrap()
                    < record["index"].as_u64().unwrap(),
                "Chrome oracle order is retained"
            );
        }
    }
    assert_eq!(fit_positions.len(), 20);
    assert_eq!(tails.len(), 6);
    assert_eq!(cap_assignments.len(), 6);
    assert_eq!(
        alignments,
        BTreeSet::from(["start".to_owned(), "stretch".to_owned()])
    );
    assert_eq!(
        topologies,
        BTreeSet::from([
            "span2Six-forward".to_owned(),
            "span2Six-reverse".to_owned(),
            "span2Span3Six-forward".to_owned(),
            "span2Span3Six-reverse".to_owned(),
        ])
    );
    for ((track_list, topology), masks) in masks_by_track_topology {
        let expected_count = if topology.starts_with("span2Six-") {
            32
        } else {
            16
        };
        assert_eq!(
            masks,
            (0..expected_count).collect(),
            "{track_list}/{topology}: all masks"
        );
    }

    assert_eq!(source_order_groups.len(), 3_456);
    for (key, mut pair) in source_order_groups {
        pair.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(pair.len(), 2, "{key}: forward/reverse pair");
        assert_ne!(pair[0].0, pair[1].0);
        assert_rows_equal(&pair[0].1.rows, &pair[1].1.rows, &key);
        assert_frames_equal(&pair[0].1.item_frames, &pair[1].1.item_frames, &key);
        assert_frames_equal(&pair[0].1.payload_frames, &pair[1].1.payload_frames, &key);
    }

    assert_eq!(alignment_groups.len(), 3_456);
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
            .expect("one auto track exists")
            + 1;
        assert_eq!(pair[0].1.rows.len(), pair[1].1.rows.len());
        for (index, (start, stretch)) in pair[0].1.rows.iter().zip(&pair[1].1.rows).enumerate() {
            if index == auto_row {
                assert!(stretch + 0.01 >= *start, "{key}: stretch may grow auto row");
            } else {
                assert!(
                    (stretch - start).abs() <= 0.01,
                    "{key}: non-auto row {index}"
                );
            }
        }
    }
    assert_eq!(grid_end_overflow_scenes, 56);
    assert_eq!(mixed_grid_end_overflow_scenes, 52);
    assert_eq!(maximum_grid_end_overflow, 20.0);
}
