use spinon_core::{HostNodeKind, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
};
use spinon_style::ComputedStyleProfile;

use super::cases::VIEWPORT;
use super::test_fixture::{NodeSpec, cascade, custom_fixture, direct_text, item, take_error};

const CSS: &str = r#"
#row-ratio, #row-control, #column-ratio, #column-control {
    display:grid;
    box-sizing:border-box;
    width:100px;
    height:auto;
    grid-template-columns:max-content max-content;
    grid-template-rows:auto;
    grid-auto-flow:row;
    column-gap:0;
    row-gap:0;
    align-items:normal;
    font-family:Ahem;
    font-size:10px;
    font-weight:400;
    font-style:normal;
    line-height:10px;
}
#column-ratio, #column-control { grid-template-rows:40px; }
.item {
    display:block;
    position:static;
    width:auto;
    height:auto;
    min-width:auto;
    max-width:none;
    min-height:auto;
    max-height:none;
    margin:0;
    padding:0;
    font-family:inherit;
    font-size:inherit;
    font-weight:inherit;
    font-style:inherit;
    line-height:inherit;
    white-space-collapse:collapse;
    text-wrap-mode:wrap;
    word-break:normal;
    overflow-wrap:normal;
}
.ratio { aspect-ratio:2 / 1; }
"#;

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    text_order: [&'static str; 2],
    ratio_item: Option<usize>,
    columns: [f32; 2],
    row: f32,
    frames: [(f32, f32, f32); 2],
}

const CASES: [Case; 4] = [
    Case {
        id: "row-ratio",
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        columns: [10.0, 40.0],
        row: 20.0,
        frames: [(0.0, 10.0, 20.0), (10.0, 40.0, 20.0)],
    },
    Case {
        id: "row-control",
        text_order: ["A", "AAAA"],
        ratio_item: None,
        columns: [10.0, 40.0],
        row: 10.0,
        frames: [(0.0, 10.0, 10.0), (10.0, 40.0, 10.0)],
    },
    Case {
        id: "column-ratio",
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        columns: [80.0, 10.0],
        row: 40.0,
        frames: [(0.0, 80.0, 40.0), (80.0, 10.0, 40.0)],
    },
    Case {
        id: "column-control",
        text_order: ["AAAA", "A"],
        ratio_item: None,
        columns: [40.0, 10.0],
        row: 40.0,
        frames: [(0.0, 40.0, 40.0), (40.0, 10.0, 40.0)],
    },
];

