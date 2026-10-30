use serde_json::Value;
use spinon_core::{HostNodeHandle, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
};
use spinon_style::ComputedCssGridPlacement;

use super::aspect_ratio::CSS;
use super::cases::VIEWPORT;
use super::test_fixture::{NodeSpec, cascade, custom_fixture, item, take_error};

const EXPECTED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/expected-values.json"
));

#[test]
fn direct_text_ratio_two_track_span_matches_independent_precomparison() {
    let expected: Value = serde_json::from_str(EXPECTED).expect("independent JSON");
    let cases = expected["cases"].as_array().expect("case list");
    assert_eq!(cases.len(), 9);

    for case in cases {
        let id = case["id"].as_str().expect("case id");
        let ratio = case["ratio"].as_str().expect("ratio");
        let constraints = case["constraints"].as_str().expect("constraints");
        let declarations = if constraints.starts_with("aspect-ratio:") {
            ""
        } else {
            constraints
        };
        let fixture = fixture("AAAA");
        let computed = cascade(
            &fixture.view,
            &format!(
                "{CSS}\n.ratio {{ grid-column:1 / span 2; aspect-ratio:{ratio}; {declarations} }}"
            ),
        );
        assert_eq!(computed.style_revision, StyleRevision::INITIAL, "{id}");

        let input = super::super::generic_direct_text::prepare_aspect_ratio_multispan(
            &fixture.snapshot,
            fixture.root,
            &computed,
        )
        .unwrap_or_else(|error| panic!("{id}: {error:?}"))
        .unwrap_or_else(|| panic!("{id}: span profile fell back"));
        assert_eq!(input.paragraph_styles.len(), 1, "{id}");
        assert!(
            input.paragraph_styles.contains_key(&fixture.items[0].id()),
            "{id}"
        );
        let item_style = computed
            .elements
            .iter()
            .find(|style| style.node_id == fixture.items[0].id())
            .expect("item computed style");
        let placement = item_style.grid_item_placement.as_ref().expect("placement");
        assert_eq!(
            placement.column_start,
            ComputedCssGridPlacement::Line(1),
            "{id}"
        );
        assert_eq!(
            placement.column_end,
            ComputedCssGridPlacement::Span(2),
            "{id}"
        );
        assert_eq!(placement.row_start, ComputedCssGridPlacement::Auto, "{id}");
        assert_eq!(placement.row_end, ComputedCssGridPlacement::Auto, "{id}");

        let output = if let Some(node) = input.aspect_ratio_minmax_node {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_aspect_ratio_minmax_text(
                &fixture.snapshot,
                fixture.root,
                computed,
                VIEWPORT,
                &input.paragraph_styles,
                &input.engine,
                node,
            )
        } else {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
                &fixture.snapshot,
                fixture.root,
                computed,
                VIEWPORT,
                &input.paragraph_styles,
                &input.engine,
            )
        }
        .unwrap_or_else(|error| panic!("{id}: {error:?}"));

        let root = output.layout.frames[&fixture.root.id()];
        let frame = output.layout.frames[&fixture.items[0].id()];
        close(
            root.width,
            expected["basis"]["root"]["width"].as_f64().unwrap() as f32,
            id,
        );
        close(root.height, case["rows"][0].as_f64().unwrap() as f32, id);
        close(frame.x, case["item"]["x"].as_f64().unwrap() as f32, id);
        close(frame.y, case["item"]["y"].as_f64().unwrap() as f32, id);
        close(
            frame.width,
            case["item"]["width"].as_f64().unwrap() as f32,
            id,
        );
        close(
            frame.height,
            case["item"]["height"].as_f64().unwrap() as f32,
            id,
        );

        let text_layout = &output.layout.text_layouts[&fixture.items[0].id()].layout;
        assert_eq!(text_layout.text, "AAAA", "{id}");
        close(text_layout.available_width, frame.width, id);
        assert_eq!(text_layout.lines.len(), 1, "{id}");
        close(
            text_layout.lines[0].advance,
            case["text"]["width"].as_f64().unwrap() as f32,
            id,
        );
        close(
            (text_layout.lines[0].advance - frame.width).max(0.0),
            case["text_inline_overflow"].as_f64().unwrap() as f32,
            id,
        );

        let intrinsic = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.items[0].id())
            .expect("direct text intrinsic input");
        assert!(
            matches!(
                &intrinsic.source,
                LayoutGridItemIntrinsicSource::Text { .. }
            ),
            "{id}"
        );
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("Grid sizing trace");
        let columns = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .expect("Columns pass");
        let rows = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .expect("Rows pass");
        let trace_expected = &case["track_sizing"];
        let expected_column_contributions = &trace_expected["columns"];
        assert_contribution_kind(
            columns,
            fixture.items[0],
            2,
            LayoutGridTrackSizingContributionKind::Minimum,
            expected_column_contributions["minimum_contribution"]
                .as_f64()
                .unwrap() as f32,
            1,
            id,
        );
        assert_contribution_kind(
            columns,
            fixture.items[0],
            2,
            LayoutGridTrackSizingContributionKind::MinContent,
            expected_column_contributions["min_content_contribution"]
                .as_f64()
                .unwrap() as f32,
            expected_column_contributions["min_content_trace_count"]
                .as_u64()
                .unwrap() as usize,
            id,
        );
        assert_contribution_kind(
            columns,
            fixture.items[0],
            2,
            LayoutGridTrackSizingContributionKind::MaxContent,
            expected_column_contributions["max_content_contribution"]
                .as_f64()
                .unwrap() as f32,
            expected_column_contributions["max_content_trace_count"]
                .as_u64()
                .unwrap() as usize,
            id,
        );
        let expected_row_contributions = &trace_expected["rows"];
        assert_contribution_kind(
            rows,
            fixture.items[0],
            1,
            LayoutGridTrackSizingContributionKind::Minimum,
            expected_row_contributions["minimum_contribution"]
                .as_f64()
                .unwrap() as f32,
            1,
            id,
        );
        assert_contribution_kind(
            rows,
            fixture.items[0],
            1,
            LayoutGridTrackSizingContributionKind::MinContent,
            expected_row_contributions["min_content_contribution"]
                .as_f64()
                .unwrap() as f32,
            expected_row_contributions["phase_count"].as_u64().unwrap() as usize,
            id,
        );
        assert_contribution_kind(
            rows,
            fixture.items[0],
            1,
            LayoutGridTrackSizingContributionKind::MaxContent,
            expected_row_contributions["max_content_contribution"]
                .as_f64()
                .unwrap() as f32,
            expected_row_contributions["phase_count"].as_u64().unwrap() as usize,
            id,
        );

        let expected_columns = case["columns"].as_array().expect("columns");
        let distribution = &expected_column_contributions["span_distribution"];
        let initialize = columns
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::Initialize)
            .expect("initial track state");
        assert_stage_tracks(
            initialize,
            &distribution["initial_track_base_sizes"],
            &distribution["initial_track_growth_limits"],
            id,
        );
        let span_batch = columns
            .stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.stage,
                    LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span: 2, .. }
                )
            })
            .expect("two-track contribution batch");
        assert_stage_tracks(
            span_batch,
            &distribution["after_span_batch_base_sizes"],
            &distribution["after_span_batch_growth_limits"],
            id,
        );
        let intrinsic_complete = columns
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::IntrinsicSizingComplete)
            .expect("intrinsic sizing completion");
        assert_stage_tracks(
            intrinsic_complete,
            &distribution["after_intrinsic_sizing_base_sizes"],
            &distribution["after_intrinsic_sizing_growth_limits"],
            id,
        );
        for (index, expected) in expected_columns.iter().enumerate() {
            let planned = distribution["per_track_planned_increases"][index]
                .as_f64()
                .unwrap() as f32;
            close(planned, expected.as_f64().unwrap() as f32, id);
        }
        close(
            trace
                .used_columns
                .used_track_list
                .split_whitespace()
                .next()
                .and_then(|value| value.strip_suffix("px"))
                .and_then(|value| value.parse::<f32>().ok())
                .unwrap_or(f32::NAN),
            expected_columns[0].as_f64().unwrap() as f32,
            id,
        );
        close(
            trace
                .used_columns
                .used_track_list
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.strip_suffix("px"))
                .and_then(|value| value.parse::<f32>().ok())
                .unwrap_or(f32::NAN),
            expected_columns[1].as_f64().unwrap() as f32,
            id,
        );
        assert_eq!(
            trace.used_rows.used_track_list,
            format!("{}px", frame.height),
            "{id}"
        );
    }
}

