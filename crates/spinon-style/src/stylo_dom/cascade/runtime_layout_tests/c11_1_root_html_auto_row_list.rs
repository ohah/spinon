use super::super::incremental::{
    IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
};
use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoRows, ComputedCssGridTrackSize, CssCascadeError,
};

#[test]
fn root_html_profile_preserves_fixed_px_auto_row_lists() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:15px 25px;grid-auto-columns:25%;margin:0;padding:0",
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
        ComputedCssGridAutoColumns::Unsupported("25%".to_owned())
    );
}

#[test]
fn root_html_profile_preserves_single_percentage_auto_row_as_a_list() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:25%;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("a direct percentage auto row should be preserved as a one-entry list");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("the Grid snapshot should be present");

    assert_eq!(
        grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Percentage(0.25)])
    );
}

#[test]
fn root_html_profile_preserves_mixed_px_percentage_auto_row_list() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:15px 25%;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("a direct px/percentage row list should remain typed");
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
            ComputedCssGridTrackSize::Percentage(0.25),
        ])
    );
}

#[test]
fn root_html_profile_preserves_fractional_and_mixed_auto_row_lists() {
    for (value, expected) in [
        ("1fr", vec![ComputedCssGridTrackSize::Fraction(1.0)]),
        (
            "0fr 0.25fr",
            vec![
                ComputedCssGridTrackSize::Fraction(0.0),
                ComputedCssGridTrackSize::Fraction(0.25),
            ],
        ),
        (
            "15px 25% 1fr",
            vec![
                ComputedCssGridTrackSize::LengthPx(15.0),
                ComputedCssGridTrackSize::Percentage(0.25),
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
        ),
    ] {
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
        .expect("finite direct fractional auto rows should remain typed in Root HTML");
        let grid = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == root)
            .and_then(|element| element.grid_layout.as_ref())
            .expect("the Grid snapshot should be present");

        assert_eq!(grid.auto_rows, ComputedCssGridAutoRows::TrackList(expected));
    }
}

#[test]
fn root_html_profile_keeps_unsupported_auto_row_lists_unsupported() {
    for (value, serialized) in [
        ("15px min-content", "15px min-content"),
        ("15px calc(10px + 1%)", "15px calc(1% + 10px)"),
        ("15px auto", "15px auto"),
    ] {
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
            ComputedCssGridAutoRows::Unsupported(serialized.to_owned()),
            "{value}"
        );
    }
}

#[test]
fn root_html_profile_falls_back_after_syntax_invalid_negative_fractional_auto_row() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:-1fr;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("syntax-invalid negative fraction should fall back to the initial value");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("the Grid snapshot should be present");

    assert_eq!(grid.auto_rows, ComputedCssGridAutoRows::Auto);
}

#[test]
fn root_html_profile_falls_back_after_syntax_invalid_negative_auto_row_declaration() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:-1px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("syntax-invalid declaration should fall back to the initial auto value");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("the Grid snapshot should be present");

    assert_eq!(grid.auto_rows, ComputedCssGridAutoRows::Auto);
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

#[test]
fn root_html_percentage_auto_row_allocation_failure_is_atomic_and_recovers() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:25%;margin:0;padding:0",
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
        .expect_err("failed percentage row-list reservation should abort publication")
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
    .expect("same view should recover after percentage row-list allocation failure");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("recovered cascade should publish the complete Grid");
    assert_eq!(
        grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Percentage(0.25)])
    );
}
