use std::sync::Arc;

use spinon_core::{HostNodeKind, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
};
use spinon_style::{ComputedAspectRatioSizingBox, ComputedLayoutAspectRatio, ComputedStyleProfile};

use super::cases::VIEWPORT;
use super::test_fixture::{
    NodeSpec, cascade, custom_fixture, direct_text, item, stylesheet, take_error,
};

pub(super) const CSS: &str = r#"
.scene {
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
.column-row-20 { grid-template-rows:20px; }
.column-row-30 { grid-template-rows:30px; }
.column-row-40 { grid-template-rows:40px; }
.column-row-45 { grid-template-rows:45px; }
.column-row-60 { grid-template-rows:60px; }
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
"#;

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    root_class: &'static str,
    item_box_sizing: &'static str,
    item_edges: bool,
    text_order: [&'static str; 2],
    ratio_item: Option<usize>,
    ratio_css: Option<&'static str>,
    column_case: bool,
    columns: [f32; 2],
    row: f32,
    frames: [(f32, f32, f32); 2],
}

const CASES: [Case; 23] = [
    Case {
        id: "row-ratio",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("2 / 1"),
        column_case: false,
        columns: [10.0, 40.0],
        row: 20.0,
        frames: [(0.0, 10.0, 20.0), (10.0, 40.0, 20.0)],
    },
    Case {
        id: "row-control",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: None,
        ratio_css: None,
        column_case: false,
        columns: [10.0, 40.0],
        row: 10.0,
        frames: [(0.0, 10.0, 10.0), (10.0, 40.0, 10.0)],
    },
    Case {
        id: "column-ratio",
        root_class: "scene column-row-40",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("2 / 1"),
        column_case: true,
        columns: [80.0, 10.0],
        row: 40.0,
        frames: [(0.0, 80.0, 40.0), (80.0, 10.0, 40.0)],
    },
    Case {
        id: "column-control",
        root_class: "scene column-row-40",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [40.0, 10.0],
        row: 40.0,
        frames: [(0.0, 40.0, 40.0), (40.0, 10.0, 40.0)],
    },
    Case {
        id: "row-range-1-2",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("1 / 2"),
        column_case: false,
        columns: [10.0, 40.0],
        row: 80.0,
        frames: [(0.0, 10.0, 80.0), (10.0, 40.0, 80.0)],
    },
    Case {
        id: "row-range-4-3",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("4 / 3"),
        column_case: false,
        columns: [10.0, 40.0],
        row: 30.0,
        frames: [(0.0, 10.0, 30.0), (10.0, 40.0, 30.0)],
    },
    Case {
        id: "row-range-3-2",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("3 / 2"),
        column_case: false,
        columns: [10.0, 40.0],
        row: 26.65625,
        frames: [(0.0, 10.0, 26.65625), (10.0, 40.0, 26.65625)],
    },
    Case {
        id: "row-range-5-2",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("5 / 2"),
        column_case: false,
        columns: [10.0, 40.0],
        row: 16.0,
        frames: [(0.0, 10.0, 16.0), (10.0, 40.0, 16.0)],
    },
    Case {
        id: "column-range-1-2",
        root_class: "scene column-row-60",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("1 / 2"),
        column_case: true,
        columns: [40.0, 10.0],
        row: 60.0,
        frames: [(0.0, 40.0, 80.0), (40.0, 10.0, 60.0)],
    },
    Case {
        id: "column-range-4-3",
        root_class: "scene column-row-45",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("4 / 3"),
        column_case: true,
        columns: [60.0, 10.0],
        row: 45.0,
        frames: [(0.0, 60.0, 45.0), (60.0, 10.0, 45.0)],
    },
    Case {
        id: "column-range-3-2",
        root_class: "scene column-row-30",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("3 / 2"),
        column_case: true,
        columns: [45.0, 10.0],
        row: 30.0,
        frames: [(0.0, 45.0, 30.0), (45.0, 10.0, 30.0)],
    },
    Case {
        id: "column-range-5-2",
        root_class: "scene column-row-20",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("5 / 2"),
        column_case: true,
        columns: [50.0, 10.0],
        row: 20.0,
        frames: [(0.0, 50.0, 20.0), (50.0, 10.0, 20.0)],
    },
    Case {
        id: "column-control-60",
        root_class: "scene column-row-60",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [40.0, 10.0],
        row: 60.0,
        frames: [(0.0, 40.0, 60.0), (40.0, 10.0, 60.0)],
    },
    Case {
        id: "column-control-45",
        root_class: "scene column-row-45",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [40.0, 10.0],
        row: 45.0,
        frames: [(0.0, 40.0, 45.0), (40.0, 10.0, 45.0)],
    },
    Case {
        id: "column-control-30",
        root_class: "scene column-row-30",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [40.0, 10.0],
        row: 30.0,
        frames: [(0.0, 40.0, 30.0), (40.0, 10.0, 30.0)],
    },
    Case {
        id: "column-control-20",
        root_class: "scene column-row-20",
        item_box_sizing: "border-box",
        item_edges: false,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [40.0, 10.0],
        row: 20.0,
        frames: [(0.0, 40.0, 20.0), (40.0, 10.0, 20.0)],
    },
    Case {
        id: "row-auto-ratio-border-box",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("auto 3 / 2"),
        column_case: false,
        columns: [20.0, 50.0],
        row: 32.65625,
        frames: [(0.0, 20.0, 32.65625), (20.0, 50.0, 32.65625)],
    },
    Case {
        id: "row-numeric-ratio-border-box",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("3 / 2"),
        column_case: false,
        columns: [20.0, 50.0],
        row: 33.328125,
        frames: [(0.0, 20.0, 33.328125), (20.0, 50.0, 33.328125)],
    },
    Case {
        id: "row-no-ratio-border-box",
        root_class: "scene",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["A", "AAAA"],
        ratio_item: None,
        ratio_css: None,
        column_case: false,
        columns: [20.0, 50.0],
        row: 16.0,
        frames: [(0.0, 20.0, 16.0), (20.0, 50.0, 16.0)],
    },
    Case {
        id: "row-auto-ratio-content-box",
        root_class: "scene",
        item_box_sizing: "content-box",
        item_edges: true,
        text_order: ["A", "AAAA"],
        ratio_item: Some(1),
        ratio_css: Some("auto 3 / 2"),
        column_case: false,
        columns: [20.0, 50.0],
        row: 32.65625,
        frames: [(0.0, 20.0, 32.65625), (20.0, 50.0, 32.65625)],
    },
    Case {
        id: "row-60-auto-ratio-border-box",
        root_class: "scene column-row-60",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("auto 3 / 2"),
        column_case: true,
        columns: [91.0, 20.0],
        row: 60.0,
        frames: [(0.0, 91.0, 60.0), (91.0, 20.0, 60.0)],
    },
    Case {
        id: "row-60-numeric-ratio-border-box",
        root_class: "scene column-row-60",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["AAAA", "A"],
        ratio_item: Some(0),
        ratio_css: Some("3 / 2"),
        column_case: true,
        columns: [90.0, 20.0],
        row: 60.0,
        frames: [(0.0, 90.0, 60.0), (90.0, 20.0, 60.0)],
    },
    Case {
        id: "row-60-no-ratio-border-box",
        root_class: "scene column-row-60",
        item_box_sizing: "border-box",
        item_edges: true,
        text_order: ["AAAA", "A"],
        ratio_item: None,
        ratio_css: None,
        column_case: true,
        columns: [50.0, 20.0],
        row: 60.0,
        frames: [(0.0, 50.0, 60.0), (50.0, 20.0, 60.0)],
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
        let fixture = custom_fixture(case.id, case.root_class, children, None);
        let mut css = format!(
            "{CSS}\n.item {{ box-sizing:{}; padding:{}; border:{}; }}",
            case.item_box_sizing,
            if case.item_edges { "2px 4px" } else { "0" },
            if case.item_edges { "1px solid" } else { "0" },
        );
        if let Some(ratio) = case.ratio_css {
            css.push_str(&format!("\n.ratio {{ aspect-ratio:{ratio}; }}"));
        }
        let computed = cascade(&fixture.view, &css);
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
                let ratio = case.ratio_css.unwrap();
                let ratio_value = ratio.strip_prefix("auto ").unwrap_or(ratio);
                let (numerator, denominator) = ratio_value.split_once('/').unwrap();
                let expected = numerator.trim().parse::<f32>().unwrap()
                    / denominator.trim().parse::<f32>().unwrap();
                let aspect_ratio = style
                    .layout_aspect_ratio
                    .expect("numeric fixture ratio is typed");
                assert_eq!(aspect_ratio.ratio, expected, "{}", case.id);
                assert_eq!(
                    aspect_ratio.sizing_box,
                    if ratio.starts_with("auto ") {
                        ComputedAspectRatioSizingBox::ContentBox
                    } else {
                        ComputedAspectRatioSizingBox::BoxSizing
                    },
                    "{}",
                    case.id
                );
                assert_eq!(style.properties["aspect-ratio"], ratio, "{}", case.id);
            } else {
                assert_eq!(style.layout_aspect_ratio, None, "{}", case.id);
                assert_eq!(style.properties["aspect-ratio"], "auto", "{}", case.id);
            }
        }

        let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id))
            .unwrap_or_else(|| panic!("{}: aspect-ratio candidate fell back", case.id));
        assert_eq!(input.paragraph_styles.len(), 2, "{}", case.id);
        assert_eq!(
            input.content_box_ratio_node,
            case.ratio_css
                .filter(|ratio| ratio.starts_with("auto "))
                .and_then(|_| case.ratio_item.map(|index| fixture.items[index].id())),
            "{}",
            case.id
        );

        let output = if let Some(node) = input.content_box_ratio_node {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_content_box_ratio_text(
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
        let expected_row_track = match case.id {
            "row-range-3-2" => "26.666666px".to_owned(),
            "row-auto-ratio-border-box" | "row-auto-ratio-content-box" => "32.666664px".to_owned(),
            "row-numeric-ratio-border-box" => "33.333332px".to_owned(),
            _ => format!("{}px", case.row),
        };
        assert_eq!(
            trace.used_rows.used_track_list, expected_row_track,
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
            let expected = case.columns[item_index];
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
        if case.column_case {
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
                let expected = if case.ratio_item == Some(item_index) {
                    case.row
                } else if case.item_edges {
                    16.0
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
fn aspect_ratio_profile_rejects_ratio_on_the_wrong_item_and_accepts_other_finite_ratios() {
    let wrong_item = profile_fixture("ratio", "A", "item", "AAAA");
    let computed = cascade(
        &wrong_item.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; }}"),
    );
    let error = take_error(
        super::super::prepare(&wrong_item.snapshot, wrong_item.root, &computed),
        "ratio on the short text item",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(wrong_item.items[0].id().get()));

    let two_ratios = profile_fixture("ratio", "A", "ratio", "AAAA");
    let computed = cascade(
        &two_ratios.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; }}"),
    );
    let error = take_error(
        super::super::prepare(&two_ratios.snapshot, two_ratios.root, &computed),
        "two ratio items in the direct-text profile",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");

    let finite_ratio = profile_fixture("item", "A", "ratio", "AAAA");
    let computed = cascade(
        &finite_ratio.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:3 / 2; }}"),
    );
    assert!(
        super::super::prepare(&finite_ratio.snapshot, finite_ratio.root, &computed)
            .unwrap()
            .is_some()
    );
}

