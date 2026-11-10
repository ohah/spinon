use super::*;
use std::collections::BTreeMap;

#[path = "automatic_minimum/rejections.rs"]
mod rejections;
#[path = "automatic_minimum/trace.rs"]
mod trace;

const OBSERVATIONS: &str = include_str!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson"
);

#[derive(Clone)]
struct SceneResult {
    rows: Vec<f32>,
    track_members: Vec<String>,
    item_frames: BTreeMap<String, [f32; 4]>,
}

fn records() -> Vec<Value> {
    OBSERVATIONS
        .lines()
        .map(|line| serde_json::from_str(line).expect("Chrome observation should be valid JSON"))
        .collect()
}

fn computed_tracks(scene: &Value) -> Vec<ComputedCssGridTrackSize> {
    scene["trackMembers"]
        .as_array()
        .expect("Chrome should record authored track members")
        .iter()
        .map(|member| {
            let member = member.as_str().expect("track member should be a string");
            match member {
                "min-content" => ComputedCssGridTrackSize::MinContent,
                "max-content" => ComputedCssGridTrackSize::MaxContent,
                "auto" => ComputedCssGridTrackSize::Auto,
                value if value.starts_with("fit-content(") && value.ends_with(')') => {
                    let value = &value[12..value.len() - 1];
                    let cap = if let Some(percent) = value.strip_suffix('%') {
                        ComputedCssGridTrackLengthPercentage::Percentage(
                            percent.parse::<f32>().expect("percentage cap should parse") / 100.0,
                        )
                    } else {
                        ComputedCssGridTrackLengthPercentage::LengthPx(
                            value
                                .strip_suffix("px")
                                .expect("length cap should use px")
                                .parse::<f32>()
                                .expect("px cap should parse"),
                        )
                    };
                    ComputedCssGridTrackSize::FitContent(cap)
                }
                other => panic!("unsupported Chrome track member {other:?}"),
            }
        })
        .collect()
}

fn rows(record: &Value) -> Vec<f32> {
    record["rowValues"]
        .as_array()
        .expect("Chrome should record used rows")
        .iter()
        .map(|value| value.as_f64().expect("row size should be numeric") as f32)
        .collect()
}

