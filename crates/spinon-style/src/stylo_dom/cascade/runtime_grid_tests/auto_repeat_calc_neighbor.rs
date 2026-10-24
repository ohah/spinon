use super::*;

fn root_grid(style: &str) -> Fixture {
    fixture(&format!(
        "display:grid;box-sizing:border-box;width:240px;height:48px;min-width:0;min-height:0;margin:0;padding:0;{style}"
    ))
}

fn assert_calc_neighbor(
    value: &ComputedCssGridTrackSize,
    expected_length: f32,
    expected_percentage: f32,
) {
    let ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(terms)) = value else {
        panic!("expected typed length-percentage sum, got {value:?}");
    };
    assert_eq!(terms.len(), 2);
    assert!(terms.iter().any(
        |term| matches!(term, ComputedCssMath::LengthPx(actual) if *actual == expected_length)
    ));
    assert!(terms.iter().any(
        |term| matches!(term, ComputedCssMath::Percentage(actual) if *actual == expected_percentage)
    ));
}

#[test]
fn track_sizing_profile_preserves_typed_calc_neighbors_in_either_term_order() {
    let fixture = root_grid(
        "grid-template-columns:calc(10px + 5%) repeat(auto-fill,minmax(40px,1fr)) calc(2.5% + 4px);grid-template-rows:12px",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("supported calc auto-repeat neighbors: {error:?}"));
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();

    assert_eq!(grid.columns.len(), 3);
    assert_calc_neighbor(&grid.columns[0], 10.0, 0.05);
    assert_calc_neighbor(&grid.columns[2], 4.0, 0.025);
    assert_eq!(
        grid.columns_auto_repeat.as_ref().map(|repeat| repeat.start),
        Some(1)
    );

    let normalized_fixture = root_grid(
        "grid-template-columns:calc(10px + 5% + 1px) repeat(auto-fill,minmax(40px,1fr));grid-template-rows:12px",
    );
    let normalized = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &normalized_fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("canonical two-term calc neighbor: {error:?}"));
    let normalized_grid = normalized
        .elements
        .iter()
        .find(|style| style.node_id == normalized_fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();
    assert_calc_neighbor(&normalized_grid.columns[0], 11.0, 0.05);
}

#[test]
fn track_sizing_profile_accepts_a_subtracted_length_term() {
    let fixture = root_grid(
        "grid-template-columns:calc(5% - 2px) repeat(auto-fill,minmax(40px,1fr));grid-template-rows:12px",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("signed calc neighbor length term: {error:?}"));
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();
    let ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(terms)) = &grid.columns[0] else {
        panic!("expected typed calc neighbor, got {:?}", grid.columns[0]);
    };
    assert_eq!(terms.len(), 2);
    assert!(
        terms
            .iter()
            .any(|term| matches!(term, ComputedCssMath::Percentage(actual) if *actual == 0.05))
    );
    assert!(terms.iter().any(|term| match term {
        ComputedCssMath::LengthPx(actual) => *actual == -2.0,
        ComputedCssMath::Negate(value) => {
            matches!(value.as_ref(), ComputedCssMath::LengthPx(actual) if *actual == 2.0)
        }
        _ => false,
    }));
}

#[test]
fn track_sizing_profile_preserves_typed_neighbors_for_layout_boundary_rejection() {
    for neighbor in [
        "calc(20px - 5%)",
        "calc(10px + 5% + clamp(0px, 0%, 1px))",
        "min(20px, 5%)",
    ] {
        let fixture = root_grid(&format!(
            "grid-template-columns:{neighbor} repeat(auto-fill,minmax(40px,1fr));grid-template-rows:12px"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("typed auto-repeat neighbor {neighbor}: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();

        let Some(track) = grid.columns.first() else {
            panic!("CSS-valid neighbor {neighbor} lost its computed track: {grid:?}");
        };
        assert!(
            matches!(track, ComputedCssGridTrackSize::Calc(_)),
            "CSS-valid neighbor {neighbor} must reach the typed projection boundary: {track:?}"
        );
    }
}