#[test]
fn ratio_span_keeps_a_stronger_minimum_when_maximum_is_smaller() {
    let fixture = fixture("AAAA");
    let computed = cascade(
        &fixture.view,
        &format!(
            "{CSS}\n.ratio {{ grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:60px; max-width:40px; }}"
        ),
    );
    let input = super::super::generic_direct_text::prepare_aspect_ratio_multispan(
        &fixture.snapshot,
        fixture.root,
        &computed,
    )
    .unwrap()
    .unwrap();
    let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_aspect_ratio_minmax_text(
        &fixture.snapshot,
        fixture.root,
        computed,
        VIEWPORT,
        &input.paragraph_styles,
        &input.engine,
        input.aspect_ratio_minmax_node.unwrap(),
    )
    .unwrap();
    let frame = output.layout.frames[&fixture.items[0].id()];
    close(frame.width, 60.0, "min-width wins over smaller max-width");
    close(frame.height, 30.0, "ratio follows the winning minimum");
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap();
    assert_eq!(trace.used_columns.used_track_list, "30px 30px");
}

#[test]
fn ratio_span_rejects_wrong_topology_content_and_unsupported_constraints() {
    let automatic = fixture("AAAA");
    let automatic_style = cascade(
        &automatic.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; min-width:45px; }}"),
    );
    assert!(
        super::super::generic_direct_text::prepare_aspect_ratio_multispan(
            &automatic.snapshot,
            automatic.root,
            &automatic_style,
        )
        .unwrap()
        .is_none(),
        "the span profile must leave automatic placement to the existing adapter",
    );

    for (label, item_style, root_override) in [
        (
            "span-one",
            "grid-column:1 / span 1; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "span-three",
            "grid-column:1 / span 3; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "wrong-start",
            "grid-column:2 / span 2; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "line-end",
            "grid-column:1 / 3; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "row-span",
            "grid-column:1 / span 2; grid-row:1 / span 2; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "named-line",
            "grid-column:missing / span 2; aspect-ratio:2 / 1; min-width:45px;",
            "",
        ),
        (
            "ratio-inverted",
            "grid-column:1 / span 2; aspect-ratio:1 / 2; min-width:45px;",
            "",
        ),
        (
            "auto-ratio",
            "grid-column:1 / span 2; aspect-ratio:auto 2 / 1; min-width:45px;",
            "",
        ),
        (
            "percent-min",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:10%;",
            "",
        ),
        (
            "min-over-60",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:60.5px;",
            "",
        ),
        (
            "fixed-second-track",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px;",
            ".scene { grid-template-columns:max-content 20px; }",
        ),
        (
            "nonzero-gap",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px;",
            ".scene { column-gap:1px; }",
        ),
        (
            "fixed-row",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px;",
            ".scene { grid-template-rows:20px; }",
        ),
        (
            "padding",
            "grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px; padding:1px;",
            "",
        ),
    ] {
        let fixture = fixture("AAAA");
        let computed = cascade(
            &fixture.view,
            &format!("{CSS}\n.ratio {{ {item_style} }}\n{root_override}"),
        );
        let error = take_error(
            super::super::generic_direct_text::prepare_aspect_ratio_multispan(
                &fixture.snapshot,
                fixture.root,
                &computed,
            ),
            label,
        );
        assert!(
            error.code == "unsupported_c114_text_style"
                || error.code == "unsupported_c114_text_grid_style",
            "{label}: unexpected error {error:?}"
        );
    }
}

