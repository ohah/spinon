use super::*;

const OBSERVATIONS: &str = include_str!(
    "../../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson"
);

fn records_4_6() -> Vec<Value> {
    OBSERVATIONS
        .lines()
        .map(|line| serde_json::from_str(line).expect("six-row Chrome observation should be JSON"))
        .collect()
}

fn candidate_record() -> Value {
    records_4_6()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 4
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-six-forward"
        })
        .expect("six-row Chrome matrix should contain a position-four candidate")
}

#[test]
fn parsed_root_html_six_row_auto_minimum_preserves_forced_fit_cap_rounding() {
    let record = records_4_6()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 6
                && scene["tail"]
                    == serde_json::json!([
                        "auto",
                        "max-content",
                        "max-content",
                        "min-content",
                        "min-content"
                    ])
                && scene["cap"] == "0px"
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-span3-six-forward"
        })
        .expect("matrix contains the forced fit-cap auto-minimum scene");
    let _ = observe(&record, "forced fit-cap auto-minimum rounding");
}

#[test]
fn parsed_root_html_six_row_auto_minimum_preserves_unclamped_fit_minimum_rounding() {
    let record = records_4_6()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 6
                && scene["tail"]
                    == serde_json::json!([
                        "auto",
                        "max-content",
                        "max-content",
                        "min-content",
                        "min-content"
                    ])
                && scene["cap"] == "20px"
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-span3-six-forward"
        })
        .expect("matrix contains the twenty-pixel fit-cap auto-minimum scene");
    let _ = observe(&record, "unclamped fit-content minimum rounding");
}

#[test]
fn parsed_root_html_six_row_auto_minimum_trace_identifies_eligible_spans() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
    };

    let record = records_4_6()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 4
                && scene["cap"] == "0px"
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-six-forward"
        })
        .expect("matrix contains a position-four zero-cap span scene");
    let (session, layout) = run(&source_for(&record));
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Ready);
    let completed = layout
        .completed
        .as_ref()
        .expect("trace scene publishes layout");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same trace revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);
    assert_optional_track_trace(completed, &record, "six-row zero-cap span trace");

    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes track-sizing diagnostics");
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    assert_eq!(trace.used_rows.explicit_track_count, 1);
    assert_eq!(trace.used_rows.positive_implicit_track_count, 6);

    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row contribution trace exists");
    let styles = &cascade.roots[0].styles.elements;
    let grid_node = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists")
        .node_id;
    assert_eq!(trace.node, grid_node);

    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        if id == "empty" {
            continue;
        }
        let start = item["rowStart"].as_str().unwrap().parse::<usize>().unwrap();
        let span = item["rowEnd"]
            .as_str()
            .unwrap()
            .strip_prefix("span ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let tracks = record["scene"]["trackMembers"].as_array().unwrap();
        let spans_auto_min_track = tracks[start - 2..start - 2 + span].iter().any(|track| {
            let track = track.as_str().unwrap();
            track == "auto" || track.starts_with("fit-content(")
        });
        if !spans_auto_min_track {
            continue;
        }

        let style = style_for_color(styles, color_for_item(id), id);
        let minimums = row_pass
            .contributions
            .iter()
            .filter(|contribution| {
                contribution.item == style.node_id
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            })
            .collect::<Vec<_>>();
        assert_eq!(minimums.len(), 1, "{id}: one minimum contribution");
        let minimum = minimums[0];
        assert!(
            matches!(
                minimum.minimum_source,
                Some(MinimumSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: true,
                    spans_flexible_max_track: false,
                    content_based: true,
                    is_compressible_replaced: false,
                    ..
                })
            ),
            "{id}: eligible span should use a content-based automatic minimum: {minimum:?}"
        );
        let expected_minimum = item["lineRects"].as_array().unwrap().len() as f32 * 10.0;
        assert!(
            (minimum.value - expected_minimum).abs() <= 0.01,
            "{id}: minimum {:?}, expected {expected_minimum:?}",
            minimum.value
        );
    }
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(
        source.contains(from),
        "fixture fragment not found: {from:?}"
    );
    source.replacen(from, to, 1)
}

fn assert_rejected_atomically(name: &str, source: &str) {
    let (_session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail closed: {:?}",
        layout.error
    );
    assert!(layout.completed.is_none(), "{name} must not publish layout");
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| error.node_id.is_some() && error.property.is_some()),
        "{name} should identify the unsupported node and property"
    );
}

