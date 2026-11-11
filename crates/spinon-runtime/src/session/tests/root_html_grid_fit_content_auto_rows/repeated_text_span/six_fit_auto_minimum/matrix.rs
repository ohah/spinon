use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

fn batches() -> &'static BTreeMap<String, Vec<&'static Value>> {
    static BATCHES: OnceLock<BTreeMap<String, Vec<&'static Value>>> = OnceLock::new();
    BATCHES.get_or_init(|| {
        let mut batches = BTreeMap::<String, Vec<&'static Value>>::new();
        for record in records() {
            let track_list = record["scene"]["trackList"]
                .as_str()
                .expect("track list is a string")
                .to_owned();
            batches.entry(track_list).or_default().push(record);
        }
        batches
    })
}

fn source_order_key(scene: &Value) -> String {
    let topology = scene["topology"].as_str().unwrap();
    let family = topology
        .strip_suffix("-forward")
        .or_else(|| topology.strip_suffix("-reverse"))
        .expect("topology records its source order");
    format!(
        "{}|{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["alignContent"].as_str().unwrap(),
        scene["minimum"].as_str().unwrap(),
        family
    )
}

fn alignment_key(scene: &Value) -> String {
    format!(
        "{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["topology"].as_str().unwrap(),
        scene["minimum"].as_str().unwrap()
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
        assert_frame(*frame, expected[id], &format!("{label}: {id}"));
    }
}

fn geometry_equal(
    left: &super::super::three_fit_auto_minimum::SceneResult,
    right: &super::super::three_fit_auto_minimum::SceneResult,
) -> bool {
    left.rows.len() == right.rows.len()
        && left
            .rows
            .iter()
            .zip(&right.rows)
            .all(|(left, right)| (left - right).abs() <= 0.01)
        && left.item_frames.len() == right.item_frames.len()
        && left.item_frames.iter().all(|(id, left)| {
            right.item_frames.get(id).is_some_and(|right| {
                left.iter()
                    .zip(right)
                    .all(|(left, right)| (left - right).abs() <= 0.01)
            })
        })
        && left.payload_frames.len() == right.payload_frames.len()
        && left.payload_frames.iter().all(|(id, left)| {
            right.payload_frames.get(id).is_some_and(|right| {
                left.iter()
                    .zip(right)
                    .all(|(left, right)| (left - right).abs() <= 0.01)
            })
        })
}