#[test]
fn ratio_span_rejects_extra_items_and_non_direct_text() {
    let extra = fixture_with_children(vec![
        item("first", "item ratio", vec![NodeSpec::text("AAAA")]),
        item("second", "item", vec![NodeSpec::text("A")]),
    ]);
    let computed = cascade(
        &extra.view,
        &format!("{CSS}\n.ratio {{ grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px; }}"),
    );
    assert_eq!(
        take_error(
            super::super::generic_direct_text::prepare_aspect_ratio_multispan(
                &extra.snapshot,
                extra.root,
                &computed
            ),
            "extra item",
        )
        .code,
        "unsupported_c114_text_source"
    );

    let nested = fixture_with_children(vec![item(
        "ratio",
        "item ratio",
        vec![NodeSpec::Element {
            namespace: "http://www.w3.org/1999/xhtml",
            tag: "span",
            id: None,
            class: None,
            style: None,
            children: vec![NodeSpec::text("AAAA")],
        }],
    )]);
    let computed = cascade(
        &nested.view,
        &format!("{CSS}\n.ratio {{ grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px; }}"),
    );
    assert_eq!(
        take_error(
            super::super::generic_direct_text::prepare_aspect_ratio_multispan(
                &nested.snapshot,
                nested.root,
                &computed
            ),
            "nested text",
        )
        .code,
        "unsupported_c114_text_source"
    );
}