fn style_for_color<'a>(
    styles: &'a [spinon_style::ComputedElementStyle],
    color: OpaqueCssSrgb,
    label: &str,
) -> &'a spinon_style::ComputedElementStyle {
    let matches = styles
        .iter()
        .filter(|style| {
            style.background_color.is_some_and(|actual| {
                (actual.red, actual.green, actual.blue)
                    == (color.red(), color.green(), color.blue())
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "{label}: unique computed background");
    matches[0]
}

fn item_span_height(item: &Value, rows: &[f32], label: &str) -> f32 {
    let start = item["rowStart"]
        .as_str()
        .expect("item row start should be serialized")
        .parse::<usize>()
        .expect("item row start should be a positive line");
    let end = item["rowEnd"]
        .as_str()
        .expect("item row end should be serialized");
    let span = if let Some(span) = end.strip_prefix("span ") {
        span.parse::<usize>().expect("row span should parse")
    } else {
        end.parse::<usize>()
            .expect("row end should be a positive line")
            .checked_sub(start)
            .expect("row end should follow its start")
    };
    let first = start.checked_sub(1).expect("row lines start at one");
    let last = first.checked_add(span).expect("row span should be bounded");
    rows.get(first..last)
        .unwrap_or_else(|| panic!("{label}: row span {start} / {end} exceeds {rows:?}"))
        .iter()
        .sum()
}

fn observe(record: &Value, label: &str) -> SceneResult {
    let scene = &record["scene"];
    let (session, layout) = run(&source_for(record));
    let cascade_snapshot = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{label}: layout={:?}; cascade={:?}; diagnostics={:?}",
        layout.error,
        cascade_snapshot.error,
        layout.diagnostics
    );
    let completed = layout.completed.as_ref().expect("scene publishes layout");
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("scene publishes render output");
    let cascade = cascade_snapshot
        .completed
        .expect("same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key, "{label}: revision key");

    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("scene has one parsed Grid owner");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(scene)),
        "{label}: authored track order, type, and cap"
    );
    assert_eq!(
        grid.properties.get("align-content").map(String::as_str),
        scene["alignContent"].as_str(),
        "{label}: content alignment"
    );

    let minimum_mode = scene["minHeight"]
        .as_str()
        .expect("minimum mode should be recorded");
    let control = style_for_color(styles, EMPTY_ITEM, label);
    assert_eq!(
        control.layout_dimensions.min_height,
        spinon_style::ComputedCssDimension::LengthPx(0.0),
        "{label}: explicit-row control keeps min-height: 0"
    );

    let expected_rows = rows(record);
    assert_eq!(
        expected_rows.len(),
        4,
        "{label}: explicit plus three implicit rows"
    );
    assert_optional_track_trace(completed, record, label);

    let mut item_frames = BTreeMap::new();
    for item in record["items"]
        .as_array()
        .expect("Chrome should record rendered items")
    {
        let id = item["id"].as_str().expect("item id should be a string");
        let expected_frame = frame(&item["frame"]);
        let actual_frame = frame_for_paint(render, color_for_item(id));
        assert_frame(actual_frame, expected_frame, &format!("{label}: item {id}"));
        assert!(
            item_frames.insert(id.to_owned(), actual_frame).is_none(),
            "{label}: item {id} should be unique"
        );

        let item_style = style_for_color(styles, color_for_item(id), label);
        let expected_minimum = if id == "empty" || minimum_mode == "0px" {
            spinon_style::ComputedCssDimension::LengthPx(0.0)
        } else {
            spinon_style::ComputedCssDimension::Auto
        };
        assert_eq!(
            item_style.layout_dimensions.min_height, expected_minimum,
            "{label}: item {id} computed min-height"
        );

        let span_height = item_span_height(item, &expected_rows, &format!("{label}: item {id}"));
        assert!(
            (expected_frame[3] - span_height).abs() <= 0.01,
            "{label}: item {id} frame height {:?} should equal used span height {span_height:?}",
            expected_frame[3]
        );
        assert!(
            (actual_frame[3] - span_height).abs() <= 0.01,
            "{label}: runtime item {id} frame should equal the used row span"
        );

        if id != "empty" {
            let payload_frame = frame(&item["payloadFrame"]);
            let actual_payload = frame_for_paint(render, color_for_payload(id));
            assert_frame(
                actual_payload,
                payload_frame,
                &format!("{label}: payload {id}"),
            );
            let line_count = item["lineRects"]
                .as_array()
                .expect("Chrome should record text line rectangles")
                .len();
            assert!(line_count > 0, "{label}: text {id} should have a line");
            let payload_style = style_for_color(styles, color_for_payload(id), label);
            let runtime_text = render
                .texts()
                .iter()
                .find(|text| text.node_id() == payload_style.node_id)
                .unwrap_or_else(|| panic!("{label}: runtime text {id} should be present"));
            let runtime_line_count = runtime_text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            assert_eq!(
                runtime_line_count, line_count,
                "{label}: runtime text {id} line count"
            );
            assert!(
                (payload_frame[3] - line_count as f32 * 10.0).abs() <= 0.01,
                "{label}: payload {id} height should preserve its {line_count} text lines"
            );
            assert!(
                actual_frame[3] + 0.01 >= actual_payload[3],
                "{label}: item {id} should preserve the payload content floor"
            );
            assert!(
                actual_payload[1] + actual_payload[3] <= actual_frame[1] + actual_frame[3] + 0.01,
                "{label}: payload {id} should not overflow its item bottom"
            );
            assert!(
                item["contentOverflowBottom"]
                    .as_f64()
                    .unwrap_or(f64::INFINITY)
                    .abs()
                    <= 0.01,
                "{label}: Chrome reports no visible bottom overflow for {id}"
            );
        }
    }

    SceneResult {
        rows: expected_rows,
        track_members: scene["trackMembers"]
            .as_array()
            .expect("track members should be an array")
            .iter()
            .map(|member| member.as_str().unwrap().to_owned())
            .collect(),
        item_frames,
    }
}

fn topology_pair(scene: &Value) -> &'static str {
    match scene["topology"]
        .as_str()
        .expect("topology should be a string")
    {
        "span2-forward" | "span2-reversed" => "span2",
        "span2-before-span3" | "span3-before-span2" => "span2-span3",
        other => panic!("unsupported source-order topology {other:?}"),
    }
}

fn source_order_key(scene: &Value) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        scene["fitPosition"],
        scene["trackList"],
        scene["cap"],
        scene["alignContent"],
        scene["minHeight"],
        topology_pair(scene)
    )
}

fn alignment_key(scene: &Value) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        scene["fitPosition"],
        scene["trackList"],
        scene["cap"],
        scene["topology"],
        scene["minHeight"]
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

fn assert_semantic_frames_equal(
    actual: &BTreeMap<String, [f32; 4]>,
    expected: &BTreeMap<String, [f32; 4]>,
    label: &str,
) {
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );
    for (id, frame) in actual {
        assert_frame(
            *frame,
            expected[id],
            &format!("{label}: source-order item {id}"),
        );
    }
}

