use super::dynamic::{configure_scene, observe_current, source_for_track_list};
use super::*;
use std::collections::{BTreeMap, BTreeSet};

fn source_order_key(scene: &Value) -> String {
    let topology = scene["topology"].as_str().unwrap();
    let family = topology
        .strip_suffix("-forward")
        .or_else(|| topology.strip_suffix("-reverse"))
        .expect("topology records a source order");
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
        expected.keys().collect::<Vec<_>>()
    );
    for (id, frame) in actual {
        assert_frame(*frame, expected[id], &format!("{label}: item {id}"));
    }
}

#[test]
fn parsed_root_html_three_fit_row_span_matches_all_11520_chrome_scenes() {
    let mut batches = BTreeMap::<String, Vec<Value>>::new();
    for record in records() {
        let track_list = record["scene"]["trackList"]
            .as_str()
            .expect("track list is a string")
            .to_owned();
        batches.entry(track_list).or_default().push(record);
    }
    assert_eq!(batches.len(), 720, "20 triples × 6 tails × 6 caps");

    let mut fit_positions = BTreeSet::new();
    let mut tails = BTreeSet::new();
    let mut cap_assignments = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut minimums = BTreeSet::new();
    let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
    let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
    let mut scene_index = 0;

    for (track_list, scenes) in batches {
        assert_eq!(scenes.len(), 16, "{track_list}: complete scene batch");
        for (pair_index, pair) in scenes.chunks_exact(2).enumerate() {
            let baseline_scene = &pair[0]["scene"];
            let candidate_scene = &pair[1]["scene"];
            assert_eq!(baseline_scene["minimum"], "zero");
            assert_eq!(candidate_scene["minimum"], "auto");
            for field in [
                "trackList",
                "trackMembers",
                "fitPositions",
                "fitCaps",
                "tail",
                "alignContent",
                "topology",
                "items",
            ] {
                assert_eq!(
                    baseline_scene[field], candidate_scene[field],
                    "{track_list} pair {pair_index}: {field}"
                );
            }
        }

        let (session, initial) = run(&source_for_track_list(&track_list));
        assert_eq!(
            initial.state,
            crate::session::RuntimeLayoutState::Ready,
            "{track_list}: static matrix source initializes: {:?}; diagnostics={:?}",
            initial.error,
            initial.diagnostics
        );
        for record in scenes {
            let scene = &record["scene"];
            let label = format!("three-fit scene {scene_index}: {scene}");
            configure_scene(&session, &record);
            let result = observe_current(&session, &record, &label);
            fit_positions.insert(scene["fitPositions"].to_string());
            tails.insert(scene["tail"].to_string());
            cap_assignments.insert(scene["fitCaps"].to_string());
            alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
            topologies.insert(scene["topology"].as_str().unwrap().to_owned());
            minimums.insert(scene["minimum"].as_str().unwrap().to_owned());
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
    }

    assert_eq!(scene_index, 11_520);
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
    assert_eq!(
        minimums,
        BTreeSet::from(["auto".to_owned(), "zero".to_owned()])
    );

    assert_eq!(source_order_groups.len(), 5_760);
    for (key, mut scenes) in source_order_groups {
        scenes.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(scenes.len(), 2, "{key}: forward/reverse pair");
        assert_ne!(scenes[0].0, scenes[1].0, "{key}: DOM order differs");
        assert_rows_equal(&scenes[0].1.rows, &scenes[1].1.rows, &key);
        assert_frames_equal(&scenes[0].1.item_frames, &scenes[1].1.item_frames, &key);
    }

    assert_eq!(alignment_groups.len(), 5_760);
    for (key, mut scenes) in alignment_groups {
        scenes.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(scenes.len(), 2, "{key}: start/stretch pair");
        assert_eq!(scenes[0].0, "start");
        assert_eq!(scenes[1].0, "stretch");
        let track_members = key
            .split('|')
            .next()
            .expect("alignment key starts with track list")
            .split_whitespace()
            .collect::<Vec<_>>();
        let auto_row = track_members
            .iter()
            .position(|member| *member == "auto")
            .expect("one auto track exists")
            + 1;
        assert_eq!(scenes[0].1.rows.len(), scenes[1].1.rows.len());
        for (index, (start, stretch)) in scenes[0].1.rows.iter().zip(&scenes[1].1.rows).enumerate()
        {
            if index == auto_row {
                assert!(
                    stretch + 0.01 >= *start,
                    "{key}: stretch may grow the auto row"
                );
            } else {
                assert!(
                    (stretch - start).abs() <= 0.01,
                    "{key}: non-auto row {index} must not stretch"
                );
            }
        }
    }
}
