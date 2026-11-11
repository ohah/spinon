use super::*;
use spinon_style::ComputedCssGridAutoRows;
use std::collections::BTreeSet;

#[test]
fn parsed_root_html_six_fit_auto_minimum_trace_preserves_all_fit_track_floors() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
        LayoutGridTrackSizingStageKind,
    };

    let scene = scenes()
        .into_iter()
        .find(|scene| is_automatic_minimum(scene) && scene["scene"]["index"] == 1)
        .expect("matrix has a forward auto-minimum scene");
    let (session, layout) = run(&source_for(&scene));
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
    assert_eq!(used_row_count, 9, "the trace contains nine used Grid rows");
    assert_eq!(
        trace.used_rows.used_track_list.split_whitespace().count(),
        9,
        "one explicit plus eight implicit rows"
    );

    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key);
    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Grid owner exists");
    assert_eq!(trace.node.get(), grid.node_id.get());
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(&scene))
    );
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

    let items = scene["items"].as_array().unwrap();
    let mut checked_eligible_rows = BTreeSet::new();
    for item in items.iter().skip(1) {
        let id = item_id(item);
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
        assert!(minimum.value > 0.0, "zero fit cap keeps item {id} floor");

        let row_internal_index = u16::try_from(id * 2 + 1).unwrap();
        let row = final_stage
            .tracks
            .iter()
            .find(|track| !track.is_gutter && track.internal_index == row_internal_index)
            .expect("eligible row has a final track state");
        assert!(row.base_size + 0.01 >= minimum.value);
        checked_eligible_rows.insert(id);
    }
    assert_eq!(
        checked_eligible_rows,
        BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8])
    );
}