#[test]
fn direct_text_preferred_aspect_ratio_changes_only_the_expected_intrinsic_axis() {
    for case in CASES {
        let children = case
            .text_order
            .iter()
            .enumerate()
            .map(|(index, text)| {
                let class = if case.ratio_item == Some(index) {
                    "item ratio"
                } else {
                    "item"
                };
                item(
                    if index == 0 { "first" } else { "second" },
                    class,
                    vec![NodeSpec::text(text)],
                )
            })
            .collect();
        let fixture = custom_fixture(case.id, "scene", children, None);
        let computed = cascade(&fixture.view, CSS);
        assert_eq!(
            computed.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        );
        assert_eq!(computed.style_revision, StyleRevision::INITIAL);
        for (index, item) in fixture.items.iter().enumerate() {
            let style = computed
                .elements
                .iter()
                .find(|style| style.node_id == item.id())
                .unwrap_or_else(|| panic!("{}: missing item style", case.id));
            if case.ratio_item == Some(index) {
                assert_eq!(style.layout_aspect_ratio, Some(2.0), "{}", case.id);
                assert_eq!(style.properties["aspect-ratio"], "2 / 1", "{}", case.id);
            } else {
                assert_eq!(style.layout_aspect_ratio, None, "{}", case.id);
            }
        }

        let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id))
            .unwrap_or_else(|| panic!("{}: aspect-ratio candidate fell back", case.id));
        assert_eq!(input.paragraph_styles.len(), 2, "{}", case.id);

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
            &fixture.snapshot,
            fixture.root,
            computed,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));

        let root = output.layout.frames[&fixture.root.id()];
        close(root.x, 0.0, case.id);
        close(root.y, 0.0, case.id);
        close(root.width, 100.0, case.id);
        close(root.height, case.row, case.id);
        assert_eq!(
            output.layout.grid_item_intrinsic_inputs.len(),
            2,
            "{}",
            case.id
        );
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: missing track sizing trace", case.id));
        assert_eq!(
            trace.used_columns.used_track_list,
            format!("{}px {}px", case.columns[0], case.columns[1]),
            "{}",
            case.id
        );
        assert_eq!(trace.used_columns.explicit_track_count, 2, "{}", case.id);
        assert_eq!(
            trace.used_rows.used_track_list,
            format!("{}px", case.row),
            "{}",
            case.id
        );
        assert_eq!(trace.used_rows.explicit_track_count, 1, "{}", case.id);
        assert_eq!(trace.passes.len(), 2, "{}", case.id);
        let columns = &trace.passes[0];
        assert_eq!(columns.sequence, 0, "{}", case.id);
        assert_eq!(
            columns.axis,
            LayoutGridTrackSizingAxis::Columns,
            "{}",
            case.id
        );
        assert_eq!(columns.contributions.len(), 6, "{}", case.id);
        for (index, contribution) in columns.contributions.iter().enumerate() {
            let item_index = index / 3;
            let kind = match index % 3 {
                0 | 2 => LayoutGridTrackSizingContributionKind::MaxContent,
                1 => LayoutGridTrackSizingContributionKind::MinContent,
                _ => unreachable!(),
            };
            let text = case.text_order[item_index];
            let ratio_transfers_to_columns = case.id == "column-ratio" && item_index == 0;
            let expected = if ratio_transfers_to_columns {
                80.0
            } else if text == "A" {
                10.0
            } else {
                40.0
            };
            assert_eq!(
                contribution.item,
                fixture.items[item_index].id(),
                "{}",
                case.id
            );
            assert_eq!(contribution.kind, kind, "{}", case.id);
            assert_eq!(contribution.span, 1, "{}", case.id);
            assert!(!contribution.crosses_flexible_track, "{}", case.id);
            assert_eq!(contribution.minimum_source, None, "{}", case.id);
            close(contribution.value, expected, case.id);
        }

        let rows = &trace.passes[1];
        assert_eq!(rows.sequence, 1, "{}", case.id);
        assert_eq!(rows.axis, LayoutGridTrackSizingAxis::Rows, "{}", case.id);
        if case.id.starts_with("column-") {
            assert!(rows.contributions.is_empty(), "{}", case.id);
        } else {
            assert_eq!(rows.contributions.len(), 6, "{}", case.id);
            for (index, contribution) in rows.contributions.iter().enumerate() {
                let item_index = index / 3;
                let kind = match index % 3 {
                    0 => LayoutGridTrackSizingContributionKind::Minimum,
                    1 => LayoutGridTrackSizingContributionKind::MinContent,
                    2 => LayoutGridTrackSizingContributionKind::MaxContent,
                    _ => unreachable!(),
                };
                let expected = if case.id == "row-ratio" && item_index == 1 {
                    20.0
                } else {
                    10.0
                };
                assert_eq!(
                    contribution.item,
                    fixture.items[item_index].id(),
                    "{}",
                    case.id
                );
                assert_eq!(contribution.kind, kind, "{}", case.id);
                assert_eq!(contribution.span, 1, "{}", case.id);
                assert!(!contribution.crosses_flexible_track, "{}", case.id);
                if kind == LayoutGridTrackSizingContributionKind::Minimum {
                    assert!(matches!(
                        contribution.minimum_source,
                        Some(
                            LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                                spans_auto_min_track: true,
                                spans_multiple_tracks: false,
                                spans_flexible_max_track: false,
                                content_based: true,
                                is_compressible_replaced: false,
                                ..
                            }
                        )
                    ));
                } else {
                    assert_eq!(contribution.minimum_source, None, "{}", case.id);
                }
                close(contribution.value, expected, case.id);
            }
        }

        for (index, (x, width, height)) in case.frames.iter().copied().enumerate() {
            let item = fixture.items[index];
            let frame = output.layout.frames[&item.id()];
            close(frame.x, x, case.id);
            close(frame.y, 0.0, case.id);
            close(frame.width, width, case.id);
            close(frame.height, height, case.id);

            let intrinsic = output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .find(|candidate| candidate.node == item.id())
                .unwrap_or_else(|| panic!("{}: missing item intrinsic input", case.id));
            assert_eq!(intrinsic.source_order, index, "{}", case.id);
            match &intrinsic.source {
                LayoutGridItemIntrinsicSource::Text { sizes } => {
                    let expected_width = if case.text_order[index] == "A" {
                        10.0
                    } else {
                        40.0
                    };
                    close(sizes.min_content(), expected_width, case.id);
                    close(sizes.max_content(), expected_width, case.id);
                    close(sizes.min_content_height(), 10.0, case.id);
                    close(sizes.max_content_height(), 10.0, case.id);
                }
                source => panic!("{}: unexpected intrinsic input {source:?}", case.id),
            }
            let text_node = direct_text(&fixture.snapshot, item);
            assert!(matches!(
                fixture.snapshot.node(text_node).map(|node| node.kind()),
                Some(HostNodeKind::Text(text)) if text.to_string_lossy() == case.text_order[index]
            ));
            close(
                output.layout.text_layouts[&item.id()].layout.height,
                10.0,
                case.id,
            );
        }
    }
}