#[test]
fn empty_explicit_grid_item_does_not_enter_the_text_only_span_adapter() {
    let empty = fixture_with_children(vec![item("ratio", "item ratio", Vec::new())]);
    let computed = cascade(
        &empty.view,
        &format!("{CSS}\n.ratio {{ grid-column:1 / span 2; aspect-ratio:2 / 1; min-width:45px; }}"),
    );

    assert!(
        super::super::generic_direct_text::prepare_aspect_ratio_multispan(
            &empty.snapshot,
            empty.root,
            &computed,
        )
        .unwrap()
        .is_none()
    );
}

fn fixture(text: &str) -> super::test_fixture::Fixture {
    fixture_with_children(vec![item(
        "ratio",
        "item ratio",
        vec![NodeSpec::text(text)],
    )])
}

fn fixture_with_children(children: Vec<NodeSpec>) -> super::test_fixture::Fixture {
    custom_fixture("span-root", "scene", children, None)
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{label}: expected {expected}, got {actual}"
    );
}

fn assert_contribution_kind(
    pass: &LayoutGridTrackSizingPassTrace,
    item: HostNodeHandle,
    span: u16,
    kind: LayoutGridTrackSizingContributionKind,
    expected_value: f32,
    expected_count: usize,
    label: &str,
) {
    assert!(
        pass.contributions.iter().all(|entry| {
            entry.item == item.id() && entry.span == span && !entry.crosses_flexible_track
        }),
        "{label}: unexpected item/span contribution"
    );
    let contributions = pass
        .contributions
        .iter()
        .filter(|entry| entry.kind == kind)
        .collect::<Vec<_>>();
    assert_eq!(
        contributions.len(),
        expected_count,
        "{label}: {kind:?} count"
    );
    for contribution in contributions {
        close(contribution.value, expected_value, label);
    }
}

fn assert_stage_tracks(
    stage: &LayoutGridTrackSizingStageTrace,
    expected_base_sizes: &Value,
    expected_growth_limits: &Value,
    label: &str,
) {
    let expected_base_sizes = expected_base_sizes.as_array().expect("track base sizes");
    let expected_growth_limits = expected_growth_limits
        .as_array()
        .expect("track growth limits");
    assert_eq!(
        expected_base_sizes.len(),
        2,
        "{label}: expected track count"
    );
    assert_eq!(
        expected_growth_limits.len(),
        2,
        "{label}: expected growth-limit count"
    );
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), 2, "{label}: observed track count");
    for ((track, base), limit) in tracks
        .iter()
        .zip(expected_base_sizes)
        .zip(expected_growth_limits)
    {
        close(track.base_size, base.as_f64().unwrap() as f32, label);
        match limit.as_f64() {
            Some(expected) => close(
                track.growth_limit.unwrap_or(f32::NAN),
                expected as f32,
                label,
            ),
            None => assert_eq!(
                track.growth_limit, None,
                "{label}: expected unbounded growth limit"
            ),
        }
    }
}
