use spinon_core::StyleRevision;
use spinon_layout::LayoutGridTrackSizingAxis;

use super::aspect_ratio::{CSS, close, profile_fixture};
use super::cases::VIEWPORT;
use super::test_fixture::{cascade, take_error};

struct Case {
    id: &'static str,
    declarations: &'static str,
    ratio: &'static str,
    edges: bool,
    width: f32,
    height: f32,
}

const CASES: [Case; 19] = [
    Case {
        id: "numeric-baseline",
        declarations: "",
        ratio: "2 / 1",
        edges: false,
        width: 40.0,
        height: 20.0,
    },
    Case {
        id: "min-width-60",
        declarations: "min-width:60px",
        ratio: "2 / 1",
        edges: false,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "min-width-45",
        declarations: "min-width:45px",
        ratio: "2 / 1",
        edges: false,
        width: 45.0,
        height: 22.5,
    },
    Case {
        id: "max-width-30-min-width-0",
        declarations: "min-width:0;max-width:30px",
        ratio: "2 / 1",
        edges: false,
        width: 30.0,
        height: 15.0,
    },
    Case {
        id: "min-height-30",
        declarations: "min-height:30px",
        ratio: "2 / 1",
        edges: false,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "max-height-15-min-height-0",
        declarations: "min-height:0;max-height:15px",
        ratio: "2 / 1",
        edges: false,
        width: 40.0,
        height: 15.0,
    },
    Case {
        id: "max-height-15-min-height-0-min-width-0",
        declarations: "min-width:0;min-height:0;max-height:15px",
        ratio: "2 / 1",
        edges: false,
        width: 30.0,
        height: 15.0,
    },
    Case {
        id: "max-height-17-75-min-controls",
        declarations: "min-width:0;min-height:0;max-height:17.75px",
        ratio: "2 / 1",
        edges: false,
        width: 35.5,
        height: 17.75,
    },
    Case {
        id: "min-width-60-clamped-by-max-height-20",
        declarations: "min-width:60px;max-height:20px",
        ratio: "2 / 1",
        edges: false,
        width: 60.0,
        height: 20.0,
    },
    Case {
        id: "min-height-20-clamps-transferred-max-width-30",
        declarations: "min-width:0;min-height:20px;max-width:30px",
        ratio: "2 / 1",
        edges: false,
        width: 30.0,
        height: 20.0,
    },
    Case {
        id: "source-min-width-60-over-max-width-40",
        declarations: "min-width:60px;max-width:40px",
        ratio: "2 / 1",
        edges: false,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "source-min-height-30-over-max-height-20",
        declarations: "min-height:30px;max-height:20px",
        ratio: "2 / 1",
        edges: false,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "auto-ratio-min-width-60",
        declarations: "min-width:60px",
        ratio: "auto 2 / 1",
        edges: false,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "no-ratio-min-width-60",
        declarations: "min-width:60px",
        ratio: "auto",
        edges: false,
        width: 60.0,
        height: 10.0,
    },
    Case {
        id: "no-ratio-min-height-30",
        declarations: "min-height:30px",
        ratio: "auto",
        edges: false,
        width: 40.0,
        height: 30.0,
    },
    Case {
        id: "numeric-border-box-min-width-60",
        declarations: "min-width:60px",
        ratio: "2 / 1",
        edges: true,
        width: 60.0,
        height: 30.0,
    },
    Case {
        id: "auto-content-box-min-width-60",
        declarations: "min-width:60px",
        ratio: "auto 2 / 1",
        edges: true,
        width: 60.0,
        height: 31.0,
    },
    Case {
        id: "numeric-border-box-max-height-15-min-controls",
        declarations: "min-width:0;min-height:0;max-height:15px",
        ratio: "2 / 1",
        edges: true,
        width: 30.0,
        height: 15.0,
    },
    Case {
        id: "auto-content-box-max-height-15-min-controls",
        declarations: "min-width:0;min-height:0;max-height:15px",
        ratio: "auto 2 / 1",
        edges: true,
        width: 28.0,
        height: 15.0,
    },
];

