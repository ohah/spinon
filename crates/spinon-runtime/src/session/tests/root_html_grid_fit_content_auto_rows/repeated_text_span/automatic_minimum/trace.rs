use super::*;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn parsed_root_html_one_fit_zero_cap_preserves_overlapping_span_minimums() {
    let record = records()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 3
                && scene["cap"] == "0%"
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-before-span3"
                && scene["tail"]
                    == serde_json::json!([
                        "max-content",
                        "max-content",
                        "auto",
                        "min-content",
                        "min-content"
                    ])
        })
        .expect("matrix contains the zero-cap overlapping-span regression");
    observe(&record, "zero-cap overlapping span minimum");
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn parsed_root_html_one_fit_row_span_trace_keeps_both_span_auto_minimums_above_zero_cap() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
    };

    let record = records()
        .into_iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["minHeight"] == "auto"
                && scene["fitPosition"] == 1
                && scene["cap"] == "0%"
                && scene["alignContent"] == "start"
                && scene["topology"] == "span2-forward"
        })
        .expect("matrix contains a zero-cap forward span scene");
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
    assert_optional_track_trace(completed, &record, "zero-cap row-span trace");

    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("feature exposes track-sizing diagnostics");
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    assert_eq!(trace.node.get(), grid.node_id.get());
    assert_eq!(trace.used_rows.explicit_track_count, 1);
    assert_eq!(trace.used_rows.positive_implicit_track_count, 3);

    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row contributions are traced");
    let stage = row_pass.stages.last().expect("final row state is traced");
    let final_rows = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(final_rows.len(), 4);

    for (id, color) in [("A", A_ITEM), ("B", B_ITEM)] {
        let style = style_for_color(&cascade.roots[0].styles.elements, color, id);
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
            "{id}: trace should identify the eligible spanning automatic minimum: {minimum:?}"
        );
        let expected_item = record["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["id"] == id)
            .expect("Chrome records the traced item");
        let expected_minimum = expected_item["lineRects"].as_array().unwrap().len() as f32 * 10.0;
        assert!(
            (minimum.value - expected_minimum).abs() <= 0.01,
            "{id}: automatic minimum {:?}, expected {expected_minimum:?}",
            minimum.value
        );
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
        let span_size = final_rows[start - 1..start - 1 + span].iter().sum::<f32>();
        assert!(
            span_size + 0.01 >= minimum.value,
            "{id}: final spanning rows {span_size:?} should preserve minimum {:?}",
            minimum.value
        );
    }
}