#[test]
fn aspect_ratio_profile_preserves_auto_ratio_content_box_basis_for_exact_owner() {
    let fixture = profile_fixture("item", "A", "ratio", "AAAA");
    let css = format!("{CSS}\n.ratio {{ aspect-ratio:auto 2 / 1; }}");
    let computed =
        spinon_style::compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[stylesheet(&css)],
            VIEWPORT,
            StyleRevision::INITIAL,
        )
        .expect("C11.4 direct-text profile preserves auto <ratio>");
    let style = computed
        .elements
        .iter()
        .find(|style| style.node_id == fixture.items[1].id())
        .expect("ratio item has computed style");
    let ratio = style.layout_aspect_ratio.expect("typed auto ratio");
    assert_eq!(ratio.ratio, 2.0);
    assert_eq!(ratio.sizing_box, ComputedAspectRatioSizingBox::ContentBox);
    assert_eq!(style.properties["aspect-ratio"], "auto 2 / 1");

    let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
        .expect("supported direct-text auto ratio")
        .expect("auto ratio fixture candidate");
    assert_eq!(input.content_box_ratio_node, Some(fixture.items[1].id()));
    let ungated = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
        &fixture.snapshot,
        fixture.root,
        computed.clone(),
        VIEWPORT,
        &input.paragraph_styles,
        &input.engine,
    )
    .expect_err("content-box ratio must not leak through the shared profile wrapper");
    assert!(matches!(
        ungated,
        spinon_style_to_layout::StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "aspect-ratio",
            ..
        } if node == fixture.items[1].id()
    ));
    let wrong_owner = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_content_box_ratio_text(
        &fixture.snapshot,
        fixture.root,
        computed.clone(),
        VIEWPORT,
        &input.paragraph_styles,
        &input.engine,
        fixture.items[0].id(),
    )
    .expect_err("a different owner must not receive the content-box basis");
    assert!(matches!(
        wrong_owner,
        spinon_style_to_layout::StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "aspect-ratio",
            ..
        } if node == fixture.items[0].id()
    ));
    let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_content_box_ratio_text(
        &fixture.snapshot,
        fixture.root,
        computed,
        VIEWPORT,
        &input.paragraph_styles,
        &input.engine,
        fixture.items[1].id(),
    )
    .expect("the exact content-box owner is admitted");
    close(
        output.layout.frames[&fixture.root.id()].height,
        20.0,
        "auto ratio",
    );
}

