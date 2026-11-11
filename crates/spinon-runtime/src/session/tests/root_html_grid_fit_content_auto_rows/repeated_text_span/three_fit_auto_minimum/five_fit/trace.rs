use super::*;
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
};

#[test]
fn parsed_root_html_five_fit_auto_minimum_resolves_zero_growth_limit_after_spanning_minimum() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "fit-content(0px) fit-content(10px) fit-content(20%) auto fit-content(30px) fit-content(40px)"
                && record["scene"]["minimum"] == "auto"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
        })
        .expect("matrix scene exists");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit auto minimum resolves a zero-growth span limit to the base size",
    );
}

#[test]
fn parsed_root_html_five_fit_zero_minimum_rounds_fractional_fit_row_up() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "auto fit-content(0px) fit-content(10px) fit-content(30px) fit-content(40px) fit-content(20%)"
                && record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
        })
        .expect("matrix contains a fractional fit track after an integer auto tail");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit fractional fit row with integer auto tail",
    );
}

#[test]
fn parsed_root_html_five_fit_zero_minimum_max_content_tail_keeps_fractional_distribution() {
    for track_list in [
        "fit-content(0px) max-content fit-content(10px) fit-content(30px) fit-content(40px) fit-content(20%)",
        "fit-content(10px) max-content fit-content(0px) fit-content(30px) fit-content(40px) fit-content(20%)",
        "max-content fit-content(0px) fit-content(10px) fit-content(30px) fit-content(40px) fit-content(20%)",
    ] {
        let record = records()
            .iter()
            .find(|record| {
                record["scene"]["trackList"] == track_list
                    && record["scene"]["minimum"] == "zero"
                    && record["scene"]["alignContent"] == "start"
                    && record["scene"]["topology"] == "span2Span3Six-forward"
            })
            .expect("matrix includes each previously failing max-content tail track list");
        let (session, initial) = run(&source_for_track_list(track_list));
        assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
        configure_scene(&session, record);
        observe_current(
            &session,
            record,
            "five-fit zero-minimum max-content tail preserves Chrome fractional distribution",
        );
    }
}

#[test]
fn parsed_root_html_five_fit_zero_minimum_keeps_fractional_px_fit_row_nearest() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "auto fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px)"
                && record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
        })
        .expect("matrix contains the fractional px-cap rounding boundary");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit fractional px cap keeps nearest rounding",
    );
}

#[test]
fn parsed_root_html_five_fit_zero_minimum_allocates_remainder_by_fit_caps() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "fit-content(0px) fit-content(10px) auto fit-content(20%) fit-content(30px) fit-content(40px)"
                && record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
        })
        .expect("matrix contains tied fractional rows with different fit caps");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit fractional residual follows authored fit-content caps",
    );
}

#[test]
fn parsed_root_html_five_fit_auto_minimum_keeps_remainder_outside_row_span() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "auto fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px)"
                && record["scene"]["minimum"] == "auto"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Span3Six-forward"
        })
        .expect("matrix contains a row-span ending before the highest-cap fit track");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit automatic minimum keeps the final residual outside the spanning item",
    );
}

#[test]
fn parsed_root_html_five_fit_row_span_trace_preserves_owner_rows_and_auto_minimum() {
    let record = candidate_record();
    let source = super::super::source_for(&record);
    let (session, layout) = run(&source);
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
        .expect("fixture feature exposes track-sizing diagnostics");
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.revision == diagnostics.revision),
        "contribution inputs belong to the trace revision"
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
    let final_rows = row_pass
        .stages
        .last()
        .expect("final row sizing stage exists")
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
        assert_eq!(minimum.span, 2, "{id}: span-2 source contribution");
        assert!(!minimum.crosses_flexible_track, "{id}: no flexible track");
        assert!(
            (minimum.value - expected_line_count as f32 * 10.0).abs() <= 0.01,
            "{id}: minimum {:?} matches payload floor {expected_line_count}",
            minimum.value
        );
    }
    assert_eq!(eligible_count, 5);
}
