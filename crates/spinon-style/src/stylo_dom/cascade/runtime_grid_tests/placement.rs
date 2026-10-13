use super::*;

#[test]
fn placement_profile_copies_numeric_negative_and_span_values_from_stylo_types() {
    let base = "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0";
    let child_styles = [
        format!("{base};grid-row:1 / 3;grid-column:1 / span 2"),
        format!("{base};grid-row:-2 / -1;grid-column:-2 / -1"),
    ];
    let borrowed = child_styles.iter().map(String::as_str).collect::<Vec<_>>();
    let (view, root, children) = placement_fixture(&borrowed);
    let snapshot = compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
        &view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let item = |node| {
        snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap()
    };

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
    );
    assert!(item(root).grid_layout.is_some());
    assert_eq!(
        item(children[0])
            .grid_item_placement
            .as_ref()
            .unwrap()
            .row_start,
        ComputedCssGridPlacement::Line(1)
    );
    assert_eq!(
        item(children[0])
            .grid_item_placement
            .as_ref()
            .unwrap()
            .row_end,
        ComputedCssGridPlacement::Line(3)
    );
    assert_eq!(
        item(children[0])
            .grid_item_placement
            .as_ref()
            .unwrap()
            .column_end,
        ComputedCssGridPlacement::Span(2)
    );
    assert_eq!(
        item(children[1])
            .grid_item_placement
            .as_ref()
            .unwrap()
            .column_start,
        ComputedCssGridPlacement::Line(-2)
    );
    assert_eq!(
        item(children[1])
            .grid_item_placement
            .as_ref()
            .unwrap()
            .row_end,
        ComputedCssGridPlacement::Line(-1)
    );
}

#[test]
fn placement_profile_applies_grid_area_then_longhand_cascade_precedence() {
    let child_styles =
        ["display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0"];
    let (view, _, children) = placement_fixture(&child_styles);
    let stylesheet = StylesheetSource {
        id: "grid-placement-cascade".to_owned(),
        base_url: "https://spinon.invalid/c11/placement.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#item-0 { grid-area: 1 / 1 / 3 / 3; grid-column-start: 2 }".to_owned(),
    };
    let snapshot = compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let placement = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == children[0])
        .unwrap()
        .grid_item_placement
        .as_ref()
        .unwrap();

    assert_eq!(placement.row_start, ComputedCssGridPlacement::Line(1));
    assert_eq!(placement.row_end, ComputedCssGridPlacement::Line(3));
    assert_eq!(placement.column_start, ComputedCssGridPlacement::Line(2));
    assert_eq!(placement.column_end, ComputedCssGridPlacement::Line(3));
}

#[test]
fn placement_profile_preserves_named_values_as_unsupported_typed_data() {
    let child_styles = [
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0;grid-column:custom-start / 2",
    ];
    let (view, _, children) = placement_fixture(&child_styles);
    let snapshot = compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
        &view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let placement = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == children[0])
        .unwrap()
        .grid_item_placement
        .as_ref()
        .unwrap();

    assert_eq!(
        placement.column_start,
        ComputedCssGridPlacement::Unsupported("custom-start".to_owned())
    );
}

#[test]
fn placement_profile_inline_allowlist_accepts_grid_shorthands_only() {
    let (view, _, _) = placement_fixture(&[
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0;grid-area:1 / 1 / 2 / 2",
    ]);
    assert_eq!(
        first_unsupported_runtime_block_flex_grid_placement_inline_property(&view),
        None
    );
    let (view, root) = {
        let fixture = fixture(
            "display:grid;box-sizing:border-box;width:250px;height:85px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;margin:0;padding:0;grid-template-areas:'x'",
        );
        (fixture.view, fixture.root)
    };
    assert_eq!(
        first_unsupported_runtime_block_flex_grid_placement_inline_property(&view),
        Some((root, "grid-template-areas".to_owned()))
    );
}

#[test]
fn template_profile_preserves_named_line_occurrences_and_both_span_token_orders() {
    let child_styles = [
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0",
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0",
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0",
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0",
    ];
    let (view, _, children) = placement_fixture(&child_styles);
    let stylesheet = StylesheetSource {
        id: "grid-named-line-typed-values".to_owned(),
        base_url: "https://spinon.invalid/c11/named-lines.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#item-0 { grid-column-start: middle 2; grid-column-end: finish } \
             #item-1 { grid-column-start: 2 middle; grid-column-end: finish } \
             #item-2 { grid-column-end: span 2 middle } \
             #item-3 { grid-column-end: span middle 2 }"
            .to_owned(),
    };
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let placement = |node| {
        snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap()
            .grid_item_placement
            .as_ref()
            .unwrap()
    };

    assert_eq!(
        placement(children[0]).column_start,
        ComputedCssGridPlacement::NamedLine {
            name: "middle".to_owned(),
            occurrence: 2,
        }
    );
    assert_eq!(
        placement(children[1]).column_start,
        ComputedCssGridPlacement::NamedLine {
            name: "middle".to_owned(),
            occurrence: 2,
        }
    );
    assert_eq!(
        placement(children[0]).column_end,
        ComputedCssGridPlacement::NamedLine {
            name: "finish".to_owned(),
            occurrence: 1,
        }
    );
    assert_eq!(
        placement(children[2]).column_end,
        ComputedCssGridPlacement::SpanNamed {
            name: "middle".to_owned(),
            count: 2,
        }
    );
    assert_eq!(
        placement(children[3]).column_end,
        ComputedCssGridPlacement::SpanNamed {
            name: "middle".to_owned(),
            count: 2,
        }
    );
}