#[test]
fn aspect_ratio_profile_rejects_degenerate_or_inconsistent_computed_ratio() {
    let degenerate = profile_fixture("item", "A", "ratio", "AAAA");
    let computed = cascade(
        &degenerate.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:0 / 1; }}"),
    );
    let degenerate_style = computed
        .elements
        .iter()
        .find(|style| style.node_id == degenerate.items[1].id())
        .unwrap();
    assert_eq!(
        degenerate_style.layout_aspect_ratio, None,
        "Stylo maps the degenerate ratio to the typed auto representation"
    );
    assert_eq!(
        degenerate_style.properties["aspect-ratio"], "0 / 1",
        "numeric degenerate serialization must not pass as a no-ratio control"
    );
    let error = take_error(
        super::super::prepare(&degenerate.snapshot, degenerate.root, &computed),
        "degenerate numeric ratio serialized as no-ratio",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(degenerate.items[1].id().get()));

    let inconsistent = profile_fixture("item", "A", "ratio", "AAAA");
    let computed = cascade(
        &inconsistent.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:3 / 2; }}"),
    );
    let computed = replace_ratio_style(computed, inconsistent.items[1].id(), Some(1.5), "2 / 1");
    let error = take_error(
        super::super::prepare(&inconsistent.snapshot, inconsistent.root, &computed),
        "typed ratio differs from computed serialization",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(inconsistent.items[1].id().get()));
}

