use super::super::*;
use super::source::{
    MIXED_LAYOUT_WIDTH_BASE, MIXED_MINIMUM_HEIGHT_BASE, item_color, topology_index,
};
use std::time::Duration;

pub(super) fn verify_scene(
    session: &super::super::super::RuntimeSession,
    record: &Value,
    sample_index: usize,
) -> usize {
    use spinon_style::CssMediaEnvironment;

    let scene = &record["scene"];
    let variant = topology_index(
        scene["rowProfile"].as_str().unwrap(),
        scene["topology"].as_str().unwrap(),
    );
    let alignment = match scene["alignContent"].as_str().unwrap() {
        "start" => 0,
        "stretch" => 1,
        other => panic!("unsupported one-fit mixed alignment {other:?}"),
    };
    let mask = usize::try_from(scene["minimumMask"].as_u64().unwrap())
        .expect("one-fit minimum mask fits usize");
    let width = MIXED_LAYOUT_WIDTH_BASE + variant * 2 + alignment;
    let height = MIXED_MINIMUM_HEIGHT_BASE + mask;
    let label = format!("one-fit mixed scene {sample_index}: {scene}");
    session
        .set_root_html_environment(
            width as f32,
            height as f32,
            1.0,
            CssMediaEnvironment::MOBILE,
        )
        .expect("one-fit mixed media environment selects a bounded scene");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(30));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{label}: layout={:?}",
        layout.error
    );
    let completed = layout
        .completed
        .expect("one-fit mixed scene publishes layout");
    assert_optional_track_trace(&completed, record, &label);
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("one-fit mixed scene publishes render output");
    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("same revision publishes computed styles");
    assert_eq!(completed.key, cascade.key, "{label}: revision key");
    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one Root HTML Grid owner exists");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(super::super::computed_tracks(scene)),
        "{label}: computed track order and cap"
    );
    assert_eq!(
        grid.properties.get("align-content").map(String::as_str),
        scene["alignContent"].as_str(),
        "{label}: computed alignment"
    );

    let mask_ids = scene["maskItemIds"].as_array().unwrap();
    let expected_auto_items = mask_ids
        .iter()
        .enumerate()
        .filter_map(|(bit, id)| (mask & (1 << bit) != 0).then(|| id.as_str().unwrap().to_owned()))
        .collect::<Vec<_>>();
    let actual_auto_items = scene["autoItems"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        actual_auto_items, expected_auto_items,
        "{label}: semantic mask IDs"
    );
    let auto_items = actual_auto_items
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let control = super::super::style_for_color(styles, EMPTY_ITEM, &label);
    assert_eq!(
        control.layout_dimensions.min_height,
        spinon_style::ComputedCssDimension::LengthPx(0.0),
        "{label}: explicit control stays at zero"
    );

    let expected_rows = super::super::rows(record);
    let mut eligible_auto_items = 0;
    let grid_frame = super::super::frame(&record["gridFrame"]);
    let row_pass = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes Grid sizing trace")
        .traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row sizing pass exists");
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        let color = if id == "empty" {
            EMPTY_ITEM
        } else {
            item_color(variant, id, false)
        };
        let item_style =
            super::super::style_for_color(styles, color, &format!("{label}: item {id}"));
        let expected_minimum = if auto_items.contains(id) {
            spinon_style::ComputedCssDimension::Auto
        } else {
            spinon_style::ComputedCssDimension::LengthPx(0.0)
        };
        assert_eq!(
            item_style.layout_dimensions.min_height, expected_minimum,
            "{label}: item {id} computed minimum"
        );
        let expected_frame = super::super::frame(&item["frame"]);
        let actual_frame = super::super::frame_for_paint(render, color);
        super::super::assert_frame(actual_frame, expected_frame, &format!("{label}: item {id}"));
        if id == "empty" {
            continue;
        }

        let payload_color = item_color(variant, id, true);
        let payload_style =
            super::super::style_for_color(styles, payload_color, &format!("{label}: payload {id}"));
        let expected_payload = super::super::frame(&item["payloadFrame"]);
        let actual_payload = super::super::frame_for_paint(render, payload_color);
        super::super::assert_frame(
            actual_payload,
            expected_payload,
            &format!("{label}: payload {id}"),
        );
        let span_height =
            super::super::item_span_height(item, &expected_rows, &format!("{label}: {id}"));
        assert!(
            (expected_frame[3] - span_height).abs() <= 0.01
                && (actual_frame[3] - span_height).abs() <= 0.01,
            "{label}: item {id} frame height matches its used row span"
        );
        let line_count = item["lineRects"].as_array().unwrap().len();
        let runtime_text = render
            .texts()
            .iter()
            .find(|text| text.node_id() == payload_style.node_id)
            .unwrap_or_else(|| panic!("{label}: runtime text {id} exists"));
        let runtime_line_count = runtime_text
            .glyphs()
            .iter()
            .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert_eq!(
            runtime_line_count, line_count,
            "{label}: text {id} line count"
        );
        assert!(
            actual_frame[3] + 0.01 >= actual_payload[3],
            "{label}: item {id} preserves text content floor"
        );
        let grid_bottom = grid_frame[1] + grid_frame[3];
        if expected_payload[1] + expected_payload[3] > grid_bottom + 0.01 {
            panic!("{label}: payload {id} overflows the Grid content edge");
        }

        if auto_items.contains(id) {
            let placement = scene["items"]
                .as_array()
                .unwrap()
                .iter()
                .find(|placement| placement[0] == id)
                .unwrap();
            let start = usize::try_from(placement[1].as_u64().unwrap()).unwrap();
            let span = usize::try_from(placement[2].as_u64().unwrap()).unwrap();
            let tracks = scene["trackMembers"].as_array().unwrap();
            let covered = tracks[start - 2..start - 2 + span]
                .iter()
                .map(|track| track.as_str().unwrap())
                .collect::<Vec<_>>();
            let eligible = covered
                .iter()
                .any(|track| *track == "auto" || track.starts_with("fit-content("));
            if eligible {
                use spinon_layout::{
                    LayoutGridTrackSizingContributionKind as ContributionKind,
                    LayoutGridTrackSizingMinimumContributionSource as MinimumSource,
                };
                eligible_auto_items += 1;
                let minimums = row_pass
                    .contributions
                    .iter()
                    .filter(|contribution| {
                        contribution.item == item_style.node_id
                            && contribution.kind == ContributionKind::Minimum
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    minimums.len(),
                    1,
                    "{label}: item {id} has one minimum trace"
                );
                assert!(
                    matches!(
                        minimums[0].minimum_source,
                        Some(MinimumSource::AutomaticMinimum {
                            spans_auto_min_track: true,
                            spans_multiple_tracks: true,
                            spans_flexible_max_track: false,
                            content_based: true,
                            is_compressible_replaced: false,
                            ..
                        })
                    ),
                    "{label}: item {id} trace is content-based automatic minimum: {:?}",
                    minimums[0]
                );
                assert!(
                    (minimums[0].value - line_count as f32 * 10.0).abs() <= 0.01,
                    "{label}: item {id} automatic minimum matches text lines"
                );
            }
        }
    }
    eligible_auto_items
}
