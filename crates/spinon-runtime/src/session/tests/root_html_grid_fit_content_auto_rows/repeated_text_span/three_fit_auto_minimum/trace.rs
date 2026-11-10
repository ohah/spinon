use super::*;
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
};

#[test]
fn parsed_root_html_three_fit_row_span_trace_preserves_caps_owner_and_auto_minimums() {
    let record = records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "auto"
                && record["scene"]["fitPositions"] == serde_json::json!([1, 2, 3])
                && record["scene"]["fitCaps"] == serde_json::json!(["0px", "10px", "20%"])
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("matrix contains the ordered zero/ten/percentage cap scene");
    let (session, layout) = run(&source_for(&record));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}; cascade={:?}",
        layout.error,
        layout.diagnostics,
        session.ua_cascade_snapshot().error
    );
    let completed = layout.completed.as_ref().unwrap();
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);

    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("feature exposes track-sizing diagnostics");
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.revision == diagnostics.revision),
        "all intrinsic item inputs belong to the trace revision"
    );
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    assert_eq!(trace.used_rows.explicit_track_count, 1);
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0);
    assert_eq!(trace.used_rows.positive_implicit_track_count, 6);
    assert_eq!(
        trace.used_rows.used_track_list.split_whitespace().count(),
        7
    );

    let grid = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(trace.node, grid.node_id);
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(&record["scene"]))
    );
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.grid_container == grid.node_id)
    );

    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row contribution pass exists");
    let final_stage = row_pass
        .stages
        .last()
        .expect("final row sizing stage exists");
    let final_rows = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    let expected_rows = row_values(&record);
    assert_eq!(final_rows.len(), expected_rows.len());
    for (index, (actual, expected)) in final_rows.iter().zip(&expected_rows).enumerate() {
        assert!(
            (actual - expected).abs() <= 1.0 / 64.0,
            "row {index}: trace {actual:?} differs from Chrome {expected:?}"
        );
    }

    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        if id == "empty" || !eligible_for_auto_minimum(&record, id) {
            continue;
        }
        let style = style_for_color(&cascade.roots[0].styles.elements, color_for_item(id), id);
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
            "{id}: trace records eligible automatic minimum: {minimum:?}"
        );
        let expected_line_count = item["lineRects"].as_array().unwrap().len();
        assert_eq!(minimum.span, 2, "{id}: source row span is preserved");
        assert!(!minimum.crosses_flexible_track, "{id}: no flexible track");
        assert!(
            (minimum.value - expected_line_count as f32 * 10.0).abs() <= 0.01,
            "{id}: automatic minimum {:?} matches payload line floor {expected_line_count}",
            minimum.value
        );
        assert!(
            item_span_height(&record, item, &final_rows, id) + 0.01 >= minimum.value,
            "{id}: final spanned rows preserve the traced minimum"
        );
    }
}