#[test]
fn direct_text_aspect_ratio_minmax_cases_match_pinned_chrome_geometry() {
    for case in CASES {
        let fixture = profile_fixture("ratio case", "AAAA", "item", "A");
        let edge_style = if case.edges {
            "box-sizing:border-box;padding:2px 4px;border:1px solid;"
        } else {
            ""
        };
        let css = format!(
            "{CSS}\n.ratio {{{edge_style}aspect-ratio:{};{}}}",
            case.ratio, case.declarations
        );
        let computed = cascade(&fixture.view, &css);
        assert_eq!(
            computed.style_revision,
            StyleRevision::INITIAL,
            "{}",
            case.id
        );

        let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id))
            .unwrap_or_else(|| panic!("{}: direct-text fixture fell back", case.id));
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
        } else if let Some(node) = input.content_box_ratio_node {
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
        close(root.width, 100.0, case.id);
        close(root.height, case.height, case.id);
        let ratio_item = output.layout.frames[&fixture.items[0].id()];
        close(ratio_item.x, 0.0, case.id);
        close(ratio_item.width, case.width, case.id);
        close(ratio_item.height, case.height, case.id);

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: missing track trace", case.id));
        assert_eq!(
            trace.used_columns.used_track_list,
            format!("{}px 10px", case.width),
            "{}",
            case.id
        );
        assert_eq!(
            trace.used_rows.used_track_list,
            format!("{}px", case.height),
            "{}",
            case.id
        );
        let column_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap_or_else(|| panic!("{}: missing column sizing pass", case.id));
        assert_eq!(column_pass.contributions.len(), 6, "{}", case.id);
        let row_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .unwrap_or_else(|| panic!("{}: missing row sizing pass", case.id));
        assert_eq!(row_pass.contributions.len(), 6, "{}", case.id);
        for (index, contribution) in row_pass.contributions.iter().enumerate() {
            let item_index = index / 3;
            let kind = index % 3;
            let expected = if item_index == 0 {
                if kind == 0 {
                    expected_row_minimum(case.id)
                } else {
                    case.height
                }
            } else {
                10.0
            };
            close(contribution.value, expected, case.id);
        }
    }
}

fn expected_row_minimum(id: &str) -> f32 {
    match id {
        "numeric-baseline" => 20.0,
        "min-width-45" => 22.5,
        "min-width-60"
        | "min-height-30"
        | "source-min-height-30-over-max-height-20"
        | "no-ratio-min-height-30"
        | "numeric-border-box-min-width-60" => 30.0,
        "max-width-30-min-width-0" => 15.0,
        "min-width-60-clamped-by-max-height-20"
        | "min-height-20-clamps-transferred-max-width-30" => 20.0,
        "max-height-15-min-height-0"
        | "max-height-15-min-height-0-min-width-0"
        | "max-height-17-75-min-controls"
        | "numeric-border-box-max-height-15-min-controls" => 0.0,
        "source-min-width-60-over-max-width-40" | "auto-ratio-min-width-60" => 30.0,
        "no-ratio-min-width-60" => 10.0,
        "auto-content-box-min-width-60" => 31.0,
        "auto-content-box-max-height-15-min-controls" => 6.0,
        other => panic!("missing expected row minimum for {other}"),
    }
}

#[test]
fn ratio_minmax_profile_rejects_unbounded_values_and_wrong_owner_gate() {
    let fixture = profile_fixture("ratio", "AAAA", "item", "A");
    let computed = cascade(
        &fixture.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; min-width:10%; }}"),
    );
    let error = take_error(
        super::super::prepare(&fixture.snapshot, fixture.root, &computed),
        "percentage min-width outside the fixed fixture matrix",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");

    let fixture = profile_fixture("ratio", "AAAA", "item", "A");
    let computed = cascade(
        &fixture.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; max-width:60.5px; }}"),
    );
    let error = take_error(
        super::super::prepare(&fixture.snapshot, fixture.root, &computed),
        "fixed min/max values above the declared 60px limit",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");

    let fixture = profile_fixture("ratio case", "AAAA", "item", "A");
    let computed = cascade(
        &fixture.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:2 / 1; min-width:60px; }}"),
    );
    let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
        .expect("fixed ratio min/max fixture")
        .expect("profile candidate");
    let ratio_owner = input.aspect_ratio_minmax_node.expect("ratio min/max owner");
    let wrong_owner = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_aspect_ratio_minmax_text(
        &fixture.snapshot,
        fixture.root,
        computed,
        VIEWPORT,
        &input.paragraph_styles,
        &input.engine,
        fixture.items[1].id(),
    )
    .expect_err("a different direct item must not receive the min/max ratio gate");
    assert!(matches!(
        wrong_owner,
        spinon_style_to_layout::StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "aspect-ratio",
            ..
        } if node == fixture.items[1].id() && node != ratio_owner
    ));
}
