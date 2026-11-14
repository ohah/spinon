use super::super::*;
use super::scene::verify_scene;
use super::source::{observation_lines, read_metadata, source_for_mixed, source_for_track_list};

fn verify_track_list_batch(records: &[Value], batch_index: usize) -> usize {
    assert!(!records.is_empty());
    let track_list = records[0]["scene"]["trackList"].as_str().unwrap();
    let row_profile = records[0]["scene"]["rowProfile"].as_str().unwrap();
    assert!(records.iter().all(|record| {
        record["scene"]["trackList"] == track_list && record["scene"]["rowProfile"] == row_profile
    }));
    let expected_scene_count = if row_profile == "three-row" { 48 } else { 192 };
    assert_eq!(
        records.len(),
        expected_scene_count,
        "batch {batch_index}: full mask/alignment/topology matrix"
    );
    let source = source_for_track_list(track_list, row_profile, records);
    let (session, initial) = run(&source);
    let cascade_error = session.ua_cascade_snapshot().error;
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "batch {batch_index} {track_list}: initial mixed source publishes: layout={:?}; diagnostics={:?}; cascade={:?}",
        initial.error,
        initial.diagnostics,
        cascade_error
    );
    let eligible = records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            verify_scene(&session, record, batch_index * expected_scene_count + index)
        })
        .sum();
    if batch_index % 5 == 4 || batch_index == 0 {
        println!(
            "one-fit mixed RuntimeSession batches: {}/900 ({track_list})",
            batch_index + 1
        );
        std::io::Write::flush(&mut std::io::stdout().lock())
            .expect("flush one-fit mixed matrix progress");
    }
    eligible
}

fn verify_recorded_mixed_scene(scene_index: usize) {
    let mut lines = observation_lines();
    let _metadata = read_metadata(&mut lines);
    let mut group = Vec::new();
    let mut current_track_list = None::<String>;
    let mut target = None::<Value>;

    for line in lines {
        let line = line.expect("one-fit mixed Chrome scene decompresses");
        let record =
            serde_json::from_str::<Value>(&line).expect("one-fit mixed Chrome scene is valid JSON");
        let track_list = record["scene"]["trackList"]
            .as_str()
            .expect("scene records its track list")
            .to_owned();
        if current_track_list
            .as_ref()
            .is_some_and(|current| current != &track_list)
        {
            if target.is_some() {
                break;
            }
            group.clear();
        }
        current_track_list = Some(track_list);
        if record["index"] == scene_index {
            target = Some(record.clone());
        }
        group.push(record);
    }

    let target = target.unwrap_or_else(|| panic!("Chrome mixed scene {scene_index} exists"));
    let track_list = target["scene"]["trackList"].as_str().unwrap();
    let row_profile = target["scene"]["rowProfile"].as_str().unwrap();
    let source = source_for_track_list(track_list, row_profile, &group);
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "one-fit mixed scene {scene_index} publishes: {:?}",
        layout.error
    );
    verify_scene(&session, &target, scene_index);
}

