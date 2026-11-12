use super::super::three_fit_auto_minimum::SceneResult;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

fn batches() -> &'static BTreeMap<String, Vec<&'static Value>> {
    static BATCHES: OnceLock<BTreeMap<String, Vec<&'static Value>>> = OnceLock::new();
    BATCHES.get_or_init(|| {
        let mut batches = BTreeMap::<String, Vec<&'static Value>>::new();
        for record in mixed_records() {
            let track_list = record["scene"]["trackList"]
                .as_str()
                .expect("track list is a string")
                .to_owned();
            batches.entry(track_list).or_default().push(record);
        }
        batches
    })
}

fn topology_item_ids(topology: &str) -> &'static [&'static str] {
    if topology.starts_with("span2Six-") {
        &["A", "B", "C", "D", "E"]
    } else if topology.starts_with("span2Span3Six-") {
        &["A", "B", "C", "D"]
    } else {
        panic!("unknown mixed-minimum topology {topology:?}")
    }
}

fn encoded_minimum_mask(scene: &Value) -> usize {
    let topology = scene["topology"].as_str().expect("topology is a string");
    let ids = topology_item_ids(topology);
    let auto_items = scene["autoItems"]
        .as_array()
        .expect("scene records auto item IDs");
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
        expected.keys().collect::<Vec<_>>(),
        "{label}: frame owners"
    );
    for (id, frame) in actual {
        super::super::assert_frame(*frame, expected[id], &format!("{label}: {id}"));
    }
}

fn run_matrix_chunk(start: usize, end: usize) {
    let batches = batches();
    assert!(start < end && end <= batches.len());
    assert_eq!(batches.len(), 720, "all six-cap orders are present");

    let mut scene_index = start * 192;
    for (batch_index, (track_list, scenes)) in
        batches.iter().enumerate().skip(start).take(end - start)
    {
        assert_eq!(scenes.len(), 192, "{track_list}: complete mixed-mask batch");
        let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: mixed-mask source initializes: {:?}; diagnostics={:?}; cascade={:?}",
            initial.error,
            initial.diagnostics,
            session.ua_cascade_snapshot().error
        );

        let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
        let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
        for record in scenes {
            let scene = &record["scene"];
            let label = format!("six-fit mixed scene {scene_index}: {scene}");
            assert_eq!(
                encoded_minimum_mask(scene),
                scene["minimumMask"].as_u64().unwrap() as usize
            );
            configure_six_fit_mixed_minimum_scene(&session, record);
            let result = observe_current(&session, record, &label);
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
            scene_index += 1;
        }

        assert_eq!(source_order_groups.len(), 96);
        for (key, mut scenes) in source_order_groups {
            scenes.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(scenes.len(), 2, "{key}: forward/reverse pair");
            assert_ne!(scenes[0].0, scenes[1].0, "{key}: DOM order differs");
            assert_rows_equal(&scenes[0].1.rows, &scenes[1].1.rows, &key);
            assert_frames_equal(&scenes[0].1.item_frames, &scenes[1].1.item_frames, &key);
            assert_frames_equal(
                &scenes[0].1.payload_frames,
                &scenes[1].1.payload_frames,
                &format!("{key}: payload"),
            );
        }

        assert_eq!(alignment_groups.len(), 96);
        for (key, mut scenes) in alignment_groups {
            scenes.sort_by(|left, right| left.0.cmp(&right.0));
            assert_eq!(scenes.len(), 2, "{key}: start/stretch pair");
            assert_eq!(scenes[0].0, "start");
            assert_eq!(scenes[1].0, "stretch");
            assert_rows_equal(&scenes[0].1.rows, &scenes[1].1.rows, &key);
            assert_frames_equal(&scenes[0].1.item_frames, &scenes[1].1.item_frames, &key);
            assert_frames_equal(
                &scenes[0].1.payload_frames,
                &scenes[1].1.payload_frames,
                &format!("{key}: payload"),
            );
        }

        if (batch_index + 1) % 10 == 0 || batch_index + 1 == end {
            eprintln!(
                "six-fit mixed-minimum cap orders: {} / {}",
                batch_index + 1,
                end
            );
        }
    }
    assert_eq!(scene_index, end * 192);
}

#[test]
fn parsed_root_html_six_fit_mixed_minimum_matrix_metadata_covers_all_138240_scenes() {
    let batches = batches();
    assert_eq!(mixed_records().len(), 138_240);
    assert_eq!(batches.len(), 720);
    let mut cap_assignments = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut masks_by_topology = BTreeMap::<String, BTreeSet<usize>>::new();
    let mut uniform_scene_count = 0;

    for (track_list, scenes) in batches {
        assert_eq!(
            scenes.len(),
            192,
            "{track_list}: full mask/alignment/order set"
        );
        for record in scenes {
            let scene = &record["scene"];
            let topology = scene["topology"].as_str().unwrap().to_owned();
            let mask = encoded_minimum_mask(scene);
            let ids = topology_item_ids(&topology);
            masks_by_topology
                .entry(topology.clone())
                .or_default()
                .insert(mask);
            cap_assignments.insert(scene["fitCaps"].to_string());
            alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
            topologies.insert(topology.clone());
            let auto_items = scene["autoItems"].as_array().unwrap();
            for item in record["items"].as_array().unwrap() {
                let id = item["id"].as_str().unwrap();
                let expected_auto = auto_items.iter().any(|value| value.as_str() == Some(id));
                assert_eq!(
                    item["minHeight"].as_str(),
                    Some(if expected_auto { "auto" } else { "0px" })
                );
                if id != "empty" {
                    assert!(ids.contains(&id));
                }
            }
            if mask == 0 || mask == (1 << ids.len()) - 1 {
                uniform_scene_count += 1;
            }
        }
    }

    assert_eq!(cap_assignments.len(), 720);
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
    assert_eq!(uniform_scene_count, 11_520);
    for (topology, masks) in masks_by_topology {
        let expected = if topology.starts_with("span2Six-") {
            32
        } else {
            16
        };
        assert_eq!(masks.len(), expected, "{topology}: minimum masks");
        assert_eq!(masks.first().copied(), Some(0));
        assert_eq!(masks.last().copied(), Some(expected - 1));
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
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_01_matches_chrome,
    0,
    60
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_02_matches_chrome,
    60,
    120
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_03_matches_chrome,
    120,
    180
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_04_matches_chrome,
    180,
    240
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_05_matches_chrome,
    240,
    300
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_06_matches_chrome,
    300,
    360
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_07_matches_chrome,
    360,
    420
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_08_matches_chrome,
    420,
    480
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_09_matches_chrome,
    480,
    540
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_10_matches_chrome,
    540,
    600
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_11_matches_chrome,
    600,
    660
);
matrix_chunk!(
    parsed_root_html_six_fit_mixed_minimum_matrix_chunk_12_matches_chrome,
    660,
    720
);
