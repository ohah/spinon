use super::*;
use spinon_style::{ComputedCssGridAutoRows, ComputedCssGridPlacement};
use std::collections::BTreeSet;

#[test]
fn parsed_root_html_later_fit_content_positions_four_through_six_match_all_4320_chrome_scenes() {
    let (metadata, scenes) = capture();
    assert_eq!(
        metadata["fitContentPositions"],
        serde_json::json!([4, 5, 6])
    );
    assert_eq!(metadata["tailPermutationCount"], 30);
    assert_eq!(metadata["caseCount"], 4320);
    assert_eq!(scenes.len(), 4320);

    let mut seen = BTreeSet::new();
    let mut fit_position_counts = BTreeMap::<usize, usize>::new();
    let mut topology_counts = BTreeMap::<String, usize>::new();
    let mut tails = BTreeSet::new();
    let mut caps = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut observations = BTreeMap::<SceneKey, (FramesByItem, Vec<f32>)>::new();
    let mut payload_line_counts = BTreeMap::<String, BTreeSet<usize>>::new();

    for (scene_index, expected) in scenes.iter().enumerate() {
        let key = scene_key(expected);
        let fit_position = key.0;
        let topology = key.4.clone();
        assert!(matches!(fit_position, 4..=6));
        assert!(matches!(
            key.2.as_str(),
            "0px" | "0%" | "10px" | "20px" | "20%" | "120%"
        ));
        assert!(matches!(key.3.as_str(), "start" | "stretch"));
        assert!(seen.insert(key.clone()), "duplicate scene key {key:?}");
        *fit_position_counts.entry(fit_position).or_default() += 1;
        *topology_counts.entry(topology.clone()).or_default() += 1;
        caps.insert(key.2.clone());
        alignments.insert(key.3.clone());
        tails.insert(key.1.clone());

        let track_list = expected["trackList"]
            .as_array()
            .expect("trackList is an array")
            .iter()
            .map(|track| track.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(track_list.len(), 6);
        assert_eq!(
            track_list
                .iter()
                .filter(|track| **track == "min-content")
                .count(),
            2
        );
        assert_eq!(
            track_list
                .iter()
                .filter(|track| **track == "max-content")
                .count(),
            2
        );
        assert_eq!(
            track_list.iter().filter(|track| **track == "auto").count(),
            1
        );
        assert_eq!(
            track_list
                .iter()
                .filter(|track| track.starts_with("fit-content("))
                .count(),
            1
        );

        let source = source_for_scene(expected, None);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index} {fit_position}/{topology}/{} /{}: layout={:?}; diagnostics={:?}; cascade={:?}",
            key.2,
            key.3,
            layout.error,
            layout.diagnostics,
            session.ua_cascade_snapshot().error
        );
        let completed = layout.completed.as_ref().expect("scene publishes layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("scene publishes render output");
        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("same revision publishes computed styles");
        assert_eq!(completed.key, cascade.key, "scene {scene_index}: revision");

        let grid_style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("scene has one parsed Grid owner");
        let expected_tracks = track_list
            .iter()
            .map(|track| computed_track(track))
            .collect::<Vec<_>>();
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "scene {scene_index}: authored typed track order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(|value| value.split_whitespace().collect::<Vec<_>>()),
            Some(track_list.clone()),
            "scene {scene_index}: computed track serialization"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            Some(key.3.as_str()),
            "scene {scene_index}: computed alignment"
        );

        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame publishes");
        assert_frame_001(
            [
                grid_frame.x,
                grid_frame.y,
                grid_frame.width,
                grid_frame.height,
            ],
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let rows = row_sizes(completed, expected);
        let expected_rows = expected["rows"].as_array().expect("rows is an array");
        assert_eq!(
            rows.len(),
            7,
            "scene {scene_index}: one explicit plus six implicit rows"
        );
        let mut frames = BTreeMap::new();
        let expected_items = expected["items"].as_array().expect("items is an array");
        let mut expected_texts = Vec::new();
        for expected_item in expected_items {
            let id = expected_item["id"].as_str().unwrap();
            let item_frame = frame_for_paint(render, color_for_item(id));
            assert_frame_001(
                item_frame,
                frame(&expected_item["frame"]),
                &format!("scene {scene_index} item {id}"),
            );
            frames.insert(id.to_owned(), item_frame);
            if expected_item["payload"].is_array() {
                let payload_frame = frame_for_paint(render, color_for_payload(id));
                let expected_payload = frame(&expected_item["payload"]);
                assert_frame_001(
                    payload_frame,
                    expected_payload,
                    &format!("scene {scene_index} payload {id}"),
                );
                let line_count = expected_item["lineYs"].as_array().unwrap().len();
                assert_eq!(expected_payload[3], line_count as f32 * 10.0);
                payload_line_counts
                    .entry(id.to_owned())
                    .or_default()
                    .insert(line_count);
                expected_texts.push((id, text_for(id), expected_payload[1], line_count));

                let start = expected_item["rowStart"]
                    .as_str()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let span = expected_item["rowEnd"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("span ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let start_index = start - 1;
                let expected_y = expected_rows[..start_index]
                    .iter()
                    .map(|row| row.as_f64().unwrap() as f32)
                    .sum::<f32>();
                let expected_height = expected_rows[start_index..start_index + span]
                    .iter()
                    .map(|row| row.as_f64().unwrap() as f32)
                    .sum::<f32>();
                assert!(
                    (item_frame[1] - expected_y).abs() <= 0.01,
                    "scene {scene_index} {id}: span top follows used rows"
                );
                assert!(
                    (item_frame[3] - expected_height).abs() <= 0.01,
                    "scene {scene_index} {id}: span height equals used-row sum"
                );
            }
        }

        let runtime_texts = render.texts();
        assert_eq!(
            runtime_texts.len(),
            expected_texts.len(),
            "scene {scene_index}: text payload count"
        );
        for (id, value, expected_origin_y, expected_line_count) in expected_texts {
            let glyph_count = value.bytes().filter(u8::is_ascii_uppercase).count();
            let text = runtime_texts
                .iter()
                .find(|text| text.glyphs().len() == glyph_count)
                .unwrap_or_else(|| panic!("scene {scene_index} {id}: text glyphs not found"));
            assert_eq!(
                text.origin_y_css_px(),
                expected_origin_y,
                "scene {scene_index} {id}: text origin"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                expected_line_count,
                "scene {scene_index} {id}: line count"
            );
            let baselines = baselines.into_iter().collect::<Vec<_>>();
            assert!(
                baselines.windows(2).all(|pair| pair[1] - pair[0] == 10),
                "scene {scene_index} {id}: 10px line-height baselines {baselines:?}"
            );
        }

        let placements = cascade.roots[0]
            .styles
            .elements
            .iter()
            .filter_map(|style| style.grid_item_placement.as_ref())
            .filter(|placement| {
                placement.row_start != ComputedCssGridPlacement::Auto
                    || placement.row_end != ComputedCssGridPlacement::Auto
            })
            .collect::<Vec<_>>();
        assert_eq!(
            placements.len(),
            expected_items.len(),
            "scene {scene_index}: explicit placements"
        );
        assert!(placements.iter().all(|placement| {
            placement.column_start == ComputedCssGridPlacement::Line(1)
                && placement.column_end == ComputedCssGridPlacement::Line(2)
        }));
        for expected_item in expected_items {
            let row_start = expected_item["rowStart"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let row_end = match expected_item["rowEnd"]
                .as_str()
                .unwrap()
                .strip_prefix("span ")
            {
                Some(span) => ComputedCssGridPlacement::Span(span.parse::<i32>().unwrap()),
                None => ComputedCssGridPlacement::Line(
                    expected_item["rowEnd"].as_str().unwrap().parse().unwrap(),
                ),
            };
            assert!(
                placements.iter().any(|placement| {
                    placement.row_start == ComputedCssGridPlacement::Line(row_start)
                        && placement.row_end == row_end
                }),
                "scene {scene_index}: missing row placement {row_start}/{row_end:?}"
            );
        }

        observations.insert(key, (frames, rows));
    }

    assert_eq!(seen.len(), 4320);
    assert_eq!(
        fit_position_counts,
        BTreeMap::from([(4, 1440), (5, 1440), (6, 1440)])
    );
    assert_eq!(topology_counts.len(), 4);
    assert!(topology_counts.values().all(|count| *count == 1080));
    assert_eq!(tails.len(), 30);
    assert_eq!(caps.len(), 6);
    assert_eq!(
        alignments,
        BTreeSet::from(["start".to_owned(), "stretch".to_owned()])
    );
    assert_eq!(payload_line_counts["A"], BTreeSet::from([6]));
    assert_eq!(payload_line_counts["B"], BTreeSet::from([5]));
    assert_eq!(payload_line_counts["C"], BTreeSet::from([4]));
    assert_eq!(payload_line_counts["D"], BTreeSet::from([4]));
    assert_eq!(payload_line_counts["E"], BTreeSet::from([3]));

    let source_order_pairs = [
        (
            "span2-six-forward",
            "span2-six-reversed",
            &["A", "B", "C", "D", "E"][..],
        ),
        (
            "span2-span3-six-forward",
            "span2-span3-six-reversed",
            &["A", "B", "C", "D"][..],
        ),
    ];
    for fit_position in 4..=6 {
        for tail in &tails {
            for cap in ["0px", "0%", "10px", "20px", "20%", "120%"] {
                for align in ["start", "stretch"] {
                    for (forward, reversed, ids) in source_order_pairs {
                        let left_key = (
                            fit_position,
                            tail.clone(),
                            cap.to_owned(),
                            align.to_owned(),
                            forward.to_owned(),
                        );
                        let right_key = (
                            fit_position,
                            tail.clone(),
                            cap.to_owned(),
                            align.to_owned(),
                            reversed.to_owned(),
                        );
                        let (left_frames, left_rows) = observations.get(&left_key).unwrap();
                        let (right_frames, right_rows) = observations.get(&right_key).unwrap();
                        assert_eq!(
                            left_rows, right_rows,
                            "source-order row invariance {left_key:?}"
                        );
                        for id in ids {
                            assert_frame_001(
                                left_frames[*id],
                                right_frames[*id],
                                &format!(
                                    "source-order invariant {fit_position}/{tail}/{cap}/{align}/{id}"
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    for fit_position in 4..=6 {
        let mut low_cap_differences = 0;
        for tail in &tails {
            for topology in topology_counts.keys() {
                for align in ["start", "stretch"] {
                    for cap in ["0px", "0%", "10px", "20px", "20%", "120%"] {
                        let start_key = (
                            fit_position,
                            tail.clone(),
                            cap.to_owned(),
                            "start".to_owned(),
                            topology.clone(),
                        );
                        let stretch_key = (
                            fit_position,
                            tail.clone(),
                            cap.to_owned(),
                            "stretch".to_owned(),
                            topology.clone(),
                        );
                        let (_, start_rows) = observations.get(&start_key).unwrap();
                        let (_, stretch_rows) = observations.get(&stretch_key).unwrap();
                        assert_eq!(start_rows[0], stretch_rows[0], "explicit row is unchanged");
                        let track_list = scenes
                            .iter()
                            .find(|scene| scene_key(scene) == start_key)
                            .unwrap()["trackList"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|track| track.as_str().unwrap())
                            .collect::<Vec<_>>();
                        for row_index in 1..7 {
                            if track_list[row_index - 1] != "auto" {
                                assert_eq!(
                                    start_rows[row_index], stretch_rows[row_index],
                                    "non-auto row {row_index} remains unchanged"
                                );
                            }
                        }
                        assert!(
                            stretch_rows.iter().sum::<f32>() + 1e-7
                                >= start_rows.iter().sum::<f32>()
                        );
                    }

                    let key_for = |cap: &str| {
                        (
                            fit_position,
                            tail.clone(),
                            cap.to_owned(),
                            align.to_owned(),
                            topology.clone(),
                        )
                    };
                    let low = &observations[&key_for("10px")].1;
                    let high = &observations[&key_for("20px")].1;
                    if low != high {
                        low_cap_differences += 1;
                        assert_ne!(low[fit_position], high[fit_position]);
                    }
                    assert_eq!(
                        observations[&key_for("0px")].1,
                        observations[&key_for("0%")].1
                    );
                    assert_eq!(
                        observations[&key_for("20px")].1,
                        observations[&key_for("20%")].1
                    );
                    assert_eq!(
                        observations[&key_for("20px")].1,
                        observations[&key_for("120%")].1
                    );
                }
            }
        }
        assert_eq!(
            low_cap_differences,
            match fit_position {
                4 | 5 => 48,
                6 => 24,
                _ => unreachable!(),
            },
            "position {fit_position}: low and high caps differ in the Chrome-defined corpus"
        );
    }
}