#[test]
fn aspect_ratio_profile_rejects_non_finite_and_unrepresentable_geometry_before_layout() {
    for ratio in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let fixture = profile_fixture("item", "A", "ratio", "AAAA");
        let computed = cascade(
            &fixture.view,
            &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; }}"),
        );
        let computed = replace_ratio_style(computed, fixture.items[1].id(), Some(ratio), "2 / 1");
        let error = take_error(
            super::super::prepare(&fixture.snapshot, fixture.root, &computed),
            "non-finite or non-positive typed ratio",
        );
        assert_eq!(error.code, "unsupported_c114_text_style");
    }

    for ratio in [f32::from_bits(1), f32::MAX] {
        let fixture = custom_fixture(
            "column-extreme",
            "scene column-row-60",
            vec![
                item("long", "item ratio", vec![NodeSpec::text("AAAA")]),
                item("short", "item", vec![NodeSpec::text("A")]),
            ],
            None,
        );
        let computed = cascade(
            &fixture.view,
            &format!("{CSS}\n.ratio {{ aspect-ratio:1 / 2; }}"),
        );
        let serialized = format!("{ratio} / 1");
        let computed =
            replace_ratio_style(computed, fixture.items[0].id(), Some(ratio), &serialized);
        let error = take_error(
            super::super::prepare(&fixture.snapshot, fixture.root, &computed),
            "ratio-derived geometry outside finite f32 range",
        );
        assert_eq!(error.code, "unsupported_c114_text_style");
        assert_eq!(error.node_id, Some(fixture.items[0].id().get()));
    }
}

fn replace_ratio_style(
    mut computed: spinon_style::ComputedStyleSnapshot,
    node: spinon_core::NodeId,
    ratio: Option<f32>,
    serialized: &str,
) -> spinon_style::ComputedStyleSnapshot {
    let mut elements = computed.elements.to_vec();
    let style = elements
        .iter_mut()
        .find(|style| style.node_id == node)
        .expect("synthetic ratio target style exists");
    style.layout_aspect_ratio = ratio.map(|ratio| ComputedLayoutAspectRatio {
        ratio,
        sizing_box: ComputedAspectRatioSizingBox::BoxSizing,
    });
    style
        .properties
        .insert("aspect-ratio".to_owned(), serialized.to_owned());
    computed.elements = Arc::from(elements);
    computed
}

#[test]
fn aspect_ratio_profile_rejects_nonzero_item_edges_and_nondefault_line_height() {
    for declaration in [
        "margin-left:1px",
        "padding-left:1px",
        "border-left:1px solid",
        "line-height:11px",
    ] {
        let fixture = profile_fixture("item", "A", "ratio", "AAAA");
        let css = format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; {declaration}; }}");
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

pub(super) fn profile_fixture(
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

pub(super) fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{label}: expected {expected}, got {actual}"
    );
}
