use super::*;

#[test]
fn implicit_named_equal_and_mixed_auto_placement_fail_closed() {
    let implicit = auto_sized_item("grid-column:4 / 5;grid-row:1 / 2");
    let named = auto_sized_item("grid-column:custom / 2;grid-row:1 / 2");
    let equal = auto_sized_item("grid-column:2 / 2;grid-row:1 / 2");
    let explicit = auto_sized_item("grid-column:2 / 3;grid-row:1 / 2");
    let invalid = [
        (
            Fixture::new(grid_style(), &[&implicit]).compute(),
            "implicit track",
        ),
        (
            Fixture::new(grid_style(), &[&named]).compute(),
            "named line",
        ),
        (
            Fixture::new(grid_style(), &[&equal]).compute(),
            "equal lines",
        ),
        (
            Fixture::new(
                grid_style(),
                &[
                    &explicit,
                    "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
                ],
            )
            .compute(),
            "mixed auto placement",
        ),
    ];
    for (result, label) in invalid {
        let error = result.unwrap_err();
        assert!(
            matches!(
                error,
                StyleLayoutError::Layout(spinon_layout::LayoutError::InvalidGridPlacement { .. })
                    | StyleLayoutError::UnsupportedComputedValue { .. }
            ),
            "{label} must fail with a contextual placement error: {error:?}"
        );
    }
}

#[test]
fn css_line_zero_is_rejected_before_taffy_can_treat_it_as_auto() {
    let fixture = Fixture::new(
        grid_style(),
        &[&auto_sized_item("grid-column:0 / 2;grid-row:1 / 2")],
    );
    let error = fixture.compute().unwrap_err();
    match error {
        StyleLayoutError::CascadeDiagnostic(diagnostic) => {
            assert_eq!(diagnostic.node_id, Some(fixture.items[0].id()));
            assert!(diagnostic.diagnostic.message.contains("grid-column"));
        }
        StyleLayoutError::Layout(spinon_layout::LayoutError::InvalidGridPlacement {
            property: "grid-column-start",
            ..
        }) => {}
        other => panic!("line zero must fail closed before Taffy Auto: {other:?}"),
    }
}

#[test]
fn invalid_second_item_does_not_publish_a_partial_layout() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            &auto_sized_item("grid-column:1 / 2;grid-row:1 / 2"),
            &auto_sized_item("grid-column:4 / 5;grid-row:1 / 2"),
        ],
    );
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::InvalidGridPlacement { .. }
        ))
    ));
}

#[test]
fn stylo_item_placement_values_reach_layout_without_string_reparsing() {
    let fixture = Fixture::new(
        grid_style(),
        &[&auto_sized_item("grid-column:1 / span 2;grid-row:1 / 3")],
    );
    let output = fixture.compute().unwrap();
    let style = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == fixture.items[0].id())
        .unwrap();
    let placement = style.grid_item_placement.as_ref().unwrap();
    assert_eq!(placement.column_start, ComputedCssGridPlacement::Line(1));
    assert_eq!(placement.column_end, ComputedCssGridPlacement::Span(2));
    assert_eq!(placement.row_start, ComputedCssGridPlacement::Line(1));
    assert_eq!(placement.row_end, ComputedCssGridPlacement::Line(3));
}
