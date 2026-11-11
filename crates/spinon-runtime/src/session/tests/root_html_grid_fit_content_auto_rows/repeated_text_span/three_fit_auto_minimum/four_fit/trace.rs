use super::*;
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
};

#[test]
fn parsed_root_html_four_fit_zero_minimum_span_rounds_later_track_to_chrome() {
    let record = four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
                && record["scene"]["fitPositions"] == serde_json::json!([1, 3, 4, 5])
                && record["scene"]["tail"] == serde_json::json!(["min-content", "auto"])
        })
        .expect("matrix preserves the baseline fractional row-span case");
    let (session, layout) = run(&source_for(&record));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}; cascade={:?}",
        layout.error,
        layout.diagnostics,
        session.ua_cascade_snapshot().error
    );
    let render = layout
        .completed
        .as_ref()
        .unwrap()
        .render_snapshot
        .as_ref()
        .expect("baseline publishes render output");
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        assert_frame(
            frame_for_paint(render, color_for_item(id)),
            frame(&item["frame"]),
            &format!("baseline item {id}"),
        );
        if item["payloadFrame"].is_array() {
            assert_frame(
                frame_for_paint(render, color_for_payload(id)),
                frame(&item["payloadFrame"]),
                &format!("baseline payload {id}"),
            );
        }
    }
}

#[test]
fn parsed_root_html_four_fit_zero_minimum_span_prioritizes_fractional_auto_track() {
    let record = four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
                && record["scene"]["trackList"]
                    == "fit-content(0px) fit-content(10px) min-content auto fit-content(20%) fit-content(30px)"
                && record["scene"]["fitPositions"] == serde_json::json!([1, 2, 5, 6])
                && record["scene"]["tail"] == serde_json::json!(["min-content", "auto"])
        })
        .expect("matrix preserves the auto-track residual case");
    let (session, layout) = run(&source_for(&record));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}; cascade={:?}",
        layout.error,
        layout.diagnostics,
        session.ua_cascade_snapshot().error
    );
    let render = layout
        .completed
        .as_ref()
        .unwrap()
        .render_snapshot
        .as_ref()
        .expect("baseline publishes render output");
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        assert_frame(
            frame_for_paint(render, color_for_item(id)),
            frame(&item["frame"]),
            &format!("baseline item {id}"),
        );
        if item["payloadFrame"].is_array() {
            assert_frame(
                frame_for_paint(render, color_for_payload(id)),
                frame(&item["payloadFrame"]),
                &format!("baseline payload {id}"),
            );
        }
    }
}

#[test]
fn parsed_root_html_four_fit_zero_minimum_span_rounds_fractional_track_total_to_nearest() {
    let record = four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
                && record["scene"]["trackList"]
                    == "fit-content(0px) min-content fit-content(10px) auto fit-content(20%) fit-content(30px)"
                && record["scene"]["fitPositions"] == serde_json::json!([1, 3, 5, 6])
        })
        .expect("matrix includes a below-half fractional total with a spanning item");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_track_list(track_list));
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}; cascade={:?}",
        initial.error,
        initial.diagnostics,
        session.ua_cascade_snapshot().error
    );
    configure_scene(&session, &record);
    observe_current(&session, &record, "four-fit nearest fractional total");
}

#[test]
fn parsed_root_html_four_fit_stretch_preserves_fractional_auto_allocation() {
    let record = four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "stretch"
                && record["scene"]["topology"] == "span2Span3Six-forward"
                && record["scene"]["trackList"]
                    == "fit-content(0px) min-content fit-content(10px) auto fit-content(20%) fit-content(30px)"
                && record["scene"]["fitPositions"] == serde_json::json!([1, 3, 5, 6])
        })
        .expect("matrix includes a stretched auto row beside a fractional fit-content row");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, &record);
    observe_current(&session, &record, "four-fit stretch fractional allocation");
}

#[test]
fn parsed_root_html_four_fit_row_span_trace_matches_auto_minimum_contributions() {
    let record = four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "auto"
                && record["scene"]["trackList"]
                    == "fit-content(0px) fit-content(10px) fit-content(20%) auto fit-content(30px) min-content"
                && record["scene"]["topology"] == "span2Span3Six-forward"
                && record["scene"]["alignContent"] == "start"
        })
        .expect("matrix includes the failing automatic-minimum scene");
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
        "all contribution inputs belong to the trace revision"
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
        .expect("one parsed Grid owner exists");
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

    let mut eligible_count = 0;
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        if id == "empty" || !eligible_for_auto_minimum(&record, id) {
            continue;
        }
        eligible_count += 1;
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
            "{id}: trace records an eligible automatic minimum: {minimum:?}"
        );
        let expected_line_count = item["lineRects"].as_array().unwrap().len();
        let (_, expected_span) = scene_item_placement(&record, id, id);
        assert_eq!(
            minimum.span,
            u16::try_from(expected_span).expect("supported row span fits u16"),
            "{id}: source span is preserved"
        );
        assert!(!minimum.crosses_flexible_track, "{id}: no flexible track");
        assert!(
            (minimum.value - expected_line_count as f32 * 10.0).abs() <= 0.01,
            "{id}: automatic minimum {:?} matches payload floor {expected_line_count}",
            minimum.value
        );
        assert!(
            item_span_height(&record, item, &final_rows, id) + 0.01 >= minimum.value,
            "{id}: final spanned rows preserve the traced automatic minimum"
        );
    }
    assert!(
        eligible_count > 0,
        "candidate scene contains eligible items"
    );
}