fn eligible_item_count(scene: &Value) -> usize {
    let tracks = scene["trackMembers"]
        .as_array()
        .expect("track members should be recorded");
    scene["items"]
        .as_array()
        .expect("item placement tuples should be recorded")
        .iter()
        .filter(|item| {
            let start = item[1].as_u64().expect("row start should be numeric") as usize;
            let span = item[2].as_u64().expect("row span should be numeric") as usize;
            tracks[start - 2..start - 2 + span].iter().any(|member| {
                let member = member.as_str().unwrap();
                member == "auto" || member.starts_with("fit-content(")
            })
        })
        .count()
}

#[test]
fn parsed_root_html_one_fit_row_span_auto_minimum_matches_all_2880_chrome_pairs() {
    let records = records();
    assert_eq!(records.len(), 5_760, "2,880 baseline/candidate pairs");
    let mut eligible_count = 0;
    let mut changed_item_frames = 0;
    let mut source_order_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();
    let mut alignment_groups = BTreeMap::<String, Vec<(String, SceneResult)>>::new();

    for (pair_index, pair) in records.chunks_exact(2).enumerate() {
        let baseline = &pair[0];
        let candidate = &pair[1];
        let baseline_scene = &baseline["scene"];
        let candidate_scene = &candidate["scene"];
        assert_eq!(baseline_scene["key"], candidate_scene["key"]);
        assert_eq!(baseline_scene["minHeight"].as_str(), Some("0px"));
        assert_eq!(candidate_scene["minHeight"].as_str(), Some("auto"));
        assert_eq!(
            baseline["items"].as_array().unwrap().len(),
            candidate["items"].as_array().unwrap().len()
        );

        eligible_count += eligible_item_count(candidate_scene);
        let base = observe(baseline, &format!("pair {pair_index} baseline"));
        let auto = observe(candidate, &format!("pair {pair_index} auto"));
        for (id, candidate_frame) in &auto.item_frames {
            if base.item_frames[id]
                .iter()
                .zip(candidate_frame)
                .any(|(before, after)| (before - after).abs() > 0.01)
            {
                changed_item_frames += 1;
            }
        }

        source_order_groups
            .entry(source_order_key(baseline_scene))
            .or_default()
            .push((
                baseline_scene["topology"].as_str().unwrap().to_owned(),
                base.clone(),
            ));
        source_order_groups
            .entry(source_order_key(candidate_scene))
            .or_default()
            .push((
                candidate_scene["topology"].as_str().unwrap().to_owned(),
                auto.clone(),
            ));
        alignment_groups
            .entry(alignment_key(baseline_scene))
            .or_default()
            .push((
                baseline_scene["alignContent"].as_str().unwrap().to_owned(),
                base,
            ));
        alignment_groups
            .entry(alignment_key(candidate_scene))
            .or_default()
            .push((
                candidate_scene["alignContent"].as_str().unwrap().to_owned(),
                auto,
            ));
    }

    assert_eq!(
        eligible_count, 6_048,
        "content-based automatic-minimum eligibility"
    );
    assert_eq!(
        changed_item_frames, 6_036,
        "automatic minimum changes item frames"
    );

    assert_eq!(source_order_groups.len(), 2_880);
    let mut source_order_scene_count = 0;
    for (key, mut scenes) in source_order_groups {
        scenes.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(scenes.len(), 2, "{key}: both source orders are captured");
        assert_ne!(scenes[0].0, scenes[1].0, "{key}: source orders differ");
        assert_rows_equal(&scenes[0].1.rows, &scenes[1].1.rows, &key);
        assert_semantic_frames_equal(&scenes[0].1.item_frames, &scenes[1].1.item_frames, &key);
        source_order_scene_count += scenes.len();
    }
    assert_eq!(source_order_scene_count, 5_760);

    assert_eq!(alignment_groups.len(), 2_880);
    for (key, scenes) in alignment_groups {
        assert_eq!(scenes.len(), 2, "{key}: start/stretch pair");
        let (start, stretch) = if scenes[0].0 == "start" {
            (&scenes[0].1, &scenes[1].1)
        } else {
            (&scenes[1].1, &scenes[0].1)
        };
        assert_eq!(scenes.iter().filter(|scene| scene.0 == "start").count(), 1);
        assert_eq!(
            scenes.iter().filter(|scene| scene.0 == "stretch").count(),
            1
        );
        assert_eq!(
            start.track_members, stretch.track_members,
            "{key}: same tracks"
        );
        for row_index in 1..start.rows.len() {
            let member = &start.track_members[row_index - 1];
            if member == "auto" {
                assert!(
                    stretch.rows[row_index] + 0.01 >= start.rows[row_index],
                    "{key}: align-content:stretch cannot shrink auto row {row_index}"
                );
            } else {
                assert!(
                    (stretch.rows[row_index] - start.rows[row_index]).abs() <= 0.01,
                    "{key}: stretch changed non-auto row {row_index} ({member})"
                );
            }
        }
    }
}