#[test]
fn parsed_root_html_six_row_one_fit_auto_minimum_matches_all_4320_chrome_pairs() {
    let records = records_4_6();
    assert_eq!(records.len(), 8_640, "4,320 baseline/candidate pairs");
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
            baseline_scene["fitPosition"],
            candidate_scene["fitPosition"]
        );
        assert_eq!(baseline_scene["topology"], candidate_scene["topology"]);

        eligible_count += eligible_item_count(candidate_scene);
        let base = observe(baseline, &format!("six-row pair {pair_index} baseline"));
        let auto = observe(candidate, &format!("six-row pair {pair_index} auto"));
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

    assert_eq!(eligible_count, 12_240, "automatic-minimum eligible items");
    assert_eq!(
        changed_item_frames, 15_348,
        "item frames changed by auto minimum"
    );

    assert_eq!(source_order_groups.len(), 4_320);
    let mut source_order_scene_count = 0;
    for (key, mut scenes) in source_order_groups {
        scenes.sort_by(|left, right| left.0.cmp(&right.0));
        assert_eq!(scenes.len(), 2, "{key}: both source orders are captured");
        assert_ne!(scenes[0].0, scenes[1].0, "{key}: source orders differ");
        assert_rows_equal(&scenes[0].1.rows, &scenes[1].1.rows, &key);
        assert_semantic_frames_equal(&scenes[0].1.item_frames, &scenes[1].1.item_frames, &key);
        source_order_scene_count += scenes.len();
    }
    assert_eq!(source_order_scene_count, 8_640);

    assert_eq!(alignment_groups.len(), 4_320);
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
                    "{key}: stretch cannot shrink auto row {row_index}"
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

#[test]
fn parsed_root_html_six_row_auto_minimum_rejects_mixed_minimum_bad_position_and_control() {
    let source = source_for(&candidate_record());
    let mixed_minimum = replace_once(
        &source,
        "data-item=\"B\" style=\"grid-row:3 / span 2\"",
        "data-item=\"B\" style=\"grid-row:3 / span 2; min-height:0\"",
    );
    assert_rejected_atomically("mixed auto|0 span item minimums", &mixed_minimum);

    let automatic_control = replace_once(
        &source,
        ".empty { min-height:0; height:",
        ".empty { min-height:auto; height:",
    );
    assert_rejected_atomically("explicit-row control min-height:auto", &automatic_control);

    let mut position_three = candidate_record();
    position_three["scene"]["fitPosition"] = Value::from(3);
    assert_rejected_atomically(
        "single-fit position three in the six-row topology",
        &source_for(&position_three),
    );
}

#[test]
fn parsed_root_html_six_row_auto_minimum_rejects_atomically_and_recovers_same_session() {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};
    use std::time::Duration;

    let source = source_for(&candidate_record());
    let source = replace_once(
        &source,
        "data-item=\"B\" style=\"grid-row:3 / span 2\"",
        "id=\"span-b\" data-item=\"B\" style=\"grid-row:3 / span 2\"",
    );
    let source = replace_once(
        &source,
        "</style>",
        "@media (prefers-color-scheme: dark) { #span-b { min-height:0; } }\n</style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "light scheme accepts six-row auto minimum: {:?}",
        initial.error
    );
    let initial_completed = initial
        .completed
        .as_ref()
        .expect("candidate publishes layout");
    let initial_key = initial_completed.key;
    let initial_frame = frame_for_paint(
        initial_completed
            .render_snapshot
            .as_ref()
            .expect("candidate publishes render output"),
        A_ITEM,
    );

    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = CssColorScheme::Dark;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("dark scheme should apply the unsupported mixed minimum");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(rejected.completed.is_none(), "rejected revision is atomic");

    media.color_scheme = CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("light scheme should restore the supported minimum profile");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    let recovered_completed = recovered.completed.expect("recovery publishes layout");
    assert_ne!(recovered_completed.key, initial_key);
    assert_frame(
        frame_for_paint(
            recovered_completed
                .render_snapshot
                .as_ref()
                .expect("recovery publishes render output"),
            A_ITEM,
        ),
        initial_frame,
        "recovered A item",
    );
}