fn run_matrix_chunk(start: usize, end: usize) {
    let batches = batches();
    assert!(start < end && end <= batches.len());
    assert_eq!(batches.len(), 720, "all six-cap orders are present");

    let mut source_order_groups =
        BTreeMap::<String, Vec<(String, super::super::three_fit_auto_minimum::SceneResult)>>::new();
    let mut alignment_groups =
        BTreeMap::<String, Vec<(String, super::super::three_fit_auto_minimum::SceneResult)>>::new();
    let mut scene_index = start * 16;
    let mut baseline_floor_violations = 0;
    let mut candidate_floor_violations = 0;
    let mut paired_geometry_differences = 0;

    for (batch_index, (track_list, scenes)) in
        batches.iter().enumerate().skip(start).take(end - start)
    {
        assert_eq!(scenes.len(), 16, "{track_list}: complete scene batch");
        for pair in scenes.chunks_exact(2) {
            let baseline = &pair[0]["scene"];
            let candidate = &pair[1]["scene"];
            assert_eq!(baseline["minimum"], "zero");
            assert_eq!(candidate["minimum"], "auto");
            for field in [
                "trackList",
                "trackMembers",
                "fitPositions",
                "fitCaps",
                "alignContent",
                "topology",
                "items",
            ] {
                assert_eq!(
                    baseline[field], candidate[field],
                    "{track_list}: baseline/candidate pair field {field}"
                );
            }
        }

        let (session, initial) = run(&source_for_track_list(track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: matrix source initializes: {:?}; diagnostics={:?}",
            initial.error,
            initial.diagnostics
        );
        let mut results = Vec::with_capacity(scenes.len());
        for &record in scenes {
            let scene = &record["scene"];
            let label = format!("six-fit scene {scene_index}: {scene}");
            configure_scene(&session, record);
            let result = observe_current(&session, record, &label);

            for item in record["items"].as_array().unwrap() {
                let id = item["id"].as_str().unwrap();
                if id == "empty" || !eligible_for_auto_minimum(record, id) {
                    continue;
                }
                let item_height = result.item_frames[id][3];
                let payload_height = result.payload_frames[id][3];
                if item_height + 0.01 < payload_height {
                    if scene["minimum"] == "auto" {
                        candidate_floor_violations += 1;
                    } else {
                        baseline_floor_violations += 1;
                    }
                }
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
                .push((
                    scene["alignContent"].as_str().unwrap().to_owned(),
                    result.clone(),
                ));
            results.push(result);
            scene_index += 1;
        }

        for (pair_index, (pair, observed)) in scenes
            .chunks_exact(2)
            .zip(results.chunks_exact(2))
            .enumerate()
        {
            assert_eq!(pair[0]["scene"]["minimum"], "zero");
            assert_eq!(pair[1]["scene"]["minimum"], "auto");
            assert!(
                !geometry_equal(&observed[0], &observed[1]),
                "{track_list} pair {pair_index}: baseline and candidate geometry should differ"
            );
            paired_geometry_differences += 1;
        }

        if (batch_index + 1) % 30 == 0 || batch_index + 1 == end {
            eprintln!("six-fit matrix track lists: {} / {}", batch_index + 1, end);
        }
    }

    assert_eq!(scene_index, end * 16);
    assert_eq!(candidate_floor_violations, 0);
    assert_eq!(paired_geometry_differences, (end - start) * 8);
    eprintln!(
        "six-fit chunk {start}..{end} payload-floor violations: candidate={candidate_floor_violations}, baseline={baseline_floor_violations}"
    );

    assert_eq!(source_order_groups.len(), (end - start) * 8);
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

    assert_eq!(alignment_groups.len(), (end - start) * 8);
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
}

#[test]
fn parsed_root_html_six_fit_row_span_matrix_metadata_covers_all_11520_scenes() {
    let batches = batches();
    assert_eq!(batches.len(), 720);
    let mut cap_assignments = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut minimums = BTreeSet::new();
    let mut scene_count = 0;
    for (track_list, scenes) in batches {
        assert_eq!(scenes.len(), 16, "{track_list}: complete scene batch");
        for pair in scenes.chunks_exact(2) {
            let baseline = &pair[0]["scene"];
            let candidate = &pair[1]["scene"];
            assert_eq!(baseline["minimum"], "zero");
            assert_eq!(candidate["minimum"], "auto");
            for field in [
                "trackList",
                "trackMembers",
                "fitPositions",
                "fitCaps",
                "alignContent",
                "topology",
                "items",
            ] {
                assert_eq!(baseline[field], candidate[field], "{track_list}: {field}");
            }
        }
        for record in scenes {
            let scene = &record["scene"];
            cap_assignments.insert(scene["fitCaps"].to_string());
            alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
            topologies.insert(scene["topology"].as_str().unwrap().to_owned());
            minimums.insert(scene["minimum"].as_str().unwrap().to_owned());
            scene_count += 1;
        }
    }
    assert_eq!(scene_count, 11_520);
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
    assert_eq!(
        minimums,
        BTreeSet::from(["auto".to_owned(), "zero".to_owned()])
    );
}

#[test]
fn parsed_root_html_six_fit_row_span_matrix_chunk_1_matches_chrome() {
    run_matrix_chunk(0, 180);
}

#[test]
fn parsed_root_html_six_fit_row_span_matrix_chunk_2_matches_chrome() {
    run_matrix_chunk(180, 360);
}

#[test]
fn parsed_root_html_six_fit_row_span_matrix_chunk_3_matches_chrome() {
    run_matrix_chunk(360, 540);
}

#[test]
fn parsed_root_html_six_fit_row_span_matrix_chunk_4_matches_chrome() {
    run_matrix_chunk(540, 720);
}
