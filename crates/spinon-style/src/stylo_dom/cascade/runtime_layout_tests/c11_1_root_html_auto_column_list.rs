use super::super::incremental::{
    IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
};
use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoRows, ComputedCssGridTrackSize, CssCascadeError,
};

fn root_grid(source: &str) -> (super::super::StyloDocumentView, spinon_core::NodeId) {
    view_for_inline_style(&format!(
        "display:grid;box-sizing:border-box;width:175px;height:63px;grid-template-columns:30px 40px;grid-template-rows:20px 40px;grid-auto-flow:column;grid-auto-rows:auto;grid-auto-columns:{source};margin:0;padding:0"
    ))
}

fn computed_auto_columns(source: &str) -> (spinon_core::NodeId, ComputedCssGridAutoColumns) {
    let (view, root) = root_grid(source);
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("Root HTML cascade should retain the grid-auto-columns typed result");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("Grid snapshot should be present");
    assert_eq!(grid.auto_rows, ComputedCssGridAutoRows::Auto);
    (root, grid.auto_columns.clone())
}

#[test]
fn root_html_profile_preserves_fixed_px_auto_column_values_and_lists() {
    assert_eq!(
        computed_auto_columns("15px").1,
        ComputedCssGridAutoColumns::LengthPx(15.0)
    );
    assert_eq!(
        computed_auto_columns("15px 25px").1,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(15.0),
            ComputedCssGridTrackSize::LengthPx(25.0),
        ])
    );
}

#[test]
fn root_html_profile_preserves_direct_percentage_auto_columns_and_mixed_lists() {
    assert_eq!(
        computed_auto_columns("15%").1,
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Percentage(0.15)])
    );
    assert_eq!(
        computed_auto_columns("10px 15%").1,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::Percentage(0.15),
        ])
    );
    assert_eq!(
        computed_auto_columns("10% 15%").1,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::Percentage(0.1),
            ComputedCssGridTrackSize::Percentage(0.15),
        ])
    );
}

#[test]
fn root_html_profile_preserves_direct_fraction_auto_columns_and_mixed_lists() {
    assert_eq!(
        computed_auto_columns("1fr").1,
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Fraction(1.0)])
    );
    assert_eq!(
        computed_auto_columns("1fr 2fr").1,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ])
    );
    assert_eq!(
        computed_auto_columns("10px 10% 1fr").1,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::Percentage(0.1),
            ComputedCssGridTrackSize::Fraction(1.0),
        ])
    );
    assert_eq!(
        computed_auto_columns("1e38fr").1,
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Fraction(1.0e38)])
    );
    assert_eq!(
        computed_auto_columns("-1fr").1,
        ComputedCssGridAutoColumns::Auto,
        "CSS-invalid negative fractions fall back to the initial auto value"
    );
}

#[test]
fn root_html_profile_keeps_intrinsic_auto_column_lists_unsupported() {
    for (source, serialized) in [
        ("minmax(10px, 20px)", "minmax(10px, 20px)"),
        ("fit-content(20px)", "fit-content(20px)"),
        ("calc(50% + 5px)", "calc(50% + 5px)"),
    ] {
        assert_eq!(
            computed_auto_columns(source).1,
            ComputedCssGridAutoColumns::Unsupported(serialized.to_owned()),
            "{source}"
        );
    }
    assert_eq!(
        computed_auto_columns("auto").1,
        ComputedCssGridAutoColumns::Auto
    );
    assert_eq!(
        computed_auto_columns("calc(10px + 5px)").1,
        ComputedCssGridAutoColumns::LengthPx(15.0)
    );
}

#[test]
fn root_html_fraction_auto_column_list_allocation_failure_is_atomic_and_recovers() {
    let (view, root) = root_grid("1fr 2fr");
    let error = {
        let _guard =
            inject_integer_repeat_reservation_failure(IntegerRepeatReservationSite::AutoColumns);
        compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .expect_err("failed fraction auto-column list reservation must abort cascade publication")
    };
    assert!(matches!(
        error,
        CssCascadeError::GridAllocationFailure {
            node,
            property: "grid-auto-columns"
        } if node == root
    ));

    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("same Root HTML view should recover after fraction allocation failure");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("recovered cascade should publish the complete Grid");
    assert_eq!(
        grid.auto_columns,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Fraction(2.0),
        ])
    );
}

#[test]
fn root_html_fixed_px_auto_column_list_allocation_failure_is_atomic_and_recovers() {
    let (view, root) = root_grid("15px 25px");
    let error = {
        let _guard =
            inject_integer_repeat_reservation_failure(IntegerRepeatReservationSite::AutoColumns);
        compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .expect_err("failed auto-column list reservation must abort cascade publication")
    };
    assert!(matches!(
        error,
        CssCascadeError::GridAllocationFailure {
            node,
            property: "grid-auto-columns"
        } if node == root
    ));

    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("same Root HTML view should recover after the injected allocation failure");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("recovered cascade should publish the complete Grid");
    assert_eq!(
        grid.auto_columns,
        ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(15.0),
            ComputedCssGridTrackSize::LengthPx(25.0),
        ])
    );
}
