use super::*;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn parsed_root_html_three_fit_auto_placement_matches_all_5760_chrome_scenes() {
    let scenes = scenes();
    assert_eq!(scenes.len(), 5_760);
    let track_lists = scenes
        .iter()
        .map(|scene| scene["scene"]["trackList"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let fit_positions = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitPositions"]).unwrap())
        .collect::<BTreeSet<_>>();
    let tail_orders = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["tail"]).unwrap())
        .collect::<BTreeSet<_>>();
    let cap_assignments = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["scene"]["fitCaps"]).unwrap())
        .collect::<BTreeSet<_>>();
    let alignments = scenes
        .iter()
        .map(|scene| scene["scene"]["alignContent"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let dom_orders = scenes
        .iter()
        .map(|scene| scene["scene"]["domOrder"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(track_lists.len(), 720);
    assert_eq!(fit_positions.len(), 20);
    assert_eq!(tail_orders.len(), 6);
    assert_eq!(cap_assignments.len(), 6);
    assert_eq!(alignments, BTreeSet::from(["start", "stretch"]));
    assert_eq!(dom_orders, BTreeSet::from(["forward", "reverse"]));

    let mut zero_minimum_keys = BTreeSet::new();
    let mut auto_minimum_keys = BTreeSet::new();
    let mut zero_minimum_overflow_scenes = 0;
    let mut auto_minimum_overflow_scenes = 0;
    let mut zero_minimum_scene_count = 0;
    let mut auto_minimum_scene_count = 0;

    for (scene_index, expected_scene) in scenes.iter().enumerate() {
        let auto_minimum = is_automatic_minimum(expected_scene);
        let key = format!(
            "{}|{}|{}",
            expected_scene["scene"]["trackList"].as_str().unwrap(),
            expected_scene["scene"]["alignContent"].as_str().unwrap(),
            expected_scene["scene"]["domOrder"].as_str().unwrap()
        );
        if auto_minimum {
            auto_minimum_scene_count += 1;
            assert!(auto_minimum_keys.insert(key));
        } else {
            zero_minimum_scene_count += 1;
            assert!(zero_minimum_keys.insert(key));
        }

        let (session, layout) = super::super::run(&source_for(expected_scene));
        let cascade_snapshot = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index}: layout={:?}; cascade={:?}",
            layout.error,
            cascade_snapshot.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported scene publishes complete layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("supported scene publishes a render snapshot");
        let cascade = cascade_snapshot
            .completed
            .expect("same revision publishes computed styles");
        assert_eq!(completed.key, cascade.key, "scene {scene_index}: revision");
        let styles = &cascade.roots[0].styles.elements;
        let grid_style = styles
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("fixture has one Grid owner");
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(computed_tracks(expected_scene)),
            "scene {scene_index}: typed caps and authored member order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("grid-auto-rows")
                .map(String::as_str),
            Some(expected_scene["computedAutoRows"].as_str().unwrap()),
            "scene {scene_index}: serialized computed track list"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            Some(expected_scene["computedAlignContent"].as_str().unwrap()),
            "scene {scene_index}: computed content alignment"
        );
        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame should be published");
        super::super::two_fit_auto_placement::assert_frame(
            super::super::two_fit_auto_placement::runtime_frame(grid_frame),
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["items"]
            .as_array()
            .expect("Chrome records item frames");
        assert_eq!(expected_items.len(), 9);
        let expected_rows = expected_scene["rows"]
            .as_array()
            .expect("Chrome records used rows");
        assert_eq!(expected_rows.len(), 9);
        let track_members = expected_scene["scene"]["trackMembers"]
            .as_array()
            .expect("Chrome records the six authored track members");
        assert_eq!(track_members.len(), 6);
        let mut item_overflow = false;
        let mut actual_item_frames = BTreeMap::new();
        for (item_index, expected_item) in expected_items.iter().enumerate() {
            let id = if item_index == 0 {
                0
            } else {
                item_id(expected_item)
            };
            let color = super::super::two_fit_auto_placement::ITEM_COLORS[id];
            let item_styles = styles
                .iter()
                .filter(|style| {
                    style.background_color.is_some_and(|actual| {
                        (actual.red, actual.green, actual.blue)
                            == (color.red(), color.green(), color.blue())
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(
                item_styles.len(),
                1,
                "scene {scene_index} item {id}: unique computed style"
            );
            assert_eq!(
                item_styles[0]
                    .properties
                    .get("min-height")
                    .map(String::as_str),
                expected_item["minHeight"].as_str(),
                "scene {scene_index} item {id}: computed minimum"
            );
            let actual_item = super::super::two_fit_auto_placement::frame_for_background(
                completed,
                styles,
                color,
                &format!("scene {scene_index} item {id}"),
            );
            super::super::two_fit_auto_placement::assert_frame(
                actual_item,
                super::super::two_fit_auto_placement::frame(&expected_item["frame"]),
                &format!("scene {scene_index} item {id}"),
            );
            assert!(!auto_minimum || actual_item[1] + actual_item[3] <= 220.01);
            super::super::two_fit_auto_placement::assert_frame(
                [actual_item[3], 0.0, 0.0, 0.0],
                [
                    expected_rows[item_index].as_f64().unwrap() as f32,
                    0.0,
                    0.0,
                    0.0,
                ],
                &format!("scene {scene_index} row {item_index}"),
            );
            actual_item_frames.insert(id, actual_item);
            if id == 0 {
                continue;
            }
            let implicit_row = id - 1;
            let track_position = implicit_row % track_members.len();
            let track_member = track_members[track_position].as_str().unwrap();
            let automatic_minimum_track =
                track_member.starts_with("fit-content(") || track_member == "auto";
            if auto_minimum && automatic_minimum_track {
                let expected_payload =
                    super::super::two_fit_auto_placement::frame(&expected_item["payload"]);
                assert!(
                    actual_item[3] + 0.01 >= expected_payload[3],
                    "scene {scene_index} item {id}: automatic minimum must preserve the text floor"
                );
            }
        }

        let mut texts = render.texts().iter().collect::<Vec<_>>();
        texts.sort_unstable_by_key(|text| text.node_id().get());
        assert_eq!(
            texts.len(),
            8,
            "scene {scene_index}: eight payload text nodes"
        );
        for (text, expected_item) in texts.into_iter().zip(expected_items.iter().skip(1)) {
            let id = item_id(expected_item);
            let expected_payload =
                super::super::two_fit_auto_placement::frame(&expected_item["payload"]);
            let actual_payload = super::super::two_fit_auto_placement::frame_for_payload(
                completed,
                text.node_id().get(),
                &format!("scene {scene_index} payload {id}"),
            );
            super::super::two_fit_auto_placement::assert_frame(
                actual_payload,
                expected_payload,
                &format!("scene {scene_index} payload {id}"),
            );
            let actual_item = actual_item_frames[&id];
            let item_overflows = actual_payload[3] > actual_item[3] + 0.01;
            assert_eq!(
                item_overflows,
                expected_payload[3]
                    > super::super::two_fit_auto_placement::frame(&expected_item["frame"])[3]
                        + 0.01,
                "scene {scene_index} item {id}: visible payload overflow"
            );
            item_overflow |= item_overflows;
            let text_value = super::super::two_fit_auto_placement::PAYLOAD_TEXTS[id - 1];
            assert_eq!(
                text.glyphs().len(),
                text_value.bytes().filter(u8::is_ascii_uppercase).count(),
                "scene {scene_index} payload {id}: glyph count"
            );
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            assert_eq!(
                baselines.len(),
                expected_item["lineCount"].as_u64().unwrap() as usize,
                "scene {scene_index} payload {id}: line count"
            );
        }

        if auto_minimum {
            assert!(
                !item_overflow,
                "scene {scene_index}: automatic minimum prevents item overflow"
            );
            auto_minimum_overflow_scenes += usize::from(item_overflow);
        } else {
            assert!(
                item_overflow,
                "scene {scene_index}: zero minimum preserves baseline overflow"
            );
            zero_minimum_overflow_scenes += usize::from(item_overflow);
        }
    }

    assert_eq!(zero_minimum_scene_count, 2_880);
    assert_eq!(auto_minimum_scene_count, 2_880);
    assert_eq!(
        zero_minimum_keys, auto_minimum_keys,
        "all Chrome keys are paired"
    );
    assert_eq!(zero_minimum_overflow_scenes, 2_880);
    assert_eq!(auto_minimum_overflow_scenes, 0);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn parsed_root_html_three_fit_auto_minimum_trace_preserves_all_eligible_floors() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
        LayoutGridTrackSizingStageKind,
    };

    let scene = scenes()
        .into_iter()
        .find(|scene| {
            is_automatic_minimum(scene)
                && scene["scene"]["fitPositions"] == serde_json::json!([1, 2, 3])
                && scene["scene"]["alignContent"] == "start"
                && scene["scene"]["domOrder"] == "forward"
        })
        .expect("matrix includes the first three-fit auto-minimum scene");
    let (session, layout) = super::super::run(&source_for(&scene));
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Ready);
    let completed = layout.completed.as_ref().expect("scene publishes layout");
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes track-sizing diagnostics");
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    let used_row_count = usize::from(trace.used_rows.explicit_track_count)
        + usize::from(trace.used_rows.negative_implicit_track_count)
        + usize::from(trace.used_rows.positive_implicit_track_count);
    assert_eq!(used_row_count, 9, "one explicit plus eight implicit rows");
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0);
    assert_eq!(
        trace.used_rows.used_track_list.split_whitespace().count(),
        9
    );

    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);
    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(trace.node.get(), grid.node_id.get());
    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("final row-sizing pass is recorded");
    let final_stage = row_pass
        .stages
        .iter()
        .find(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        })
        .expect("final auto-stretch stage is recorded");
    let members = scene["scene"]["trackMembers"].as_array().unwrap();
    let items = scene["items"].as_array().unwrap();
    let mut checked_eligible_rows = BTreeSet::new();
    for item in items.iter().skip(1) {
        let id = item_id(item);
        let track_position = (id - 1) % members.len();
        let member = members[track_position].as_str().unwrap();
        if !member.starts_with("fit-content(") && member != "auto" {
            continue;
        }
        let color = super::super::two_fit_auto_placement::ITEM_COLORS[id];
        let style = styles
            .iter()
            .find(|style| {
                style.background_color.is_some_and(|actual| {
                    (actual.red, actual.green, actual.blue)
                        == (color.red(), color.green(), color.blue())
                })
            })
            .expect("eligible repeated item style exists");
        let minimums = row_pass
            .contributions
            .iter()
            .filter(|contribution| {
                contribution.item == style.node_id
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            })
            .collect::<Vec<_>>();
        assert_eq!(minimums.len(), 1, "item {id}: one minimum contribution");
        let minimum = minimums[0];
        assert!(
            matches!(
                minimum.minimum_source,
                Some(MinimumSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: false,
                    spans_flexible_max_track: false,
                    content_based: true,
                    is_compressible_replaced: false,
                    ..
                })
            ),
            "item {id}: {minimum:?}"
        );
        let expected_floor = item["lineCount"].as_u64().unwrap() as f32 * 10.0;
        assert!((minimum.value - expected_floor).abs() <= 0.01);
        assert!(minimum.value > 0.0, "zero fit cap must not clamp item {id}");

        let row_internal_index = u16::try_from(id * 2 + 1).unwrap();
        let row = final_stage
            .tracks
            .iter()
            .find(|track| !track.is_gutter && track.internal_index == row_internal_index)
            .expect("eligible row has a final track state");
        assert!(row.base_size + 0.01 >= minimum.value);
        checked_eligible_rows.insert(id);
    }
    assert_eq!(checked_eligible_rows, BTreeSet::from([1, 2, 3, 6, 7, 8]));
}
