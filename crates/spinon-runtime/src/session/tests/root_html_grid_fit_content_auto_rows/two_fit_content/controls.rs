use super::*;
use std::collections::BTreeMap;

#[test]
fn parsed_root_html_two_fit_content_preserves_min_content_floor_above_both_caps() {
    let expected: Value = serde_json::from_str(MIN_CONTENT_FLOOR_CONTROL)
        .expect("Chrome min-content floor control is valid JSON");
    let scene = &expected["scene"];
    assert_eq!(scene["fitPositions"], serde_json::json!([1, 2]));
    assert_eq!(scene["fitCaps"], serde_json::json!(["0.5px", "0.5px"]));
    assert_eq!(scene["itemMinHeight"], "auto");

    let source = source_for(scene);
    let source = source.replacen("min-height: 0;", "min-height: auto;", 1);
    assert!(source.contains("min-height: auto;"));

    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "the min-content floor control should remain inside the bounded row-span profile: {:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    let completed = layout.completed.as_ref().expect("control publishes layout");
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("control publishes render output");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same Root HTML revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);

    let grid_style = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one parsed Grid owner exists");
    let expected_tracks = scene["trackMembers"]
        .as_array()
        .expect("track members are present")
        .iter()
        .map(|member| computed_track(member.as_str().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(
        grid_style.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(expected_tracks)
    );
    assert_eq!(
        grid_style
            .properties
            .get("align-content")
            .map(String::as_str),
        Some("start")
    );

    let grid_actual = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == grid_style.node_id.get())
        .map(|frame| [frame.x, frame.y, frame.width, frame.height])
        .expect("Grid frame is published");
    assert_frame(
        grid_actual,
        frame(&expected["gridFrame"]),
        "min-content floor control Grid",
    );

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    {
        let actual_rows = assert_track_trace(completed, &expected, "min-content floor control");
        assert!(actual_rows[1] > 0.5);
        assert!(actual_rows[2] > 0.5);
    }

    let mut frames_by_id = BTreeMap::new();
    let mut expected_texts = Vec::new();
    for expected_item in expected["items"]
        .as_array()
        .expect("Chrome items are present")
    {
        let id = expected_item["id"].as_str().unwrap();
        let actual_item = frame_for_computed_background(
            completed,
            &cascade.roots[0].styles.elements,
            item_color(id).0,
            &format!("min-content floor item {id}"),
        );
        assert_frame(
            actual_item,
            frame(&expected_item["frame"]),
            &format!("min-content floor item {id}"),
        );
        assert_frame(
            frame_for_paint(render, item_color(id).0),
            actual_item,
            &format!("min-content floor painted item {id}"),
        );
        frames_by_id.insert(id.to_owned(), actual_item);
        if id == "empty" {
            continue;
        }

        let payload_frame = frame_for_paint(render, payload_color(id).0);
        assert_frame(
            payload_frame,
            frame(&expected_item["payloadFrame"]),
            &format!("min-content floor payload {id}"),
        );
        let text = expected_item["text"].as_str().unwrap();
        expected_texts.push((
            text.bytes().filter(u8::is_ascii_uppercase).count(),
            payload_frame[1],
            expected_item["lineRects"].as_array().unwrap().len(),
        ));
    }

    assert!(
        frames_by_id["B"][1] - frames_by_id["A"][1] > 0.5,
        "the first used fit-content row keeps its minimum above the 0.5px cap"
    );
    assert!(
        frames_by_id["C"][1] - frames_by_id["B"][1] > 0.5,
        "the second used fit-content row keeps its minimum above the 0.5px cap"
    );

    let mut actual_texts = payload_text_descriptors(render);
    assert_eq!(actual_texts.len(), expected_texts.len());
    for (glyph_count, origin_y, line_count) in expected_texts {
        let matching = actual_texts
            .iter()
            .position(|(actual_glyphs, actual_y, actual_lines)| {
                *actual_glyphs == glyph_count
                    && (*actual_y - origin_y).abs() <= 0.01
                    && *actual_lines == line_count
            });
        assert!(
            matching.is_some(),
            "missing text payload {glyph_count}/{origin_y}/{line_count}: {actual_texts:?}"
        );
        actual_texts.remove(matching.unwrap());
    }
}
