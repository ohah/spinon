use super::*;
use spinon_style::{ComputedCssGridAutoRows, ComputedCssGridPlacement};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes() {
    let (metadata, scenes) = scenes();
    assert_eq!(metadata["fitContentPositions"], serde_json::json!([2, 3]));
    assert_eq!(metadata["tailPermutationCount"], 30);
    assert_eq!(metadata["caseCount"], 1920);
    assert_eq!(scenes.len(), 1920);

    let mut seen = BTreeSet::new();
    let mut fit_position_counts = BTreeMap::<usize, usize>::new();
    let mut topology_counts = BTreeMap::<String, usize>::new();
    let mut tails = BTreeSet::new();
    let mut caps = BTreeSet::new();
    let mut actual_cases =
        BTreeMap::<(usize, String, String, String, String), BTreeMap<String, [f32; 4]>>::new();
    let mut text_lines = BTreeMap::<String, BTreeSet<usize>>::new();

    for (scene_index, expected) in scenes.iter().enumerate() {
        let fit_position = expected["fitPosition"]
            .as_u64()
            .expect("fit position is numeric") as usize;
        let topology = expected["topology"]
            .as_str()
            .expect("topology is a string")
            .to_owned();
        let cap = expected["cap"]
            .as_str()
            .expect("cap is a string")
            .to_owned();
        let align = expected["align"]
            .as_str()
            .expect("alignment is a string")
            .to_owned();
        let tail_key = serde_json::to_string(&expected["tail"]).expect("tail serializes");
        assert!(matches!(fit_position, 2 | 3));
        assert!(matches!(align.as_str(), "start" | "stretch"));
        assert!(matches!(cap.as_str(), "20px" | "20%" | "0%" | "120%"));
        assert!(seen.insert((
            fit_position,
            tail_key.clone(),
            cap.clone(),
            align.clone(),
            topology.clone(),
        )));
        assert_eq!(tail_css(expected).len(), 5);
        assert_eq!(
            tail_css(expected)
                .iter()
                .filter(|value| *value == "min-content")
                .count(),
            2
        );
        assert_eq!(
            tail_css(expected)
                .iter()
                .filter(|value| *value == "max-content")
                .count(),
            2
        );
        assert_eq!(
            tail_css(expected)
                .iter()
                .filter(|value| *value == "auto")
                .count(),
            1
        );
        *fit_position_counts.entry(fit_position).or_default() += 1;
        *topology_counts.entry(topology.clone()).or_default() += 1;
        tails.insert(tail_key.clone());
        caps.insert(cap.clone());

        let source = source_for(expected);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index} position {fit_position} {topology} {cap} {align}: layout={:?}; diagnostics={:?}; cascade={:?}",
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
        assert_eq!(
            completed.key, cascade.key,
            "scene {scene_index}: revision key"
        );

        let grid_style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("scene has one parsed Grid owner");
        let expected_track_list = expected["trackList"]
            .as_array()
            .expect("track list is an array")
            .iter()
            .map(|track| track.as_str().expect("track is a string"))
            .collect::<Vec<_>>();
        let computed_tracks = expected_track_list
            .iter()
            .map(|track| computed_track(track))
            .collect::<Vec<_>>();
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(computed_tracks),
            "scene {scene_index}: exact typed track order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(|value| value.split_whitespace().collect::<Vec<_>>()),
            Some(expected_track_list.clone()),
            "scene {scene_index}: computed serialization order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            Some(align.as_str()),
            "scene {scene_index}: computed alignment"
        );

        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame publishes");
        assert_frame(
            [
                grid_frame.x,
                grid_frame.y,
                grid_frame.width,
                grid_frame.height,
            ],
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected["items"].as_array().expect("items is an array");
        let mut frames = BTreeMap::new();
        for expected_item in expected_items {
            let id = expected_item["id"].as_str().expect("item id is a string");
            let item_frame = frame_for_paint(render, color_for_item(id));
            assert_frame(
                item_frame,
                frame(&expected_item["frame"]),
                &format!("scene {scene_index} item {id}"),
            );
            frames.insert(id.to_owned(), item_frame);
            if expected_item["payload"].is_array() {
                let line_count = expected_item["lineYs"]
                    .as_array()
                    .expect("line positions are an array")
                    .len();
                text_lines
                    .entry(id.to_owned())
                    .or_default()
                    .insert(line_count);
                let expected_payload = frame(&expected_item["payload"]);
                assert_eq!(expected_payload[3], line_count as f32 * 10.0);
                let payload_frame = frame_for_paint(render, color_for_payload(id));
                assert_frame(
                    payload_frame,
                    expected_payload,
                    &format!("scene {scene_index} payload {id}, {line_count} lines"),
                );
            }
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
            "scene {scene_index}: items"
        );
        assert!(placements.iter().all(|placement| {
            placement.column_start == ComputedCssGridPlacement::Line(1)
                && placement.column_end == ComputedCssGridPlacement::Line(2)
        }));
        for (row_start, row_end, expected_count) in [
            (
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Line(2),
                1,
            ),
            (
                ComputedCssGridPlacement::Line(2),
                ComputedCssGridPlacement::Span(2),
                1,
            ),
            (
                ComputedCssGridPlacement::Line(3),
                ComputedCssGridPlacement::Span(2),
                1,
            ),
            (
                ComputedCssGridPlacement::Line(2),
                ComputedCssGridPlacement::Span(3),
                usize::from(expected_items.len() == 4),
            ),
        ] {
            assert_eq!(
                placements
                    .iter()
                    .filter(|placement| {
                        placement.row_start == row_start && placement.row_end == row_end
                    })
                    .count(),
                expected_count,
                "scene {scene_index}: placement {row_start:?}/{row_end:?}"
            );
        }

        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        assert_track_trace(completed, expected, &format!("scene {scene_index}"));
        actual_cases.insert((fit_position, tail_key, cap, align, topology), frames);
    }

    assert_eq!(seen.len(), 1920);
    assert_eq!(fit_position_counts, BTreeMap::from([(2, 960), (3, 960)]));
    assert_eq!(tails.len(), 30);
    assert_eq!(caps.len(), 4);
    assert!(topology_counts.values().all(|count| *count == 480));
    assert_eq!(topology_counts.len(), 4);
    assert_eq!(
        text_lines,
        BTreeMap::from([
            ("A".to_owned(), BTreeSet::from([2])),
            ("B".to_owned(), BTreeSet::from([3])),
            ("C".to_owned(), BTreeSet::from([4])),
        ])
    );

    for fit_position in [2, 3] {
        for tail in &tails {
            for cap in ["0%", "120%", "20%", "20px"] {
                for align in ["start", "stretch"] {
                    for (left_topology, right_topology, ids) in [
                        ("span2-forward", "span2-reversed", &["A", "B"][..]),
                        (
                            "span2-before-span3",
                            "span3-before-span2",
                            &["A", "B", "C"][..],
                        ),
                    ] {
                        let left = actual_cases
                            .get(&(
                                fit_position,
                                tail.clone(),
                                cap.to_owned(),
                                align.to_owned(),
                                left_topology.to_owned(),
                            ))
                            .expect("forward scene was checked");
                        let right = actual_cases
                            .get(&(
                                fit_position,
                                tail.clone(),
                                cap.to_owned(),
                                align.to_owned(),
                                right_topology.to_owned(),
                            ))
                            .expect("reversed scene was checked");
                        for id in ids {
                            assert_frame(
                                left[*id],
                                right[*id],
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
}