#[test]
fn parsed_root_html_one_fit_row_span_accepts_itemwise_mixed_minimum() {
    let mut lines = observation_lines();
    let _metadata = read_metadata(&mut lines);
    let record = lines
        .map(|line| {
            serde_json::from_str::<Value>(&line.expect("one-fit mixed scene decompresses"))
                .expect("one-fit mixed scene is JSON")
        })
        .find(|record| {
            let scene = &record["scene"];
            scene["fitPosition"] == 1
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-forward"
                && scene["minimumMask"] == 1
        })
        .expect("Chrome matrix has an itemwise mixed one-fit scene");
    let scene = &record["scene"];
    let (session, layout) = run(&source_for_mixed(&record));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "one-fit mixed scene publishes layout: {:?}",
        layout.error
    );
    let completed = layout
        .completed
        .expect("mixed scene publishes layout snapshot");
    assert_optional_track_trace(&completed, &record, "one-fit mixed item minimum");
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("mixed scene publishes render output");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);
    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Root HTML Grid owner exists");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(scene)),
        "mixed minimum keeps the candidate track list"
    );

    let auto_items = scene["autoItems"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_rows = rows(&record);
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        let actual = style_for_color(styles, color_for_item(id), id);
        let expected = if id != "empty" && auto_items.contains(id) {
            spinon_style::ComputedCssDimension::Auto
        } else {
            spinon_style::ComputedCssDimension::LengthPx(0.0)
        };
        assert_eq!(
            actual.layout_dimensions.min_height, expected,
            "{id} computed minimum"
        );
        assert_frame(
            frame_for_paint(render, color_for_item(id)),
            frame(&item["frame"]),
            &format!("{id} item frame"),
        );
        if id != "empty" {
            let expected_payload = frame(&item["payloadFrame"]);
            assert_frame(
                frame_for_paint(render, color_for_payload(id)),
                expected_payload,
                &format!("{id} payload frame"),
            );
            let span_height = item_span_height(item, &expected_rows, id);
            assert!(
                (frame(&item["frame"])[3] - span_height).abs() <= 0.01,
                "{id} item height matches covered Chrome rows"
            );
            assert!(
                expected_payload[1] + expected_payload[3]
                    <= frame(&item["frame"])[1] + frame(&item["frame"])[3] + 0.01,
                "{id} payload remains inside its item"
            );
        }
    }
}

#[test]
fn parsed_root_html_one_fit_mixed_minimum_preserves_residual_total_for_scene_110() {
    verify_recorded_mixed_scene(110);
}

#[test]
fn parsed_root_html_one_fit_mixed_minimum_accepts_single_auto_item_for_scene_2() {
    verify_recorded_mixed_scene(2);
}

#[test]
fn parsed_root_html_one_fit_mixed_minimum_replays_three_row_tail_for_scene_1164() {
    verify_recorded_mixed_scene(1164);
}

#[test]
fn parsed_root_html_one_fit_mixed_minimum_preserves_auto_leading_row_for_scene_5774() {
    verify_recorded_mixed_scene(5774);
}

#[test]
fn parsed_root_html_one_fit_mixed_minimum_replays_reversed_tail_for_scene_99449() {
    verify_recorded_mixed_scene(99449);
}

#[test]
fn parsed_root_html_one_fit_row_span_itemwise_mixed_minimum_matches_all_120960_chrome_scenes() {
    let mut lines = observation_lines();
    let metadata = read_metadata(&mut lines);
    let mut batch = Vec::new();
    let mut current_track_list = None::<String>;
    let mut seen_track_lists = std::collections::BTreeSet::new();
    let mut scene_count = 0usize;
    let mut eligible_auto_items = 0usize;
    let mut batch_count = 0usize;

    for line in lines {
        let line = line.expect("one-fit mixed Chrome scene decompresses");
        let record =
            serde_json::from_str::<Value>(&line).expect("one-fit mixed Chrome scene is valid JSON");
        assert_eq!(record["record"], "scene");
        let track_list = record["scene"]["trackList"]
            .as_str()
            .expect("scene records its track list")
            .to_owned();
        if current_track_list
            .as_ref()
            .is_some_and(|current| current != &track_list)
        {
            let finished = current_track_list
                .take()
                .expect("a scene batch has a track list");
            assert!(
                seen_track_lists.insert(finished),
                "track-list batch is unique"
            );
            scene_count += batch.len();
            eligible_auto_items += verify_track_list_batch(&batch, batch_count);
            batch_count += 1;
            batch.clear();
        }
        current_track_list.get_or_insert(track_list);
        batch.push(record);
    }

    if !batch.is_empty() {
        let finished = current_track_list
            .take()
            .expect("final scene batch has a track list");
        assert!(
            seen_track_lists.insert(finished),
            "final track-list batch is unique"
        );
        scene_count += batch.len();
        eligible_auto_items += verify_track_list_batch(&batch, batch_count);
        batch_count += 1;
    }

    assert_eq!(scene_count, 120_960);
    assert_eq!(batch_count, 900);
    assert_eq!(seen_track_lists.len(), 900);
    assert_eq!(eligible_auto_items, 169_344);
    println!(
        "verified {} RuntimeSession scenes across {} track lists; eligible automatic-minimum item occurrences {}",
        metadata["sceneCount"], batch_count, eligible_auto_items
    );
}
