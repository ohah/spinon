use super::super::incremental::{
    IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
};
use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoRows, ComputedCssGridTrackSize, CssCascadeError,
};

#[test]
fn root_html_profile_preserves_only_fixed_px_auto_row_lists() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:15px 25px;grid-auto-columns:12px 18px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("the Root HTML profile should preserve a fixed-px auto-row list");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("the Grid snapshot should be present");

    assert_eq!(
        grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(15.0),
            ComputedCssGridTrackSize::LengthPx(25.0),
        ])
    );
    assert_eq!(
        grid.auto_columns,
        ComputedCssGridAutoColumns::Unsupported("12px 18px".to_owned())
    );
}

#[test]
fn root_html_profile_keeps_non_px_auto_row_lists_unsupported() {
    for value in ["15px 25%", "15px 1fr", "15px min-content"] {
        let style = format!(
            "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:{value};margin:0;padding:0"
        );
        let (view, root) = view_for_inline_style(&style);
        let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .expect("unsupported computed list values should stay typed as unsupported");
        let grid = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == root)
            .and_then(|element| element.grid_layout.as_ref())
            .expect("the Grid snapshot should be present");

        assert_eq!(
            grid.auto_rows,
            ComputedCssGridAutoRows::Unsupported(value.to_owned()),
            "{value}"
        );
    }
}

#[test]
fn root_html_fixed_px_auto_row_list_allocation_failure_is_atomic_and_recovers() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:15px 25px;margin:0;padding:0",
    );
    let error = {
        let _guard =
            inject_integer_repeat_reservation_failure(IntegerRepeatReservationSite::AutoRows);
        compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .expect_err("failed row-list reservation should abort cascade publication")
    };
    assert!(matches!(
        error,
        CssCascadeError::GridAllocationFailure {
            node,
            property: "grid-auto-rows"
        } if node == root
    ));

    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("same Root HTML view should recover after the injected reservation failure");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("recovered cascade should publish the complete Grid");
    assert_eq!(
        grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::LengthPx(15.0),
            ComputedCssGridTrackSize::LengthPx(25.0),
        ])
    );
}
