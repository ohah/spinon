use super::*;

#[test]
fn track_sizing_profile_preserves_intrinsic_track_functions_and_calc_values() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:auto min-content max-content minmax(20px,max-content) minmax(min-content,2fr) fit-content(80px) calc(30px + 5%);grid-template-rows:fit-content(50%);margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();
    let grid = style.grid_layout.as_ref().unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    assert_eq!(
        grid.columns,
        [
            ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(20.0),
                max: ComputedCssGridTrackBreadth::MaxContent,
            },
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::MinContent,
                max: ComputedCssGridTrackBreadth::Fraction(2.0),
            },
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                80.0
            )),
            ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(0.05),
                ComputedCssMath::LengthPx(30.0),
            ])),
        ]
    );
    assert_eq!(
        grid.rows,
        [ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::Percentage(0.5)
        )]
    );
}

#[test]
fn previous_template_profile_rejects_intrinsic_track_functions() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:minmax(20px,1fr);grid-template-rows:40px;margin:0;padding:0",
    );

    let error = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::UnsupportedComputedGridValue {
            property: "grid-template-columns",
            ..
        }
    ));
}

#[test]
fn track_sizing_preserves_zero_fraction_in_minmax_maximum() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:minmax(auto,0fr);grid-template-rows:40px;margin:0;padding:0",
    );

    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert_eq!(
        grid.columns,
        [ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(0.0),
        }]
    );
}

#[test]
fn track_sizing_carries_aspect_ratio_for_transferred_size_suggestions() {
    let fixture = fixture(
        "display:grid;aspect-ratio:2 / 1;grid-template-columns:100px;grid-template-rows:40px",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();
    assert_eq!(style.layout_aspect_ratio, Some(2.0));
}
