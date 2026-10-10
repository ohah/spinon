use super::fixture::{
    FROZEN_ITEM, FROZEN_ITEM_50, FixtureNode, compare_reference_case, compute_case,
};

#[test]
fn flex_wrap_row_collects_three_lines_and_applies_row_and_column_gaps() {
    let children = [
        FixtureNode {
            id: "c101-lines-first",
            parent: None,
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-lines-second",
            parent: None,
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-lines-third",
            parent: None,
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-lines-fourth",
            parent: None,
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-lines-fifth",
            parent: None,
            style: "display:block;box-sizing:border-box;width:20px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (actual, nodes) = compute_case(
        "c101-lines-root",
        "display:flex;box-sizing:border-box;width:95px;height:34px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;row-gap:2px;column-gap:5px;margin:0;padding:0;border:0",
        &children,
    );
    compare_reference_case(&actual, &nodes, "row-three-lines-gap");
}

#[test]
fn runtime_three_lines_gap_fixture_matches_chromium_node_frames() {
    let children = [
        FixtureNode {
            id: "c101-runtime-flex",
            parent: None,
            style: "display:flex;box-sizing:border-box;width:170px;height:68px;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;row-gap:4px;column-gap:6px;margin:8px;padding:0;border:0;background-color:#243047",
        },
        FixtureNode {
            id: "c101-runtime-item-1",
            parent: Some("c101-runtime-flex"),
            style: "display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:#3366ff",
        },
        FixtureNode {
            id: "c101-runtime-item-2",
            parent: Some("c101-runtime-flex"),
            style: "display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:#12b981",
        },
        FixtureNode {
            id: "c101-runtime-item-3",
            parent: Some("c101-runtime-flex"),
            style: "display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:#f59e0b",
        },
        FixtureNode {
            id: "c101-runtime-item-4",
            parent: Some("c101-runtime-flex"),
            style: "display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:#ec4899",
        },
        FixtureNode {
            id: "c101-runtime-item-5",
            parent: Some("c101-runtime-flex"),
            style: "display:block;box-sizing:border-box;width:70px;height:20px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0;background-color:#a78bfa",
        },
    ];
    let (actual, nodes) = compute_case(
        "c101-runtime-root",
        "display:flex;box-sizing:border-box;width:320px;height:240px;flex-direction:column;flex-wrap:nowrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#101827",
        &children,
    );
    compare_reference_case(&actual, &nodes, "runtime-three-lines-gap");
}

#[test]
fn flex_wrap_column_uses_row_gap_on_main_axis_and_column_gap_between_columns() {
    let children = [
        FixtureNode {
            id: "c101-column-first",
            parent: None,
            style: "display:block;box-sizing:border-box;width:10px;height:18px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-column-second",
            parent: None,
            style: "display:block;box-sizing:border-box;width:10px;height:18px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-column-third",
            parent: None,
            style: "display:block;box-sizing:border-box;width:10px;height:18px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (actual, nodes) = compute_case(
        "c101-column-root",
        "display:flex;box-sizing:border-box;width:23px;height:40px;min-width:0;min-height:0;flex-direction:column;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;row-gap:4px;column-gap:3px;margin:0;padding:0;border:0",
        &children,
    );
    compare_reference_case(&actual, &nodes, "column-wrap-gap");
}

#[test]
fn shorthand_longhand_override_and_default_nowrap_match_chromium() {
    let exact_children = [
        FixtureNode {
            id: "c101-flow-first",
            parent: None,
            style: "display:block;box-sizing:border-box;width:50px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-flow-second",
            parent: None,
            style: "display:block;box-sizing:border-box;width:50px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (exact, exact_nodes) = compute_case(
        "c101-flow-root",
        "display:flex;box-sizing:border-box;width:105px;height:10px;min-width:0;min-height:0;flex-flow:column wrap;flex-direction:row;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;column-gap:5px;margin:0;padding:0;border:0",
        &exact_children,
    );
    compare_reference_case(&exact, &exact_nodes, "flow-shorthand-override");

    let default_children = [
        FixtureNode {
            id: "c101-default-first",
            parent: None,
            style: "display:block;box-sizing:border-box;width:60px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-default-second",
            parent: None,
            style: "display:block;box-sizing:border-box;width:60px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (default, default_nodes) = compute_case(
        "c101-default-root",
        "display:flex;box-sizing:border-box;width:100px;height:12px;min-width:0;min-height:0;flex-direction:row;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;margin:0;padding:0;border:0",
        &default_children,
    );
    compare_reference_case(&default, &default_nodes, "row-default-nowrap");
}

#[test]
fn exact_fit_and_one_css_pixel_over_match_the_chromium_line_break() {
    let children = [
        FixtureNode {
            id: "c101-exact-first",
            parent: None,
            style: FROZEN_ITEM_50,
        },
        FixtureNode {
            id: "c101-exact-second",
            parent: None,
            style: FROZEN_ITEM_50,
        },
    ];
    let (exact, exact_nodes) = compute_case(
        "c101-exact-root",
        "display:flex;box-sizing:border-box;width:105px;height:10px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;column-gap:5px;margin:0;padding:0;border:0",
        &children,
    );
    compare_reference_case(&exact, &exact_nodes, "row-exact-fit");

    let children = [
        FixtureNode {
            id: "c101-over-first",
            parent: None,
            style: FROZEN_ITEM_50,
        },
        FixtureNode {
            id: "c101-over-second",
            parent: None,
            style: FROZEN_ITEM_50,
        },
    ];
    let (over, over_nodes) = compute_case(
        "c101-over-root",
        "display:flex;box-sizing:border-box;width:104px;height:20px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;column-gap:5px;row-gap:0;margin:0;padding:0;border:0",
        &children,
    );
    compare_reference_case(&over, &over_nodes, "row-one-pixel-over");
}

#[test]
fn empty_single_and_display_none_items_do_not_create_extra_lines() {
    let (empty, empty_nodes) = compute_case(
        "c101-empty-root",
        "display:flex;box-sizing:border-box;width:100px;height:20px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;margin:0;padding:0;border:0",
        &[],
    );
    compare_reference_case(&empty, &empty_nodes, "empty-container");

    let (single, single_nodes) = compute_case(
        "c101-single-root",
        "display:flex;box-sizing:border-box;width:30px;height:10px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;margin:0;padding:0;border:0",
        &[FixtureNode {
            id: "c101-single-child",
            parent: None,
            style: "display:block;box-sizing:border-box;width:20px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        }],
    );
    compare_reference_case(&single, &single_nodes, "single-item");

    let hidden_children = [
        FixtureNode {
            id: "c101-hidden-first",
            parent: None,
            style: "display:block;box-sizing:border-box;width:45px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-hidden-item",
            parent: None,
            style: "display:none;box-sizing:border-box;width:80px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-hidden-last",
            parent: None,
            style: "display:block;box-sizing:border-box;width:45px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (hidden, hidden_nodes) = compute_case(
        "c101-hidden-root",
        "display:flex;box-sizing:border-box;width:100px;height:20px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;column-gap:10px;margin:0;padding:0;border:0",
        &hidden_children,
    );
    compare_reference_case(&hidden, &hidden_nodes, "display-none-child");
}

#[test]
fn block_declaration_is_computed_without_changing_block_flow_and_wrap_is_not_inherited() {
    let block_children = [
        FixtureNode {
            id: "c101-block-first",
            parent: None,
            style: "display:block;box-sizing:border-box;width:50px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-block-second",
            parent: None,
            style: "display:block;box-sizing:border-box;width:50px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0",
        },
    ];
    let (block, block_nodes) = compute_case(
        "c101-block-root",
        "display:block;box-sizing:border-box;width:100px;height:auto;min-width:0;min-height:0;flex-wrap:wrap;margin:0;padding:0;border:0",
        &block_children,
    );
    compare_reference_case(&block, &block_nodes, "block-wrap-noop");

    let nested_children = [
        FixtureNode {
            id: "c101-nested-inner",
            parent: None,
            style: "display:flex;box-sizing:border-box;width:60px;height:10px;min-width:0;min-height:0;flex-direction:row;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;margin:0;padding:0;border:0",
        },
        FixtureNode {
            id: "c101-nested-first",
            parent: Some("c101-nested-inner"),
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-nested-second",
            parent: Some("c101-nested-inner"),
            style: FROZEN_ITEM,
        },
        FixtureNode {
            id: "c101-nested-sibling",
            parent: None,
            style: "display:block;box-sizing:border-box;width:45px;height:10px;min-width:0;min-height:0;flex-grow:0;flex-shrink:0;flex-basis:auto;margin:0;padding:0;border:0",
        },
    ];
    let (nested, nested_nodes) = compute_case(
        "c101-nested-outer",
        "display:flex;box-sizing:border-box;width:100px;height:20px;min-width:0;min-height:0;flex-direction:row;flex-wrap:wrap;justify-content:flex-start;align-items:flex-start;flex-grow:0;flex-shrink:0;row-gap:0;column-gap:5px;margin:0;padding:0;border:0",
        &nested_children,
    );
    compare_reference_case(&nested, &nested_nodes, "nested-wrap-not-inherited");
}
