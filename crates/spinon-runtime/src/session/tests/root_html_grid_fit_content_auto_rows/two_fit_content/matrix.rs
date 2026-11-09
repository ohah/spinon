use super::*;

#[test]
fn parsed_root_html_two_fit_content_row_spans_match_all_3840_chrome_scenes() {
    let verification: Value =
        serde_json::from_str(VERIFICATION).expect("verification is valid JSON");
    let scenes = observations();
    assert_eq!(scenes.len(), 3840);
    assert_eq!(verification["sceneCount"], 3840);
    assert_eq!(verification["uniqueTrackListCount"], 480);
    assert_eq!(verification["trackPositionPairs"], 15);
    assert_eq!(verification["capPairsIncludingBothAssignments"], 8);
    assert_eq!(verification["tailOrders"], 4);
    assert_eq!(verification["independentChecks"]["sourceOrderPairs"], 1920);
    assert_eq!(verification["independentChecks"]["alignmentPairs"], 1920);

    let mut track_lists = BTreeSet::new();
    let mut fit_position_pairs = BTreeSet::new();
    let mut cap_assignments = BTreeSet::new();
    let mut tails = BTreeSet::new();
    let mut alignments = BTreeSet::new();
    let mut topologies = BTreeSet::new();
    let mut order_frames =
        BTreeMap::<(String, String, String), BTreeMap<String, FramesByItem>>::new();
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let mut alignment_rows = BTreeMap::<(String, String), BTreeMap<String, Vec<f32>>>::new();

    for (scene_index, expected) in scenes.iter().enumerate() {
        let scene = &expected["scene"];
        let track_list = scene["trackList"].as_str().unwrap();
        let members = scene["trackMembers"].as_array().unwrap();
        let positions = scene["fitPositions"].as_array().unwrap();
        let caps = scene["fitCaps"].as_array().unwrap();
        assert_eq!(members.len(), 6, "scene {scene_index}: six authored tracks");
        assert_eq!(positions.len(), 2, "scene {scene_index}: two fit positions");
        assert_eq!(caps.len(), 2, "scene {scene_index}: two cap assignments");
        let first = positions[0].as_u64().unwrap() as usize;
        let second = positions[1].as_u64().unwrap() as usize;
        assert!(first < second && (1..=6).contains(&first) && (1..=6).contains(&second));
        assert_eq!(
            members
                .iter()
                .filter(|member| member.as_str().unwrap().starts_with("fit-content("))
                .count(),
            2
        );
        assert_eq!(
            members
                .iter()
                .filter(|member| member.as_str() == Some("min-content"))
                .count(),
            2
        );
        assert_eq!(
            members
                .iter()
                .filter(|member| member.as_str() == Some("max-content"))
                .count(),
            1
        );
        assert_eq!(
            members
                .iter()
                .filter(|member| member.as_str() == Some("auto"))
                .count(),
            1
        );
        for (position, cap) in positions.iter().zip(caps) {
            let position = position.as_u64().unwrap() as usize;
            assert_eq!(
                members[position - 1].as_str(),
                Some(format!("fit-content({})", cap.as_str().unwrap()).as_str()),
                "scene {scene_index}: cap remains attached to authored fit position"
            );
        }

        track_lists.insert(track_list.to_owned());
        fit_position_pairs.insert((first, second));
        cap_assignments.insert(serde_json::to_string(caps).unwrap());
        tails.insert(serde_json::to_string(&scene["tail"]).unwrap());
        alignments.insert(scene["alignContent"].as_str().unwrap().to_owned());
        let topology = scene["topology"].as_str().unwrap();
        topologies.insert(topology.to_owned());
        let (topology_family, order) = topology
            .strip_suffix("-forward")
            .map(|family| (family, "forward"))
            .or_else(|| {
                topology
                    .strip_suffix("-reverse")
                    .map(|family| (family, "reverse"))
            })
            .expect("topology records source order");

        let (session, layout) = run(&source_for(scene));
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index} {track_list}/{topology}/{}: error={:?}; diagnostics={:?}; cascade={:?}",
            scene["alignContent"],
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
            .expect("same RuntimeSession revision publishes computed styles");
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
        let expected_tracks = members
            .iter()
            .map(|member| computed_track(member.as_str().unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "scene {scene_index}: typed cap values and authored order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            scene["alignContent"].as_str(),
            "scene {scene_index}: computed alignment"
        );

        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame publishes");
        let grid_actual = [
            grid_frame.x,
            grid_frame.y,
            grid_frame.width,
            grid_frame.height,
        ];
        let grid_expected = frame(&expected["gridFrame"]);
        assert_frame(
            grid_actual,
            grid_expected,
            &format!("scene {scene_index} Grid"),
        );
        let rows = expected["rowValues"]
            .as_array()
            .expect("rowValues is an array");
        assert_eq!(
            rows.len(),
            7,
            "scene {scene_index}: explicit plus six used rows"
        );
        assert!(
            rows.iter()
                .all(|row| row.as_f64().unwrap().is_finite() && row.as_f64().unwrap() >= 0.0)
        );

        let mut frames_by_id = FramesByItem::new();
        let mut expected_texts = Vec::new();
        for expected_item in expected["items"].as_array().unwrap() {
            let id = expected_item["id"].as_str().unwrap();
            let actual_item = frame_for_computed_background(
                completed,
                &cascade.roots[0].styles.elements,
                item_color(id).0,
                &format!("scene {scene_index} item {id}"),
            );
            let expected_item_frame = frame(&expected_item["frame"]);
            assert_frame(
                actual_item,
                expected_item_frame,
                &format!("scene {scene_index} item {id}"),
            );
            if actual_item[2] > 0.0 && actual_item[3] > 0.0 {
                assert_frame(
                    frame_for_paint(render, item_color(id).0),
                    actual_item,
                    &format!("scene {scene_index} painted item {id}"),
                );
            }
            frames_by_id.insert(id.to_owned(), actual_item);
            if id == "empty" {
                continue;
            }

            let payload_expected = frame(&expected_item["payloadFrame"]);
            let payload_actual = frame_for_paint(render, payload_color(id).0);
            assert_frame(
                payload_actual,
                payload_expected,
                &format!("scene {scene_index} payload {id}"),
            );
            let declarations = scene["items"].as_array().unwrap();
            let declaration = declarations
                .iter()
                .find(|entry| entry[0].as_str() == Some(id))
                .expect("item has a row-span declaration");
            let row_start = declaration[1].as_u64().unwrap() as usize;
            let span = declaration[2].as_u64().unwrap() as usize;
            let first_occupied_row = row_start - 1;
            let expected_top = grid_expected[1]
                + rows[..first_occupied_row]
                    .iter()
                    .map(|row| row.as_f64().unwrap() as f32)
                    .sum::<f32>();
            let expected_height = rows[first_occupied_row..first_occupied_row + span]
                .iter()
                .map(|row| row.as_f64().unwrap() as f32)
                .sum::<f32>();
            assert!(
                (actual_item[1] - expected_top).abs() <= 0.01,
                "scene {scene_index} item {id}: span top equals occupied row offsets"
            );
            assert!(
                (actual_item[3] - expected_height).abs() <= 0.01,
                "scene {scene_index} item {id}: span height={} equals used-row sum={expected_height}",
                actual_item[3]
            );

            let expected_text = expected_item["text"].as_str().unwrap();
            let glyph_count = expected_text.bytes().filter(u8::is_ascii_uppercase).count();
            let line_count = expected_item["lineRects"].as_array().unwrap().len();
            assert_eq!(
                payload_expected[3],
                line_count as f32 * 10.0,
                "scene {scene_index} payload {id}: line box height"
            );
            expected_texts.push((glyph_count, payload_expected[1], line_count));
        }

        let mut actual_texts = payload_text_descriptors(render);
        assert_eq!(
            actual_texts.len(),
            expected_texts.len(),
            "scene {scene_index}: text payload count"
        );
        for (glyph_count, origin_y, line_count) in expected_texts {
            let matching =
                actual_texts
                    .iter()
                    .position(|(actual_glyphs, actual_y, actual_lines)| {
                        *actual_glyphs == glyph_count
                            && (*actual_y - origin_y).abs() <= 0.01
                            && *actual_lines == line_count
                    });
            assert!(
                matching.is_some(),
                "scene {scene_index}: missing text descriptor {glyph_count}/{origin_y}/{line_count}; actual={actual_texts:?}"
            );
            actual_texts.remove(matching.unwrap());
        }

        order_frames
            .entry((
                track_list.to_owned(),
                scene["alignContent"].as_str().unwrap().to_owned(),
                topology_family.to_owned(),
            ))
            .or_default()
            .insert(order.to_owned(), frames_by_id);

        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        {
            let row_sizes =
                assert_track_trace(completed, expected, &format!("scene {scene_index}"));
            alignment_rows
                .entry((track_list.to_owned(), topology.to_owned()))
                .or_default()
                .insert(
                    scene["alignContent"].as_str().unwrap().to_owned(),
                    row_sizes,
                );
        }
    }

    assert_eq!(track_lists.len(), 480);
    assert_eq!(fit_position_pairs.len(), 15);
    assert_eq!(cap_assignments.len(), 8);
    assert_eq!(tails.len(), 4);
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
    for (key, orders) in order_frames {
        let forward = orders.get("forward").expect("source-order forward scene");
        let reverse = orders.get("reverse").expect("source-order reverse scene");
        assert_eq!(
            forward.keys().collect::<Vec<_>>(),
            reverse.keys().collect::<Vec<_>>(),
            "source-order items {key:?}"
        );
        for (id, forward_frame) in forward {
            assert_frame(
                *forward_frame,
                reverse[id],
                &format!("source-order invariant {key:?} item {id}"),
            );
        }
    }

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    for ((track_list, topology), alignments) in alignment_rows {
        let start = alignments.get("start").expect("start alignment scene");
        let stretch = alignments.get("stretch").expect("stretch alignment scene");
        let members = track_list.split_whitespace().collect::<Vec<_>>();
        let auto_row = members
            .iter()
            .position(|member| *member == "auto")
            .expect("each track list has one auto member")
            + 1;
        assert_eq!(start.len(), 7);
        assert_eq!(stretch.len(), 7);
        for row in 0..7 {
            if row == auto_row {
                assert!(
                    stretch[row] + 0.01 >= start[row],
                    "align-content stretch only grows auto row: {track_list}/{topology} row {row}"
                );
            } else {
                assert!(
                    (stretch[row] - start[row]).abs() <= 0.01,
                    "align-content stretch changes non-auto row: {track_list}/{topology} row {row}: {} vs {}",
                    start[row],
                    stretch[row]
                );
            }
        }
    }
}