#[test]
fn aspect_ratio_profile_rejects_ratio_on_the_wrong_item_and_wrong_value() {
    let wrong_item = profile_fixture("ratio", "A", "item", "AAAA");
    let computed = cascade(&wrong_item.view, CSS);
    let error = take_error(
        super::super::prepare(&wrong_item.snapshot, wrong_item.root, &computed),
        "ratio on the short text item",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(wrong_item.items[0].id().get()));

    let wrong_value = profile_fixture("item", "A", "ratio", "AAAA");
    let css = format!("{CSS}\n#row-ratio .ratio {{ aspect-ratio:3 / 2; }}");
    let computed = cascade(&wrong_value.view, &css);
    let error = take_error(
        super::super::prepare(&wrong_value.snapshot, wrong_value.root, &computed),
        "ratio other than 2 / 1",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(wrong_value.items[1].id().get()));
}

#[test]
fn aspect_ratio_profile_rejects_unsupported_item_edges_and_dimensions() {
    for declaration in [
        "min-width:1px",
        "max-width:1px",
        "min-height:1px",
        "max-height:1px",
        "margin-left:1px",
        "padding-left:1px",
        "border-left:1px solid",
        "line-height:11px",
    ] {
        let fixture = profile_fixture("item", "A", "ratio", "AAAA");
        let css = format!("{CSS}\n#row-ratio .ratio {{ {declaration}; }}");
        let computed = cascade(&fixture.view, &css);
        let error = take_error(
            super::super::prepare(&fixture.snapshot, fixture.root, &computed),
            declaration,
        );
        assert_eq!(error.code, "unsupported_c114_text_style", "{declaration}");
        assert_eq!(
            error.node_id,
            Some(fixture.items[1].id().get()),
            "{declaration}"
        );
    }
}

#[test]
fn aspect_ratio_profile_rejects_wrong_text_shape_and_extra_grid_item() {
    let wrong_text = profile_fixture("item", "A", "ratio", "AAA");
    let computed = cascade(&wrong_text.view, CSS);
    let error = take_error(
        super::super::prepare(&wrong_text.snapshot, wrong_text.root, &computed),
        "text other than A or AAAA",
    );
    assert_eq!(error.code, "unsupported_c114_text_source");

    let extra_item = custom_fixture(
        "row-ratio",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item("long", "item ratio", vec![NodeSpec::text("AAAA")]),
            item("extra", "item", vec![NodeSpec::text("A")]),
        ],
        None,
    );
    let computed = cascade(&extra_item.view, CSS);
    let error = take_error(
        super::super::prepare(&extra_item.snapshot, extra_item.root, &computed),
        "more than two direct items",
    );
    assert_eq!(error.code, "unsupported_c114_text_source");
}

fn profile_fixture(
    first_class: &str,
    first_text: &str,
    second_class: &str,
    second_text: &str,
) -> super::test_fixture::Fixture {
    custom_fixture(
        "row-ratio",
        "scene",
        vec![
            item(
                first_text,
                &format!("item {first_class}"),
                vec![NodeSpec::text(first_text)],
            ),
            item(
                second_text,
                &format!("item {second_class}"),
                vec![NodeSpec::text(second_text)],
            ),
        ],
        None,
    )
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{label}: expected {expected}, got {actual}"
    );
}
