use super::*;
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
};

#[test]
fn parsed_root_html_six_fit_row_span_trace_keeps_percentage_cap_basis_and_owned_rows() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(50px)"
                && record["scene"]["minimum"] == "auto"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("matrix contains the typed percentage-cap trace case");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}",
        initial.error,
        initial.diagnostics
    );
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "six-fit trace percentage cap uses the definite 220px basis",
    );
    let layout = session.wait_for_layout_snapshot(std::time::Duration::from_secs(5));
    let completed = layout.completed.as_ref().expect("layout is published");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same Root HTML revision publishes computed styles");
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
    assert_eq!(
        grid.layout_dimensions.height,
        spinon_style::ComputedCssDimension::LengthPx(220.0),
        "the grid exposes a definite block-size percentage basis"
    );
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                0.0
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                10.0
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                0.2
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                30.0
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                40.0
            )),
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                50.0
            )),
        ]),
        "computed track list preserves all six caps and the typed 20% member"
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
    let final_rows = row_pass
        .stages
        .last()
        .expect("final row sizing stage exists")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    let expected_rows = row_values(record);
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
        if id == "empty" || !eligible_for_auto_minimum(record, id) {
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
        let placement = record["scene"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|placement| placement[0] == id)
            .expect("text item placement exists");
        let span = u16::try_from(placement[2].as_u64().unwrap()).unwrap();
        assert!(matches!(
            minimum.minimum_source,
            Some(MinimumSource::AutomaticMinimum {
                spans_auto_min_track: true,
                spans_multiple_tracks: true,
                spans_flexible_max_track: false,
                content_based: true,
                is_compressible_replaced: false,
                ..
            })
        ));
        assert_eq!(minimum.span, span, "{id}: trace records its row span");
        assert!(!minimum.crosses_flexible_track, "{id}: no flexible track");
        let expected_line_count = item["lineRects"].as_array().unwrap().len();
        assert!(
            (minimum.value - expected_line_count as f32 * 10.0).abs() <= 0.01,
            "{id}: automatic minimum {:?} matches payload floor",
            minimum.value
        );
    }
    assert_eq!(eligible_count, 5);
}

#[test]
fn parsed_root_html_six_fit_row_span_trace_resolves_percentage_cap_against_definite_basis() {
    let record = records()
        .iter()
        .find(|record| {
            record["scene"]["trackList"]
                == "fit-content(0px) fit-content(20%) fit-content(10px) fit-content(30px) fit-content(40px) fit-content(50px)"
                && record["scene"]["minimum"] == "zero"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("matrix has a baseline scene where the percentage cap binds");
    let (session, initial) = run(&source_for_track_list(
        record["scene"]["trackList"].as_str().unwrap(),
    ));
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; diagnostics={:?}",
        initial.error,
        initial.diagnostics
    );
    configure_scene(&session, record);
    observe_current(
        &session,
        record,
        "six-fit zero-minimum scene binds the 20% cap",
    );
    let completed = session
        .wait_for_layout_snapshot(std::time::Duration::from_secs(5))
        .completed
        .expect("layout is published");
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes track-sizing diagnostics");
    let trace = diagnostics
        .traces
        .first()
        .expect("one row-span Grid trace is present");
    let row_pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row sizing pass exists");
    let percentage_track_position = record["scene"]["trackMembers"]
        .as_array()
        .unwrap()
        .iter()
        .position(|member| member == "fit-content(20%)")
        .expect("authored list contains one 20% cap")
        + 1;
    let percentage_track = row_pass
        .stages
        .last()
        .expect("final track sizing stage exists")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .nth(percentage_track_position)
        .expect("percentage track state exists after the explicit row");
    assert_eq!(
        percentage_track.growth_limit,
        Some(220.0 * 0.2),
        "the computed 20% cap resolves to 44px from the definite 220px grid height"
    );
}
